# Project instructions

- Read docs/STATE.md and docs/ENGINE-TRIAL.md before working.
- The approved channel name is Black Axiom. Read branding/README.md and branding/tokens.json before new visual work. Preserve the approved digital black/red identity; do not change branding or publish videos without approval.
- Nix flakes are mandatory. No global installs, npm, ad hoc downloaded binaries or Python unless explicitly adopting a Nix-managed Manim fallback.
- Prefer Rust and Typst. Keep research, narration, references and shot specifications separate from renderer code.
- Use GitHub Actions for build and preview checks where practical; preserve caches and avoid requiring repeated local downloads.
- Keep PCL Draft 2 text unchanged apart from the copyright placeholder already filled; preserve third-party notices.
- Never claim builds or visual reviews passed without evidence. Record exact commit, run links, artifact names and remaining blockers in docs/STATE.md.
- Continue the next unfinished acceptance criterion. Do not expand scope into building a custom engine.
- Keep generated video and intermediates out of Git; preserve original source assets and provenance.
