use tokio::net::UdpSocket;

use super::handler::Handler;
use super::publisher::Publisher;
use anyhow::Result;

pub struct Server {
    pub socket: UdpSocket,
    pub buf: Vec<u8>,
    pub handlers: Vec<Box<dyn Handler>>,
    pub publishers: Vec<Box<dyn Publisher>>,
}

impl Server {
    pub async fn run(self) -> Result<()> {
        let Server {
            socket,
            mut buf,
            handlers,
            publishers,
        } = self;

        loop {
            match socket.recv_from(&mut buf).await {
                Ok((size, addr)) => {
                    let buf_c = buf.clone();
                    let handlers_c = handlers.clone();
                    let publishers_c = publishers.clone();
                    tokio::spawn(async move {
                        for handler in handlers_c.iter() {
                            match handler.handle(&buf_c, size, addr) {
                                Ok(flowdatas) => {
                                    for publisher in publishers_c.iter() {
                                        match publisher.publish(&flowdatas) {
                                            Ok(_) => {}
                                            Err(e) => {
                                                eprintln!("{}", e);
                                            }
                                        }
                                    }
                                    break;
                                }
                                Err(e) => {
                                    eprintln!("{}", e);
                                }
                            }
                        }
                    });
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            };
        }
    }
}
