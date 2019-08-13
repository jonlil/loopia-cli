use clap::{SubCommand, App, Arg};

use util;

pub fn add_zone_record_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("add-zone-record")
        .arg(util::domain_argument())
        .arg(util::subdomain_argument())
        .arg(util::type_argument())
        .arg(util::ttl_argument())
        .arg(util::priority_argument())
        .arg(Arg::with_name("VALUE").required(true))
}

pub fn update_zone_record_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("update-zone-record")
        .arg(util::id_argument())
        .arg(util::domain_argument())
        .arg(util::subdomain_argument())
        .arg(util::type_argument())
        .arg(util::ttl_argument())
        .arg(util::priority_argument())
        .arg(Arg::with_name("VALUE").required(true))
}
