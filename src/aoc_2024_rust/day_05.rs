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
                        // println!("{:?}", subsequent_slice);
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

    fn incorrectly_ordered_pages(&self) -> Vec<Vec<i32>> {
        self.pages
            .clone()
            .into_iter()
            .filter(|row| !self.correctly_ordered_pages().contains(row))
            .collect()
    }

    fn indeces_to_swap(&self) -> Vec<Vec<(usize, Vec<usize>)>> {
        self.incorrectly_ordered_pages()
            // .clone()
            .into_iter()
            .map(|row| {
                row.clone()
                    .into_iter()
                    .enumerate()
                    .map(|(i, page)| {
                        let mut copied_row = row.clone();
                        if i == row.len() && self.rules.contains_key(&page) {
                            if !self.rules[&page]
                                .clone()
                                .into_iter()
                                .all(|must_go_after| !copied_row.contains(&must_go_after))
                            {
                                return (
                                    i,
                                    self.rules
                                        .get_mut(&page)
                                        .unwrap()
                                        .into_iter()
                                        .map(|after| row.into_iter().position(after).unwrap()),
                                );
                            } else {
                                return (55000, vec![55000]);
                            }
                        }
                        let subsequent_slice = copied_row.split_off(i + 1);
                        if self.rules.contains_key(&page) {
                            // println!("{:?}", subsequent_slice);
                            if !self.rules[&page].clone().into_iter().all(|must_go_after| {
                                subsequent_slice.contains(&must_go_after)
                                    || !copied_row.contains(&must_go_after)
                            }) {
                                return (
                                    i,
                                    self.rules.get_mut(&page).unwrap().into_iter().map(|after| {
                                        subsequent_slice.into_iter().position(after).unwrap() + i
                                    }).collect()
                                );
                            } else {
                                return (55000, vec![55000]);
                            }
                        } else {
                            return (55000, vec![55000]);
                        }
                    })
                    .filter(|indeces| (*indeces).0 != 55000)
                    .collect()
            })
            .filter(|index_row: &Vec<(usize, usize)>| index_row.len() != 0)
            .collect()
    }

    fn corrected_pages(&self) -> Vec<Vec<i32>> {
        println!("{:?}", self.incorrectly_ordered_pages());
        println!("{:?}", self.indeces_to_swap());
        self.incorrectly_ordered_pages()
            .into_iter()
            .zip(self.indeces_to_swap())
            .map(|(row, left_right)| {
                let mut rearranged = row;
                if left_right.len() > 1 {
                    println!("{:?}", rearranged);
                    println!("{:?}", left_right);
                    for pair in left_right {
                        for after_position in pair.1 {
                            rearranged.swap(pair.0, after_position);
                        }
                    }
                }
                rearranged
            })
            .collect()
    }

    fn middles(&self) -> Vec<i32> {
        self.correctly_ordered_pages()
            .into_iter()
            .map(|row| row[usize::from(row.len() / 2)])
            .collect()
    }

    fn middles_corrected(&self) -> Vec<i32> {
        self.correctly_ordered_pages()
            .into_iter()
            .chain(self.corrected_pages().into_iter())
            .map(|row| row[usize::from(row.len() / 2)])
            .collect()
    }
}

impl Solution for PageOrdering {
    fn solve(&self) -> PyResult<i32> {
        Ok(self.middles().into_iter().sum())
    }

    fn solve_02(&self) -> PyResult<i32> {
        Ok(self.middles_corrected().into_iter().sum())
    }
}

#[pyfunction]
pub fn solve_day_05_pt_01(input: String) -> PyResult<i32> {
    return PageOrdering::new(input).solve();
}

#[pyfunction]
pub fn solve_day_05_pt_02(input: String) -> PyResult<i32> {
    return PageOrdering::new(input).solve_02();
}
