use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};
use wasm_bindgen::prelude::*;

// Define your application state
#[derive(Default)]
struct App {
    window: Option<Window>,
}

// Implement the ApplicationHandler trait
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            // Create window attributes with resizable set to true
            let attrs = WindowAttributes::default()
                .with_title("Winit Web Example")
                .with_resizable(true); // Enable resizing

            // Create the window
            let window = event_loop
                .create_window(attrs)
                .expect("Failed to create window");

            // On web, append the canvas to the document body and set initial size
            #[cfg(target_arch = "wasm32")]
            {
                use winit::platform::web::WindowExtWebSys;
                let canvas = window.canvas().expect("Canvas not found");
                let result = web_sys::window()
                    .and_then(|win| win.document())
                    .and_then(|doc| doc.body())
                    .and_then(|body| {
                        body.append_child(&canvas).map(|_| ()).ok()
                    });
                if result.is_none() {
                    panic!("Failed to append canvas to document body");
                }

                // Set the canvas style to allow resizing with the window
                let style = canvas.style();
                style.set_property("width", "50%").expect("Failed to set width");
                style.set_property("height", "50%").expect("Failed to set height");
            }

            self.window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested; exiting...");
                self.window = None;
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(_window) = self.window.as_ref() {
                    println!("Redraw requested");
                    // Add rendering logic here (e.g., WebGL or wgpu)
                }
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(window) = self.window.as_ref() {
                    println!("Window resized to {:?}", physical_size);
                    // On web, the canvas size updates automatically if styled correctly,
                    // but you might need this for rendering context updates (e.g., WebGL)
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        if let StartCause::Init = cause {
            println!("Application initialized");
        }
    }
}

// WebAssembly entry point
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Set up panic hook for better error reporting
    console_error_panic_hook::set_once();

    // Create the event loop
    let event_loop = EventLoop::new().expect("Failed to create event loop");

    // Initialize the application
    let mut app = App::default();

    // Run the event loop with the application
    event_loop.run_app(&mut app).expect("Failed to run event loop");

    Ok(())
}