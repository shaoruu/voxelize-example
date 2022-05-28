use voxelize::{
    pipeline::FlatlandStage,
    world::{registry::Registry, voxels::block::Block, WorldConfig},
    Server, Voxelize,
};

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

        pipeline.add_stage(FlatlandStage::new(10, dirt, stone, stone));
    }

    Voxelize::run(server);
}
