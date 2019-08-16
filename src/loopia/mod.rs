use serde::Serialize;

pub mod api;
pub mod error;
mod transport;

use loopia::transport::LoopiaTransport as Transport;

#[derive(Debug, Serialize)]
pub struct ZoneRecords<'a> {
    domain: &'a str,
    subdomain: &'a str,
    records: Vec<ZoneRecord>
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
            serde_json::to_string(&ZoneRecords {
                domain: "example.com",
                subdomain: "www",
                records: vec![
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
                ]
            }).is_ok(),
        );
    }
}
