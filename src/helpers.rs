pub type Solution = [String; 2];

pub type DayString = &'static str;

pub fn read_day(n: u8) -> DayString {
    let path = format!("inputs/day{:02}.in", n);
    let error_msg = format!("Unable to access {}", path);
    Box::leak(
        std::fs::read_to_string(path)
            .expect(&error_msg)
            .into_boxed_str(),
    )
    .trim_end()
}

#[cfg(test)]
pub fn read_example<T: std::fmt::Display>(n: T) -> DayString {
    let path = format!("examples/ex{}.in", n);
    let error_msg = format!("Unable to access {}", path);
    Box::leak(
        std::fs::read_to_string(path)
            .expect(&error_msg)
            .into_boxed_str(),
    )
    .trim_end()
}
