use std::collections::HashMap;

use pyo3::{pyfunction, PyResult};

use super::solve::Solution;

struct PageOrdering {
    rules: HashMap<i32, Vec<i32>>,
    pages: Vec<Vec<i32>>,
}

impl PageOrdering {
    fn new(input: String) -> Self {
        let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut pages: Vec<Vec<i32>> = vec![];
        for line in input.split_terminator("\n") {
            if line.contains("|") {
                let pair: Vec<i32> = line
                    .split_terminator("|")
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect();
                if rules.contains_key(&pair[0]) {
                    rules.get_mut(&pair[0]).unwrap().push(pair[1]);
                } else {
                    rules.insert(pair[0], vec![pair[1]]);
                }
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
                    let mut copied_row = row.clone();
                    if i == row.len() && self.rules.contains_key(&page) {
                        return self.rules[&page]
                            .clone()
                            .into_iter()
                            .all(|must_go_after| !copied_row.contains(&must_go_after));
                    }
                    let subsequent_slice = copied_row.split_off(i + 1);
                    if self.rules.contains_key(&page) {
                        println!("{:?}", subsequent_slice);
                        return self.rules[&page].clone().into_iter().all(|must_go_after| {
                            subsequent_slice.contains(&must_go_after)
                                || !copied_row.contains(&must_go_after)
                        });
                    } else {
                        return true;
                    }
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
