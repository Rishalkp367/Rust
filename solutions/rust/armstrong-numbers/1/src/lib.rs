pub fn is_armstrong_number(num: u32) -> bool {
let num_string = num.to_string();
    let length = num_string.len() as u32;

    let sum_of_powers: u32 = num_string
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(length))
        .sum();

    sum_of_powers == num
}
