use pyo3::prelude::*;

#[pyclass]
struct Vector2 {
    x: f64,
    y: f64,
}

#[pymethods]
impl Vector2 {
    #[new]
    fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }

    fn __add__(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn __sub__(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn __mul__(&self, scalar: f64) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    fn __truediv__(&self, scalar: f64) -> PyResult<Vector2> {
        if scalar == 0.0 {
            return Err(pyo3::exceptions::PyZeroDivisionError::new_err(
                "Division by zero",
            ));
        }
        Ok(Vector2 {
            x: self.x / scalar,
            y: self.y / scalar,
        })
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn dot(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn __repr__(&self) -> String {
        format!("Vector2({}, {})", self.x, self.y)
    }
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Vector2>()?;
    Ok(())
}
