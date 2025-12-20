use itertools::Itertools;
use num_bigint::{BigInt, Sign};
use z3::{ast::Int, Optimize};

#[derive(Clone, Debug)]
struct Machine {
    buttons: Vec<Vec<usize>>,
    joltage_requirements: Vec<usize>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let mut groups = line.split(' ').skip(1).peekable();

        // buttons
        let buttons = groups
            .peeking_take_while(|group| group.starts_with('('))
            .map(|group| {
                let nums = &group[1..group.len() - 1];
                let nums = nums.split(',');
                nums.map(|n| n.parse::<usize>().unwrap()).collect_vec()
            })
            .collect_vec();

        // joltages
        let joltage_requirements = groups.next().unwrap();
        let joltage_requirements = &joltage_requirements[1..joltage_requirements.len() - 1];
        let joltage_requirements = joltage_requirements.split(',');
        let joltage_requirements = joltage_requirements
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec();

        Self {
            buttons,
            joltage_requirements,
        }
    }

    fn solve(&self) -> usize {
        let coefficient_names = (0..self.buttons.len()).map(|i| i.to_string()).collect_vec();
        let coefficients = (0..self.buttons.len())
            .map(|i| Int::new_const(coefficient_names[i].as_str()))
            .collect_vec();

        let optimize = Optimize::new();

        for coeff in coefficients.iter() {
            optimize.assert(&coeff.ge(0));
        }

        optimize.minimize(&Int::add(&coefficients));

        for (slot_index, joltage) in self.joltage_requirements.iter().enumerate() {
            let mut values = Vec::new();
            for (button_index, button) in self.buttons.iter().enumerate() {
                if button.iter().contains(&slot_index) {
                    values.push(&coefficients[button_index]);
                }
            }
            let sum = Int::add(&values);
            optimize.assert(&Int::eq(
                &sum,
                Int::from_big_int(&BigInt::new(Sign::Plus, vec![*joltage as u32])),
            ));
        }

        if let z3::SatResult::Sat = optimize.check(&[]) {
            let model = optimize.get_model().unwrap();
            coefficients
                .iter()
                .map(|c| model.eval(c, true).unwrap().as_u64().unwrap() as usize)
                .sum::<usize>()
        } else {
            panic!("no solution");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = std::fs::read_to_string("input/d10.txt")?
        .lines()
        .map(str::to_string)
        .collect_vec();
    println!(
        "{}",
        lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                println!("{}/{}", i, lines.len());
                Machine::parse(line).solve()
            })
            .sum::<usize>()
    );
    Ok(())
}
