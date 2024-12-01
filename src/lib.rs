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
    Ok(0)
}

/// A Python module implemented in Rust.
#[pymodule]
fn aoc_2024(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(solve_day_01_pt_02, m)?)?;
    Ok(())
}
