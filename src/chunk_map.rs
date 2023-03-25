use bevy::{prelude::*, utils::HashSet};

use crate::chunk::Chunk;

#[derive(Resource, Default)]
pub struct ChunkMap {
    chunks: bevy::utils::HashMap<IVec3, Chunk>,
}

impl ChunkMap {
    pub fn insert_chunk(&mut self, coord: IVec3, chunk: Chunk) -> Option<Chunk> {
        self.chunks.insert(coord, chunk)
    }

    pub fn get_chunk(&self, coord: &IVec3) -> Option<&Chunk> {
        self.chunks.get(coord)
    }
}

pub struct DirtyChunks(HashSet<IVec3>);

impl DirtyChunks {
    pub fn insert(&mut self, key: IVec3) -> bool {
        self.0.insert(key)
    }

    pub fn iter(&mut self) -> impl Iterator<Item = &IVec3> {
        self.0.iter()
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }
}