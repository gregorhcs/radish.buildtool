// ----------------------------------------------
mod batch;
mod menu;

mod aux_windows;

// ----------------------------------------------
use std::path::PathBuf;

use batch::BatchCommand;
use menu::MenuCommand;

use crate::utils::{self, UIInfo};


// ----------------------------------------------
// Build Tool Model
// ----------------------------------------------
pub struct BTAppState {
    pub menu: menu::MenuState,
    pub batch: batch::BatchState,

    pub aux_windows: aux_windows::AuxWindowsState
}

// ----------------------------------------------
impl From<PathBuf> for BTAppState {
    fn from(filepath: PathBuf) -> BTAppState {
        BTAppState {
            menu: menu::MenuState::from(filepath),
            batch: batch::BatchState::default(),

            aux_windows: aux_windows::AuxWindowsState::default(),
        }
    }
}

// ----------------------------------------------
// Build Tool View
// ----------------------------------------------
fn show(ui_info: &UIInfo, state: &BTAppState) -> Vec<BTAppCommand> {

    let mut results = Vec::new();

    egui::CentralPanel::default().show(&ui_info.ctx, |_ui| {
            
        if let Some(command) = menu::show(ui_info, state) {
            results.push(BTAppCommand::Menu(command));
        }

        if state.menu.projectdir.is_some() {
            if let Some(command) = batch::show(ui_info, state) {
                results.push(BTAppCommand::Batch(command));
            }
        }
        else {
            egui::CentralPanel::default().show(ui_info.ctx, |ui| {
                ui.add_enabled_ui(false, |ui| ui.label("No project open."));
            });
        }

    });

    results

}

// ----------------------------------------------
// Build Tool Command
// ----------------------------------------------
enum BTAppCommand {
    Menu(MenuCommand),
    Batch(BatchCommand)
}

// ----------------------------------------------
fn handle_command(ui_info: &mut UIInfo, state: &mut BTAppState, command: BTAppCommand) -> Result<(), String> {
    match command {
        BTAppCommand::Menu(sub_command) => menu::handle_command(ui_info, state, sub_command),
        BTAppCommand::Batch(sub_command) => batch::handle_command(ui_info, state, sub_command)
    }
}

// --------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------

// ----------------------------------------------
// Build Tool App
// ----------------------------------------------
pub struct BTApp {
    state: BTAppState,
}

// ----------------------------------------------
impl From<PathBuf> for BTApp {
    fn from(filepath: PathBuf) -> BTApp {
        BTApp {
            state: BTAppState::from(filepath),
        }
    }
}

// ----------------------------------------------
impl eframe::App for BTApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        frame.set_window_title(utils::config::gui_title(&self.state.menu.projectdir).as_str());

        let mut ui_info = UIInfo { ctx, frame };
        
        // process main command list

        let mut commands = show(&ui_info, &self.state);

        while let Some(command) = commands.pop() {
            let result = handle_command(&mut ui_info, &mut self.state, command);

            if let Err(message) = result {
                self.state.aux_windows.set_showing_error(message);
            }
        }

        
        // process auxiliary windows
        
        let potential_command = aux_windows::show(&ui_info, &self.state);

        if let Some(command) = potential_command {
            aux_windows::handle_command(&mut ui_info, &mut self.state, command);
        }

    }
}
