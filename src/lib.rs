use pyo3::prelude::*;
use regex::Regex;
mod aoc_2024_rust;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn solve_day_01_pt_02(input: String) -> PyResult<i32> {
    let mut l_parsed: Vec<i32> = vec![];
    let mut r_parsed: Vec<i32> = vec![];
    for line in input.split_terminator("\n") {
        let pair: Vec<&str> = line.split("   ").collect();
        l_parsed.push(pair[0].parse::<i32>().unwrap());
        r_parsed.push(pair[1].parse::<i32>().unwrap());
    }
    let mut tally = 0;
    for i in l_parsed {
        tally += i * i32::try_from(r_parsed.iter().filter(|&e| *e == i).count()).unwrap();
    }
    Ok(tally)
}

/// solving day 02 part 01
#[pyfunction]
fn solve_day_02_pt_01(input: String) -> PyResult<i32> {
    let reactor: Vec<Vec<i32>> = input
        .split_terminator("\n")
        .map(|row| {
            row.split_terminator(" ")
                .filter(|c| *c != "")
                .map(|str_num| str_num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let mut tally = 0;
    for row in reactor.iter() {
        let zipped = row.iter().zip(row.clone().split_off(1));
        let purely_increasing = zipped.clone().all(|(first, second)| first < &second);
        let purely_decreasing = zipped.clone().all(|(first, second)| first > &second);
        let diff_under_eq_3 = zipped
            .clone()
            .all(|(first, second)| (first - &second).abs() <= 3);
        if (purely_increasing || purely_decreasing) && diff_under_eq_3 {
            tally += 1;
        }
    }
    Ok(tally)
}

#[allow(dead_code)]
trait InIterPop: Iterator + Clone {
    type ResultType: Iterator<Item = Self::Item>;
    fn chained_iter_pop(&self, index: &usize) -> impl Iterator<Item = Self::Item>;
}
impl<T> InIterPop for T
where
    T: Iterator + Clone,
{
    type ResultType = T;
    fn chained_iter_pop(&self, index: &usize) -> impl Iterator<Item = Self::Item> {
        self.clone()
            .enumerate()
            .filter(|(j, _)| *j != *index)
            .map(|(_, pair)| pair)
            .into_iter()
    }
}

#[pyfunction]
fn solve_day_02_pt_02(input: String) -> PyResult<i32> {
    print!("{}", input);
    let reactor: Vec<Vec<i32>> = input
        .split_terminator("\n")
        .map(|row| {
            row.split_terminator(" ")
                .filter(|c| *c != "")
                .map(|str_num| str_num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let mut tally = 0;
    for row in reactor.iter() {
        let can_inc = (0..row.len()).any(|i| {
            let mut temp_row = row.clone();
            temp_row.remove(i);
            let zipped = temp_row.iter().zip(temp_row.clone().split_off(1));
            let purely_increasing = zipped.clone().all(|(first, second)| first < &second);
            let purely_decreasing = zipped.clone().all(|(first, second)| first > &second);
            let diff_under_eq_3 = zipped
                .clone()
                .all(|(first, second)| (first - &second).abs() <= 3);
            (purely_increasing || purely_decreasing) && diff_under_eq_3
        });
        if can_inc {
            tally += 1;
        }
    }
    Ok(tally)
}

trait HasGroup {
    fn has_group(&self, group: &str) -> bool;
}
impl HasGroup for regex::Captures<'_> {
    fn has_group(&self, group: &str) -> bool {
        self.name(group).map_or("", |c| c.as_str()) != ""
    }
}

#[pyfunction]
fn solve_day_03_pt_01(input: String) -> PyResult<i32> {
    Ok(Regex::new(r"mul\((?P<arg0>\d+)\,(?P<arg1>\d+)\)")
        .unwrap()
        .captures_iter(&input)
        .map(|m| m["arg0"].parse::<i32>().unwrap() * m["arg1"].parse::<i32>().unwrap())
        .sum())
}

#[pyfunction]
fn solve_day_03_pt_02(input: String) -> PyResult<i32> {
    let mut capturing = true;
    Ok(
        Regex::new(r"(?:mul\((?P<arg0>\d+),(?P<arg1>\d+)\))|(?P<stop>don't\(\))|(?P<start>do\(\))")
            .unwrap()
            .captures_iter(&input)
            .map(|m| {
                if m.has_group("start") {
                    capturing = true;
                    0
                } else if m.has_group("stop") {
                    capturing = false;
                    0
                } else if capturing {
                    m["arg0"].parse::<i32>().unwrap() * m["arg1"].parse::<i32>().unwrap()
                } else {
                    0
                }
            })
            .sum(),
    )
}

/// A Python module implemented in Rust.
#[pymodule]
fn aoc_2024(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(solve_day_01_pt_02, m)?)?;
    m.add_function(wrap_pyfunction!(solve_day_02_pt_01, m)?)?;
    m.add_function(wrap_pyfunction!(solve_day_02_pt_02, m)?)?;
    m.add_function(wrap_pyfunction!(solve_day_03_pt_01, m)?)?;
    m.add_function(wrap_pyfunction!(solve_day_03_pt_02, m)?)?;
    m.add_function(wrap_pyfunction!(aoc_2024_rust::day_04::solve_day_04_pt_01, m)?)?;
    m.add_function(wrap_pyfunction!(aoc_2024_rust::day_04::solve_day_04_pt_02, m)?)?;
    m.add_function(wrap_pyfunction!(aoc_2024_rust::day_05::solve_day_05_pt_01, m)?)?;
    m.add_function(wrap_pyfunction!(aoc_2024_rust::day_05::solve_day_05_pt_02, m)?)?;
    Ok(())
}
