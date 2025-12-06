use advent_25::input::get_input_lines;

fn main() {
    // let turns = EXAMPLE
    //     .split('\n')
    //     .filter(|l| !l.is_empty())
    //     .map(parse_direction);

    let turns = get_input_lines(1).into_iter().map(|s| parse_direction(&s));

    let mut dial = 50;
    let mut zeros = 0;

    for turn in turns {
        
        dial += turn;
        dial %= 100;
        if dial == 0 {
            zeros += 1;
        }
    }

    println!("Stopped at zero {zeros} times");
}

fn parse_direction(turn: &str) -> i32 {
    let (dir, num) = turn.split_at(1);
    let num: i32 = num.parse().unwrap();
    match dir.chars().next().unwrap() {
        'L' => -num,
        'R' => num,
        _ => panic!(),
    }
}

static EXAMPLE: &str = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
