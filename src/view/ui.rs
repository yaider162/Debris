use iced::widget::canvas::{self, Event, Cache};
use iced::{mouse, Rectangle, Renderer, Theme};

use crate::World;
use crate::particles::world::Cell;
use crate::message::Message;

pub struct MyCanvas<'a>{
    pub world: &'a World,
    pub cache: &'a Cache,
}

pub struct CanvasState{
    pub is_clicked:bool,
}

impl Default for CanvasState{
    fn default()->Self{
        Self{is_clicked:false}
    }
}

impl canvas::Program<Message> for MyCanvas<'_> {
    type State = CanvasState;

    fn draw(
        &self,
        _state: &CanvasState,
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
                    if _state.is_clicked{
                        canvas::Action::publish(
                            Message::CanvasMouseClick(point),
                        )
                    }else{
                        canvas::Action::publish(
                            Message::CanvasMouseMove(point),
                        )
                    }
                })
            }

            Event::Mouse(mouse::Event::ButtonPressed(
                mouse::Button::Left,
            )) => {
                current_position.map(|point| {
                    _state.is_clicked=true;
                    canvas::Action::publish(
                        Message::CanvasMouseClick(point),
                    )
                })
            }

            Event::Mouse(mouse::Event::ButtonReleased(
                mouse::Button::Left,
            )) => {
                _state.is_clicked=false;
                None
            }

            Event::Mouse(mouse::Event::ButtonReleased(
                mouse::Button::Left,
            )) => {
                _state.is_clicked=false;
                None
            }

            _ => None,
        }
    }
}