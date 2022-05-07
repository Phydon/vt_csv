use std::{io, fs};
use std::io::Write;

const FILEPATH: &str = "./output.txt";

fn main() {
    let x = input();
    let vt = equation(x);

    write_to_file(&vt.to_string()).expect("Failed to write to file");

    println!("vt = {}", vt);
}

fn equation(num: f64) -> f64 {
    // change to whatever you want ot calculate
    let vt = num + 2.0;
    vt
}

fn input() -> f64 {
    println!("Please enter a number: ");

    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Failed to read input");
    let numeric = inp.trim().parse::<f64>();

    match numeric {
        Ok(_ok) => numeric.unwrap(),
        Err(_e) => 1 as f64,
    }
}

fn write_to_file(content: &String) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(FILEPATH)?;

    writeln!(file, "{}", content)?;
    Ok(())
    }
