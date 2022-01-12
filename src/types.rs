use bevy::math::DVec2;

pub type Scalar = f64;
pub type Vector = DVec2;

/// Bodies with `Mass` will be treated as gravitational sources.
pub struct Mass(pub Scalar);

/// Bodies with `Radius` will be checked for collisions.
pub struct Radius(pub Scalar);

/// `Position` tracks the "true" position of the body.
pub struct Position(pub Vector);

/// Bodies with `Velocity` will be affected by forces.
pub struct Velocity(pub Vector);
