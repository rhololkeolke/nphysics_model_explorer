# MJCF Parser

Goal is to be able to parse a Mujoco XML file and create an equivalent
nphysics3d simulation.

MJCF XML Reference: http://www.mujoco.org/book/XMLreference.html
nphysics project: https://www.nphysics.org/

# Current Status

- Some basic sanity checks that the XML document has the right tags.
- Can parse some basic static geoms attached to the worldbody
  - Not every property in the MJCF file maps to an nphysics property,
    for now these additional properties are stored in user_data
