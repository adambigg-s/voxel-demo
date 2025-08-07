use bevy::asset::RenderAssetUsages;
use bevy::math::Vec2;
use bevy::math::Vec3;
use bevy::render::mesh::Indices;
use bevy::render::mesh::Mesh;
use bevy::render::mesh::PrimitiveTopology;

use crate::block::Voxel;
use crate::chunk::Chunk;
use crate::config::aesthetics::ATLAS_SIZE;
use crate::config::aesthetics::TEXTURE_SIZE;
use crate::config::blocks::CHUNK_SIZE;
use crate::config::blocks::VOXEL_SIZE;

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
    const fn normal(&self) -> Vec3 {
        match self {
            | Self::Top => Vec3::new(0. , 1. , 0. ),
            | Self::Bot => Vec3::new(0. , -1., 0. ),
            | Self::Rig => Vec3::new(1. , 0. , 0. ),
            | Self::Lef => Vec3::new(-1., 0. , 0. ),
            | Self::Fro => Vec3::new(0. , 0. , 1. ),
            | Self::Bac => Vec3::new(0. , 0. , -1.),
        }
    }
}

#[derive(Debug)]
pub struct Quad {
    vox_loc: [usize; 3],
    face: VoxelFace,
    block: Voxel,
}

impl Quad {
    const fn indices(&self, start: u32) -> [u32; 6] {
        [start, start + 2, start + 1, start + 1, start + 2, start + 3]
    }

    fn texture_uvs(&self) -> [Vec2; 4] {
        const STEP: f32 = TEXTURE_SIZE as f32 / ATLAS_SIZE as f32;
        const TEX_EPS: f32 = 1. / TEXTURE_SIZE as f32;

        let Voxel::Full(block) = self.block
        else {
            unreachable!();
        };
        let modifier = block.texture();

        match self.face {
            | VoxelFace::Top => [
                Vec2::new(0. + TEX_EPS, 0. + TEX_EPS) * STEP + modifier.top.as_vec2() * STEP,
                Vec2::new(1. - TEX_EPS, 0. + TEX_EPS) * STEP + modifier.top.as_vec2() * STEP,
                Vec2::new(0. + TEX_EPS, 1. - TEX_EPS) * STEP + modifier.top.as_vec2() * STEP,
                Vec2::new(1. - TEX_EPS, 1. - TEX_EPS) * STEP + modifier.top.as_vec2() * STEP,
            ],
            | VoxelFace::Bot => [
                Vec2::new(0. + TEX_EPS, 0. + TEX_EPS) * STEP + modifier.bot.as_vec2() * STEP,
                Vec2::new(1. - TEX_EPS, 0. + TEX_EPS) * STEP + modifier.bot.as_vec2() * STEP,
                Vec2::new(0. + TEX_EPS, 1. - TEX_EPS) * STEP + modifier.bot.as_vec2() * STEP,
                Vec2::new(1. - TEX_EPS, 1. - TEX_EPS) * STEP + modifier.bot.as_vec2() * STEP,
            ],
            | _ => [
                Vec2::new(1. - TEX_EPS, 1. - TEX_EPS) * STEP + modifier.sid.as_vec2() * STEP,
                Vec2::new(1. - TEX_EPS, 0. + TEX_EPS) * STEP + modifier.sid.as_vec2() * STEP,
                Vec2::new(0. + TEX_EPS, 1. - TEX_EPS) * STEP + modifier.sid.as_vec2() * STEP,
                Vec2::new(0. + TEX_EPS, 0. + TEX_EPS) * STEP + modifier.sid.as_vec2() * STEP,
            ],
        }
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

pub fn generate_mesh(chunk: &Chunk) -> Vec<Quad> {
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
                    if let Voxel::Full(_) = neighbor {
                        continue;
                    }

                    output.push(Quad { vox_loc: [x, y, z], face: direction, block: current });
                }
            }
        }
    }

    output
}

pub fn build_mesh(mesh: &[Quad]) -> Mesh {
    let mut pos = Vec::new();
    let mut nor = Vec::new();
    let mut uvs = Vec::new();
    let mut ind = Vec::new();

    for face in mesh.iter() {
        let offset = pos.len() as u32;
        ind.extend_from_slice(&face.indices(offset));
        pos.extend_from_slice(&face.positions(VOXEL_SIZE));
        nor.extend_from_slice(&face.normals());
        uvs.extend_from_slice(&face.texture_uvs());
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    mesh.insert_indices(Indices::U32(ind));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, pos);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, nor);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    mesh
}
