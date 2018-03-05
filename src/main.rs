extern crate clap;
use clap::{Arg, App, SubCommand};

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

    let matches = App::new("Loopia CLI")
        .about("Loopia CLI that wraps XMLRPC")
        .subcommand(SubCommand::with_name("domain")
            .subcommand(SubCommand::with_name("get-zone-records")
                .arg(Arg::with_name("domain")
                     .index(1))
                .arg(Arg::with_name("subdomain")
                     .index(2)))
            ).get_matches();

    match matches.subcommand() {
        ("domain", Some(domain_matches)) => {
            match domain_matches.subcommand() {
                ("get-zone-records", Some(command)) => {
                    client.get_zone_records(&GetZoneRecordsRequest {
                        domain: command.value_of("domain").unwrap(),
                        subdomain: command.value_of("subdomain").unwrap_or("@"),
                        customer_number: None
                    });
                },
                _ => unreachable!()
            }
        },
        ("", None) => {},
        _ => unreachable!(),
    }
}
