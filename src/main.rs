use iced::{debug, widget::{Canvas, Column, button, column, text}};
use iced::widget::canvas;
use iced::Element;

use view::ui::{MyCanvas,MessageUI};
use particles::world::World;

// Módulos
mod particles;
mod logic;
mod view;


fn main() -> iced::Result{
    let app= App::new();
    iced::run(App::update, App::view)
}


fn view(){

}

struct App{
    canvas:MyCanvas,
    world:World
}

impl Default for App{
    fn default()->Self{
        Self{canvas:MyCanvas{},world:World::new(80,60)}
    }
}

impl App{

    fn new()->Self{
        Self { canvas:  MyCanvas{},world:World::new(80,60)}
    }

    fn update(&mut self,message:MessageUI){
        match message {
            MessageUI::CanvasMouseMove(point) => {println!("{:?}", point);}
            MessageUI::CanvasMouseClick(point) => {println!("Click en {:?}", point);}
        }
    }

    fn view(&self)->Element<MessageUI>{
        canvas(&self.canvas).into()
    }
}