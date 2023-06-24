use futures::{
    task::{FutureObj, SpawnError},
    FutureExt,
};
use handler::{DnsRequestError, DnsRequestHandler};
// You can run this example from the root of the mio repo:
// cargo run --example udp_server --features="os-poll net"
use futures::task::Spawn;
use log::{info, warn};
use mio::{Events, Interest, Poll, Token};
use response::{DnsRecordData, DnsResourceRecord, DnsResponse, DnsResponseHeader};
use server::DnsServer;
use std::{io, net::IpAddr, sync::Arc};
use tokio::runtime::Runtime;

use crate::request::DNSRequest;

mod handler;
mod label;
mod request;
mod resourcerecord;
mod response;
mod server;

// A token to allow us to identify which event is for the `UdpSocket`.
const UDP_SOCKET: Token = Token(0);
const TCP_SOCKET: Token = Token(1);

fn main() -> io::Result<()> {
    env_logger::init();

    let mut runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let handle = runtime.handle().clone();
    let dns = DnsServer::new(HandlerImplementation);
    let future = dns.run();
    runtime.block_on(future)?;

    Ok(())
}

struct HandlerImplementation;

impl DnsRequestHandler for HandlerImplementation {
    fn handle_request(
        self: Arc<Self>,
        request: DNSRequest,
    ) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<DnsResponse, DnsRequestError>> + Send>>
    {
        Box::pin(async move {
            let response = DnsResponse {
                header: DnsResponseHeader {
                    id: request.header.id,
                    flags: 0x8180,
                    qdcount: request.header.qdcount,
                    ancount: 1,
                    nscount: 0,
                    arcount: 0,
                },
                questions: request.questions.clone(),
                answers: vec![DnsResourceRecord {
                    name: request.questions[0].qname.clone(),
                    rtype: request.questions[0].qtype,
                    class: request.questions[0].qclass,
                    ttl: 0,
                    rdata: DnsRecordData::A(std::net::Ipv4Addr::new(1, 1, 1, 1)),
                }],
                additional: vec![],
                authority: vec![],
            };

            Ok(response)
        })
    }
}
