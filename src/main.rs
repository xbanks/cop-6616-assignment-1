use std::sync::mpsc::Sender;
use std::sync::mpsc;
use std::thread;
use std::time::{Instant, Duration};

extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


use rand::thread_rng;
use rand::seq::SliceRandom;

static ONE_K: usize = 1_000;
static ONE_HM: usize = 100_000_000;


fn main() {
    println!("Starting Parallel Version");
    let (primes, time_taken) = parallel_sieve(ONE_HM, 8);
    write_results(primes, time_taken)
}

fn write_results(primes: Vec<usize>, time_taken: Duration) {
    let top_10 = primes.iter().rev().take(10).rev().collect::<Vec<&usize>>();

    let output_path = Path::new("output.txt");
    let display = output_path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&output_path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    
    let results = format!("total duration:\t{}s\ntotal primes:\t{}\nsum of primes:\t{}\ntop 10 primes:\t{}",
        time_taken.as_secs_f32(), 
        primes.len(), 
        primes.iter().sum::<usize>(), 
        top_10.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));

    match file.write_all(results.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

}

fn sieve_list(range: Vec<i32>, max: i32, sender: Sender<Vec<i32>>) {
    let mut non_prime_idxs = Vec::new();
    let start = Instant::now();
    let actual_range = range.iter().filter(|&x| x % 2 == 1).collect::<Vec<&i32>>();
    let range_len = actual_range.len();
    for &i in actual_range {
        if i <= 2 {
            continue;
        }
        for j in 3 .. {
            let idx = j * i;
            if idx >= max {
                break
            }
            non_prime_idxs.push(idx)
        }
    }
    sender.send(non_prime_idxs).unwrap();
    println!("Range Complete in {}s, Length of Range: {}", start.elapsed().as_secs(), range_len);
}

fn parallel_sieve(max: usize, threads: i32) ->  (Vec<usize>, Duration) {
    let mut sieved = vec![true; max];
    sieved[0] = false;
    sieved[1] = false;

    let max_iter = (max as f32).sqrt() as i32;
    let chunk = ((max_iter as f32) / (threads as f32)) as i32;

    let (sender, receiver) = mpsc::channel::<Vec<i32>>();
    
    let mut index_range: Vec<i32> = (0 ..= max_iter).collect();
    index_range.shuffle(&mut thread_rng());

    let mut children = Vec::new();
    for idx in 0 .. threads {
        let thread_sender = sender.clone();
        let from = (chunk * idx) as usize;
        let to = if idx == (threads - 1) {
            (max_iter + 1) as usize
        } else {
            (chunk * (idx + 1)) as usize
        };

        let range = index_range[from..to].to_vec();
        let child = thread::spawn(move || sieve_list(range, max as i32, thread_sender));

        children.push(child)
    }

    let start = Instant::now();
    for _ in 0 .. threads {
        let non_primes = receiver.recv().unwrap();
        for &idx in non_primes.iter() {
            sieved[idx as usize] = false;
        }
    }
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }
    let time_elapsed = start.elapsed();

    let prime_numbers: Vec<usize> = sieved.iter()
        .enumerate()
        .filter(|&(value, _)| value % 2 == 1 || value == 2)
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(prime_number, _)| prime_number)
        .collect();

    (prime_numbers, time_elapsed)
}