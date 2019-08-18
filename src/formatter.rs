use std::io::{self};
use serde_json::Result;
use serde::Serialize;

pub struct JSONOutputFormatter;

impl JSONOutputFormatter {
    pub fn write<T: ?Sized>(value: &T) -> Result<()>
        where T: Serialize
    {
        serde_json::to_writer(io::stdout(), &value)
    }
}
