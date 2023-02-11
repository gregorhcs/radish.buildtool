mod utils;

// ----------------------------------------------
use std::path::PathBuf;

use crate::app::{BTAppState, utils::UIInfo};

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
    OpenProject,
    CloseProject,
    Quit,
}

// ----------------------------------------------
pub fn handle_command(ui_info: &mut UIInfo, state: &mut BTAppState, command: MenuCommand) -> Result<(), String> {
    match command {
        MenuCommand::OpenProject => {
            match rfd::FileDialog::new().pick_folder() {
                Some(filepath) => { 
                    let potential_radishpath = utils::radish_dir_check(filepath);
                    if potential_radishpath.is_some() {
                        state.menu.projectdir = potential_radishpath;
                    }
                    else {
                        state.aux_windows.set_showing_info(format!("No radish project found in the selected directory."));
                    }
                },
                None => { return Err(String::from("No project directory picked."));  }
            }
        },
        MenuCommand::CloseProject => { state.menu.projectdir = None; },
        MenuCommand::Quit => ui_info.frame.close()
    }
    Ok(())
}

// ----------------------------------------------
// UI
// ----------------------------------------------
pub fn show(ui_info: &UIInfo, state: &BTAppState) -> Option<MenuCommand> {
    let mut result = None;

    egui::TopBottomPanel::top("top_panel").show(ui_info.ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.set_enabled(true);

            ui.menu_button("Project", |ui| {

                if ui.button("Open project…").clicked() {
                    result = Some(MenuCommand::OpenProject);
                    ui.close_menu();
                }

                let is_project_open = state.menu.projectdir.is_some();
                ui.add_enabled_ui(is_project_open, |ui| {

                    if ui.button("Close project").clicked() {
                        result = Some(MenuCommand::CloseProject);
                        ui.close_menu();
                    }

                });

                ui.separator();

                if ui.button("Quit").clicked() {
                    result = Some(MenuCommand::Quit);
                    ui.close_menu();
                }
                
            })
        })
    });

    result
}
