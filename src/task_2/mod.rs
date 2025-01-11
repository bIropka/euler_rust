pub fn get_result(number: i64) -> i64 {
    let mut prev: i64 = 0;
    let mut next: i64 = 1;
    let mut summa: i64 = 0;
    while next <= number {
        let new_next = prev + next;
        prev = next;
        next = new_next;
        if next % 2 == 0 {
            summa += next;
        }
    }
    summa
}
