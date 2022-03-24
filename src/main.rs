use std::{thread, time};

fn main() {
    let mut columns: usize;
    let mut rows: usize;

    if let Some((w, h)) = term_size::dimensions() {
        columns = w / 2;
        rows = h - 2;
    }
    else {
        eprintln!("Cannot get terminal dimensions.");
        std::process::exit(1);
    }

    let mut iterations = 1;
    let mut last_table: Vec<Vec<bool>> = new_table(rows, columns);

    loop {
        println!("--- Iteration {} ---\n", iterations);

        let mut current_table = last_table.clone();

        for x in 0..rows - 1 {
            for y in 0..columns - 1 {
                let print_value = if last_table[x][y] {
                    "0"
                }
                else {
                    " "
                };

                print!("{} ", print_value);

                let mut top_left: u8 = 0;
                let mut top: u8 = 0;
                let mut top_right: u8 = 0;
                let mut left: u8 = 0;
                let mut right: u8 = 0;
                let mut bottom_left: u8 = 0;
                let mut bottom: u8 = 0;
                let mut bottom_right: u8 = 0;

                if x > 0 {
                    left = last_table[x - 1][y] as u8;

                    if y > 0 {
                        top_left = last_table[x - 1][y - 1] as u8;
                    }

                    if y < columns {
                        bottom_left = last_table[x - 1][y + 1] as u8;
                    }
                }

                if x < rows {
                    right = last_table[x + 1][y] as u8;

                    if y > 0 {
                        top_right = last_table[x + 1][y - 1] as u8;
                    }

                    if y < columns {
                        bottom_right = last_table[x + 1][y + 1] as u8;
                    }
                }

                if y > 0 {
                    top = last_table[x][y - 1] as u8;
                }

                if y < columns {
                    bottom = last_table[x][y + 1] as u8;
                }

                let neighbors = top_left
                    + top
                    + top_right
                    + left
                    + right
                    + bottom_left
                    + bottom
                    + bottom_right;

                if last_table[x][y] { // if cell is alive
                    if neighbors < 2 || neighbors > 3 {
                        current_table[x][y] = false;
                    }
                }
                else { // if cell is dead
                    if neighbors == 3 {
                        current_table[x][y] = true;
                    }
                }
            }

            print!("\n");
        }

        if last_table == current_table {
            last_table = new_table(rows, columns);
            iterations = 1;
        }
        else {
            last_table = current_table;
            iterations += 1;
        }

        thread::sleep(time::Duration::from_millis(120));
    }
}

fn new_table(rows: usize, columns: usize) -> Vec<Vec<bool>> {
    let mut table: Vec<Vec<bool>> = vec![vec![Default::default(); columns]; rows];

    for y in 0..columns {
        for x in 0..rows {
            table[x][y] = rand::random();
        }
    }

    return table;
}