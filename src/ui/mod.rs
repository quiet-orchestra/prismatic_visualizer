use bevy::{prelude::{ResMut, Resource},};
use bevy_egui::{
    EguiContexts,
    egui::{
        self,
        global_theme_preference_buttons
    }
};

pub(crate) mod ui_traits;

use crate::{
    three_dim_viz::{
        Attribution,
        ColorChannelSettings,
        ColorModelSettings,
        ControlSettings,
        DimensionalitySettings,
        GridSettings,
        PerceptualOffsetSettings,
        ScaleSettings,
    },

    ui::ui_traits::Setting};

#[derive(Resource, Clone)]
pub struct Settings{
    pub minimized: bool,

    pub three_dimension_settings: ThreeDimensionSettings,

    pub scale_settings: ScaleSettings,

    pub grid_settings: GridSettings,

    pub perceptual_offset_settings: PerceptualOffsetSettings,

    pub color_channel_settings: ColorChannelSettings,

    pub color_model_settings: ColorModelSettings,

    pub dimensionality_settings: DimensionalitySettings,

    pub controls_settings: ControlSettings,

    pub attribution: Attribution,


}



impl Default for Settings{
    fn default() -> Self {
        Self {
            minimized: false,

            three_dimension_settings: ThreeDimensionSettings::Scale,

            scale_settings: ScaleSettings::default(),

            grid_settings: GridSettings::default(),

            perceptual_offset_settings: PerceptualOffsetSettings::default(),

            color_channel_settings: ColorChannelSettings::default(),

            color_model_settings: ColorModelSettings::default(),

            dimensionality_settings: DimensionalitySettings::default(),

            controls_settings: ControlSettings::default(),

            attribution: Attribution::default(), 

        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ThreeDimensionSettings {
    Scale,
    PerceptualOffset,
    ChannelSettings,
    ColorModel,
    // ColorSpace,
    Dimensionality,
    Controls,
    Attribution,
}

pub fn ui_overlay(mut contexts: EguiContexts, mut settings: ResMut<Settings>) {

    //Create window for variable sliders
    egui::TopBottomPanel::top("Settings")
        .resizable(true)
        .show(contexts.ctx_mut().unwrap(), | ui|{
        egui::Sense::hover();
        

        ui.horizontal(|ui|{
            let min_text = if settings.minimized {"Ʌ"} else {"V"};
            let is_minimized = settings.minimized;
            ui.selectable_value(&mut settings.minimized, !is_minimized , min_text);
            ui.label("3D Settings");
        });
        ui.separator();

        if !settings.minimized {
            ui.horizontal_wrapped(|ui| {
                ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::Scale, "Scale");
                ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::PerceptualOffset, "Perceptual Offset");
                ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::ChannelSettings, "Channel Settings");
                ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::ColorModel, "Color Model");
                // ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::ColorSpace, "Color Space");
                ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::Dimensionality, "Shape");
                ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::Controls, "Controls");
                ui.selectable_value(&mut settings.three_dimension_settings, ThreeDimensionSettings::Attribution, "Attribution");
            });

           ui.separator(); 
        
        match settings.three_dimension_settings {
            ThreeDimensionSettings::Scale => {
                settings.scale_settings.ui(ui);
                settings.grid_settings.ui(ui);
            },
            ThreeDimensionSettings::PerceptualOffset => {
                settings.perceptual_offset_settings.ui(ui);
            },
            ThreeDimensionSettings::ChannelSettings => {
                settings.color_channel_settings.ui(ui);
            },
            ThreeDimensionSettings::ColorModel => {
                settings.color_model_settings.ui(ui);
            },
            // ThreeDimensionSettings::ColorSpace => {
                
            // },
            ThreeDimensionSettings::Dimensionality => {
                settings.dimensionality_settings.ui(ui);

            },
            ThreeDimensionSettings::Controls => {
                settings.controls_settings.ui(ui);
            },
            ThreeDimensionSettings::Attribution => {
                settings.attribution.ui(ui);
            },
        }

        ui.separator();
        global_theme_preference_buttons(ui);
        };

    });

}
