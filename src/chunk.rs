use crate::block::Voxel;
use crate::config::CHUNK_SIZE;

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
