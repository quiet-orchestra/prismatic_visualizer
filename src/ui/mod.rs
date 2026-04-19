use std::ops::IndexMut;

use bevy::{prelude::*};
use bevy_egui::{
    EguiContexts,
    egui::{
        self, Ui, global_theme_preference_buttons
    }
};

pub(crate) mod ui_traits;

use crate::{
    UiCamera, ViewportState, three_dim_viz::{
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

    pub settings_menu: SettingsMenu,

}

impl Settings {

    //Want to make this generic. I think I can if I pull out the settings_menu into its own struct, but I'm not sure about borrowing issues
    // fn get_3D_settings(&mut self) -> Vec<&mut dyn Setting> {
    //     vec![
    //         &mut self.scale_settings,
    //         &mut self.grid_settings,
    //         &mut self.perceptual_offset_settings,
    //         &mut self.color_channel_settings,
    //         &mut self.color_model_settings,
    //         &mut self.dimensionality_settings,
    //         &mut self.controls_settings,
    //         &mut self.attribution,
    //     ]
    // }

    pub fn three_dim_ui(&mut self, ui: &mut Ui){
        let mut settings_list: Vec<&mut dyn Setting> = //self.get_3D_settings();
        vec![
            &mut self.scale_settings,
            &mut self.grid_settings,
            &mut self.perceptual_offset_settings,
            &mut self.color_channel_settings,
            &mut self.color_model_settings,
            &mut self.dimensionality_settings,
            &mut self.controls_settings,
            &mut self.attribution,
        ];

        ui.horizontal(|ui| {

            // ui.label(self.settings_menu.heading);

            let min_text = if self.settings_menu.minimized {"Ʌ"} else {"V"};
            let is_minimized = self.settings_menu.minimized;
            ui.selectable_value(&mut self.settings_menu.minimized, !is_minimized , min_text);
            
            ui.separator();

            if !self.settings_menu.minimized {
                ui.horizontal_wrapped(|ui|{
                    for (i, setting) in settings_list.iter().enumerate() {
                        ui.selectable_value( &mut self.settings_menu.index,  i, setting.heading());
                    }  
                });

            }

        });

        ui.separator();

        if !self.settings_menu.minimized {
            if let Some(setting) = settings_list.get_mut(self.settings_menu.index) {
                setting.ui(ui);
            }
        }




    }
}

impl Default for Settings{
    fn default() -> Self {
        Self {

            viewport_state: ViewportState::default(),
            
            scale_settings: ScaleSettings::default(),

            grid_settings: GridSettings::default(),

            perceptual_offset_settings: PerceptualOffsetSettings::default(),

            color_channel_settings: ColorChannelSettings::default(),

            color_model_settings: ColorModelSettings::default(),

            dimensionality_settings: DimensionalitySettings::default(),

            controls_settings: ControlSettings::default(),

            attribution: Attribution::default(), 

            settings_menu: SettingsMenu::new("Three Dim Viz"),


        }
    }
}

pub fn ui(
    mut contexts: EguiContexts,
    mut settings: ResMut<Settings>,
    camera: Single<&Camera, With<UiCamera>>,
) {

    let size = camera.into_inner().viewport.as_ref().unwrap().physical_size;
    let (_ , height) = (size.x as f32 , size.y as f32);


    egui::TopBottomPanel::top("Settings")
        .exact_height(height)
        .show(contexts.ctx_mut().unwrap(), | ui|{
        egui::Sense::hover();

        ui.horizontal(|ui|{
            ui.selectable_value(&mut settings.viewport_state, ViewportState::ThreeDimOnly, "3D");
            ui.selectable_value(&mut settings.viewport_state, ViewportState::TwoDimOnly, "2D");
            ui.selectable_value(&mut settings.viewport_state, ViewportState::SplitDim, "Split");
            ui.separator();
            global_theme_preference_buttons(ui);
        });

        egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
        
            match settings.viewport_state {
                ViewportState::ThreeDimOnly => {
                    settings.three_dim_ui(ui);
                },
                ViewportState::TwoDimOnly => {

                },
                ViewportState::SplitDim => {
                    settings.three_dim_ui(ui);
                },
            }

        });


    
    });
}
