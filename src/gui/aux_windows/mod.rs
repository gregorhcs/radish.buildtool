use super::{utils::UIInfo, BTAppState};

// ----------------------------------------------
// Model
// ----------------------------------------------
pub struct AuxWindowsState {
    pub show: Option<AuxWindowToShow>,
    pub message: String
}

impl AuxWindowsState {

    pub fn set_showing_info<S>(&mut self, message: S) where S: Into<String> {
        self.set_showing(AuxWindowToShow::Info, message.into());
    }

    pub fn set_showing_confirm<S>(&mut self, message: S) where S: Into<String> {
        self.set_showing(AuxWindowToShow::Confirm, message.into());
    }

    pub fn set_showing_error<S>(&mut self, message: S) where S: Into<String> {
        self.set_showing(AuxWindowToShow::Error, message.into());
    }

    pub fn set_showing<S>(&mut self, show: AuxWindowToShow, message: S) where S: Into<String> {
        self.show = Some(show);
        self.message = message.into();
    }

    pub fn set_not_showing(&mut self) {
        self.show = None;
        self.message = String::default();
    }

}

impl Default for AuxWindowsState {
    fn default() -> Self {
        AuxWindowsState { 
            show: None,
            message: String::default()
        }
    }
}

pub enum AuxWindowToShow {
    Info,
    Confirm,
    Error
}

// ----------------------------------------------
// Commands
// ----------------------------------------------
pub enum AuxWindowCommand {
    InfoExit,
    ConfirmExit(bool),
    ErrorExit,
}

// ----------------------------------------------
pub fn handle_command(_ui_info: &mut UIInfo, state: &mut BTAppState, _command: AuxWindowCommand) {
    state.aux_windows.set_not_showing()
}

// ----------------------------------------------
// UI
// ----------------------------------------------
const _WINDOW_TITLE_INFO: &str = "Info";
const _WINDOW_TITLE_CONFIRM: &str = "Confirm";
const _WINDOW_TITLE_ERROR: &str = "Error!";

const _WINDOW_BTN1_OK: &str = "Ok";
const _WINDOW_BTN1_CONFIRM: &str = "Confirm";
const _WINDOW_BTN1_CANCEL: &str = "Cancel";

// ----------------------------------------------
pub fn show(ui_info: &UIInfo, state: &BTAppState) -> Option<AuxWindowCommand> {

    let mut command = None;

    if let Some(show) = &state.aux_windows.show {

        let title = match show {
            AuxWindowToShow::Info    => _WINDOW_TITLE_INFO,
            AuxWindowToShow::Confirm => _WINDOW_TITLE_CONFIRM,
            AuxWindowToShow::Error   => _WINDOW_TITLE_ERROR,
        };

        egui::Window::new(title)
            .collapsible(false)
            .resizable(false)
            .show(ui_info.ctx, |ui| {
                ui.horizontal(|ui| {

                    ui.label(state.aux_windows.message.as_str());

                    match show {
                        AuxWindowToShow::Info => {
                            if ui.button(_WINDOW_BTN1_OK).clicked() {
                                command = Some(AuxWindowCommand::InfoExit)
                            }
                        },
                        AuxWindowToShow::Error => {
                            if ui.button(_WINDOW_BTN1_OK).clicked() {
                                command = Some(AuxWindowCommand::ErrorExit)
                            }
                        },
                        AuxWindowToShow::Confirm => {
                            if ui.button(_WINDOW_BTN1_CONFIRM).clicked() {
                                command = Some(AuxWindowCommand::ConfirmExit(true))
                            }
                            if ui.button(_WINDOW_BTN1_CANCEL).clicked() {
                                command = Some(AuxWindowCommand::ConfirmExit(false))
                            }
                        },
                    }

                });
            });

    }
    command

}