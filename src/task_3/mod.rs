pub fn get_result(number: u64) -> u64 {
    let primes_list: Vec<bool> = primes(number);
    let mut biggest_prime: u64 = 0;
    for (index, element) in primes_list.iter().enumerate() {
        if *element {
            if number % (index as u64) == 0 {
                biggest_prime = index as u64;
            }
        }
    }
    biggest_prime
}

fn primes(number: u64) -> Vec<bool> {
    let sqrt: usize = ((number as f64).sqrt() as u64) as usize;
    let mut result = vec![true; sqrt];
    result[0] = false;
    result[1] = false;
    for i in 2..result.len() {
        if result[i] {
            let mut j = i + i;
            while j < result.len() {
                result[j] = false;
                j += i;
            }
        }
    }
    result
}