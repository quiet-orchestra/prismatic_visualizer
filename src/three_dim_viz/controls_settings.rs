use bevy_egui::egui::{Ui};

use crate::ui::Setting;


#[derive(Debug, Clone, Copy)]
pub struct ControlSettings{
    
}

impl Setting for ControlSettings {
    fn heading() -> &'static str {
        "Controls"
    }
    fn ui(&mut self, ui: &mut Ui){
        ui.horizontal(|ui|{
        ui.label(
            "Mouse:
•Left - Orbit
•Right - Pan
•Scroll - Zoom"
        );
        ui.label(
            "Touch:
•One Finger - Orbit
•Two Fingers - Pan
•Pinch - Zoom"
        );
        ui.label(
            "Keyboard:
•WASD - Horizontal
•Ctrl & Space - Vertical
•Arrow Keys - Pitch and Yaw"
        );  
    });

    }
}

impl Default for ControlSettings {
    fn default() -> ControlSettings {
        ControlSettings {

        }
    }
}