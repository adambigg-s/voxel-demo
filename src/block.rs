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

impl From<Voxel> for String {
    fn from(val: Voxel) -> Self {
        match val {
            | Voxel::Empty => unreachable!("player can't select this"),
            | Voxel::Full(block) => String::from(block),
            | Voxel::_Semi(_) => todo!(),
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
}

pub const BLOCKS: [Voxel; 5] = [
    Voxel::Full(BlockType::Grass),
    Voxel::Full(BlockType::Dirt),
    Voxel::Full(BlockType::Sand),
    Voxel::Full(BlockType::Wood),
    Voxel::Full(BlockType::Leaf),
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
        }
    }
}

impl BlockType {
    pub const fn texture(&self) -> BlockTexture {
        match self {
            | Self::Grass => BlockTexture {
                top: UVec2::new(0, 0),
                bot: UVec2::new(2, 0),
                sid: UVec2::new(1, 0),
            },
            | Self::Dirt => BlockTexture {
                top: UVec2::new(2, 0),
                bot: UVec2::new(2, 0),
                sid: UVec2::new(2, 0),
            },
            | Self::Sand => BlockTexture {
                top: UVec2::new(3, 0),
                bot: UVec2::new(3, 0),
                sid: UVec2::new(3, 0),
            },
            | Self::Wood => BlockTexture {
                top: UVec2::new(4, 0),
                bot: UVec2::new(4, 0),
                sid: UVec2::new(4, 0),
            },
            | Self::Leaf => BlockTexture {
                top: UVec2::new(9, 9),
                bot: UVec2::new(9, 9),
                sid: UVec2::new(9, 9),
            },
        }
    }
}

pub struct BlockTexture {
    pub top: UVec2,
    pub bot: UVec2,
    pub sid: UVec2,
}
