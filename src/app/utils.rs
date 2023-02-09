// ----------------------------------------------
pub struct GUIInfo<'a> {
    pub ctx: &'a egui::Context, 
    pub frame: &'a mut eframe::Frame,
}

// ----------------------------------------------
pub fn filter_push<A, B: Into<A>>(vec: &mut Vec<A>, item: Option<B>) {
    if let Some(inner_item) = item {
        vec.push(inner_item.into());
    }
}