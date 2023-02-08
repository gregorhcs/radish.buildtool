pub mod comp_menu;
pub mod comp_batch_exec;
pub mod utils;

// ----------------------------------------------
use std::path::PathBuf;

use comp_menu::*;
use comp_batch_exec::*;
use utils::radish_dir_check;

// ----------------------------------------------
// Model
// ----------------------------------------------
pub struct BuildToolAppModel {
    pub menu: MenuModel,
}

// ----------------------------------------------
// Commands
// ----------------------------------------------
enum BuildToolAppCommand {
    Menu(MenuCommand),
    BatchExec(BatchExecCommand),
}

// ----------------------------------------------
// App
// ----------------------------------------------
pub struct BuildToolApp {
    model: BuildToolAppModel,
}

// ----------------------------------------------
impl From<PathBuf> for BuildToolApp {
    fn from(filepath: PathBuf) -> BuildToolApp {
        BuildToolApp { model: BuildToolAppModel { menu: MenuModel { projectdir: radish_dir_check(filepath) } } }
    }
}

// ----------------------------------------------
impl eframe::App for BuildToolApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {

            let mut commands = Vec::new();

            if let Some(command) = show_menu(ctx) {
                commands.push(BuildToolAppCommand::Menu(command));
            }
            if let Some(command) = show_batch_exec(ctx, &mut self.model) {
                commands.push(BuildToolAppCommand::BatchExec(command));
            }
            
            for command in commands {
                match command {
                    BuildToolAppCommand::Menu(sub_command)      => handle_command_menu(      ctx, frame, sub_command, &mut self.model),
                    BuildToolAppCommand::BatchExec(sub_command) => handle_command_batch_exec(ctx, frame, sub_command, &mut self.model),
                }
            }

        });
    }
}
// ----------------------------------------------
