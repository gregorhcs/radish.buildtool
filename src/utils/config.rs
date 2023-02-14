use std::{fs::File, path::PathBuf};

use ini::{Ini, Properties};

// ----------------------------------------------------------------------------
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
pub const NAME: &str = "w3 build tool";
pub const ABOUT: &str = "Build tool for radish project templates.";

const CONFIG_FILEPATH: &str = "conf.ini";
const CONFIG_SECTION_PROJECTDIRS: Option<&str> = Some("Recent Project Directories");

const CONFIG_SLOT_LAST: u8 = 7; // slots: 1..CONFIG_SLOT_LAST

// ----------------------------------------------------------------------------
// gui versioning info
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
// config file operations
// ----------------------------------------------------------------------------
pub fn ensure_conf_exists() {
    if !PathBuf::from(CONFIG_FILEPATH).exists() {
        File::create(CONFIG_FILEPATH).expect("Config file could not be created");
    }
}

// ----------------------------------------------------------------------------
pub fn load_recent_projectpaths() -> Vec<PathBuf> {
    let config = Ini::load_from_file(CONFIG_FILEPATH).unwrap();

    // load Vec<(slot, path)> from config
    let mut slots_paths: Vec<(i32, &str)> = match config.section(CONFIG_SECTION_PROJECTDIRS) {
        Some(section) => section
            .iter()
            .map(|(k, v)| (k.parse::<i32>().unwrap_or(i32::MAX), v))
            .collect(),
        None => Vec::default(),
    };

    // sort by slot
    slots_paths.sort_by(|t1, t2| t1.0.cmp(&t2.0));

    // map to path bufs
    slots_paths.iter().map(|t| PathBuf::from(t.1)).collect()
}

// ----------------------------------------------------------------------------
pub fn load_most_recent_projectpath() -> Option<PathBuf> {
    let mut recent_paths = load_recent_projectpaths();
    if recent_paths.is_empty() {
        return None;
    }
    Some(recent_paths.remove(0))
}

// ----------------------------------------------------------------------------
pub fn set_most_recent_projectpath(projectpath: &PathBuf) {
    let mut config = Ini::load_from_file(CONFIG_FILEPATH).unwrap();

    let new_value = projectpath
        .to_str()
        .expect("Config file new projectpath to str conversion failed");

    let potential_key = config
        .section(CONFIG_SECTION_PROJECTDIRS)
        .unwrap_or(&Properties::new())
        .iter()
        .find(|p| String::from(p.1).eq(new_value))
        .map(|p| String::from(p.0));

    // if projectpath already saved: shiftup instead of add new key

    match potential_key {
        Some(old_key) => shiftup_recent_projectpath(&mut config, &old_key),
        None => new_recent_projectpath(&mut config, new_value),
    }

    config
        .write_to_file(CONFIG_FILEPATH)
        .expect("Config file writing did not work out");
}

// ----------------------------------------------------------------------------
pub fn clear_recent_projectpaths() {
    let mut config = Ini::load_from_file(CONFIG_FILEPATH).unwrap();

    config.delete(CONFIG_SECTION_PROJECTDIRS);

    config
        .write_to_file(CONFIG_FILEPATH)
        .expect("Config file writing did not work out");

    //for slot in 1..=CONFIG_SLOTS {
    //    config.delete_from(CONFIG_SECTION_PROJECTDIRS, &slot.to_string());
    //}
}

// ----------------------------------------------------------------------------
// config file operations (internal)
// ----------------------------------------------------------------------------
fn new_recent_projectpath(config: &mut Ini, new_value: &str) {
    // iterating from slot CONFIG_SLOT_LAST (oldest)
    // to 1 (were new_value is to be saved)
    for slot in (1..=CONFIG_SLOT_LAST).rev() {
        let key = slot.to_string();
        let next_key = (slot - 1).to_string();

        // delete older path from this slot
        config.delete_from(CONFIG_SECTION_PROJECTDIRS, &key);

        // skip iteration if there's nothing to set to this slot
        if config
            .get_from(CONFIG_SECTION_PROJECTDIRS, &next_key)
            .is_none()
            && slot != 1
        {
            continue;
        }

        // set next recent path to this slot
        let value = if slot == 1 {
            new_value
        } else {
            config
                .get_from(CONFIG_SECTION_PROJECTDIRS, &next_key)
                .unwrap()
        };
        config.set_to(CONFIG_SECTION_PROJECTDIRS, key, value.into());
    }
}

// ----------------------------------------------------------------------------
fn shiftup_recent_projectpath(config: &mut Ini, old_key: &str) {
    let old_key_i32 = old_key.parse::<i32>().unwrap();

    // value already in the right place
    if old_key_i32 == 1 {
        return;
    }

    // set old slot free and save value
    let new_most_recent_value = config
        .delete_from(CONFIG_SECTION_PROJECTDIRS, old_key)
        .unwrap();

    // shift all values above the old slot one down
    for slot in (1..old_key_i32).rev() {
        let shift_value = config
            .delete_from(CONFIG_SECTION_PROJECTDIRS, &slot.to_string())
            .unwrap();

        config.set_to(
            CONFIG_SECTION_PROJECTDIRS,
            (slot + 1).to_string(),
            shift_value,
        );
    }

    // set value to most recent slot
    config.set_to(
        CONFIG_SECTION_PROJECTDIRS,
        "1".to_string(),
        new_most_recent_value,
    );
}
