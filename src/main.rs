use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() -> Result<(), String> {
    struct WindowProperties {
        title: String,
        width: u32,
        height: u32,
    }

    impl Default for WindowProperties {
        fn default() -> Self {
            WindowProperties {
                title: "Hello World!".to_string(),
                width: 800,
                height: 600,
            }
        }
    }

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window_properties: WindowProperties = Default::default(); 
    let window = video_subsystem.window(&window_properties.title, window_properties.width, window_properties.height)
        .position_centered()
        .build()
        .expect("Could not create window.");

    let mut canvas = window.into_canvas().build()
        .expect("Could not initialize canvas.");

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    'running: loop {
        i = (i + 1) % 255;
        println!("{}", i);

        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | 
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },
                
                _ => {}
            }

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    Ok(())
}

