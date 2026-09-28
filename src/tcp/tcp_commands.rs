use crate::models::utils::{CommandsAction, check_url};

use clap::Args;
use anyhow::Error;
use std::net::{IpAddr, TcpStream, SocketAddr};
use std::time::Duration;
use std::collections::HashMap;
use crossterm::event::{self, poll, Event, KeyCode, KeyModifiers};

#[derive(Args, Debug)]
pub struct TCPCommands {
    /// The ip address or url you want to scan. If none is specified, it will scan your local machine.
    #[arg(default_value = "127.0.0.1")]
    target: String,

    /// The first port you want to scan from.
    #[arg(short, long="fP", default_value = "1")]
    first_port: Option<u16>,

    /// The last port you want to scan.
    #[arg(short, long="lP", default_value = "65535")]
    last_port: Option<u16>,

    /// Scans only selected ports (e.g.: -s 22-23-80-443-).
    /// If it's selected, eventual first-port and last-port flags are going to be ignored.
    #[arg(short, long="sP", value_delimiter = '-')]
    selected_ports: Option<Vec<u16>>,

    /// The ranges of ports you want to scan (e.g.: -r 1-101).
    /// If a range is selected, eventual first-port, last-port and selected-ports flags are going to be ignored.
    #[arg(short, long, value_delimiter = '-')]
    range: Option<Vec<u16>>,

    /// Asks Rmap to wait at least the given amount of time (ms) between sending requests to the host.
    #[arg(short = 'd', long="delay", default_value = "50")]
    scan_delay: Option<u64>,

    /// Shows only the open ports
    #[arg(short = 'o', long="open")]
    show_open: bool,

    /// Shows all the ports that aren't open.
    #[arg(short = 'c', long="closed")]
    show_closed: bool,
}
impl CommandsAction for TCPCommands {
    fn run(&self) -> Result<(), Error> {
        let ip_target: IpAddr = match self.target.parse::<IpAddr>() {
            Ok(ip) => { ip }
            Err(_) => {
                match check_url(&self.target) {
                    Some(ip) => ip,
                    None => {
                        println!("ERROR: Error found while checking url!");
                        return Ok(())
                    }
                }
            }
        };

        let timeout: Duration = Duration::from_millis(self.scan_delay.unwrap());
        if timeout <= Duration::from_millis(0) {
            println!("Make sure the scan delay is higher than 0!");
            return Ok(())
        }

        let show_port_status: ShowPortStatus;
        if self.show_open && !self.show_closed {
            show_port_status = ShowPortStatus::ShowOpen;
        } else if self.show_closed && !self.show_open {
            show_port_status = ShowPortStatus::ShowClosed;
        } else {
            show_port_status = ShowPortStatus::ShowAll;
        }

        let tcp_services: HashMap<u16, &str> = HashMap::from([
            (20, "ftp-data"),
            (21, "ftp"),
            (22, "ssh"),
            (23, "telnet"),
            (25, "smtp"),
            (53, "domain"),
            (80, "http"),
            (88, "kerberos"),
            (110, "pop3"),
            (135, "epmap"),
            (139, "netbios-ssn"),
            (143, "imap"),
            (194, "irc"),
            (389, "ldap"),
            (443, "https"),
            (445, "microsoft-ds"),
            (465, "submissions"),
            (587, "submission"),
            (631, "ipp"),
            (636, "ldaps"),
            (873, "rsync"),
            (993, "imaps"),
            (995, "pop3s"),
            (1433, "ms-sql-s"),
            (1521, "oracle"),
            (1723, "pptp"),
            (3306, "mysql"),
            (3389, "ms-wbt-server"),
            (5432, "postgresql"),
            (5900, "vnc"),
            (6379, "redis"),
            (8080, "http-alt"),
            (27017, "mongodb"),
        ]);

        if let Some(selected_ports) = &self.selected_ports {
            println!("| Port Number |    | Status |    | Service |");

            'scan: for port in selected_ports {
                while poll(Duration::ZERO)? {
                    if let Event::Key(key) = event::read()? {
                        if key.code == KeyCode::Char('s') && key.modifiers == KeyModifiers::ALT {
                            println!("Stopping the scan...");
                            break 'scan;
                        }
                    }
                }
                scan_ports(ip_target, port.to_owned(), timeout, &tcp_services, &show_port_status);
            }
        } else {
            let first_port: u16;
            let last_port: u16;
            match self.range.clone() {
                Some(range) => {
                    first_port = range[0];
                    last_port = range[1];
                }
                None => {
                    first_port = self.first_port.unwrap();
                    last_port = self.last_port.unwrap();
                }
            }

            if first_port > last_port {
                println!("Make sure the first port is a lower number than the last port.");
                return Ok(())
            }

            if first_port <= 0 {
                println!("Make sure the first port is higher than 0!");
                return Ok(())
            }

            println!("| Port Number |    | Status |    | Service |");

            'scan: for port in first_port..=last_port {
                while poll(Duration::ZERO)? {
                    if let Event::Key(key) = event::read()? {
                        if key.code == KeyCode::Char('s') && key.modifiers == KeyModifiers::ALT {
                            println!("Stopping the scan...");
                            break 'scan;
                        }
                    }
                }
                scan_ports(ip_target, port, timeout, &tcp_services, &show_port_status);
            }
        }

        println!("Scan Terminated!");

        Ok(())
    }
}

