use std::cell;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Sand,
    Nothing,
    Wall,
}

pub struct World {
    pub particles: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub cell_size: f32
}

impl World {
    pub fn new(width: usize, height:usize) -> Self {
        Self { particles: vec![Cell::Nothing; width*height], width, height, cell_size: 10.0 }
    }

    // Funcion para indexar como 2d
    pub fn index(&self, x: usize, y:usize) -> usize {
        y*self.width+x
    }

    pub fn update(&mut self){
        let last = self.particles.clone();

        // Itero de abajo a arriba para actualizar estado
        for y in (0..self.height-1).rev() {
            for x in 0..self.width {
                let idx = self.index(x, y);
                let under = self.index(x, y+1);

                // Si hay arena y el espacio de abajo estaba vacío en el estado anterior
                if last[idx] == Cell::Sand && last[under] == Cell::Nothing {
                    self.particles[idx]=Cell::Nothing;
                    self.particles[under] = Cell::Sand;
                }
            }
        }
    }

    pub fn set_cell(&mut self, x:usize, y:usize, cell: Cell){
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            self.particles[idx] = cell;
        }
    }
}