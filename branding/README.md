# Black Axiom — Brand standard v1.0

Approved creative direction: 2026-11-27. Digital-first revision of concept II (the graphite / white / red identity board); the user explicitly approved its black-and-red direction. This document translates that approved visual direction into operational rules. Exact typography selection, geometry, sizes and motion timings below are implementation decisions, not claims that every detail appeared in the concept image.

## 1. Identity

- Display name: **Black Axiom**. Wordmark: **BLACK AXIOM**.
- Tagline: **All else follows.** Use uppercase and wide tracking in the lockup.
- Editorial territory: mathematics, theory, proof, insight. Mathematical exposition, visualization, research-adjacent exploration and occasional deadpan mathematical satire.
- Character: prestigious, severe, elegant, intellectually curious, mysterious. Think a cinematic mathematical observatory, not an occult cult, gaming channel, or generic software company.
- Mystery comes from scale, symmetry, negative space and controlled contrast; intellectual trust comes from correct mathematics, honest uncertainty and legible exposition.
- Name selected by owner; handle availability and trademark clearance are not established. Do not claim registration, exclusivity, verification badges, or publish accounts without approval.

## 2. Color system

The five core tokens reproduce the labels on the approved digital board. Additional functional tokens are intentionally defined for contrast and motion.

| Token | Hex | Role |
|---|---|---|
| Graphite | `#0B0B0D` | Primary background |
| Bone white | `#F2F2F0` | Wordmark, prose, formulas, main geometry |
| Silver gray | `#9A9FA6` | Secondary readable text, axes |
| Iron | `#4B4F55` | Secondary geometry, dividers, not small text |
| Axiom red | `#7A0E12` | Dark crimson surfaces and decorative accents |
| Signal red | `#FF4048` | Bright point, active trace, emphasis, accessible small red text |
| Grid | `#202126` | Subtle construction guides |
| Raised graphite | `#15161A` | Secondary panel surface |

Black is the dominant visual field. White conveys the mathematics. Red means attention: one point, one curve, one reveal, one decisive result. As a composition target, 75–90% dark field, 10–25% grayscale content, red normally under 5% of frame. Not a pixel-count requirement. Do not add gold as a permanent brand color; warmer tones are allowed only when the subject requires them. Use color with shape, labels or line style, never alone. Dark Axiom red is not suitable for small text on graphite; use signal red or bone white. Treat glow as an atmosphere layer, not a substitute for crisp strokes.

## 3. The Axiom mark

The master construction is a triangle and an incircle with a central red point and axial guide lines. It is a designed mark, not a claim of mystical significance.

Canonical 256-unit artboard:
- Equilateral triangle centered horizontally, apex (128,32), base endpoints (38,187.8845727) and (218,187.8845727).
- Incircle center (128,135.9230485), radius 51.9615242; genuinely tangent to the triangle sides. The point is its center.
- Master geometry bone-white, 4-unit stroke, rounded joins; axes 1.5 units in silver; red point radius 5.5.
- Optional dotted construction circle centered on the incircle, radius 101; guides are decorative and do not claim a circumcircle. Omit all such guides at small sizes.
- Master artwork: [axiom-mark.svg](assets/axiom-mark.svg). Compact artwork: [axiom-mark-small.svg](assets/axiom-mark-small.svg). Monochrome: [axiom-mark-mono.svg](assets/axiom-mark-mono.svg).
- Keep at least 12% of mark width clear around visible artwork. No stretching, rotation of the canonical lockup, shadows that obscure geometry, glossy metallic treatment, or letters inside the incircle.
- At 32–95 px use the compact mark; at 96 px and above use the master. Below 32 px use a simple triangle/point or omit the mark. Check 32 px circular crop before export.
- Avatar: centered compact mark on graphite in square crop with generous margins. The compact mark is the source, not a final PNG export.
- Wordmark: centered uppercase Cinzel, tracking +0.18 em; do not place a stretched bitmap of the wordmark in a video. Tagline: Inter medium, +0.22 em, optional thin red rules. Logo may sit above the wordmark or to its left for horizontal lockup.

## 4. Type

Implementation defaults chosen from the alternatives on the approved concept:
- Display/wordmark: **Cinzel** regular or semibold; title case acceptable for an episode title, uppercase for wordmark and short chapter labels.
- Body, subtitles, axis labels, UI: **Inter** regular / medium / semibold. Use tabular numerals for changing counts.
- Scholarly emphasis: **EB Garamond Italic**, sparse. Do not use a fictitious Cinzel Italic font.
- Equations: Typst math with **Libertinus Math** where available; equation source remains real math, not image-generated lettering. Verify glyph coverage, font resolution and licenses before pinning. These choices are design defaults; renderer/font integration has not been performed by this specification.
- Prefer open-licensed font distributions and pin through Nix; do not copy commercial Trajan Pro files. Preserve license notices. Fallback to a readable serif/sans/math combination with explicit documentation; never silently change the approved wordmark.

