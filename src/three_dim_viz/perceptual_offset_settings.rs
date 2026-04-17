use bevy_egui::egui::{Ui, Slider};
use crate::ui::ui_traits::Setting;

#[derive(Debug, Clone, Copy)]
pub struct PerceptualOffsetSettings{
    pub component_limit: (f32,f32,f32),
    pub per_component_gamma: bool,
    pub gamma: (f32,f32,f32),

    pub gamma_deform: bool,

}

impl Setting for PerceptualOffsetSettings {
    fn heading(&self) -> &str{
        "Perceptual Offset"
    }
    fn ui(&mut self, ui: &mut Ui){
        ui.add(Slider::new( &mut self.component_limit.0 ,0.1..=1.0).text("Red"));
        ui.add(Slider::new( &mut self.component_limit.1 ,0.1..=1.0).text("Green"));
        ui.add(Slider::new( &mut self.component_limit.2 ,0.1..=1.0).text("Blue"));

        ui.horizontal(|ui| {
            ui.label("Gamma");
            ui.checkbox(&mut self.per_component_gamma, "per component");
        });
        if self.per_component_gamma {
            ui.add(Slider::new( &mut self.gamma.0 ,0.1..=3.0).text("Red"));
            ui.add(Slider::new( &mut self.gamma.1 ,0.1..=3.0).text("Green"));
            ui.add(Slider::new( &mut self.gamma.2 ,0.1..=3.0).text("Blue"));
        }
        else {
            ui.add(Slider::new( &mut self.gamma.0 ,0.1..=3.0));
            self.gamma.1 = self.gamma.0;
            self.gamma.2 = self.gamma.0;
        }
        ui.separator();
        ui.checkbox(&mut self.gamma_deform, "Gamma Deform");
    }
}

impl Default for PerceptualOffsetSettings {
    fn default() -> PerceptualOffsetSettings {
        PerceptualOffsetSettings {
            component_limit: (1., 1., 1.), 
            per_component_gamma: false,
            gamma: (2.2, 2.2, 2.2),
            gamma_deform: false,
        }
    }
}