use iced::widget::{canvas,text};
use iced::{Element};
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
    spawn_cooldown: Duration // Cada cuanto deben nacer
}

impl Default for App{
    fn default()->Self{
        Self{world:World::new(200,150, 4.0),
            canvas_cache: canvas::Cache::default(),
            actual_cell: world::Cell::Sand,
            spawn_last: Instant::now(),
            spawn_cooldown: Duration::from_millis(50),
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
            Message::CanvasMouseMove(_point) => {}
            Message::CanvasMouseClick(point) => {
                if self.spawn_last.elapsed() >= self.spawn_cooldown{
                    let x = (point.x/self.world.cell_size) as usize;
                    let y = (point.y/self.world.cell_size) as usize;

                    self.world.set_cell(x, y, self.actual_cell);

                    self.spawn_last = Instant::now();
                }
            }

            Message::CanvasSendCommand(val)=>{
                self.actual_cell=match val{
                    Command::SetSandCell=>world::Cell::Sand,
                    Command::SetWallCell=>world::Cell::Wall,
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
}