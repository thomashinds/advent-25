use std::fs;

pub fn get_input_lines(day: i32) -> Vec<String>
{
    get_input(day).split('\n').filter(|s| !s.is_empty()).map(String::from).collect()
}

pub fn get_input(day: i32) -> String
{
    fs::read_to_string(format!("input/day{day}.txt")).unwrap()
}