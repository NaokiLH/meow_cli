#[macro_use]
extern crate clap;
use clap::App;
use meow_cli::prime::get_factor::get_factor;
fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();

    // Same as previous examples...
    match m.subcommand() {
        ("meow", Some(times)) => {
            let times = times.value_of("times").unwrap();
            let times: u32 = times.parse().unwrap();
            for i in 0..times {
                println!("meow! ");
            }
        }
        ("factor", Some(number)) => {
            let number = number.value_of("number").unwrap();
            let number: u32 = number.parse().unwrap();
            let list = get_factor(number);
            println!("{:?}", list);
        }
        _ => unreachable!(),
    }
}
