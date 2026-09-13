use anyhow::Error;
use std::net::IpAddr;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::Resolver;

pub trait CommandsAction {
    fn run(&self) -> Result<(), Error>;
}

pub fn check_url(target: &str) -> Option<IpAddr> {
    let resolver: Resolver = match Resolver::new(ResolverConfig::default(), ResolverOpts::default()) {
        Ok(resolver) => resolver,
        Err(_) => {
            println!("Failed to initialize resolver");
            return None
        },
    };
    let lookup_response = resolver.lookup_ip(target);

    let address = lookup_response.iter().next();
    match address {
        Some (lookup_ip) => {
            if let Some(ip) = lookup_ip.iter().next() {
                println!("{:?}", ip);
                Some(ip)
            } else {
                None
            }
        }
        None => {
            println!("Could not find a valid IP address to the url '{}'", target);
            None
        }
    }
}