# Renderer evaluation

Decision: dogfood Oxframe first. Manim Community is the fallback, not excluded on language preference alone.

## Evidence and limitations

- [Oxframe 0.24.3 SDK](https://github.com/oxygenframe/oxframe-sdk/releases/tag/v0.24.3), pinned for this trial; pre-1.0.
- [Oxframe mathematical use cases](https://oxframe.dev/use-cases/math), [formulas](https://oxframe.dev/docs/formulas), [Manim migration](https://oxframe.dev/docs/migrate/manim).
- [Manim reference](https://docs.manim.community/en/stable/reference.html): established mathematical objects, graphs, coordinate systems and animations.
- Oxframe's migration guide explicitly lacks native parametric surfaces and 3D axes; mesh support is not equivalent. Treat this as a potential reason to switch, not a requirement to write a renderer.
- [Ranim](https://github.com/AzurIce/ranim) and [Noon](https://github.com/yongkyuns/noon) are reserve Rust candidates; no parity or runtime claims are made.

## Acceptance criteria

| ID | Trial | Pass condition |
| --- | --- | --- |
| T01 | Nix environment and lockfiles | Nix shell evaluates; Rust version and FFmpeg verified; both lockfiles committed |
| T02 | Typst and plot smoke test | Compiles; headless MP4 with legible identity and correctly moving sine-curve point; inspect actual output |
| T03 | Formula transformation | Multi-step derivation with meaningful symbol correspondence |
| T04 | Geometry and proof | Animated construction, labels, and exact visual relationships |
| T05 | Calculus | Animated tangent/area or convergence with numerically checked data |
| T06 | Linear algebra | Matrix transformation and vectors with clear coordinate mapping |
| T07 | Complex/3D | Complex plane and surface example; record missing primitives and required custom work |
| T08 | Production | Narration timing, captions, sound mix, 1080p output, storyboard checks, rerender behavior |
| T09 | Dogfood | One coherent short mathematical explanation with actual visual and editorial QA |

Record compile/render times, scene complexity, actual bugs, workarounds, and output artifacts. Do not assume the upstream benchmarks predict local performance. If essential visual features require repeated bespoke implementation, or reliability prevents publishing, compare the same shot in Nix-managed Manim and choose based on evidence. Avoid a home-grown universal renderer abstraction; portable content and shot plans are sufficient.

First smoke scene: a trigonometric identity and a moving point on the sine graph. These are capability tests, not a script or a channel identity.
