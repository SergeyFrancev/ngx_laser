use crate::{LineConsumer, LogLine, ResourceConsumer, VisitorsConsumer, FormatVisitor, StatusConsumer};
use serde_json::{Map, Value};

// pub trait ConsumerItem:LineConsumer + FormatVisitor;

#[derive(Default)]
pub struct Consumers {
    pub consumers: Vec<Box<dyn LineConsumer>>,
    lines: Vec<LogLine>,
    size: usize,
}

impl Consumers {
    pub fn new() -> Self {
        let consumers: Vec<Box<dyn LineConsumer>> = Vec::from([]);
        Self {
            consumers,
            lines: Vec::new(),
            size: 0,
        }
    }

    pub fn add_consumer(&mut self, c: Box<dyn LineConsumer>) {
        self.consumers.push(c);
        // [
        //     Box::new(ResourceConsumer::default()) as Box<dyn LineConsumer>,
        //     Box::new(VisitorsConsumer::default()) as Box<dyn LineConsumer>,
        //     Box::new(StatusConsumer::default()) as Box<dyn LineConsumer>,
        // ]
    }

    pub fn eat_line(&mut self, log_line: LogLine) {
        for c in &mut self.consumers {
            c.eat_line(&log_line);
        }
        self.size += std::mem::size_of_val(&log_line);
        self.lines.push(log_line);
    }

    pub fn end_eating(&mut self) {
        for c in &mut self.consumers {
            c.end_eating();
        }
        println!("Size of lines[{}]: {} Mb", self.lines.len(), (self.size / 1024) / 1024);
    }

    pub fn get_data(&self) -> Map<String, Value> {
        let mut out = Map::new();
        for c in &self.consumers {
            out.extend(c.get_data());
        }
        out.insert("hits".into(), self.lines.len().into());
        out.extend(self.get_dates());
        out
    }

    fn get_dates(&self) -> Map<String, Value> {
        let mut out = Map::new();
        out.insert("date_start".into(), self.lines[0].date_time.to_string().into());
        out.insert("date_end".into(), self.lines[self.lines.len() - 1].date_time.to_string().into());
        out
    }

    pub fn reset(&mut self) {
        for c in &mut self.consumers {
            c.reset();
        }
    }
}