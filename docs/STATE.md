# Project state

## Decisions

- Working repository name: math-channel. Final channel name and branding deferred.
- Rust, Typst, Nix flakes, Git, PCL 1.0 Draft 2.
- Oxframe 0.24.3 is the first dogfood candidate; Manim remains a valid fallback.
- Public GitHub Actions for trial builds and preview artifacts. No automated YouTube publication.

## Current checkpoint

- Bootstrap commit 10bba2da349e13906516ecaea9e6fa5d975fe680: [CI passed](https://github.com/sguzman/math-channel/actions/runs/45919333312), including dependency resolution, compilation, headless render and FFmpeg probe. Generated lockfiles from the corrected run are available; a one-time CI job is configured to persist their exact verified bytes. Original smoke visual QA remains unestablished.
- Example 0001: square-cube law, text-led five-scene demo. Separate Rust binary with Oxframe math and 3d features; CI now renders the original smoke and new episode and exports a contact sheet. [First episode CI](https://github.com/sguzman/math-channel/actions/runs/45922381331) compiled but failed the layout check before rendering: overlapping scale-factor label, safe-area warnings and static pauses. The subsequent fix reduces the chart dimensions, moves labels into the safe area, shortens pauses, and renders/decodes/uploads the MP4 before the layout check. [Corrected CI run](https://github.com/sguzman/math-channel/actions/runs/46025896423) passed compilation, render, full FFmpeg decode and layout check (zero errors, one dead-time warning at 9.80–13.70 seconds). Downloaded [video artifact](https://github.com/sguzman/math-channel/actions/runs/46025896423/artifacts/15212895179) and independently decoded the MP4: H.264, 1280×720, 30 fps, 834 frames, 27.8 s, silent. Inspected the actual contact sheet: all five scenes present, labels legible, cube visibly rotates between sampled frames, and graph endpoint matches the cubic curve. Eight sampled frames of the 3D region during the warning interval had distinct hashes, supporting that the checker misses 3D movement. This is sampled QA, not a full editorial viewing. User visual feedback pending.
- No branding or upload selected. No audio recorded.

## Next steps

1. Verify the one-time lockfile persistence job and remote blob hashes; validate locked builds.
2. Obtain user visual feedback on the example before developing production branding or a longer episode.
3. Inspect original smoke and evaluate remaining T02–T09 criteria without claiming full coverage.

No local install required for the CI trial.
