use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{Window, WindowId},
};

use crate::{
    app::state::State,
    rt::{frame_buffer::FrameBuffer, renderer::renderer::Renderer},
    util::types::Uint,
};

#[allow(unused)]
pub struct App {
    width: Uint,
    height: Uint,
    state: Option<State>,
    frame_buffer: Arc<FrameBuffer>,
}

impl App {
    pub fn new(width: Uint, height: Uint, renderer: Arc<Renderer>, frame_buffer: Arc<FrameBuffer>) -> Self {
        tokio::spawn(async move {
            renderer.render().await;
        });

        Self {
            width,
            height,
            state: None,
            frame_buffer,
        }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::info!("Window resumed");
        let window_attrs = Window::default_attributes()
            .with_inner_size(LogicalSize {
                width: self.width as u32,
                height: self.height as u32,
            })
            .with_resizable(false);

        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
        self.state = Some(pollster::block_on(State::new(window, Arc::clone(&self.frame_buffer))).unwrap());
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: State) {
        self.state = Some(event);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => std::process::exit(0),

            WindowEvent::Resized(size) => state.resize(size.width as Uint, size.height as Uint),

            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width as Uint, size.height as Uint);
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