At 2560×1440, recommended display 112–148 px, chapter title 76–100 px, hero formula 116–180 px depending on length, ordinary math 64–100 px, body 48–64 px, labels 32–44 px, metadata 28–36 px. Scale proportionally for alternate resolutions. Captions 52–64 px with dark backing when needed. Use sentence case for explanation and labels unless deliberate small-caps metadata. Keep mobile readability at 320 px wide; simplify rather than shrinking a dense slide.

## 5. Composition and assets

- Native 2560×1440 / 60 fps default for serious episodes; optional 3840×2160 master after successful rendering tests. Rec.709 SDR, H.264 yuv420p and faststart for delivery. Prefer actual native resolution, not upscaling presented as native.
- At 1440p use 128 px horizontal and 96 px vertical minimum safe inset; a 12-column conceptual grid and 8 px spacing unit. Give the subject at least half the visual attention. Keep axes, legends and captions clear of the proof and each other.
- Background: nearly black graphite, restrained wireframe terrain, faint square grid, arcs and construction lines. Decorative grid lines at low opacity; avoid clutter behind formulas. A single luminous red point or trace anchors the frame. No ubiquitous red fog, fake scanlines, random formulas, classical statues as default, fake scientific evidence, or constant neon.
- Geometry: crisp bone/silver strokes, approximately 2–4 px ordinary, 5–8 px featured at 1440p. Use contrast and depth to separate a foreground proof from decorative detail. 3D surfaces can use grayscale mesh, subdued material, controlled rim light and a red critical trace.
- Opening: title subject first; subtle small episode identifier and optional mark. Centered wordmark lockup may be used for channel ident. Avoid a long compulsory logo intro.
- Theorem card: label THEOREM or KEY RESULT, restrained red corner/tick, large accurate formula, optional short interpretation. If diagram and formula cannot fit, split shots.
- Closing: one clean hero result, then a brief Black Axiom lockup and tagline when appropriate. Never cover the result with a promotional overlay.
- Thumbnail: 1280×720, one dominant subject or formula, strong silhouette and readable title (prefer 2–5 words). Reserve bottom-right for platform timestamp. No invented verification badges or sensational claims.
- Channel banner: 2560×1440 design canvas; conservatively center all essential mark, wordmark and tagline inside 1546×423 centered rectangle (x=507..2053, y≈509..931). Exterior artwork may crop. Validate current platform preview before publishing. This is a conservative internal design target, not a claim of verified current platform policy.
- Social avatar: 800×800 export target; square, circular crop safe.
- Social graphics: 1080×1080 square or 1080×1920 vertical; keep critical content clear of platform overlays. Academic satire may use an intentionally contrasting cream-paper palette, as in example 0005, with branding optional. Do not pretend invented data are research findings.

## 6. Motion and sound

- Motion demonstrates the mathematics: curve construction, coordinate transformation, 3D orbit, highlighting an inference. Start and end at understandable states.
- Ident idea (not yet rendered): faint axes draw in, triangle resolves, incircle forms, red point appears last, wordmark and tagline fade in. 2–3 s total, subtle glow only. Skip or shorten for shorts.
- Micro transitions 0.25–0.5 s; conceptual scene transitions 0.8–1.4 s; intentional formula reading holds 2–5 s or longer where needed. Linear interpolation for physical/parametric time; smooth ease-in/out for camera and layout. Avoid arbitrary jumps in mathematical objects.
- Camera: steady slow orbit, restrained depth/parallax, avoid whip pans or gratuitous shake. Maintain orientation cues when changing viewpoints.
- Audio character: restrained ambient, sparse dark harmonics and subtle designed accents; clarity first. No unlicensed audio. Narration optional; no voice identity selected. Where narrated, aim for intelligible speech above the music, conservative peaks, test on speakers and headphones. Captions match actual content and timing. Do not claim audio or motion QA until tested.

## 7. Editorial voice

Precise, confident where proved, explicit where unknown. Use tension through a real mathematical question and reveal, not through clickbait or false claims. Avoid gratuitous jargon and unverified attributions. Exposition and references stay separate from scene code. Mathematical correctness overrides decorative styling. Humor may be absurd, but distinguish satire from actual discoveries.

## 8. Acceptance and maintenance

Before publishing a branded episode: check mathematical content; actual render and decoding; clipping, contrast, mobile-size legibility, safe areas, transitions and meaningful motion; audio/captions if present; asset provenance and licensing; human visual review. Store source and references, not generated MP4s, in Git. Treat this document and [tokens.json](tokens.json) as authoritative; update together with any approved changes. Preserve earlier examples as historical prototypes; no retroactive rebranding or repo rename required.

Scope of v1.0: identity and production standard, reusable SVG mark source, machine-readable tokens and sample composition SVG. No rendered ident, channel account, final banner upload, font installation or trademark clearance is claimed. The generated concept board is the visual reference; the editable SVG assets are independently constructed interpretations, not extracted vector artwork.
