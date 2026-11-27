# Project state

## Decisions

- Working repository name: math-channel. Final channel name and branding deferred.
- Rust, Typst, Nix flakes, Git, PCL 1.0 Draft 2.
- Oxframe 0.24.3 is the first dogfood candidate; Manim remains a valid fallback.
- Public GitHub Actions for trial builds and preview artifacts. No automated YouTube publication.

## Current checkpoint

Bootstrap source and environment written. Initial CI must establish actual compilation/render results; no successful build or visual review has been established at this checkpoint. Exact lockfiles are not yet present. The first CI run is intended to produce them and a preview.

## Next steps

1. Inspect initial CI logs and fix any environment, SDK API or workflow issues.
2. Retrieve and commit the exact generated Cargo.lock and flake.lock, then validate with locked builds.
3. Inspect preview frames/video before marking T02 passed.
4. Continue T03–T09 in ENGINE-TRIAL.md, then make a renderer decision from evidence.

No human installation, branding or video upload is required for this bootstrap.
