use clap::Arg;

pub fn id_argument<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("id")
        .long("id")
        .takes_value(true)
        .required(true)
}

pub fn domain_argument<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("domain")
        .long("domain")
        .takes_value(true)
        .required(true)
        .short("d")
}

pub fn subdomain_argument<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("subdomain")
        .long("subdomain")
        .default_value("@")
        .takes_value(true)
        .required(true)
        .short("s")
}

pub fn type_argument<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("type")
        .takes_value(true)
        .short("t")
        .default_value("A")
}

pub fn ttl_argument<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("ttl")
        .takes_value(true)
        .long("ttl")
        .default_value("300")
}

pub fn priority_argument<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("priority")
        .takes_value(true)
        .long("priority")
        .default_value("0")
        .short("p")
}
