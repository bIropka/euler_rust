pub fn get_result() -> u32 {
    for number in (900009..999999).rev() {
        if is_palindrome(number) {
            for n in (900..999).rev() {
                if number % n == 0 {
                    let factor = number_to_vec(number / n);
                    if factor.len() == 3 {
                        return number;
                    }
                }
            }
        }
    }
    0
}

fn is_palindrome(input: u32) -> bool {
    let digits: Vec<u32> = number_to_vec(input);
    for n in 0..(digits.len() / 2) {
        if digits[n] != digits[digits.len() - n - 1] {
            return false;
        }
    }
    true
}

fn number_to_vec(number: u32) -> Vec<u32> {
    let mut n = number;
    let mut result: Vec<u32> = Vec::new();
    while n > 0 {
        result.push(n % 10);
        n /= 10;
    }
    result.reverse();
    result
}
