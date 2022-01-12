use bevy::math::DVec2;

type Scalar = f64;
type Vector = DVec2;
const GRAV: Scalar = 100_000.0;
const SOFT: Scalar = 10.0;
pub const TIMESTEP: Scalar = 1.0 / 60.0;
const EPSILON: Scalar = GRAV * TIMESTEP;

pub struct StaticBody {
    radius: Scalar,
    mass: Scalar,
    position: Vector,
}

pub struct DynamicBody {
    radius: Scalar,
    mass: Scalar,
    position: Vector,
    velocity: Vector,
}

pub struct TestBody {
    radius: Scalar,
    position: Vector,
    velocity: Vector,
}

fn calc_gravity(x1: Vector, x2: Vector) -> Vector {
    let delta = x1 - x2;
    let r2 = delta.length_squared() + SOFT;
    delta * EPSILON / (r2 * r2.sqrt())
}

trait Mass {
    fn mass(&self) -> Scalar;
}

trait Position {
    fn position(&self) -> Vector;
}

trait Dynamic: Position {
    fn update_velocity(&mut self, delta: Vector);
    fn apply_gravity<T: Mass + Position>(&mut self, source: &T) {
        self.update_velocity(-source.mass() * calc_gravity(self.position(), source.position()));
    }
}

macro_rules! impl_mass {
    ($name:ty) => {
        impl Mass for $name {
            #[inline]
            fn mass(&self) -> Scalar {
                self.mass
            }
        }
    };
}

macro_rules! impl_position {
    ($name:ty) => {
        impl Position for $name {
            #[inline]
            fn position(&self) -> Vector {
                self.position
            }
        }
    };
}

macro_rules! impl_dynamic {
    ($name:ty) => {
        impl_position!($name);
        impl Dynamic for $name {
            #[inline]
            fn update_velocity(&mut self, delta: Vector) {
                self.velocity += delta;
            }
        }
    };
}

impl_mass!(StaticBody);
impl_mass!(DynamicBody);
impl_position!(StaticBody);
impl_dynamic!(DynamicBody);
impl_dynamic!(TestBody);

pub struct Simulation {
    pub time: Scalar,
    pub static_bodies: Vec<StaticBody>,
    pub dynamic_bodies: Vec<DynamicBody>,
    pub test_bodies: Vec<TestBody>,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            static_bodies: Vec::new(),
            dynamic_bodies: Vec::new(),
            test_bodies: Vec::new(),
        }
    }

    pub fn integrate(&mut self, time: Scalar) {
        while self.time < time {
            self.step();
        }
    }

    fn step(&mut self) {
        self.apply_gravity();
        self.advance_time();
    }

    fn apply_gravity(&mut self) {
        for body in self.test_bodies.iter_mut() {
            for source in self.static_bodies.iter() {
                body.apply_gravity(source);
            }
            for source in self.dynamic_bodies.iter() {
                body.apply_gravity(source);
            }
        }

        for body in self.dynamic_bodies.iter_mut() {
            for source in self.static_bodies.iter() {
                body.apply_gravity(source);
            }
        }

        for n in 0..self.dynamic_bodies.len() {
            if let Some((body, rest)) = self.dynamic_bodies[n..].split_first_mut() {
                for source in rest.iter_mut() {
                    let grav = calc_gravity(body.position(), source.position());
                    body.update_velocity(-source.mass() * grav);
                    source.update_velocity(body.mass() * grav);
                }
            }
        }
    }

    fn advance_time(&mut self) {
        for body in self.test_bodies.iter_mut() {
            body.position += EPSILON * body.velocity;
        }
        for body in self.dynamic_bodies.iter_mut() {
            body.position += EPSILON * body.velocity;
        }
        self.time += TIMESTEP;
    }
}
