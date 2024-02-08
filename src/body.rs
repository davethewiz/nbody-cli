use glam::Vec2;

#[derive(PartialEq)]
pub struct Body {
    pub mass: f32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub force: Vec2,
}

impl Body {
    pub fn new(mass: f32, velocity: Vec2, position: Vec2) -> Body {
        Body {
            mass,
            position,
            velocity,
            acceleration: Vec2::ZERO,
            force: Vec2::ZERO,
        }
    }

    /// Adds to the current force applied on the body
    pub fn apply_force(&mut self, new_force: Vec2) {
        self.force += new_force;
    }

    /// Sets the force applied to the body to zero
    pub fn reset_force(&mut self) {
        self.force = Vec2::ZERO;
    }
}
