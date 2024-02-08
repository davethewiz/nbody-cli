use std::thread::sleep;
use std::time::{Duration, SystemTime};

use glam::Vec2;
use nbody_cli::body::Body;
use nbody_cli::integrators::Verlet;
use nbody_cli::render::TerminalRender;
use nbody_cli::simulation::{Gravity, SimComponent, Simulation};

fn get_bodies() -> Vec<Body> {
    let mut bodies = vec![];

    for x in (-40..40).step_by(2) {
        for y in (-20..20).step_by(2) {
            bodies.push(Body::new(1.0, Vec2::ZERO, Vec2::new(x as f32, y as f32)));
        }
    }

    bodies
}

fn main() {
    let gravity = Box::new(Gravity::new(0.1, 1.0));
    let renderer = Box::new(TerminalRender::new());
    let components: Vec<Box<dyn SimComponent>> = vec![gravity, renderer];

    let integrator = Box::new(Verlet::default());

    let bodies = get_bodies();
    let mut sim = Simulation::new(bodies, components, integrator);
    
    sim.start();

    let dt: f64 = 1.0 / 10.0;
    loop {
        let frame_start = SystemTime::now();

        sim.run(dt as f32);

        let elapsed = frame_start.elapsed().unwrap();

        if elapsed.as_secs_f64() < dt {
            sleep(Duration::from_secs_f64(dt) - elapsed);
        }
    }
}
