
use std::time::{Duration, Instant};
use std::thread::sleep;
use sdl2;
use sdl2::render::WindowCanvas;
use sdl2::pixels::Color;
use config::{CONFIG, SCREEN_CFG};

use eventhandler::EventHandler;
use window::WindowManager;

pub struct Screen {
    canvas: WindowCanvas,
    event_handler: EventHandler,
}

impl Screen {
    pub fn new(sdl_context: &sdl2::Sdl) -> Screen {
        let video_subsystem = sdl_context.video().expect("Init Failed : SDL Video Subsystem");

        let window = video_subsystem.window(
            "Rusted Ruins", SCREEN_CFG.screen_w, SCREEN_CFG.screen_h)
            .position_centered()
            .build()
            .unwrap();

        let canvas_builder = window.into_canvas();
        let canvas_builder = if CONFIG.hardware_acceleration {
            canvas_builder.accelerated()
        }else{
            canvas_builder.software()
        };
        let canvas = canvas_builder.build().unwrap();

        Screen {
            canvas: canvas,
            event_handler: EventHandler::new(sdl_context),
        }
    }

    pub fn main_loop(&mut self, sdl_context: &::SdlContext) {
        let fps_duration = Duration::from_millis(1000 / 30);
        let mut event_pump = sdl_context.sdl_context.event_pump().unwrap();
        let mut prev_instant = Instant::now();
        let mut after_redraw_instant;
        let mut is_skip_next_frame = false;
        let texture_creator = self.canvas.texture_creator();
        let mut window_manager = WindowManager::new(sdl_context, &texture_creator);
        
        'mainloop: loop {
            self.event_handler.update_dir(&event_pump);
            for event in event_pump.poll_iter() {
                if !self.event_handler.process_event(event) {
                    break 'mainloop;
                }
            }
            
            if !window_manager.animation_now() {
                if !window_manager.advance_turn(&mut self.event_handler) { break 'mainloop }
            }

            if !is_skip_next_frame {
                self.redraw(&mut window_manager);
            }

            after_redraw_instant = Instant::now();
            if after_redraw_instant > prev_instant + fps_duration {
                // Skip next drawing
                is_skip_next_frame = true;
            }else{
                let used_time = after_redraw_instant.duration_since(prev_instant);
                sleep(fps_duration - used_time);
                is_skip_next_frame = false;
            }
            prev_instant = Instant::now();
        }
    }

    fn redraw(&mut self, window_manager: &mut WindowManager) {
        self.canvas.set_viewport(None);
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        window_manager.draw(&mut self.canvas);
        self.canvas.present();
    }
}

