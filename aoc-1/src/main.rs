use std::fs::{File, self};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./src/input.txt") {
        let mut current_max: u32 = 0;
        let mut current_sum: u32 = 0;

        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let line_unwrapped = ip;

                if line_unwrapped == "" {
                    current_max = std::cmp::max(current_max, current_sum);
                    current_sum = 0;
                    continue;
                }
                let amount = line_unwrapped.parse::<u32>().unwrap();
                current_sum += amount;
            }
        }

        print!("a={}", current_max);
    }


    let file = fs::read_to_string("./src/input.txt").unwrap();
    print!("b={}", second(&file.as_str()));
}

pub fn second(input: &str) -> u32 {
    let mut sums: Vec<u32> = vec![];
    let mut current_sum: u32 = 0;
    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(number) => {
                current_sum += number;
            }
            Err(_) => {
                // lets do some house keeping
                if sums.len() < 3 {
                    sums.push(current_sum);
                    current_sum = 0;
                    continue;
                }
                // ok, so its full, let's check what we need to do
                sums.sort();
                check(&mut sums, &mut current_sum);
            }
        }
    }

    // handle end as well
    sums.sort();
    check(&mut sums, &mut current_sum);
    return sums.iter().sum();
}

fn check(sums: &mut Vec<u32>, current_sum: &mut u32) {
    for (i, item) in sums.iter_mut().enumerate() {
        if &current_sum > &item {
            sums[i] = current_sum.clone();
            break;
        }
    }
    *current_sum = 0;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_second() {
        let example = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

        let result = second(&example);

        assert_eq!(result, 45000);
    }
}
