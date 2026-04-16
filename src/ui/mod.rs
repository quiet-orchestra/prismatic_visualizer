use bevy::{ecs::component::Component, prelude::{ResMut, Resource}, reflect::Reflect};
use egui_double_slider::DoubleSlider;
use prismatic_color::{ColorModel, ColorSpace};
use bevy_egui::{
    EguiContexts, egui::{self, global_theme_preference_buttons}
};

pub(crate) mod ui_traits;

use crate::{three_dim_viz::{
    ColorModelCategory, Dimensionality, ScaleSettings, GridSettings, SlicingMethod
}, ui::ui_traits::Setting};

#[derive(Resource, Clone)]
pub struct Settings{
    pub minimized: bool,

    pub three_dimension_settings: ThreeDimensionSettings,

    pub scale_settings: ScaleSettings,

    pub grid_settings: GridSettings,

    pub component_limit: (f32,f32,f32),
    pub per_component_gamma: bool,
    pub gamma: (f32,f32,f32),

    pub channel_settings: (ColorChannel,ColorChannel,ColorChannel),

    pub color_model_category: ColorModelCategory,
    pub color_model: ColorModel,
    pub color_space: ColorSpace,
    pub dimensionality: Dimensionality,

    pub instance_scale: f32,
    pub line_width: f32,

    pub face_slicing: SlicingMethod,
    pub gamma_deform: bool,
    pub discrete_color: bool,
    pub color_space_model: ColorModel,

    pub mirrored: bool,
    pub rotated: RotationChirality,

}

#[derive(Component, Debug, Clone, Reflect, PartialEq)]
pub enum RotationChirality{
    None,
    Left,
    Right,
}

#[derive(Component, Debug, Clone, Reflect)]
pub struct ColorChannel {
    pub start: f32,
    pub end: f32,
    pub steps: usize,
    pub step_type: StepType,
}

impl Default for ColorChannel {
    fn default() -> Self {
        Self {
            start: 0.,
            end: 1.,
            steps: 8,
            step_type: StepType::Forward,
        }
    }
}

impl ColorChannel {
    pub fn generate_range(&self, along_grain: bool) -> (f32, usize, f32) {

        let mut start = self.start;
        let mut wrap = false;

        match self.step_type {
                StepType::Forward => {},
                StepType::Reverse => start = self.end,
                StepType::Inclusive => wrap = true,
        };



        let steps = if wrap { self.steps + 1 } else { self.steps };
        let steps = if along_grain { steps - 1 } else { steps };

        let step_size = (self.end - self.start) / (self.steps as f32);

        (start, steps, step_size)
    }

}

#[derive(Component, Debug, Clone, Reflect, PartialEq)]
pub enum StepType {
    Forward,
    Reverse,
    Inclusive,
}



