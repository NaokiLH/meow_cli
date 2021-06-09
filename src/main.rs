#[macro_use]
extern crate clap;
use clap::App;
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
            "wrong input";
        }
    }
}
