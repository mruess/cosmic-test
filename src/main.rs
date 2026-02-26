use iced::widget::{button, column, container, text, vertical_space};
use iced::{Alignment, Element, Length, Task};

#[derive(Default)]
struct MyApp;

#[derive(Debug, Clone)]
enum Message {
    ButtonClicked,
}

fn main() -> iced::Result {
    use iced::{window, Size};
    iced::application("Daten-Dashboard", update, view)
        .theme(|_| iced::Theme::CatppuccinMocha)
        .window(window::Settings {
            size: Size::new(800.0, 500.0),
            ..window::Settings::default()
        })
        .run()
}

fn view(_app: &MyApp) -> Element<'_, Message> {
    let content = column![
        text("Daten-Dashboard").size(24),
        vertical_space(),
        text("Hier könnten Ihre Delphi-Daten stehen."),
        button("Details anzeigen").on_press(Message::ButtonClicked),
    ]
    .spacing(10)
    .align_x(Alignment::Center);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}

fn update(_app: &mut MyApp, message: Message) -> Task<Message> {
    match message {
        Message::ButtonClicked => {
            println!("Button geklickt!");
            Task::none()
        }
    }
}
