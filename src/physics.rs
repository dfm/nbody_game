use bevy::core::FixedTimestep;
use bevy::prelude::*;

mod simulation;

use crate::types::{Mass, Radius, Velocity};

const GRAV: f32 = 100_000.0;
const SOFT: f32 = 10.0;
const TIMESTEP: f64 = 2.0 / 60.0;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub struct PhysicsStage;
pub struct PhysicsPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum PhysicsSteps {
    Gravity,
    TimeStep,
    Collision,
}

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
        );
    }
}

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
    println!("{} {}", dt, TIMESTEP);
    for (mut transform, velocity) in bodies.iter_mut() {
        transform.translation.x += dt * velocity.0.x;
        transform.translation.y += dt * velocity.0.y;
    }
}

fn collisions(time: Res<Time>, bodies: Query<(&Radius, &Transform, Option<&Velocity>)>) {
    let dt = time.delta().as_secs_f32();
    let zero = Velocity(Vec2::ZERO);
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
                    let dx = x1.translation - x2.translation;
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
fn distance_squared_at_time(dt: f32, dx: &Vec3, dv: &Vec2) -> f32 {
    let dy = dx.y + dt * dv.y;
    let dx = dx.x + dt * dv.x;
    dx * dx + dy * dy
}

fn check_collides(dt: f32, r2: f32, dx: &Vec3, v1: &Vec2, v2: &Vec2) -> bool {
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
