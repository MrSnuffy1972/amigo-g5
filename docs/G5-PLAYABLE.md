# G5 playable builds: the pre-boot menu, Windows and Android

Written 29 Oct 2026 for milestones M7 to M10. This file is the authority for those milestones, the way
docs/G5-NOTES.md is for M1 to M6.

## Decided by Karl
- **29 Oct 2026**: build Android versions of every game that passes G5 (today games 296, 336 and 608), with an
  **updated pre-boot menu in the same theme as Laya Game Playa (LGP)**: a modernised Amiga app, metallic plus neon.
  The **same pre-boot on Windows**. Claude Code builds it as milestones M7 to M9 in this repo; Ryzen packages, checks,
  copies and installs the builds (M10).
- **14 Oct 2026 (standing)**: the pre-start menu shows at **every** game start, stays **30 seconds** and **auto-loads
  the game if nothing is touched**, and clearly shows the controller defaults with **a picture of an Xbox-style
  controller and arrows to each mapping**.
- **14 Oct 2026**: modern fixed-timestep timing; CPU frame composition with SDL2 GPU presentation (D3D11 on Windows,
  GLES on Android).
- Finished ports are self-contained: one Windows .exe and one Android .apk, no emulator, no separate files.
- Android builds must run on Karl's NVIDIA Shield (Android TV, Shield controller or Xbox pads, TV remote) and on phones.
- **29 Oct 2026 (LGP decision, applies here)**: openly licensed fonts only, bundled with their licence files. No
  Slayer (Monotype-owned, cannot be bundled).

## The theme (from LGP-SPEC section 15, the same look)
- Modern with Amiga cues, readable on a TV from a couch (design for 1920x1080 and scale; minimum text height about
  28 px at 1080p for body text).
- Brushed-metal panels: near-black metal, subtle vertical gradient with fine striping; raised and recessed bevels
  (light top-left edge, dark bottom-right edge, as Workbench 2.0 did).
- Neon accents: cyan `#00E5FF` and magenta `#FF2BD6` glows and outlines on focus and headings; a soft outer glow on
  the selected item, subtle enough that text stays crisp.
