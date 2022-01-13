use bevy::{math::DVec2, prelude::*};

mod physics;
use physics::{DynamicBody, Mass, Position, StaticBody, TestParticle, Velocity};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(physics::PhysicsPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(DynamicBody {
        sprite: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        },
        position: Position(DVec2::new(0.0, -100.0)),
        velocity: Velocity(DVec2::new(40.0, 0.0)),
        mass: Mass(1.0),
        ..Default::default()
    });

    commands.spawn_bundle(DynamicBody {
        sprite: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        },
        position: Position(DVec2::new(0.0, 100.0)),
        velocity: Velocity(DVec2::new(-40.0, 0.0)),
        mass: Mass(1.0),
        ..Default::default()
    });

    commands.spawn_bundle(DynamicBody {
        sprite: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        },
        position: Position(DVec2::new(100.0, 0.0)),
        velocity: Velocity(DVec2::new(00.0, 40.0)),
        mass: Mass(1.0),
        ..Default::default()
    });

    commands.spawn_bundle(StaticBody {
        sprite: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                color: Color::RED,
                ..Default::default()
            },
            ..Default::default()
        },
        position: Position(DVec2::new(0.0, 0.0)),
        mass: Mass(3.0),
        ..Default::default()
    });

    commands.spawn_bundle(TestParticle {
        sprite: SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(10.0, 10.0)),
                color: Color::GREEN,
                ..Default::default()
            },
            ..Default::default()
        },
        position: Position(DVec2::new(200.0, 0.0)),
        ..Default::default()
    });
}
