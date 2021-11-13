use bevy::{
    ecs::world::{World, FromWorld},
    render::texture::Texture,
    asset::{Handle, AssetServer}
};

pub struct GameResources {
    pub car: Handle<Texture>
}

impl FromWorld for GameResources {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut materials = world
            .get_resource_mut<:
        let asset_server = world
            .get_resource_mut::<AssetServer>()
            .expect("AssetServer not accessible");
        GameResources {
            car: asset_server.load("green_car.png")
        }
     }
}
