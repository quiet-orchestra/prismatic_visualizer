use bevy::{prelude::{ResMut, Resource}, state::state::States};
use bevy_egui::{
    EguiContexts,
    egui::{
        self,
        global_theme_preference_buttons
    }
};

pub(crate) mod ui_traits;

use crate::{
    ViewportState, three_dim_viz::{
        Attribution,
        ColorChannelSettings,
        ColorModelSettings,
        ControlSettings,
        DimensionalitySettings,
        GridSettings,
        PerceptualOffsetSettings,
        ScaleSettings,
    }, ui::ui_traits::{Setting, SettingsMenu}
};


#[derive(Resource, Clone, Copy)]
pub struct Settings{
    pub viewport_state: ViewportState,

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
            viewport_state: ViewportState::default(),

            //Three Dim
            scale_settings: ScaleSettings::default(),

            grid_settings: GridSettings::default(),

            perceptual_offset_settings: PerceptualOffsetSettings::default(),

            color_channel_settings: ColorChannelSettings::default(),

            color_model_settings: ColorModelSettings::default(),

            dimensionality_settings: DimensionalitySettings::default(),

            controls_settings: ControlSettings::default(),

            attribution: Attribution::default(), 

            //Two Dim


        }
    }
}

#[derive(Resource)]
pub struct SettingsMenus{
    pub three_dim: SettingsMenu,
}

impl SettingsMenus {
    pub fn new(settings: Settings) -> SettingsMenus {
        SettingsMenus { 
            three_dim: SettingsMenu::new(
                "Three Channel Color Viz",
                vec![
                    Box::new(settings.scale_settings),
                    Box::new(settings.grid_settings),
                    Box::new(settings.perceptual_offset_settings),
                    Box::new(settings.color_channel_settings),
                    Box::new(settings.color_model_settings),
                    Box::new(settings.dimensionality_settings),
                    Box::new(settings.controls_settings),
                    Box::new(settings.attribution),
                ],
            ) 
        }
    }
}


pub fn ui(
    mut contexts: EguiContexts,
    mut settings: ResMut<Settings>,
    mut settings_menus: ResMut<SettingsMenus>,
) {

    //Create window for variable sliders
    egui::TopBottomPanel::top("Settings")
        .resizable(true)
        .show(contexts.ctx_mut().unwrap(), | ui|{
        egui::Sense::hover();

        ui.horizontal(|ui|{
            ui.selectable_value(&mut settings.viewport_state, ViewportState::ThreeDimOnly, "3D");
            ui.selectable_value(&mut settings.viewport_state, ViewportState::TwoDimOnly, "2D");
            ui.selectable_value(&mut settings.viewport_state, ViewportState::SplitDim, "Split");
        });
        
        match settings.viewport_state {
            ViewportState::ThreeDimOnly => {
                settings_menus.three_dim.ui(ui)
            },
            ViewportState::TwoDimOnly => {

            },
            ViewportState::SplitDim => {
                settings_menus.three_dim.ui(ui)
            },
        }
        

        ui.separator();
        global_theme_preference_buttons(ui);
    
    });
}
