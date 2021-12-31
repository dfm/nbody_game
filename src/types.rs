use bevy::prelude::Vec2;

/// Bodies with `Mass` will be treated as gravitational sources.
pub struct Mass(pub f32);

/// Bodies with `Radius` will be checked for collisions.
pub struct Radius(pub f32);

/// Bodies with `Velocity` will be affected by forces.
pub struct Velocity(pub Vec2);
