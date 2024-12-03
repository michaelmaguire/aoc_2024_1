use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use std::collections::HashMap;

fn main() {
    println!("Hello, AoC_2024_1!");

    let mut left_vec: Vec<i64> = vec![];
    let mut right_vec: Vec<i64> = vec![];

    let mut right_map: HashMap<i64, i64> = HashMap::new();

    if let Ok(lines) = read_lines("./src/input.txt") {
        // Consumes the iterator, returns an ( Optional) String
        for line in lines.flatten() {
            // println!("{}", line);

            let inputs: Vec<i64> = line.split_whitespace()
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

            let left = inputs[0];
            let right = inputs[1];
            left_vec.push(left);
            right_vec.push(right);

            *right_map.entry(right.to_owned()).or_default() += 1;

        }

        left_vec.sort();
        right_vec.sort();

        let length = left_vec.len();

        println!("length {}",length);


        {
            let mut distances: i64 = 0;

            for i in 0..length {
                let left = left_vec[i];
                let right = right_vec[i];
                let distance    = (left - right).abs();
                distances += distance;

                //println!("i {} left {} right {} distance {} distances {}",i, left, right, distance, distances);
            }

            println!("Sum of distances: {}", distances);
        }


        {
            let mut similarities: i64 = 0;

            for i in 0..length {
                let left = left_vec[i];
                let mut similarity= 0;

                match right_map.get(&left) {
                    Some(count) => similarity=left*count,
                    None => ()
                }                

                similarities += similarity;

                //println!("i {} left {} right {} distance {} distances {}",i, left, right, distance, distances);
            }

            println!("Sum of similarities: {}", similarities);
        }



    } else {
        if let Ok(path) = env::current_dir() {
            println!("Error reading lines, the current directory is {}", path.display());
        } else {
            println!("Error reading lines, and can't print the current directory");

        }
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}