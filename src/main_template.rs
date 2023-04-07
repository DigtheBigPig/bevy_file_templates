use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


// INFO: 
//mod debug;



#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    Paused,
    Map1,
    Map2,
    Map3,
}

const INSPECT: bool = true;

fn main() {
    App::new()
        // ---- Window plugin ----
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window {
                title: String::from(
                    "Name in top bar",
                ),
                mode: bevy_window::WindowMode::Windowed,
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        // ---------- Appstate ----------
        .add_state::<AppState>()
        // ----------  Initial Setup ----------
        .add_plugin(camera::CameraPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(controller::ControllerPlugin)

        // ---- Scenes ----
        
        .add_plugin(map1::MapPlugin)
        .add_plugin(ui::UiPlugin)

        // ----------  Debug ----------
        .add_plugin(debug::DebugPlugin)
        .run();
}

fn startup_setup(
    mut windows: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
){
    let Ok(mut window) = windows.get_single_mut() else {
        return;
    };
    window.set_maximized(true);
    window.cursor.icon = bevy_window::CursorIcon::Move;
    //window.cursor.visible = false;
    //.set_cursor_grab_mode(bevy::window::CursorGrabMode::Locked);
}