# Example 0001 — Twice as big, eight times the volume

An experimental, text-led visual explanation of the square-cube law. The title is an episode title only. No channel branding, recorded narration, music or external assets.

## Mathematical exposition

For a positive scale factor s, a similarity transformation multiplies every length by s. Area scales by s², volume by s³, and d-dimensional measure scales by sᵈ for a measurable subset of d-dimensional Euclidean space. For a unit cube doubled along all three axes, eight unit cubes exactly fill the new cube. Gaps in the exploded view are visual separation only, not part of the volume calculation.

The plot uses normalized measures for unit reference objects: L=s, A=s², V=s³, for 0 ≤ s ≤ 3 (zero is the limiting degenerate case). At s=2, these are 2, 4, 8; at s=3, 3, 9, 27.

## Shot plan

| Shot | Visual / text | Capability |
| --- | --- | --- |
| Hook | Twice as big. Twice as much? | Animated typography |
| Area | One square versus four adjacent unit squares | 2D shapes, sequencing, Typst |
| Volume | Eight unit cubes separated to reveal the partition; camera and object rotate | Real 3D meshes, scene camera, material, 2D formula overlay |
| Powers | Linear, square and cubic functions with moving marker | Animated plot, color-coded labels and math |
| Conclusion | General sᵈ scaling law | Typst glyph animation |

## Optional future narration

Double the length of a square and you don't get twice the area. You get four times the area: two rows of two squares. Now add a third dimension. Two cubes across, two high and two deep make eight. As the scale factor grows, the gap between length, area and volume grows too. In d dimensions, scaling every length by s multiplies the measure by s to the power d.

Narration is a draft for possible later recording; the current episode intentionally communicates with on-screen text alone.

## Rendering and QA

Run `nix develop -c cargo run --release --bin example-0001 -- render --out out/example-0001.mp4`. CI checks and renders a 1280×720, 30 fps MP4, records ffprobe metadata and creates a contact sheet. Visual inspection is still required: text legibility and boundaries, eight-cube geometry, plot correspondence and shot pacing. The scene code lives at [src/bin/example-0001.rs](../../src/bin/example-0001.rs). No finished video is committed to Git.

## Rendered preview

[Download the actual MP4 and storyboard archive](https://github.com/sguzman/math-channel/actions/runs/46025896423/artifacts/15212895179) · [Successful CI run](https://github.com/sguzman/math-channel/actions/runs/46025896423).

Render from commit `6986e451b9420179beb96d21de4b8bb95a265e75`: H.264, 1280×720, 30 fps, 834 frames, 27.8 seconds. Independent FFmpeg decoding completed without errors. The actual contact sheet was inspected. Oxframe reported zero errors and one dead-time warning during the 3D rotation; sampled frames of that region are distinct. User editorial feedback remains pending. The artifact is subject to GitHub Actions retention; the source remains in Git.

## Human visual QA — 2026-11-27

Overall visual quality received positive feedback from the channel owner after watching the rendered MP4. Scene transitions were too abrupt. Preserve the current visual direction; refine transitions in the next revision. No permanent branding or audio decision yet.
