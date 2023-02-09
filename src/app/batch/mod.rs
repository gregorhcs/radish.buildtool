use std::process::Command;

use super::{BTAppState, utils::GUIInfo};

// ----------------------------------------------
// Model
// ----------------------------------------------
#[derive(Default)]
pub struct BatchState {
    pub x: u32,
}

// ----------------------------------------------
// Commands
// ----------------------------------------------
pub enum BatchCommand {
    TestRunFullRebuild,
}

// ----------------------------------------------
pub fn handle_command(_gui_info: &mut GUIInfo, state: &mut BTAppState, command: BatchCommand) {
    if let Some(dir) = state.menu.projectdir.clone() {
        let full_rebuild_path = dir.join("full.rebuild.bat");
        match command {
            BatchCommand::TestRunFullRebuild => Command::new(full_rebuild_path).spawn().expect("Command-line not spawning"),
        };
    }
}

// ----------------------------------------------
// UI
// ----------------------------------------------
pub fn show(gui_info: &GUIInfo, state: &BTAppState) -> Option<BatchCommand> {
    let mut result = None;

    egui::CentralPanel::default().show(gui_info.ctx, |ui| {
        if let Some(projectdir) = &state.menu.projectdir {
            if ui.button("full.rebuild.bat").clicked() {
                result = Some(BatchCommand::TestRunFullRebuild);
            }
            ui.label(format!("Current directory…: '{:?}'", projectdir));
        };
    });

    result
}