use crate::{LineConsumer, LogLine};

#[derive(Default)]
pub struct ResourceConsumer {
    pub counter: usize,
}

impl ResourceConsumer {
    fn is_resource(&self, log_line: &LogLine) -> bool {
        let s = &log_line.path;
        s.ends_with(".png")
            || s.ends_with(".css")
            || s.ends_with(".svg")
            || s.ends_with(".jpg")
            || s.ends_with(".jpeg")
            || s.ends_with(".gif")
            || s.ends_with(".ico")
            || s.ends_with(".js")
            || s.ends_with(".woff2")
            || s.ends_with(".webp")
    }
}

impl LineConsumer for ResourceConsumer {
    fn eat_line(&mut self, log_line: &LogLine) {
        if self.is_resource(&log_line) {
            self.counter += 1;
        }
    }

    fn get_data(&self) -> serde_json::Map<String, serde_json::Value> {
        let mut out = serde_json::Map::new();
        out.insert("resources_count".into(), self.counter.into());
        out
    }

    fn reset(&mut self) {
        self.counter = 0;
    }
}
