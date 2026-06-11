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

    pub chunks_size: usize,
    pub chunks_w: usize,
    pub chunks_h: usize,
    pub dirty: Vec<bool>, // Array que contiene los chunks
    pub dirty_next: Vec<bool> // Array del siguiente frame.
}

impl World {
    pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
        let size = width * height;
        let chunks_size = 16;
        let chunks_w = (width + chunks_size - 1) / chunks_size;
        let chunks_h = (height + chunks_size - 1) / chunks_size;
        let num_chunks = chunks_w * chunks_h;

        Self {
            particles: vec![Cell::Nothing; size],
            particles_next: vec![Cell::Nothing; size],
            width,
            height,
            cell_size,
            count_particles: 0,
            left_to_right: true,
            chunks_size,
            chunks_w,
            chunks_h,
            dirty: vec![false; num_chunks],
            dirty_next: vec![false; num_chunks]
        }
    }

    // Funcion para indexar como 2d
    #[inline]
    pub fn index(&self, x: usize, y: usize) -> usize {y * self.width + x}
    // Funcion que se ejecuta 60x por frame
    pub fn update(&mut self) {
        self.particles_next.copy_from_slice(&self.particles);
        self.count_particles=0;
        self.dirty_next.fill(false);

        for y in (0..self.height - 1).rev() {
            let cy = y / self.chunks_size;
            if self.left_to_right {
                for x in 0..self.width {
                    if !self.is_chunk_dirty(x / self.chunks_size, cy) { continue; }
                    self.update_cell(x, y);
                }
            } else {
                for x in (0..self.width).rev() {
                    if !self.is_chunk_dirty(x / self.chunks_size, cy) { continue; }
                    self.update_cell(x, y);
                }
            }
        }

        self.left_to_right = !self.left_to_right;
        // Les hago swap a los vec para que no ocupe mas memoria y en el siguiente frame particles sea el next
        std::mem::swap(&mut self.particles, &mut self.particles_next);
        std::mem::swap(&mut self.dirty, &mut self.dirty_next);
        self.count_particles = self.particles.iter()
        .filter(|&&cell| cell == Cell::Sand || cell == Cell::Water)
        .count() as isize;
        
        // Prueba de activacion
        // let active_chunks = self.dirty.iter().filter(|&&d| d ).count();
        // println!("Chunks activos: {}/{}", active_chunks, self.dirty.len());
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
            self.mark_dirty(x, y);
            self.mark_dirty(x, y+1); // porque q tal se salga del limite del chunk
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
            self.mark_dirty(x, y);
            self.mark_dirty(x, y+1);
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
                self.mark_dirty(x, y);
                self.mark_dirty(nx as usize, y);
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
                self.mark_dirty(x, y);
                self.mark_dirty(nx as usize, y);
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
            self.mark_dirty_now(x, y);
        }
    }
    pub fn mark_dirty(&mut self, x:usize, y:usize){
        let cx = x / self.chunks_size;
        let cy = y / self.chunks_size;
        let idx = cy * self.chunks_w + cx;
        self.dirty_next[idx] = true;

        // También marca los chunks vecinos porque una partícula
        // en el borde puede afectar al chunk de al lado
        if cx > 0 { self.dirty_next[idx - 1] = true; }
        if cx + 1 < self.chunks_w { self.dirty_next[idx + 1] = true; }
        if cy > 0 { self.dirty_next[idx - self.chunks_w] = true; }
        if cy + 1 < self.chunks_h { self.dirty_next[idx + self.chunks_w] = true; }
    }
    pub fn mark_dirty_now(&mut self, x:usize, y:usize){
        let cx = x / self.chunks_size;
        let cy = y / self.chunks_size;
        let idx = cy * self.chunks_w + cx;
        self.dirty[idx] = true;

        if cx > 0 { self.dirty[idx - 1] = true; }
        if cx + 1 < self.chunks_w { self.dirty[idx + 1] = true; }
        if cy > 0 { self.dirty[idx - self.chunks_w] = true; }
        if cy + 1 < self.chunks_h { self.dirty[idx + self.chunks_w] = true; }
    }
    pub fn is_chunk_dirty(&mut self, cx:usize, cy:usize) -> bool {self.dirty[cy*self.chunks_w + cx]}
}
