//Quiet Orchestra
//Prismatic Color Visualizer

use bevy::{camera::Viewport, prelude::*, render::view::NoIndirectDrawing};
use bevy_egui::{
    EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext
};

mod camera;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use camera::camera_controls;

mod ui;
use ui::{ui, Settings};

mod three_dim_viz;
use three_dim_viz::{
    spawn_3d_visualization, 
    spawn_grid, 
    ThreeDimMesh, 
    SCALE,
};

mod two_dim_viz;
use two_dim_viz::TwoDimViz;

use bevy_pointcloud::{
    render::PointCloudRenderMode, 
    PointCloudPlugin, 
    point_cloud::{PointCloud}, 
    point_cloud_material::PointCloudMaterial,
};

use crate::{two_dim_viz::{TwoDimSceneConfig, TwoDimMesh}, ui::SettingsMenus};

pub const UI_RENDER_LAYER: isize = 99;
pub const THREE_DIM_RENDER_LAYER: isize = 10;
pub const TWO_DIM_RENDER_LAYER: isize = 5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set( WindowPlugin {
            primary_window: Some(Window {
                title: "Prismatic Visualizer".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(PointCloudPlugin)
        .add_plugins(TwoDimViz)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_visualization, update_gizmo_config, update_grid, update_viewports))
        .add_systems(FixedUpdate, camera_controls)
        .add_systems(EguiPrimaryContextPass, ui)
        .run();
}

#[derive(Component)]
pub struct UiCamera;

#[derive(Component)]
pub struct ThreeDimCamera;

#[derive(Component)]
pub struct TwoDimCamera;
 
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
enum ViewportState {
    #[default]
    ThreeDimOnly,
    TwoDimOnly,
    SplitDim,
}

fn setup(
    mut gizmos: Gizmos,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut point_clouds: ResMut<Assets<PointCloud>>,
    mut point_cloud_materials: ResMut<Assets<PointCloudMaterial>>,
    window: Single<&Window>
) {

    let window_size = window.resolution.physical_size();
    let (window_width, window_height) = (window_size.x, window_size.y);
    let ui_height =  window_height / 5;
    
    commands.spawn((
        PrimaryEguiContext,
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::ZERO,
                physical_size: UVec2 { x: window_width, y: ui_height },
                ..default()
            }),
            order: UI_RENDER_LAYER,
            ..default()
        },
        UiCamera,
    ));


    commands.spawn((
        PanOrbitCamera::default(),
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2 { x: 0, y: ui_height },
                physical_size: UVec2 { x: window_width, y: window_height - ui_height },
                ..default()
            }),
            order: THREE_DIM_RENDER_LAYER,
            ..default()
        },
        Transform::from_xyz(SCALE*2., SCALE*2., SCALE*2.)
        .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        NoIndirectDrawing,
        Msaa::Off,
        PointCloudRenderMode {
            use_edl: false,
            edl_radius: 2.8,
            edl_strength: 0.4,
            edl_neighbour_count: 4,
            ..Default::default()
        },
        ThreeDimCamera,
    ));

        commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2 { x: 0,  y: ui_height },
                physical_size: UVec2 { x: 0, y: window_height - ui_height },
                ..default()
            }),
            order: TWO_DIM_RENDER_LAYER,
            ..default()
        },
        TwoDimCamera,
    ));


    let settings = Settings {
        ..Default::default()
    };

    commands.insert_resource(settings);
    commands.insert_resource(SettingsMenus::new(settings));

    spawn_3d_visualization(&mut gizmos, &mut commands, &mut meshes, &mut materials, &mut point_clouds, &mut point_cloud_materials, &settings);

}

