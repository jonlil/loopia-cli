use std::iter::FromIterator;
use serde::Serialize;

pub mod api;

#[derive(Debug, Serialize)]
pub struct ZoneRecords(Vec<ZoneRecord>);

impl ZoneRecords {
    pub fn new() -> Self {
        Self(vec![])
    }

    fn add(&mut self, elem: ZoneRecord) {
        self.0.push(elem);
    }
}

impl FromIterator<ZoneRecord> for ZoneRecords {
    fn from_iter<I: IntoIterator<Item=ZoneRecord>>(iter: I) -> Self {
        let mut c = ZoneRecords::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}

#[derive(Debug, Serialize)]
pub struct ZoneRecord {
    pub id: Option<i32>,
    pub data: String,
    pub ttl: i32,
    pub priority: i32,
    #[serde(rename(serialize = "type"))]
    pub type_: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_zone_records() {
        assert_eq!(
            true,
            serde_json::to_string(&ZoneRecords(vec![
                ZoneRecord {
                    id: Some(1234),
                    data: "spf".to_string(),
                    ttl: 300,
                    priority: 0,
                    type_: "TXT".to_string(),
                },
                ZoneRecord {
                    id: Some(1235),
                    data: "some_weird_value".to_string(),
                    ttl: 300,
                    priority: 0,
                    type_: "TXT".to_string(),
                },
            ])).is_ok(),
        );
    }
}
