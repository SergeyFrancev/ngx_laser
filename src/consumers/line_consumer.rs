use serde_json::Map;

use crate::{Date, LogLine};

pub trait LineConsumer {
    fn start_eating(&mut self, _first_date: Date) {}
    fn eat_line(&mut self, log_line: &LogLine);
    fn end_eating(&mut self) {}
    fn get_data(&self) -> Map<String, serde_json::Value> {
        Map::new()
    }
    fn reset(&mut self) {}
}
