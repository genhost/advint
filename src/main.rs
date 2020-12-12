// Main file. Implements user interaction using the advint library functions.

mod file;
mod intpr;
mod str;

// use std::env;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let config_file = &args[1];
    // let adventure_file = &args[2];

    let cfg_path = "config.json";
    let adv_path = "adventure.json";

    intpr::start(intpr::get(cfg_path, adv_path));
}
