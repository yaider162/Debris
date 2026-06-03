use iced::{debug, widget::{Column, button, column, text}};

// Módulos
mod particles;
mod logic;


fn main() -> iced::Result{
    iced::run(Counter::update, Counter::view)
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