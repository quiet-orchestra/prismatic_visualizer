use bevy_egui::egui::{Ui};

use crate::ui::ui_traits::Setting;


#[derive(Debug, Clone, Copy)]
pub struct FooSettings{
    
}

impl Setting for FooSettings {
    fn heading(&self) -> &str{
        "Foo"
    }
    fn ui(&mut self, ui: &mut Ui){
        
    }
}

impl Default for FooSettings {
    fn default() -> FooSettings {
        FooSettings {

        }
    }
}