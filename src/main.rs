use bevy::prelude::*;


// INFO: add file connections here. 
//mod debug;



#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
    Paused,
    Scene1,
    Scene2,
    Scene3,
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
        //.add_plugin(camera::CameraPlugin)
        //.add_plugin(player::PlayerPlugin)
        //.add_plugin(controller::ControllerPlugin)
        // ---- Main menu ----

        //.add_plugin(ui::UiPlugin)
        // ---- Scenes ----
        
        //.add_plugin(map1::MapPlugin)
        

        // ----------  Debug ----------
        //.add_plugin(debug::DebugPlugin)
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