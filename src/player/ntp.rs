use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use byteorder::{BigEndian, WriteBytesExt};

pub struct NTP {
    time_ref: i64,
}

impl NTP {
    pub fn new() -> Self {
        Self {            
            time_ref : Self::get_time_ref(),
        }
    }

    fn get_time_ref() -> i64 {
        let config_ntp_epoch = 0x83aa7e80; // Replace with the actual value from your config
        let current_time = Utc::now().timestamp_millis();
        let epoch_time = NaiveDateTime::from_timestamp(config_ntp_epoch, 0).timestamp_millis();
        return current_time - epoch_time;
    }

    pub fn time_ref(&self) -> i64 {
        self.time_ref
    }

    pub fn timestamp(&self) -> Vec<u8> {
        let current_time = Utc::now().timestamp_millis();
        println!("current_time: {:?}", current_time);
        let time = current_time - self.time_ref;
        let sec = (time / 1000) as u32;
        let msec = (time - sec as i64 * 1000) as u32;
        let ntp_msec = (msec as f64 * 4294967.296) as u32;

        let mut ts: Vec<u8> = Vec::with_capacity(8);
        ts.write_u32::<BigEndian>(sec).unwrap();
        ts.write_u32::<BigEndian>(ntp_msec).unwrap();

        ts
    }

}