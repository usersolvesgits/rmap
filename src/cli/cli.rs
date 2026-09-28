use crate::tcp::tcp_commands::TCPCommands;
use crate::udp::udp_commands::UDPCommands;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "rmap", long_about = None)]
pub struct Cli {
    /// The command to execute
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Clears the console.
    Clear {},
    /// Quits rmap
    Quit {},

    /// Scans TCP ports.
    /// Press `Alt + S` to stop the scan prematurely.
    TCP(TCPCommands),
    /// Scans UDP ports.  
    /// Press `Alt + S` to stop the scan prematurely.
    UDP(UDPCommands),
}