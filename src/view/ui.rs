use crate::World;
use crate::message::{Command, Message};
use crate::particles::world::Cell;

use iced::keyboard::{Event as KeyEvent};
use iced::mouse::{Button as MouseButton, Event as MouseEvent};
use iced::widget::canvas::Event::{Keyboard, Mouse};
use iced::widget::canvas::{self, Cache, Event};
use iced::{Point, Rectangle, Renderer, Theme, mouse};

type Action = canvas::Action<Message>;

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

impl canvas::Program<Message> for MyCanvas<'_> {
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
            // es solo el tamaño del rect, osea la particula
            let size = iced::Size::new(self.world.cell_size, self.world.cell_size);

            for y in 0..self.world.height {
                for x in 0..self.world.width {
                    let idx = self.world.index(x, y);
                    if let Some(color) = self.cell_color(self.world.particles[idx]){
                        // Dibujo directo ultra optimizado
                        frame.fill_rectangle(
                        iced::Point::new(
                            x as f32 * self.world.cell_size,
                            y as f32 * self.world.cell_size,
                        ),
                        size,
                        color,
                    );
                    }
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
        let current_position: Option<Point> = cursor.position_in(bounds);

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

            Keyboard(KeyEvent::KeyPressed { key, .. }) => {
                match key {
                    iced::keyboard::Key::Character(s) => {
                       Some(on_option_key_pressed(s.as_str()))
                    }
                    _ => None,
                }
            }

            _ => on_non_action(current_position, _state),
        }
    }
}

impl MyCanvas<'_>{
    fn cell_color(&self, cell: Cell) -> Option<iced::Color>{
        match cell {
            Cell::Nothing => None,
            Cell::Sand => Some(iced::Color::from_rgb(0.9, 0.8, 0.3)),
            Cell::Wall => Some(iced::Color::from_rgb(0.4, 0.4, 0.4))
        }
    }
}
fn on_cursor_moved(point: Point, state: &CanvasState) -> Action {
    if state.is_clicked {
        canvas::Action::publish(Message::CanvasMouseClick(point))
    } else {
        canvas::Action::publish(Message::CanvasMouseMove(point))
    }
}

fn on_cursor_click(point: Point, state: &mut CanvasState) -> Action {
    state.is_clicked = true;
    canvas::Action::publish(Message::CanvasMouseClick(point))
}

fn on_cursor_leave(state: &mut CanvasState) {
    state.is_clicked = false;
}

fn on_non_action(point: Option<Point>, state: &mut CanvasState) -> Option<Action> {
    if state.is_clicked {
        point.map(|val| canvas::Action::publish(Message::CanvasMouseClick(val)))
    } else {
        None
    }
}

fn on_option_key_pressed(s: &str) -> Action{
    canvas::Action::publish(Message::CanvasSendCommand(
        match s{
            "2"=>Command::SetWallCell,
            _=>Command::SetSandCell,
        }
    ))
}
