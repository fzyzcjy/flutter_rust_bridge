# Online Demo Gallery Smoke Test

## Purpose

Verify that the published flutter_rust_bridge website demo at `https://cjycode.com/flutter_rust_bridge/demo/` loads the embedded Flutter web gallery, accepts the Start action, and renders the Mandelbrot animation instead of staying on the placeholder or a broken loading state.

## Source

- Context: Maintainer request to catch regressions in the public website demo before users discover that the hosted demo is broken.
- Related docs or skills: `.claude/skills/frb-manual-test/SKILL.md`, `website/docs/demo.md`, `website/src/components/FrbExampleGallery.tsx`

## When To Run

Run this before publishing or announcing a release, after changing the website demo page, after changing `frb_example/gallery`, after changing web build/deployment logic, and when validating that the already-published documentation site is healthy.

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`
- Required checkout state: not required; the browser verification targets the published website, not local files.
- Required credentials or account state: none.
- Required device or simulator state: none.
- Required network state: outbound HTTPS access to `https://cjycode.com`.

## Environment

- OS: macOS, Linux, or Windows with a graphical browser or browser automation tool.
- Flutter: not required.
- Dart: not required.
- Rust: not required.
- Device or simulator: not required.
- Browser or external service: Chromium, Chrome, Firefox, Safari, or an equivalent browser that supports Flutter web and WebAssembly threads. Record the browser name and version in the execution result.

## Preparation

Open the published demo page in a fresh browser tab or clean browser context.

```text
https://cjycode.com/flutter_rust_bridge/demo/
```

## Test Data

- Input files, API examples, account fixtures, or generated assets: none.
- Reset procedure before each run: reload `https://cjycode.com/flutter_rust_bridge/demo/` in a fresh tab or clean browser context.

## Steps

1. Navigate to `https://cjycode.com/flutter_rust_bridge/demo/`.

2. Wait for the page to finish its initial load. If GitHub Pages causes the documented one-time refresh, wait for that refresh to complete before judging the demo.

3. Find the `Demo` section and confirm the embedded gallery area appears. Before starting, it should contain the controls `Num threads`, `Image size`, `Start`, and `Stop`, plus a square placeholder that says `Tap to start`.

4. Capture a screenshot of the initial embedded gallery state.

5. Click the `Start` control in the embedded Flutter demo. If browser automation cannot target the Flutter text directly, click the visible `Start` label coordinates inside the embedded gallery.

6. Wait up to 20 seconds for the Mandelbrot render to appear.

7. Confirm that the square on the right changes from the gray `Tap to start` placeholder into a colored Mandelbrot image. The rendered image should include multiple saturated colors and the characteristic Mandelbrot shape, including a dark central region surrounded by colored fractal bands.

8. Confirm that a time label such as `Time: 13ms` appears above the rendered image and remains visible after the render completes.

9. Capture a screenshot of the rendered state after clicking `Start`.

## Expected Result

The test passes when all of the following are true:

- The page loads `https://cjycode.com/flutter_rust_bridge/demo/` without browser crash, visible Docusaurus error, or permanent `Loading...` state.
- The embedded gallery controls are visible before starting.
- Clicking `Start` causes the right-hand preview square to stop showing `Tap to start`.
- Within 20 seconds, the preview square displays a colored Mandelbrot render with a dark central region and colored fractal bands.
- A `Time: <number>ms` label is visible above the rendered image.

## Failure Criteria

The test fails if any of the following happens:

- The page does not load, repeatedly reloads, or stays blank.
- The embedded gallery remains in `Loading...` or only shows the `Tap to start` placeholder after `Start` is clicked.
- The `Start` control is missing or cannot be activated.
- No colored Mandelbrot render appears within 20 seconds.
- The rendered square is blank, uniformly gray, visually corrupted, or missing the Mandelbrot structure.
- Browser console errors, network errors, or WebAssembly/thread errors correlate with the broken demo behavior.

Mark the run as blocked, not failed, if the executor has no network access to `https://cjycode.com` or no compatible browser is available.

## Results To Capture

- Browser name and version.
- Final URL after any one-time refresh.
- Screenshot before clicking `Start`.
- Screenshot after the Mandelbrot render appears.
- Whether the `Time: <number>ms` label appeared, including the observed text.
- Any relevant browser console errors, network failures, or crash messages.

## Troubleshooting

- If the page shows the documented GitHub Pages limitation message and refreshes once, wait for the refresh to settle before failing the run.
- If the browser blocks WebAssembly threads, rerun with a modern Chromium-based browser and record the original browser/version as blocked.
- If the page loads but the demo is blank, capture browser console and network failures before retrying.
- If a flaky network request fails, reload once in a fresh browser context and record both attempts.

## Cleanup

Close the browser tab or browser automation context used for the run. No repository files, devices, services, or external account state should be changed by this test.

## Future Automation

This manual test is a good candidate for browser automation using Chromium. A future automated version should load the published URL, click `Start`, wait for `Time: <number>ms`, and compare screenshots or canvas pixels before and after the click to confirm that the Mandelbrot render changed from the placeholder to a non-uniform colored fractal image.
