use iced::widget::canvas::{self, Event};
use iced::{mouse, Color, Point, Rectangle, Renderer, Theme};

pub struct MyCanvas;

#[derive(Debug, Clone, Copy)]
pub enum MessageUI {
    CanvasMouseMove(Point),
    CanvasMouseClick(Point),
}

impl canvas::Program<MessageUI> for MyCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let rect = canvas::Path::rectangle(
            Point::ORIGIN,
            bounds.size(),
        );

        frame.fill(&rect, Color::from_rgb(0.2, 0.4, 0.8));

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<MessageUI>> {
        let current_position = cursor.position_in(bounds);

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                current_position.map(|point| {
                    canvas::Action::publish(
                        MessageUI::CanvasMouseMove(point),
                    )
                })
            }

            Event::Mouse(mouse::Event::ButtonPressed(
                mouse::Button::Left,
            )) => {
                current_position.map(|point| {
                    canvas::Action::publish(
                        MessageUI::CanvasMouseClick(point),
                    )
                })
            }

            _ => None,
        }
    }
}