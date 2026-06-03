use iced::{debug, widget::{Canvas, Column, button, column, text}, window::UserAttention};
use iced::widget::canvas;
use iced::Element;

use view::ui::{MyCanvas,MessageUI};
use particles::world::World;

use crate::particles::world;

// Módulos
mod particles;
mod logic;
mod view;


fn main() -> iced::Result{
    iced::run(App::update, App::view)
}


struct App{
    world:World
}

impl Default for App{
    fn default()->Self{
        Self{world:World::new(80,60)}
    }
}

impl App{

    fn update(&mut self,message:MessageUI){
        match message {
            MessageUI::CanvasMouseMove(point) => {println!("{:?}", point);}
            MessageUI::CanvasMouseClick(point) => {
                println!("Click en {:?}", point);
                let x = (point.x/self.world.cell_size) as usize;
                let y = (point.y/self.world.cell_size) as usize;
                self.world.set_cell(x, y, world::Cell::Sand);
            }
        }
    }

    fn view(&self)->Element<MessageUI>{
        canvas(MyCanvas {
            world: &self.world
        }).width(iced::Fill).height(iced::Fill).into()
    }
}