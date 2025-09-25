use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub struct UniformMotion {
    initial_position: f64,
    velocity: f64,
}

#[pymethods]
impl UniformMotion {
    #[new]
    pub fn new(initial_position: f64, velocity: f64) -> Self {
        UniformMotion {
            initial_position,
            velocity,
        }
    }

    #[getter]
    pub fn initial_position(&self) -> f64 {
        self.initial_position
    }

    #[getter]
    pub fn velocity(&self) -> f64 {
        self.velocity
    }

    pub fn position_at_time(&self, time: f64) -> f64 {
        self.initial_position + self.velocity * time
    }

    pub fn distance_traveled(&self, time: f64) -> f64 {
        self.velocity * time
    }

    pub fn __repr__(&self) -> String {
        format!(
            "UniformMotion(initial_position={}, velocity={})",
            self.initial_position, self.velocity
        )
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct UniformAcceleratedMotion {
    initial_position: f64,
    initial_velocity: f64,
    acceleration: f64,
}

#[pymethods]
impl UniformAcceleratedMotion {
    #[new]
    pub fn new(initial_position: f64, initial_velocity: f64, acceleration: f64) -> Self {
        UniformAcceleratedMotion {
            initial_position,
            initial_velocity,
            acceleration,
        }
    }

    pub fn position_at_time(&self, time: f64) -> f64 {
        self.initial_position + self.initial_velocity * time + 0.5 * self.acceleration * time * time
    }

    pub fn velocity_at_time(&self, time: f64) -> f64 {
        self.initial_velocity + self.acceleration * time
    }

    pub fn __repr__(&self) -> String {
        format!(
            "UniformAcceleratedMotion(initial_position={}, initial_velocity={}, acceleration={})",
            self.initial_position, self.initial_velocity, self.acceleration
        )
    }
}