impl Default for Settings{
    fn default() -> Self {
        Self {
            minimized: false,

            three_dimension_settings: ThreeDimensionSettings::Scale,

            scale_settings: ScaleSettings::default(),

            grid_settings: GridSettings::default(),

            component_limit: (1., 1., 1.), 
            per_component_gamma: false,
            gamma: (2.2, 2.2, 2.2),

            channel_settings: (
                ColorChannel::default(),
                ColorChannel::default(),
                ColorChannel::default(),
            ),

            color_model_category: ColorModelCategory::Spherical,
            color_model: ColorModel::SphericalHCLA,
            dimensionality: Dimensionality::Vertex,

            instance_scale: 1.0,
            line_width: 1.0,

            face_slicing: SlicingMethod::Y,
            gamma_deform: false,
            discrete_color: true,
            color_space: ColorSpace::XYZ,
            color_space_model: ColorModel::RGBA,

            // model_rotation: RotationDirection::None,
            mirrored: false,
            rotated: RotationChirality::None,

        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThreeDimensionSettings {
    Scale,
    PerceptualOffset,
    ChannelSettings,
    ColorModel,
    ColorSpace,
    Dimensionality,
    Controls,
    Attribution,
}

pub fn ui_overlay(mut contexts: EguiContexts, mut settings: ResMut<Settings>) {

    //Create window for variable sliders
    egui::TopBottomPanel::top("Settings")
        .resizable(true)
        .show(contexts.ctx_mut().unwrap(), | mut ui|{
        egui::Sense::hover();
        let width = ui.available_width();


        let min_text = if settings.minimized {"Ʌ"} else {"V"};
        let is_minimized = settings.minimized;
        ui.selectable_value(&mut settings.minimized, !is_minimized , min_text);

        if !settings.minimized {
                    ui.horizontal(|ui| {
            ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::Scale, "Scale");
            ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::PerceptualOffset, "Perceptual Offset");
            ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::ChannelSettings, "Channel Settings");
            ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::ColorModel, "Color Model");
            ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::ColorSpace, "Color Space");
            ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::Dimensionality, "Shape");
            ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::Controls, "Controls");
            ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::Attribution, "Attribution");
        });

        match settings.three_dimension_settings {
            ThreeDimensionSettings::Scale => {
                settings.scale_settings.ui(ui);
                settings.grid_settings.ui(ui);
            },
            ThreeDimensionSettings::PerceptualOffset => {
                ui.add(egui::Slider::new( &mut settings.component_limit.0 ,0.0..=1.0).text("Red"));
                ui.add(egui::Slider::new( &mut settings.component_limit.1 ,0.0..=1.0).text("Green"));
                ui.add(egui::Slider::new( &mut settings.component_limit.2 ,0.0..=1.0).text("Blue"));

                ui.horizontal(|ui| {
                    ui.label("Gamma");
                    ui.checkbox(&mut settings.per_component_gamma, "per component");
                });
                if settings.per_component_gamma {
                    ui.add(egui::Slider::new( &mut settings.gamma.0 ,0.1..=3.0).text("Red"));
                    ui.add(egui::Slider::new( &mut settings.gamma.1 ,0.1..=3.0).text("Green"));
                    ui.add(egui::Slider::new( &mut settings.gamma.2 ,0.1..=3.0).text("Blue"));
                }
                else {
                    ui.add(egui::Slider::new( &mut settings.gamma.0 ,0.1..=3.0));
                    settings.gamma.1 = settings.gamma.0;
                    settings.gamma.2 = settings.gamma.0;
                }
                ui.separator();
                ui.checkbox(&mut settings.gamma_deform, "Gamma Deform");
            },
            ThreeDimensionSettings::ChannelSettings => {
                ui.horizontal(|ui| {
                    ui.label("Channel Settings");
                });

                //Channel A
                ui_channel(&mut ui, "A", &mut settings.channel_settings.0, width);

                //Channel B
                ui_channel(&mut ui, "B", &mut settings.channel_settings.1, width);

                //Channel C
                ui_channel(&mut ui, "C", &mut settings.channel_settings.2, width);
            },
            ThreeDimensionSettings::ColorModel => {
                ui.label("Color Model");
                ui.horizontal(|ui| {
                    egui::ComboBox::from_label("")
                    .selected_text(format!("{:?}", settings.color_space_model))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut settings.color_model_category, ColorModelCategory::Spherical, "Spherical");
                        ui.selectable_value(&mut settings.color_model_category, ColorModelCategory::Cubic, "Cubic");
                        ui.selectable_value(&mut settings.color_model_category, ColorModelCategory::LumaChroma, "Luma-Chroma");
                    });
                });

                ui.separator();

                ui.horizontal(|ui| {
                    match settings.color_model_category {
                    ColorModelCategory::Spherical => {
                        ui.selectable_value(&mut settings.color_model, ColorModel::SphericalHCLA, "HCL");
                    },
                    ColorModelCategory::Cubic => {
                        ui.selectable_value(&mut settings.color_model, ColorModel::CubicHSVA, "HSV");
                        // ui.selectable_value(&mut settings.color_model, ColorModel::CubicHSLA, "HSL");
                    },
                    ColorModelCategory::LumaChroma => {
                        ui.selectable_value(&mut settings.color_model, ColorModel::YUVA, "YUV");
                    },
                }});
            },
            ThreeDimensionSettings::ColorSpace => {
                let current_color_model = settings.color_model;

                ui.label("Color Space");
                egui::ComboBox::from_label("")
                .selected_text(format!("{:?}", settings.color_space_model))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut settings.color_space_model, current_color_model, "Current Color Model");
                    ui.selectable_value(&mut settings.color_space_model, ColorModel::RGBA, "RGB");
                    ui.selectable_value(&mut settings.color_space_model, ColorModel::CMYA, "CMY");
                    ui.selectable_value(&mut settings.color_space_model, ColorModel::SphericalHCLA, "Spherical HCL");
                    ui.selectable_value(&mut settings.color_space_model, ColorModel::CubicHSVA, "Cubic HSV");
                    ui.selectable_value(&mut settings.color_space_model, ColorModel::YUVA, "YUV");
                });

                ui.horizontal(|ui| {
                    ui.selectable_value(&mut settings.color_space, ColorSpace::XYZ, "XYZ");
                    ui.selectable_value(&mut settings.color_space, ColorSpace::Cylindrical, "Cylindrical");
                });

                ui.horizontal(|ui| {
                    let mirrored = settings.mirrored;
                    ui.selectable_value( &mut settings.mirrored, !mirrored, "Mirror");
                    ui.label("Rotate: ");

                    ui.selectable_value(&mut settings.rotated, RotationChirality::Left, "Left");
                    ui.selectable_value(&mut settings.rotated, RotationChirality::None, "None");
                    ui.selectable_value(&mut settings.rotated, RotationChirality::Right, "Right");
                });
            },
            ThreeDimensionSettings::Dimensionality => {
                ui.label("Dimensions");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut settings.dimensionality, Dimensionality::Vertex, "Vertex");
                    ui.selectable_value(&mut settings.dimensionality, Dimensionality::Edge, "Edge");
                    ui.selectable_value(&mut settings.dimensionality, Dimensionality::Face, "Face");
                    ui.selectable_value(&mut settings.dimensionality, Dimensionality::Volume, "Volume");
                });
        
                match settings.dimensionality {
                    Dimensionality::Vertex => {
                        // ui.label("Mesh Shape");
                        ui.horizontal(|ui| {
                            ui.add(egui::Slider::new( &mut settings.instance_scale ,0.0..=2.0).text("Shape Scale"));
                        });
                    },
                    Dimensionality::Edge => {
                        ui.label("Edge Direction");
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut settings.face_slicing, SlicingMethod::Y, "X|Axial");
                            ui.selectable_value(&mut settings.face_slicing, SlicingMethod::X, "Y|Radial");
                            ui.selectable_value(&mut settings.face_slicing, SlicingMethod::Z, "Z|Concentric");
                        });
                        ui.add(egui::Slider::new( &mut settings.line_width ,0.0..=10.0).text("Line Width"));
                        ui.checkbox(&mut settings.discrete_color, "Discrete Color");
                    },
                    Dimensionality::Face => {
                        ui.label("Quad Direction");
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut settings.face_slicing, SlicingMethod::X, "X|Axial");
                            ui.selectable_value(&mut settings.face_slicing, SlicingMethod::Y, "Y|Radial");
                            ui.selectable_value(&mut settings.face_slicing, SlicingMethod::Z, "Z|Concentric");
                        });
                        ui.checkbox(&mut settings.discrete_color, "Discrete Color");

                    },
                    Dimensionality::Volume => {
                        ui.checkbox(&mut settings.discrete_color, "Discrete Color");
                    },
                }

            },
            ThreeDimensionSettings::Controls => {
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

            },
            ThreeDimensionSettings::Attribution => {
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
            },
        }

        ui.separator();
        global_theme_preference_buttons(ui);
        };

    });

}

fn ui_channel(ui: &mut egui::Ui, label: &str, channel: &mut ColorChannel, width: f32) {
    // Steps
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(
            egui::DragValue::new(&mut channel.steps)
                .range(1..=24)
                .prefix("Steps: "),
        );
    });

    // Step type
    ui.horizontal(|ui| {
        ui.selectable_value(&mut channel.step_type, StepType::Forward, "Forward");
        ui.selectable_value(&mut channel.step_type, StepType::Reverse, "Backward");

        if channel.steps == 1 && channel.step_type == StepType::Inclusive {
            channel.step_type = StepType::Forward;
        } else {
            ui.selectable_value(&mut channel.step_type, StepType::Inclusive, "Inclusive");
        }
    });

    // Start/End slider
    let mut start = channel.start;
    let mut end = channel.end;

    ui.horizontal(|ui| {
        ui.add(
            DoubleSlider::new(&mut start, &mut end, 0.0..=1.0)
                .width(width)
                .separation_distance(0.0),
        );
    });

    channel.start = start;
    channel.end = end;
}