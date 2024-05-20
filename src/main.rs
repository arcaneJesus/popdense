use std::f64::consts::PI;
use std::fmt::Debug;
use std::ops::Neg;

const TWO_PI: f64 = PI * 2.;
const HALF_PI: f64 = PI / 2.;

trait Coordinate: Sized {
    fn new(a: f64, b: f64, c: f64) -> Result<Self, String>;
    fn zero() -> Self {
        Self::new(0., 0., 0.).unwrap()
    }

    fn round(&self, fix: u32) -> Self;
}

#[inline(always)]
fn tens(fix: u32) -> f64 {
    10f64.powi(fix as i32)
}

#[inline(always)]
fn quick_fix(n: f64, tens: f64) -> f64 {
    (n * tens).round() / tens
}

#[derive(Debug, PartialEq)]
struct SphericalCoordinate {
    rho: f64,
    theta: f64,
    phi: f64
}

impl SphericalCoordinate {
    fn vertical_unit() -> Self {
        Self {
            rho: 1.,
            theta: 0.,
            phi: 0.,
        }
    }

    fn horizontal_unit() -> Self {
        Self {
            rho: 1.,
            theta: HALF_PI,
            phi: 0.,
        }
    }
}

impl Coordinate for SphericalCoordinate {
    fn new(rho: f64, theta: f64, phi: f64) -> Result<Self, String> {
        if rho < 0. {
            Err(format!("rho = {} is out of bounds", rho))
        } else if theta < 0. || theta > PI {
            Err(format!("theta = {} is out of bounds", theta))
        } else if phi < 0. || phi > TWO_PI {
            Err(format!("phi = {} is out of bounds", phi))
        } else {
            Ok(Self {
                rho,
                theta,
                phi,
            })
        }
    }

    fn round(&self, fix: u32) -> Self {
        let tens = tens(fix);
        let rho = quick_fix(self.rho, tens);
        let theta = quick_fix(self.theta, tens);
        let phi = quick_fix(self.phi, tens);
        Self {
            rho,
            theta,
            phi
        }
    }
}

impl From<CartesianCoordinate> for SphericalCoordinate {
    fn from(value: CartesianCoordinate) -> Self {
        /* Slower????
        let rho = f64::sqrt(value.x.powi(2) + value.y.powi(2) + value.z.powi(2));
        let theta = f64::acos(value.z / rho);
        let phi = f64::signum(value.y) * f64::acos(value.x / f64::sqrt(value.x.powi(2) + value.y.powi(2)));
         */
        let delta = (value.x.powi(2) + value.y.powi(2)).sqrt();
        let rho = (value.x.powi(2) + value.y.powi(2) + value.z.powi(2)).sqrt();
        let theta= if value.z > 0. {
            delta.atan2(value.z)
        } else if value.z < 0. {
            PI + delta.atan2(value.z)
        } else if value.z == 0. && delta != 0. {
            HALF_PI
        } else {
            0.
        };
        let phi = if value.x > 0. {
            value.y.atan2(value.x)
        } else if value.x < 0. {
            value.y.atan2(value.x)
        } else {
            if value.y > 0. {
                HALF_PI
            } else if value.y < 0. {
                -HALF_PI
            } else {
                0.
            }
        };
        Self {
            rho,
            theta,
            phi,
        }
    }
}

#[derive(Debug, PartialEq)]
struct CartesianCoordinate {
    x: f64,
    y: f64,
    z: f64,
}

impl Coordinate for CartesianCoordinate {
    fn new(x: f64, y:f64, z: f64) -> Result<Self, String> {
        Ok(Self {
            x,
            y,
            z,
        })
    }

    fn round(&self, fix: u32) -> Self {
        let tens = tens(fix);
        let x = quick_fix(self.x, tens);
        let y = quick_fix(self.y, tens);
        let z = quick_fix(self.z, tens);
        Self {
            x,
            y,
            z,
        }
    }
}

impl From<SphericalCoordinate> for CartesianCoordinate {
    fn from(value: SphericalCoordinate) -> Self {
        let x = value.rho * f64::sin(value.theta) * f64::cos(value.phi);
        let y = value.rho * f64::sin(value.theta) * f64::sin(value.phi);
        let z = value.rho * f64::cos(value.theta);
        Self {
            x,
            y,
            z,
        }
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_cartesian_coordinate() {
        let coord = CartesianCoordinate::new(1., 1., 1.,).unwrap();
        let zero = CartesianCoordinate::zero();
        assert_eq!(coord, CartesianCoordinate {
            x: 1.,
            y: 1.,
            z: 1.,
        });
        assert_eq!(zero, CartesianCoordinate {
            x: 0.,
            y: 0.,
            z: 0.,
        });
    }

    #[test]
    fn create_spherical_coordinate() {
        let coord = SphericalCoordinate::new(1., HALF_PI, PI).unwrap();
        let zero = SphericalCoordinate::zero();
        let error = SphericalCoordinate::new(1., TWO_PI, PI * 3.).unwrap_err();
        assert_eq!(coord, SphericalCoordinate {
            rho: 1.,
            theta: HALF_PI,
            phi: PI
        });
        assert_eq!(zero, SphericalCoordinate {
            rho: 0.,
            theta: 0.,
            phi: 0.,
        });
        assert_eq!(error, format!("theta = {} is out of bounds", TWO_PI));
    }

    #[test]
    fn spherical_to_cartesian() {
        let coord = SphericalCoordinate::new(1., HALF_PI, PI).unwrap();
        let conversion = CartesianCoordinate::from(coord).round(1);
        assert_eq!(conversion, CartesianCoordinate::new(-1., 0., 0.).unwrap().round(1))
    }

    #[test]
    fn cartesian_to_spherical() {
        let coord = CartesianCoordinate::new(-1., 0., 0.).unwrap();
        let conversion = SphericalCoordinate::from(coord).round(1);
        assert_eq!(conversion, SphericalCoordinate::new(1., HALF_PI, PI).unwrap().round(1));
    }
}

/*
// Redering is hard so math time first

use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

// I stole all this from the winit docs and have no clue what it does
// https://docs.rs/winit/latest/winit/#event-handling
// Cite your sources kids
#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes().with_title("Population center simulator.")).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Sporadically updated elements draw here
                // Constantly updated elements draw in AboutToWait event
                self.window.as_ref().unwrap().request_redraw();
            },
            _ => (),
        }
    }
}

fn main() -> Result<(), EventLoopError> {
    // Create event loop
    let event_loop = EventLoop::new().unwrap();
    // ControlFlow::Wait for update signals from OS
    // May need to change to ControlFlow::Poll for openGL
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::default();
    event_loop.run_app(&mut app)
}

*/