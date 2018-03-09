extern crate xmlrpc;
extern crate reqwest;

use self::reqwest::{
    Url,
};

use self::xmlrpc::{
    Value,
    Request,
};

pub struct ApiClient {
    username: String,
    password: String,
    url: Url,
}

#[derive(Debug)]
pub struct ZoneRecord {
    id: i32,
    data: String,
    ttl: i32,
    priority: i32,
    type_: String,
}

pub struct GetZoneRecordsRequest<'a> {
    pub customer_number: Option<&'a str>,
    pub domain: &'a str,
    pub subdomain: &'a str,
}

impl<'a> GetZoneRecordsRequest<'a> {
    fn customer_number(&self) -> Value {
        return match self.customer_number.is_none() {
            false => Value::from(self.customer_number.unwrap()),
            true => Value::from(String::new())
        };
    }
}

impl ApiClient {
    pub fn new(username: String, password: String) -> Self {
        let api_url = Url::parse("https://api.loopia.se/RPCSERV").unwrap();

        ApiClient {
            url: api_url,
            username: username,
            password: password,
        }
    }

    pub fn get_zone_records<'a >(
        &self,
        parameters: &GetZoneRecordsRequest
    ) -> Vec<ZoneRecord> {
        let xml_client = Request::new("getZoneRecords")
            .arg(Value::from(self.username.to_owned()))
            .arg(Value::from(self.password.to_owned()))
            .arg(parameters.customer_number().to_owned())
            .arg(Value::from(parameters.domain))
            .arg(Value::from(parameters.subdomain));

        match xml_client.call_url(self.url.to_owned()) {
            Ok(res) => {
                let mut zone_records: Vec<ZoneRecord> = vec![];
                res.as_array().map(|x| {
                    for x in x.iter() {
                        zone_records.push(ZoneRecord {
                            id: x["record_id"].as_i32().unwrap(),
                            data: x["rdata"].as_str().unwrap().to_string(),
                            ttl: x["ttl"].as_i32().unwrap(),
                            priority: x["priority"].as_i32().unwrap(),
                            type_: x["type"].as_str().unwrap().to_string()
                        });
                    }
                }).into_iter();
                return zone_records;

            },
            Err(err) => {
                eprintln!("Error: {:#?}", err);
                return vec![];
            }
        }
    }
}
