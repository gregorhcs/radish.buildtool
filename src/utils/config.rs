use std::{path::PathBuf, fs::File};

use ini::Ini;

// ----------------------------------------------------------------------------
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
pub const NAME: &str = "w3 build tool";
pub const ABOUT: &str = "Build tool for radish project templates.";

const CONFIG_FILEPATH: &str = "conf.ini";
const CONFIG_SECTION_PROJECTDIRS: Option<&str> = Some("Recent Project Directories");

// ----------------------------------------------------------------------------
pub fn gui_title(projectdir: &Option<PathBuf>) -> String {
    
    let mut app_name = format!("{} v{}", NAME, VERSION.unwrap_or("unknown"));

    if let Some(dir) = projectdir {
        app_name.push_str(" - ");
        app_name.push_str(dir.iter().next_back().unwrap().to_str().unwrap())
    }
    app_name

}

// ----------------------------------------------------------------------------

// public

pub fn ensure_conf_exists() {
    if !PathBuf::from(CONFIG_FILEPATH).exists() {
        File::create(CONFIG_FILEPATH).expect("Config file could not be created");
    }
}

pub fn load_recent_projectpaths() -> Vec<PathBuf> {
    let config = Ini::load_from_file(CONFIG_FILEPATH).unwrap();

    let mut key_values: Vec<(i32, &str)> = 
        match config.section(CONFIG_SECTION_PROJECTDIRS) {
            Some(section) => section
                .iter()
                .map(|(k,v)| (k.parse::<i32>().unwrap_or(i32::MAX), v))
                .collect(),
            None => Vec::new(),
        };
    key_values.sort_by(|(t1,_),(t2,_)| t1.cmp(t2));

    key_values.iter().map(|(_,v)| PathBuf::from(v)).collect::<Vec<PathBuf>>().clone()
}

pub fn load_most_recent_projectpath() -> Option<PathBuf> {
    let mut recent_paths = load_recent_projectpaths();
    if recent_paths.is_empty() {
        return None
    }
    let fp = recent_paths.remove(0);
    Some(fp)
}

pub fn set_most_recent_projectpath(projectpath: &PathBuf) {
    let mut config = Ini::load_from_file(CONFIG_FILEPATH).unwrap();

    let new_value = projectpath.to_str().expect("Config file new projectpath to str conversion failed");

    // if projectpath already saved: shiftup instead of add new key

    let potential_key = config
            .section(CONFIG_SECTION_PROJECTDIRS)
            .unwrap()
            .iter()
            .collect::<Vec<(&str,&str)>>()
            .iter()
            .find(|p| String::from(p.1).eq(new_value))
            .map(|p| String::from(p.0));

    if let Some(old_key) = potential_key {
        shiftup_recent_projectpath(&mut config, &old_key);
    }
    else {
        new_recent_projectpath(&mut config, new_value);
    }

    config.write_to_file(CONFIG_FILEPATH).expect("Config file writing did not work out");
}


fn new_recent_projectpath(config: &mut Ini, new_value: &str) {
    
    // iterating from slot 7 (oldest) 
    // to 1 (were new_value is to be saved)
    for slot in (1..=7).rev() {

        let key = slot.to_string();
        let next_key = (slot - 1).to_string();

        // delete older path from this slot
        config.delete_from(CONFIG_SECTION_PROJECTDIRS, &key);

        // skip iteration if there's nothing to set to this slot
        if config.get_from(CONFIG_SECTION_PROJECTDIRS, &next_key).is_none() && slot != 1 {
            continue;
        }

        // set next recent path to this slot
        let value = 
            if slot == 1 
                { new_value } 
            else 
                { config.get_from(CONFIG_SECTION_PROJECTDIRS, &next_key).unwrap() };
        config.set_to(CONFIG_SECTION_PROJECTDIRS, key, value.into());

    } 

}


fn shiftup_recent_projectpath(config: &mut Ini, old_key: &str) {

    let old_key_i32 = old_key.parse::<i32>().unwrap();
    
    // value already in the right place
    if old_key_i32 == 1 {
        return;
    }

    // set old slot free and save value
    let new_most_recent_value = config.delete_from(CONFIG_SECTION_PROJECTDIRS, old_key).unwrap();

    // shift all values above the old slot one down
    for slot in (1..old_key_i32).rev() {

        let shift_value = config.delete_from(CONFIG_SECTION_PROJECTDIRS, &slot.to_string()).unwrap();

        config.set_to(CONFIG_SECTION_PROJECTDIRS, (slot + 1).to_string(), shift_value);

    }

    // set value to most recent slot
    config.set_to(CONFIG_SECTION_PROJECTDIRS, String::from("1"), new_most_recent_value);

}