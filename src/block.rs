use bevy::math::UVec2;

trait _Block
where
    Self: PartialEq + Eq,
{
    type Output;

    fn parameters() -> Self::Output;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Voxel {
    Empty,
    Full(BlockType),
    _Semi(BlockType),
}

impl Default for Voxel {
    fn default() -> Self {
        Self::Full(BlockType::default())
    }
}

impl From<Voxel> for String {
    fn from(val: Voxel) -> Self {
        match val {
            | Voxel::Full(block) => String::from(block),
            | _ => todo!("these blocks don't even exist yet"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlockType {
    Grass,
    Dirt,
    Sand,
    Wood,
    Leaf,
    Stone,
    Plank,
    Coal,
    Water,
}

impl Default for BlockType {
    fn default() -> Self {
        Self::Grass
    }
}

pub const BLOCKS: [Voxel; 9] = [
    Voxel::Full(BlockType::Grass),
    Voxel::Full(BlockType::Dirt),
    Voxel::Full(BlockType::Sand),
    Voxel::Full(BlockType::Wood),
    Voxel::Full(BlockType::Leaf),
    Voxel::Full(BlockType::Stone),
    Voxel::Full(BlockType::Plank),
    Voxel::Full(BlockType::Coal),
    Voxel::Full(BlockType::Water),
];

pub fn get_block(index: usize) -> Voxel {
    BLOCKS[index % BLOCKS.len()]
}

impl From<BlockType> for String {
    fn from(value: BlockType) -> Self {
        match value {
            | BlockType::Grass => String::from("grass"),
            | BlockType::Dirt => String::from("dirt"),
            | BlockType::Sand => String::from("sand"),
            | BlockType::Wood => String::from("wood"),
            | BlockType::Leaf => String::from("leaf"),
            | BlockType::Stone => String::from("stone"),
            | BlockType::Plank => String::from("plank"),
            | BlockType::Coal => String::from("Coal"),
            | BlockType::Water => String::from("water"),
        }
    }
}

impl BlockType {
    pub const fn texture(&self) -> BlockTexture {
        match self {
            | Self::Grass => BlockTexture {
                top: UVec2::new(4, 4),
                bot: UVec2::new(1, 1),
                sid: UVec2::new(4, 1),
            },
            | Self::Dirt => BlockTexture {
                top: UVec2::new(1, 1),
                bot: UVec2::new(1, 1),
                sid: UVec2::new(1, 1),
            },
            | Self::Sand => BlockTexture {
                top: UVec2::new(10, 1),
                bot: UVec2::new(10, 1),
                sid: UVec2::new(10, 1),
            },
            | Self::Wood => BlockTexture {
                top: UVec2::new(2, 7),
                bot: UVec2::new(2, 7),
                sid: UVec2::new(2, 4),
            },
            | Self::Leaf => BlockTexture {
                top: UVec2::new(7, 4),
                bot: UVec2::new(7, 4),
                sid: UVec2::new(7, 4),
            },
            | Self::Stone => BlockTexture {
                top: UVec2::new(13, 1),
                bot: UVec2::new(13, 1),
                sid: UVec2::new(13, 1),
            },
            | Self::Plank => BlockTexture {
                top: UVec2::new(10, 4),
                bot: UVec2::new(10, 4),
                sid: UVec2::new(10, 4),
            },
            | Self::Coal => BlockTexture {
                top: UVec2::new(13, 4),
                bot: UVec2::new(13, 4),
                sid: UVec2::new(13, 4),
            },
            | Self::Water => BlockTexture {
                top: UVec2::new(7, 1),
                bot: UVec2::new(7, 1),
                sid: UVec2::new(7, 1),
            },
        }
    }
}

pub struct BlockTexture {
    pub top: UVec2,
    pub bot: UVec2,
    pub sid: UVec2,
}
