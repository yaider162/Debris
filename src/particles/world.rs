#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Sand,
    Nothing,
    Wall,
}

struct world {
    particles: Vec<Cell>,
    width: usize,
    height: usize,
}

impl world {
    fn new(width: usize, height:usize) -> Self {
        Self { particles: vec![Cell::Nothing; width*height], width, height }
    }

    // Funcion para indexar como 2d
    fn index(&self, x: usize, y:usize) -> usize {
        y*self.width+x
    }

}