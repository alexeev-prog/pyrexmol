pub mod math;

use math::vector2::Vector2;
use pyo3::prelude::*;

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Vector2>()?;
    Ok(())
}
