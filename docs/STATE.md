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

Examples 0001 and 0002 are complete. Example 0003 approved: render, inspect, fix actual defects, deliver playable MP4 and record user QA.

No local install required for the CI trial.

## Example 0002

The Circle Hidden in Exponentials: cinematic Euler-formula episode, one continuous circle/wave scene with shared angle, native mesh helix with projection traces, matched crossfade and half-turn identity finale. Initial [CI run](https://github.com/sguzman/math-channel/actions/runs/46048654670) compiled, rendered, fully decoded and uploaded [the playable draft](https://github.com/sguzman/math-channel/actions/runs/46048654670/artifacts/15222829935). Independently decoded all 1684 frames, H.264 1280×720 at 30 fps, 56.133 s. Actual storyboard inspected: title, unit circle, synchronized traces, 3D helix/projections and identity finale all present. Checker errors were three oversized decorative concentric rings intentionally cropped by frame; changed them to mathematically clipped arcs within safe margins. The sole dead-time warning at 30.70–40.10 s is contradicted by eight distinct decoded frame hashes of the 3D region at one-second intervals (31–38 s) and visible motion in storyboard. Corrected render/check pending; draft delivered as a preview, not final QA. See [episode notes](../episodes/0002-euler-circle/README.md).

### Example 0002 acceptance — 2026-11-27

- User watched the playable initial render and approved the overall visual quality, smooth movement and transitions.
- Corrected render at 22611eb5c39f688259eeaefc8e4dbc63339f6571 passed [all CI steps](https://github.com/sguzman/math-channel/actions/runs/46049091156), including locked build, render, full decode and layout check: zero errors, one previously investigated 3D dead-time warning. [Final video and storyboard](https://github.com/sguzman/math-channel/actions/runs/46049091156/artifacts/15222904344).
- Only decorative background arc clipping changed between the approved preview and corrected render; corrected render is CI-verified, not separately viewed by the user. The earlier preview supplied the human visual QA.
- Example 0002 accepted; Oxframe remains the primary renderer. Preserve both examples. No further work on this episode required absent new direction. Narration and channel branding remain deferred.

## Example 0003

An Orchestra of Circles: Fourier **series** of a square wave (not the continuous Fourier transform). Continuous linked epicycles, wave trace, native 3D separated harmonic sculpture, amplitude spectrum and convergence from 1 to 17 odd harmonics. Source and dedicated locked CI configured; compilation, render and actual QA pending. Silent prototype; previous episodes preserved.
