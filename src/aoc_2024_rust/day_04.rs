use pyo3::prelude::*;
use std::collections::HashSet;

const TARGET: &str = "XMAS";

fn paths() -> Vec<Vec<(usize, usize)>> {
    let inc = 0..TARGET.len();
    let dec = (0..TARGET.len()).rev();
    let mut path_collection: Vec<Vec<(usize, usize)>> = vec![];
    // horizontal
    for i in inc.clone() {
        path_collection.push(vec![i, i, i, i].into_iter().zip(inc.clone()).collect());
        path_collection.push(vec![i, i, i, i].into_iter().zip(dec.clone()).collect());
    }
    // vertical
    for i in inc.clone() {
        path_collection.push(inc.clone().zip(vec![i, i, i, i].into_iter()).collect());
        path_collection.push(dec.clone().zip(vec![i, i, i, i].into_iter()).collect());
    }
    // diagonal
    path_collection.push(inc.clone().zip(inc.clone()).collect());
    path_collection.push(dec.clone().zip(dec.clone()).collect());
    path_collection.push(inc.clone().zip(dec.clone()).collect());
    path_collection.push(dec.clone().zip(inc.clone()).collect());
    path_collection
}

fn coords(x: usize, y: usize) -> Vec<Vec<(usize, usize)>> {
    paths()
        .into_iter()
        .map(|path| {
            path.into_iter()
                .map(|(x_offset, y_offset)| (x + x_offset, y + y_offset))
                .collect()
        })
        .collect()
}

trait XmasGrid {
    fn words_at(&self, x: usize, y: usize) -> impl Iterator<Item = (Vec<(usize, usize)>, String)>;
    fn target_words(&self) -> HashSet<(Vec<(usize, usize)>, String)>;
    fn xmas_count(&self) -> usize;
}

impl XmasGrid for Vec<Vec<&str>> {
    fn words_at(&self, x: usize, y: usize) -> impl Iterator<Item = (Vec<(usize, usize)>, String)> {
        coords(x, y).into_iter().map(|path| {
            let mut some_word = String::from("");
            path.clone().into_iter().for_each(|(x, y)| {
                if self.len() > y && self[0].len() > x {
                    some_word += self[y][x];
                }
            });
            (path, some_word)
        })
    }

    fn target_words(&self) -> HashSet<(Vec<(usize, usize)>, String)> {
        let mut word_collection = HashSet::new();
        self.into_iter().enumerate().for_each(|(y, row)| {
            row.into_iter().enumerate().for_each(|(x, _)| {
                self.words_at(x, y).for_each(|(path, word)| {
                    if word == TARGET {
                        word_collection.insert((path, word));
                    }
                })
            })
        });
        word_collection
    }

    fn xmas_count(&self) -> usize {
        self.target_words().len()
    }
}

#[pyfunction]
pub fn solve_day_04_pt_01(input: String) -> PyResult<usize> {
    let grid: Vec<Vec<&str>> = input
        .split_terminator("\n")
        .filter(|row| *row != "")
        .map(|row| row.split_terminator("").filter(|ch| *ch != "").collect())
        .collect();
    Ok(grid.xmas_count())
}
