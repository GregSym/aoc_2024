use pyo3::prelude::*;

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

#[pyfunction]
fn solve_day_02_pt_01(input: String) -> PyResult<i32> {
    let reactor: Vec<Vec<i32>> = input
        .split_terminator("\n")
        .map(|row| {
            row.split_terminator(" ").filter(|c| *c != "")
                .map(|str_num| str_num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    let mut tally = 0;
    for row in reactor.iter() {
        let zipped = row
            .iter()
            .zip(row.clone().split_off(1));
        let purely_increasing = zipped.clone().all(|(first, second)| first < &second);
        let purely_decreasing = zipped.clone().all(|(first, second)| first > &second);
        let diff_under_eq_3 = zipped.clone().all(|(first, second)| (first - &second).abs() <= 3);
        if (purely_increasing || purely_decreasing) && diff_under_eq_3 {
            tally += 1;
        }
    }
    Ok(tally)
}

/// A Python module implemented in Rust.
#[pymodule]
fn aoc_2024(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(solve_day_01_pt_02, m)?)?;
    m.add_function(wrap_pyfunction!(solve_day_02_pt_01, m)?)?;
    Ok(())
}
