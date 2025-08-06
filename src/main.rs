use bevy::{
    asset::RenderAssetUsages,
    image::{ImageFilterMode, ImageSamplerDescriptor},
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use bevy_plugins::{camera::CameraPlugin, window::WindowManagerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor {
                mag_filter: ImageFilterMode::Nearest,
                min_filter: ImageFilterMode::Nearest,
                ..Default::default()
            },
        }))
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
    let grass_texture: Handle<Image> = asset_server.load("atlas.png");
    let mut chunk = Chunk::default();
    for i in 0..CHUNK_SIZE {
        for k in 0..CHUNK_SIZE {
            let height = CHUNK_SIZE as f32 / 2.
                + ((i + k) as f32 / (CHUNK_SIZE + CHUNK_SIZE) as f32) * CHUNK_SIZE as f32 / 2.;
            for j in 0..height as usize {
                chunk.voxels[i][j][k] = Voxel::Full;
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

    let chunk2 = Chunk {
        voxels: [[[Voxel::Full; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    };
    let chunk2_mesh = generate_mesh(&chunk2);
    let mesh2 = build_mesh(&chunk2_mesh);
    commands
        .spawn(Mesh3d(meshes.add(mesh2)))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(grass_texture.clone()),
            perceptual_roughness: 1.,
            reflectance: 0.03,
            cull_mode: None,
            ..Default::default()
        })))
        .insert(Transform::from_xyz(CHUNK_SIZE as f32, 0., 0.));
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct _FooBar;

const CHUNK_SIZE: usize = 16;
const VOXEL_SIZE: f32 = 1.;
const ATLAS_SIZE: usize = 9;

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
            voxels: [[[Voxel::Empty; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum VoxelFace {
    Top,
    Bot,
    Rig,
    Lef,
    Fro,
    Bac,
}

impl VoxelFace {
    #[rustfmt::skip]
    fn normal(&self) -> Vec3 {
        match self {
            | Self::Top =>  Vec3::Y,
            | Self::Bot => -Vec3::Y,
            | Self::Rig =>  Vec3::X,
            | Self::Lef => -Vec3::X,
            | Self::Fro =>  Vec3::Z,
            | Self::Bac => -Vec3::Z,
        }
    }
}

#[derive(Debug)]
struct Quad {
    vox_loc: [usize; 3],
    face: VoxelFace,
    _block: Voxel,
}

impl Quad {
    fn indices(&self, start: u16) -> [u16; 6] {
        [start, start + 2, start + 1, start + 1, start + 2, start + 3]
    }

    fn positions(&self, voxel_size: f32) -> [Vec3; 4] {
        let [x, y, z] = self.vox_loc.map(|value| value as f32);
        let positions = match self.face {
            | VoxelFace::Top => [[0, 1, 0], [1, 1, 0], [0, 1, 1], [1, 1, 1]],
            | VoxelFace::Bot => [[0, 0, 0], [1, 0, 0], [0, 0, 1], [1, 0, 1]],
            | VoxelFace::Rig => [[1, 0, 0], [1, 1, 0], [1, 0, 1], [1, 1, 1]],
            | VoxelFace::Lef => [[0, 0, 0], [0, 1, 0], [0, 0, 1], [0, 1, 1]],
            | VoxelFace::Fro => [[0, 0, 1], [0, 1, 1], [1, 0, 1], [1, 1, 1]],
            | VoxelFace::Bac => [[0, 0, 0], [0, 1, 0], [1, 0, 0], [1, 1, 0]],
        };
        [
            Vec3::new(
                (x + positions[0][0] as f32) * voxel_size,
                (y + positions[0][1] as f32) * voxel_size,
                (z + positions[0][2] as f32) * voxel_size,
            ),
            Vec3::new(
                (x + positions[1][0] as f32) * voxel_size,
                (y + positions[1][1] as f32) * voxel_size,
                (z + positions[1][2] as f32) * voxel_size,
            ),
            Vec3::new(
                (x + positions[2][0] as f32) * voxel_size,
                (y + positions[2][1] as f32) * voxel_size,
                (z + positions[2][2] as f32) * voxel_size,
            ),
            Vec3::new(
                (x + positions[3][0] as f32) * voxel_size,
                (y + positions[3][1] as f32) * voxel_size,
                (z + positions[3][2] as f32) * voxel_size,
            ),
        ]
    }

    fn normals(&self) -> [Vec3; 4] {
        [self.face.normal(); 4]
    }
}

fn generate_mesh(chunk: &Chunk) -> Vec<Quad> {
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
                    (VoxelFace::Top, chunk.get(x, y, z, 0 , 1 , 0 )),
                    (VoxelFace::Bot, chunk.get(x, y, z, 0 , -1, 0 )),
                    (VoxelFace::Rig, chunk.get(x, y, z, 1 , 0 , 0 )),
                    (VoxelFace::Lef, chunk.get(x, y, z, -1, 0 , 0 )),
                    (VoxelFace::Fro, chunk.get(x, y, z, 0 , 0 , 1 )),
                    (VoxelFace::Bac, chunk.get(x, y, z, 0 , 0 , -1)),
                ];

                for (direction, neighbor) in neighbors {
                    if neighbor == Voxel::Full {
                        continue;
                    }

                    output.push(Quad { vox_loc: [x, y, z], face: direction, _block: current });
                }
            }
        }
    }

    output
}

fn build_mesh(mesh: &[Quad]) -> Mesh {
    let mut pos = Vec::new();
    let mut nor = Vec::new();
    let mut uvs = Vec::new();
    let mut ind = Vec::new();

    for (face_index, face) in mesh.iter().enumerate() {
        let step = 1. / ATLAS_SIZE as f32;
        let uv_step = match face.face {
            | VoxelFace::Top => {
                #[rustfmt::skip]
                let out = [
                    [0.       , 0. + step       ,],
                    [0. + step, 0. + step       ,],
                    [0.       , 0. + step + step,],
                    [0. + step, 0. + step + step,],
                ];
                out
            }
            | VoxelFace::Bot => {
                #[rustfmt::skip]
                let out = [
                    [0. + step       , 0.       ,],
                    [0. + step + step, 0.       ,],
                    [0. + step       , 0. + step,],
                    [0. + step + step, 0. + step,],
                ];
                out
            }
            | _ => {
                #[rustfmt::skip]
                let out = [
                    [0. + step, 0. + step,],
                    [0. + step, 0.       ,],
                    [0.       , 0. + step,],
                    [0.       , 0.       ,],
                ];
                out
            }
        };

        let offset = face_index as u16 * 4;
        pos.extend_from_slice(&face.positions(VOXEL_SIZE));
        nor.extend_from_slice(&face.normals());
        uvs.extend_from_slice(&uv_step);
        ind.extend_from_slice(&face.indices(offset));
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    mesh.insert_indices(Indices::U16(ind));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, pos);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, nor);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}
