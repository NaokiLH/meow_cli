#[macro_use]
extern crate clap;
use clap::App;
use meow_cli::dns_resolver::resolver::DnsWorker;
use meow_cli::prime::get_factor::get_factor;
fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();

    match m.subcommand() {
        ("meow", Some(times)) => {
            let times = times.value_of("times").unwrap();
            let times: u32 = times.parse().unwrap();
            for _ in 0..times {
                println!("meow! ");
            }
        }
        ("factor", Some(number)) => {
            let number = number.value_of("number").unwrap();
            let number: u32 = number.parse().unwrap();
            let list = get_factor(number);
            println!("{:?}", list);
        }
        ("resolver", Some(host)) => {
            let host = host.value_of("host").unwrap();
            let dnsworker = DnsWorker::new(host);
            let ips = dnsworker.resolver();
            for i in ips {
                println!("{}", i);
            }
        }
        _ => unreachable!(),
    }
}
