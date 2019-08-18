extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate xmlrpc;
extern crate reqwest;

use clap::{Arg, App, SubCommand};
use std::process;

mod loopia;
mod util;
mod formatter;
mod command;
mod config;
mod error;

use loopia::{
    ZoneRecord,
    api::{
        ApiClient,
        GetZoneRecordsRequest,
        AddZoneRecordRequest,
        UpdateZoneRecordRequest,
    }
};
use formatter::JSONOutputFormatter;
use config::Config;

fn main() {
    let config = Config::default();
    let client = ApiClient::new(
        config.username,
        config.password,
    );

    let matches = App::new("Loopia CLI")
        .about("Loopia CLI that wraps XMLRPC")
        .subcommand(SubCommand::with_name("get-zone-records")
            .arg(util::format_argument())
            .arg(Arg::with_name("domain")
                .index(1))
            .arg(Arg::with_name("subdomain")
                .index(2)))
        .subcommand(command::add_zone_record_command())
        .subcommand(command::update_zone_record_command())
        .get_matches();

    match matches.subcommand() {
        ("get-zone-records", Some(command)) => {
            let request = GetZoneRecordsRequest {
                domain: command.value_of("domain").unwrap(),
                subdomain: command.value_of("subdomain").unwrap_or("@"),
                customer_number: None
            };

            match client.get_zone_records(&request) {
                Ok(records) => {
                    if let Err(err) = JSONOutputFormatter::write(&records) {
                        eprintln!("Error: {:?}", err);
                        process::exit(1);
                    }
                },
                Err(err) => {
                    if let Err(err) = JSONOutputFormatter::write(&err) {
                        eprintln!("Error: {:?}", err);
                        process::exit(1);
                    } else {
                        process::exit(1);
                    }
                }
            }
       },

       ("add-zone-record", Some(command)) => {
           let request = AddZoneRecordRequest {
               customer_number: None,
               domain: command.value_of("domain").unwrap(),
               subdomain: command.value_of("subdomain").unwrap_or("@"),
               record: ZoneRecord {
                   id: None,
                   ttl: command.value_of("ttl").unwrap().parse::<i32>().unwrap(),
                   type_: command.value_of("type").unwrap().to_string(),
                   data: command.value_of("VALUE").unwrap().to_string(),
                   priority: command.value_of("priority").unwrap().parse::<i32>().unwrap()
               }
           };

           match client.add_zone_record(request) {
               Ok(_response) => {},
               Err(err) => eprintln!("{:#?}", err),
           };
       },

       ("update-zone-record", Some(command)) => {
           let request = UpdateZoneRecordRequest {
               customer_number: None,
               domain: command.value_of("domain").unwrap(),
               subdomain: command.value_of("subdomain").unwrap_or("@"),
               record: ZoneRecord {
                   id: Some(command.value_of("id").unwrap().parse::<i32>().unwrap()),
                   ttl: command.value_of("ttl").unwrap().parse::<i32>().unwrap(),
                   type_: command.value_of("type").unwrap().to_string(),
                   data: command.value_of("VALUE").unwrap().to_string(),
                   priority: command.value_of("priority").unwrap().parse::<i32>().unwrap()
               }
           };

           match client.update_zone_record(request) {
               Ok(_response) => {},
               Err(err) => eprintln!("{:#?}", err),
           };
       },
       _ => unreachable!()
    }
}
