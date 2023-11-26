use std::{num::ParseIntError, ops::Range};

use thiserror::Error;

use crate::{DateTime, LogLine, Method, NgxLaserError, ParseDateTimeError};

#[derive(Debug, Error)]
pub enum ParseLogError {
    #[error("invalid log line {0:?}")]
    InvalidLogLine(String),
    #[error("character not found {0:?}")]
    CharNotFound(char),
    #[error("date parse error")]
    InvalidDateTime(#[from] ParseDateTimeError),
    #[error("expected int")]
    IntExpected(#[from] ParseIntError),
}

#[derive(Debug)]
pub struct Parser {
    parts: Vec<(Box<str>, Vec<char>)>,
}

impl Parser {
    const VARS: [&'static str; 8] = [
        "$remote_addr",
        "$remote_user",
        "$time_local",
        "$request",
        "$status",
        "$body_bytes_sent",
        "$http_referer",
        "$http_user_agent",
    ];

    pub fn parse(&self, line: &str) -> Result<LogLine, ParseLogError> {
        let mut data: Vec<&str> = vec![""; 8];
        let mut cursor: usize = 0;
        let mut item = self.parts[cursor].0.as_ref();
        let mut sep = self.parts[cursor].1.as_slice();

        let mut part_idx: usize = 0;
        let mut val_range = Range { start: 0, end: 0 };

        for (idx, ch) in &mut line.char_indices() {
            if ch == sep[part_idx] {
                if part_idx < (sep.len() - 1) {
                    // Continue separator
                    part_idx += 1;
                } else {
                    // End of separator
                    if item == "" {
                        // Case line start with delimeter
                        val_range.start = idx + 1;
                        val_range.end = idx + 1;
                        cursor += 1;
                        item = self.parts[cursor].0.as_ref();
                        sep = self.parts[cursor].1.as_slice();
                        continue;
                    }

                    // Save value to data
                    let var_name: &str = item.as_ref();
                    // println!("var_name: {}", var_name);
                    let field_idx = Parser::VARS.iter().position(|x| x == &var_name);
                    match field_idx {
                        Some(index) => data[index] = &line[val_range.start..=val_range.end],
                        None => {}
                    }

                    // Move cursor
                    part_idx = 0;
                    if cursor + 1 < self.parts.len() {
                        cursor += 1;
                        item = self.parts[cursor].0.as_ref();
                        sep = self.parts[cursor].1.as_slice();
                    } else {
                        break;
                    }
                    val_range.start = idx + 1;
                    val_range.end = idx + 1;
                }
            } else {
                val_range.end = idx;
            }
        }

        // println!(">> parse data: {:?}", data);

        let date_time = DateTime::from_nginx(data[2].into())?;
        let status: u16 = data[4].parse()?;
        let mut request = data[3].split(' ');
        let (method, path) = match (request.next(), request.next()) {
            (Some(method), Some(path)) => (Method::from(method), path),
            (Some(path), None) => (Method::None, path),
            _ => unreachable!(),
        };
        let bytes_sent = data[5].parse()?;
        let referer = data[6].into();
        let agent = data[7].into();
        Ok(LogLine {
            date_time,
            method,
            path: path.into(),
            remote_addr: data[0].into(),
            // date_idx: 1,
            status,
            bytes_sent,
            referer,
            agent,
        })
    }

    pub fn new(format: &str) -> Result<Self, NgxLaserError> {
        let mut parts: Vec<(Box<str>, Vec<char>)> = Vec::new();
        let mut cursor = String::from("");
        let mut in_var = false;
        let mut part: (Box<str>, Vec<char>) = ("".into(), Vec::new());
        for (_, c) in format.char_indices() {
            match c {
                '$' => {
                    if cursor.len() > 0 {
                        part.1 = cursor.chars().collect();
                        parts.push(part);
                        part = ("".into(), Vec::new());
                    }
                    cursor = String::from("");
                    in_var = true;
                }
                'a'..='z' | '_' => {}
                _ => {
                    if in_var {
                        part.0 = cursor.into();
                        cursor = String::from("");
                        in_var = false;
                    }
                }
            }
            cursor.push(c);
        }

        if cursor.len() > 0 {
            if in_var {
                part.0 = cursor.into();
                part.1 = "\n".chars().collect();
            } else {
                part.1 = cursor.chars().collect();
            }
            parts.push(part);
        }

        // println!("{:?}", parts);
        Ok(Self { parts })
    }
}
//

#[cfg(test)]
mod line_parser_tests {
    use crate::{Parser, Method};

    #[test]
    fn parse_sio_with_default_formate() {
        let format: &str = r#"$remote_addr - $remote_user [$time_local] "$request" $status $body_bytes_sent "$http_referer" "$http_user_agent""#;
        let parser = Parser::new(format).unwrap();
        let line: &str = r#"10.232.28.160 - - [22/Jan/2021:02:49:30 +0000] "GET /socket.io/?EIO=3&transport=polling&t=NSd_nu- HTTP/1.1" 200 99 "https://miaou.dystroy.org/3" "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/73.0.3683.103 Safari/537.36""#;
        let ll = parser.parse(line).unwrap();

        assert_eq!(&*ll.remote_addr, "10.232.28.160");
        assert_eq!(ll.method, Method::Get);
        assert_eq!(&*ll.path, "/socket.io/?EIO=3&transport=polling&t=NSd_nu-");
        assert_eq!(ll.status, 200);
        assert_eq!(ll.bytes_sent, 99);
        assert_eq!(&*ll.referer, "https://miaou.dystroy.org/3");
    }

    #[test]
    fn parse_line_with_custom_formate() {
        let format: &str = r#""$time_local" client=$remote_addr method=$request_method request="$request" request_length=$request_length status=$status bytes_sent=$bytes_sent body_bytes_sent=$body_bytes_sent referer=$http_referer user_agent="$http_user_agent" upstream_addr=$upstream_addr upstream_status=$upstream_status request_time=$request_time upstream_response_time=$upstream_response_time"#;
        let parser = Parser::new(format).unwrap();
        let line: &str = r#""23/Jun/2023:03:17:47 +0300" client=10.4.44.30 method=GET request="GET /api/notifications/?limit=10000&notification_type=4,5 HTTP/1.1" request_length=1217 status=200 bytes_sent=325 body_bytes_sent=52 referer=https://vk.ecur.mosreg.ru/junior_controller/petitions/waiting_review?s=published%7C0 user_agent="Mozilla/5.0 (Windows NT 6.3; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.116 (Chromium GOST) Safari/537.36" upstream_addr=10.10.19.137:8000 upstream_status=200 request_time=0.032 upstream_response_time=0.032"#;
        let ll = parser.parse(line).unwrap();
        
        assert_eq!(&*ll.remote_addr, "10.4.44.30");
        assert_eq!(ll.method, Method::Get);
        assert_eq!(
            &*ll.path,
            "/api/notifications/?limit=10000&notification_type=4,5"
        );
        assert_eq!(ll.status, 200);
        assert_eq!(ll.bytes_sent, 52);
        assert_eq!(
            &*ll.referer,
            "https://vk.ecur.mosreg.ru/junior_controller/petitions/waiting_review?s=published%7C0"
        );
    }
}
