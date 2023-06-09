use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_inspector_egui::inspector_options::std_options::NumberDisplay;
use bevy_inspector_egui::{prelude::*, DefaultInspectorConfigPlugin};
use bevy_pbr::PbrBundle;
use bevy_window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

#[derive(Reflect, Default, InspectorOptions)]
#[reflect(InspectorOptions)]
struct Config {
    // `f32` uses `NumberOptions<f32>`
    #[inspector(min = 10.0, max = 70.0, display = NumberDisplay::Slider)]
    font_size: f32,
    #[inspector(min = -1.0, speed = 0.001)] // you can specify inner options for `Option<T>`
    option: Option<f32>,
    #[inspector(min = 10, max = 20)] // same for Vec<T>
    vec: Vec<u32>,
}


pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) && crate::INSPECT {
            app
            //.add_plugin(WorldInspectorPlugin::new())
            //.add_plugins(DefaultPlugins)
            .add_plugin(DefaultInspectorConfigPlugin)
            .add_plugin(bevy_egui::EguiPlugin)
            // types need to be registered
            .init_resource::<UiData>()
            .register_type::<Config>()
            .register_type::<Shape>()
            .register_type::<UiData>()
            .add_system(display_events)
            //.add_startup_system(setup)
            .add_system(ui_example);
        }
    }
}



// Enums can be have `InspectorOptions` as well.
// Note that in order to switch to another enum variant, all its fields need to have [`ReflectDefault`] type data.
#[derive(Default, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
enum Shape {
    Box {
        size: Vec2,
    },
    Icosphere {
        #[inspector(min = 1)]
        subdivisions: usize,
        #[inspector(min = 0.1)]
        radius: f32,
    },
    Capsule {
        radius: f32,
        rings: usize,
        depth: f32,
        latitudes: usize,
        longitudes: usize,
    },
    Line(Vec2, Vec2),
    #[default]
    UnitSphere,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct UiData {
    config: Config,
    shape: Shape,
    pub entity: Option<Entity>,
}

fn ui_example(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            bevy_inspector_egui::bevy_inspector::ui_for_resource::<UiData>(world, ui);
        });
    });
}

/* A system that displays the events. */
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}