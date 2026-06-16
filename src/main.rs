use env_logger;
use log::info;
use sdl3::pixels::PixelFormat;
// use log::Debug;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::time::Duration;
fn main() {
    env_logger::Builder::from_default_env().filter_level(log::LevelFilter::Debug).init();

    info!("Starting RustGBA Emulation v{}", env!("CARGO_PKG_VERSION"));

    // Initialize SDL3
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Create window for GBA screen (240x160) scaled up
    const GBA_WIDTH: u32 = 240;
    const GBA_HEIGHT: u32 = 160;
    const SCALE: u32 = 3;

    let window = video_subsystem
        .window("RustGBA", GBA_WIDTH * SCALE, GBA_HEIGHT * SCALE)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture(
            PixelFormat::RGB24,
            sdl3::render::TextureAccess::Streaming,
            GBA_WIDTH,
            GBA_HEIGHT
        )
        .unwrap();

    // Event loop
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    info!("Shutting down");
                    break 'running;
                }
                _ => {}
            }
        }

        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..GBA_HEIGHT as usize {
                    for x in 0..GBA_WIDTH as usize {
                        let offset = y * pitch + x * 3;
                        buffer[offset] = (x % 256) as u8;
                        buffer[offset + 1] = (y % 256) as u8;
                        buffer[offset + 2] = 128;
                    }
                }
            })
            .unwrap();

        canvas.clear();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
