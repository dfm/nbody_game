use bevy::{core::FixedTimestep, math::DVec2, prelude::*};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::Update,
            GravityStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(TIME_STEP))
                .with_system(dynamic_self_gravity)
                .with_system(dynamic_gravity)
                .with_system(test_particle_gravity)
                .with_system(time_step.exclusive_system().at_end()),
        )
        .add_system(synchronize);
    }
}

const TIME_STEP: f64 = 0.01;
const GRAV: f64 = 100_000.0;
const SOFT: f64 = 10.0;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct GravityStage;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct TimeStepStage;

#[derive(Component, Default)]
pub struct Mass(pub f64);

#[derive(Component, Default)]
pub struct Acceleration(pub DVec2);

#[derive(Component, Default)]
pub struct Position(pub DVec2);

#[derive(Component, Default)]
pub struct Velocity(pub DVec2);

#[derive(Bundle, Default)]
pub struct StaticBody {
    #[bundle]
    pub sprite: SpriteBundle,
    pub mass: Mass,
    pub position: Position,
}

#[derive(Bundle, Default)]
pub struct DynamicBody {
    #[bundle]
    pub sprite: SpriteBundle,
    pub mass: Mass,
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

#[derive(Bundle, Default)]
pub struct TestParticle {
    #[bundle]
    pub sprite: SpriteBundle,
    pub position: Position,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
}

fn unit_gravity(x1: &Position, x2: &Position) -> DVec2 {
    let r = x2.0 - x1.0;
    let r2 = r.length_squared() + SOFT;
    let f = GRAV / (r2 * r2.sqrt());
    r * f
}

fn dynamic_self_gravity(mut query: Query<(&Mass, &Position, &mut Acceleration)>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(m1, x1, mut a1), (m2, x2, mut a2)]) = iter.fetch_next() {
        let force = unit_gravity(x1, x2);
        a1.0 += force * m2.0;
        a2.0 -= force * m1.0;
    }
}

fn dynamic_gravity(
    sources: Query<(&Mass, &Position), Without<Velocity>>,
    mut particles: Query<(&Position, &mut Acceleration), With<Mass>>,
) {
    for (x1, mut a1) in particles.iter_mut() {
        for (mass, x2) in sources.iter() {
            a1.0 += unit_gravity(x1, x2) * mass.0;
        }
    }
}

fn test_particle_gravity(
    sources: Query<(&Mass, &Position)>,
    mut particles: Query<(&Position, &mut Acceleration), Without<Mass>>,
) {
    for (x1, mut a1) in particles.iter_mut() {
        for (mass, x2) in sources.iter() {
            a1.0 += unit_gravity(x1, x2) * mass.0;
        }
    }
}

fn time_step(mut query: Query<(&mut Acceleration, &mut Position, &mut Velocity)>) {
    for (mut a, mut x, mut v) in query.iter_mut() {
        v.0 += TIME_STEP * a.0;
        x.0 += TIME_STEP * v.0;
        a.0 = DVec2::ZERO;
    }
}

fn synchronize(mut query: Query<(&Position, &mut Transform)>) {
    for (x, mut transform) in query.iter_mut() {
        transform.translation.x = x.0.x as f32;
        transform.translation.y = x.0.y as f32;
    }
}
