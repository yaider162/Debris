use iced::widget::canvas;
use iced::{Renderer, Theme, Rectangle};
use iced::mouse;

pub struct MyCanvas;

#[derive(Debug, Clone, Copy)]
pub enum MessageUI{
    
}
 

impl canvas::Program<MessageUI> for MyCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {         
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        
        let rect = canvas::Path::rectangle(
            iced::Point::ORIGIN,
            bounds.size(),
        );

        frame.fill(&rect, iced::Color::from_rgb(0.2, 0.4, 0.8));

        vec![frame.into_geometry()]
    }
}