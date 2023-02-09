// ----------------------------------------------
mod menu;
mod batch;

mod utils;

// ----------------------------------------------
use std::path::PathBuf;

use menu::MenuCommand;
use batch::BatchCommand;

use utils::{GUIInfo, filter_push};

// ----------------------------------------------
// Build Tool Model
// ----------------------------------------------
pub struct BTAppState {
    pub menu: menu::MenuState,
    pub batch: batch::BatchState,
}

// ----------------------------------------------
impl From<PathBuf> for BTAppState {
    fn from(filepath: PathBuf) -> BTAppState {
        BTAppState { 
            menu:  menu::MenuState::from(filepath),
            batch: batch::BatchState::default()
        }
    }
}

// ----------------------------------------------
// Build Tool View
// ----------------------------------------------
fn show(gui_info: &GUIInfo, state: &BTAppState) -> Vec<Option<BTAppCommand>> {

    let mut results = Vec::new();

    egui::CentralPanel::default().show(&gui_info.ctx, |_ui| {

        if let Some(command) = menu::show(gui_info, state) {
            filter_push(&mut results, Some(BTAppCommand::Menu(command)));
        }

        if let Some(command) = batch::show(gui_info, state) {
            filter_push(&mut results, Some(BTAppCommand::Batch(command)));
        }

    });

    results
}

// ----------------------------------------------
// Build Tool Command
// ----------------------------------------------
enum BTAppCommand {
    Menu(MenuCommand),
    Batch(BatchCommand),
}

// ----------------------------------------------
fn handle_command(gui_info: &mut GUIInfo, state: &mut BTAppState, command: BTAppCommand) {
    
    match command {
        BTAppCommand::Menu(sub_command) => menu::handle_command(gui_info, state, sub_command),
        BTAppCommand::Batch(sub_command) => batch::handle_command(gui_info, state, sub_command),
    }

}

// --------------------------------------------------------------------------------------------
// --------------------------------------------------------------------------------------------

// ----------------------------------------------
// Build Tool App
// ----------------------------------------------
pub struct BTApp {
    model: BTAppState,
}

// ----------------------------------------------
impl From<PathBuf> for BTApp {
    fn from(filepath: PathBuf) -> BTApp {
        BTApp { 
            model: BTAppState::from(filepath),
        }
    }
}

// ----------------------------------------------
impl eframe::App for BTApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        let mut gui_info = GUIInfo{ ctx, frame };

        for wrapped_command in show(&gui_info, &self.model) {
            if let Some(command) = wrapped_command {
                handle_command(&mut gui_info, &mut self.model, command);
            }
        }

    }
}
