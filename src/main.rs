use std::env;
use std::fs::File;
use std::io::Read;
use std::io::{BufReader, Error};

// how many digits are there in a base-10 number
// max is 2^64 - 1 (I think?)
fn number_width(number: &mut u64, width: &mut u64) {
    assert_eq!(*width, 0);
    *width = 1;
    loop {
        if *number / 10 > 0 {
            *width += 1;
            *number /= 10;
        } else {
            break;
        }
    }
}

// convert a list of bytes which repesent ascii digits
// to a base-10 number
fn bytes_to_num(bytes: &[u8], number: &mut u64) {
    for (i, &b) in bytes.iter().rev().enumerate() {
        // TODO: naughty - assuming lots of things here ;)
        if u64::from(b) < 48 || u64::from(b) > 57 {
            println!("byte problem: {}", b);
            println!("bytes: {:?}", bytes);
            continue;
        }
        let base = u64::from(b) - 48;
        *number += base * 10u64.pow(i as u32) as u64;
    }
}

pub fn self_locating(digits: &[u8]) -> Result<(), Error> {
    // could use zip() to handle indexes into π that might be greater than usize
    // let digit = digits.iter().zip(0u64..);
    for (i, &d) in digits.iter().enumerate() {
        // skip over any decimal points if we see them
        if u64::from(d) < 48 || u64::from(d) > 57 {
            continue;
        }

        let mut i_width = 0;
        let mut number = 0u64;
        number_width(&mut (i as u64), &mut i_width);
        if (i + i_width as usize) < digits.len() {
            bytes_to_num(&digits[i..(i + i_width as usize)], &mut number);
        }
        /*
        println!(
            "d: {}, i: {} width of i: {}, number string: {}",
            d as char, i, i_width, number
        );
        */

        // we don't count from 0 :)
        if number == (i + 1) as u64 {
            //println!("Found self locating number: {} at {}", number, i);
            println!("{}", number);
        }
    }

    Ok(())
}

use std::time::{Duration, Instant};
fn show_benchmarks(bench: &[(&str, Duration)]) {
    for (msg, duration) in bench {
        println!("{}: {:?}", msg, duration);
    }
}

// take a path to a data file containing digits of pi, default to a data file
// containing the first 1 million digits
fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "data/pi-1million.txt"
    };

    println!("Calculating the self locating numbers in π:");
    let mut bench = Vec::new();

    // store the whole thing in a single vector
    let mut pi_digits = Vec::new();
    let data = File::open(&path).expect("Couldn't open π data file");
    let mut buffer = BufReader::new(data);
    buffer
        .read_to_end(&mut pi_digits)
        .expect("Couldn't read all π digits");

    // only look in the decimal expansion (after 3.)
    let timer = Instant::now();

    // iterate over all digits in the main thread
    self_locating(&pi_digits[2..]).expect("Error finding all self locating strings in π");
    bench.push(("Elapsed time", timer.elapsed()));
    show_benchmarks(&bench);

    // iterate over all digits using all cores naively

    // iterate over all digits using a work stealing scheduler, aka

    Ok(())
}
