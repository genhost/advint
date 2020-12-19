// Implements functions for interpreting configuration or adventure files.

use super::{file::ConfigFile, fmt, str};
use std::{thread, time::Duration};

// allows to transfer the necessary configuration files to the start function
pub fn get<'a>(cfg_path: &'static str, adv_path: &'static str) -> (ConfigFile<'a>, ConfigFile<'a>) {
    (ConfigFile::parse(cfg_path), ConfigFile::parse(adv_path))
}

fn init_adv_info(adv: &ConfigFile, cfg: &ConfigFile) -> (String, String) {
    (
        str::cut_json(
            // adventure name
            &adv.data["name"],
            &cfg.data["adventure"]["name"]["length"],
        ),
        str::cut_json(
            // adventure description
            &adv.data["description"],
            &cfg.data["adventure"]["description"]["length"],
        ),
    )
}

fn init_intervals(cfg: &ConfigFile, adv: &ConfigFile) -> Vec<u64> {
    let mut ints: Vec<u64> = Vec::new();

    if !adv.data["variables"]["intervals"].is_empty() {
        for i in 0..adv.data["variables"]["intervals"].len() {
            ints.push(
                adv.data["variables"]["intervals"][i]
                    .as_u64()
                    .unwrap_or_default(),
            );
        }
    } else if !cfg.data["adventure"]["intervals"].is_empty() {
        for i in 0..cfg.data["adventure"]["intervals"].len() {
            ints.push(
                cfg.data["adventure"]["intervals"][i]
                    .as_u64()
                    .unwrap_or_default(),
            );
        }
    } else {
        for i in 0..6 {
            ints.push(i);
        }
    }

    ints
}

fn pause(seconds: u64) {
    thread::sleep(Duration::from_secs(seconds));
}

fn display_adv_info(cfg: &ConfigFile, adv_name: &str, adv_desc: &str, int_between: u64) {
    if cfg.data["adventure"]["display_adventure_info"]
        .as_bool()
        .unwrap_or(true)
    {
        println!(
            "{}",
            fmt::simple_fmt(
                &adv_name,
                &cfg.data["adventure"]["name"]["format"]
                    .as_str()
                    .unwrap_or_default()
            )
        );
        pause(int_between);
        println!(
            "{}",
            fmt::simple_fmt(
                &adv_desc,
                &cfg.data["adventure"]["description"]["format"]
                    .as_str()
                    .unwrap_or_default()
            )
        );
    }
}

fn load_scripts(cfg: &ConfigFile, adv: &ConfigFile) {
    for i in 0..adv.data["loading_order"].len() {
        start_script(&cfg, &adv, adv.data["loading_order"][i].as_str().unwrap());
    }
}

fn start_script(cfg: &ConfigFile, adv: &ConfigFile, name: &str) {
    let global_script = &adv.data["scripts"][name];
    for (script_key, script_val) in global_script.entries() {
        if cfg.data["adventure"]["display_adventure_script_name"]
            .as_bool()
            .unwrap_or(false)
        {
            println!("Script: {}\n", fmt::match_get_cmd(script_key, &adv.data["varuables"]));
        }
        for (action_key, action_val) in script_val.entries() {}
    }
}

pub fn start(configs: (ConfigFile, ConfigFile)) {
    let (cfg, adv) = (configs.0, configs.1); // init config and adventure files
    let (adv_name, adv_desc) = init_adv_info(&adv, &cfg); // init adventure name and description
    let ints: Vec<u64> = init_intervals(&cfg, &adv); // a vector containing the interval values in seconds to suspend the interpreter

    display_adv_info(&cfg, &adv_name, &adv_desc, ints[0]);
    load_scripts(&cfg, &adv);
}
