use crate::models::traits::CommandsAction;

use clap::Args;
use anyhow::Error;
use std::net::{IpAddr, TcpStream, SocketAddr};
use std::time::Duration;

#[derive(Args, Debug)]
pub struct TCPCommands {
    /// The ip address you want to scan. If none is specified, it will scan your local machine.
    #[arg(default_value = "127.0.0.1")]
    ip_address: String,

    /// The first port you want to scan from.
    #[arg(short, long, default_value = "0")]
    first_port: Option<u16>,

    /// The last port you want to scan.
    #[arg(short, long, default_value = "65535")]
    last_port: Option<u16>,

    /// The ranges of ports you want to scan (e.g.: 1-101).
    /// If a range is selected, eventual first-port and last-port flags are going to be ignored.
    #[arg(short, long, value_delimiter = '-')]
    port_range: Option<Vec<u16>>,

    /// Asks Rmap to wait at least the given amount of time (ms) between sending requests to the host.
    #[arg(short = 'd', long, default_value = "100")]
    scan_delay: Option<u64>,

    /// Shows only the open ports
    #[arg(short = 'o', long)]
    show_open: bool,

    /// Shows all the ports that aren't open.
    #[arg(short = 'c', long)]
    show_closed: bool,
}

impl CommandsAction for TCPCommands {
    fn run(&self) -> Result<(), Error> {
        let result_first_port: u16;
        let result_last_port: u16;

        let ip_target: IpAddr = self.ip_address.parse::<IpAddr>()?;
        let timeout: Duration = Duration::from_millis(self.scan_delay.unwrap());
        let show_all: bool = !self.show_open && !self.show_closed;

        match self.port_range.clone() {
            Some(port_range) => {
                result_first_port = port_range[0];
                result_last_port = port_range[1];
            }
            None => {
                result_first_port = self.first_port.unwrap();
                result_last_port = self.last_port.unwrap();
            }
        }

        println!("| Port Number |    | Status |");

        for port in result_first_port..=result_last_port {
            let socket_addr: SocketAddr = SocketAddr::new(ip_target, port);
            match TcpStream::connect_timeout(&socket_addr, timeout) {
                Ok(_) => {
                    if self.show_open || show_all {
                        println!("| {} |    | Open |", port);
                    }
                }
                Err(e) => {
                    if self.show_closed || show_all {
                        println!("| {} |    | Closed |", port);
                    }
                }
            }
        }
        Ok(())
    }
}