use bevy_egui::egui::{Ui};

use crate::ui::ui_traits::Setting;


#[derive(Debug, Clone, Copy)]
pub struct Attribution{
    
}

impl Setting for Attribution {
    fn heading(&self) -> &str{
        "Attribution"
    }
    fn ui(&mut self, ui: &mut Ui){
        ui.label(
"
Prismatic Color Visualizer

Thanks to 
Bevy - MIT & Apache-2.0 License
Bevy Panorbit Camera - MIT & Apache-2.0 License
Egui - MIT & Apache-2.0 License
Bevy Egui - MIT License
Egui Double Slider - Apache-2.0 License
Bevy Pointcloud - MIT License

This application is licensed under MPL2"
);
    }
}

impl Default for Attribution {
    fn default() -> Attribution {
        Attribution {

        }
    }
}