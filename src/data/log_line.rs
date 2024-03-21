use crate::{DateTime, Method, Date, Time};

/// A line in the access log, describing a hit.
// perf note: parsing the remote adress as IP is costly
// (app is about 3% faster if I replace this field with a string)
#[derive(Debug)]
pub struct LogLine {
    pub remote_addr: Box<str>,
    pub date_time: DateTime,
    // pub date_idx: usize,
    pub method: Method,
    pub path: Box<str>,
    pub status: u16,
    pub bytes_sent: u64,
    pub referer: Box<str>,
    pub agent: Box<str>,
}

// impl DateIndexed for LogLine {
//     fn date_idx(&self) -> usize {
//         self.date_idx
//     }
//     fn bytes(&self) -> u64 {
//         self.bytes_sent
//     }
// }
// impl DateIndexed for &LogLine {
//     fn date_idx(&self) -> usize {
//         self.date_idx
//     }
//     fn bytes(&self) -> u64 {
//         self.bytes_sent
//     }
// }

impl LogLine {
    pub fn date(&self) -> Date {
        self.date_time.date
    }
    pub fn time(&self) -> Time {
        self.date_time.time
    }
}
