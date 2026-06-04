use iced::widget::canvas;
use iced::{Element, Program};
use iced::{Subscription, time};

use view::ui::{MyCanvas};
use particles::world::World;
use crate::particles::world;
use crate::message::{Message,Command};
use std::time::Duration;

// Módulos
mod particles;
mod logic;
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
}

impl Default for App{
    fn default()->Self{
        Self{world:World::new(80,60), canvas_cache: canvas::Cache::default(),actual_cell: world::Cell::Sand }
    }
}

impl App{
    fn update(&mut self,message:Message){
        match message {
            Message::Tick => {
                self.world.update();
                self.canvas_cache.clear();
            }
            Message::CanvasMouseMove(point) => {println!("{:?}", point);}
            Message::CanvasMouseClick(point) => {
                println!("Click en {:?}", point);
                let x = (point.x/self.world.cell_size) as usize;
                let y = (point.y/self.world.cell_size) as usize;
                self.world.set_cell(x, y, self.actual_cell);
            }

            Message::CanvasSendCommand(val)=>{
                self.actual_cell=match val{
                    Command::SetSandCell=>world::Cell::Sand,
                    Command::SetWallCell=>world::Cell::Wall,
                    _=>world::Cell::Sand
                }
            }
            _=>{}
        }
    }


    fn view(&self)->Element<Message>{
        canvas(MyCanvas {
            world: &self.world,
            cache: &self.canvas_cache
        }).width(iced::Fill).height(iced::Fill).into()
    }


    fn subscription(state: &Self) -> Subscription<Message> {
        time::every(Duration::from_millis(16)).map(|_| Message::Tick)
    }   
}