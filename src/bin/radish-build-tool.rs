#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![forbid(unsafe_code)]
// ----------------------------------------------------------------------------

use eframe::{egui, IconData};

use clap::value_parser;
use std::path::PathBuf;

use w3_buildtool::{gui::BTApp, utils::config};

// ----------------------------------------------------------------------------
fn interactive_mode(projectdir: PathBuf) {
    let app_name = format!("{} v{}", config::NAME, config::VERSION.unwrap_or("unknown"));

    let img = image::open("assets/icon.radish.png").expect("Icon not found!");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(440.0, 360.0)),
        icon_data: Some(IconData{ 
            rgba:   img.to_rgba8().to_vec(), 
            width:  img.width(), 
            height: img.height() 
        }),
        ..Default::default()
    };

    eframe::run_native(
        &app_name,
        options,
        Box::new(|_cc| Box::new(BTApp::from(projectdir))),
    );
}

// ----------------------------------------------------------------------------
fn start_main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let matches = clap::Command::new(config::NAME)
        .version(config::VERSION)
        .about(config::ABOUT)
        .arg(
            clap::Arg::new("project-path")
                .long("project-path")
                .short('p')
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    if matches.get_one::<String>("help").is_some() {
        return;
    }

    config::ensure_conf_exists();
    config::load_recent_projectpaths();

    interactive_mode(
        config::load_most_recent_projectpath()
            .unwrap_or(
                matches
                .get_one::<PathBuf>("project-path")
                .unwrap_or(&PathBuf::default())
                .to_path_buf()
            )
    );
}

// ----------------------------------------------------------------------------
use std::process;
fn main() {
    start_main();
    process::exit(0);
}

// ----------------------------------------------------------------------------
