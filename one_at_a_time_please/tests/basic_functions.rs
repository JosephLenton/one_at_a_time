use ::one_at_a_time_please::one_at_a_time;

#[test]
fn it_allows_basic_function_calls() {
    let r = add_two_numbers(123, 456);

    assert_eq!(r, 579);
}

#[test]
fn it_allows_recursion() {
    let r = recursive_double(10);

    assert_eq!(r, 20);
}

#[one_at_a_time]
fn add_two_numbers(a: u32, b: u32) -> u32 {
    a + b
}

#[one_at_a_time]
fn recursive_double(n: u32) -> u32 {
    if n > 0 {
        2 + recursive_double(n - 1)
    } else {
        0
    }
}
