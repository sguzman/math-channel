# Math channel

A public production repository for mathematical explanations, experiments, and animated videos. The channel name and visual identity are intentionally undecided.

## Status

Oxframe is the first engine under evaluation, not an irreversible commitment. No episode has been published from this repository yet. See [project state](docs/STATE.md) and the [engine trial](docs/ENGINE-TRIAL.md).

## Development

The environment is defined by [flake.nix](flake.nix). Use Nix for all project tooling; no global Cargo installation, npm, or Python is required for this trial.

```sh
nix develop
cargo run --release -- check
cargo run --release -- render --out out/trial.mp4
```

The first build downloads and compiles dependencies. Reuse the Nix store and Cargo cache between runs. The initial setup is not considered reproducibly pinned until both flake.lock and Cargo.lock have been generated, checked and committed. CI attempts the first build and publishes the lockfiles and preview as artifacts for review. Do not delete caches to troubleshoot routine errors.

The original smoke animation remains a technical test. A separate [example episode](episodes/0001-square-cube/README.md) exercises 2D graphics, true 3D meshes, plots and Typst typography. Render it with:

```sh
nix develop -c cargo run --release --bin example-0001 -- render --out out/example-0001.mp4
```

The example is a silent, text-led visual prototype; no permanent branding or publication is implied.

## Organization

- `src/`: renderer-specific scene code.
- `episodes/`: renderer-independent exposition, narration, shot plans, references and mathematical data.
- `docs/`: production decisions, engine acceptance criteria, current state.
- `out/`: generated previews, reports and videos; ignored by Git.

## License

Original source and creative materials explicitly included in this repository are released under the [Permissive Commercial License, version 1.0 — Draft 2](LICENSE). This is a custom source-available license, not an OSI-approved open-source license. Third-party dependencies and assets retain their own licenses. No trademark rights or rights in third-party media are granted. The [canonical license project](https://github.com/sguzman/permissive-commercial-license) documents the license's draft and review status.
