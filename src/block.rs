use bevy::math::UVec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Voxel {
    Empty,
    Full(BlockType),
    _Semi(BlockType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlockType {
    Grass,
    Dirt,
    Sand,
    _Wood,
    _Leaf,
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
            | Self::_Wood => BlockTexture {
                top: UVec2::new(4, 0),
                bot: UVec2::new(4, 0),
                sid: UVec2::new(4, 0),
            },
            | Self::_Leaf => BlockTexture {
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
