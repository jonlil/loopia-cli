#[macro_use]
extern crate dotenv_codegen;

mod api;

use api::{
    ApiClient,
    GetZoneRecordsRequest
};

fn main() {
    let client = ApiClient::new(
        String::from(dotenv!("LOOPIA_USERNAME")),
        String::from(dotenv!("LOOPIA_PASSWORD"))
    );

    client.get_zone_records(&GetZoneRecordsRequest {
        domain: "example.com",
        subdomain: "@",
        customer_number: None
    });
}
