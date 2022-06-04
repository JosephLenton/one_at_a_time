use ::one_at_a_time_derive::one_at_a_time;

#[test]
fn it_allows_basic_function_calls() {
    let r = add_two_numbers(123, 456);

    assert_eq!(r, 579);
}

#[one_at_a_time]
fn add_two_numbers(a: u32, b: u32) -> u32 {
    a + b
}
