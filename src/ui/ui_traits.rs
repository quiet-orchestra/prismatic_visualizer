use bevy_egui::egui::{ComboBox, Ui};

pub trait Setting: Send + Sync + 'static {
    fn heading(&self) -> &str;
    fn ui(&mut self, ui: &mut Ui);
}

pub struct SettingsMenu {
    index: usize,
    minimized: bool,
    heading: &'static str,
    pub settings_list: Vec<Box<dyn Setting>>,
}

impl SettingsMenu {
    pub fn new(heading: &'static str, settings_list: Vec<Box<dyn Setting>>) -> SettingsMenu {
        SettingsMenu { index: 0, minimized: false, heading, settings_list }
    }

    pub fn ui(
        &mut self,
        ui: &mut Ui,
    ) {
        ui.horizontal(|ui| {
            let min_text = if self.minimized {"Ʌ"} else {"V"};
            let is_minimized = self.minimized;
            ui.selectable_value(&mut self.minimized, !is_minimized , min_text);

            ui.label(self.heading);
        });
        ui.separator();

        if !self.minimized{
            ui.horizontal_wrapped(|ui|{
                for (i, setting) in self.settings_list.iter().enumerate() {
                    ui.selectable_value( &mut self.index,  i, setting.heading());
                }  
            });
            if let Some(setting) = self.settings_list.get_mut(self.index) {
                setting.ui(ui);
            }
        }
    }
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
