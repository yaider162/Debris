use iced::widget::canvas::{self, Event, Cache};
use iced::{mouse, Rectangle, Renderer, Theme};
use crate::World;
use crate::particles::world::Cell;
use crate::message::Message;

pub struct MyCanvas<'a>{
    pub world: &'a World,
    pub cache: &'a Cache,
}

impl canvas::Program<Message> for MyCanvas<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            for y in 0..self.world.height {
                for x in 0..self.world.width {
                    let idx = self.world.index(x, y);

                    let color = match self.world.particles[idx] {
                        Cell::Nothing => continue,
                        Cell::Sand => iced::Color::from_rgb(0.9, 0.8, 0.3),
                        Cell::Wall => iced::Color::from_rgb(0.4, 0.4, 0.4),
                    };

                    let rect = canvas::Path::rectangle(
                        iced::Point::new(
                            x as f32 * self.world.cell_size,
                            y as f32 * self.world.cell_size,
                        ),
                        iced::Size::new(self.world.cell_size, self.world.cell_size),
                    );

                    frame.fill(&rect, color);
                }
            }
        });
        vec![geometry]
    }
    fn update(
        &self,
        _state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<Message>> {
        let current_position = cursor.position_in(bounds);

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                current_position.map(|point| {
                    canvas::Action::publish(
                        Message::CanvasMouseMove(point),
                    )
                })
            }

            Event::Mouse(mouse::Event::ButtonPressed(
                mouse::Button::Left,
            )) => {
                current_position.map(|point| {
                    canvas::Action::publish(
                        Message::CanvasMouseClick(point),
                    )
                })
            }

            _ => None,
        }
    }
}