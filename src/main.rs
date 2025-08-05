#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::needless_range_loop)]

use bevy::{
    asset::RenderAssetUsages,
    core_pipeline::prepass::NormalPrepass,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use bevy_plugins::{camera::CameraPlugin, window::WindowManagerPlugin};
use rand::random_bool;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(WindowManagerPlugin)
        .add_plugins(VoxelPlugin)
        .run();
}

struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, voxel_setup);
    }
}

fn voxel_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let grass_texture: Handle<Image> = asset_server.load("grass.png");
    let mut chunk = Chunk::default();
    for i in 0..CHUNK_SIZE {
        for j in 0..CHUNK_SIZE {
            for k in 0..CHUNK_SIZE {
                if random_bool(0.8) {
                    chunk.voxels[i][j][k] = Voxel::Empty;
                }
            }
        }
    }

    let chunk_mesh = generate_mesh(&chunk);
    let mesh = build_mesh(&chunk_mesh);
    commands
        .spawn(Mesh3d(meshes.add(mesh)))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(grass_texture.clone()),
            perceptual_roughness: 1.,
            reflectance: 0.03,
            cull_mode: None,
            ..Default::default()
        })))
        .insert(Transform::default());

    commands
        .spawn(DirectionalLight { shadows_enabled: true, ..Default::default() })
        .insert(Transform::default().looking_at(Vec3::new(0.3, -1., 1.), Vec3::Y));
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct _FooBar;

const CHUNK_SIZE: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Voxel {
    Empty,
    Full,
}

struct Chunk {
    voxels: [[[Voxel; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    fn get(&self, x: usize, y: usize, z: usize, dx: isize, dy: isize, dz: isize) -> Voxel {
        let [nx, ny, nz] = [
            (x as isize + dx) as usize,
            (y as isize + dy) as usize,
            (z as isize + dz) as usize,
        ];
        if !(nx < CHUNK_SIZE && ny < CHUNK_SIZE && nz < CHUNK_SIZE) {
            return Voxel::Empty;
        }
        self.voxels[nz][ny][nx]
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            voxels: [[[Voxel::Full; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum FaceDirection {
    Top,
    Bot,
    Lef,
    Rig,
    Fro,
    Bac,
}

#[derive(Debug)]
struct QuadFace {
    vox_loc: [usize; 3],
    direction: FaceDirection,
}

fn generate_mesh(chunk: &Chunk) -> Vec<QuadFace> {
    let mut output = Vec::new();

    for z in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let current = chunk.voxels[z][y][x];
                if current == Voxel::Empty {
                    continue;
                }

                #[rustfmt::skip]
                let neighbors = [
                    (FaceDirection::Top, chunk.get(x, y, z, 0 , 1 , 0 )),
                    (FaceDirection::Bot, chunk.get(x, y, z, 0 , -1, 0 )),
                    (FaceDirection::Lef, chunk.get(x, y, z, -1, 0 , 0 )),
                    (FaceDirection::Rig, chunk.get(x, y, z, 1 , 0 , 0 )),
                    (FaceDirection::Fro, chunk.get(x, y, z, 0 , 0 , 1 )),
                    (FaceDirection::Bac, chunk.get(x, y, z, 0 , 0 , -1)),
                ];

                for (direction, neighbor) in neighbors {
                    if neighbor == Voxel::Full {
                        continue;
                    }

                    output.push(QuadFace { vox_loc: [x, y, z], direction });
                }
            }
        }
    }

    output
}

fn build_mesh(mesh: &[QuadFace]) -> Mesh {
    let mut pos = Vec::new();
    let mut nor = Vec::new();
    let mut uvs = Vec::new();
    let mut ind = Vec::new();

    for (face_index, face) in mesh.iter().enumerate() {
        let [x, y, z] = face.vox_loc.map(|value| value as f32);

        #[rustfmt::skip]
        let (quad_positions, quad_normal) = match face.direction {
            | FaceDirection::Top => (
                [
                    [x     , y + 1., z     ],
                    [x + 1., y + 1., z     ],
                    [x     , y + 1., z + 1.],
                    [x + 1., y + 1., z + 1.],
                ],
                [0., 1., 0.],
            ),
            | FaceDirection::Bot => (
                [
                    [x     , y, z     ],
                    [x + 1., y, z     ],
                    [x     , y, z + 1.],
                    [x + 1., y, z + 1.],
                ],
                [0., -1., 0.],
            ),
            | FaceDirection::Lef => (
                [
                    [x, y     , z     ],
                    [x, y + 1., z     ],
                    [x, y     , z + 1.],
                    [x, y + 1., z + 1.],
                ],
                [-1., 0., 0.],
            ),
            | FaceDirection::Rig => (
                [
                    [x + 1., y     , z     ],
                    [x + 1., y + 1., z     ],
                    [x + 1., y     , z + 1.],
                    [x + 1., y + 1., z + 1.],
                ],
                [1., 0., 0.,],
            ),
            | FaceDirection::Fro => (
                [
                    [x     , y     , z + 1.],
                    [x + 1., y     , z + 1.],
                    [x     , y + 1., z + 1.],
                    [x + 1., y + 1., z + 1.],
                ],
                [0., 0., 1.],
            ),
            | FaceDirection::Bac => (
                [
                    [x     , y     , z],
                    [x + 1., y     , z],
                    [x     , y + 1., z],
                    [x + 1., y + 1., z],
                ],
                [0., 0., -1.],
            ),
        };

        let offset = face_index as u16 * 4;
        pos.extend_from_slice(&quad_positions);
        nor.extend_from_slice(&[quad_normal; 4]);
        uvs.extend_from_slice(&[[0., 0.], [1., 0.], [0., 1.], [1., 1.]]);
        ind.extend_from_slice(&[offset, offset + 2, offset + 1, offset + 1, offset + 2, offset + 3]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    mesh.insert_indices(Indices::U16(ind));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, pos);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, nor);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}
