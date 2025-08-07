use crate::block::Voxel;
use crate::config::blocks::CHUNK_SIZE;

trait _Chunked {
    type Output;

    const X: usize;
    const Y: usize;
    const Z: usize;

    fn size() -> usize {
        Self::X * Self::Y * Self::Z
    }

    fn linearize(x: usize, y: usize, z: usize) -> usize {
        Self::X * Self::Y * z + Self::X * y + x
    }

    fn delinearize(mut index: usize) -> (usize, usize, usize) {
        let z = index / (Self::X * Self::Y);
        index -= z * Self::X * Self::Y;

        let y = index / Self::X;
        index -= y * Self::X;

        let x = index;

        (x, y, z)
    }

    fn get(x: usize, y: usize, z: usize) -> Self::Output;
}

pub struct Chunk {
    pub voxels: [[[Voxel; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Chunk {
    pub fn get(&self, x: usize, y: usize, z: usize, dx: isize, dy: isize, dz: isize) -> Voxel {
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
