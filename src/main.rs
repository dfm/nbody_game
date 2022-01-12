use bevy::{core::FixedTimestep, prelude::*};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_stage_after(
            CoreStage::Update,
            PhysicsStage,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(PHYSICS_TIME_STEP))
                .with_system(gravity)
                .with_system(integrate),
        )
        .run();
}

const PHYSICS_TIME_STEP: f64 = 0.01;
const GRAV: f32 = 100_000.0;
const SOFT: f32 = 10.0;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct PhysicsStage;

#[derive(Default)]
struct SimulationTime(f64, f64);

#[derive(Component, Default)]
struct Mass(f32);

#[derive(Component, Default)]
struct Acceleration(Vec2);

#[derive(Component, Default)]
struct Position(Vec2);

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Bundle, Default)]
struct BodyBundle {
    #[bundle]
    sprite: SpriteBundle,
    mass: Mass,
    position: Position,
    velocity: Velocity,
    acceleration: Acceleration,
}

fn unit_gravity(x1: &Position, x2: &Position) -> Vec2 {
    let r = x2.0 - x1.0;
    let r2 = r.length_squared() + SOFT;
    let f = GRAV / (r2 * r2.sqrt());
    r * f
}

fn gravity(mut query: Query<(&Mass, &Position, &mut Acceleration)>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(m1, x1, mut a1), (m2, x2, mut a2)]) = iter.fetch_next() {
        let force = unit_gravity(x1, x2);
        a1.0 += force * m2.0;
        a2.0 -= force * m1.0;
    }
}

fn integrate(
    mut query: Query<(
        &mut Acceleration,
        &mut Position,
        &mut Velocity,
        &mut Transform,
    )>,
) {
    let dt = PHYSICS_TIME_STEP as f32;
    for (mut a, mut x, mut v, mut transform) in query.iter_mut() {
        v.0 += dt * a.0;
        x.0 += dt * v.0;
        a.0 = Vec2::ZERO;
        info!("{}", x.0);
        transform.translation.x = x.0.x;
        transform.translation.y = x.0.y;
    }
}

// mod types;
// use types::{Mass, Position, Radius, Vector, Velocity};

// mod physics;
// use physics::{PhysicsPlugin, SimulationTime};

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(BodyBundle {
        sprite: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        },
        position: Position(Vec2::new(100.0, 0.0)),
        velocity: Velocity(Vec2::new(0.0, 0.0)),
        mass: Mass(10.0),
        ..Default::default()
    });

    commands.spawn_bundle(BodyBundle {
        sprite: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        },
        position: Position(Vec2::new(-100.0, 0.0)),
        velocity: Velocity(Vec2::new(0.0, -50.0)),
        // mass: Mass(1.0),
        ..Default::default()
    });

    // commands
    //     .spawn_bundle(SpriteBundle {
    //         material: materials.add(Color::rgb(1.0, 0.3, 0.3).into()),
    //         transform: Transform::from_xyz(0.0, -100.0, 0.0),
    //         sprite: Sprite::new(Vec2::new(10.0, 10.0)),
    //         ..Default::default()
    //     })
    //     .insert(Mass(10.0))
    //     .insert(Velocity(Vector::new(40.0, 0.0)))
    //     .insert(Position(Vector::new(0.0, -100.0)));

    // commands
    //     .spawn_bundle(SpriteBundle {
    //         material: materials.add(Color::rgb(0.3, 1.0, 0.3).into()),
    //         transform: Transform::from_xyz(0.0, 100.0, 0.0),
    //         sprite: Sprite::new(Vec2::new(10.0, 10.0)),
    //         ..Default::default()
    //     })
    //     .insert(Mass(1.0))
    //     .insert(Radius(5.0))
    //     .insert(Position(Vector::new(0.0, 100.0)))
    //     .insert(Velocity(Vector::new(-40.0, 0.0)));

    // commands
    //     .spawn_bundle(SpriteBundle {
    //         material: materials.add(Color::rgb(0.3, 0.3, 1.0).into()),
    //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //         sprite: Sprite::new(Vec2::new(10.0, 10.0)),
    //         ..Default::default()
    //     })
    //     .insert(Mass(10.0))
    //     .insert(Radius(6.0))
    //     .insert(Position(Vector::new(0.0, 0.0)));
}

// fn main() {
//     App::build()
//         .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
//         .insert_resource(WindowDescriptor {
//             title: "Space pool!".into(),
//             width: 0.5 * 2436.0,
//             height: 0.5 * 1125.0,
//             resizable: false,
//             ..Default::default()
//         })
//         .init_resource::<SimulationTime>()
//         .add_plugins(DefaultPlugins)
//         .add_plugin(PhysicsPlugin)
//         .add_startup_system(setup.system())
//         .run();
// }
