use iced::Point;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    CanvasMouseMove(Point),
    CanvasMouseClick(Point),
    Tick,
    CanvasSendCommand(Command),
    CanvasRemoveCell(Point),
}

#[derive(Debug, Clone, Copy)]
pub enum Command{
    SetWaterCell,
    SetSandCell,
    SetWallCell,
    IncreaseBrush,
    DecreaseBrush,
}