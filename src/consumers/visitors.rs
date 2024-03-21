use std::collections::HashSet;

use crate::{LineConsumer, LogLine};

#[derive(Default)]
pub struct VisitorsConsumer {
    ips: HashSet<String>,
}

impl LineConsumer for VisitorsConsumer {
    fn eat_line(&mut self, log_line: &LogLine) {
        self.ips.insert(log_line.remote_addr.to_string());
    }

    fn get_data(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut summary = serde_json::Map::new();
        summary.insert("count".into(), self.ips.len().into());

        let mut out = serde_json::Map::new();
        out.insert("visitors".into(), summary.into());
        out
    }

    fn reset(&mut self) {
        self.ips.clear();
    }
}