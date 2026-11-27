# Example 0004 — The Gaussian Integral

Premium text-led production sample. 2560×1440, 60 FPS native render target, original procedural stereo sound bed, caption file, thumbnail, upload metadata and 720p preview. No permanent channel branding or voice narration. No external visual/audio assets.

Mathematics: I is positive and finite (compare tails to e^(−|x|)). Tonelli permits the product integral because the integrand is nonnegative. Then polar change of variables gives I² = 2π ∫₀∞ e^(−r²)r dr = π, and positivity gives √π. The heatmap and finite wire surface are illustrative samples of the density, not exact finite-volume integration. Annular sector visual illustrates the first-order area element r dr dθ. The no-elementary-antiderivative fact is stated, not proved.

Five scenes: opening and obstacle; square and Cartesian density; radial 3D wire sculpture; polar sector; evaluation and theorem card. Midnight palette, teal geometry, gold results, ivory prose, matched fades. Captions are provisional until the rendered timeline is measured. The thumbnail is sampled from the final theorem hold and must be visually checked. Audio uses an original low-level additive tone bed (not an externally licensed track); no narration.

Run: `nix develop --no-update-lock-file -c cargo run --locked --release --bin example-0004 -- render --out out/example-0004-visual.mp4`, then `nix develop --no-update-lock-file -c bash episodes/0004-gaussian-integral/package.sh`. CI uploads master, preview, thumbnail, captions, metadata and storyboard. Technical and human QA pending.
