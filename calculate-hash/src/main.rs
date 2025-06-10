use sha2::{Digest, Sha256};
use std::io;

fn calculate_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());

    let res = hasher.finalize();
    format!("{:x}", res)
}

fn main() {
    println!("===Program to calculate hashes===");
    println!("Type exit to finish the program\n");

    loop {
        println!("Enter text:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed when reading text!");

        let input = input.trim();

        if input.to_lowercase() == "exit" {
            println!("bye bye!");
            break;
        }

        let hash = calculate_hash(input);
        println!("Text: {}", input);
        println!("Text hashed: {}\n", hash);
        println!("{}", "-".repeat(50));
    }
}
