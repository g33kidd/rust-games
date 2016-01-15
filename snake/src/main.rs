extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod app;
mod constants;
mod grid;
mod block;
mod location;
mod direction;
mod snake;

use constants::*;

fn main() {
    const OPENGL: OpenGL = OpenGL::V4_5;

    let window: Window = WindowSettings::new(
        "Snake Game!",
        [WINDOW_WIDTH, WINDOW_HEIGHT]
    )
    .opengl(OPENGL).exit_on_esc(true)
    .build().unwrap();

    let mut gl: GlGraphics = GlGraphics::new(OPENGL);
    let mut app: App = App::new();

    for event in window.events().ups(60).max_fps(60) {
        match event {
            Event::Input(Input::Press(button)) => {
                app.key_press(button);
            },
            Event::Render(args) => {
                app.render(args);
            },
            Event::Update(args) => {
                app.update(args);
            },
            _ => { }
        }
    }

}
