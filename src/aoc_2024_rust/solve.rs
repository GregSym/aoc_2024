use pyo3::PyResult;

pub trait Solution {
    fn solve(&self) -> PyResult<i32>;
}