use cosmic::app::{Core, Task};
use cosmic::iced::{Alignment, Length};
use cosmic::{Application, Element};

struct MyApp {
    core: Core,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ButtonClicked,
}

impl Application for MyApp {
    type Executor = cosmic::executor::Default;
    type Message = Message;
    type Flags = ();

    const APP_ID: &'static str = "com.example.mein-cosmic-tool";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        (MyApp { core }, Task::none())
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let content = cosmic::widget::column()
            .push(cosmic::widget::text("Daten-Dashboard").size(24))
            .push(cosmic::widget::vertical_space())
            .push(cosmic::widget::text("Hier könnten Ihre Delphi-Daten stehen."))
            .push(
                cosmic::widget::button::text("Details anzeigen")
                    .on_press(Message::ButtonClicked),
            )
            .spacing(10)
            .align_x(Alignment::Center);

        cosmic::widget::container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::ButtonClicked => {
                println!("Button geklickt!");
                Task::none()
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = cosmic::app::Settings::default();
    cosmic::app::run::<MyApp>(settings, ())?;
    Ok(())
}
