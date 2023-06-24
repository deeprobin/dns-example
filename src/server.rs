use log::{error, info, warn};
use tokio::{net::UdpSocket, runtime::Runtime, sync::mpsc};

use crate::{handler::DnsRequestHandler, request::DNSRequest};
use futures::task::{Spawn, SpawnExt};
use std::{io, net::SocketAddr, sync::Arc};

pub struct DnsServer<H: DnsRequestHandler> {
    handler: Arc<H>,
}

impl<H: DnsRequestHandler> DnsServer<H> {
    pub fn new(handler: H) -> DnsServer<H> {
        DnsServer {
            handler: Arc::new(handler),
        }
    }

    pub async fn run(&self) -> tokio::io::Result<()> {
        let socket = UdpSocket::bind("0.0.0.0:54").await?;
        let r = Arc::new(socket);
        let s = r.clone();
        let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);

        let handler = self.handler.clone();
        tokio::spawn(async move {
            while let Some((buf_data, peer_address)) = rx.recv().await {
                let dns_request = DNSRequest::parse(&buf_data);
                match dns_request {
                    Ok(dns_request) => {
                        info!("Received DNS request: {:?}", dns_request);

                        let handler_clone = handler.clone();
                        let handler_response = handler_clone.handle_request(dns_request).await;

                        match handler_response {
                            Ok(response) => {
                                info!("Sending DNS response: {:?}", response);
                                let response_bytes = response.to_bytes();
                                match response_bytes {
                                    Ok(response_bytes) => {
                                        s.send_to(&response_bytes[..], peer_address)
                                            .await
                                            .expect("Failed to send response");
                                    }
                                    Err(e) => {
                                        error!("Error converting response to bytes: {:?}", e);
                                    }
                                }
                            }
                            Err(e) => error!("Error handling request: {:?}", e),
                        }
                    }
                    Err(e) => {
                        info!("Error parsing DNS request: {:?}", e);
                    }
                }
            }
        });

        let mut buf = [0; 1 << 16];
        loop {
            let (len, addr) = r.recv_from(&mut buf).await?;
            println!("{:?} bytes received from {:?}", len, addr);
            tx.send((buf[..len].to_vec(), addr)).await.unwrap();
        }

        Ok(())
    }

    /*pub async fn run(&self, executor: &mut dyn Spawn) -> io::Result<()> {
        let mut poll = Poll::new()?;

        // Create storage for events. Since we will only register a single socket, a
        // capacity of 1 will do.
        let mut events = Events::with_capacity(1);
        let addr = "0.0.0.0:54".parse().unwrap();
        let mut socket = UdpSocket::bind(addr)?;

        // Register our socket with the token defined above and an interest in being
        // `READABLE`.
        poll.registry()
            .register(&mut socket, UDP_SOCKET, Interest::READABLE)?;

        // Initialize a buffer for the UDP packet. We use the maximum size of a UDP
        // packet, which is the maximum value of 16 a bit integer.
        let mut buf = [0; 1 << 16];

        loop {
            // Poll to check if we have events waiting for us.
            poll.poll(&mut events, None)?;

            // Process each event.
            for event in events.iter() {
                // Validate the token we registered our socket with,
                // in this example it will only ever be one but we
                // make sure it's valid none the less.
                match event.token() {
                    UDP_SOCKET => loop {
                        // In this loop we receive all packets queued for the socket.
                        match socket.recv_from(&mut buf) {
                            Ok((packet_size, source_address)) => {
                                let buf_data = &buf[..packet_size];
                                let dns_request = DNSRequest::parse(buf_data);

                                match dns_request {
                                    Ok(dns_request) => {
                                        info!("Received DNS request: {:?}", dns_request);
                                        let handler_response =
                                            self.handler.handle_request(&dns_request).await;

                                        match handler_response {
                                            Ok(response) => {
                                                info!("Sending DNS response: {:?}", response);
                                                let response_bytes = response.to_bytes();
                                                match response_bytes {
                                                    Ok(response_bytes) => {
                                                        socket.send_to(
                                                            &response_bytes[..],
                                                            source_address,
                                                        )?;
                                                    }
                                                    Err(e) => {
                                                        error!("Error converting response to bytes: {:?}", e);
                                                    }
                                                }
                                            }
                                            Err(e) => error!("Error handling request: {:?}", e),
                                        }
                                    }
                                    Err(e) => {
                                        info!("Error parsing DNS request: {:?}", e);
                                    }
                                }
                                println!("Received packet from: {}", source_address);

                                socket.send_to(&buf[..packet_size], source_address)?;
                            }
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                // If we get a `WouldBlock` error we know our socket
                                // has no more packets queued, so we can return to
                                // polling and wait for some more.
                                break;
                            }
                            Err(e) => {
                                // If it was any other kind of error, something went
                                // wrong and we terminate with an error.
                                return Err(e);
                            }
                        }
                    },
                    _ => {
                        // This should never happen as we only registered our
                        // `UdpSocket` using the `UDP_SOCKET` token, but if it ever
                        // does we'll log it.
                        warn!("Got event for unexpected token: {:?}", event);
                    }
                }
            }
        }
    }*/
}
