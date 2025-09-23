use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Copy)]
pub struct Vector2 {
    x: f64,
    y: f64,
}

#[pymethods]
impl Vector2 {
    #[new]
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }

    #[getter]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[getter]
    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn __add__(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn __sub__(&self, other: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn __mul__(&self, scalar: f64) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn __rmul__(&self, scalar: f64) -> Vector2 {
        self.__mul__(scalar)
    }

    pub fn __truediv__(&self, scalar: f64) -> PyResult<Vector2> {
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

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dot(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn normalize(&self) -> PyResult<Vector2> {
        let mag = self.magnitude();
        if mag == 0.0 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Cannot normalize zero vector",
            ));
        }
        Ok(Vector2 {
            x: self.x / mag,
            y: self.y / mag,
        })
    }

    pub fn __repr__(&self) -> String {
        format!("Vector2({}, {})", self.x, self.y)
    }

    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}
