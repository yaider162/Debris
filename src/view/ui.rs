use crate::World;
use crate::particles::world::Cell;
use iced::mouse::{Button as MouseButton, Event as MouseEvent};
use iced::keyboard::{Key, Event as KeyEvent};
use iced::widget::canvas::Event::{Mouse,Keyboard};
use iced::widget::canvas::{self, Event};
use iced::{Point, Rectangle, Renderer, Theme, mouse};

type Action = canvas::Action<MessageUI>;
use crate::message::Message;

pub struct MyCanvas<'a> {
    pub world: &'a World,
    pub cache: &'a Cache,
}

pub struct CanvasState {
    pub is_clicked: bool,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self { is_clicked: false }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MessageUI {
    CanvasMouseMove(Point),
    CanvasMouseClick(Point),
}

impl canvas::Program<MessageUI> for MyCanvas<'_> {
    type State = CanvasState;

    fn draw(
        &self,
        _state: &Self::State,
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
    ) -> Option<Action> {
        let current_position = cursor.position_in(bounds);

        match event {
            Mouse(MouseEvent::CursorMoved { .. }) => {
                current_position.map(|point| on_cursor_moved(point, _state))
            }

            Mouse(MouseEvent::ButtonPressed(MouseButton::Left)) => {
                current_position.map(|point| on_cursor_click(point, _state))
            }

            Mouse(MouseEvent::ButtonReleased(MouseButton::Left)) => {
                on_cursor_leave(_state);
                None
            }

            

            _ => None,
        }
    }
}

fn on_cursor_moved(point: Point, state: &CanvasState) -> Action {
    if state.is_clicked {
        canvas::Action::publish(MessageUI::CanvasMouseClick(point))
    } else {
        canvas::Action::publish(MessageUI::CanvasMouseMove(point))
    }
}

fn on_cursor_click(point: Point, state: &mut CanvasState) -> Action {
    state.is_clicked = true;
    canvas::Action::publish(MessageUI::CanvasMouseClick(point))
}

fn on_cursor_leave(state: &mut CanvasState) {
    state.is_clicked = false;
}
