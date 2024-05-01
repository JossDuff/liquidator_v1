use statrs::statistics::Statistics;
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("benchmarks/benchmark-2024-04-26.txt").unwrap();

    let runs: Vec<(String, u64)> = input
        .lines()
        .filter_map(|line| {
            if !line.is_empty() {
                Some(parse_line(line))
            } else {
                None
            }
        })
        .collect();
    let runs = group_by_mark(runs);

    let mut worst_case_scenario = 0.0;
    let mut best_case_scenario = 0.0;
    let mut average_scenario = 0.0;
    for (mark, times) in runs {
        let times: Vec<f64> = times.into_iter().map(|n| n as f64).collect();
        let max = times.clone().max();
        let min = times.clone().min();
        let avg = times.clone().mean();
        let std_dev = times.clone().std_dev();
        let range = max - min;

        worst_case_scenario += max;
        best_case_scenario += min;
        average_scenario += avg;

        println!("{}", mark);
        println!("  max:     {max}");
        println!("  min:     {min}");
        println!("  range:   {range}");
        println!("  avg:     {avg}");
        println!("  std_dev: {std_dev}\n");
    }

    // println!("worst case runtime: {worst_case_scenario}");
    // println!("best case runtime:  {best_case_scenario}");
    // println!(
    //     "range:              {}",
    //     worst_case_scenario - best_case_scenario
    // );
    // println!("average runtime:    {average_scenario}");
}

fn parse_line(line: &str) -> (String, u64) {
    let initial_split = line.split(":").collect::<Vec<&str>>();
    let mark: &str = *initial_split.first().unwrap();
    let time: &str = *initial_split.last().unwrap();
    let time: u64 = time[..time.len() - 2]
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    (mark.into(), time)
}

fn group_by_mark(pairs: Vec<(String, u64)>) -> HashMap<String, Vec<u64>> {
    let mut map: HashMap<String, Vec<u64>> = HashMap::new();

    // Grouping u64 values under their corresponding String keys
    for (key, value) in pairs {
        map.entry(key).or_insert_with(Vec::new).push(value);
    }

    map
}

type Run = HashMap<String, u64>;
