pub mod math;
pub mod physics;

use pyo3::prelude::*;

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let math_module = PyModule::new_bound(m.py(), "math")?;
    math_module.add_class::<math::core::Vector2>()?;
    m.add_submodule(&math_module)?;

    let physics_module = PyModule::new_bound(m.py(), "physics")?;
    physics_module.add_class::<physics::kinematics::UniformMotion>()?;
    physics_module.add_class::<physics::kinematics::UniformAcceleratedMotion>()?;
    m.add_submodule(&physics_module)?;

    Ok(())
}
