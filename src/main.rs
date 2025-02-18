use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{DeviceEvent, ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App<'window> {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'window>>,
    width: u32,
    height: u32,
    cursor_x: u32,
    cursor_y: u32,
}

impl<'window> ApplicationHandler for App<'window> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(
                event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_inner_size(LogicalSize::new(800, 600))
                            .with_title("Custom Cursor")
                            .with_transparent(true)
                            .with_decorations(false), // Remove borders
                    )
                    .unwrap(),
            );

            let size = window.inner_size();
            let surface_texture = SurfaceTexture::new(size.width, size.height, window.clone());
            let pixels = Pixels::new(size.width, size.height, surface_texture)
                .expect("Failed to initialize pixels");

            self.window = Some(window.clone());
            self.pixels = Some(pixels);
            self.width = size.width;
            self.height = size.height;
            self.cursor_x = size.width / 2;
            self.cursor_y = size.height / 2;
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                self.cursor_x = position.x as u32;
                self.cursor_y = position.y as u32;

                println!("Cursor moved to: ({}, {})", self.cursor_x, self.cursor_y);

                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::CursorLeft { device_id } => {
                println!("Cursor left the window. Resetting position.");
                self.cursor_x = self.width / 2;
                self.cursor_y = self.height / 2;
                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => {}
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = self.pixels.as_mut() {
                    // update pixels
                    let frame = pixels.frame_mut();
                    let width = self.width as usize;
                    let height = self.height as usize;

                    frame.fill(0);

                    let x = self.cursor_x as usize;
                    let y = self.cursor_y as usize;

                    for x_val in x..width.min(x + 100) {
                        for y_val in y..height.min(y + 100) {
                            let index = (y_val * width + x_val) * 4;
                            frame[index] = 255;
                            frame[index + 1] = 0;
                            frame[index + 2] = 0;
                            frame[index + 3] = 255;
                        }
                    }
                    // if x < width && y < height {
                    //     let index = (y * width + x) * 4;
                    //     frame[index] = 255;
                    //     frame[index + 1] = 0;
                    //     frame[index + 2] = 0;
                    //     frame[index + 3] = 255;
                    // }

                    pixels.render().expect("Render failed");
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        if let DeviceEvent::Button { button, state } = event {
            if button == 1 && state == ElementState::Pressed {
                println!("Left mouse button clicked!");
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app).unwrap();
}
