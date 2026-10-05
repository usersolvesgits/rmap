use std::io::ErrorKind;
use crate::models::utils::{CommandsAction, check_url};

use clap::Args;
use anyhow::Error;
use std::net::{IpAddr, UdpSocket, SocketAddr};
use std::time::Duration;
use std::collections::HashMap;
use crossterm::event::{self, poll, Event, KeyCode, KeyModifiers};
use indicatif::ProgressBar;

#[derive(Args, Debug)]
pub struct UDPCommands {
    /// The ip address or url you want to scan. If none is specified, it will scan your local machine.
    #[arg(short, long, default_value = "127.0.0.1")]
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
impl CommandsAction for UDPCommands {
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

        let udp_services: HashMap<u16, &str> = HashMap::from([
            (53, "domain"),
            (69, "tftp"),
            (88, "kerberos"),
            (135, "epmap"),
            (137, "netbios-ns"),
            (138, "netbios-dgm"),
            (389, "ldap"),
            (631, "ipp"),
            (1701, "l2tp"),
        ]);


        if let Some(selected_ports) = &self.selected_ports {
            println!("| Port Number |    | Status |    | Service |");

            let port_range = selected_ports.iter().copied();
            let pb: ProgressBar = ProgressBar::new(selected_ports.len() as u64);

            scan_loop(port_range, ip_target, timeout,
                      &udp_services, &show_port_status, &pb)?;
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

            let port_range = first_port..=last_port;
            let pb: ProgressBar = ProgressBar::new(port_range.len() as u64);

            scan_loop(port_range, ip_target, timeout,
                      &udp_services, &show_port_status, &pb)?;
        }

        println!("Scan Terminated!");

        Ok(())
    }
}

enum PortStatus {
    Open,
    Closed,
    PermissionDenied,
    TimedOut,
    Error
}
enum ShowPortStatus {
    ShowOpen,
    ShowClosed,
    ShowAll
}

fn scan_loop(range_ports: impl Iterator<Item = u16>,
             ip_target: IpAddr,
             timeout: Duration,
             services: &HashMap<u16, &str>,
             show_port_status: &ShowPortStatus,
             pb: &ProgressBar) -> Result<(), Error> {

    for port in range_ports {
        while poll(Duration::ZERO)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('s') && key.modifiers == KeyModifiers::ALT {
                    pb.println("Stopping the scan...");
                    pb.abandon_with_message("Scan aborted!");
                    return Ok(())
                }
            }
        }
        scan_ports(ip_target, port, timeout, services, show_port_status, pb);
        pb.inc(1);
    }
    pb.finish_with_message("Scan complete!");

    Ok(())
}

fn scan_ports(
    ip_target: IpAddr,
    port: u16,
    timeout: Duration,
    services: &HashMap<u16, &str>,
    show_port_status: &ShowPortStatus,
    pb: &ProgressBar
) {
    let socket: SocketAddr = SocketAddr::new(ip_target, port);

    let udp_socket: UdpSocket = match UdpSocket::bind(socket) {
        Ok(s) => s,
        Err(_) => {
            println!("ERROR: Could not bind UDP socket!");
            return
        }
    };

    match udp_socket.connect(socket) {
        Ok(_) => { }
        Err(_) => {
            println!("ERROR: Could not connect to UDP socket!");
            return
        }
    }

    let mut send_data: [u8; 2048] = [0; 2048];
    if let Err(_) = udp_socket.send(&mut send_data) {
        println!("ERROR: Could not send data to UDP socket!");
        return
    }

    let mut receive_data: [u8; 2048] = [0; 2048];
    match udp_socket.recv(&mut receive_data) {
        Ok(_) => {
            let status_port: PortStatus = PortStatus::Open;
            get_port_info(&services, port, &status_port, &show_port_status, pb);
        }
        Err(e) => {
            let status_port: PortStatus = match e.kind() {
                ErrorKind::PermissionDenied => PortStatus::PermissionDenied,
                ErrorKind::ConnectionRefused => PortStatus::Closed,
                ErrorKind::TimedOut => PortStatus::TimedOut,
                _ => PortStatus::Error,
            };
            get_port_info(&services, port, &status_port, &show_port_status, pb);
        }
    }

    std::thread::sleep(timeout);
}

fn get_port_info(services: &HashMap<u16, &str>, port: u16, status_port: &PortStatus,
                 show_status_port: &ShowPortStatus, pb: &ProgressBar) {
    match services.get(&port) {
        Some(s) => {
            match show_status_port {
                ShowPortStatus::ShowOpen => {
                    match status_port {
                        PortStatus::Open => {
                            pb.println(format!("| {} |    | Open |    | {} |", port, s));
                        }
                        _ => {
                            return
                        }
                    }
                },
                ShowPortStatus::ShowClosed => {
                    match status_port {
                        PortStatus::PermissionDenied => {
                            pb.println(format!("| {} |    | PERMISSION DENIED |    | {} |", port, s));
                        }
                        PortStatus::Closed => {
                            pb.println(format!("| {} |    | Closed |    | {} |", port, s));
                        }
                        PortStatus::TimedOut => {
                            pb.println(format!("| {} |    | CONNECTION TIMEDOUT |    | {} |", port, s));
                        }
                        PortStatus::Error => {
                            pb.println(format!("| {} |    | ERROR |    | {} |", port, s));
                        }
                        _ => {
                            return
                        }
                    }
                },
                ShowPortStatus::ShowAll => {
                    match status_port {
                        PortStatus::Open => {
                            pb.println(format!("| {} |    | Open |    | {} |", port, s));
                        }
                        PortStatus::PermissionDenied => {
                            pb.println(format!("| {} |    | PERMISSION DENIED |    | {} |", port, s));
                        }
                        PortStatus::Closed => {
                            pb.println(format!("| {} |    | Closed |    | {} |", port, s));
                        }
                        PortStatus::TimedOut => {
                            pb.println(format!("| {} |    | CONNECTION TIMEDOUT |    | {} |", port, s));
                        }
                        PortStatus::Error => {
                            pb.println(format!("| {} |    | ERROR |    | {} |", port, s));
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
                            pb.println(format!("| {} |    | Open |    | --- |", port));
                        }
                        _ => {
                            return
                        }
                    }
                },

                ShowPortStatus::ShowClosed => {
                    match status_port {
                        PortStatus::PermissionDenied => {
                            pb.println(format!("| {} |    | PERMISSION DENIED |    | --- |", port));
                        }
                        PortStatus::Closed => {
                            pb.println(format!("| {} |    | Closed |    | --- |", port));
                        }
                        PortStatus::TimedOut => {
                            pb.println(format!("| {} |    | CONNECTION TIMEDOUT |    | --- |", port));
                        }
                        PortStatus::Error => {
                            pb.println(format!("| {} |    | ERROR |    | --- |", port));
                        }
                        _ => {
                            return
                        }
                    }
                },

                ShowPortStatus::ShowAll => {
                    match status_port {
                        PortStatus::Open => {
                            pb.println(format!("| {} |    | Open |    | --- |", port));
                        }
                        PortStatus::PermissionDenied => {
                            pb.println(format!("| {} |    | PERMISSION DENIED |    | --- |", port));
                        }
                        PortStatus::Closed => {
                            pb.println(format!("| {} |    | Closed |    | --- |", port));
                        }
                        PortStatus::TimedOut => {
                            pb.println(format!("| {} |    | CONNECTION TIMEDOUT |    | --- |", port));
                        }
                        PortStatus::Error => {
                            pb.println(format!("| {} |    | ERROR |    | --- |", port));
                        }
                    }
                }
            }
        }
    }
}