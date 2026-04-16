use bevy_egui::egui::{ComboBox, Ui};

pub trait Setting {
    fn heading(&self) -> &str;
    fn ui(&mut self, ui: &mut Ui);
}

pub fn setting_menu(
    ui: &mut Ui, 
    settings_list: &mut [Box<dyn Setting>], 
    current_index: &mut usize
) -> usize {
    ui.horizontal_wrapped(|ui|{
        for (i, setting) in settings_list.iter().enumerate() {
            ui.selectable_value( current_index,  i, setting.heading());
        }  
    });
    if let Some(setting) = settings_list.get_mut(*current_index) {
        setting.ui(ui);
    }
    return  *current_index;
}

pub fn setting_dropdown(
    heading: &str,
    ui: &mut Ui, 
    settings_list: &mut [Box<dyn Setting>], 
    current_index: &mut usize
) -> usize {

    let current_label = settings_list.get_mut(*current_index).unwrap().heading();

    ComboBox::new(heading, current_label).show_ui(ui,|ui|{
        for (i, setting) in settings_list.iter().enumerate() {
            ui.selectable_value( current_index,  i, setting.heading());
        }  
    });
    if let Some(setting) = settings_list.get_mut(*current_index) {
        setting.ui(ui);
    }
    return  *current_index;
}