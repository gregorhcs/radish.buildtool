// ----------------------------------------------
use std::path::PathBuf;

use crate::utils::{self, UIInfo};

use super::BTAppState;

// ----------------------------------------------
// Model
// ----------------------------------------------
pub struct MenuState {
    pub projectdir: Option<PathBuf>,
}

// ----------------------------------------------
impl MenuState {
    pub fn open_project(&mut self, filepath: PathBuf) -> Result<(), ()> {

        match utils::paths::radish_dir_check(filepath) {
            Some(projectpath) => { self.projectdir = Some(projectpath); Ok(()) },
            None              => Err(())
        }

    }
}

// ----------------------------------------------
impl From<PathBuf> for MenuState {
    fn from(filepath: PathBuf) -> MenuState {
        MenuState { 
            projectdir: utils::paths::radish_dir_check(filepath)
        }
    }
}

// ----------------------------------------------
// Commands
// ----------------------------------------------
pub enum MenuCommand {
    NewProject,
    OpenProject,
    OpenRecentProject(PathBuf),
    CloseProject,
    Quit,
}

// ----------------------------------------------
pub fn handle_command(ui_info: &mut UIInfo, state: &mut BTAppState, command: MenuCommand) -> Result<(), String> {

    match command {

        MenuCommand::NewProject => { state.aux_windows.set_showing_info(format!("Feature not implemented yet.")); } // TODO: Implement adding a new project.

        MenuCommand::OpenProject => {
            match rfd::FileDialog::new().pick_folder() {
                Some(filepath) => 
                    if state.menu.open_project(filepath).is_err() {
                        state.aux_windows.set_showing_info("No radish project found in the selected directory.");
                    },
                None => { return Err(String::from("No project directory picked."));  }
            }
        },

        MenuCommand::OpenRecentProject(filepath) => 
            if state.menu.open_project(filepath).is_err() {
                state.aux_windows.set_showing_info("No radish project found in the selected recent directory.");
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
            ui.set_enabled(state.aux_windows.show.is_none());

            ui.menu_button("Project", |ui| {

                if false { // TODO: Implement adding a new project.
                    if ui.button("New project…").clicked() {
                        result = Some(MenuCommand::NewProject);
                        ui.close_menu();
                    }
                }

                if ui.button("Open project…").clicked() {
                    result = Some(MenuCommand::OpenProject);
                    ui.close_menu();
                }

                let recent_projectdirs = utils::config::load_recent_projectpaths();
                ui.add_enabled_ui(!recent_projectdirs.is_empty(), |ui| {
                    ui.menu_button("Open recent project…", |ui| {
                        ui.set_min_width(400.);

                        for (slot, path) in recent_projectdirs.iter().enumerate() {
                            if slot == 0 {
                                continue;
                            }

                            if ui.button(format!("{} {}", slot, utils::paths::pretty_print(path))).clicked() {
                                result = Some(MenuCommand::OpenRecentProject(path.to_path_buf()));
                                ui.close_menu();
                            }
                        }
    
                    });
                });

                let is_project_open = state.menu.projectdir.is_some();
                ui.add_enabled_ui(is_project_open, |ui| {
                    if ui.button("Close project").clicked() {
                        result = Some(MenuCommand::CloseProject);
                        ui.close_menu();
                    }
                });

                ui.separator();

                if ui.button("Exit").clicked() {
                    result = Some(MenuCommand::Quit);
                    ui.close_menu();
                }
                
            })
        })
    });

    result
}
