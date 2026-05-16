use bevy_egui::egui::{Ui,Slider};

use crate::ui::Setting;


#[derive(Debug, Clone, Copy)]
pub struct GridSettings{
    pub grid: GridCategory,
    pub grid_scale: f32,
    pub grid_divs: u32,
}

impl Setting for GridSettings {
    fn heading() -> &'static str {
        return "Grid Settings";
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal_wrapped( |ui| {
            let current_grid_selected = self.grid.clone();
            ui.selectable_value(&mut self.grid,  if current_grid_selected != GridCategory::TwoDGrids {GridCategory::TwoDGrids} else {GridCategory::None}, "2D Grids");
            // ui.selectable_value(&mut self.grid, GridCategory::ThreeDGrid, "3D Grid");

            if self.grid != GridCategory::None {
                ui.separator();
                ui.add(Slider::new( &mut self.grid_scale ,0.0..=2.0).text("Scale"));
                ui.add(Slider::new( &mut self.grid_divs ,1..=25).text("Divisions"));
            }

        });

    }
}

impl Default for GridSettings {
    fn default() -> GridSettings {
        return GridSettings { grid: GridCategory::None, grid_scale: 1.0, grid_divs: 10};
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridCategory {
    None,
    TwoDGrids,
    // ThreeDGrid,
}