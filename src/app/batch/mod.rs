use std::process::Command;

use super::{BTAppState, utils::UIInfo};

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
pub fn handle_command(_ui_info: &mut UIInfo, state: &mut BTAppState, command: BatchCommand) -> Result<(), String> {
    if let Some(dir) = state.menu.projectdir.clone() {
        let full_rebuild_path = dir.join("full.rebuild.bat");
        return match command {
            BatchCommand::TestRunFullRebuild => match Command::new(full_rebuild_path).spawn() {
                Ok(_)  => Ok(()),
                Err(_) => Err(String::from("Command line could not be spawned."))
            }
        }
    }
    Ok(())
}

// ----------------------------------------------
// UI
// ----------------------------------------------
pub fn show(ui_info: &UIInfo, state: &BTAppState) -> Option<BatchCommand> {
    let mut result = None;

    egui::CentralPanel::default().show(ui_info.ctx, |ui| {
        ui.set_enabled(state.aux_windows.show.is_none());

        if let Some(projectdir) = &state.menu.projectdir {
            if ui.button("full.rebuild.bat").clicked() {
                result = Some(BatchCommand::TestRunFullRebuild);
            }
            ui.label(format!("Current directory…: '{:?}'", projectdir));
        };
    });

    result
}