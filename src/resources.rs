use bevy::{
    ecs::world::{World, FromWorld},
    render::texture::Texture,
    asset::Handle, //AssetServer}
};

pub struct GameResources {
    pub car: Option<Handle<Texture>>
}

impl FromWorld for GameResources {
    fn from_world(world: &mut World) -> Self {
        let _world = world.cell();
        //let mut materials = world
        //    .get_resource_mut<:
        //let asset_server = world
        //    .get_resource_mut::<AssetServer>()
        //    .expect("AssetServer not accessible");
        GameResources {
            car: None
        }
     }
}
