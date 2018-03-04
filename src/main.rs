mod api;

use api::{
    ApiClient,
    GetZoneRecordsRequest
};

fn main() {
    let client = ApiClient::new(
        String::from("<username>"),
        String::from("<password>")
    );

    client.get_zone_records(&GetZoneRecordsRequest {
        domain: "example.com",
        subdomain: "@",
        customer_number: None
    });
}
