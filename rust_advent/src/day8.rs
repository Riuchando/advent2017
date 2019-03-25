use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

pub fn part1() -> Result<i32, Error> {
    let f = File::open("resources/day8.txt")?;
    let f = BufReader::new(f);
    let mut register = HashMap::new();
    for line in f.lines() {
        // do some parsing
        let line_split = line?
            .parse::<String>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
            .unwrap();
        let line_split = line_split.split(" ").collect::<Vec<&str>>();
        // 0  1  2 3 4 5 6
        // b inc 5 if a > 1
        // a inc 1 if b < 5
        let modify_register_value = match register.get(line_split[0]) {
            Some(x) => *x,
            None => 0,
        };

        let read_register_value = match register.get(line_split[4]) {
            Some(x) => *x,
            None => 0,
        };
        let increment_value = match line_split[1] {
            "inc" => line_split[2].parse::<i32>().unwrap(),
            "dec" => line_split[2].parse::<i32>().unwrap() * -1,
            _ => {
                println!("{:?}", line_split);
                0
            }
        };
        let comaprator_const = line_split[6].parse::<i32>().unwrap();
        let new_value = match line_split[5] {
            ">" => {
                if read_register_value > comaprator_const {
                    modify_register_value + increment_value
                } else {
                    modify_register_value
                }
            }
            "<" => {
                if read_register_value < comaprator_const {
                    modify_register_value + increment_value
                } else {
                    modify_register_value
                }
            }
            "<=" => {
                if read_register_value <= comaprator_const {
                    modify_register_value + increment_value
                } else {
                    modify_register_value
                }
            }
            ">=" => {
                if read_register_value >= comaprator_const {
                    modify_register_value + increment_value
                } else {
                    modify_register_value
                }
            }
            "==" => {
                if read_register_value == comaprator_const {
                    modify_register_value + increment_value
                } else {
                    modify_register_value
                }
            }
            "!=" => {
                if read_register_value != comaprator_const {
                    modify_register_value + increment_value
                } else {
                    modify_register_value
                }
            }
            _ => {
                println!("{:?}", line_split);
                0
            }
        };
        // println!(
        //     "{:?} {:} {:}",
        //     line_split,
        //     line_split[0].to_owned(),
        //     new_value
        // );
        register.insert(line_split[0].to_owned(), new_value);
    }
    let mut max = 0;
    for (k, v) in register {
        // println!("{:} {:}", k, v);
        if v > max {
            max = v;
        }
    }
    Ok(max)
}

pub fn part2() -> Result<i32, Error> {
    let mut max = 0;
    let f = File::open("resources/day8.txt")?;
    let f = BufReader::new(f);
    let mut register = HashMap::new();
    for line in f.lines() {
        // do some parsing
        let line_split = line?
            .parse::<String>()
            .map_err(|_err| Error::new(ErrorKind::InvalidData, format!("couldn't parse")))
            .unwrap();
        let line_split = line_split.split(" ").collect::<Vec<&str>>();
        // 0  1  2 3 4 5 6
        // b inc 5 if a > 1
        // a inc 1 if b < 5
        let modify_register_value = match register.get(line_split[0]) {
            Some(x) => *x,
            None => 0,
        };

        let read_register_value = match register.get(line_split[4]) {
            Some(x) => *x,
            None => 0,
        };
        let increment_value = match line_split[1] {
            "inc" => line_split[2].parse::<i32>().unwrap(),
            "dec" => line_split[2].parse::<i32>().unwrap() * -1,
            _ => {
                println!("{:?}", line_split);
                0
            }
        };
        let comaprator_const = line_split[6].parse::<i32>().unwrap();
        let new_value = match line_split[5] {
            ">" => {
                if read_register_value > comaprator_const {
                    modify_register_value + increment_value
                } else {
                    modify_register_value
                }
            }
            "<" => {
                if read_register_value < comaprator_const {
                    modify_register_value + increment_value
                } else {
                    modify_register_value
                }
            }
            "<=" => {
                if read_register_value <= comaprator_const {
                    modify_register_value + increment_value
                } else {
                    modify_register_value
                }
            }
            ">=" => {
                if read_register_value >= comaprator_const {
                    modify_register_value + increment_value
                } else {
                    modify_register_value
                }
            }
            "==" => {
                if read_register_value == comaprator_const {
                    modify_register_value + increment_value
                } else {
                    modify_register_value
                }
            }
            "!=" => {
                if read_register_value != comaprator_const {
                    modify_register_value + increment_value
                } else {
                    modify_register_value
                }
            }
            _ => {
                println!("{:?}", line_split);
                0
            }
        };
        // println!(
        //     "{:?} {:} {:}",
        //     line_split,
        //     line_split[0].to_owned(),
        //     new_value
        // );
        if new_value > max {
            max = new_value
        }
        register.insert(line_split[0].to_owned(), new_value);
    }
    Ok(max)
}
