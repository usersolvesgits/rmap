# RMAP
A bad version of *NMAP*, made with Rust.

> NOTE  
> 1) This project should not be taken seriously as it was made only for learning purposes only.  
> 2) All the functionalities were tested on a Windows 11 machine.

## Current Features
* Scan from IP address or URLS.
* Scan of TCP ports
  * Option to **set the first and last ports** you want to scan.
  * Option to set a **custom range** of ports you want to scan.
  * **Service recognition** for the most popular ports.
  * Option to set a **custom delay** between scans.
  * Option to print out **only** the open ports.
  * Option to print out **only** the closed ports.

## Planned changes
* Adding `UDP scan`.
* Exporting the last scan in different file formats `(.csv/.json)`.
* Lowering the time to scan each port.