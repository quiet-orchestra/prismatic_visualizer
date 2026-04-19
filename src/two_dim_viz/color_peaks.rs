use bevy::{
    prelude::*,
};

use prismatic_color::{Color as P_Color, constants as Color_Names};

use crate::two_dim_viz::TwoDimMesh;

trait BevyColorConvert {
    fn to_bevy_color(&self) -> Color;
}

impl BevyColorConvert for P_Color {
    fn to_bevy_color(&self) -> Color {
        let color = self.to_rgb().to_array();
        Color::srgba(color[0], color[1], color[2], color[3])
    }
}

pub fn generate_colors() -> Vec<Vec<P_Color>>{
    Color_Names::QUATERNARY_COLORS.iter()
    .skip(1) // Skip the zeroth row
    .map(|row| row.iter()
                  .map(|&color| color) // Apply to_bevy_color to each element
                  .collect()
    )
    .collect()
}

pub fn spawn(
    width: f32,
    height: f32,
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    color_sets: Vec<Vec<P_Color>>,
){
    let offset_ratio = 8./10.;
    let (width, height) = ( 
        width * offset_ratio, 
        height * offset_ratio,
    );

    let closest_ratio = find_closest_ratio(width, height);

    // Calculate the dimensions for the rectangles based on the closest ratio
    let rect_width = width / closest_ratio.0 as f32;
    let rect_height = height / closest_ratio.1 as f32;

    let (top, left) = (height / 2., width / -2.);
 
 let mut num_tile = 0;

    // Draw rectangles
    for row in 0..closest_ratio.1 {
        for col in 0..closest_ratio.0 {
            let colors = color_sets.get(num_tile).unwrap();

            let rect_x = left + rect_width * (col as f32);
            let rect_y = top - rect_height * (row as f32);

            let hue_tile = Tile::new_from_top_left(rect_x, rect_y, rect_width, rect_height);
            draw_hue_tile(commands,  materials, meshes, colors, hue_tile);

            num_tile += 1;
        }
    }
}

struct Tile{
    top_left: (f32,f32),
    bottom_right: (f32,f32),
}

impl Tile {
    fn new_from_top_left(x: f32 , y: f32, width: f32, height: f32) -> Tile {
        Tile { top_left: (x,y), bottom_right: (x + width, y - height) }
    }
    // fn new_from_center(x: f32 , y: f32, width: f32, height: f32) -> Tile {
    //     Tile::new_from_top_left(x - width / 2. , y + height / 2., width, height)
    // }
    fn center(&self) -> (f32,f32) {
        ((self.top_left.0 + self.bottom_right.0) / 2., (self.top_left.1 + self.bottom_right.1) / 2. )
    }
    fn top_left(&self) -> (f32,f32) {
        self.top_left
    }
    fn top_right(&self) -> (f32,f32) {
        (self.bottom_right.0, self.top_left.1)
    }
    fn bottom_left(&self) -> (f32,f32) {
        (self.top_left.0, self.bottom_right.1)
    }
    fn bottom_right(&self) -> (f32,f32) {
        self.bottom_right
    }
        
}

fn draw_hue_tile(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    colors: &Vec<P_Color>,
    tile: Tile,
){

    let tl = Vec2::from(tile.top_left());
    let tr = Vec2::from(tile.top_right());
    let bl = Vec2::from(tile.bottom_left());
    let br = Vec2::from(tile.bottom_right());
    let ct = Vec2::from(tile.center());

    let tile_pos_array = [tl, tr, br, bl];

    for (i, color) in colors.into_iter().enumerate() {
        let triangle_mesh = Mesh2d(
            meshes.add(
                Triangle2d::new(
                    tile_pos_array[i], 
                    tile_pos_array[if (i + 1) < 4 {i + 1} else {0}], 
                    ct,
                )
            )
        );
        commands.spawn((
            triangle_mesh,
            MeshMaterial2d(materials.add(color.to_linear_rgb().to_bevy_color())),
        )).insert(TwoDimMesh{});
    }

}

fn find_closest_ratio(width: f32, height: f32) -> (usize, usize) {
    let current_ratio = width / height;
    let defined_ratios = [
        (1, 24),
        (2, 12),
        (3, 8),
        (4, 6),
        (6, 4),
        (8, 3),
        (12, 2),
        (24, 1),
    ];

    let closest_ratio = defined_ratios
        .iter()
        .min_by_key(|&&(x, y)| {
            let ratio = x as f32 / y as f32;
            ((ratio - current_ratio).abs() * 1000.0) as usize // Scale the difference for comparison
        })
        .unwrap();

    *closest_ratio
}