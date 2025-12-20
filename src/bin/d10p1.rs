use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Clone, Debug)]
struct Machine {
    light_pattern: Vec<bool>,
    buttons: Vec<Vec<usize>>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let mut groups = line.split(' ').peekable();

        // light pattern
        let light_pattern = groups
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| match c {
                '.' => Some(false),
                '#' => Some(true),
                _ => None,
            })
            .collect_vec();

        // buttons
        let buttons = groups
            .peeking_take_while(|group| group.starts_with('('))
            .map(|group| {
                let nums = &group[1..group.len() - 1];
                let nums = nums.split(',');
                nums.map(|n| n.parse::<usize>().unwrap()).collect_vec()
            })
            .collect_vec();

        Self {
            light_pattern,
            buttons,
        }
    }

    fn solve(&self) -> usize {
        // breadth first search over sequences of buttons
        #[derive(Clone, Debug)]
        struct Step {
            lights: Vec<bool>,
            pressed_so_far: usize,
        }

        let mut frontier = VecDeque::new();
        frontier.push_back(Step {
            lights: vec![false; self.light_pattern.len()],
            pressed_so_far: 0,
        });

        loop {
            let step = frontier.pop_front().unwrap();

            // try pushing each button from this step
            for button in self.buttons.iter() {
                let mut next = step.clone();

                // push the button and let me know
                for &light_index in button {
                    next.lights[light_index] = !next.lights[light_index];
                }
                next.pressed_so_far += 1;

                // match?
                if next.lights == self.light_pattern {
                    return next.pressed_so_far;
                }

                frontier.push_back(next);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        std::fs::read_to_string("input/d10.txt")?
            .lines()
            .map(|line| Machine::parse(line).solve())
            .sum::<usize>()
    );
    Ok(())
}
