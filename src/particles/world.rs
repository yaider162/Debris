#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Sand,
    Nothing,
    Wall,
}

pub struct World {
    particles: Vec<Cell>,
    width: usize,
    height: usize,
}

impl World {
    pub fn new(width: usize, height:usize) -> Self {
        Self { particles: vec![Cell::Nothing; width*height], width, height }
    }

    // Funcion para indexar como 2d
    pub fn index(&self, x: usize, y:usize) -> usize {
        y*self.width+x
    }

    pub fn update(&mut self, x:usize, y:usize){
        let last = self.particles.clone();

        // Itero de abajo a arriba para actualizar estado
        for y in (0..self.height-1).rev() {
            for x in 0..self.width {
                let idx = self.index(x, y);

                // Si está abajo
                if last[idx] == Cell::Nothing && self.particles[idx]==Cell::Nothing {
                    let under = self.index(x, y+1);
                    self.particles[idx]=Cell::Nothing;
                    self.particles[under] = Cell::Sand;
                }
            }
        }
    }
}