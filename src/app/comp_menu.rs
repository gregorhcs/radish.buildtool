use std::path::PathBuf;

use super::{utils::radish_dir_check, BuildToolAppModel};

// ----------------------------------------------
// Model
// ----------------------------------------------
pub struct MenuModel {
    pub projectdir: Option<PathBuf>,
}

// ----------------------------------------------
// Commands
// ----------------------------------------------
pub enum MenuCommand {
    OpenProject(Option<PathBuf>),
    Quit,
}

// ----------------------------------------------
pub fn handle_command_menu(_ctx: &egui::Context, frame: &mut eframe::Frame, command: MenuCommand, model: &mut BuildToolAppModel) {
    match command {
        MenuCommand::OpenProject(projectdir) => model.menu.projectdir = projectdir,
        MenuCommand::Quit                    => frame.close(),
    }
}

// ----------------------------------------------
// UI
// ----------------------------------------------
pub fn show_menu(ctx: &egui::Context) -> Option<MenuCommand> {
    let mut result = None;

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.set_enabled(true);

            ui.menu_button("Project", |ui| {

                if ui.button("Open radish project…").clicked() {
                    if let Some(filepath) = rfd::FileDialog::new().pick_folder() {
                        result = Some(MenuCommand::OpenProject(radish_dir_check(filepath)));
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