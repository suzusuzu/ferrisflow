use std::error::Error;
use tokio::net::UdpSocket;

use ferrisflow::handler::{Handler, NetflowV5Handler, NetflowV9Handler};
use ferrisflow::publisher::{CsvPublisher, JsonPublisher, PrintPublisher, Publisher};
use ferrisflow::server::Server;

use ferrisflow::opt::Opt;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let addr = format!("{}{}", "0.0.0.0:", opt.port);
    let socket = UdpSocket::bind(&addr).await?;
    eprintln!("Listening on: {}", socket.local_addr()?);

    let mut handlers: Vec<Box<dyn Handler>> = Vec::new();
    if opt.netflow_v5 {
        let netflow_v5_handler = Box::new(NetflowV5Handler::new());
        handlers.push(netflow_v5_handler);
    }
    if opt.netflow_v9 {
        let netflow_v9_handler = Box::new(NetflowV9Handler::new());
        handlers.push(netflow_v9_handler);
    }
    eprintln!(
        "handlers: [{}]",
        handlers
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    let mut publishers: Vec<Box<dyn Publisher>> = Vec::new();
    if opt.print {
        let print_publisher = Box::new(PrintPublisher::new());
        publishers.push(print_publisher);
    }
    if opt.json {
        let json_publisher = Box::new(JsonPublisher::new());
        publishers.push(json_publisher);
    }
    if opt.csv {
        let csv_publisher = Box::new(CsvPublisher::new(opt.header_none));
        publishers.push(csv_publisher);
    }
    eprintln!(
        "publishers: [{}]",
        publishers
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    let server = Server {
        socket: socket,
        buf: vec![0u8; 4096],
        handlers: handlers,
        publishers: publishers,
    };

    server.run().await?;

    Ok(())
}
