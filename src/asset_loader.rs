use bevy::prelude::*;

// use std::{f32::consts::PI, time::Duration};

// use bevy::{animation::RepeatAnimation, pbr::CascadeShadowConfigBuilder, prelude::*};
use std::collections::HashMap;

#[derive(Resource, Debug, Default)]
pub struct SceneAsset {
    pub scene_root_handle: Handle<Scene>,
    pub animations: Option<Vec<AnimationNodeIndex>>,
    pub graph_handle: Option<Handle<AnimationGraph>>,
}


#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub loaded_assets: HashMap<String, SceneAsset>,
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}


const COW_PATH: &str = "Cow.glb";
const POOP_PATH: &str = "Poop.glb";
const FARMER_PATH: &str = "Farmer.glb";


fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>, mut graphs: ResMut<Assets<AnimationGraph>>) {
    let mut scene_assets = &mut *scene_assets; // Create a mutable reference
    load_glb_by_path(&mut scene_assets, &asset_server, &mut graphs, COW_PATH);
    load_glb_by_path(&mut scene_assets, &asset_server, &mut graphs, POOP_PATH);
    load_glb_by_path(&mut scene_assets, &asset_server, &mut graphs, FARMER_PATH);
}

fn load_glb_by_path(mut scene_assets: &mut SceneAssets, asset_server: &Res<AssetServer>, mut graphs: &mut Assets<AnimationGraph>, asset_path: &str) {
    
    let mut new_asset = SceneAsset {
        scene_root_handle: asset_server.load(GltfAssetLabel::Scene(0).from_asset(asset_path.to_string())),
        animations: None,
        graph_handle: None,
    };

    // Build the animation graph
    if let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(16).from_asset(asset_path.to_string())),
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(asset_path.to_string())),
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(asset_path.to_string())),
    ]) {
        new_asset.animations = Some(node_indices);
        new_asset.graph_handle = Some(graphs.add(graph));
    } 

    let asset_name = asset_path.strip_suffix(".glb").unwrap_or(asset_path);
    scene_assets.loaded_assets.insert(asset_name.to_string(), new_asset);
}