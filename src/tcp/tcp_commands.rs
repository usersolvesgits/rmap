use crate::models::traits::CommandsAction;

use clap::Args;
use anyhow::Error;

#[derive(Args, Debug)]
pub struct TCPCommands {
    /// The ip address you want to scan. If none is specified, it will scan localhost.
    #[arg(short, long, default_value = "127.0.0.1")]
    ip_address: String,

    /// The first port you want to scan from.
    #[arg(short, long, default_value = "0")]
    first_port: Option<u16>,

    /// The last port you want to scan.
    #[arg(short, long, default_value = "65535")]
    last_port: Option<u16>,

    /// The ranges of ports you want to scan (e.g.: 1-101).
    #[arg(short, long)]
    port_range: Option<String>,

    /// Asks Rmap to wait at least the given amount of time (ms) between sending requests to the host.
    #[arg(short, long, default_value = "100")]
    scan_delay: Option<u32>
}

impl CommandsAction for TCPCommands {
    fn run(&self) -> Result<(), Error> {
        todo!()
    }
}