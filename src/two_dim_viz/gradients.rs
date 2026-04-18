use bevy::{
    prelude::*, 
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    asset::RenderAssetUsages,
};

use prismatic_color::{linear_gradient, Color as P_Color, constants as Color_Names};

use crate::two_dim_viz::TwoDimMesh;


pub fn spawn(
    width: f32,
    height: f32,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    images: &mut ResMut<Assets<Image>>,
    color_sets: Vec<Vec<P_Color>>,
){
    let scale = (1./2.,1./8.);
    let (scaled_width, scaled_height) = ( width * scale.0, height * scale.1
    );
    let top = height / 2. * (1. - scale.0);
    
    for i in 0..color_sets.len(){
        let rectangle_mesh = Mesh2d(meshes.add(Rectangle::new(scaled_width, scaled_height)));
        let color_pair = color_sets.get(i).expect("Gradient Missing");
        let (start, end) = (
            color_pair.get(0).expect("Missing gradient start"), 
            color_pair.get(1).expect("Missing gradient end"),
        );
        let image_handle = images.add(gradient_texture(start, end, scaled_width, scaled_height));
        commands.spawn((
            rectangle_mesh,
            MeshMaterial2d(materials.add(ColorMaterial{
                texture: Some(image_handle),
                ..Default::default()
            })),
            Transform::from_xyz(0.0, top - scaled_height * 1.1 * i as f32, 0.0 ),
        )).insert(TwoDimMesh{});
    }

}

pub fn generate_colors() -> Vec<Vec<P_Color>> {
    let gradient_one = vec![
        Color_Names::BLACK,
        Color_Names::RED,
    ];
    let gradient_two = vec![
        Color_Names::BLACK,
        Color_Names::GREEN,
    ];
    let gradient_three = vec![
        Color_Names::BLACK,
        Color_Names::BLUE,
    ];
    let gradient_four = vec![
        P_Color::spherical_hcl(0.,1.,1.),
        P_Color::spherical_hcl(0.9999, 1., 1.),
    ];

    let gradients = vec![
        gradient_one, 
        gradient_two, 
        gradient_three,
        gradient_four,
    ];

    return gradients
}

fn gradient_texture(
    start: &P_Color, 
    end: &P_Color, 
    width: f32, 
    height: f32,
) -> Image {
    let (width, height) = (width.floor() as usize, height.floor() as usize);
    let gradient = linear_gradient(&start, &end, width);

    let mut texture_data: Vec<u8> = Vec::with_capacity(height * width * 4);

    for _ in 0..height{
        for y in 0..width {
            let color = gradient.get(y).expect("Gradient Color Access Invalid");
            let color_data = color.to_linear_rgb().to_u8_array();
            texture_data.extend_from_slice(&color_data);
        }
    }

    
    // Create the image from the packed texture data
    let image = Image::new(
        Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        texture_data,
        TextureFormat::Rgba8UnormSrgb, 
        RenderAssetUsages::RENDER_WORLD,
    );
    return image;
}
