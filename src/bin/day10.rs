use std::collections::VecDeque;

use advent_25::input::{get_input_lines, get_input_lines_ex};

fn main() {
    let machines = get_input_lines_ex(10).into_iter().map(|l| Machine::new(&l));

    let total_minimal_light_presses: i32 = machines
        .clone()
        .map(|machine| machine.minimal_light_presses())
        .sum();
    println!("Total minimal for lights presses: {total_minimal_light_presses}");

    let total_minimal_joltage_presses: usize = machines
        .map(|machine| machine.minimal_joltage_presses())
        .sum();
    println!("Total minimal for joltages presses: {total_minimal_joltage_presses}");
}

#[derive(Debug)]
struct Machine {
    required_lights: Vec<i32>,
    buttons: Vec<Vec<usize>>,
    required_joltages: Vec<i32>,
}

impl Machine {
    fn new(description: &str) -> Self {
        let lights_description = description
            .split_once('[')
            .unwrap()
            .1
            .split_once(']')
            .unwrap()
            .0;
        let buttons_description = description
            .split_once(']')
            .unwrap()
            .1
            .split_once('{')
            .unwrap()
            .0;
        let joltage_description = description
            .split_once('{')
            .unwrap()
            .1
            .split_once('}')
            .unwrap()
            .0;
        let required_lights = lights_description
            .chars()
            .map(|c| match c {
                '.' => 0,
                '#' => 1,
                _ => panic!(),
            })
            .collect();
        let buttons = buttons_description
            .replace([')', '('], "")
            .split_ascii_whitespace()
            .map(|button_description| {
                button_description
                    .trim()
                    .split(',')
                    .map(|d| d.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        let required_joltages = joltage_description
            .trim()
            .split(',')
            .map(|d| d.parse::<i32>().unwrap())
            .collect();
        Self {
            required_lights,
            buttons,
            required_joltages,
        }
    }

    fn minimal_light_presses(&self) -> i32 {
        for num_presses in 1.. {
            for combination in combinations(self.buttons.len() as i32, num_presses) {
                if self.check_lights(combination) {
                    return num_presses;
                }
            }
        }

        panic!()
    }

    fn check_lights(&self, presses: Vec<usize>) -> bool {
        let mut lights = vec![0; self.required_lights.len()];
        for press in presses {
            for &light in &self.buttons[press] {
                lights[light] ^= 1;
            }
        }

        lights == self.required_lights
    }

    fn minimal_joltage_presses(&self) -> usize {
        let mut combinations = VecDeque::new();
        combinations.push_back(Vec::new());
        let mut max_len = 0;
        loop {
            let combination = combinations.pop_front().unwrap();
            if combination.len() > max_len {
                max_len = combination.len();
                println!("New max len {max_len} - {} total combinations to check", combinations.len());
            }
            for button in 0..self.buttons.len() as u8 {
                let mut new_combination = combination.clone();
                new_combination.push(button);
                match self.check_joltages(&new_combination) {
                    JoltageComparison::Match => return new_combination.len(),
                    JoltageComparison::NotMatch => {
                        combinations.push_back(new_combination);
                    }
                    JoltageComparison::TooMuch => {}
                }
            }
        }
    }

    fn check_joltages(&self, presses: &[u8]) -> JoltageComparison {
        let mut joltages = vec![0; self.required_joltages.len()];
        for &press in presses {
            for &light in &self.buttons[press as usize] {
                joltages[light] ^= 1;
            }
        }

        if joltages == self.required_joltages {
            return JoltageComparison::Match;
        }

        for i in 0..joltages.len() {
            if joltages[i] > self.required_joltages[i] {
                return JoltageComparison::TooMuch;
            }
        }

        JoltageComparison::NotMatch
    }
}

enum JoltageComparison {
    Match,
    NotMatch,
    TooMuch,
}

fn combinations(num_buttons: i32, num_presses: i32) -> impl Iterator<Item = Vec<usize>> {
    // could be optimized by not repeating differences in order, somehow
    (0..(num_buttons.pow(num_presses as u32)))
        .map(move |i| combination(num_buttons, num_presses, i))
}

fn combination(num_buttons: i32, num_presses: i32, combination_index: i32) -> Vec<usize> {
    { 0..num_presses }
        .map(|place| ((combination_index / (num_buttons.pow(place as u32))) % num_buttons) as usize)
        .collect()
}
