extern crate sfml;

use sfml::system::Vector2f;
use sfml::window::{ContextSettings, VideoMode, event, Close};
use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Color};

fn main() {
    // Create the window of the application
    let mut window = match RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                             "SFML Example",
                                             Close,
                                             &ContextSettings::default()) {
        Some(window) => window,
        None => panic!("Cannot create a new Render Window.")
    };

    // Create a CircleShape
    let mut circle = match CircleShape::new() {
        Some(circle) => circle,
        None       => panic!("Error, cannot create ball")
    };
    circle.set_radius(30.);
    circle.set_fill_color(&Color::red());
    circle.set_position(&Vector2f::new(100., 100.));

    while window.is_open() {
        // Handle events
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                _             => {/* do nothing */}
            }
        }

        // Clear the window
        window.clear(&Color::white());
        // Draw the shape
        window.draw(&circle);
        // Display things on screen
        window.display()
    }
}
