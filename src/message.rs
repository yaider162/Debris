use iced::Point;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CanvasMouseMove(Point),
    CanvasMouseClick(Point),
    Tick,
}