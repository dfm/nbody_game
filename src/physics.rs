use bevy::core::FixedTimestep;
use bevy::prelude::*;

use crate::types::{Mass, Position, Radius, Scalar, Vector, Velocity};

const GRAV: Scalar = 100_000.0;
const SOFT: Scalar = 10.0;
const TIMESTEP: Scalar = 1.0 / 60.0;
const EPSILON: Scalar = GRAV * TIMESTEP;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub struct PhysicsStage;
pub struct PhysicsPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum PhysicsSteps {
    Gravity,
    TimeStep,
    Collision,
}

#[derive(Default)]
pub struct SimulationTime(pub Scalar, pub Scalar);

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        use PhysicsSteps::*;
        app.add_stage_after(
            CoreStage::Update,
            PhysicsStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(TIMESTEP))
                .with_system(gravity.system().label(PhysicsSteps::Gravity))
                .with_system(time_step.system().label(TimeStep).after(Gravity))
                .with_system(collisions.system().label(Collision).after(TimeStep)),
        )
        .add_system(synchronize.system());
    }
}

fn synchronize(mut bodies: Query<(&mut Transform, &Position, &Velocity)>) {
    for (mut transform, position, velocity) in bodies.iter_mut() {
        let position = position.0.as_f32();
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

/// The first half of a leapfrog integrator that updates the velocities of all bodies with
/// `Velocity` based on all bodies with `Mass`.
fn gravity(sources: Query<(&Mass, &Position)>, mut bodies: Query<(&Position, &mut Velocity)>) {
    for (position1, mut velocity) in bodies.iter_mut() {
        for (mass, position2) in sources.iter() {
            let delta = position1.0 - position2.0;
            let r2 = delta.length_squared() + SOFT;
            let scale = mass.0 * EPSILON / (r2 * r2.sqrt());
            velocity.0.x -= delta.x * scale;
            velocity.0.y -= delta.y * scale;
        }
    }
}

/// The second half of the leapfrog step: updating the coordinates of the dynamic bodies.
fn time_step(
    time: Res<Time>,
    mut simulation_time: ResMut<SimulationTime>,
    mut bodies: Query<(&mut Position, &Velocity)>,
) {
    for (mut position, velocity) in bodies.iter_mut() {
        position.0 += TIMESTEP * velocity.0;
    }
    simulation_time.0 += TIMESTEP;
    simulation_time.1 += time.delta_seconds_f64();
}

fn collisions(time: Res<Time>, bodies: Query<(&Radius, &Position, Option<&Velocity>)>) {
    let dt = time.delta().as_secs_f64();
    let zero = Velocity(Vector::ZERO);
    for (n, (r1, x1, v1)) in bodies.iter().enumerate() {
        let v1_val = v1.unwrap_or(&zero);
        for (r2, x2, v2) in bodies.iter().skip(n + 1) {
            let collides = {
                if v1.is_none() && v2.is_none() {
                    // If bodies are both static, let's assume that they never collide
                    false
                } else {
                    let mut r2 = r1.0 + r2.0;
                    r2 *= r2;
                    let v2_val = v2.unwrap_or(&zero);
                    let dx = x1.0 - x2.0;
                    check_collides(dt, r2, &dx, &v1_val.0, &v2_val.0)
                }
            };
            if collides {
                eprintln!("COLLISION! {} {}", r1.0, r2.0);
            }
        }
    }
}

#[inline]
fn distance_squared_at_time(dt: Scalar, dx: &Vector, dv: &Vector) -> Scalar {
    let dy = dx.y + dt * dv.y;
    let dx = dx.x + dt * dv.x;
    dx * dx + dy * dy
}

fn check_collides(dt: Scalar, r2: Scalar, dx: &Vector, v1: &Vector, v2: &Vector) -> bool {
    // Check current time
    if dx.length_squared() <= r2 {
        return true;
    }

    // Check minimum separation only if it is within this time step
    let dv = *v1 - *v2;
    let t0 = (dx.x * dv.x + dx.y * dv.y) / dv.length_squared();
    if t0 > 0.0 && t0 < dt {
        distance_squared_at_time(-t0, dx, &dv) <= r2
    } else {
        false
    }
}
