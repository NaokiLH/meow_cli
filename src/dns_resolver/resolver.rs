use std::{net::IpAddr, str::FromStr};

use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    Resolver,
};
pub struct DnsWorker {
    resolver: Resolver,
    host: String,
}

impl DnsWorker {
    pub fn new(host: &str) -> Self {
        let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
            .expect("create resolver error");
        let host = String::from_str(host).unwrap();
        DnsWorker { resolver, host }
    }
    pub fn resolver(&self) -> Vec<IpAddr> {
        let mut ip_list = Vec::<IpAddr>::new();
        let response = self.resolver.lookup_ip(self.host.as_str()).unwrap();
        for i in response.iter() {
            ip_list.push(i);
        }
        ip_list
    }
}