fn update_viewports(
    settings: ResMut<Settings>,
    mut cameras: ParamSet<(
        Single<&mut Camera, With<ThreeDimCamera>>,
        Single<&mut Camera, With<TwoDimCamera>>,
    )>,
    window: Single<&Window>
) {
    let window_size = window.resolution.physical_size();
    let (window_width, _) = (window_size.x, window_size.y);

    let mut three_dim_camera= cameras.p0().into_inner();
    let three_dim_viewport = three_dim_camera.viewport.as_mut().unwrap();

    match settings.viewport_state {
        ViewportState::ThreeDimOnly => {
            three_dim_viewport.physical_size = three_dim_viewport.physical_size.with_x(window_width);
        },
        ViewportState::TwoDimOnly => {
            three_dim_viewport.physical_size = three_dim_viewport.physical_size.with_x(0);
        },
        ViewportState::SplitDim => {
            three_dim_viewport.physical_size = three_dim_viewport.physical_size.with_x(window_width / 2);
        },
    }

    let mut two_dim_camera= cameras.p1().into_inner();
    let two_dim_viewport = two_dim_camera.viewport.as_mut().unwrap();

    match settings.viewport_state {
        ViewportState::ThreeDimOnly => {
            two_dim_viewport.physical_size = two_dim_viewport.physical_size.with_x(0);
        },
        ViewportState::TwoDimOnly => {
            two_dim_viewport.physical_position = two_dim_viewport.physical_position.with_x(0);
            two_dim_viewport.physical_size = two_dim_viewport.physical_size.with_x(window_width);
        },
        ViewportState::SplitDim => {
            two_dim_viewport.physical_position = two_dim_viewport.physical_position.with_x(window_width / 2);
            two_dim_viewport.physical_size = two_dim_viewport.physical_size.with_x(window_width / 2);           
        },
    }
}

fn update_visualization(
    mut gizmos: Gizmos,
    mut commands: Commands,
    mut settings:  ResMut<Settings>,
    mut meshes:  ResMut<Assets<Mesh>>,
    mut materials:  ResMut<Assets<StandardMaterial>>,
    mut point_clouds:  ResMut<Assets<PointCloud>>,
    mut point_cloud_materials:  ResMut<Assets<PointCloudMaterial>>,
    mut color_materials:  ResMut<Assets<ColorMaterial>>,
    mut images:  ResMut<Assets<Image>>,
    three_dim_entities: Query<Entity, With<ThreeDimMesh>>,
    two_dim_entities: Query<Entity, With<TwoDimMesh>>,
    two_dim_scene_config:  ResMut<TwoDimSceneConfig>,
    windows: Query<&Window>,
) 
 {



    if settings.is_changed() {

        //Despawn any existing viz meshes
        for mesh in three_dim_entities.iter(){
            commands.entity(mesh).despawn();
        }
        for mesh in two_dim_entities.iter(){
            commands.entity(mesh).despawn();
        }
        
        //Spawn new visualization
        match settings.viewport_state {
            ViewportState::ThreeDimOnly => {
                spawn_3d_visualization(&mut gizmos, &mut commands, &mut meshes, &mut materials, &mut point_clouds, &mut point_cloud_materials, &mut settings);  
            },
            ViewportState::TwoDimOnly => {
                two_dim_scene_config.spawn_scene(windows, &mut commands, &mut meshes, &mut color_materials, &mut images);
            },
            ViewportState::SplitDim => {
                spawn_3d_visualization(&mut gizmos, &mut commands, &mut meshes, &mut materials, &mut point_clouds, &mut point_cloud_materials, &mut settings);  
                two_dim_scene_config.spawn_scene(windows, &mut commands, &mut meshes, &mut color_materials, &mut images);
            },
        }
      
        

    }
 }



 fn update_grid(
    gizmos: Gizmos,
    settings: Res<Settings>,
 ){
    let grid_settings= settings.grid_settings;
    spawn_grid(gizmos, grid_settings);
 }

fn update_gizmo_config(
    mut config_store: ResMut<bevy::prelude::GizmoConfigStore>,
    settings: Res<Settings>,
) {
    if settings.is_changed() {
        let (config, _handle) = config_store.config_mut::<bevy::prelude::DefaultGizmoConfigGroup>();
        config.line.width = settings.dimensionality_settings.line_width * 5. ;
        config.line.joints = GizmoLineJoint::Miter;

    }
}

