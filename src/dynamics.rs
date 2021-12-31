use bevy::prelude::*;

use crate::types::{Mass, Velocity};

const GRAV: f32 = 100_000.0;
const SOFT: f32 = 10.0;

/// A plugin for executing the dynamics.
pub struct DynamicsPlugin;

/// The first half of a leapfrog integrator that updates the velocities of all bodies with
/// `Velocity` based on all bodies with `Mass`.
fn gravity(
    time: Res<Time>,
    sources: Query<(&Mass, &Transform)>,
    mut bodies: Query<(&Transform, &mut Velocity)>,
) {
    let factor = GRAV * time.delta().as_secs_f32();
    for (transform1, mut velocity) in bodies.iter_mut() {
        for (mass, transform2) in sources.iter() {
            let delta = transform1.translation - transform2.translation;
            let r2 = delta.length_squared() + SOFT;
            let scale = mass.0 * factor / (r2 * r2.sqrt());
            velocity.0.x -= delta.x * scale;
            velocity.0.y -= delta.y * scale;
        }
    }
}

/// The second half of the leapfrog step: updating the coordinates of the dynamic bodies.
fn time_step(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity)>) {
    let dt = time.delta().as_secs_f32();
    for (mut transform, velocity) in bodies.iter_mut() {
        transform.translation.x += dt * velocity.0.x;
        transform.translation.y += dt * velocity.0.y;
    }
}

impl Plugin for DynamicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(gravity.system())
            .add_system(time_step.system());
    }
}
