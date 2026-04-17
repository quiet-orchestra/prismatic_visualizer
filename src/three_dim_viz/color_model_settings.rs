use bevy_egui::egui::{ComboBox, Ui};
use prismatic_color::{ColorModel, ColorSpace,};
use crate::{three_dim_viz::{RotationChirality, ColorModelCategory}, ui:: ui_traits::Setting};


#[derive(Debug, Clone, Copy)]
pub struct ColorModelSettings{
    model_selected_over_space: bool,

    pub color_model_category: ColorModelCategory,
    pub color_model: ColorModel,
    
    pub color_space: ColorSpace,
    pub color_space_model: ColorModel,

    pub mirrored: bool,
    pub rotated: RotationChirality,
}

impl Setting for ColorModelSettings {
    fn heading(&self) -> &str{
        "Color Model"
    }
    fn ui(&mut self, ui: &mut Ui){
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.model_selected_over_space, true, "Model");
            ui.selectable_value(&mut self.model_selected_over_space, false, "Space");
        });
        ui.separator();

        if self.model_selected_over_space {
            ui.label("Color Model");
            ui.horizontal(|ui| {
                ComboBox::from_label("")
                .selected_text(format!("{:?}", self.color_model_category))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.color_model_category, ColorModelCategory::Spherical, "Spherical");
                    ui.selectable_value(&mut self.color_model_category, ColorModelCategory::Cubic, "Cubic");
                    ui.selectable_value(&mut self.color_model_category, ColorModelCategory::LumaChroma, "Luma-Chroma");
                });
            });

            ui.horizontal(|ui| {
                match self.color_model_category {
                ColorModelCategory::Spherical => {
                    ui.selectable_value(&mut self.color_model, ColorModel::SphericalHCLA, "HCL");
                },
                ColorModelCategory::Cubic => {
                    ui.selectable_value(&mut self.color_model, ColorModel::CubicHSVA, "HSV");
                    // ui.selectable_value(&mut self.color_model, ColorModel::CubicHSLA, "HSL");
                },
                ColorModelCategory::LumaChroma => {
                    ui.selectable_value(&mut self.color_model, ColorModel::YUVA, "YUV");
                },
            }});
        }
        else {
            let current_color_model = self.color_model;

            ui.label("Color Space");
            ComboBox::from_label("")
            .selected_text(format!("{:?}", self.color_space_model))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.color_space_model, current_color_model, "Current Color Model");
                ui.selectable_value(&mut self.color_space_model, ColorModel::RGBA, "RGB");
                ui.selectable_value(&mut self.color_space_model, ColorModel::CMYA, "CMY");
                ui.selectable_value(&mut self.color_space_model, ColorModel::SphericalHCLA, "Spherical HCL");
                ui.selectable_value(&mut self.color_space_model, ColorModel::CubicHSVA, "Cubic HSV");
                ui.selectable_value(&mut self.color_space_model, ColorModel::YUVA, "YUV");
            });

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.color_space, ColorSpace::XYZ, "XYZ");
                ui.selectable_value(&mut self.color_space, ColorSpace::Cylindrical, "Cylindrical");
            });
        }
        ui.separator();
        ui.horizontal(|ui| {
            let mirrored = self.mirrored;
            ui.selectable_value( &mut self.mirrored, !mirrored, "Mirror");
            ui.label("Rotate: ");

            let current_rotation = self.rotated;
            ui.selectable_value(&mut self.rotated, current_rotation.next_clockwise(), "↻");
            ui.selectable_value(&mut self.rotated, current_rotation.next_counterclockwise(), "↺");
        });
    }
}

impl Default for ColorModelSettings {
    fn default() -> ColorModelSettings {
        ColorModelSettings {
            model_selected_over_space: true,
            color_model_category: ColorModelCategory::Spherical,
            color_model: ColorModel::SphericalHCLA,
            color_space: ColorSpace::XYZ,
            color_space_model: ColorModel::RGBA,

            mirrored: false,
            rotated: RotationChirality::Middle,
        }
    }
}