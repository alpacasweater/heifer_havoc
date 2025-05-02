use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub farmer: Handle<Scene>,
    pub cow: Handle<Scene>,
    pub poop: Handle<Scene>
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    
    *scene_assets = SceneAssets {
        farmer: asset_server.load("Farmer.glb#Scene0"),
        cow: asset_server.load("Cow.glb#Scene0"),
        poop: asset_server.load("Poop.glb#Scene0"),
    }
}