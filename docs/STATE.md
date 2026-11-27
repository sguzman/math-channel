# Project state

## Decisions

- Working repository name: math-channel. Final channel name and branding deferred.
- Rust, Typst, Nix flakes, Git, PCL 1.0 Draft 2.
- Oxframe 0.24.3 is the first dogfood candidate; Manim remains a valid fallback.
- Public GitHub Actions for trial builds and preview artifacts. No automated YouTube publication.

## Current checkpoint

- Bootstrap commit 10bba2da349e13906516ecaea9e6fa5d975fe680: [CI passed](https://github.com/sguzman/math-channel/actions/runs/45919333312), including dependency resolution, compilation, headless render and FFmpeg probe. Generated lockfiles were uploaded but are not yet committed; visual smoke QA is not established.
- Example 0001: square-cube law, text-led five-scene demo. Separate Rust binary with Oxframe math and 3d features; CI now renders the original smoke and new episode and exports a contact sheet. New code compilation, rendering and visual QA remain unverified at this checkpoint.
- No branding or upload selected. No audio recorded.

## Next steps

1. Inspect the episode CI logs and fix actual failures, if any.
2. Retrieve and commit exact lockfiles, then validate locked builds.
3. Inspect the episode MP4/contact sheet and original smoke; record actual evidence and evaluate relevant T02–T09 criteria without claiming full coverage.
4. Obtain user visual feedback on the example before developing production branding or a longer episode.

No local install required for the CI trial.
