use rand::random;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Sand,
    Nothing,
    Wall,
    Water,
}

pub struct World {
    pub particles: Vec<Cell>,
    pub particles_next: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub cell_size: f32,
    pub count_particles: isize,
    pub left_to_right: bool,
}

impl World {
    pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
        let size = width * height;
        Self {
            particles: vec![Cell::Nothing; size],
            particles_next: vec![Cell::Nothing; size],
            width,
            height,
            cell_size,
            count_particles: 0,
            left_to_right: true,
        }
    }

    // Funcion para indexar como 2d
    #[inline]
    pub fn index(&self, x: usize, y: usize) -> usize {y * self.width + x}
    // Funcion que se ejecuta 60x por frame
    pub fn update(&mut self) {
        self.particles_next.copy_from_slice(&self.particles);
        self.count_particles=0;

        // Itero de abajo a arriba para actualizar estado
        for y in (0..self.height - 1).rev() {
            if self.left_to_right {
                for x in 0..self.width {self.update_cell(x, y)}
            }else{
                for x in (0..self.width).rev() {self.update_cell(x, y)}
            }
        }
        self.left_to_right = !self.left_to_right;
        // Les hago swap a los vec para que no ocupe mas memoria y en el siguiente frame particles sea el next
        std::mem::swap(&mut self.particles, &mut self.particles_next);

        self.count_particles = self.particles.iter()
        .filter(|&&cell| cell == Cell::Sand || cell == Cell::Water)
        .count() as isize;
    }
    
    // Donde ejecuto todas las actualizaciones
    #[inline]
    pub fn update_cell(&mut self, x: usize, y: usize){
        let idx = self.index(x, y);
        match self.particles[idx] {
            Cell::Sand => {
                self.update_sand(x, y, idx);
            }
            Cell::Water => {
                self.update_water(x, y, idx);
            }
            _ => {}
        }
    }
    // Actualizar la arena
    pub fn update_sand(&mut self, x: usize, y:usize, idx:usize){
        let under = self.index(x, y + 1);
        if self.particles_next[under] == Cell::Nothing {
            self.particles_next[idx] = Cell::Nothing;
            self.particles_next[under] = Cell::Sand;
            self.particles[idx]=Cell::Nothing;
            return;
        }
        self.update_diagonal(x, y, idx, Cell::Sand);
    }
    // Actualizar el agua
    pub fn update_water(&mut self, x: usize, y: usize, idx:usize){
        let under = self.index(x, y+1);
        // Me voy pa abajo
        if self.particles_next[under] == Cell::Nothing{
            self.particles_next[idx] = Cell::Nothing;
            self.particles_next[under] = Cell::Water;
            self.particles[idx]=Cell::Nothing;
            return;
        }

        if self.update_diagonal(x, y, idx, Cell::Water) {return;}
        // En caso de que no pueda ir para abajo, primero miro las diagonales
        // Reutilizo el codigo de sand
        let directions = if random::<bool>() {
            [-1isize, 1]
        } else {
            [1, -1]
        };
        // En caso de que no pueda ir pa el diagonal va pa los lados
        for dx in directions {
            let nx = x as isize + dx;
            if nx < 0 || nx >= self.width as isize { continue; }

            let side = self.index(nx as usize, y);
            
            // Si el lado está completamente vacío, nos movemos ahí de forma suave
            if self.particles_next[side] == Cell::Nothing {
                self.particles_next[idx] = Cell::Nothing;
                self.particles_next[side] = Cell::Water;
                self.particles[idx]=Cell::Nothing;
                return;
            }
        }
    }
    
    pub fn update_diagonal(&mut self, x: usize, y: usize, idx:usize, cell_to: Cell) -> bool{
        let directions = if random::<bool>() {
            [-1isize, 1]
        } else {
            [1, -1]
        };
        for dx in directions {
            let nx = x as isize + dx;

            // Validar límites horizontales
            if nx < 0 || nx >= self.width as isize {
                continue;
            }

            let diagonal = self.index(nx as usize, y + 1);

            // Revisamos en el estado siguiente si el espacio diagonal está libre
            // Se revisa el siguiente ole por si otra particula ya ocupó el espacio, la curren lo respete
            if self.particles_next[diagonal] == Cell::Nothing {
                self.particles_next[idx] = Cell::Nothing;
                self.particles_next[diagonal] = cell_to;
                self.particles[idx]=Cell::Nothing;
                return true; //salimos del bucle de direcciones
            }
        }
        return false;
    }
    // Originalmente era solo una cell la que spawneaba, ahora va a ser como un brush
    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            self.particles[idx] = cell;
        }
    }
}
