#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![forbid(unsafe_code)]

use eframe::egui;

use clap::value_parser;
use std::path::PathBuf;

use w3_buildtool::app::BuildToolApp;
// ----------------------------------------------------------------------------

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const NAME: &str = "w3 build tool";
const ABOUT: &str = "Build tool for radish project templates.";
// ----------------------------------------------------------------------------

fn interactive_mode(projectdir: PathBuf) {
    let app_name = format!("{} v{}", NAME, VERSION.unwrap_or("unknown"));

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(440.0, 360.0)),
        ..Default::default()
    };

    eframe::run_native(
        &app_name,
        options,
        Box::new(|_cc| {
            Box::new(BuildToolApp::from(projectdir))
        }),
    );
}
// ----------------------------------------------------------------------------

fn start_main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let matches = clap::Command::new(NAME)
        .version(VERSION)
        .about(ABOUT)
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

    interactive_mode(
        matches
            .get_one::<PathBuf>("project-path")
            .unwrap_or(&PathBuf::default())
            .to_path_buf(),
    );
}
// ----------------------------------------------------------------------------

// ----------------------------------------------------------------------------
use std::process;
fn main() {
    start_main();
    process::exit(0);
}
// ----------------------------------------------------------------------------
