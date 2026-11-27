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

Examples 0001–0003 are accepted; user said Example 0003 was good. Example 0004 rendered and packaged; sampled visual QA, caption timing and audio signal checks completed. User viewing/listening is pending.

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

- First Example 0003 [CI](https://github.com/sguzman/math-channel/actions/runs/46052481962) failed compilation: palette helper was named `mut`, a Rust reserved keyword. Renamed helper and all calls to `muted_color`; corrected compile/render pending.


## Render verification

[Successful CI run 46053096376](https://github.com/sguzman/math-channel/actions/runs/46053096376) at commit ac0786e0ac33ecbc685d105e67394670949c1064. [Playable MP4 and storyboard artifact](https://github.com/sguzman/math-channel/actions/runs/46053096376/artifacts/15224894017). Independently downloaded and decoded all 1908 frames without errors: H.264, 1280×720, 30 fps, 63.6 seconds. Actual storyboard inspected: title, linked epicycles and waveform, rotating 3D harmonic sculpture and spectrum, convergence and final equation all present. Layout checker: zero errors, two dead-time warnings at 31.00–41.10 and 51.70–56.30 seconds. Decoded 760×400 region hashes at each integer second 31–40 and 52–56 are all distinct within their respective intervals, and contact sheet shows changes in sculpture and plot; these warnings do not establish frozen footage. Sampled visual QA is not full human editorial review. Await user feedback on the actual video; no other work required unless defects are identified.

## Example 0004

Gaussian integral: five continuous scenes covering graph, square, density, native 3D radial Gaussian wire sculpture, polar area element and evaluation. Target 2560×1440 at 60 fps; original procedural stereo ambient audio, 720p preview, captions, thumbnail, upload copy. All source committed; first compile/render pending. Captions are provisional until actual scene timestamps are measured. No narration or permanent branding; no upload authorized. Prior examples preserved.


## Render verification — 2026-11-27

[Successful production CI](https://github.com/sguzman/math-channel/actions/runs/46058319141) at f418ec61141d1459c9435504c832a9e5ff27b18f. [Master and production package](https://github.com/sguzman/math-channel/actions/runs/46058319141/artifacts/15228086387). CI passed native rendering, audio mux, full master decode and layout check. Downloaded artifact; independent ffprobe confirms H.264 2560×1440 60 fps, 4901 frames, duration 81.683333 s, stereo AAC. Independently decoded 720p preview without errors. Inspected actual storyboard, one-second timeline contact sheet and thumbnail; all proof stages appear and final formula is legible. Adjusted standalone captions to sampled scene timing. Original CI ZIP contains provisional captions; use updated repository SRT until new package CI completes. Audio measurements mean −33.1 dBFS, peak −24.6 dBFS (quiet ambient bed, no clipping; subjective listening QA pending). Layout checker zero errors and one 3D dead-time warning at 34.50–45.40 s; 11 distinct decoded crop hashes from 35–45 s confirm movement. Human viewing/listening remains pending. No upload performed.

### Example 0004 visual feedback

User viewed the Gaussian integral example and described it as beautiful. Visual direction accepted. No separate listening assessment was stated; do not infer one. User requested a concept for a short cute mathematical joke video for Twitter/X. Concept only at this point: hairy ball theorem, deadpan scientific presentation with a spherical creature and an unavoidable tangent-vector zero. No joke-video production or publication authorized yet.

## Example 0005 — On the Linear Growth of Progress

User approved this separate joke episode, not the hairy ball concept. Square 1080×1080 60 fps, ~11 s, silent, one persistent academic graph. Elapsed integer years and cumulative progress both equal j, 0 through 12; labels and endpoint advance in lockstep; final exact-fit diagnostics and deadpan conclusion. Synthetic definitional data, no empirical study. Twitter-compatible H.264/yuv420p MP4 with faststart, plus source and storyboard. No posting authorized. First compilation/render/QA pending. Earlier examples preserved.

### Example 0005 render verification

[Initial CI passed](https://github.com/sguzman/math-channel/actions/runs/46062656708), zero layout errors and warnings. [Playable artifact](https://github.com/sguzman/math-channel/actions/runs/46062656708/artifacts/15229950023). Independently decoded X-ready H.264 yuv420p MP4 without errors: 1080×1080, 60 fps, 658 frames, 10.966667 s, 231804 bytes. Actual contact sheet confirms incremental years and final exact-fit conclusion. Sampled visual QA revealed diagnostics panel partly occluded the final diagonal despite the passing checker. Moved panel from y=-242 to y=30 into empty space; corrected render pending. Initial playable clip delivered. No posting performed.
