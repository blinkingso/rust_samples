//! NTP client
use byteorder::{BigEndian, ReadBytesExt};
use chrono::{DateTime, TimeZone, Timelike, Utc};
use std::boxed::Box;
use std::error::Error;
use std::net::UdpSocket;
use std::time::Duration;

const LOCAL_ADDR: &'static str = "0.0.0.0:12300";
/// Timestamp Result for a NTP request/response
#[derive(Debug)]
struct NTPResult {
    t1: DateTime<Utc>,
    t2: DateTime<Utc>,
    t3: DateTime<Utc>,
    t4: DateTime<Utc>,
}

fn ntp_roundtrip(host: &str, port: u16) -> Result<NTPResult, Box<dyn Error>> {
    let destination = format!("{}:{}", host, port);
    let timeout = Duration::from_secs(1);

    let request = NTPMessage::client();
    let mut response = NTPMessage::new();
    let message = request.data;

    let udp = UdpSocket::bind(&LOCAL_ADDR)?;
    let _ = udp
        .connect(&destination)
        .expect("unable to connect to NTP server");
    // time before request
    let t1 = Utc::now();
    // send request
    let _ = udp.send(&message)?;
    udp.set_read_timeout(Some(timeout))?;
    let (_, _) = udp.recv_from(&mut response.data)?;
    let t4 = Utc::now();
    let root_delay = response.root_delay().unwrap();
    println!("response data: {:?}", &response.data);
    println!("root delay: {}", root_delay);
    // datetime read from remote server
    let t2: DateTime<Utc> = response.rx_time().unwrap().into();
    // server time that the server sent the reply
    let t3: DateTime<Utc> = response.tx_time().unwrap().into();

    Ok(NTPResult { t1, t2, t3, t4 })
}

pub fn check_time() -> Result<f64, Box<dyn Error>> {
    const NTP_PORT: u16 = 123;
    let servers = ["time.apple.com", "time.euro.apple.com"];

    let mut times = Vec::with_capacity(servers.len());
    for server in servers.iter() {
        print!("{} =>", server);
        let calc = ntp_roundtrip(&server, NTP_PORT)?;
        println!("NTP : {:?}", calc);
        times.push(calc);
    }

    let mut offsets = Vec::with_capacity(servers.len());
    let mut offset_weights = Vec::with_capacity(servers.len());

    for time in &times {
        let offset = time.offset() as f64;
        let delay = time.delay() as f64;

        let weight = 1_000_000.0 / (delay * delay);
        if weight.is_finite() {
            offsets.push(offset);
            offset_weights.push(weight);
        }
    }

    let avg_offset = weighted_mean(&offsets, &offset_weights);
    Ok(avg_offset)
}

#[derive(Default, Debug, Copy, Clone)]
struct NTPTimestamp {
    seconds: u32,
    fraction: u32,
}

// 1900.1.1 ~ 1970.1.1, 70 years time
const NTP_TO_UNIX_SECONDS: i64 = 2_208_988_800;
impl From<NTPTimestamp> for DateTime<Utc> {
    fn from(ntp: NTPTimestamp) -> Self {
        let secs = ntp.seconds as i64 - NTP_TO_UNIX_SECONDS;
        let mut nanos = ntp.fraction as f64;
        nanos *= 1e9;
        nanos /= 2_f64.powi(32);

        Utc.timestamp(secs, nanos as u32)
    }
}

impl From<DateTime<Utc>> for NTPTimestamp {
    fn from(utc: DateTime<Utc>) -> Self {
        let seconds = utc.timestamp() + NTP_TO_UNIX_SECONDS;
        let mut fraction = utc.nanosecond() as f64;
        fraction *= 2_f64.powi(32);
        fraction /= 1e9;

        NTPTimestamp {
            seconds: seconds as u32,
            fraction: fraction as u32,
        }
    }
}

const NTP_MESSAGE_LENGTH: usize = 48;
struct NTPMessage {
    data: [u8; NTP_MESSAGE_LENGTH],
}

impl NTPResult {
    fn offset(&self) -> i64 {
        let duration = (self.t2 - self.t1) + (self.t4 - self.t3);
        duration.num_milliseconds() / 2
    }

    fn delay(&self) -> i64 {
        let duration = (self.t4 - self.t1) - (self.t3 - self.t2);
        duration.num_milliseconds()
    }
}
impl NTPMessage {
    fn new() -> Self {
        NTPMessage {
            data: [0; NTP_MESSAGE_LENGTH],
        }
    }

    fn client() -> Self {
        // 2 -> 11 reserved 2bits -> 00
        const LI: u8 = 0b00_111_111;
        // 3
        const VERSION: u8 = 0b00_011_000;
        // 3 -> client
        const MODE: u8 = 0b00_000_011;

        let mut msg = NTPMessage::new();

        msg.data[0] &= LI;
        msg.data[0] |= VERSION;
        msg.data[0] |= MODE;
        msg
    }

    fn parse_timestamp(&self, i: usize) -> Result<NTPTimestamp, Box<dyn Error>> {
        let mut reader = &self.data[i..i + 8];
        let seconds = reader.read_u32::<BigEndian>()?;
        let fraction = reader.read_u32::<BigEndian>()?;

        Ok(NTPTimestamp { seconds, fraction })
    }

    /// Receive Timestamp 8bytes
    fn rx_time(&self) -> Result<NTPTimestamp, Box<dyn Error>> {
        self.parse_timestamp(32)
    }

    /// Transmit Timestamp 8bytes
    fn tx_time(&self) -> Result<NTPTimestamp, Box<dyn Error>> {
        self.parse_timestamp(40)
    }

    /// read root_delay
    fn root_delay(&self) -> Result<u32, Box<dyn Error>> {
        let mut reader = &self.data[4..8];
        let root_delay = reader.read_u32::<BigEndian>()?;
        return Ok(root_delay);
    }
}

fn weighted_mean(values: &[f64], weights: &[f64]) -> f64 {
    let mut result = 0.0;
    let mut sum_of_weights = 0.0;
    for (v, w) in values.iter().zip(weights) {
        result += v * w;
        sum_of_weights += w;
    }

    result / sum_of_weights
}
