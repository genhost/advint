// Main file. Implements user interaction using the advint library functions.

mod cmd;
mod file;
mod fmt;
mod intpr;
mod str;

// use std::env;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let (cfg_path, adv_path) = (&args[1], &args[2]);

    let (cfg_path, adv_path) = ("config.json", "adventure.json");

    intpr::start(intpr::get(cfg_path, adv_path));
}
