extern crate clap;
use clap::{Arg, App, SubCommand};

#[macro_use]
extern crate dotenv_codegen;

mod api;
mod util;

use api::{
    ApiClient,
    GetZoneRecordsRequest,
    ZoneRecord,
    AddZoneRecordRequest,
    UpdateZoneRecordRequest,
};
use util::*;

fn main() {
    let client = ApiClient::new(
        String::from(dotenv!("LOOPIA_USERNAME")),
        String::from(dotenv!("LOOPIA_PASSWORD"))
    );

    let add_zone_record_command = SubCommand::with_name("add-zone-record")
        .arg(domain_argument())
        .arg(subdomain_argument())
        .arg(type_argument())
        .arg(ttl_argument())
        .arg(priority_argument())
        .arg(Arg::with_name("VALUE").required(true));

    let update_zone_record_command = SubCommand::with_name("update-zone-record")
        .arg(id_argument())
        .arg(domain_argument())
        .arg(subdomain_argument())
        .arg(type_argument())
        .arg(ttl_argument())
        .arg(priority_argument())
        .arg(Arg::with_name("VALUE").required(true));

    let matches = App::new("Loopia CLI")
        .about("Loopia CLI that wraps XMLRPC")
        .subcommand(SubCommand::with_name("get-zone-records")
            .arg(Arg::with_name("domain")
                .index(1))
            .arg(Arg::with_name("subdomain")
                .index(2)))
        .subcommand(add_zone_record_command)
        .subcommand(update_zone_record_command)
        .get_matches();

    match matches.subcommand() {
        ("get-zone-records", Some(command)) => {
            let request = GetZoneRecordsRequest {
                domain: command.value_of("domain").unwrap(),
                subdomain: command.value_of("subdomain").unwrap_or("@"),
                customer_number: None
            };

            for record in client.get_zone_records(&request) {
                println!("{:#?}", record);
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
