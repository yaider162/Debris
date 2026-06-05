use iced::widget::{canvas,text};
use iced::{Element, Point};
use iced::{Subscription, time};

use view::ui::{MyCanvas};
use particles::world::World;
use crate::particles::world;
use crate::message::{Message,Command};
use std::time::{Duration, Instant};

// Módulos
mod particles;
mod view;
mod message;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .subscription(App::subscription).run()
}


struct App{
    world:World,
    canvas_cache: canvas::Cache,
    actual_cell:world::Cell,

    // Control de velocidad de caida de las particulas, pq se ve chimba
    spawn_last: Instant, // Hace cuanto nacio la ultima
    spawn_cooldown: Duration, // Cada cuanto deben nacer

    // Un pincel, este no es el tamaño como tal, es el radio q ocupa
    brush_size: isize,

    // La posicion del mouse para el placeholder
    mouse_pos: Option<(isize,isize)>,
}

impl Default for App{
    fn default()->Self{
        Self{world:World::new(200,150, 4.0),
            canvas_cache: canvas::Cache::default(),
            actual_cell: world::Cell::Sand,
            spawn_last: Instant::now(),
            spawn_cooldown: Duration::from_millis(10),
            brush_size: 2,
            mouse_pos: None,
        }
    }
}

impl App{
    fn update(&mut self,message:Message){
        match message {
            Message::Tick => {
                self.world.update();
                self.canvas_cache.clear();
            }
            Message::CanvasMouseMove(point) => {
                self.update_mouse_pos(point);
            }
            Message::CanvasMouseClick(point) => {
                self.update_mouse_pos(point);
                if self.spawn_last.elapsed() >= self.spawn_cooldown{
                    self.draw_with_brush(point, self.actual_cell);
                    self.spawn_last = Instant::now();
                }
            }
            Message::CanvasRemoveCell(point) => {
                self.update_mouse_pos(point);
                let temp_cell=self.actual_cell;
                self.actual_cell=world::Cell::Nothing;
                if self.spawn_last.elapsed() >= self.spawn_cooldown{
                    self.draw_with_brush(point, world::Cell::Nothing);
                    self.spawn_last = Instant::now();
                }
                self.actual_cell=temp_cell;
            }

            Message::CanvasSendCommand(val)=>{
                match val {
                    Command::SetSandCell => self.actual_cell = world::Cell::Sand,
                    Command::SetWallCell => self.actual_cell = world::Cell::Wall,
                    Command::SetWaterCell => self.actual_cell = world::Cell::Water,
                    Command::IncreaseBrush => {self.brush_size = (self.brush_size + 1).min(10);}
                    Command::DecreaseBrush => {self.brush_size = (self.brush_size - 1).max(0);}
                }
            }
        }
    }


    fn view(&self)-> Element<'_, Message>{
        iced::widget::Column::new()
            .push(text(format!("Partículas: {}", self.world.count_particles)))
            .push(
                canvas(MyCanvas {
                    world: &self.world,
                    cache: &self.canvas_cache,

                    mouse_pos: self.mouse_pos,
                    brush_size: self.brush_size,
                    actual_cell: self.actual_cell,
                })
                .width(iced::Fill)
                .height(iced::Fill),
            )
            .spacing(10)
            .into()
    }


    fn subscription(_state: &Self) -> Subscription<Message> {
        time::every(Duration::from_millis(16)).map(|_| Message::Tick)
    }

    pub fn draw_with_brush(&mut self, point: Point, cell_to_place: world::Cell) {
        let center_x = (point.x/self.world.cell_size) as isize;
        let center_y = (point.y/self.world.cell_size) as isize;
        let radius = self.brush_size;

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy > radius * radius { continue; }

                let x = center_x + dx;
                let y = center_y + dy;

                if x >= 0 && x < self.world.width as isize && y >= 0 && y < self.world.height as isize {
                    let idx = self.world.index(x as usize, y as usize);
                    let current_tile = self.world.particles[idx];

                    match cell_to_place {
                        world::Cell::Nothing => {
                            self.world.set_cell(x as usize, y as usize, world::Cell::Nothing);
                        }
                        world::Cell::Sand => {
                            if current_tile == world::Cell::Nothing && rand::random::<f32>() > 0.3 {
                                self.world.set_cell(x as usize, y as usize, cell_to_place);
                            }
                        }
                        world::Cell::Water => {
                            if current_tile == world::Cell::Nothing {
                                self.world.set_cell(x as usize, y as usize, cell_to_place);
                            }
                        }
                        world::Cell::Wall => {
                            if current_tile != world::Cell::Wall {
                                self.world.set_cell(x as usize, y as usize, cell_to_place);
                            }
                        }
                    }
                }
            }
        }
    }

    fn update_mouse_pos(&mut self, point:Point){
        let x = (point.x / self.world.cell_size) as isize;
        let y = (point.y / self.world.cell_size) as isize;
        self.mouse_pos = Some((x, y));
        self.canvas_cache.clear();
    }
}