use bevy_egui::egui::{Slider, Ui};

use crate::{three_dim_viz::{Dimensionality, SlicingMethod}, ui::ui_traits::Setting};


#[derive(Debug, Clone, Copy)]
pub struct DimensionalitySettings{
    pub dimensionality: Dimensionality,

    pub instance_scale: f32,
    pub line_width: f32,

    pub face_slicing: SlicingMethod,
    pub discrete_color: bool,
}

impl Setting for DimensionalitySettings {
    fn heading(&self) -> &str{
        "Shape"
    }
    fn ui(&mut self, ui: &mut Ui){
        ui.label("Dimensions");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.dimensionality, Dimensionality::Vertex, "Vertex");
            ui.selectable_value(&mut self.dimensionality, Dimensionality::Edge, "Edge");
            ui.selectable_value(&mut self.dimensionality, Dimensionality::Face, "Face");
            ui.selectable_value(&mut self.dimensionality, Dimensionality::Volume, "Volume");
        });

        match self.dimensionality {
            Dimensionality::Vertex => {
                // ui.label("Mesh Shape");
                ui.horizontal(|ui| {
                    ui.add(Slider::new( &mut self.instance_scale ,0.0..=2.0).text("Shape Scale"));
                });
            },
            Dimensionality::Edge => {
                ui.label("Edge Direction");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.face_slicing, SlicingMethod::Y, "X|Axial");
                    ui.selectable_value(&mut self.face_slicing, SlicingMethod::X, "Y|Radial");
                    ui.selectable_value(&mut self.face_slicing, SlicingMethod::Z, "Z|Concentric");
                });
                ui.add(Slider::new( &mut self.line_width ,0.0..=10.0).text("Line Width"));
                ui.checkbox(&mut self.discrete_color, "Discrete Color");
            },
            Dimensionality::Face => {
                ui.label("Quad Direction");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.face_slicing, SlicingMethod::X, "X|Axial");
                    ui.selectable_value(&mut self.face_slicing, SlicingMethod::Y, "Y|Radial");
                    ui.selectable_value(&mut self.face_slicing, SlicingMethod::Z, "Z|Concentric");
                });
                ui.checkbox(&mut self.discrete_color, "Discrete Color");

            },
            Dimensionality::Volume => {
                ui.checkbox(&mut self.discrete_color, "Discrete Color");
            },
        }
    }
}

impl Default for DimensionalitySettings {
    fn default() -> DimensionalitySettings {
        DimensionalitySettings {
            dimensionality: Dimensionality::Vertex,
            instance_scale: 1.0,
            line_width: 1.0,
            face_slicing: SlicingMethod::Y,
            discrete_color: true,
        }
    }
}