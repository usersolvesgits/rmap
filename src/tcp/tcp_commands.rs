use crate::models::utils::{CommandsAction, check_url};

use clap::Args;
use anyhow::Error;
use std::net::{IpAddr, TcpStream, SocketAddr};
use std::time::Duration;
use phf::phf_map;

#[derive(Args, Debug)]
pub struct TCPCommands {
    /// The ip address or url you want to scan. If none is specified, it will scan your local machine.
    #[arg(default_value = "127.0.0.1")]
    target: String,

    /// The first port you want to scan from.
    #[arg(short, long, default_value = "0")]
    first_port: Option<u16>,

    /// The last port you want to scan.
    #[arg(short, long, default_value = "65535")]
    last_port: Option<u16>,

    /// The ranges of ports you want to scan (e.g.: -r 1-101).
    /// If a range is selected, eventual first-port and last-port flags are going to be ignored.
    #[arg(short, long, value_delimiter = '-')]
    range: Option<Vec<u16>>,

    /// Asks Rmap to wait at least the given amount of time (ms) between sending requests to the host.
    #[arg(short = 'd', long, default_value = "50")]
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

        let ip_target: IpAddr = match self.target.parse::<IpAddr>() {
            Ok(ip) => { ip }
            Err(_) => {
                match check_url(&self.target) {
                    Some(ip) => ip,
                    None => {
                        return Ok(())
                    }
                }
            }
        };
        let timeout: Duration = Duration::from_millis(self.scan_delay.unwrap());
        let show_all: bool = !self.show_open && !self.show_closed;

        match self.range.clone() {
            Some(range) => {
                result_first_port = range[0];
                result_last_port = range[1];
            }
            None => {
                result_first_port = self.first_port.unwrap();
                result_last_port = self.last_port.unwrap();
            }
        }

        if result_first_port > result_last_port {
            println!("Make sure the first port is a lower number than the last port.");
            return Ok(())
        }

        if timeout <= Duration::from_millis(0) {
            println!("Make sure the scan delay is higher than 0!");
            return Ok(())
        }

        const SERVICES: phf::Map<u16, &str> = phf_map! {
            20u16 => "ftp-data",
            21u16 => "ftp",
            22u16 => "ssh",
            23u16 => "telnet",
            25u16 => "smtp",
            53u16 => "domain",
            80u16 => "http",
            88u16 => "kerberos",
            110u16 => "pop3",
            135u16 => "epmap",
            137u16 => "netbios-ns",
            138u16 => "netbios-dgm",
            139u16 => "netbios-ssn",
            143u16 => "imap",
            194u16 => "irc",
            389u16 => "ldap",
            443u16 => "https",
            445u16 => "microsoft-ds",
            465u16 => "submissions",
            587u16 => "submission",
            631u16 => "ipp",
            636u16 => "ldaps",
            873u16 => "rsync",
            993u16 => "imaps",
            995u16 => "pop3s",
            1433u16 => "ms-sql-s",
            1521u16 => "oracle",
            1701u16 => "l2tp",
            1723u16 => "pptp",
            3306u16 => "mysql",
            3389u16 => "ms-wbt-server",
            5432u16 => "postgresql",
            5900u16 => "vnc",
            6379u16 => "redis",
            8080u16 => "http-alt",
            27017u16 => "mongodb",
        };

        println!("| Port Number |    | Status |    | Service |");

        for port in result_first_port..=result_last_port {
            let socket_addr: SocketAddr = SocketAddr::new(ip_target, port);
            match TcpStream::connect_timeout(&socket_addr, timeout) {
                Ok(_) => {
                    if self.show_open || show_all {
                        match SERVICES.get(&port) {
                            Some(s) => {
                                println!("| {} |    | Open |    | {} |", port, s);
                            }
                            None => {
                                println!("| {} |    | Open |    | --- |", port);
                            }
                        }
                    }
                }
                Err(_) => {
                    if self.show_closed || show_all {
                        match SERVICES.get(&port) {
                            Some(s) => {
                                println!("| {} |    | Closed |    | {} |", port, s);
                            }
                            None => {
                                println!("| {} |    | Closed |    | --- |", port);
                            }
                        }
                    }
                }
            }
        }

        println!("Scan Terminated!");

        Ok(())
    }
}