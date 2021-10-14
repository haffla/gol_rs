use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub struct Game {
    pub cells: Vec<Vec<bool>>,
}

impl Game {
    pub fn from(input: &str, alive_char: char) -> Game {
        let cells: Vec<Vec<bool>> = input
            .split("\n")
            .filter(|arr| arr.len() > 0)
            .map(|line| line.chars().map(|c| c == alive_char).collect())
            .collect();
        Game { cells }
    }

    fn get_dimensions(&self) -> (i64, i64) {
        let height = self.cells.len();
        let width = self.cells[0].len();
        (width as i64, height as i64)
    }

    pub fn evolve(&self) -> Game {
        let (width, height) = self.get_dimensions();
        let mut new_cells = vec![vec![false; width as usize]; height as usize];

        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                new_cells[y][x] = self.determine_new_state(*cell, x as i64, y as i64)
            }
        }

        Game { cells: new_cells }
    }

    pub fn determine_new_state(&self, cell: bool, x: i64, y: i64) -> bool {
        match (cell, self.n_alive_neighbours_for(x, y)) {
            (true, 2) => true,
            (true, 3) => true,
            (false, 3) => true,
            _ => false,
        }
    }

    pub fn n_alive_neighbours_for(&self, x: i64, y: i64) -> i64 {
        let (width, height) = self.get_dimensions();
        let mut count = 0;

        let coords = [
            [y - 1, x - 1],
            [y - 1, x],
            [y - 1, x + 1],
            [y, x - 1],
            [y, x + 1],
            [y + 1, x - 1],
            [y + 1, x],
            [y + 1, x + 1],
        ];

        for c in coords {
            let y_coord = c[0];
            let x_coord = c[1];

            if y_coord >= 0
                && y_coord < height
                && x_coord >= 0
                && x_coord < width
                && self.cells[y_coord as usize][x_coord as usize]
            {
                count += 1;
            }
        }

        return count;
    }
}

mod ui {
    const ALIVE_CHAR: char = '#';
    const DEAD_CHAR: char = '.';

    use crate::Game;
    use std::process::Command;
    use std::thread;
    use std::time::Duration;

    pub fn draw(mut game: Game) {
        loop {
            Command::new("clear").status().unwrap();
            println!("############### Conway's Game of Life ###############");
            println!("{}", game_as_string(&game));
            println!("#####################################################");
            game = game.evolve();
            thread::sleep(Duration::from_millis(500));
        }
    }

    pub fn game_as_string(game: &Game) -> String {
        game.cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|alive| if *alive { ALIVE_CHAR } else { DEAD_CHAR })
                    .collect()
            })
            .map(|row: Vec<char>| row.iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let game = Game::from(&contents, 'o');
    ui::draw(game);

    Ok(())
}
