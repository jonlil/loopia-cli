use std::collections::BTreeMap;
use reqwest::Url;
use xmlrpc::{Value, Request};
use super::{ZoneRecord, ZoneRecords};

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

    fn xml_client<'b>(&self, endpoint: &'b str) -> Request<'b> {
        Request::new(endpoint)
            .arg(Value::from(self.username.to_owned()))
            .arg(Value::from(self.password.to_owned()))
    }

    pub fn get_zone_records(&self, parameters: &'a GetZoneRecordsRequest) -> Result<ZoneRecords<'a>, String> {
        let xml_client = self.xml_client("getZoneRecords")
            .arg(customer_number(parameters.customer_number).to_owned())
            .arg(Value::from(parameters.domain))
            .arg(Value::from(parameters.subdomain));

        match xml_client.call_url(self.url.to_owned()) {
            Ok(res) => {
                if let Some(result) = res.as_array() {
                    let records = result.iter().map(|x| {
                        ZoneRecord {
                            id: Some(x["record_id"].as_i32().unwrap()),
                            data: x["rdata"].as_str().unwrap().to_string(),
                            ttl: x["ttl"].as_i32().unwrap(),
                            priority: x["priority"].as_i32().unwrap(),
                            type_: x["type"].as_str().unwrap().to_string()
                        }
                    })
                    .collect::<Vec<ZoneRecord>>();

                    Ok(ZoneRecords {
                        domain: parameters.domain,
                        subdomain: parameters.subdomain,
                        records: records,
                    })
                } else {
                    panic!("Failed executing: getZoneRecords")
                }
            },
            Err(err) => {
                eprintln!("Error: {:#?}", err);
                Err(format!("Failed fetching getZoneRecords: {:?}", err))
            }
        }
    }

    fn call(&self, client: Request) -> Result<Value, xmlrpc::Error> {
        match client.call_url(self.url.to_owned()) {
            Ok(res) => {
                match &res {
                    Value::String(value) => {
                        if value == &"AUTH_ERROR".to_string() {
                            panic!("Authentication Error");
                        } else {
                            Ok(res)
                        }
                    },
                    _ => Ok(res),
                }
            },
            Err(err) => Err(err),
        }
    }

    pub fn add_zone_record(&self, parameters: AddZoneRecordRequest) -> Result<(), &'static str> {
        let xml_client = self.xml_client("addZoneRecord")
            .arg(customer_number(parameters.customer_number).to_owned())
            .arg(Value::from(parameters.domain))
            .arg(Value::from(parameters.subdomain))
            .arg(Value::Struct(BTreeMap::from(parameters.record)));

        match self.call(xml_client) {
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
        let xml_client = self.xml_client("updateZoneRecord")
            .arg(customer_number(parameters.customer_number).to_owned())
            .arg(Value::from(parameters.domain))
            .arg(Value::from(parameters.subdomain))
            .arg(Value::Struct(BTreeMap::from(parameters.record)));

        match self.call(xml_client) {
            Ok(res) => Ok(res),
            Err(_err) => Err("Failed to update_zone_record"),
        }
    }
}
