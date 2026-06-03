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
        //todo
    }

    fn view(&self)->Element<MessageUI>{
        canvas(&self.canvas).into()
    }
}

#[derive(Default)]
struct Counter {
    count: i64,
}

impl Counter {
    fn update(&mut self, message: Message){
        match message {
            Message::Decrement => {self.count-=1}
            Message::Increment => {self.count+=1}
            Message::Reset => {self.count=0}
        }
    }

    fn view(&self) -> Column<Message>{
        let counter = Counter {count:0};

        let increment = button("+").on_press(Message::Increment);
        let decrement = button("-").on_press(Message::Decrement);
        let reset = button("RESET").on_press(Message::Reset);

        let txt = text(self.count);

        let interface = column![increment, txt, decrement, reset];

        interface
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
    Reset,
}