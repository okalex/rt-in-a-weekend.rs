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
    app::{cli::Args, state::State},
    examples::scenes::get_scene,
    get_render_options,
    util::types::Uint,
};

const CONTROL_PANEL_WIDTH: Uint = 250;

#[allow(unused)]
pub struct App {
    args: Args,
    state: Option<State>,
}

impl App {
    pub fn new(args: &Args) -> Self {
        Self { args: *args, state: None }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log::info!("Window resumed");

        let render_options = get_render_options(&self.args);
        let (camera_options, scene) = get_scene(self.args.scene);

        let window_attrs = Window::default_attributes()
            .with_inner_size(LogicalSize {
                width: (render_options.img_width + CONTROL_PANEL_WIDTH) as u32,
                height: render_options.img_height as u32,
            })
            .with_resizable(true);

        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
        let state = pollster::block_on(State::new(window, render_options, camera_options, scene)).unwrap();

        self.state = Some(state);
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: State) {
        self.state = Some(event);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        let event_response = state.egui_state.on_window_event(&state.window, &event);
        if event_response.consumed {
            return;
        }

        match event {
            WindowEvent::CloseRequested => std::process::exit(0),

            WindowEvent::Resized(size) => state.resize(size.width as Uint, size.height as Uint),

            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
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
