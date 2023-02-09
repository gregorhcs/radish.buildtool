mod utils;

// ----------------------------------------------
use std::path::PathBuf;

use crate::app::{BTAppState, utils::GUIInfo};

// ----------------------------------------------
// Model
// ----------------------------------------------
pub struct MenuState {
    pub projectdir: Option<PathBuf>,
}

// ----------------------------------------------
impl From<PathBuf> for MenuState {
    fn from(filepath: PathBuf) -> MenuState {
        MenuState { 
            projectdir: utils::radish_dir_check(filepath)
        }
    }
}

// ----------------------------------------------
// Commands
// ----------------------------------------------
pub enum MenuCommand {
    OpenProject(Option<PathBuf>),
    Quit,
}

// ----------------------------------------------
pub fn handle_command(gui_info: &mut GUIInfo, state: &mut BTAppState, command: MenuCommand) {
    match command {
        MenuCommand::OpenProject(projectdir) => state.menu.projectdir = projectdir,
        MenuCommand::Quit                    => gui_info.frame.close(),
    }
}

// ----------------------------------------------
// UI
// ----------------------------------------------
pub fn show(gui_info: &GUIInfo, _state: &BTAppState) -> Option<MenuCommand> {
    let mut result = None;

    egui::TopBottomPanel::top("top_panel").show(gui_info.ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.set_enabled(true);

            ui.menu_button("Project", |ui| {

                if ui.button("Open radish project…").clicked() {
                    if let Some(filepath) = rfd::FileDialog::new().pick_folder() {
                        result = Some(MenuCommand::OpenProject(utils::radish_dir_check(filepath)));
                    }
                    ui.close_menu();
                }

                if ui.button("Quit").clicked() {
                    result = Some(MenuCommand::Quit);
                    ui.close_menu();
                }
                
            })
        })
    });

    result
}
