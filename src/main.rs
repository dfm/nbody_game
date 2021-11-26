use bevy::prelude::*;

const GRAV: f32 = 100_000.0;
const SOFT: f32 = 10.0;

struct Mass(f32);
struct Velocity(Vec2);

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.3, 0.3).into()),
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Mass(10.0))
        .insert(Velocity(Vec2::new(40.0, 0.0)));

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.3, 1.0, 0.3).into()),
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Mass(10.0))
        .insert(Velocity(Vec2::new(-40.0, 0.0)));

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.3, 0.3, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Mass(1.0))
        .insert(Velocity(Vec2::new(0.0, 0.0)));
}

fn integrate_step(
    dt: f32,
    masses: Vec<f32>,
    positions: &mut Vec<Vec2>,
    velocities: &mut Vec<Vec2>,
) {
    let half_dt = 0.5 * dt;

    // First leapfrog step
    for (x, &v) in positions.iter_mut().zip(velocities.iter()) {
        *x += half_dt * v;
    }

    // Compute accelerations
    let num_bodies = masses.len();
    let body_iter = masses.iter().zip(positions.iter());
    let mut accelerations = Vec::with_capacity(num_bodies);
    accelerations.resize(num_bodies, Vec2::new(0.0, 0.0));
    for (i, (m1, &x1)) in body_iter.clone().enumerate() {
        for (j, (m2, &x2)) in body_iter.clone().skip(i + 1).enumerate() {
            let delta = x1 - x2;
            let r2 = delta.length_squared() + SOFT;
            let factor = GRAV / (r2 * r2.sqrt());

            // Using unsafe here to avoid bounds checks
            unsafe {
                let acc = accelerations.get_unchecked_mut(i);
                *acc -= (factor * m2) * delta;
            }

            unsafe {
                let acc = accelerations.get_unchecked_mut(i + 1 + j);
                *acc += (factor * m1) * delta;
            }
        }
    }

    // Second leapfrog step
    for (v, &a) in velocities.iter_mut().zip(accelerations.iter()) {
        *v += dt * a;
    }
    for (x, &v) in positions.iter_mut().zip(velocities.iter()) {
        *x += half_dt * v;
    }
}

fn gravity(time: Res<Time>, mut bodies: Query<(&Mass, &mut Transform, &mut Velocity)>) {
    let dt = time.delta().as_secs_f32();
    let mut masses = vec![];
    let mut positions: Vec<Vec2> = vec![];
    let mut velocities = vec![];
    for (mass, transform, velocity) in bodies.iter_mut() {
        masses.push(mass.0);
        positions.push(transform.translation.into());
        velocities.push(velocity.0);
    }

    // Do the time integration
    integrate_step(dt, masses, &mut positions, &mut velocities);

    // Update the coordinates
    for ((&x, &v), (_, mut transform, mut velocity)) in positions
        .iter()
        .zip(velocities.iter())
        .zip(bodies.iter_mut())
    {
        transform.translation = (x, transform.translation.z).into();
        velocity.0 = v;
    }
}

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_system(gravity.system())
        .run();
}
