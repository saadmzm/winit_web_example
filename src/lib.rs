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
    main_window: Option<Window>,
    child_window: Option<Window>, // Add a child window
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create the main window if it doesn't exist
        if self.main_window.is_none() {
            let attrs = WindowAttributes::default()
                .with_title("Main Window")
                .with_resizable(true);
            let window = event_loop.create_window(attrs).expect("Failed to create main window");

            #[cfg(target_arch = "wasm32")]
            {
                use winit::platform::web::WindowExtWebSys;
                let canvas = window.canvas().expect("Canvas not found");
                let doc_body = web_sys::window()
                    .and_then(|win| win.document())
                    .and_then(|doc| doc.body())
                    .expect("Failed to get document body");
                doc_body.append_child(&canvas).expect("Failed to append main canvas");
                let style = canvas.style();
                style.set_property("width", "100%").expect("Failed to set width");
                style.set_property("height", "100%").expect("Failed to set height");
                style.set_property("position", "absolute").expect("Failed to set position");
                style.set_property("top", "10px").expect("Failed to set top");
                style.set_property("left", "10px").expect("Failed to set left");
            }
            self.main_window = Some(window);
        }

        // Create the child window if it doesn't exist
        if self.child_window.is_none() {
            let attrs = WindowAttributes::default()
                .with_title("Child Window")
                .with_resizable(true);
            let window = event_loop.create_window(attrs).expect("Failed to create child window");

            #[cfg(target_arch = "wasm32")]
            {
                use winit::platform::web::WindowExtWebSys;
                let canvas = window.canvas().expect("Canvas not found");
                let doc_body = web_sys::window()
                    .and_then(|win| win.document())
                    .and_then(|doc| doc.body())
                    .expect("Failed to get document body");
                doc_body.append_child(&canvas).expect("Failed to append child canvas");
                let style = canvas.style();
                style.set_property("width", "25%").expect("Failed to set width");
                style.set_property("height", "25%").expect("Failed to set height");
                style.set_property("position", "absolute").expect("Failed to set position");
                style.set_property("top", "60px").expect("Failed to set top"); // Offset from main
                style.set_property("left", "60px").expect("Failed to set left");
                style.set_property("border", "1px solid black").expect("Failed to set border");
            }
            self.child_window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // Handle events for both windows
        let target_window = if Some(window_id) == self.main_window.as_ref().map(|w| w.id()) {
            "Main"
        } else if Some(window_id) == self.child_window.as_ref().map(|w| w.id()) {
            "Child"
        } else {
            "Unknown"
        };

        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested for {} window; exiting...", target_window);
                self.main_window = None;
                self.child_window = None;
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                println!("Redraw requested for {} window", target_window);
            }
            WindowEvent::Resized(physical_size) => {
                println!("{} window resized to {:?}", target_window, physical_size);
                if let Some(window) = self.main_window.as_ref() {
                    if window.id() == window_id {
                        window.request_redraw();
                    }
                }
                if let Some(window) = self.child_window.as_ref() {
                    if window.id() == window_id {
                        window.request_redraw();
                    }
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

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut app = App::default();
    event_loop.run_app(&mut app).expect("Failed to run event loop");
    Ok(())
}