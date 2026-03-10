use std::{sync::Arc, thread::JoinHandle};

use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::rt::{app::state::State, objects::hittable::Hittable, renderer::Renderer};

#[allow(unused)]
pub struct App {
    width: u32,
    height: u32,
    state: Option<State>,
    renderer: Arc<Renderer>,
    thread_handles: Vec<JoinHandle<()>>,
}

impl App {
    pub fn new(width: u32, height: u32, renderer: Arc<Renderer>, scene: Arc<dyn Hittable>) -> Self {
        let thread_handles = renderer.render(scene);
        Self {
            width,
            height,
            state: None,
            renderer,
            thread_handles,
        }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::info!("Window resumed");
        let window_attrs = Window::default_attributes()
            .with_inner_size(LogicalSize {
                width: self.width,
                height: self.height,
            })
            .with_resizable(false);

        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
        self.state = Some(
            pollster::block_on(State::new(window, Arc::clone(&self.renderer.frame_buffer)))
                .unwrap(),
        );
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: State) {
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(size) => {
                log::info!("Resized to {}x{}", size.width, size.height);
                state.resize(size.width, size.height)
            }

            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(event_loop, code, key_state.is_pressed()),

            _ => {}
        }
    }
}
