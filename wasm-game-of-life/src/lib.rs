mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Math;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro to provide `println!(..)`-style syntax for `console.log` logging
macro_rules! console_log {
    ( $( $t:tt )* ) => {
        log(&format!( $( $t )* ))
    }
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Cell {
    x: i32,
    y: i32,
    alive: bool,
}

#[wasm_bindgen]
impl Cell {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> Cell {
        Cell {
            x,
            y,
            alive: Math::random() > 0.5,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> i32 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> i32 {
        self.y
    }

    #[wasm_bindgen(getter)]
    pub fn alive(&self) -> bool {
        self.alive
    }

    #[wasm_bindgen(setter)]
    pub fn set_alive(&mut self, alive: bool) {
        self.alive = alive;
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Universe {
        console_log!("Creating new universe {}x{}", width, height);
        
        let mut cells = Vec::new();
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new(x as i32, y as i32));
            }
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<Cell> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = (y * self.width + x) as usize;
        self.cells.get(index).cloned()
    }

    pub fn tick(&mut self) {
        let mut next_cells = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_cell(x, y).unwrap();
                let alive_neighbors = self.count_alive_neighbors(x, y);

                let next_alive = match (cell.alive, alive_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };

                let mut next_cell = Cell::new(x as i32, y as i32);
                next_cell.set_alive(next_alive);
                next_cells.push(next_cell);
            }
        }

        self.cells = next_cells;
    }

    fn count_alive_neighbors(&self, x: u32, y: u32) -> u32 {
        let mut count = 0;
        
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && ny >= 0 {
                    if let Some(cell) = self.get_cell(nx as u32, ny as u32) {
                        if cell.alive {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }
}

// Advanced Rust features demonstration
#[wasm_bindgen]
pub fn demonstrate_rust_features() -> String {
    // Pattern matching
    let status = match Math::random() {
        x if x > 0.7 => "excellent",
        x if x > 0.4 => "good",
        _ => "learning",
    };

    // Iterators and functional programming
    let numbers: Vec<i32> = (1..=10).collect();
    let sum: i32 = numbers.iter().filter(|&&x| x % 2 == 0).sum();

    // Error handling with Result
    let result = divide(10.0, 2.0);
    let division_result = match result {
        Ok(val) => format!("Division result: {}", val),
        Err(e) => format!("Error: {}", e),
    };

    format!(
        "Rust Status: {}\nSum of evens 1-10: {}\n{}\nMemory safety guaranteed! 🦀",
        status, sum, division_result
    )
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}