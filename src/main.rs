use winit::application::ApplicationHandler;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

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