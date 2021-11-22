use bevy::{
    ecs::world::{World, FromWorld},
    asset::{Handle, AssetServer, Assets},
    sprite::ColorMaterial,
};

pub struct GameResources {
    pub car: Handle<ColorMaterial>,
    pub car_scale: f32,
}

impl FromWorld for GameResources {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("No color material");
        let asset_server = world
            .get_resource_mut::<AssetServer>()
            .expect("AssetServer not accessible");
        let car = asset_server.load("green_car.png");
        GameResources {
            car: materials.add(car.into()),
            car_scale: 0.6
        }
     }
}
