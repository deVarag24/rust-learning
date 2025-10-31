fn fabonacci_number(n: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if n == 0 {
        return first;
    }
    if n == 1 {
        return second;
    }

    for _ in 2..=n {
        let next = first + second;
        first = second;
        second = next;
    }

    return second;
}