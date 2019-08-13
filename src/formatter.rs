use std::io::{self};
use serde::Serialize;
use serde_json::Result;

pub struct JSONOutputFormatter;

impl JSONOutputFormatter {
    pub fn write<T: ?Sized>(value: &T) -> Result<()>
        where T: Serialize
    {
        serde_json::to_writer(io::stdout(), &value)
    }
}
