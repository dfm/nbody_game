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
        // .insert(Mass(10.0))
        .insert(Velocity(Vec2::new(40.0, 0.0)));

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.3, 1.0, 0.3).into()),
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Mass(1.0))
        .insert(Velocity(Vec2::new(-40.0, 0.0)));

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.3, 0.3, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Mass(10.0));
}

fn velocity_step(
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

fn position_step(time: Res<Time>, mut bodies: Query<(&mut Transform, &Velocity)>) {
    let dt = time.delta().as_secs_f32();
    for (mut transform, velocity) in bodies.iter_mut() {
        transform.translation.x += dt * velocity.0.x;
        transform.translation.y += dt * velocity.0.y;
    }
}

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_system(velocity_step.system().label("velocity").before("position"))
        .add_system(position_step.system().label("position"))
        .run();
}
