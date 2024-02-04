use super::body::Body;

pub trait Integrator {
    fn integrate(&self, body: &mut Body, dt: f32);
}


pub struct Verlet {}

impl Integrator for Verlet {
    fn integrate(&self, body: &mut Body, dt: f32) {
        let half_vel = body.velocity + 0.5 * body.acceleration * dt;
        body.position += half_vel * dt;

        body.acceleration = body.force / body.mass;
        body.velocity = half_vel + 0.5 * body.acceleration * dt;

        body.reset_force();
    }
}

pub struct Euler {}

impl Integrator for Euler {
    fn integrate(&self, body: &mut Body, dt: f32) {
        body.acceleration = body.force / body.mass;
        body.velocity += body.acceleration * dt;
        body.position += body.velocity * dt;

        body.reset_force();
    }
}
