# Book 1: [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

✅ Done

- Video explaining aperture and depth of field (defocus blur): https://www.youtube.com/watch?v=Bs9L_9iBVLQ
- Final scene took around 45 min to render on Intel Core i5 13600K

![](./out/final_scene.png)



# Book 2: [Ray Tracing: The Next Week](https://raytracing.github.io/books/RayTracingTheNextWeek.html)

## Motion blur

What a pixel really represents:

> Samples per pixel is no longer “just anti-aliasing”.
> It’s the number of Monte Carlo samples used to integrate *everything* the camera measures.

A pixel is **not** a single ray.

A pixel is an **integral over many dimensions**:

```math
\text{pixel} =
\int_{\text{pixel area}}
\int_{\text{lens}}
\int_{\text{time}}
L(x, y, \ell, t)\, dA\, d\ell\, dt
```

Each of these integrals corresponds to a physical effect:

| Dimension          | Effect             |
| ------------------ | ------------------ |
| pixel area         | anti-aliasing      |
| lens aperture      | depth of field     |
| time               | motion blur        |
| (later) wavelength | spectral rendering |

So we normalize:

```math
\text{pixel} =
\frac{1}{A_{\text{pixel}}}
\frac{1}{A_{\text{lens}}}
\frac{1}{\Delta t}
\int\!\!\int\!\!\int
L(x, y, \ell, t)\;
dA\; d\ell\; dt
```
This converts:

- **total energy → average radiance (color) **

Monte Carlo sampling is a cheap estimation of the above integral. 



## BVH

Mistakes made:

- Should choose the closest result in the left/right hits
- forgot to pass in the ray_t interval to bounding box hit check

### BVH (Bounding Volume Hierarchy)

- Build a binary tree by choosing a split axis, sorting primitives by their bounding box centers, and splitting the list in half.
- Each node stores an AABB that encloses all primitives in its subtree.
- Traversal tests the node’s AABB first; if it misses, the entire subtree is skipped.
- If the box is hit, recurse into children and tighten the `ray_t` interval so only the closest hit survives.
- The final hit returned is the nearest intersection along the ray.

Diagram (space + tree):

```
Scene space                               BVH tree
+------------------------+                Node (AABB)
|        [root AABB]     |                /         \
|   +---------+          |          Node (AABB)   Node (AABB)
|   | left    |          |            /   \          /   \
|   | child   |          |         prim prim     prim prim
|   +---------+   +---+  |
|            |   |   |   |
|            +---+   |   |
|                 right  |
|                 child  |
+------------------------+
```

Traversal idea:

```text
if ray misses root AABB -> return None
else:
  hit_left = left.hit(ray, ray_t)
  ray_t.max = hit_left.t if hit_left exists
  hit_right = right.hit(ray, ray_t)
  return closer of the two (right if it exists, else left)
```

### AABB (Axis-Aligned Bounding Box)

- Stored as three intervals: `[x_min, x_max]`, `[y_min, y_max]`, `[z_min, z_max]`.
- For each axis, compute the `t` interval where the ray is inside the slab.
- Intersect the three `t` ranges; if the intersection is empty, the ray misses.
- The test is fast and used to cull groups of objects before exact geometry checks.

Slab test sketch:

```
Ray:  P(t) = O + t*D

X slab: t in [tx0, tx1]
Y slab: t in [ty0, ty1]
Z slab: t in [tz0, tz1]

Final valid t is:
t_min = max(tx0, ty0, tz0)
t_max = min(tx1, ty1, tz1)

Hit if t_min < t_max and overlaps ray_t interval
```

## WIP

- [x] Motion Blur
- [x] BVH
- [ ] Texure Mapping
- [ ] Perlin Noise
- [ ] Quadrilaterals
- [ ] Lights
- [ ] Instances
- [ ] Volumes
- [ ] Final Scene #2
