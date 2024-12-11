use pyo3::prelude::*;
use std::collections::HashSet;

const TARGET: &str = "XMAS";
const TARGET_MAS: &str = "MAS";

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

fn paths_x() -> Vec<Vec<((usize, usize), (usize, usize))>> {
    let inc = 0..TARGET_MAS.len();
    let dec = (0..TARGET_MAS.len()).rev();
    let mut path_collection: Vec<Vec<((usize, usize), (usize, usize))>> = vec![];
    // diagonal
    let right_down = inc.clone().zip(inc.clone());
    let right_up = dec.clone().zip(inc.clone());
    let left_up = dec.clone().zip(dec.clone());
    let left_down = inc.clone().zip(dec.clone());
    path_collection.push(right_down.clone().zip(right_up.clone()).collect());
    path_collection.push(left_down.clone().zip(left_up.clone()).collect());
    path_collection.push(right_down.clone().zip(left_down.clone()).collect());
    path_collection.push(right_up.clone().zip(left_up.clone()).collect());
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

fn coords_x(x: usize, y: usize) -> Vec<Vec<((usize, usize), (usize, usize))>> {
    paths_x()
        .into_iter()
        .map(|path_combo| {
            path_combo
                .into_iter()
                .map(|((x1, y1), (x2, y2))| ((x + x1, y + y1), (x + x2, y + y2)))
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

trait XmasGrid2 {
    fn double_words_at(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (Vec<((usize, usize), (usize, usize))>, (String, String))>;
    fn target_words_mas(
        &self,
    ) -> HashSet<(Vec<((usize, usize), (usize, usize))>, (String, String))>;
    fn mas_count(&self) -> usize;
}

impl XmasGrid2 for Vec<Vec<&str>> {
    fn double_words_at(
        &self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (Vec<((usize, usize), (usize, usize))>, (String, String))> {
        coords_x(x, y).into_iter().map(|path_combo| {
            let mut some_word_0 = String::from("");
            let mut some_word_1 = String::from("");
            path_combo
                .clone()
                .into_iter()
                .for_each(|((x0, y0), (x1, y1))| {
                    if self.len() > y0 && self[0].len() > x0 {
                        some_word_0 += self[y0][x0];
                    }
                    if self.len() > y1 && self[0].len() > x1 {
                        some_word_1 += self[y1][x1];
                    }
                });
            (path_combo, (some_word_0, some_word_1))
        })
    }

    fn target_words_mas(
        &self,
    ) -> HashSet<(Vec<((usize, usize), (usize, usize))>, (String, String))> {
        let mut word_collection: HashSet<(Vec<((usize, usize), (usize, usize))>, (String, String))> = HashSet::new();
        self.into_iter().enumerate().for_each(|(y, row)| {
            row.into_iter().enumerate().for_each(|(x, _)| {
                self.double_words_at(x, y)
                    .for_each(|(path_combo, (word_0, word_1))| {
                        println!("{}, {}", word_0, word_1);
                        if word_0 == TARGET_MAS && word_1 == TARGET_MAS {
                            word_collection.insert((path_combo, (word_0, word_1)));
                        }
                    });
            });
        });
        word_collection
    }

    fn mas_count(&self) -> usize {
        self.target_words_mas().len()
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

#[pyfunction]
pub fn solve_day_04_pt_02(input: String) -> PyResult<usize> {
    let grid: Vec<Vec<&str>> = input
        .split_terminator("\n")
        .filter(|row| *row != "")
        .map(|row| row.split_terminator("").filter(|ch| *ch != "").collect())
        .collect();
    Ok(grid.mas_count())
}
