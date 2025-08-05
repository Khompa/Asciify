use iced::Task;


mod gui;
use gui::app::{FilterApp, Message};


fn main() -> iced::Result {
    iced::application("Filter App", update, view)
        .theme(theme)
        .run_with(FilterApp::new)
}

fn update(app: &mut FilterApp, message: Message) -> Task<Message> {
    app.update(message)
}

fn view(app: &FilterApp) -> iced::Element<Message> {
    app.view()
}

fn theme(app: &FilterApp) -> iced::Theme {
    app.theme()
}


