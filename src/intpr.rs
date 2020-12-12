// Implements functions for interpreting configuration or adventure files.

use super::{file::ConfigFile, str};
use std::{thread, time::Duration};

// allows to transfer the necessary configuration files to the start function
pub fn get<'a>(cfg_path: &'static str, adv_path: &'static str) -> (ConfigFile<'a>, ConfigFile<'a>) {
    (ConfigFile::parse(cfg_path), ConfigFile::parse(adv_path))
}

pub fn start(configs: (ConfigFile, ConfigFile)) {
    let cfg = configs.0; // init config file
    let adv = configs.1; // init adventure file
    let mut pause: Vec<u64> = Vec::new(); // a vector that will contains pause breaks
    let adv_name = str::cut_json(
        // adventure name
        &adv.data["name"],
        &cfg.data["adventure"]["name"]["length"],
    );
    let adv_desc = str::cut_json(
        // adventure description
        &adv.data["description"],
        &cfg.data["adventure"]["description"]["length"],
    );

    if adv.data["variables"]["pause"].as_bool().unwrap_or(false) {
        for i in 0..adv.data["variables"]["pause"].len() - 1 {
            pause.push(
                adv.data["variables"]["pause"][i]
                    .as_u64()
                    .unwrap_or_default(),
            );
        }
    } else if cfg.data["adventure"]["pause"].as_bool().unwrap_or(false) {
        for i in 0..cfg.data["adventure"]["pause"].len() - 1 {
            pause.push(
                cfg.data["adventure"]["pause"][i]
                    .as_u64()
                    .unwrap_or_default(),
            );
        }
    } else {
        for i in 0..5 {
            pause.push(i);
        }
    }

    if cfg.data["adventure"]["show_start_message"]
        .as_bool()
        .unwrap_or(true)
    {
        println!(
            "{}",
            str::fmt(
                &adv_name,
                &cfg.data["adventure"]["name"]["format"]
                    .as_str()
                    .unwrap_or_default()
            )
        );
        thread::sleep(Duration::from_secs(pause[1]));
        println!(
            "{}",
            str::fmt(
                &adv_desc,
                &cfg.data["adventure"]["description"]["format"]
                    .as_str()
                    .unwrap_or_default()
            )
        );
    }
}
