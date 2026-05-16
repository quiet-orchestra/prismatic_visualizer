use bevy_egui::egui::{Ui};

use crate::ui::Setting;

#[derive(Debug, Clone, Copy)]
pub struct Attribution{
    
}

impl Setting for Attribution {
    fn heading() -> &'static str {
        "Attribution"
    }
    fn ui(&mut self, ui: &mut Ui){
        ui.label(
"Prismatic Color Visualizer - MPL2

Thanks to 
Bevy - MIT & Apache-2.0 License
Bevy Panorbit Camera - MIT & Apache-2.0 License
Egui|Eframe - MIT & Apache-2.0 License
Bevy Egui - MIT License
Egui Double Slider - Apache-2.0 License
Bevy Pointcloud - MIT License"
);
    }
}

impl Default for Attribution {
    fn default() -> Attribution {
        Attribution {

        }
    }
}