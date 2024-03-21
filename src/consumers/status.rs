use std::collections::HashMap;

use crate::{LineConsumer, LogLine};

#[derive(Default)]
pub struct StatusConsumer {
    pub counter: HashMap<u16, usize>,
}

impl LineConsumer for StatusConsumer {
    fn eat_line(&mut self, log_line: &LogLine) {
        *self.counter.entry(log_line.status).or_insert(0) += 1;
    }

    fn get_data(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut summary = serde_json::Map::new();
        for (&k, &v) in self.counter.iter() {
            summary.insert(k.to_string(), v.into());
        }

        let mut out = serde_json::Map::new();
        out.insert("status".into(), summary.into());
        out
    }
}