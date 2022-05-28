use voxelize::{
    chunk::Chunk,
    pipeline::{ChunkStage, ResourceResults},
    vec::Vec3,
    world::{
        registry::Registry,
        voxels::{access::VoxelAccess, block::Block, space::Space},
        WorldConfig,
    },
    Server, Voxelize,
};

pub struct FlatlandStage {
    /// The height of the flat land.
    height: i32,

    /// Block type of the top of the flat land.
    top: u32,

    /// Block type of the middle of the flat land.
    middle: u32,

    /// Block type of the bottom of the flat land.
    bottom: u32,
}

impl FlatlandStage {
    pub fn new(height: i32, top: u32, middle: u32, bottom: u32) -> Self {
        Self {
            height,
            top,
            middle,
            bottom,
        }
    }
}

impl ChunkStage for FlatlandStage {
    fn name(&self) -> String {
        "Flatland".to_owned()
    }

    fn process(&self, mut chunk: Chunk, _: ResourceResults, _: Option<Space>) -> Chunk {
        let Vec3(min_x, _, min_z) = chunk.min;
        let Vec3(max_x, _, max_z) = chunk.max;

        for vx in min_x..max_x {
            for vz in min_z..max_z {
                for vy in 0..self.height {
                    if vy == 0 {
                        chunk.set_voxel(vx, vy, vz, self.bottom);
                    } else if vy == self.height - 1 {
                        chunk.set_voxel(vx, vy, vz, self.top);
                    } else {
                        chunk.set_voxel(vx, vy, vz, self.middle);
                    }
                }
            }
        }

        chunk
    }
}

fn main() {
    let mut registry = Registry::new();

    let dirt = Block::new("Dirt").build();
    let stone = Block::new("Stone").build();

    registry.register_blocks(&[dirt, stone]);

    let mut server = Server::new().port(4000).registry(&registry).build();

    let config = WorldConfig::new()
        .min_chunk([-1, -1])
        .max_chunk([1, 1])
        .build();
    let world = server.create_world("Test", &config).unwrap();

    {
        let registry = world.registry();

        let dirt = registry.get_block_by_name("Dirt").id;
        let stone = registry.get_block_by_name("Stone").id;

        drop(registry);

        let mut pipeline = world.pipeline_mut();

        // Add chunk stages here...
        pipeline.add_stage(FlatlandStage::new(0, dirt, stone, stone));
    }

    Voxelize::run(server);
}
