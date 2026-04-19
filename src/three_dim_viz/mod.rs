
use bevy_egui::egui::emath::OrderedFloat;
use indexmap::IndexMap;
use prismatic_color::{Color as P_Color, ColorSpace, constants};

use bevy_pointcloud::{
    point_cloud::{PointCloud, PointCloud3d, PointCloudData},
    point_cloud_material::{PointCloudMaterial, PointCloudMaterial3d},
};

use bevy::{
    asset::RenderAssetUsages, mesh::Indices, prelude::*, render::render_resource::PrimitiveTopology
};

use crate::ui::*;

mod scale_settings;
pub use scale_settings::ScaleSettings;

mod grid_settings;
pub use grid_settings::{GridCategory, GridSettings};

mod perceptual_offset_settings;
pub use perceptual_offset_settings::PerceptualOffsetSettings;

mod color_channel_settings;
pub use color_channel_settings::ColorChannelSettings;

mod color_model_settings;
pub use color_model_settings::ColorModelSettings;

mod dimensionality_settings;
pub use dimensionality_settings::DimensionalitySettings;

mod controls_settings;
pub use controls_settings::ControlSettings;

mod attribution;
pub use attribution::Attribution;

// A marker component for our components so we can query them separately from the ground plane
#[derive(Component)]
pub struct ThreeDimMesh;



#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorModelCategory {
    Spherical,
    Cubic,
    LumaChroma,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SlicingMethod {
    X,
    Y,
    Z,
}

impl SlicingMethod {
    fn get_face_offsets(&self) -> [[usize; 3]; 4] {
        match self {
            SlicingMethod::X => [
                [0, 0, 0],
                [0, 1, 0],
                [0, 1, 1],
                [0, 0, 1],
            ],
            SlicingMethod::Y => [                
                [0, 0, 0], 
                [1, 0, 0], 
                [1, 0, 1], 
                [0, 0, 1], 
            ],
            SlicingMethod::Z => [
                [0, 0, 0],
                [1, 0, 0],
                [1, 1, 0],
                [0, 1, 0], 
            ],

        }
    }

