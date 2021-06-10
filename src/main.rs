#[macro_use]
extern crate clap;
use clap::App;
use meow_cli::prime::get_factor::get_factor;
fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();

    // Same as previous examples...
    match m.value_of("meow") {
        Some(times) => {
            let t: i32 = times.parse().unwrap();
            for i in 0..t {
                print!("meow! ");
            }
            println!("");
        }
        _ => {
            println!("wrong input");
        }
    }
    match m.value_of("prime") {
        Some(number) => {
            let n: i32 = number.parse().unwrap();
            let list = get_factor(n);
            println!("{:?}", list);
        }
        _ => {
            println!("wrong input");
        }
    }
}
