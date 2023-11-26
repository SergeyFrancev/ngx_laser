use crate::{LineConsumer, LogLine};

#[derive(Default)]
pub struct ResourceConsumer {
    counter: usize,
}

impl LineConsumer for ResourceConsumer {
    fn eat_line(&mut self, log_line: &LogLine) {
        if log_line.is_resource() {
            self.counter += 1;
        }
    }

    fn get_data(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut out = serde_json::Map::new();
        out.insert("resources_count".into(), self.counter.into());
        out
    }
}