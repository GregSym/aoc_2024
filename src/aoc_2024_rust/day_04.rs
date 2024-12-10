use std::collections::{HashMap, HashSet};

use pyo3::prelude::*;

trait MaskIter {
    /// 4x4 mask on the main vector
    fn iter_masks(&self) -> impl Iterator<Item = Vec<Vec<&str>>>;
}

trait XmasEval {
    fn eval_xmas(&self) -> i32;
}

trait XmasConversion {
    fn mask_to_xmas(&self) -> HashMap<String, HashSet<Vec<(usize, usize)>>>;
    fn iter_paths(&self) -> impl Iterator<Item = Vec<(usize, usize)>>;
    fn str_from_iter_pair(&self, pairs: impl Iterator<Item = (usize, usize)>) -> String;
}

impl MaskIter for Vec<Vec<&str>> {
    fn iter_masks(&self) -> impl Iterator<Item = Vec<Vec<&str>>> {
        let row1 = self.clone().split_off(1);
        let row2 = self.clone().split_off(2);
        let row3 = self.clone().split_off(3);
        let long_rows = self
            .clone()
            .into_iter()
            .zip(row1)
            .zip(row2)
            .zip(row3)
            .map(|(((a, b), c), d)| vec![a, b.clone(), c.clone(), d.clone()]);
        long_rows.flat_map(|horizontal_slice| {
            let mut collect_masks: Vec<Vec<Vec<&str>>> = vec![];
            for i in 0..(self.len() - 3) {
                let mut vertical_slice: Vec<Vec<&str>> = vec![];
                for row in horizontal_slice.clone().into_iter() {
                    vertical_slice.push(row[i..(i + 4)].to_vec());
                }
                collect_masks.push(vertical_slice);
            }
            collect_masks
        })
    }
}

impl XmasConversion for Vec<Vec<&str>> {
    fn str_from_iter_pair(&self, pairs: impl Iterator<Item = (usize, usize)>) -> String {
        let mut word = String::from("");
        for (i, j) in pairs {
            word.push_str(self[i][j])
        }
        word
    }

    fn iter_paths(&self) -> impl Iterator<Item = Vec<(usize, usize)>> {
        let mut iter_paths_collected: Vec<Vec<(usize, usize)>> = vec![];
        for i in 0..4 {
            iter_paths_collected.push(vec![i, i, i, i].into_iter().zip(0..4).collect());
            iter_paths_collected.push(vec![i, i, i, i].into_iter().zip((0..4).rev()).collect());
        }
        // handle the verticals
        for i in 0..4 {
            iter_paths_collected.push((0..4).zip(vec![i, i, i, i].into_iter()).collect());
            iter_paths_collected.push((0..4).rev().zip(vec![i, i, i, i].into_iter()).collect());
        }
        iter_paths_collected.push((0..4).zip(0..4).collect());
        iter_paths_collected.push((0..4).rev().zip((0..4).rev()).collect());
        iter_paths_collected.push((0..4).zip((0..4).rev()).collect());
        iter_paths_collected.push(((0..4).rev()).zip(0..4).collect());
        iter_paths_collected.into_iter()
    }

    fn mask_to_xmas(&self) -> HashMap<String, HashSet<Vec<(usize, usize)>>> {
        let mut collect_concats: HashMap<String, HashSet<Vec<(usize, usize)>>> = HashMap::new();
        for paths in self.iter_paths() {
            let paths_copy = paths.clone();
            let word = self.str_from_iter_pair(paths.into_iter());
            if !collect_concats.contains_key(&word) {
                collect_concats.insert(word, HashSet::new());
            }
            collect_concats[word].to_owned().insert(paths_copy);
        }
        collect_concats
    }
}

impl XmasEval for Vec<Vec<&str>> {
    fn eval_xmas(&self) -> i32 {
        // println!("{:?}", self);
        let mut local_tally = 0;
        println!("{:?}", self);
        self.mask_to_xmas()
            .into_iter()
            .map(|test| {
                println!("{:?}", test);
                if test == "XMAS" {
                    println!("{:?}", self);
                    local_tally += 1;
                    println!("{:?}", local_tally);
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

#[pyfunction]
pub fn solve_day_04_pt_01(input: String) -> PyResult<i32> {
    let grid: Vec<Vec<&str>> = input
        .split_terminator("\n")
        .filter(|row| *row != "")
        .map(|row| row.split_terminator("").filter(|ch| *ch != "").collect())
        .collect();
    let mut tally = 0;
    for mask in grid.iter_masks() {
        tally += mask.eval_xmas();
    }
    Ok(tally)
}
