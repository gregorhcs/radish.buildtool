use std::process::Command;

use crate::utils::{self, UIInfo};

use super::BTAppState;

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
pub fn handle_command(_ui_info: &mut UIInfo, state: &mut BTAppState, _command: BatchCommand) -> Result<(), String> {
    if let Some(dir) = state.menu.projectdir.clone() {
        println!("smpd {:?}", state.menu.projectdir);
        println!("dir1 {:?}", dir);
        let full_rebuild_path = dir.join("full.rebuild.bat");
        println!("dir2 {:?}", dir);
        println!("frpt {:?}", full_rebuild_path);
        let mut cmd = Command::new(full_rebuild_path);
        cmd.current_dir(dir);
        println!("curd {:?}", cmd.get_current_dir().unwrap().to_str());
        println!("curp {:?}", cmd.get_program().to_str());
        cmd.status().expect("failed to execute");
/*         return match command {
            BatchCommand::TestRunFullRebuild => match Command::new(full_rebuild_path).spawn() {
                Ok(child)  => {
                    Ok(())
                },
                Err(_) => Err(String::from("Command line could not be spawned."))
            }
        } */
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
            ui.label(format!("Current directory…: '{}'", utils::paths::pretty_print(projectdir)));
        };
    });

    result
}