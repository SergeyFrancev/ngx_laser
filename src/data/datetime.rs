use thiserror::Error;
use std::{
        fmt,
        num::ParseIntError,
        str::FromStr,
    };

// struct MyDateTime(u16, u8, u8, u8, u8, u8);

#[derive(Debug, Error)]
pub enum ParseDateTimeError {

    #[error("unexpected end")]
    UnexpectedEnd,

    #[error("invalid day {0:?}")]
    InvalidDay(u8),

    #[error("date is ambiguous in context {0:?}")]
    AmbiguousDate(String),

    #[error("invalid month {0:?}")]
    InvalidMonth(u8),

    #[error("unrecognized month {0:?}")]
    UnrecognizedMonth(String),

    #[error("invalid hour {0:?}")]
    InvalidHour(u8),

    #[error("invalid minute {0:?}")]
    InvalidMinute(u8),

    #[error("invalid second {0:?}")]
    InvalidSecond(u8),

    #[error("expected int")]
    IntExpected(#[from] ParseIntError),

    #[error("expected int")]
    IntExpectedInternal,
}


pub static MONTHS_3_LETTERS: &[&str] = &[
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
];


/// a date with time.
///
/// It's implicitely in the timezone of the log files
/// (assuming all the files have the same one).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl DateTime {
    pub fn new(
        year: u16, // in [0-4000]
        month: u8, // in [1-12]
        day: u8, // in [1-31]
        hour: u8, // in [0-23]
        minute: u8,
        second: u8,
    ) -> Result<Self, ParseDateTimeError> {
        Ok(Self {
            date: Date::new(year, month, day)?,
            time: Time::new(hour, minute, second)?,
        })
    }
    /// parse the date_time part of a nginx log line
    ///
    /// a datetime in nginx is either in
    /// - "common log format", eg `10/Jan/2021:10:27:01 +0000`
    /// - ISO 8601, eg `1977-04-22T01:00:00-05:00`
    pub fn from_nginx(s: &str) -> Result<Self, ParseDateTimeError> {
        if s.len()<20 {
            return Err(ParseDateTimeError::UnexpectedEnd);
        }
        if let Ok(year) = s[0..4].parse() {
            // let's go with ISO 8601
            let month = s[5..7].parse()?;
            let day = s[8..10].parse()?;
            let hour = s[11..13].parse()?;
            let minute = s[14..16].parse()?;
            let second = s[17..19].parse()?;
            Self::new(year, month, day, hour, minute, second)
        } else {
            // maybe common log format ?
            let day = s[0..2].parse()?;
            let month = &s[3..6];
            let month = MONTHS_3_LETTERS
                .iter()
                .position(|&m| m == month)
                .ok_or_else(|| ParseDateTimeError::UnrecognizedMonth(s.to_owned()))?;
            let month = (month + 1) as u8;
            let year = s[7..11].parse()?;
            let hour = s[12..14].parse()?;
            let minute = s[15..17].parse()?;
            let second = s[18..20].parse()?;
            Self::new(year, month, day, hour, minute, second)
        }
    }
    pub fn round_up(date: Date, time: Option<Time>) -> Self {
        Self {
            date,
            time: time.unwrap_or(Time { hour:23, minute:59, second:59}),
        }
    }
    pub fn round_down(date: Date, time: Option<Time>) -> Self {
        Self {
            date,
            time: time.unwrap_or(Time { hour:0, minute:0, second:0}),
        }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{:0>2}/{:0>2}T{:0>2}:{:0>2}:{:0>2}",
            self.date.year,
            self.date.month,
            self.date.day,
            self.time.hour,
            self.time.minute,
            self.time.second,
        )
    }
}

