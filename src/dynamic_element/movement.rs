use bevy::prelude::*;

#[derive(Component, Default)]
pub struct MovingDynamicObject {
    pub velocity : Vec2
}

impl MovingDynamicObject {
    pub fn velocity_3d(&self) -> Vec3 {
        self.velocity.extend(0f32)
    }
}


pub fn move_dynamic_objects(
    mut query: Query<(&mut Transform, &MovingDynamicObject)>,
    time : Res<Time>,
) {
    let secs = time.delta().as_secs_f32();

    for (mut transform, move_data) in query.iter_mut()
    {
        transform.translation += move_data.velocity_3d() * secs;
    }
}