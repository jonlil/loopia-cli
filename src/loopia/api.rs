use std::collections::BTreeMap;
use reqwest::{Url, Client};
use xmlrpc::{Value, Request};
use super::{ZoneRecord, ZoneRecords, Transport};
use super::error::{Error, LoopiaErrorKind};

pub struct ApiClient {
    username: String,
    password: String,
    url: Url,
}

impl From<ZoneRecord> for BTreeMap<String, Value> {
    fn from(zone_record: ZoneRecord) -> BTreeMap<String, Value> {
        let mut record_data = BTreeMap::new();

        if let Some(id) = zone_record.id {
            record_data.insert("record_id".to_string(), Value::from(id));
        }

        record_data.insert("rdata".to_string(), Value::from(zone_record.data));
        record_data.insert("ttl".to_string(), Value::from(zone_record.ttl));
        record_data.insert("priority".to_string(), Value::from(zone_record.priority));
        record_data.insert("type".to_string(), Value::from(zone_record.type_));

        record_data
    }
}

#[derive(Debug)]
pub struct GetZoneRecordsRequest<'a> {
    pub customer_number: Option<&'a str>,
    pub domain: &'a str,
    pub subdomain: &'a str,
}

#[derive(Debug)]
pub struct AddZoneRecordRequest<'a> {
    pub customer_number: Option<&'a str>,
    pub domain: &'a str,
    pub subdomain: &'a str,
    pub record: ZoneRecord,
}

#[derive(Debug)]
pub struct UpdateZoneRecordRequest<'a> {
    pub customer_number: Option<&'a str>,
    pub domain: &'a str,
    pub subdomain: &'a str,
    pub record: ZoneRecord,
}

fn customer_number<'a>(customer_number: Option<&'a str>) -> Value {
    match customer_number.is_none() {
        false => Value::from(customer_number.unwrap()),
        true => Value::from(String::new())
    }
}

impl<'a> ApiClient {
    const API_URL: &'a str = "https://api.loopia.se/RPCSERV";

    pub fn new(username: String, password: String) -> Self {
        let api_url = Url::parse(Self::API_URL).unwrap();

        ApiClient {
            url: api_url,
            username: username,
            password: password,
        }
    }

    fn request<'b>(&self, procedure: &'b str) -> Request<'b> {
        Request::new(procedure)
            .arg(Value::from(self.username.to_owned()))
            .arg(Value::from(self.password.to_owned()))
    }

    fn send(&self, request: Request) -> Result<Value, Error> {
        let tp = Transport::new(Client::new().post(self.url.to_owned()));

        match request.call(tp) {
            Ok(response) => parse_response(response),
            Err(_err) => Err(Error::new(LoopiaErrorKind::RuntimeError)),
        }
    }

    pub fn get_zone_records(&self, parameters: &'a GetZoneRecordsRequest) -> Result<ZoneRecords<'a>, Error> {
        let request = self.request("getZoneRecords")
            .arg(customer_number(parameters.customer_number).to_owned())
            .arg(Value::from(parameters.domain))
            .arg(Value::from(parameters.subdomain));

        match self.send(request) {
            Ok(Value::Array(records)) => {
                let zrs = records
                    .iter()
                    .map(|record| {
                        ZoneRecord {
                            id: Some(record["record_id"].as_i32().unwrap()),
                            data: record["rdata"].as_str().unwrap().to_string(),
                            ttl: record["ttl"].as_i32().unwrap(),
                            priority: record["priority"].as_i32().unwrap(),
                            type_: record["type"].as_str().unwrap().to_string(),
                        }
                    })
                    .collect::<Vec<ZoneRecord>>();

                Ok(ZoneRecords {
                    domain: parameters.domain,
                    subdomain: parameters.subdomain,
                    records: zrs,
                })
            },
            _ => Err(Error::new(LoopiaErrorKind::UnknownError)),
        }
    }

    pub fn add_zone_record(&self, parameters: AddZoneRecordRequest) -> Result<(), &'static str> {
        let request = self.request("addZoneRecord")
            .arg(customer_number(parameters.customer_number).to_owned())
            .arg(Value::from(parameters.domain))
            .arg(Value::from(parameters.subdomain))
            .arg(Value::Struct(BTreeMap::from(parameters.record)));

        match self.send(request) {
            Ok(res) => {
                eprintln!("Successfully created zoneRecord: {:#?}", res);
                Ok(())
            },
            Err(err) => {
                eprintln!("Failed creating zoneRecord: {:#?}", err);
                Err("Failed creating zoneRecord")
            }
        }
    }

    pub fn update_zone_record(&self, parameters: UpdateZoneRecordRequest) -> Result<Value, &'static str> {
        let request = self.request("updateZoneRecord")
            .arg(customer_number(parameters.customer_number).to_owned())
            .arg(Value::from(parameters.domain))
            .arg(Value::from(parameters.subdomain))
            .arg(Value::Struct(BTreeMap::from(parameters.record)));

        match self.send(request) {
            Ok(res) => Ok(res),
            Err(_err) => Err("Failed to update_zone_record"),
        }
    }
}

fn parse_response(response: xmlrpc::Value) -> Result<Value, Error> {
    match &response {
        Value::Array(val) if val.len() == 1 => {
            match &val[0] {
                Value::String(val) => {
                    match val.as_ref() {
                        "AUTH_ERROR" => Err(Error::new(LoopiaErrorKind::AuthenticationError)),
                        "UNKNOWN_ERROR" => Err(Error::new(LoopiaErrorKind::UnknownError)),
                        _ => Ok(response),
                    }
                },
                _ => Ok(response),
            }
        },
        _ => Ok(response),
    }
}

#[cfg(test)]
mod test {
    use xmlrpc::Value;
    use super::*;

    #[test]
    fn test_unknown_error() {
        assert_eq!(
            Err(Error::new(LoopiaErrorKind::UnknownError)),
            parse_response(Value::Array(vec![
                Value::String("UNKNOWN_ERROR".to_string())
            ])),
        );
    }

    #[test]
    fn test_auth_error() {
        assert_eq!(
            Err(Error::new(LoopiaErrorKind::AuthenticationError)),
            parse_response(Value::Array(vec![
                Value::String("AUTH_ERROR".to_string())
            ])),
        );
    }

    #[test]
    fn test_valid_string_response() {
        assert_eq!(
            Ok(Value::Array(vec![Value::String("SOME_RANDOM_VALUE".to_string())])),
            parse_response(Value::Array(vec![
                Value::String("SOME_RANDOM_VALUE".to_string())
            ])),
        );
    }

    #[test]
    fn test_zone_records() {
        let mut hash_map: BTreeMap<String, Value> = BTreeMap::new();

        hash_map.insert("priority".to_string(), Value::Int(0));
        hash_map.insert("rdata".to_string(), Value::String("OA9WYZglUMNqzRKcIX78zdxhdj1Z_M3zj8nZb8w2iys".to_string()));
        hash_map.insert("record_id".to_string(), Value::Int(39065987));
        hash_map.insert("ttl".to_string(), Value::Int(300));
        hash_map.insert("type".to_string(), Value::String("TXT".to_string()));

        let parsed_response = Value::Array(vec![
            Value::Struct(hash_map),
        ]);
        assert_eq!(Ok(parsed_response.clone()), parse_response(parsed_response));
    }
}
