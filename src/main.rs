use std::time::Instant;

fn main() {
    let start = Instant::now(); // Start timing

    for _ in 1..=1_000_000 {
        // Simulating the loop without printing each number
    }

    let duration = start.elapsed(); // End timing

    println!("Finished processing 1 million iterations.");
    println!("It took {:?} to reach 1 million.", duration);
}
