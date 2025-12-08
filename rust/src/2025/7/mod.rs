use crate::util::get_input;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Air,
    Splitter,
    Laser,
    Emitter,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Air,
            '^' => Tile::Splitter,
            '|' => Tile::Laser,
            'S' => Tile::Emitter,
            _ => Tile::Air,
        }
    }
}

impl From<Tile> for char {
    fn from(t: Tile) -> Self {
        match t {
            Tile::Air => '.',
            Tile::Splitter => '^',
            Tile::Laser => '|',
            Tile::Emitter => 'S',
        }
    }
}

#[derive(Debug, Clone)]
struct Particle {
    x: usize,
    y: usize,
    count: usize,
}

struct State {
    grid: Vec<Vec<Tile>>,
    particles: Vec<Particle>,
    split_count: usize,
    ignore_collisions: bool,
}

impl State {
    fn new(input: &str, ignore_collisions: bool) -> Self {
        let grid = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();

        Self {
            grid,
            particles: Vec::new(),
            split_count: 0,
            ignore_collisions,
        }
    }

    fn is_out_of_bounds(&self, x: usize, y: usize) -> bool {
        y >= self.grid.len() || x >= self.grid[0].len()
    }

    fn paint(&mut self, x: usize, y: usize, tile: Tile) {
        self.grid[y][x] = tile;
    }

    fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.grid[y][x]
    }

    fn print(&self) {
        for row in &self.grid {
            let line: String = row.iter().map(|&t| char::from(t)).collect();
            println!("{}", line);
        }
        println!();
    }

    fn descend_particle(&mut self, particle_idx: usize) {
        let particle = &self.particles[particle_idx];
        let x = particle.x;
        let y = particle.y;
        let count = particle.count;

        self.paint(x, y, Tile::Laser);

        let below_y = y + 1;

        if self.is_out_of_bounds(x, below_y) {
            return;
        }

        let tile_below = self.get_tile(x, below_y);

        if tile_below != Tile::Splitter {
            self.move_particle(particle_idx, 0, 1);
        } else {
            self.split_count += count;

            if self.ignore_collisions {
                // Try to merge with existing particles moving left
                let left_x = x.wrapping_sub(1);
                if let Some(existing_idx) = self
                    .particles
                    .iter()
                    .position(|p| p.x == left_x && p.y == below_y)
                {
                    self.particles[existing_idx].count += count;
                } else if left_x < self.grid[0].len() {
                    self.particles.push(Particle {
                        x: left_x,
                        y: below_y,
                        count,
                    });
                }

                // Try to merge with existing particles moving right
                let right_x = x + 1;
                if let Some(existing_idx) = self
                    .particles
                    .iter()
                    .position(|p| p.x == right_x && p.y == below_y)
                {
                    self.particles[existing_idx].count += count;
                } else if right_x < self.grid[0].len() {
                    self.particles.push(Particle {
                        x: right_x,
                        y: below_y,
                        count,
                    });
                }

                // Mark for removal (will be removed later)
                self.particles[particle_idx].count = 0;
            } else {
                // Create new particle
                self.particles.push(Particle { x, y, count: 1 });

                // Move original left, new one right
                let new_idx = self.particles.len() - 1;
                self.move_particle(particle_idx, -1_i32, 1);
                self.move_particle(new_idx, 1, 1);
            }
        }
    }

    fn move_particle(&mut self, particle_idx: usize, dx: i32, dy: i32) {
        if particle_idx >= self.particles.len() {
            return;
        }

        let particle = &self.particles[particle_idx];
        let target_x = (particle.x as i32 + dx) as usize;
        let target_y = (particle.y as i32 + dy) as usize;

        if self.ignore_collisions {
            // When ignoring collisions, merge with existing particle at target
            if let Some(existing_idx) = self
                .particles
                .iter()
                .position(|p| p.x == target_x && p.y == target_y)
            {
                let count = self.particles[particle_idx].count;
                self.particles[existing_idx].count += count;
                self.particles[particle_idx].count = 0; // Mark for removal
                return;
            }
        } else {
            // Check if position is occupied
            let is_occupied = self
                .particles
                .iter()
                .any(|p| p.x == target_x && p.y == target_y);

            if is_occupied {
                self.particles[particle_idx].count = 0; // Mark for removal
                return;
            }
        }

        self.particles[particle_idx].x = target_x;
        self.particles[particle_idx].y = target_y;
    }

    fn run_simulation(&mut self) {
        let height = self.grid.len();

        for _ in 1..height {
            let particle_indices: Vec<usize> = (0..self.particles.len()).collect();

            for idx in particle_indices {
                if idx < self.particles.len() && self.particles[idx].count > 0 {
                    self.descend_particle(idx);
                }
            }

            // Remove particles marked for deletion (count = 0)
            self.particles.retain(|p| p.count > 0);
        }
    }
}

pub fn part1() {
    let input = get_input(2025, 7);

    let emitter_idx = input.find('S').expect("No emitter found");
    let first_line_len = input.lines().next().unwrap().len();
    let emitter_x = emitter_idx % (first_line_len + 1); // +1 for newline

    let mut state = State::new(&input, false);
    state.particles.push(Particle {
        x: emitter_x,
        y: 1,
        count: 1,
    });

    state.run_simulation();
    state.print();

    println!("{}", state.split_count);
}

pub fn part2() {
    let input = get_input(2025, 7);

    let emitter_idx = input.find('S').expect("No emitter found");
    let first_line_len = input.lines().next().unwrap().len();
    let emitter_x = emitter_idx % (first_line_len + 1); // +1 for newline

    let mut state = State::new(&input, true);
    state.particles.push(Particle {
        x: emitter_x,
        y: 1,
        count: 1,
    });

    state.run_simulation();
    state.print();

    println!("{}", state.split_count + 1);
}
