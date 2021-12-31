use bevy::prelude::*;

use crate::types::{Radius, Velocity};

pub struct CollisionsPlugin;

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
                    let r2 = r1.0 * r1.0 + r2.0 * r2.0;
                    let v2_val = v2.unwrap_or(&zero);
                    let dx = x1.translation - x2.translation;
                    check_collides(dt, r2, &dx, &v1_val.0, &v2_val.0)
                }
            };
            if collides {
                eprintln!("COLLISION!");
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
    if dt > 0.0 && t0 < dt {
        distance_squared_at_time(-t0, dx, &dv) <= r2
    } else {
        false
    }
}

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(collisions.system());
    }
}
