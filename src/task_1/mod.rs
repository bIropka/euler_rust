pub fn get_result(number: i32) -> i32 {
    let mut summa = 0;
    for n in 0..number {
        if n % 3 == 0 || n % 5 == 0 {
            summa = summa + n;
        }
    }
    summa
}