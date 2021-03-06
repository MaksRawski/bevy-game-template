use {{crate_name}}::Setup;
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(Setup)
        .run();
}
