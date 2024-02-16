use std::net::Ipv4Addr;

use clap::Parser;

/// 计算给定区间的太阳黄道经度、月亮黄道经度、新月黄道经度 api
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Server ip
    #[clap(short, long, value_parser, default_value = "0.0.0.0")]
    pub ip: Ipv4Addr,

    /// Server port
    #[clap(short, long, value_parser, default_value_t = 8080)]
    pub port: u16,

    /// thread num
    #[clap(short, value_parser, default_value_t = 1)]
    pub n: usize,
}
