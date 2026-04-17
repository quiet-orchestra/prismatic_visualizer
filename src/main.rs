//Quiet Orchestra
//Prismatic Color Visualizer

use bevy::{prelude::*, render::view::NoIndirectDrawing};
use bevy_egui::{
    EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext
};

mod camera;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use camera::camera_controls;

mod ui;
use ui::{ui, Settings, CurrentVizCategory};

mod three_dim_viz;
use three_dim_viz::{
    spawn_3d_visualization, 
    spawn_grid, 
    VisualizationMesh, 
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

use crate::{two_dim_viz::SceneConfig, ui::SettingsMenus};


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
        .add_systems(OnEnter(CurrentVizCategory::ThreeDim), setup_three_cam)
        .add_systems(OnEnter(CurrentVizCategory::TwoDim), setup_two_cam)
        .add_systems(Update, (update_visualization, update_gizmo_config, update_grid))
        .add_systems(FixedUpdate, camera_controls)
        .add_systems(EguiPrimaryContextPass, ui)
        .run();
}
 
fn setup(
    gizmos: Gizmos,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    point_clouds: ResMut<Assets<PointCloud>>,
    point_cloud_materials: ResMut<Assets<PointCloudMaterial>>,
) {

    commands.spawn((
        PrimaryEguiContext,
        PanOrbitCamera::default(),
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
    ));


    let settings = Settings {
        ..Default::default()
    };

    commands.insert_resource(settings);
    commands.insert_resource(SettingsMenus::new(settings));

    spawn_3d_visualization(gizmos, commands, meshes, materials, point_clouds, point_cloud_materials, &settings);

}
 
fn setup_three_cam(
    mut commands: Commands,
    cam_query: Query<Entity,With<Camera>>
){
    commands.entity(cam_query.single().unwrap()).despawn();
    commands.spawn((
        PanOrbitCamera::default(),
        Transform::from_xyz(SCALE*2., SCALE*2., SCALE*2.)
        .looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
    ));
}

fn setup_two_cam(
    mut commands: Commands,
    cam_query: Query<Entity,With<Camera>>
){
    commands.entity(cam_query.single().unwrap()).despawn();
    commands.spawn(Camera2d::default());
}

fn update_visualization(
    gizmos: Gizmos,
    mut commands: Commands,
    settings: ResMut<Settings>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    point_clouds: ResMut<Assets<PointCloud>>,
    point_cloud_materials: ResMut<Assets<PointCloudMaterial>>,
    entities: Query<Entity, With<VisualizationMesh>>,
    windows: Query<&Window>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut scene_config: ResMut<SceneConfig>,
) 
 {



    if settings.is_changed() {
 
        //Delete previous visualization 
        for mesh in entities.iter(){
            commands.entity(mesh).despawn();
        }
  
        match settings.current_viz {
            CurrentVizCategory::TwoDim => {
                scene_config.spawn_scene(windows, &mut commands, &mut meshes, &mut color_materials, &mut images);
            },
            CurrentVizCategory::ThreeDim => {
                spawn_3d_visualization(gizmos, commands, meshes, materials, point_clouds, point_cloud_materials, & *settings);
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

