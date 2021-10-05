use bevy::prelude::*;
use bevy_console::*;
use wasm_bindgen::prelude::*;

const SPEED: f32 = 500.;

struct WindowSize {
    w: f32,
    h: f32,
}

struct Materials {
    player_material: Handle<ColorMaterial>,
}

struct Player {
    dir: (f32, f32),
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // initalize all the materials used in game
    commands.insert_resource(Materials {
        player_material: materials.add(Color::rgb(1.0, 0., 0.).into()),
    });

    // save the windows size in resource
    let window = windows.get_primary().unwrap();
    commands.insert_resource(WindowSize {
        w: window.width(),
        h: window.height(),
    });
}

fn spawn_player(mut commands: Commands, materials: Res<Materials>, win_size: Res<WindowSize>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_material.clone(),
            sprite: Sprite::new(Vec2::new(25.0, 25.0)),
            ..Default::default()
        })
        .insert(Player { dir: (0.0, 0.0) });
}

fn handle_input(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Player>) {
    if let Ok(mut player) = query.single_mut() {
        player.dir = (0., 0.);
        if keyboard_input.pressed(KeyCode::A) {
            player.dir.0 = -1.;
        }
        if keyboard_input.pressed(KeyCode::D) {
            if player.dir.0 == -1. {
                player.dir.0 = 0.;
            } else {
                player.dir.0 = 1.;
            }
        }

        if keyboard_input.pressed(KeyCode::W) {
            player.dir.1 = 1.;
        }
        if keyboard_input.pressed(KeyCode::S) {
            if player.dir.1 == -1. {
                player.dir.1 = 0.;
            } else {
                player.dir.1 = -1.;
            }
        }
    }
}

fn move_player(
    time: Res<Time>,
    mut player: Query<(&mut Transform, &Player)>
){
    if let Ok((mut transform, player)) = player.single_mut() {
        transform.translation.x += player.dir.0 * SPEED * time.delta_seconds();
        transform.translation.y += player.dir.1 * SPEED * time.delta_seconds();
    }
}

pub struct Setup;

impl Plugin for Setup {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(ConsolePlugin)
            .insert_resource(ConsoleConfiguration {
                ..Default::default()
            })
            .add_startup_system(setup.system())
            .add_startup_stage("game setup", SystemStage::single(spawn_player.system()))
            .add_system(handle_input.system().label("input"))
            .add_system(move_player.system().label("movement").after("input"));
    }
}

#[wasm_bindgen]
fn run() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin)
        .add_plugin(Setup)
        .run();
}
