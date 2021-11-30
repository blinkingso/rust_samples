extern crate structopt;

use chrono::prelude::*;
use std::mem::zeroed;
use structopt::StructOpt;

#[cfg(windows)]
use kernel32;
#[cfg(not(windows))]
use libc;
#[cfg(windows)]
use winapi;

mod ntp;

#[derive(StructOpt)]
#[structopt(name = "clock", version = "0.1", about = "Gets and sets the time.")]
enum Options {
    /// Get the time. Usage: get --use-standard rfc2822
    Get {
        /// Timestamp using stds
        #[structopt(name = "TIMESTAMP STANDARD", short = "s", long = "use-standard")]
        std: Option<String>,
    },
    /// Sets the time
    Set {
        /// Datetime to set
        #[structopt(name = "DATETIME", short, long = "datetime")]
        datetime: String,
        /// Timestamp using stds
        #[structopt(name = "TIMESTAMP STANDARD", short = "s", long = "use-standard")]
        std: String,
    },
    /// Check from NTP
    #[structopt(name = "check-ntp")]
    CheckNtp,
}

impl Default for Options {
    fn default() -> Self {
        Options::Get { std: None }
    }
}

fn main() {
    let options: Options = Options::from_args();
    match options {
        Options::Get { std: _std } => {
            let timestamp = get(&_std);
            println!("timestamp is : {}", &timestamp);
        }
        Options::Set {
            datetime,
            std: _std,
        } => {
            let timestamp = set_and_read_datetime(&_std, datetime);
            println!("now time is : {}", timestamp);
        }
        Options::CheckNtp => {
            if let Ok(offset) = ntp::check_time() {
                println!("offset is : {}", offset);
                let offset = offset as isize;
                let adjust_ms_ = offset.signum() * offset.abs().min(200) / 5;
                let adjust_ms = chrono::Duration::milliseconds(adjust_ms_ as i64);
                let now: DateTime<Utc> = Utc::now() + adjust_ms;
                println!("now is : {:?}", now);
                Clock::set(now);
            } else {
                eprintln!("sync time error");
            }
        }
    }
}

/// get formatted timestamp
fn get(std: &Option<String>) -> String {
    let now = Clock::get();
    return match std {
        None => format!("{}", now.timestamp()),
        Some(_std) => match _std.as_str() {
            "rfc2822" => format!("{}", now.to_rfc2822()),
            "rfc3339" => format!("{}", now.to_rfc3339()),
            _ => format!("{}", now.timestamp()),
        },
    };
}

fn set_and_read_datetime(std: &String, datetime: String) -> String {
    let parser = match std.as_str() {
        "rfc2822" => DateTime::parse_from_rfc2822,
        "rfc3339" => DateTime::parse_from_rfc3339,
        _ => unimplemented!(),
    };

    let error_msg = format!("Unable to parse {} according to {}", &datetime, &std);
    let parsed_time = parser(datetime.as_str()).expect(&error_msg);
    // set time
    Clock::set(parsed_time);
    get(&Some(String::from(std.as_str())))
}

struct Clock;
impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }
    /// non windows system function.
    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) {
        use libc::{settimeofday, timezone};
        use libc::{suseconds_t, time_t, timeval};

        let t = t.with_timezone(&Local);
        // UNIX time format with libc
        let mut u: timeval = unsafe { zeroed() };
        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            // call libc to set system time
            settimeofday(&u as *const timeval, mock_tz);
        }
    }

    #[cfg(windows)]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) {
        use chrono::Weekday;
        use kernel32::SetSystemTime;
        use winapi::um::minwinbase::{SYSTEMTIME, WORD};

        // Local timezone DateTime
        let t = t.with_timezone(&Local);
        let mut systime: SYSTEMTIME = unsafe { zeroed() };

        let dow = match t.weekday() {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 0,
        };

        let mut ns = t.nanosecond();
        let mut leap = 0;
        let is_leap_second = ns > 1_000_000_000;
        if is_leap_second {
            ns -= 1_000_000_000;
            leap += 1;
        }
        systime.wYear = t.year() as WORD;
        systime.wMonth = t.month() as WORD;
        systime.wDayOfWeek = dow as WORD;
        systime.wDay = t.day() as WORD;
        systime.wHour = t.hour() as WORD;
        systime.wMinute = t.minute() as WORD;
        systime.wSecond = (leap + t.second()) as WORD;
        systime.wMilliseconds = (ns / 1_000_000) as WORD;

        let systime_ptr = &systime as *const SYSTETIME;
        unsafe { SetSystemTime(systime_ptr) }
    }
}

#[cfg(test)]
mod test {
    use chrono::{FixedOffset, Local, Utc};

    #[test]
    fn test_chrono() {
        let now = Utc::now();
        println!("now is: {}", now.to_rfc3339());
        let now = Utc::now().with_timezone(&Local);
        println!("real now is : {}", now);
        let tz = FixedOffset::east(-6 * 60 * 60);
        let fixed_datetime = Utc::now().with_timezone(&tz);
        println!("fixed datetime is : {}", fixed_datetime);
    }
}
