pub mod config;
pub mod paths;

// ----------------------------------------------
pub struct UIInfo<'a> {
    pub ctx: &'a egui::Context, 
    pub frame: &'a mut eframe::Frame,
}