    fn get_edge_offsets(&self) -> [[usize; 3]; 2] {
        match self {
            SlicingMethod::X => [
                [0, 0, 0],
                [1, 0, 0],
            ],
            SlicingMethod::Y => [
                [0, 0, 0],
                [0, 1, 0], 
            ],
            SlicingMethod::Z => [
                [0, 0, 0], 
                [0, 0, 1], 
            ],

        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dimensionality {
    Vertex,
    Edge,
    Face,
    Volume,
}

pub enum DimensionList {
    Vertex(VertexList),
    Edge(EdgeList),
    Face(FaceList),
    Volume(FaceList),
}

impl DimensionList {
    pub fn render(
        &self,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        point_clouds: &mut Assets<PointCloud>,
        point_cloud_material: &mut Assets<PointCloudMaterial>,
        gizmos: &mut Gizmos,
        settings: &Settings,
    ) {
        match self {
            DimensionList::Vertex(vertex_list) => {
                // Render vertices as a point cloud
                let points: Vec<PointCloudData> = vertex_list
                    .iter()
                    .map(|(vertex_object, _)| {
                        let position: Vec3 =
                            vertex_object.point.into_vec3() * SCALE * settings.scale_settings.viz_scale;
                        let color = P_Color::from_array(
                            vertex_object.color.map(|x| x.into_inner()),
                            settings.color_model_settings.color_model,
                        )
                        .to_bevy_color()
                        .to_srgba()
                        .to_f32_array();
                        PointCloudData {
                            position,
                            point_size: settings.dimensionality_settings.instance_scale * SCALE,
                            color,
                        }
                    })
                    .collect::<Vec<_>>();

                    let my_material = point_cloud_material.add(PointCloudMaterial {
                        point_size: 50.0 * settings.dimensionality_settings.instance_scale * SCALE,
                        ..default()
                    });

                    let point_cloud: Handle<PointCloud> = point_clouds.add(PointCloud{
                        points
                    });
                    commands.spawn((
                        PointCloud3d(point_cloud),
                        PointCloudMaterial3d(my_material),
                        ThreeDimMesh,
                    )); 
             },
             DimensionList::Edge(edge_list) => {
                // Render edges with gizmo lines
                for edge in &edge_list.edges {
                    let vertex_1 = edge_list.vertex_registry.get_index(edge.0).unwrap().0;
                    let vertex_2 = edge_list.vertex_registry.get_index(edge.1).unwrap().0;
                    
                    if settings.dimensionality_settings.discrete_color {
                        gizmos.line(vertex_1.point.map(|axis| axis.into_inner() * SCALE * settings.scale_settings.viz_scale).into(), vertex_2.point.map(|axis| axis.into_inner() * SCALE * settings.scale_settings.viz_scale).into(), vertex_1.color.into_color(settings));
                    } 
                    else {
                        gizmos.line_gradient(vertex_1.point.map(|axis| axis.into_inner() * SCALE * settings.scale_settings.viz_scale).into(), vertex_2.point.map(|axis| axis.into_inner() * SCALE * settings.scale_settings.viz_scale).into(), vertex_1.color.into_color(settings), vertex_2.color.into_color(settings));
                    }
                }
            },       
            DimensionList::Face(face_list) => {
                //Render faces with a triangle based mesh
                // Collect positions, normals, and colors
                let mut positions: Vec<[f32; 3]> = Vec::new();
                let mut normals: Vec<[f32; 3]> = Vec::new();
                let mut colors: Vec<[f32; 4]> = Vec::new();
                let mut indices: Vec<u32> = Vec::new();

                for (i1, i2, i3, i4) in &face_list.faces {
                    // Lookup vertices from registry
                    let v1 = face_list.vertex_registry.get_index(*i1).unwrap().0;
                    let v2 = face_list.vertex_registry.get_index(*i2).unwrap().0;
                    let v3 = face_list.vertex_registry.get_index(*i3).unwrap().0;
                    let v4 = face_list.vertex_registry.get_index(*i4).unwrap().0;

                    let verts = [v1, v2, v3, v4];

                    // Push positions/colors

                    
                    let base = positions.len() as u32;
                    for v in &verts {
                        positions.push(v.point.map(|p| p.into_inner() * SCALE * settings.scale_settings.viz_scale));
                        let color = 
                            if settings.dimensionality_settings.discrete_color {
                                P_Color::from_array(v1.color.map(|x| x.into_inner()), settings.color_model_settings.color_model)
                                .to_bevy_color()
                                .to_linear()
                                .to_f32_array()
                            }
                            else {
                                P_Color::from_array(v.color.map(|x| x.into_inner()), settings.color_model_settings.color_model)
                                .to_bevy_color()
                                .to_linear()
                                .to_f32_array()
                        };
                        colors.push(color);
                    }

                    // Compute a simple normal (cross product of two edges)
                    let p1 = Vec3::from(positions[base as usize]);
                    let p2 = Vec3::from(positions[base as usize + 1]);
                    let p3 = Vec3::from(positions[base as usize + 2]);
                    let normal = (p2 - p1).cross(p3 - p1).normalize_or_zero();
                    for _ in 0..4 {
                        normals.push(normal.into());
                    }

                    // Add indices for two triangles: (0,1,2) and (0,2,3)
                    indices.extend_from_slice(&[
                        base, base + 1, base + 2,
                        base, base + 2, base + 3,
                    ]);
                }

                // Build mesh
                let mut mesh = Mesh::new(
                    PrimitiveTopology::TriangleList,
                    RenderAssetUsages::default(),
                );
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
                mesh.insert_indices(Indices::U32(indices));

                // Unlit so vertex colors are shown directly
                let material = materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    unlit: true,
                    cull_mode: None,
                    ..default()
                });

                commands.spawn((
                     
                    Mesh3d( meshes.add(mesh)),
                    MeshMaterial3d(material),
                    ThreeDimMesh,
                ));
            },
            DimensionList::Volume(face_list) => {
                //Render faces with a triangle based mesh
                // Collect positions, normals, and colors
                let mut positions: Vec<[f32; 3]> = Vec::new();
                let mut normals: Vec<[f32; 3]> = Vec::new();
                let mut colors: Vec<[f32; 4]> = Vec::new();
                let mut indices: Vec<u32> = Vec::new();

                for (i1, i2, i3, i4) in &face_list.faces {
                    // Lookup vertices from registry
                    let v1 = face_list.vertex_registry.get_index(*i1).unwrap().0;
                    let v2 = face_list.vertex_registry.get_index(*i2).unwrap().0;
                    let v3 = face_list.vertex_registry.get_index(*i3).unwrap().0;
                    let v4 = face_list.vertex_registry.get_index(*i4).unwrap().0;

                    let verts = [v1, v2, v3, v4];

                    // Push positions/colors                    
                    let base = positions.len() as u32;
                    for v in &verts {
                        positions.push(v.point.map(|p| p.into_inner() * SCALE * settings.scale_settings.viz_scale));
                        let color = 
                            if settings.dimensionality_settings.discrete_color {
                                P_Color::from_array(v1.color.map(|x| x.into_inner()), settings.color_model_settings.color_model)
                                .to_bevy_color()
                                .to_linear()
                                .to_f32_array()
                            }
                            else {
                                P_Color::from_array(v.color.map(|x| x.into_inner()), settings.color_model_settings.color_model)
                                .to_bevy_color()
                                .to_linear()
                                .to_f32_array()
                        };
                        colors.push(color);
                    }

                    // Compute a simple normal (cross product of two edges)
                    let p1 = Vec3::from(positions[base as usize]);
                    let p2 = Vec3::from(positions[base as usize + 1]);
                    let p3 = Vec3::from(positions[base as usize + 2]);
                    let normal = (p2 - p1).cross(p3 - p1).normalize_or_zero();
                    for _ in 0..4 {
                        normals.push(normal.into());
                    }

                    // Add indices for two triangles: (0,1,2) and (0,2,3)
                    indices.extend_from_slice(&[
                        base, base + 1, base + 2,
                        base, base + 2, base + 3,
                    ]);
                }

                // Build mesh
                let mut mesh = Mesh::new(
                    PrimitiveTopology::TriangleList,
                    RenderAssetUsages::default(), // or RenderAssetUsages::RENDER_WORLD if you only need rendering
                );
                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
                mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
                mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
                mesh.insert_indices(Indices::U32(indices));

                // Unlit so vertex colors are shown directly
                let material = materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    unlit: true,
                    cull_mode: None,
                    ..default()
                });

                commands.spawn((
                     
                    Mesh3d( meshes.add(mesh)),
                    MeshMaterial3d(material),
                    ThreeDimMesh,
                ));
            },
        }
    }
}

