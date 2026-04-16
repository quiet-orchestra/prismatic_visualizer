use bevy_egui::egui::{Ui, Slider};

use crate::ui::ui_traits::Setting;


#[derive(Debug, Clone, Copy)]
pub struct ScaleSettings{
    pub viz_scale: f32,
    pub viz_alpha: f32,
}

impl Setting for ScaleSettings {
    fn heading(&self) -> &str{
        "Scale"
    }
    fn ui(&mut self, ui: &mut Ui){
        ui.add(Slider::new( &mut self.viz_scale ,0.0..=2.0).text("Visualization Scale"));
        ui.add(Slider::new( &mut self.viz_alpha, 0.0..=1.0).text("Alpha"));
    }
}

impl Default for ScaleSettings {
    fn default() -> ScaleSettings {
        ScaleSettings {
            viz_scale: 1.,
            viz_alpha: 1.,
        }
    }
}