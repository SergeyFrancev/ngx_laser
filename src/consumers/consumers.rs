use crate::{LineConsumer, LogLine, ResourceConsumer};

#[derive(Default)]
pub struct Consumers {
    consumers: Vec<Box<dyn LineConsumer>>,
    lines: Vec<LogLine>,
    size: usize,
}

impl Consumers {
    pub fn new() -> Self {
        let consumers: Vec<Box<dyn LineConsumer>> = Vec::from([
            Box::new(ResourceConsumer::default()) as Box<dyn LineConsumer>,
        ]);
        Self {
            consumers,
            lines: Vec::new(),
            size: 0,
        }
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

    pub fn get_data(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut out = serde_json::Map::new();
        for c in &self.consumers {
            out.extend(c.get_data());
        }
        out.insert("hits".into(), self.lines.len().into());
        out.extend(self.get_dates());
        out
    }

    fn get_dates(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut out = serde_json::Map::new();
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