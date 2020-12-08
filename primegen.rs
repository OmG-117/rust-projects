fn main() {
    println!("Enter number to check primes up to: ");

    let mut lim: String = String::new();

    std::io::stdin().read_line(&mut lim)
        .expect("Failed read; aborting.");

    let lim = match lim.trim().parse::<usize>() {
        Ok(lim) => lim + 1,
        Err(_) => {
            println!("Invalid integer; aborting.");
            return;
        }
    };

    let start_time = std::time::SystemTime::now();

    let primes = find_until(lim);

    let exec_time = std::time::SystemTime::now().duration_since(start_time)
        .expect("Execution took negative time.");

    println!("Found {:?} primes in {:?}", primes.len(), exec_time);
    println!("Largest prime found: {:?}", primes.last().expect("None found."));
}

fn find_until(lim: usize) -> Vec<usize> {
    let lim_fp = lim as f64;
    let expected_primes = (lim_fp/(lim_fp.ln() - 1.1)).ceil() as usize;
    println!("Allocating vector with capacity {}", expected_primes);
    let mut primes = Vec::with_capacity(expected_primes);

    for num in 2..lim {
        let mut is_prime = true;
        let lim_sqrt = (num as f64).sqrt().ceil() as usize;
        for candidate in &primes {
            if *candidate > lim_sqrt {
                break;
            }
            if num % candidate == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(num);
        }
    }

    return primes;
}

/* Unused (slower) code

fn check_until(lim: usize) -> usize {
    let mut count: usize = 0;

    for num in 2..lim {
        if is_prime(num) {
            count += 1;
        }
    }

    return count;
}

fn is_prime(num: usize) -> bool {
    let lim = (num as f64).sqrt() as usize + 1;

    for candidate in 2..lim {
        if num % candidate == 0 {
            return false;
        }
    }

    return true;
}

*/