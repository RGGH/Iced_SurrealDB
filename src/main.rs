use database::dodb;
use iced::widget::image::Handle;
use iced::alignment::Horizontal;
use iced::alignment::Vertical;
use iced::widget;
use iced::widget::Container;
use iced::widget::Theme;
use iced::window;
use iced::window::Position;
use iced::Length;
use iced::Renderer;
use iced::Size;
use iced::Task;
use iced::widget::image;

fn theme(_: &App) -> Theme {
    Theme::CatppuccinMacchiato
}

mod database;

struct App {
    count: i32,
    dbdata: String,
    search_for: String,
}

#[derive(Debug, Clone)]
enum Message {
    // Emitted when the increment ("+") button is pressed
    IncrementCount,
    // Emitted when decrement ("-") button is pressed
    DecrementCount,
    // Emitted when Fetch ("Fetch") button is pressed
    FetchRecords,
    // Data from Fetch response
    DBdata(String),
    //
    ContentChanged(String),
}

// Implement our App
impl App {
    fn new() -> Self {
        // initialize the app struct
        // with count value as 0.
        Self {
            count: 0,
            dbdata: "".to_string(),
            search_for : "".to_string(),
        }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::IncrementCount => self.count += 1,
            Message::DecrementCount => self.count -= 1,
            Message::FetchRecords => {
                // Task::perform requires two arguments:
                // 1. The future (an async operation)
                // 2. A function to map the result into a message
                return Task::perform(
                    async {
                        dodb() // The async operation
                    },
                    |result| {
                        match result {
                            Ok(data) => Message::DBdata(data),
                            Err(_) => Message::DBdata("Error fetching data".to_string()), // Handle error case
                        }
                    },
                );
            }
            Message::DBdata(data) => {
                // Process the data here, for example:
                self.dbdata = data;
                println!("{:?}", self.dbdata);
            }
            Message::ContentChanged(app)=>{
                self.search_for = app;
            }
        }
        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let image: iced::widget::Image<Handle> = image("resources/banner.jpg")
            .width(Length::Fill)
            .height(Length::Fill);

        let column = widget::column![
            image,
            widget::text_input("Type something here...", &self.search_for)
        .on_input(Message::ContentChanged),
            widget::button("-").on_press(Message::DecrementCount),
            widget::text(self.count.to_string()),
            widget::button("+").on_press(Message::IncrementCount),
            widget::button("Fetch Records").on_press(Message::FetchRecords),
            widget::text(self.dbdata.clone()),
        ]
        .spacing(10)
        .padding(25)
        .align_x(Horizontal::Center);

        let stuff_centered: Container<'_, Message, Theme, Renderer> =
            widget::Container::new(column)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .width(Length::Fill)
                .height(Length::Fill);

        stuff_centered.into()
    }
}

fn main() -> Result<(), iced::Error> {

    iced::application("Learning Rust", App::update, App::view)
        .window(window::Settings {
            position: Position::Centered,
            size: Size::new(800.0, 600.0),
            ..Default::default()
        })
        .theme(theme)
        .run_with(|| (App::new(), iced::Task::none()))
}
