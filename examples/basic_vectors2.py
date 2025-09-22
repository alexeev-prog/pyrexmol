from pyrexmol import Vector2

v1 = Vector2(3.0, 4.0)
v2 = Vector2(1.0, 2.0)

print(f"v1 = {v1}")
print(f"v2 = {v2}")

print(f"v1 + v2 = {v1 + v2}")
print(f"v1 - v2 = {v1 - v2}")
print(f"v1 * 2.0 = {v1 * 2.0}")
print(f"v1 / 2.0 = {v1 / 2.0}")

print(f"Magnitude of v1: {v1.magnitude()}")
print(f"Dot product v1·v2: {v1.dot(v2)}")
