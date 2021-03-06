//extern mod sdl2;

use sdl2;

pub fn main() {
    sdl2::init([sdl2::InitVideo]);

    let window = match sdl2::video::Window::new("Window Demo", sdl2::video::PosCentered, sdl2::video::PosCentered, 800, 600, [sdl2::video::Shown]) {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, [sdl2::render::Accelerated]) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };

    let surface = match sdl2::surface::Surface::new([sdl2::surface::SWSurface], 50, 50, 32, 0, 100, 100, 0){
	Ok(surface) => surface,
        Err(err) => fail!(format!("failed to create surface: {}", err))
    };

    
    renderer.set_draw_color(sdl2::pixels::RGB(100, 200, 100));
    
    //renderer.copy(renderer.create_texture_from_surface(surface), None, None);
    //renderer.clear();
    //renderer.present();
    let mut i = 0;

    //'main : loop {
        'event : loop {
	    
            match sdl2::event::poll_event() {
                sdl2::event::QuitEvent(_) => break 'event,
                sdl2::event::KeyDownEvent(_, _,key, _, _) => {
                    if key == sdl2::keycode::EscapeKey {
                        break 'event
                    }
		    if key == sdl2::keycode::RightKey{
			println!("Right key!");	
			if(i > 250){
			i = 0;
			}			
			i = i+25;
			renderer.set_draw_color(sdl2::pixels::RGB(i, 0, 0));    
			}
		    if key == sdl2::keycode::LeftKey{
			println!("Left key!");	
			if(i < 0){
			i = 250;
			}			
			i = i-25;
			renderer.set_draw_color(sdl2::pixels::RGB(i, 0, 0));    
			}
                }
                _ => {}
            }
	 renderer.clear();
    	 renderer.present();
        }
	
    //}

    sdl2::quit();
}
