mod dynamic_element;

use bevy::prelude::*;
use crate::dynamic_element::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DynamicPlugin)
        .run();
}
