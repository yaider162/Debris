use crate::World;
use crate::message::{Command, Message};
use crate::particles::world::Cell;

use iced::keyboard::Event as KeyEvent;
use iced::mouse::{Button as MouseButton, Event as MouseEvent, ScrollDelta};
use iced::widget::canvas::Event::{Keyboard, Mouse};
use iced::widget::canvas::{self, Cache, Event};
use iced::{Point, Rectangle, Renderer, Theme, mouse};

type Action = canvas::Action<Message>;

pub struct MyCanvas<'a> {
    pub world: &'a World,
    pub cache: &'a Cache,

    pub mouse_pos: Option<(isize, isize)>,
    pub brush_size: isize,
    pub actual_cell: Cell,
}

pub struct CanvasState {
    pub is_clicked_left: bool,
    pub is_clicked_right: bool,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            is_clicked_left: false,
            is_clicked_right: false,
        }
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
                    if let Some(color) = self.cell_color(self.world.particles[idx]) {
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

            // Aqui dibujo otras cosas
            if let Some((mx, my)) = self.mouse_pos {
                // El color del placeholder
                let color = match self.actual_cell {
                    Cell::Nothing => iced::Color::TRANSPARENT,
                    Cell::Sand => iced::Color::from_rgba(0.9, 0.8, 0.3, 0.35),
                    Cell::Wall => iced::Color::from_rgba(0.4, 0.4, 0.4,0.35),
                    Cell::Water => iced::Color::from_rgba(0.102, 0.241, 0.194, 0.35),
                };

                let radius = self.brush_size;

                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        if dx * dx + dy * dy > radius * radius {
                            continue;
                        }

                        let px = mx + dx;
                        let py = my + dy;

                        if px >= 0
                            && px < self.world.width as isize
                            && py >= 0
                            && py < self.world.height as isize
                        {
                            frame.fill_rectangle(
                                iced::Point::new(
                                    px as f32 * self.world.cell_size,
                                    py as f32 * self.world.cell_size,
                                ),
                                size,
                                color,
                            );
                        }
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

            Mouse(MouseEvent::ButtonPressed(MouseButton::Right)) => {
                current_position.map(|point| on_cursor_click_right(point, _state))
            }

            Mouse(MouseEvent::ButtonReleased(MouseButton::Left)) => {
                on_cursor_leave(_state);
                None
            }

            Mouse(MouseEvent::ButtonReleased(MouseButton::Right)) => {
                on_cursor_right_leave(_state);
                None
            }

            Mouse(MouseEvent::WheelScrolled { delta }) => match delta {
                ScrollDelta::Lines { y, .. } | ScrollDelta::Pixels { y, .. } => {
                    on_scroll(y)
                }
            },

            Keyboard(KeyEvent::KeyPressed { key, .. }) => match key {
                iced::keyboard::Key::Character(s) => Some(on_option_key_pressed(s.as_str())),
                _ => None,
            },

            _ => on_non_action(current_position, _state),
        }
    }
}

impl MyCanvas<'_> {
    fn cell_color(&self, cell: Cell) -> Option<iced::Color> {
        match cell {
            Cell::Nothing => None,
            Cell::Sand => Some(iced::Color::from_rgb(0.9, 0.8, 0.3)),
            Cell::Wall => Some(iced::Color::from_rgb(0.4, 0.4, 0.4)),
            Cell::Water => Some(iced::Color::from_rgb(35.0/255.0, 195./255.0, 255.0/255.0)),
        }
    }
}
fn on_cursor_moved(point: Point, state: &CanvasState) -> Action {
    if state.is_clicked_left {
        canvas::Action::publish(Message::CanvasMouseClick(point))
    } else {
        canvas::Action::publish(Message::CanvasMouseMove(point))
    }
}

fn on_cursor_click(point: Point, state: &mut CanvasState) -> Action {
    state.is_clicked_left = true;
    canvas::Action::publish(Message::CanvasMouseClick(point))
}

fn on_cursor_click_right(point: Point, state: &mut CanvasState) -> Action {
    state.is_clicked_right = true;
    canvas::Action::publish(Message::CanvasRemoveCell(point))
}

fn on_cursor_leave(state: &mut CanvasState) {
    state.is_clicked_left = false;
}

fn on_cursor_right_leave(state: &mut CanvasState) {
    state.is_clicked_right = false;
}

fn on_non_action(point: Option<Point>, state: &mut CanvasState) -> Option<Action> {
    if state.is_clicked_left {
        point.map(|val| canvas::Action::publish(Message::CanvasMouseClick(val)))
    } else if state.is_clicked_right {
        point.map(|val| canvas::Action::publish(Message::CanvasRemoveCell(val)))
    } else {
        None
    }
}

fn on_option_key_pressed(s: &str) -> Action {
    canvas::Action::publish(Message::CanvasSendCommand(match s {
        "3" => Command::SetWaterCell,
        "2" => Command::SetWallCell,
        "1" => Command::SetSandCell,
        "s" => Command::DecreaseBrush,
        "w" => Command::IncreaseBrush,

        _ => Command::SetSandCell,
    }))
}

fn on_scroll(y: &f32) -> Option<Action> {
    if *y > 0.0 {
        Some(canvas::Action::publish(Message::CanvasSendCommand(
            Command::IncreaseBrush,
        )))
    } else if *y < 0.0 {
        Some(canvas::Action::publish(Message::CanvasSendCommand(
            Command::DecreaseBrush,
        )))
    } else {
        None
    }
}