- Workbench-style title bar with drag-bar stripes and gadget-like buttons.
- Classic Amiga blue `#0055AA` and orange `#FF8800` as secondary accents (the countdown bar, status chips).
- Fonts, all SIL Open Font Licence 1.1, in `rust/assets/fonts/` with the licence text beside each (added 29 Oct by
  the M7 prep PR; Ryzen's leak gate allows exactly these files by sha256):
  - **Cinzel** (`Cinzel-Variable.ttf`): the game title and headings.
  - **VT323** (`VT323-Regular.ttf`): the Amiga touch: countdown, version line, small labels.
  - **Inter** (`Inter-Variable.ttf`): body text and mapping labels.
- One theme module (colours, spacing, fonts, bevel and glow helpers) so it is tuned in one place.

## The pre-boot menu
- Shown at every start, before the game boots. A **30 second countdown** (orange bar plus VT323 seconds) then the
  game starts by itself. Pressing start (A, Start, Enter, mouse click on Start, TV remote centre) starts at once.
  Any other input pauses the countdown, so a player reading the controls is never cut off; it resumes after 10 s idle.
- Content: the game's name (Cinzel), an "Ami-Go G5" badge, the build line (repo rev, date) in VT323.
- **Controller panel**: an Xbox-style pad **drawn in code** (vector shapes: body, sticks, D-pad, face buttons
  A/B/X/Y, bumpers, triggers, Start/Back). No copied artwork and no trademark logos. **Leader lines with arrows** run
  from each control to a label naming what it does in this game (for example "D-pad / left stick: joystick",
  "A: fire", "Start: pause"). Unmapped controls are drawn dimmed.
- **Keyboard and mouse panel**: the keys this game uses and the mouse mapping.
- Items: Start game, Controls (full-screen diagram), Quit. Navigable by pad, keyboard, mouse and TV remote
  (Android KEYCODE_DPAD_* and DPAD_CENTER; Back = Quit on the pre-boot, Pause in game).
- The mapping is **data**, not code: read from the game's game.json `controls` block when present, otherwise the
  default Amiga mapping (joystick in port 2, fire, mouse in port 1, keyboard pass-through). Define the block's schema
  in this file when M7 adds it.
- Headless review: `--preboot-shot OUT.png --preboot-at SECONDS` renders the menu at that countdown point with the
  SDL dummy video driver and exits, so Ryzen and CI can capture it without a display.

## The playable runtime (M7)
- A binary `amigo-g5-play` and a library entry used by Android (`SDL_main`), on the same core as `amigo-g5-boot`.
  For the same game, inputs and tick, the frames are **identical** to `amigo-g5-boot`'s headless dumps (test it).
- SDL2 through the `sdl2` crate (justify it in the PR, per CLAUDE.md). On Linux and Windows build SDL2 statically
  (`bundled` and `static-link` features) so the exe needs no DLL; Ryzen has cmake and the X11, Wayland and ALSA
  development packages but no system libSDL2. On Android link the prebuilt `libSDL2.so` (SDL 2.30.9) that Ryzen's
  packager bundles with SDL's own Java activity.
- 50 Hz fixed timestep, decoupled from the display refresh; audio 44,100 Hz stereo, 882 frames per tick, queued with
  a small latency target.
- Inputs: SDL GameController (Xbox, Shield, Switch Pro and others through SDL's mapping database), keyboard and
  mouse, mapped to the Amiga joystick, mouse and keyboard the game reads.
- In-game pause (Start or Back on a pad, Esc on a keyboard, Back on a TV remote): Resume, Restart, Controls, Quit, in
  the same theme. Android Back must never just kill the game.
- Scale to fill the screen at the correct aspect with square-ish Amiga pixels; letterbox in brushed metal.
- **Game pack**: the game's files plus game.json in one blob with a small documented format, written by a packer in
  `tools/` (tested on synthetic files only). Windows embeds it in the exe at build time on Ryzen; Android carries it
  as the APK asset `gamepack.bin`, read through SDL_RWops. No game bytes ever enter this repo.

## The packaging contract with Ryzen (M9)
- `tools/g5_package.sh --pack PACK.bin --name NAME --id ID --out OUTDIR [--android] [--windows] [--icon PNG]
  [--banner PNG]`, run on Ryzen inside the `amigo-tools` image, offline. The image provides: `/opt/android` (SDK,
  NDK 26.3.11579264, build-tools 37.0.0, platform android-34, `debug.keystore`), `/opt/sdl2-android/<abi>/libSDL2.so`
  with headers in `/opt/SDL2` (SDL 2.30.9 source in `/opt/SDL2-2.30.9`, which has the Java activity), and mingw-w64.
  Ryzen mounts its Rust toolchain (1.99.1) with the targets aarch64-linux-android, armv7-linux-androideabi,
  x86_64-linux-android and x86_64-pc-windows-gnu, and runs `cargo fetch` before the offline build.
- Reuse `kit/amigo/android/build_apk.sh`'s Java, manifest, resource and signing stages where you can; replace only
  the native step with the Rust `cdylib` named `libmain.so`.
- Android: ABIs arm64-v8a and armeabi-v7a (x86_64 optional); LAUNCHER and LEANBACK_LAUNCHER entries, a TV banner,
  touchscreen not required, gamepad declared, landscape, package `org.amigo.g5.<id>`, debug-signed. Must pass
  `kit/amigo/g6check.py` (only Android system libraries plus the bundled SDL2 and libc++; `SDL_main` exported).
- Windows: x86_64-pc-windows-gnu, SDL2 static, one exe, no console window, and the G5 no-interpreter symbol check.
- CI: `cargo check` for aarch64-linux-android and x86_64-pc-windows-gnu (add the targets in CI; linking needs the
  NDK and mingw, which only Ryzen has). Everything else Ryzen runs in M10.

## M10 (Ryzen only, not for a Claude Code session)
Package games 296, 336 and 608 for Windows and Android, run g6check.py, check that `amigo-g5-play` frames match the
G5 headless frames, capture the pre-boot with `--preboot-shot`, then copy the builds to WinPC
(`F:\Game Source\Amiga\Android` and `\Windows`), install the APKs on the YugeTV Shield, and send them to Karl.