enum PortStatus {
    Open,
    Closed
}
enum ShowPortStatus {
    ShowOpen,
    ShowClosed,
    ShowAll
}

fn scan_ports(
    ip_target: IpAddr,
    port: u16,
    timeout: Duration,
    services: &HashMap<u16, &str>,
    show_port_status: &ShowPortStatus
) {
    let socket_addr: SocketAddr = SocketAddr::new(ip_target, port);
    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(_) => {
            let status_port: PortStatus = PortStatus::Open;
            get_port_info(&services, port, &status_port, &show_port_status);
        }
        Err(_) => {
            let status_port: PortStatus = PortStatus::Closed;
            get_port_info(&services, port, &status_port, &show_port_status);
        }
    }
}

fn get_port_info(services: &HashMap<u16, &str>, port: u16, status_port: &PortStatus, show_status_port: &ShowPortStatus) {
    match services.get(&port) {
        Some(s) => {
            match show_status_port {
                ShowPortStatus::ShowOpen => {
                    match status_port {
                        PortStatus::Open => {
                            println!("| {} |    | Open |    | {} |", port, s);
                        }
                        _ => {
                            return
                        }
                    }
                },
                ShowPortStatus::ShowClosed => {
                    match status_port {
                        PortStatus::Closed => {
                            println!("| {} |    | Closed |    | {} |", port, s);
                        }
                        _ => {
                            return
                        }
                    }
                },
                ShowPortStatus::ShowAll => {
                    match status_port {
                        PortStatus::Open => {
                            println!("| {} |    | Open |    | {} |", port, s);
                        }
                        PortStatus::Closed => {
                            println!("| {} |    | Closed |    | {} |", port, s);
                        }
                    }
                }
            }
        }
        None => {
            match show_status_port {
                ShowPortStatus::ShowOpen => {
                    match status_port {
                        PortStatus::Open => {
                            println!("| {} |    | Open |    | --- |", port);
                        }
                        _ => {
                            return
                        }
                    }
                },
                ShowPortStatus::ShowClosed => {
                    match status_port {
                        PortStatus::Closed => {
                            println!("| {} |    | Closed |    | --- |", port);
                        }
                        _ => {
                            return
                        }
                    }
                },
                ShowPortStatus::ShowAll => {
                    match status_port {
                        PortStatus::Open => {
                            println!("| {} |    | Open |    | --- |", port);
                        }
                        PortStatus::Closed => {
                            println!("| {} |    | Closed |    | -- |", port);
                        }
                    }
                }
            }
        }
    }
}