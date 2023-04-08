use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    sprite::MaterialMesh2dBundle,
};

use bevy_rapier2d::prelude::*;

pub struct MapPlugin;  

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
        // --- ADD resources ---
            //.init_resource::<ButtonMaterials>()
        // --- Spawn in stuff ---
            .add_system(map_setup.in_schedule(OnEnter(crate::AppState::Map1)))
        // --- Add systems for on update ---
            .add_system(map_run.in_set(OnUpdate(crate::AppState::Map1)))
        // --- Despawn stuff ---
            .add_system(map_despawn.in_schedule(OnExit(crate::AppState::Map1)))
            ;
    }
}

#[derive(Component)]
pub struct Map1;

fn map_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    audio: Res<Audio>,
) {
    // Floor
    commands.spawn((Map1,MaterialMesh2dBundle::<ColorMaterial> {
        mesh: meshes.add(shape::Quad::new(Vec2::new(1000.0,100.0)).into()).into(),
        material: materials.add(ColorMaterial::from(Color::GREEN)),
        transform: Transform::from_translation(Vec3::new(0.0, -300.0, 0.0)),
        ..default()
    }))
    .insert(Collider::cuboid(500.0, 50.0));

    let music = asset_server.load("music/Windless Slopes.ogg");
    audio.play(music);
}

fn despawn_map(
    mut commands: Commands,
    query: Query<Entity, With<Map1>>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
    music_controller: Local<Handle<AudioSink>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    if let Some(sink) = audio_sinks.get(&*music_controller) {
        if sink.is_paused() {
            sink.play()
        } else {
            sink.pause()
        }
    }
}

fn map_run() {
    
}