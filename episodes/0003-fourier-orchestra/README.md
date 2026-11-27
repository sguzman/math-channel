# Example 0003 — An Orchestra of Circles

Cinematic silent short on the Fourier series of a square wave. Gold resultant, emerald/cyan/violet/pink harmonics, midnight background, synchronized motion and a native 3D spectral sculpture. Target 60–80 seconds, 1280×720, 30 fps. No permanent branding or outside assets.

## Exposition

For the 2π-periodic square wave f(t)=sign(sin t), its sine coefficients are 4/(πn) for positive odd n and zero for even n. Its cosine and constant coefficients vanish. This follows by integrating the two constant half-periods against sine and cosine. Thus the series is (4/π) Σ[k≥0] sin((2k+1)t)/(2k+1). It converges to f away from the jumps, and to zero at jumps. The Gibbs overshoot persists near jumps as the number of terms increases; convergence is not uniform. The curve shown uses actual finite partial sums (with continuously interpolated gains during transitions), never arbitrary smoothing.

Each rotating vector is a complex phasor with radius 4/(πn), angular frequency n, and the same phase parameter t. Their sum's imaginary part is the square-wave approximation. The screen y-coordinate is negated. The five-vector stage uses n=1,3,5,7,9. The 3D sculpture separates these same sinusoidal components along a categorical depth axis, and shows the sum on a separate plane. The amplitude bars are relative to the first coefficient, so widths are proportional to 1/n. The finale shows 1, 5 and 17 odd terms, with a muted ideal target whose discontinuities are not connected.

## Shot plan

1. Title and series motif; formula rises to become the heading. One circle draws one wave; four further circles grow into the chain. Shared phase, moving tip, graph and horizontal projection.
2. Matched fade to a native 3D sculpture of five harmonics and their sum; slow orbit with a color-matched relative amplitude spectrum.
3. Smooth convergence from one through five to seventeen harmonics, then a hero formula and restrained closing text.

## Narration draft (not recorded)

A complicated shape can be made from simple rotations. One circle draws a sine wave. Add smaller circles spinning at odd multiples of the first speed, and a new shape emerges. Separate the frequencies and you can see each contribution. Add more harmonics and the sum approaches a square wave, except for the characteristic overshoot near its jumps. This is the Fourier series: complexity made from harmony.

## Render

`nix develop --no-update-lock-file -c cargo run --locked --release --bin example-0003 -- render --out out/example-0003.mp4`

Dedicated workflow produces MP4, full decode verification, probe and contact sheet before layout checks. Source: [example-0003.rs](../../src/bin/example-0003.rs). Render completed; human QA pending.


## Render verification

[Successful CI run 46053096376](https://github.com/sguzman/math-channel/actions/runs/46053096376) at commit ac0786e0ac33ecbc685d105e67394670949c1064. [Playable MP4 and storyboard artifact](https://github.com/sguzman/math-channel/actions/runs/46053096376/artifacts/15224894017). Independently downloaded and decoded all 1908 frames without errors: H.264, 1280×720, 30 fps, 63.6 seconds. Actual storyboard inspected: title, linked epicycles and waveform, rotating 3D harmonic sculpture and spectrum, convergence and final equation all present. Layout checker: zero errors, two dead-time warnings at 31.00–41.10 and 51.70–56.30 seconds. Decoded 760×400 region hashes at each integer second 31–40 and 52–56 are all distinct within their respective intervals, and contact sheet shows changes in sculpture and plot; these warnings do not establish frozen footage. Sampled visual QA is not full human editorial review. Await user feedback on the actual video; no other work required unless defects are identified.
