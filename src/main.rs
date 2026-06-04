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
                let x = (point.x / self.world.cell_size) as isize;
                let y = (point.y / self.world.cell_size) as isize;
                self.mouse_pos = Some((x, y));
                self.canvas_cache.clear();
            }
            Message::CanvasMouseClick(point) => {
                
                let x = (point.x / self.world.cell_size) as isize;
                let y = (point.y / self.world.cell_size) as isize;
                self.mouse_pos = Some((x, y));

                if self.spawn_last.elapsed() >= self.spawn_cooldown{
                    self.draw_with_brush(point);
                    self.spawn_last = Instant::now();
                }
            }
            Message::CanvasRemoveCell(point) => {
                let temp_cell=self.actual_cell;
                self.actual_cell=world::Cell::Nothing;
                if self.spawn_last.elapsed() >= self.spawn_cooldown{
                    self.draw_with_brush(point);
                    self.spawn_last = Instant::now();
                }
                self.actual_cell=temp_cell;
            }

            Message::CanvasSendCommand(val)=>{
                match val {
                    Command::SetSandCell => self.actual_cell = world::Cell::Sand,
                    Command::SetWallCell => self.actual_cell = world::Cell::Wall,
                    Command::IncreaseBrush => {
                        // Limitamos el tamaño máximo para no congelar la CPU (ej. radio de 10 celdas)
                        self.brush_size = (self.brush_size + 1).min(10);
                    }
                    Command::DecreaseBrush => {
                        // Radio mínimo 0 (una sola celda)
                        self.brush_size = (self.brush_size - 1).max(0);
                    }
                }
            }
        }
    }


    fn view(&self)-> Element<'_, Message>{
        iced::widget::Column::new()
            .push(text(format!("Partículas: {}", self.particle_count())))
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
    pub fn particle_count(&self) -> usize{
        self.world.particles.iter().filter(|&&c| c==self::world
            ::Cell::Sand).count()
    }
    pub fn draw_with_brush(&mut self, point: Point) {
        let center_x = (point.x/self.world.cell_size) as isize;
        let center_y = (point.y/self.world.cell_size) as isize;
        let radius = self.brush_size;
                    
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                
                // (Para pincel redondo): 
                // desmarcar la siguiente línea de la ecuación del círculo:
                if dx*dx + dy*dy > radius*radius { continue; }

                let x = center_x + dx;
                let y = center_y + dy;

                if x >= 0 && x < self.world.width as isize && y >= 0 && y < self.world.height as isize {

                    let target_cell = iced::Point::new(x, y);
                    let idx = self.world.index(target_cell.x as usize, target_cell.y as usize);
                    let target_cell_value = self.world.particles[idx];

                    // Cuando es arena, que tenga un pocco de aleatoriedad
                    if self.actual_cell == world::Cell::Sand {
                        if target_cell_value == world::Cell::Nothing{
                            if rand::random::<f32>() > 0.3 {
                                self.world.set_cell(x as usize, y as usize, self.actual_cell);
                            }
                        }
                    } else if self.actual_cell == world::Cell::Nothing{
                        self.world.set_cell(x as usize, y as usize, self.actual_cell);
                    } 
                    else {
                        // Por ahora, este es cuando es pared
                        if target_cell_value != world::Cell::Wall{
                            self.world.set_cell(x as usize, y as usize, self.actual_cell);
                        }
                    }
                }
            }
        }
    }
}