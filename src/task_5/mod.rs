pub fn get_result(input: u64) -> u64 {
    let mut is_solution = false;
    let mut number: u64 = 2519;
    while !is_solution {
        number += 1;
        is_solution = check_number(number, input);
    }
    number
}

fn check_number(number: u64, input: u64) -> bool {
    for n in 2..=input {
        if number % n != 0 {
            return false;
        }
    }
    true
}
