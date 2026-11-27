# Example 0002 — The Circle Hidden in Exponentials

A cinematic, silent visual prototype. Dark midnight, ivory, gold, cyan (real) and violet (imaginary). No permanent channel branding. Target approximately 45–60 seconds, 1280×720 at 30 fps; actual duration to be measured from render.

- [Exposition](exposition.md)
- [Narration draft](script.md)
- [Shot plan](shot-plan.md)
- [Rust scene source](../../src/bin/example-0002.rs)
- [Render workflow](../../.github/workflows/example-0002.yml)

```sh
mkdir -p out
nix develop --no-update-lock-file -c cargo run --locked --release --bin example-0002 -- render --out out/example-0002.mp4
```

The workflow publishes a decoded MP4, ffprobe metadata and a storyboard before running the layout gate, so a diagnostic video is available even if the gate finds problems. Compile/render/visual validation is pending. No external footage, music or artwork; all graphics are programmatic. The earlier episode is preserved.
