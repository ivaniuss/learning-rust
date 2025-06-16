mod amm;
use amm::AMMPool;
use std::io::{self, Write};

fn main() {
    let mut pool = AMMPool::new(100.0, 100.0, 0.003);
    pool.status();

    loop {
        println!("\n What would you like to do? \n 1. View Status \n 2. Swap X for Y \n 3. Swap Y for X \n 4. Add Liquidity \n 5. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                pool.status();
            },
            "2" => {
                let x_in = get_input("How much X would you like to swap?").trim().parse::<f64>().unwrap();
                let y_out = pool.swap_x_for_y(x_in);
                println!("\n You got {:.4} y", y_out);
            }
            "3" => {
                let y_in = get_input("How much Y would you like to swap?").trim().parse::<f64>().unwrap();
                let x_out = pool.swap_y_for_x(y_in);
                println!("\n Swapped {} Y for {} X", y_in, x_out);
            }
            "4" => {
                let x_in = get_input("How much X would you like to add?").trim().parse::<f64>().unwrap();
                let y_out = pool.add_liquidity(x_in);
                println!("\n Added {} X for {} Y", x_in, y_out);
            }
            "5" => break,
            _ => println!("\n Invalid input"),
        }
    }
}

fn get_input(msg: &str) -> String {
    println!("{}", msg);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}
    
