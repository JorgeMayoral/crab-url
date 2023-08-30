use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use clap::Parser;

mod instrumentation;
mod logger;

#[derive(Parser)]
pub struct Cli {
    #[clap(long, env = "BIND", default_value_t = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000))]
    pub bind: SocketAddr,
    #[clap(flatten)]
    pub instrumentation: instrumentation::Instrumentation,
}
