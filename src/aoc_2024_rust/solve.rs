use pyo3::PyResult;

pub trait Solution {
    fn solve(&self) -> PyResult<i32>;
    fn solve_02(&self) -> PyResult<i32>;
}