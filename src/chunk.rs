use bevy::prelude::*;

use crate::constants::*;

#[derive(Clone)]
pub struct Chunk {
    pub data: [Sd8; UNPADDED_CHUNK_SIZE],
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            data: [DEFAULT_SDF_VALUE; UNPADDED_CHUNK_SIZE],
        }
    }
}

#[derive(Component, Reflect, Debug)]
pub struct ChunkCoord(pub IVec3);

#[derive(Component)]
pub struct NeedGenerating;

#[derive(Component)]
pub struct NeedMeshing;
