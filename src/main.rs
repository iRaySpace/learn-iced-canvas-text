use iced::{
    alignment::{self},
    canvas::{self, Cache, Canvas, Cursor, Geometry, Path, Text},
    executor, Application, Color, Command, Container, Element, Length, Point, Rectangle, Settings,
    Vector,
};

fn main() -> iced::Result {
    Example::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct Example {
    cache: Cache,
}

impl Application for Example {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<()>) {
        (
            Example {
                cache: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Canvas Text")
    }

    fn update(&mut self, _message: ()) -> Command<()> {
        Command::none()
    }

    fn view(&mut self) -> Element<()> {
        let canvas = Canvas::new(self).width(Length::Fill).height(Length::Fill);
        Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}

impl canvas::Program<()> for Example {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let cache = self.cache.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;

            let background = Path::circle(center, radius);
            frame.fill(&background, Color::from_rgb8(10, 20, 30));

            let text = Text {
                color: Color::from_rgb8(100, 0, 0),
                size: 36.0,
                position: Point::new(frame.width(), frame.height()),
                horizontal_alignment: alignment::Horizontal::Right,
                vertical_alignment: alignment::Vertical::Bottom,
                ..Text::default()
            };

            frame.fill_text(Text {
                content: String::from("This is a text"),
                position: text.position - Vector::new(0.0, 16.0),
                ..text
            });
        });
        vec![cache]
    }
}
