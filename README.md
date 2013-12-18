Getting Started with SDL-2.0 and Rust
=================
This project is a tutorial for using the multimedia library SDL-2.0 with Rust. While the tools exist to use this library with Rust, there's relatively scant documentation. I will guide users on how to use this library with Rust and provide sample code and documentation on how to use it.


### Initial Setup

To start working with Rust-SDL you will want to follow the installation guide provided here: https://github.com/AngryLawyer/rust-sdl2


```rust
extern mod sdl2;

mod windowDemo;

#[main]
fn main() {
	println!("Going to window system...");
	windowDemo::main();
}
 
 ```

Then, to start with your own code you will want to create two files a main file, and a file to work with SDL's windowing system. In this case, we'll call it windowDemo.rs. You will have to declare extern mod sdl2 at the beginning of the main file and put a short empty public main function in windowDemo.rs for now. In order to run properly, both of these files should be placed in your /rust-sdl2/src folder. Then you can proceed to call "rustpkg install <folder>" from your rust-sdl2 director, where <folder> is the folder they're placed in. For this example, I placed them in a folder called winDemo. To run your program after calling rustpkg install winDemo, call ./bin/winDemo. Right now because there's no code in windowDemo.rs, nothing should happen, but we'll get to that next.

### Opening a Window

First, as part of SDL, we want to declare what parts of SDL we want initiliazed. In this case we want their video system, so in Rust we simply declare:

```rust
sdl2::init([sdl2::InitVideo]);
 ```
Next, we'll need to declare our window. 

```rust
     let window = match sdl2::video::Window::new("rust-sdl2 demo: Videoooooo", sdl2::video::PosCentered, sdl2::video::PosCentered, 800, 600, [sdl2::video::Shown]) {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };

 ```


We need to handle the case where for some reason, the window is not set properly, which in Rust we do with a match statement that simply continues if nothing is wrong and throws an error if something does fail. sdl2::video can be used to get us most of the information we need. Passing the initialization flag for the window's type is a bit different in Rust. The various types you can pass besides Shown can be found in the /src/sdl2/video.rs file as a windowFlags enum.

```rust
 let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, [sdl2::render::Accelerated]) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };
    renderer.set_draw_color(sdl2::pixels::RGB(100, 200, 100));
    renderer.clear();
    renderer.present();
 ```
 
Next, we set up the renderer, which is very similar to how we set up a window, passing in our window instance, as well as some options to our renderer. After that we can start setting up some of the renderer's assets in the standard way. Here we simply initialize a background color, then we clear the screen and present it. This is the standard way of drawing the screen in SDL and the screen must always be cleared before being presented. Everytime a change is made to what is to be displayed, clear and present must be called again.

### Handling Events

```rust
let mut i = 0;
match sdl2::event::poll_event() {
                sdl2::event::QuitEvent(_) => break 'event,
                sdl2::event::KeyDownEvent(_, _, key, _, _) => {
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
 ```   
         
Handling events is relativly simple in SDL and we only need a main event loop. Rust's match feature is once again very useful here. First we declare the loop and then use Rust's matching feature to check which event happens. Essentially, we are purposefully entering an unending loop that will constantly poll for events. If no event is found to have occured, of course nothing happens. However, if an event is registered, it'll match to that and do the specified action. The main events we need to worry about are key pressed events, which happen if a key is pressed on the keyboard. Once a KeyDownEvent is detected, we can figure out exactly which key is pressed and do an action from there. If no action is specified for a specific key, it assumes nothing to do. Other wise, it performs the action. 

KeyDownEvent will provide you with 5 optional fields that it will give you information on. You'll mainly be concerned with the key pressed (the third field), but the other options may be useful to you as well. The second option will provide a pointer to the window that the event was detected from, and the first option will provide you with the time (since the windows instantiation) as an unsigned integer. I'm not exactly sure what the latter two options provide, actually, but I believe they relate to the keyboard and key pressed.

Here, we simply have the it detecting the right and left keys to darken or lighten the color of red, as an example of how events can be used to drive the program. Also important, as mentioned previously, is to clear and present the renderer every time so the screen can be updated.

You also probably want to detect other events such as the QuitEvent, which detects if the upper right red x is clicked. If you don't handle this event, your program won't close properly upon clicking the exit button. More details on what events you can detect can be found in SDL's api documentation; http://www.libsdl.org/release/SDL-1.2.15/docs/html/eventstructures.html .

### Rendering an Image

Perhaps the biggest flaw with rust-sdl2.0 right now is that it doesn't provide support for loading images. However, we can draw some basic shapes and surfaces to the screen.

#### "Final" Notes

While Rust's SDL bindings are an in progress work, they work very well, and are a good match if you're looking to design anything utilizing graphics.
This is an in progress tutorial and is continuing to be updated...