trait IntoVec3 {
    fn into_vec3(self) -> Vec3;
}

impl IntoVec3 for [OrderedFloat<f32>; 3] {
    fn into_vec3(self) -> Vec3 {
        let [x, y, z] = self;
        Vec3::new(x.into_inner(), y.into_inner(), z.into_inner())
    }
}

trait OrderedArrayIntoColor {
    fn into_color(self, settings: &Settings) -> Color;
}

impl OrderedArrayIntoColor for [OrderedFloat<f32>; 4] {
    fn into_color(self, settings: &Settings) -> Color {
        P_Color::from_array(self.map(|x| x.into_inner()), settings.color_model_settings.color_model).to_bevy_color()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexObject {
    pub point: [OrderedFloat<f32>;3],
    pub color: [OrderedFloat<f32>;4],
}

impl VertexObject {
    fn new(point: [f32;3], color: P_Color) -> VertexObject {
        VertexObject { point: point.map(OrderedFloat::from), color: color.to_array().map(OrderedFloat::from) }
    }
    fn from_tuple(point_color: ([f32;3], P_Color)) -> VertexObject {
        VertexObject::new(point_color.0,point_color.1)
    }
}

pub struct VertexList {
    registry: IndexMap<VertexObject, usize>,
}

impl VertexList {
    pub fn iter(&self) -> impl Iterator<Item = (&VertexObject, &usize)> {
        self.registry.iter()
    }
}

pub struct EdgeList {
    vertex_registry: IndexMap<VertexObject, usize>,
    edges: Vec<(usize, usize)>,
}

pub struct FaceList {
    vertex_registry: IndexMap<VertexObject, usize>,
    faces: Vec<(usize, usize, usize, usize)>,
}

// Common trait for vertex management
pub trait VertexCollection {
    fn add_vertex(&mut self, v1: &VertexObject) -> usize{
        self.get_or_insert_index(v1)
    }
    
    fn get_or_insert_index(&mut self, vertex: &VertexObject) -> usize {
        if let Some(&index) = self.vertex_registry().get(vertex) {
            index
        } else {
            let index = self.vertex_registry().len();
            self.vertex_registry_mut().insert(*vertex, index);
            index
        }
    }
    
    fn vertex_registry(&self) -> &IndexMap<VertexObject, usize>;
    fn vertex_registry_mut(&mut self) -> &mut IndexMap<VertexObject, usize>;
}

impl VertexCollection for VertexList {
    fn vertex_registry(&self) -> &IndexMap<VertexObject, usize> {
        &self.registry
    }

    fn vertex_registry_mut(&mut self) -> &mut IndexMap<VertexObject, usize> {
        &mut self.registry
    }
}

impl VertexList {
    pub fn new() -> Self {
        Self {
            registry: IndexMap::new(),
        }
    }
}

impl EdgeList {
    pub fn new() -> Self {
        Self {
            vertex_registry: IndexMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, v1: VertexObject, v2: VertexObject) -> usize {
        let i1 = self.get_or_insert_index(&v1);
        let i2 = self.get_or_insert_index(&v2);
        self.edges.push((i1, i2));
        self.edges.len() - 1
    }
}

impl VertexCollection for EdgeList {

    fn vertex_registry(&self) -> &IndexMap<VertexObject, usize> {
        &self.vertex_registry
    }

    fn vertex_registry_mut(&mut self) -> &mut IndexMap<VertexObject, usize> {
        &mut self.vertex_registry
    }
}

impl FaceList {
    pub fn new() -> Self {
        Self {
            vertex_registry: IndexMap::new(),
            faces: Vec::new(),
        }
    }

    pub fn add_quad(
        &mut self,
        v1: VertexObject,
        v2: VertexObject,
        v3: VertexObject,
        v4: VertexObject,
    ) -> usize {
        let i1 = self.get_or_insert_index(&v1);
        let i2 = self.get_or_insert_index(&v2);
        let i3 = self.get_or_insert_index(&v3);
        let i4 = self.get_or_insert_index(&v4);
        self.faces.push((i1, i2, i3, i4));
        self.faces.len() - 1
    }
}

impl VertexCollection for FaceList {

    fn vertex_registry(&self) -> &IndexMap<VertexObject, usize> {
        &self.vertex_registry
    }

    fn vertex_registry_mut(&mut self) -> &mut IndexMap<VertexObject, usize> {
        &mut self.vertex_registry
    }
}

fn transform_coordinates(vertex: (f32,f32,f32), settings: &Settings) -> (f32,f32,f32) {
    let (a, b, c) =  vertex;

    if !settings.color_model_settings.mirrored{
        match settings.color_model_settings.rotated {
            RotationChirality::Middle => {
                return (a,b,c);
            },
            RotationChirality::Right => {
                return (b,c,a);
            },
            RotationChirality::Left => {
                return (c,a,b);
            },
        }
    }
    else {
        match settings.color_model_settings.rotated {
            RotationChirality::Middle => {
                return (c,b,a);
            },
            RotationChirality::Right => {
                return (a,c,b);
            },
            RotationChirality::Left => {
                return (b,a,c);
            },
        }
    }

}

pub const SCALE: f32 = 5.0;

trait BevyColorConvert {
    fn to_bevy_color(&self) -> Color;
}

impl BevyColorConvert for P_Color {
    fn to_bevy_color(&self) -> Color {
        let color = self.to_rgb().to_array();
        Color::srgba(color[0], color[1], color[2], color[3])
    }
}

pub fn spawn_3d_visualization(
    gizmos: &mut Gizmos,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    point_clouds: &mut ResMut<Assets<PointCloud>>,
    point_cloud_material: &mut ResMut<Assets<PointCloudMaterial>>,
    settings: &Settings,
    )
{
    generate_dimension_lists(settings)
        .render(commands, meshes, materials, point_clouds, point_cloud_material, gizmos, settings);
}

fn generate_dimension_lists(settings: &Settings) ->  DimensionList{
    
    let (mut dim_list, along_grain): (DimensionList, [bool;3]) = 
    match settings.dimensionality_settings.dimensionality {
        Dimensionality::Vertex => (DimensionList::Vertex(VertexList::new()),
        [false, false, false]
        ),
        Dimensionality::Edge => (DimensionList::Edge(EdgeList::new()),
        settings.dimensionality_settings.face_slicing.get_edge_offsets()[1].map(|x| x != 0 ),
        ),
        Dimensionality::Face => (DimensionList::Face(FaceList::new()),
        settings.dimensionality_settings.face_slicing.get_edge_offsets()[1].map(|x| x == 0 ),
        ),
        Dimensionality::Volume => (DimensionList::Volume(FaceList::new()),
        [false, false, false]
        ),
    };



    let channel_a = Channel::from_tuple(settings.color_channel_settings.0.generate_range(along_grain[0]));
    let channel_b = Channel::from_tuple(settings.color_channel_settings.1.generate_range(along_grain[1]));
    let channel_c = Channel::from_tuple(settings.color_channel_settings.2.generate_range(along_grain[2]));

    for a in 0..channel_a.steps  {
        for b in 0..channel_b.steps {
            for c in 0..channel_c.steps {

                // Generate points of the dimension
                let color_point_1 = (
                    channel_a.generate_point(a, 0),
                    channel_b.generate_point(b, 0),
                    channel_c.generate_point(c, 0),
                );
                let point_1 = VertexObject::from_tuple(get_point_and_color(color_point_1, settings));
                match &mut dim_list {
                    DimensionList::Vertex(vertex_list) => {
                        vertex_list.add_vertex(&point_1);
                    },
                    DimensionList::Edge(edge_list) => {
                        let slice_offset = settings.dimensionality_settings.face_slicing.get_edge_offsets();

                        let offset_a = slice_offset[1][0];
                        let offset_b = slice_offset[1][1];
                        let offset_c = slice_offset[1][2];

                        let color_point_2 = (
                            channel_a.generate_point(a, offset_a),
                            channel_b.generate_point(b, offset_b),
                            channel_c.generate_point(c, offset_c),
                        );

                        let point_2 = VertexObject::from_tuple(get_point_and_color(color_point_2, settings));
                        edge_list.add_edge(point_1, point_2);
                    },
                    DimensionList::Face(face_list) => {
                        let slice_offset = settings.dimensionality_settings.face_slicing.get_face_offsets();

                        let points: [VertexObject; 3] = std::array::from_fn(|i| {
                            let offset_a = slice_offset[i+1][0];
                            let offset_b = slice_offset[i+1][1];
                            let offset_c = slice_offset[i+1][2];

                            let color_point = (
                                channel_a.generate_point(a, offset_a),
                                channel_b.generate_point(b, offset_b),
                                channel_c.generate_point(c, offset_c),
                            );

                            VertexObject::from_tuple(get_point_and_color(color_point, settings))

                        });
                        
                        face_list.add_quad(point_1, points[0], points[1], points[2]);
                    },
                    DimensionList::Volume(face_list) => {
                        // Check each axis separately
                        // X-min and X-max
                        if (a == 0 || a == channel_a.steps - 1) && !(b == channel_b.steps - 1 || c == channel_c.steps - 1) {
                            let offsets = SlicingMethod::X.get_face_offsets();
                            let mut verts = Vec::new();
                            for o in &offsets {
                                let color_point = (
                                    channel_a.generate_point(a, o[0]),
                                    channel_b.generate_point(b, o[1]),
                                    channel_c.generate_point(c, o[2]),
                                );
                                verts.push(VertexObject::from_tuple(get_point_and_color(color_point, settings)));
                            }
                            face_list.add_quad(verts[0], verts[1], verts[2], verts[3]);
                        }

                        // Y-min and Y-max
                        if (b == 0 || b == channel_b.steps - 1) && !(a == channel_a.steps - 1 || c == channel_c.steps - 1) {
                            let slice = SlicingMethod::Y;
                            let offsets = slice.get_face_offsets();
                            let mut verts = Vec::new();
                            for o in &offsets {
                                let color_point = (
                                    channel_a.generate_point(a, o[0]),
                                    channel_b.generate_point(b, o[1]),
                                    channel_c.generate_point(c, o[2]),
                                );
                                verts.push(VertexObject::from_tuple(get_point_and_color(color_point, settings)));
                            }
                            face_list.add_quad(verts[0], verts[1], verts[2], verts[3]);
                        }

                        // Z-min and Z-max
                        if (c == 0 || c == channel_c.steps - 1) && !(b == channel_b.steps - 1 || a == channel_a.steps - 1)  {
                            let slice = SlicingMethod::Z;
                            let offsets = slice.get_face_offsets();
                            let mut verts = Vec::new();
                            for o in &offsets {
                                let color_point = (
                                    channel_a.generate_point(a, o[0]),
                                    channel_b.generate_point(b, o[1]),
                                    channel_c.generate_point(c, o[2]),
                                );
                                verts.push(VertexObject::from_tuple(get_point_and_color(color_point, settings)));
                            }
                            face_list.add_quad(verts[0], verts[1], verts[2], verts[3]);
                        }
                    },
                }
            
            }
        }
    }

    dim_list
}

struct Channel{
    start: f32,
    steps: usize,
    step_size: f32,
}

impl Channel{
    fn from_tuple (tuple: (f32, usize, f32)) -> Channel{
        Channel { start: tuple.0, steps: tuple.1, step_size: tuple.2} 

    }
    fn generate_point (&self, step: usize, offset: usize) -> f32{
        let mut coordinate = self.start + self.step_size * (step + offset) as f32;
        while coordinate > 1.0 {
            coordinate = coordinate - 1.0;
        }
        return coordinate;
    }
}



fn get_point_and_color(base_color: (f32,f32,f32), settings: &Settings) -> ([f32;3], P_Color){
    let (r_gamma,g_gamma,b_gamma) = settings.perceptual_offset_settings.gamma;
    let gamma_adjust = 2.2;
    let gamma = [
        (r_gamma/gamma_adjust) as f32,
        (g_gamma/gamma_adjust) as f32,
        (b_gamma/gamma_adjust) as f32,
    ];
    
    let yuv_offset = if settings.color_model_settings.color_model.is_luma_chroma() {-0.5} else {0.};
    
    let base_color = (base_color.0,base_color.1 + yuv_offset, base_color.2 + yuv_offset, settings.scale_settings.viz_alpha);
    let raw_color = P_Color::from_tuple(base_color, settings.color_model_settings.color_model);
    let chroma = base_color.1;

    let color: P_Color = 
        raw_color.
        remap_rgb_components(
        chroma, 
        settings.perceptual_offset_settings.component_limit.0, 
        settings.perceptual_offset_settings.component_limit.1, 
        settings.perceptual_offset_settings.component_limit.2
        ).
        component_gamma_transform(
            gamma[0],
            gamma[1], 
            gamma[2],
        );

    let base_color = raw_color;
    
    let point: Vec3 = {
        let point = base_color.convert_color(settings.color_model_settings.color_space_model).from_space_to_space(settings.color_model_settings.color_space, ColorSpace::XYZ);
        let point = if settings.perceptual_offset_settings.gamma_deform {color.convert_color(settings.color_model_settings.color_space_model).from_space_to_space(settings.color_model_settings.color_space, ColorSpace::XYZ)} else {point};
        let (x,y,z, _) = point.to_tuple(); 
        let (x,y,z) = transform_coordinates((x,y,z), settings);
        Vec3 {x, y, z}
    };

    (point.into(), color.set_alpha(settings.scale_settings.viz_alpha))
}

#[derive(Component, Debug, Clone, Reflect, PartialEq, Copy)]
pub enum RotationChirality{
    Middle,
    Left,
    Right,
}

impl RotationChirality {
    fn next_counterclockwise(&self) -> RotationChirality{
        match self {
            RotationChirality::Middle => RotationChirality::Left,
            RotationChirality::Left => RotationChirality::Right,
            RotationChirality::Right => RotationChirality::Middle,
        }
    }
    fn next_clockwise(&self) -> RotationChirality{
        match self {
            RotationChirality::Middle => RotationChirality::Right,
            RotationChirality::Left => RotationChirality::Middle,
            RotationChirality::Right => RotationChirality::Left,
        }
    }
}


pub fn spawn_grid(
    mut gizmos: Gizmos,
    settings: GridSettings,
){

    let scale = settings.grid_scale;
    let divs = settings.grid_divs;
    let half_scale = scale / 2. * divs as f32;

    match settings.grid {
        GridCategory::None => {},
        GridCategory::TwoDGrids => {
            gizmos.grid(
                Isometry3d::from_translation(
                   Vec3::new(half_scale, half_scale, -0.01),
                ),
                UVec2::new(divs, divs),
                Vec2::splat(scale),
                constants::BLACK.to_bevy_color(),
            )
            .outer_edges();
            gizmos.grid(
                Isometry3d::new(
                    Vec3::new(half_scale, -0.01, half_scale),
                    Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)
                ),
                UVec2::new(divs, divs),
                Vec2::splat(scale),
                constants::BLACK.to_bevy_color(),
            )
            .outer_edges();
            gizmos.grid(
                Isometry3d::new(
                    Vec3::new(-0.01, half_scale, half_scale),
                    Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)
                ),
                UVec2::new(divs, divs),
                Vec2::splat(scale),
                constants::BLACK.to_bevy_color(),
            )
            .outer_edges();
        },
        // GridCategory::ThreeDGrid => {
        //     gizmos.grid_3d(
        //         Isometry3d::from_translation(
        //             Vec3::new(5.,5.,5.),
        //         ),
        //         UVec3::new(10, 10, 10),
        //         Vec3::splat(1.0),
        //         constants::BLACK.set_alpha(0.5).to_bevy_color(),
        //     )
        //     .outer_edges();
        // },
    }

}