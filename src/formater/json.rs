use serde_json::{Map, Value};

use crate::{Consumers, ResourceConsumer, StatusConsumer};

pub trait FormatVisitor {
    fn to_json(&self) -> serde_json::Map<String, serde_json::Value>;
}

// impl ToJson for Consumers {
//     fn to_json(&self) -> Map<String, Value> {
//         let mut out = Map::new();
//         for c in self.consumers {
//             out.extend(c.to_json());
//         }
//         out
//     }
// }

// pub fn to_json(consumers: &Vec<impl ToJson>) -> Map<String, Value> {
//     let mut out = Map::new();
//     for c in consumers {
//         out.extend(c.to_json());
//     }
//     out
// }

impl FormatVisitor for ResourceConsumer {
    fn to_json(&self) -> Map<String, Value> {
        let mut summary = serde_json::Map::new();
        summary.insert("count".into(), self.counter.into());

        let mut out = serde_json::Map::new();
        out.insert("resources".into(), summary.into());
        out
    }
}

impl FormatVisitor for StatusConsumer {
    fn to_json(&self) -> Map<String, Value> {
        let mut summary = serde_json::Map::new();
        let mut total: usize = 0;
        for (&k, &v) in self.counter.iter() {
            summary.insert(k.to_string(), v.to_string().into());
            total += v;
        }

        let mut out = serde_json::Map::new();
        out.insert("status".into(), summary.into());
        out
    }
}