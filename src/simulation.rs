use super::body::Body;
use super::integrators::Integrator;

pub struct Simulation {
    pub bodies: Vec<Body>,
    pub components: Vec<Box<dyn SimComponent>>,
    pub integrator: dyn Integrator,
}

impl Simulation {
    pub fn run(&mut self, dt: f32) {
        for component in self.components.iter_mut() {
            component.as_mut().update(&mut self.bodies);
        }

        for body in self.bodies.iter_mut() {
            self.integrator.integrate(body, dt);
        }
    }
}

pub trait SimComponent {
    fn update(&mut self, bodies: &mut Vec<Body>);
}

pub struct Gravity {
    pub gravity_constant: f32,
    pub min_distance: f32
}

impl Gravity {
    fn new(gravity_constant: f32, min_distance: f32) -> Gravity {
        Gravity {
            gravity_constant,
            min_distance
        }
    }
}

impl SimComponent for Gravity {
    fn update(&mut self, bodies: &mut Vec<Body>) {
        let n = bodies.len();
    
        for i in 0..n {
            for j in 0..n {
                if i == j { continue; }
    
                let r = bodies[j].position - bodies[i].position;
                
                let r_sq = r.length_squared();
                let m1 = bodies[i].mass;
                let m2 = bodies[j].mass;
    
                let fg = (self.gravity_constant * m1 * m2 / r_sq) * r.normalize();
    
                if r.length() >= self.min_distance {
                    bodies[i].apply_force(fg);
                    bodies[j].apply_force(-fg);
                }
            }
        }
    }
}