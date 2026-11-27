# Mathematical exposition

For real t, Euler's formula states that exp(it) = cos(t) + i sin(t). One proof is to expand the absolutely convergent power series of the complex exponential and separate even and odd powers using i² = −1. The even powers give cosine, the odd powers give i times sine. Consequently |exp(it)| = 1, and its argument is t modulo 2π.

The real and imaginary components are cos(t) and sin(t), so a point moving around the unit circle has two sinusoidal coordinate projections. The visual uses y-down screen coordinates, hence screen y = −r sin(t). The two plots share the angle signal with the circle; at each frame the traces and markers represent the same t.

The space curve (cos t, sin t, t) is a helix. The 3D stage uses Y as the vertical time axis and maps the real and imaginary components to X and Z, with uniform radial scaling and an affine time scaling to fit the frame. The two projected traces are offset to parallel planes for clarity. Two complete turns are shown. The helix is rendered from short cylinder chords, not a built-in analytic surface.

At t = π, cos π = −1 and sin π = 0, so exp(iπ) = −1 and exp(iπ) + 1 = 0. The animation illustrates the identity; the power-series argument is the mathematical justification.
