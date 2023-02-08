use std::process::Command;

use super::BuildToolAppModel;

// ----------------------------------------------
// Model
// ----------------------------------------------
pub struct BatchExecModel {
    pub x: u32,
}

// ----------------------------------------------
// Commands
// ----------------------------------------------
pub enum BatchExecCommand {
    TestRunFullRebuild,
}

// ----------------------------------------------
pub fn handle_command_batch_exec(_ctx: &egui::Context, _frame: &mut eframe::Frame, command: BatchExecCommand, model: &mut BuildToolAppModel) {
    if let Some(dir) = model.menu.projectdir.clone() {
        let full_rebuild_path = dir.join("full.rebuild.bat");
        match command {
            BatchExecCommand::TestRunFullRebuild => Command::new(full_rebuild_path).spawn().expect("Command-line not spawning"),
        };
    }
}

// ----------------------------------------------
// UI
// ----------------------------------------------
pub fn show_batch_exec(ctx: &egui::Context, model: &mut BuildToolAppModel) -> Option<BatchExecCommand> {
    let mut result = None;

    egui::CentralPanel::default().show(ctx, |ui| {
        if let Some(projectdir) = &model.menu.projectdir {
            if ui.button("full.rebuild.bat").clicked() {
                result = Some(BatchExecCommand::TestRunFullRebuild);
            }
            ui.label(format!("Current directory…: '{:?}'", projectdir));
        };
    });

    result
}