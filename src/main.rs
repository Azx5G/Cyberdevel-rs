// Code: Cyberdevel-rs
// Copyright: Cyberdevel, 2023
// License: MIT

use iced::{
    Application, Command, Container, Element, Length, Settings, Text,
    executor, Subscription, window, Align, HorizontalAlignment, VerticalAlignment, Color, Clipboard
};

// Define the main function
pub fn main() -> iced::Result {
    // Run the application
    CyberdevelApp::run(Settings {
        window: window::Settings {
            size: (1453, 768), // Set window size
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

// Define the CyberdevelApp struct
struct CyberdevelApp;

// Implement the Application trait for the CyberdevelApp
impl Application for CyberdevelApp {
    type Executor = executor::Default; // Use the default executor
    type Message = (); // Use no messages
    type Flags = (); // Use no flags

    // Define the new function for the application
    fn new(_flags: ()) -> (CyberdevelApp, Command<Self::Message>) {
        (CyberdevelApp, Command::none())
    }

    // Define the title function for the application
    fn title(&self) -> String {
        String::from("Cyberdevel")
    }

    // Define the update function for the application
    fn update(&mut self, _message: Self::Message, _clipboard: &mut Clipboard) -> Command<Self::Message> {
        Command::none()
    }
    // Define the subscription function for the application
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    // Define the view function for the application
    fn view(&mut self) -> Element<Self::Message> {

        let content = Text::new("Coming Soon...\nCyberdevel, the development of your cyberspace.")
            .size(40) // Set text size
            .color(Color::from_rgb(0.0, 0.5, 0.0)) // Set text color
            .horizontal_alignment(HorizontalAlignment::Center) // Center text horizontally
            .vertical_alignment(VerticalAlignment::Center); // Center text vertically

        // Create a container with the content centered
        Container::new(content)
            .width(Length::Fill) // Container fills available width
            .height(Length::Fill) // Container fills available height
            .align_x(Align::Center) // Center container horizontally
            .align_y(Align::Center) // Center container vertically
            .into() // Convert container into Element
    }
}
