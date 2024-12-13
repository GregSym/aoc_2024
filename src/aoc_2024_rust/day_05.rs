use std::collections::HashMap;

use pyo3::{pyfunction, PyResult};

use super::solve::Solution;

struct PageOrdering {
    rules: HashMap<i32, i32>,
    pages: Vec<Vec<i32>>,
}

impl PageOrdering {
    fn new(input: String) -> Self {
        let mut rules = HashMap::new();
        let mut pages: Vec<Vec<i32>> = vec![];
        for line in input.split_terminator("\n") {
            if line.contains("|") {
                let pair: Vec<i32> = line
                    .split_terminator("|")
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect();
                rules.insert(pair[0], pair[1]);
            }
            if line.contains(",") {
                let page = line
                    .split_terminator(",")
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect();
                pages.push(page);
            }
        }
        Self {
            rules: rules,
            pages: pages,
        }
    }

    fn correctly_ordered_pages(&self) -> Vec<Vec<i32>> {
        self.pages
            .clone()
            .into_iter()
            .filter(|row| {
                row.clone().into_iter().enumerate().all(|(i, page)| {
                    row.clone().split_off(i).into_iter().all(|subsequent_page| {
                        (self.rules.contains_key(&subsequent_page)
                            && self.rules[&subsequent_page] != page)
                            || !self.rules.contains_key(&subsequent_page)
                    })
                })
            })
            .collect()
    }

    fn middles(&self) -> Vec<i32> {
        self.correctly_ordered_pages()
            .into_iter()
            .map(|row| row[usize::from(row.len() / 2)])
            .collect()
    }
}

impl Solution for PageOrdering {
    fn solve(&self) -> PyResult<i32> {
        Ok(self.middles().into_iter().sum())
    }
}

#[pyfunction]
pub fn solve_day_05_pt_01(input: String) -> PyResult<i32> {
    return PageOrdering::new(input).solve();
}
