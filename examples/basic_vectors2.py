from pyrexmol.math import Vector2
from pyrexmol.physics.kinematics import UniformAcceleratedMotion, UniformMotion

v1 = Vector2(3.0, 4.0)
print(f"Vector: {v1}, Magnitude: {v1.magnitude()}")

motion1 = UniformMotion(initial_position=10.0, velocity=5.0)
print(f"Position after 2s: {motion1.position_at_time(2.0)}")

motion2 = UniformAcceleratedMotion(
    initial_position=0.0, initial_velocity=10.0, acceleration=-9.5
)
print(f"Velocity after 1s: {motion2.velocity_at_time(1.0)}")
