extern crate xmlrpc;
extern crate reqwest;

use xmlrpc::{Request, Transport};
use xmlrpc::http::{build_headers, check_response};

use reqwest::RequestBuilder;

use std::error::Error;

pub struct LoopiaTransport(RequestBuilder);
impl LoopiaTransport {
    pub fn new(rb: RequestBuilder) -> LoopiaTransport {
        LoopiaTransport(rb)
    }
}
impl Transport for LoopiaTransport {
    type Stream = reqwest::Response;

    fn transmit(self, request: &Request) -> Result<Self::Stream, Box<dyn Error + Send + Sync>> {
        let mut body = Vec::new();
        request.write_as_xml(&mut body).expect("could not write request to buffer (this should never happen)");

        let response = build_headers(self.0, body.len() as u64)
            .body(body)
            .send()?;

        check_response(&response)?;

        Ok(response)
    }
}
