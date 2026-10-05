# RMAP
A bad version of *NMAP*, made with Rust.

> NOTE  
> 1) This project should not be taken seriously as it was made only for learning purposes only.  
> 2) All the functionalities were tested on a Windows 11 machine.

---

## Current Features
* Scan from IP address or URLS.
* Scan of **TCP ports**
  * Option to **set the first and last ports** you want to scan.
  * Option to set a **custom range** of ports you want to scan.
  * **Service recognition** for the most popular ports.
  * Option to set a **custom delay** between scans.
  * Option to print out **only** the open ports.
  * Option to print out **only** the closed ports.
* Scan of **UDP ports**
  * Option to **set the first and last ports** you want to scan.
  * Option to set a **custom range** of ports you want to scan.
  * **Service recognition** for the most popular ports.
  * Option to set a **custom delay** between scans.
  * Option to print out **only** the open ports.
  * Option to print out **only** the closed ports.
* Stop the scan at any time by pressing `Alt + S`.
* **Progress bar** to show the progress of the scan.

---

## Planned changes
* Exporting the last scan in different file formats `(.csv/.json)`.
* Lowering the time to scan each port.

---

## Known Bugs
* While scanning UDP ports, the program will block on port 137

---

## Crates Used
| Crate                                 | Version | Features |
|---------------------------------------| ------- | -------- |
| [anyhow](https://crates.io/crates/anyhow) | 1.0.104 | |
| [crossterm](https://crates.io/crates/crossterm) | 0.29.0 | |
| [clap](https://crates.io/crates/clap) | 4.6.4 | derive |
| [trust-dns-resolver](https://crates.io/crates/trust-dns-resolver) | 0.23.2 | |  
