use pyo3::prelude::*;

trait MaskIter {
    /// 4x4 mask on the main vector
    fn iter_masks(&self) -> impl Iterator<Item = Vec<Vec<&str>>>;
}

trait XmasEval {
    fn eval_xmas(&self) -> bool;
}

trait XmasConversion {
    fn mask_to_xmas(&self) -> Vec<String>;
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
        long_rows
            .flat_map(|horizontal_slice| {
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
    fn mask_to_xmas(&self) -> Vec<String> {
        let mut collect_concats: Vec<String> = vec![];
        // handle the horizontals
        for i in 0..4 {
            collect_concats.push(self.str_from_iter_pair(vec![i, i, i, i].into_iter().zip(0..4)));
            collect_concats
                .push(self.str_from_iter_pair(vec![i, i, i, i].into_iter().zip((0..4).rev())));
        }
        // handle the verticals
        for i in 0..4 {
            collect_concats.push(self.str_from_iter_pair((0..4).zip(vec![i, i, i, i].into_iter())));
            collect_concats
                .push(self.str_from_iter_pair(((0..4).rev()).zip(vec![i, i, i, i].into_iter())));
        }
        // handle diag
        collect_concats.push(self.str_from_iter_pair((0..4).zip(0..4)));
        collect_concats.push(self.str_from_iter_pair((0..4).rev().zip((0..4).rev())));
        collect_concats.push(self.str_from_iter_pair((0..4).zip((0..4).rev())));
        collect_concats.push(self.str_from_iter_pair(((0..4).rev()).zip(0..4)));
        collect_concats
    }
}

impl XmasEval for Vec<Vec<&str>> {
    fn eval_xmas(&self) -> bool {
        println!("{:?}", self);
        self.mask_to_xmas().into_iter().any(|test| {
            println!("{:?}", test);
            test == "XMAS"
        })
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
        if mask.eval_xmas() {
            tally += 1;
        }
    }
    Ok(tally)
}
