use bevy::app::*;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::math::primitives::Circle;
use crate::dynamic_element::movement::{move_dynamic_objects, MovingDynamicObject};

mod movement;

pub struct DynamicPlugin;

impl Plugin for DynamicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, set_up_dynamic)
            .add_systems(Update, move_dynamic_objects);
    }
}

fn set_up_dynamic(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let mesh_handle = Mesh2dHandle(meshes.add(Circle::new(50f32)));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: materials.add(Color::BLUE),
            ..default()
        },
        MovingDynamicObject { velocity : Vec2::new(-100f32, 0f32) },
        ));
}