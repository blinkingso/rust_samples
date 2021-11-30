use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crc::{Crc, CRC_32_CKSUM};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;

type ByteString = Vec<u8>;
type ByteStr = [u8];

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: ByteString,
    pub value: ByteString,
}

/// File Storage Format:
/// checksum(u32) key_len(u32) value_len(u32) key([u8;key_len]) value([u8;value_len])
#[derive(Debug)]
pub struct ActionKV {
    f: File,
    pub index: HashMap<ByteString, u64>,
}

impl ActionKV {
    /// open or create a file storage
    pub fn open(path: &Path) -> io::Result<Self> {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(path)?;
        let index = HashMap::new();
        Ok(ActionKV { f, index })
    }

    /// load all data into the map;
    pub fn load(&mut self) -> io::Result<()> {
        let mut f = BufReader::new(&mut self.f);
        loop {
            // number of bytes from the start of the file, initialize is from 0;
            let position = f.seek(SeekFrom::Current(0))?;
            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };
            self.index.insert(kv.key, position);
        }
        Ok(())
    }

    /// seek to the end of the file
    pub fn seek_to_end(&mut self) -> io::Result<u64> {
        self.f.seek(SeekFrom::End(0))
    }

    /// Read one line data.
    fn process_record<R: Read>(f: &mut R) -> io::Result<KeyValuePair> {
        let saved_check_sum = f.read_u32::<LittleEndian>()?;
        let key_len = f.read_u32::<LittleEndian>()?;
        let value_len = f.read_u32::<LittleEndian>()?;
        let data_len = key_len + value_len;
        let mut data = ByteString::with_capacity(data_len as usize);
        //read data from one record;
        {
            // read more data_len data from `f: &mut R`
            f.by_ref().take(data_len as u64).read_to_end(&mut data)?;
        }
        debug_assert_eq!(data.len(), data_len as usize);
        let checksum = Crc::<u32>::new(&CRC_32_CKSUM).checksum(&data);
        if checksum != saved_check_sum {
            panic!(
                "data corruption encountered ({:08x} != {:08x})",
                checksum, saved_check_sum
            );
        }
        let value = data.split_off(key_len as usize);
        let key = data;
        Ok(KeyValuePair { key, value })
    }

    #[allow(dead_code)]
    /// read u32 from `std::io::Read`
    fn read_u32<R: Read>(r: &mut R) -> io::Result<u32> {
        let mut buffer: [u8; 4] = [0; 4];
        r.read_exact(&mut buffer)?;
        let value = u32::from_be_bytes(buffer[..4].try_into().unwrap());
        Ok(value)
    }

    /// insert new record to the file db;
    pub fn insert(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        let position = self.insert_but_ignore_index(key, value)?;
        self.index.insert(key.to_vec(), position);
        Ok(())
    }

    /// insert new record to the file db using key and index(position start in the db)
    pub fn insert_but_ignore_index(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<u64> {
        let mut f = BufWriter::new(&mut self.f);
        let key_len = key.len();
        let value_len = value.len();
        let data_len = key_len + value_len;
        let mut tmp = ByteString::with_capacity(data_len);
        for byte in key {
            tmp.push(*byte);
        }
        for byte in value {
            tmp.push(*byte);
        }
        let checksum = Crc::<u32>::new(&CRC_32_CKSUM).checksum(&tmp);
        let next_byte = SeekFrom::End(0);
        let current_position = f.seek(SeekFrom::Current(0))?;
        f.seek(next_byte)?;
        f.write_u32::<LittleEndian>(checksum)?;
        f.write_u32::<LittleEndian>(key_len as u32)?;
        f.write_u32::<LittleEndian>(value_len as u32)?;
        f.write_all(&mut tmp)?;
        println!("full size : {}", f.by_ref().buffer().len());
        Ok(current_position)
    }

    /// get from db
    pub fn get(&mut self, key: &ByteStr, scan: bool) -> io::Result<Option<ByteString>> {
        let position = match self.index.get(key) {
            None => {
                if scan {
                    if let Some(val) = self.find(key)? {
                        return Ok(Some(val.1));
                    }
                }
                return Ok(None);
            }
            Some(position) => *position,
        };
        let kv = self.get_at(position)?;
        Ok(Some(kv.value))
    }

    /// get data at specified index
    pub fn get_at(&mut self, position: u64) -> io::Result<KeyValuePair> {
        let mut f = BufReader::new(&mut self.f);
        f.seek(SeekFrom::Start(position))?;
        let kv = ActionKV::process_record(&mut f)?;
        Ok(kv)
    }

    /// find data from db
    pub fn find(&mut self, key: &ByteStr) -> io::Result<Option<(u64, ByteString)>> {
        let mut f = BufReader::new(&mut self.f);
        let mut found: Option<(u64, ByteString)> = None;
        loop {
            let position = f.seek(SeekFrom::Current(0))?;
            println!("seek to : {}", position);
            let maybe_kv = ActionKV::process_record(&mut f);
            let kv = match maybe_kv {
                Ok(kv) => kv,
                Err(err) => match err.kind() {
                    io::ErrorKind::UnexpectedEof => {
                        break;
                    }
                    _ => return Err(err),
                },
            };
            if kv.key == key {
                found = Some((position, kv.value));
            }
        }

        Ok(found)
    }

    /// update kv
    #[inline]
    pub fn update(&mut self, key: &ByteStr, value: &ByteStr) -> io::Result<()> {
        self.insert(key, value)
    }

    #[inline]
    pub fn delete(&mut self, key: &ByteStr) -> io::Result<()> {
        self.insert(key, b"")
    }

    /// store index on disk
    pub fn store_index_on_disk(&mut self, index_key: &ByteStr) {
        self.index.remove(index_key);
        let index_as_bytes = bincode::serialize(&self.index).unwrap();
        self.index = HashMap::new();
        self.insert(index_key, &index_as_bytes).unwrap();
    }
}

#[test]
fn test_read_u32() -> io::Result<()> {
    let str = "hello world";
    let mut r = std::io::stdin();
    let mut buffer: [u8; 6] = [0; 6];
    r.read_exact(&mut buffer)?;
    let value = u32::from_be_bytes(buffer[..4].try_into().unwrap());
    r.read_exact(&mut buffer)?;
    let value2 = u32::from_be_bytes(buffer[..4].try_into().unwrap());
    println!("checksum is : {}, 2is : {}", value, value2);
    let checksum = Crc::<u32>::new(&CRC_32_CKSUM).checksum(&buffer);
    println!("checksum is : {}", checksum);
    let mut f = File::open(Path::new("./src/akv_mem.rs"))?;
    let mut buf = BufReader::new(&mut f);
    loop {
        let postion = buf.seek(SeekFrom::Current(0))?;
        buf.read_u32::<LittleEndian>()?;
        buf.read_u32::<LittleEndian>()?;
        println!("position is : {}", postion);
    }

    Ok(())
}

#[test]
fn test_writing_integers_to_file() {
    let mut w = vec![];
    let one: u32 = 1;
    let two: i8 = 2;
    let three: f64 = 3.1;

    w.write_u32::<BigEndian>(one).unwrap();
    println!("{:?}", &w);

    w.write_i8(two).unwrap();
    println!("{:?}", &w);

    w.write_f64::<LittleEndian>(three).unwrap();
    println!("{:?}", &w);

    w.write(&u32::to_be_bytes(one));
    w.write(&i8::to_be_bytes(two));
    w.write(&f64::to_be_bytes(three));
    println!("{:?}", &w);

    println!("{}", 0b00000000 as u8);
    println!("{}", 0b00000001 as u8);
}
