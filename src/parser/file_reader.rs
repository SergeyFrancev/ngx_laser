use cli_log::{debug, warn};

use {
    crate::*,
    flate2::bufread::GzDecoder,
    std::{
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    },
};

pub struct FileReader<'c> {
    consumer: &'c mut Consumers,
    // stop_on_error: bool,
    parser: &'c Parser,
}

impl<'c> FileReader<'c> {
    pub fn new(consumer: &'c mut Consumers, parser: &'c Parser) -> Self {
        Self {
            parser,
            consumer,
            // stop_on_error: false,
        }
    }

    pub fn get_first_date(&mut self, path: &Path) -> Option<LogLine> {
        LogFile::new(path, self.parser).into_iter().take(5).next()
    }

    pub fn read_all_lines(&mut self, path: &Path) {
        LogFile::new(path, self.parser)
            .into_iter()
            .for_each(|x| self.consumer.eat_line(x));
    }
}

pub struct LogFile<'c> {
    content: Box<dyn BufRead>,
    parser: &'c Parser,
    line: String,
    errors: usize,
    // counter: usize,
}

impl<'c> LogFile<'c> {
    pub fn new(path: &Path, parser: &'c Parser) -> LogFile<'c> {
        // println!("Path: {:?}", path);
        let file = File::open(path).unwrap();
        let file = BufReader::new(file);
        let file: Box<dyn BufRead> = match path.extension().and_then(|e| e.to_str()) == Some("gz") {
            true => Box::new(BufReader::new(GzDecoder::new(file))),
            false => Box::new(file),
        };

        Self {
            parser,
            content: file,
            line: String::with_capacity(600),
            errors: 0,
            // counter: 0,
        }
    }

    // fn err_count(&self) -> usize {
    //     self.errors
    // }
}

impl<'c> Iterator for LogFile<'c> {
    type Item = LogLine;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.line.clear();
            let result = self.content.read_line(&mut self.line);
            if result.is_err() || result.unwrap() == 0 {
                return None;
            }
            // println!("line: {}", self.line);
            match self.parser.parse(&self.line) {
                Ok(log_line) => {
                    // let filtered_out = !self.filterer.accepts(&log_line);
                    return Some(log_line);
                }
                Err(e) => {
                    // we only log the first error
                    match self.errors {
                        0 => warn!("{} in {}", e, self.line),
                        1 => {
                            warn!("logging other errors in this file as debug only");
                            debug!("{} in {}", e, self.line);
                        }
                        _ => {
                            debug!("{} in {}", e, self.line);
                        }
                    }
                    self.errors += 1;
                }
            }
        }
    }
}
