# Project state

## Decisions

- Working repository name: math-channel. Final channel name and branding deferred.
- Rust, Typst, Nix flakes, Git, PCL 1.0 Draft 2.
- Oxframe 0.24.3 is the first dogfood candidate; Manim remains a valid fallback.
- Public GitHub Actions for trial builds and preview artifacts. No automated YouTube publication.

## Current checkpoint

- Bootstrap commit 10bba2da349e13906516ecaea9e6fa5d975fe680: [CI passed](https://github.com/sguzman/math-channel/actions/runs/45919333312), including dependency resolution, compilation, headless render and FFmpeg probe. Exact Cargo.lock and flake.lock now committed in 785e63a73819f1114484c0ea6a16f6c9a7cac2e8; [persistence CI passed](https://github.com/sguzman/math-channel/actions/runs/46040311193). Original smoke visual QA remains unestablished.
- Example 0001: square-cube law, text-led five-scene demo. Separate Rust binary with Oxframe math and 3d features; CI now renders the original smoke and new episode and exports a contact sheet. [First episode CI](https://github.com/sguzman/math-channel/actions/runs/45922381331) compiled but failed the layout check before rendering: overlapping scale-factor label, safe-area warnings and static pauses. The subsequent fix reduces the chart dimensions, moves labels into the safe area, shortens pauses, and renders/decodes/uploads the MP4 before the layout check. [Corrected CI run](https://github.com/sguzman/math-channel/actions/runs/46025896423) passed compilation, render, full FFmpeg decode and layout check (zero errors, one dead-time warning at 9.80–13.70 seconds). Downloaded [video artifact](https://github.com/sguzman/math-channel/actions/runs/46025896423/artifacts/15212895179) and independently decoded the MP4: H.264, 1280×720, 30 fps, 834 frames, 27.8 s, silent. Inspected the actual contact sheet: all five scenes present, labels legible, cube visibly rotates between sampled frames, and graph endpoint matches the cubic curve. Eight sampled frames of the 3D region during the warning interval had distinct hashes, supporting that the checker misses 3D movement. This is sampled QA, not a full editorial viewing. User viewed the actual MP4 on 2026-11-27 and liked the overall visual quality; scene transitions were too abrupt. Keep Oxframe as the preferred dogfood engine. Next visual refinement: smoother, intentional transitions between scenes, without sacrificing readability.
- No branding or upload selected. No audio recorded.

## Next steps

1. Example 0002 is approved; build/render/inspect its actual MP4, fix blocking defects, and deliver the artifact. Keep Example 0001 unchanged as visual baseline.
2. Obtain human QA only after a verified playable episode is available. Do not expand into branding or voice production without approval.

No local install required for the CI trial.

## Example 0002

The Circle Hidden in Exponentials: cinematic Euler-formula episode, one continuous circle/wave scene with shared angle, native mesh helix with projection traces, matched crossfade and half-turn identity finale. Source and locked CI configured; compilation/render/visual QA pending. See [episode notes](../episodes/0002-euler-circle/README.md).
