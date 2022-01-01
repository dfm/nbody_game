use bevy::ecs::schedule::ReportExecutionOrderAmbiguities;
use bevy::prelude::*;

mod types;
use types::{Mass, Radius, Velocity};

mod physics;
use physics::PhysicsPlugin;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.3, 0.3).into()),
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Velocity(Vec2::new(40.0, 0.0)))
        .insert(Radius(4.0));

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.3, 1.0, 0.3).into()),
            transform: Transform::from_xyz(0.0, 100.0, 0.0),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Mass(1.0))
        .insert(Radius(5.0))
        .insert(Velocity(Vec2::new(-40.0, 0.0)));

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.3, 0.3, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Radius(6.0))
        .insert(Mass(10.0));
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Space pool!".into(),
            width: 0.5 * 2436.0,
            height: 0.5 * 1125.0,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin)
        .insert_resource(ReportExecutionOrderAmbiguities)
        .run();
}
