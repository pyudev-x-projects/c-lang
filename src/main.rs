use pyu_rust_util as pyu;
use std::env;
use std::fs;
use std::io::*;
mod util;
use crate::util::*;


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let file = fs::File::open(args[1].trim()).expect("File not found.");
    let reader = BufReader::new(file);
    let mut var = String::from("nil");

    for v in reader.lines() {
        let line = v?;

        let split = line.split_whitespace();

        let collection = split.collect::<Vec<&str>>();

        let block = collection[0];

        match block.trim() {
            "print" => {
                for v in collection {
                    if v != block {
                        print!("{} ", v.trim());
                    }
                }

                println!("");
            }

            "set" => {
                var = String::from(collection[1]);
            }

            "printv" => {
                println!("{}", var.trim());
            }

            "input" => {
                var = pyu::input("");
            }

            "printc" => {
                let n1: f32 = collection[1].parse().expect("Could not parse");
                let n2: f32 = collection[3].parse().expect("Could not parse");
                let operator = collection[2];

                match operator {
                    "+" => {
                        println!("{}", n1 + n2);
                    }

                    "-" => {
                        println!("{}", n1 - n2);
                    }

                    "*" => {
                        println!("{}", n1 * n2);
                    }

                    "/" => {
                        println!("{}", n1 / n2);
                    }

                    _ => {
                        println!("Invalid operator.");
                    }
                }
            }

            "setclr" => {
                pyu::change_color(collection[1]);
            }

            "lorem" => {
                println!("Lorem ipsum dolor sit amet, consectetur adipisicing elit. (Latin)");
                println!("It is very important for the customer to pay attention to the undergraduate process. (English - Translated)");
            }

            "newl" => {
                println!("");
            }

            "curl" => {
                pyu::curl(collection[1]);
            }

            "curlv" => {
                pyu::curl(var.trim());
            }

            "count" => {
                let n: i32 = collection[1].parse().expect("Could not parse");

                let v = pyu::number_vec(n);

                for i in v {
                    println!("{}", i);
                }
            }

            "ndir" => {
                fs::create_dir(collection[1])?;
                println!("Folder created at: {}", collection[1]);
            }

            "nfile" => {
                fs::File::create(collection[1])?;
                println!("New file created at: {}", collection[1]);
            }

            "ndirv" => {
                fs::create_dir(var.trim())?;
                println!("Folder created at: {}", var.trim());
            }

            "nfilev" => {
                fs::File::create(var.trim())?;
                println!("New file created at: {}", var.trim());
            }
            
            "exec" => {
                let output = pyu::exec(collection[1], collection[2]);
                
                pyu::output(output);
            }
            
            "execv" => {
                let output = pyu::exec(collection[1], &var.trim());
                
                pyu::output(output); 
            }
            
            "date" => {
                date();
                
            }

            _ => {
                if !block.starts_with("//") {
                    println!("Invalid syntax...");
                }
            }
        }
    }

    pyu::change_color("white");
    println!("\n\nRan program: {}", args[1].trim());

    return Ok(());
}
