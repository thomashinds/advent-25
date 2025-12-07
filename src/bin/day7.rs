use advent_25::input::{get_input_lines, get_input_lines_ex};

fn main() {
    let lines = get_input_lines(7);
    let width = lines[0].len();
    let start = lines[0].find('S').unwrap();
    let mut splitters_reached = 0;
    let mut cur_beams = vec![false; width];
    let mut timelines = vec![0; width];
    cur_beams[start] = true;
    timelines[start] = 1;

    for line in lines {
        let mut next_beams = cur_beams.clone();
        let mut next_timelines = timelines.clone();
        for i in 0..width {
            if cur_beams[i] && line.chars().nth(i).unwrap() == '^' {
                splitters_reached += 1;
                next_beams[i] = false;
                next_beams[i-1] = true;
                next_beams[i+1] = true;
                next_timelines[i] = 0;
                next_timelines[i-1] += timelines[i];
                next_timelines[i+1] += timelines[i];
            }
        }
        cur_beams = next_beams;
        timelines = next_timelines;
    }
    
    println!("{splitters_reached} total splitters reached");
    let total_timelines: i64 = timelines.into_iter().sum();
    println!("{total_timelines} total timelines");
}