/// a not precise date, only valid in the context
/// of the local set of log files.
/// It's implicitely in the timezone of the log files
/// (assuming all the files have the same one).
/// As nginx didn't exist before JC, a u16 is good enough
/// for the year.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: u16,
    pub month: u8, // in [1,12]
    pub day: u8,   // in [1,31]
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, ParseDateTimeError> {
        if day < 1 || day > 31 {
            return Err(ParseDateTimeError::InvalidDay(day));
        }
        if month < 1 || month > 12 {
            return Err(ParseDateTimeError::InvalidMonth(month));
        }
        Ok(Self { year, month, day })
    }
    /// parse the date part of a nginx datetime.
    ///
    /// a datetime in nginx is either in
    /// - "common log format", eg `10/Jan/2021:10:27:01 +0000`
    /// - ISO 8601, eg `1977-04-22T01:00:00-05:00`
    pub fn from_nginx(s: &str) -> Result<Self, ParseDateTimeError> {
        if s.len()<11 {
            return Err(ParseDateTimeError::UnexpectedEnd);
        }
        if let Ok(year) = s[0..4].parse() {
            // let's go with ISO 8601
            let month = s[5..7].parse()?;
            let day = s[8..10].parse()?;
            Self::new(year, month, day)
        } else {
            // maybe common log format ?
            let day = s[0..2].parse()?;
            let month = &s[3..6];
            let month = MONTHS_3_LETTERS
                .iter()
                .position(|&m| m == month)
                .ok_or_else(|| ParseDateTimeError::UnrecognizedMonth(s.to_owned()))?;
            let month = (month + 1) as u8;
            let year = s[7..11].parse()?;
            Self::new(year, month, day)
        }
    }
    /// parse a numeric date with optionally implicit parts
    /// The part separator is the '/'
    pub fn with_implicit(
        s: &str,
        default_year: Option<u16>,
        default_month: Option<u8>,
    ) -> Result<Self, ParseDateTimeError> {
        let mut t = s.split('/');
        match (t.next(), t.next(), t.next()) {
            (Some(year), Some(month), Some(day)) => {
                Date::new(year.parse()?, month.parse()?, day.parse()?)
            }
            (Some(month), Some(day), None) => {
                if let Some(year) = default_year {
                    Date::new(year, month.parse()?, day.parse()?)
                } else {
                    Err(ParseDateTimeError::AmbiguousDate(s.to_owned()))
                }
            }
            (Some(day), None, None) => {
                if let (Some(year), Some(month)) = (default_year, default_month) {
                    Date::new(year, month, day.parse()?)
                } else {
                    Err(ParseDateTimeError::AmbiguousDate(s.to_owned()))
                }
            }
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{:0>2}/{:0>2}", self.year, self.month, self.day)
    }
}

pub fn unique_year_month(start_date: Date, end_date: Date) -> (Option<u16>, Option<u8>) {
    let y1 = start_date.year;
    let y2 = end_date.year;
    if y1 == y2 {
        let m1 = start_date.month;
        let m2 = end_date.month;
        if m1 == m2 {
            (Some(y1), Some(m1))
        } else {
            (Some(y1), None)
        }
    } else {
        (None, None)
    }
}

/// a time, only valid in the context of the local set of log files.
/// It's implicitely in the timezone of the log files
/// (assuming all the files have the same one).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    pub hour: u8, // in [0,23]
    pub minute: u8, // in [0,59]
    pub second: u8,   // in [0,59]
}
impl Time {
    pub fn new(
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Self, ParseDateTimeError> {
        if second > 59 {
            return Err(ParseDateTimeError::InvalidSecond(second));
        }
        if minute > 59 {
            return Err(ParseDateTimeError::InvalidMinute(minute));
        }
        if hour > 23 {
            return Err(ParseDateTimeError::InvalidHour(hour));
        }
        Ok(Self { hour, minute, second })
    }
}

impl FromStr for Time {
    type Err = ParseDateTimeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len()<2 {
            return Err(ParseDateTimeError::UnexpectedEnd);
        }
        let hour = s[0..2].parse()?;
        let mut minute = 0;
        let mut second = 0;
        if s.len()>4 {
            minute = s[3..5].parse()?;
            if s.len()>6 {
                second = s[6..7].parse()?;
            }
        }
        Time::new(hour, minute, second)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}:{:0>2}", self.hour, self.minute, self.second)
    }
}
