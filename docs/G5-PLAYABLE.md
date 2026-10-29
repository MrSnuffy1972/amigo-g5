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

## Per-game controls on the pre-boot (Karl, 29 Oct 2026; for M8)
Karl: "update the preboot for each game with specific controls shown as controller mapping, if we know them."
- Ryzen builds each game's `controls` block (the M7 schema) with `~/bin/amigo-controls.py` from the evidence it
  has: the game's own screen (screen reader), its manual (quote-gated), quote-gated web research, the keys its code
  reads, and model research. It goes into the game pack's game.json at packaging time. Nothing here changes.
- The block adds two optional fields M8 must read (and ignore when absent):
  - `label_status`: by the same control or key names as `labels`, one of `confirmed`, `manual`, `web`, `code`,
    `unconfirmed`. **Karl decided (29 Oct) to show unconfirmed mappings, marked**: the pre-boot shows a small chip
    beside a label that is not `confirmed` (for example "from manual", "unconfirmed"), in VT323, dim magenta. A
    `confirmed` label has no chip.
  - `start_hint`: `{"text": "F1 or fire: start", "status": "web"}` when known; the pre-boot shows it under the title,
    with the same chip rule.
  - `touch`: the on-screen controls for phones (next section).
- Pad layout is automatic (Karl, 29 Oct: no review step): joystick on the D-pad and left stick, fire on A, second
  fire on B when the game uses it, the game's keys with a known meaning on X, Y, B, RB, LB, RT, LT in order of
  evidence strength, Start opens the pause menu (a game's own pause key is shown in its label).

## On-screen touch controls for Android (Karl, 29 Oct 2026; pre-boot checkbox in M8, Android in M9)
Karl: a checkbox in the pre-boot to add an on-screen directional controller on the left side of the screen, four
buttons mapped to the game's controls and/or the key that starts the game (for example F1), and a pop-up keyboard
when the game uses the keyboard, for the Android builds, in case they run on a phone.
- **Pre-boot checkbox** "On-screen controls", Android only (hidden on Windows). Default: on when the device has a
  touchscreen and no game controller is connected, otherwise off. Remembered per game (app storage).
- **Left side**: a virtual joystick (a thumb pad with 8 directions plus a visible dead zone), feeding the same
  `joystick` action as the D-pad.
- **Right side**: four round buttons in a diamond, labelled from `touch.buttons` (or, when absent, fire, second fire,
  then the first two keys from the pad layout, with the start key always included when `start_hint` names one).
- **Keyboard button**: a small key icon that pops up an Amiga-layout on-screen keyboard (function keys, Esc, Help,
  Del, cursor keys, letters, digits, Space, Return, Shift, Ctrl, Alt, Amiga keys), shown only when `touch.keyboard`
  is true or the game's controls use keys beyond the four buttons. Keys go to the game exactly as a real keyboard's.
- **Pause**: a small pause button top-right opens the pause menu (Android Back does the same).
- Themed like the rest: brushed-metal ring for the stick, neon cyan focus, labels in Inter, opacity about 40% at rest
  and 80% while touched, never covering the middle third of the game picture; scales with screen size.
- Multi-touch: stick and buttons at the same time; a finger that slides off a button releases it.
- `touch` schema: `{"buttons": [{"action": "fire", "label": "Fire"}, {"action": "key:F1", "label": "Start"}, ...]
  (exactly 4), "keyboard": true}`. Ryzen fills it per game; the code must work with it absent.
- Tests: the layout at 1080x2400 and 720x1600 portrait-rotated-to-landscape (no overlap, the middle third clear),
  touch events driving the same input path as the pad in the scripted-input test, the checkbox default rule.

## The playable runtime (M7)
- A binary `amigo-g5-play` and a library entry used by Android (`SDL_main`), on the same core as `amigo-g5-boot`.
  For the same game, inputs and tick, the frames are **identical** to `amigo-g5-boot`'s headless dumps (test it).
- SDL2 through the `sdl2` crate (justify it in the PR, per CLAUDE.md). On Linux and Windows build SDL2 statically
  (`bundled` and `static-link` features) so the exe needs no DLL; Ryzen has cmake and the X11, Wayland and ALSA
  development packages but no system libSDL2. On Android link the prebuilt `libSDL2.so` (SDL 2.30.9) that Ryzen's
  packager bundles with SDL's own Java activity.
- Measured on Ryzen 29 Oct 2026: `sdl2` 0.37 with `bundled` and `static-link` builds and opens a window under the
  dummy driver (SDL 2.26.4), but only with `CMAKE_POLICY_VERSION_MINIMUM=3.5` (cmake 4 refuses SDL's old minimum)
  and `CFLAGS=-std=gnu17` (gcc 15 defaults to C23, where SDL 2.26's `false` enum fails). Ryzen's gate sets both;
  set them in CI too if its cmake or gcc is that new.
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

## Added by M7 (29 Oct 2026)

### amigo-g5-play
```
amigo-g5-play (--pack PACK.bin | --game DIR [--config game.json]) [--program REL] [--fullscreen]
              [--ticks N --frames OUTDIR] [--script FILE]
```
- `--program` picks the program as amigo-g5-boot's does (both use `amigo_g5::game::GameTree`, moved out of the boot
  binary unchanged). The game's code is compiled in the same way (AMIGO_G5_GAME_RS, crates amigo_g5_part0..7), and
  the binary prints `play: game code fnv <16 hex>`.
- `--ticks N` runs N ticks unpaced and exits; with `--frames` it writes them as `pNNNNN.rgba`, the format of
  amigo-g5-boot's `--frames`, so the two can be compared byte for byte. `--script` feeds scripted input, one event a
  line: `TICK key down|up NAME`, `TICK pad down|up NAME`, `TICK axis NAME X Y`, `TICK mouse DX DY`,
  `TICK button down|up left|right` (an event is taken just before its tick runs; ticks count from 1).
- Exit 0 when the player quits or N ticks ran, 3 when the game stopped early, 2 for bad arguments or files.
- The crate `rust/amigo_g5_play` is **its own cargo workspace** with its own Cargo.lock. Measured 29 Oct: with sdl2,
  fontdue and serde_json in the main workspace, `cargo build --release --offline --locked --bin amigo-g5-boot` fails
  ("no matching package named `fontdue` found") on a cargo cache that lacks them, which would break Ryzen's G5 gate.
  Build it with `cd rust/amigo_g5_play && cargo build --release` (after `cargo fetch` for an offline build).
- The `sdl2` crate's bundled source is SDL 2.26.4 (sdl2-sys 0.37).

### Game pack format
Written by `tools/g5_pack.py pack --game DIR [--config game.json] --out PACK.bin` (also `list PACK.bin` and
`unpack PACK.bin --out DIR`), read by `rust/amigo_g5/src/pack.rs`. All integers little-endian.

| Part | Bytes |
|---|---|
| Header | `AG5PACK1` (8 bytes), u32 entry count |
| Each entry | u16 name length, the name (UTF-8, `/` separated), u32 data length, the data |
| Trailer | u64 FNV-1a 64 of every byte before it (a damaged or cut pack is refused) |

Entry names: `game.json` (optional, first) and `files/<path>` for each game file, in directory-walk order (each
directory's entries sorted by name, a subdirectory's files where its name sorts): the order amigo-g5-boot serves a
game directory's files in, so a pack and its directory boot the same way.

### The controls block
An optional `controls` object in the game's game.json. Every field is optional; what it leaves out keeps the
default, so a game names only what differs. Without the block the game gets the default Amiga mapping.

```json
"controls": {
  "pad":  {"a": "fire", "b": "fire2", "x": "key:Space", "start": "pause"},
  "keys": {"Left Ctrl": "fire"},
  "keyboard_passthrough": true,
  "mouse": "port1",
  "labels": {"x": "Smart bomb"}
}
```
- `pad`: pad control to action. Controls (Xbox layout, SDL GameController): `dpad`, `left_stick`, `right_stick`,
  `a`, `b`, `x`, `y`, `lb`, `rb`, `lt`, `rt`, `start`, `back`, `ls`, `rs` (stick clicks). `dpad` and the sticks take
  `joystick`, `mouse` (sticks) or `none`; a trigger past half way is a button.
- `keys`: host key (SDL scancode name, for example `Right Ctrl`, `Space`, `F1`, `Keypad 5`) to action. A key named
  here is not passed through.
- Actions: `up`, `down`, `left`, `right`, `joystick`, `fire`, `fire2` (port 2), `mouse`, `mouse_left`,
  `mouse_right` (port 1), `key:NAME` (the Amiga key at that host key's place, or `key:$HH` by raw key code),
  `pause`, `none`.
- `keyboard_passthrough` (default true): every other host key with an Amiga key reaches the game as that key.
- `mouse`: `port1` (default) or `none`.
- `labels`: what the pre-boot (M8) shows for a control, by pad control or key name; absent ones are named by action.
- Defaults: `dpad` and `left_stick` joystick, `right_stick` mouse, `a` fire, `b` fire2, `lb` / `rb` mouse buttons,
  `start` and `back` pause; keys: the arrows are the joystick, `Right Ctrl` fire, `Escape` and `AC Back` (the
  Android TV remote's Back) pause; passthrough on; mouse in port 1. The host mouse moves the Amiga mouse one count a
  pixel; a stick on `mouse` moves it up to 6 counts a tick.

## Added by M8 (29 Oct 2026)
- `amigo-g5-play` opens on the pre-boot at every start; only a `--ticks` run (headless review, the frame test) goes
  straight into the game. `--preboot-shot OUT.png [--preboot-at SECONDS]` draws the pre-boot as it looks SECONDS
  (0 to 30, default 0) after it opened, as a 1920x1080 RGB PNG, and exits without booting. Every screen is drawn on
  the CPU, so the shot needs no display and no SDL video driver at all (it also works under the dummy driver).
- Code: `theme.rs` (the one theme module: colours, the three fonts, brushed metal, bevels, glow, text, arrows, the
  PNG writer), `pad.rs` (the pad drawn in code, the label layout, the keyboard and mouse rows), `preboot.rs` (the
  countdown, navigation, the menu and Controls screens). Screens are laid out in a 1920x1080 design space and scaled
  to the window, centred.
- Countdown: 30 s; any input other than start pauses it (shown in magenta, "PAUSED"); 10 s with no input resumes it
  from where it stopped. The mouse only pauses it when it hovers a button, so a mouse bumped while the window opens
  does not. The countdown never starts the game while the Controls screen is open.
- Keys: arrows / D-pad move, Enter / Space / A / the remote's centre (SDL `Select`) choose, pad Start starts at once,
  Escape / B / Back / the remote's Back quit on the menu and return from the Controls screen; mouse hover focuses
  and a click chooses.
- Labels: `"<control>: <labels entry or the action in words>"`, cut with "..." to fit the column. The chip of a
  `label_status` other than `confirmed` reads "from manual", "from web", "from game code" or "unconfirmed"; it goes
  under the label, or after it on the same line when a side is too crowded. Controls with no action are drawn
  dimmed with no label. `start_hint` shows under the title with the same chip rule.
- The build line reads `rev $AMIGO_G5_REV  $AMIGO_G5_BUILD_DATE`, both set at build time (Ryzen's packager sets them
  in M9); `rev dev` when unset.
- The in-game pause menu has its Controls entry (the pre-boot's Controls screen; any press returns).

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

## Added by M9 part 1 (29 Oct 2026)
- **One start for every platform**: `amigo_g5_play::app::main(args, built-in pack, game code)` holds the command line,
  the pack choice and the exit codes. The desktop binary calls it from `main`; the Android library calls it from
  `SDL_main`. Before SDL starts it sets `SDL_ANDROID_TRAP_BACK_BUTTON=1`, so the TV remote's or phone's Back arrives
  as the `AC Back` key (pause in game, quit on the pre-boot) and never just closes the activity.
- **Where the pack comes from**, in order: `--pack PACK.bin` or `--game DIR`; the pack built in at build time
  (`AMIGO_G5_PACK=PACK.bin cargo build ...`, read by `build.rs` with `include_bytes!`); else `gamepack.bin` through
  SDL_RWops (the APK asset on Android, a file in the working directory elsewhere; stderr says `play: pack
  gamepack.bin`). With a pack built in, `build.rs` sets the cfg `g5_embedded`, and on Windows that makes the exe a GUI
  program (`windows_subsystem = "windows"`: no console window). A build without a pack keeps its console, so a
  `--ticks` frame check under wine still prints.
- **Android library**: crate `rust/amigo_g5_play/android` (package `amigo_g5_android`, a member of the play
  workspace), a `cdylib` named `main`, so the file is `libmain.so`. `SDL_main` is in its module `ffi`; it reads no
  argument (SDL passes only the library name), so it has no `unsafe`, and it catches a panic so unwinding never
  enters SDL's C. It builds the play crate without the `desktop` feature: SDL2 is linked as the prebuilt
  `libSDL2.so`, not built from source.
- **Cargo features of `amigo_g5_play`**: `desktop` (default) = `sdl2/bundled` + `sdl2/static-link`, as in M7.
- **`tools/g5_package.sh`** (the contract's command line, plus `--skip-symcheck` and `--dry-run`):
  - `--windows`: `AMIGO_G5_PACK=PACK cargo build --release --offline --locked --target x86_64-pc-windows-gnu --bin
    amigo-g5-play`, then `OUTDIR/unstripped/ID.exe` (symbols kept) and `OUTDIR/ID.exe` (mingw strip).
  - `--android`: for arm64-v8a and armeabi-v7a (x86_64 too with `G5_ANDROID_X86_64=1`), `cargo build --release
    --offline --locked --target T -p amigo_g5_android` with the NDK's API 21 clang as linker and
    `-L native=/opt/sdl2-android/ABI`, without `AMIGO_G5_PACK` (the pack is the asset, not in the library twice);
    each ABI's directory gets the stripped `libmain.so`, the prebuilt `libSDL2.so` and the NDK's `libc++_shared.so`;
    `OUTDIR/unstripped/ABI-libmain.so` keeps the symbols. Then the stages of `kit/amigo/android/build_apk.sh`: SDL's
    Java activity with the one-line `org.amigo.GameActivity`, icon and TV banner (given, or shapes in the theme's
    colours), the same manifest (LAUNCHER and LEANBACK_LAUNCHER, banner, sensorLandscape, touchscreen not
    required, gamepad declared) with package `org.amigo.g5.ID`, aapt2 with the asset directory holding
    `gamepack.bin`, zipalign and debug signing. The stages are copied, not called: build_apk.sh needs C sources,
    and changing any kit/ file moves Ryzen's kit_rev, which rebuilds every G2 to G4 workspace.
  - Every unstripped binary goes through `$G5_SYMCHECK BINARY` (Ryzen's g5_symcheck.sh) before it is stripped;
    without `G5_SYMCHECK` the script stops with exit 4, unless `--skip-symcheck` (which prints `SYMCHECK
    SKIPPED`). `kit/amigo/g6check.py` is Ryzen's and is not in this repo: run it on the APK there.
  - `AMIGO_G5_GAME_RS` (the game's code), `AMIGO_G5_REV` and `AMIGO_G5_BUILD_DATE` come from the environment (the
    last two default to the repo's short rev and today's UTC date). Exit 0 built, 2 bad arguments or pack, 4 a
    symbol check failed or could not run, 1 a build step failed.
- **CI**: `cargo clippy -D warnings` for `aarch64-linux-android` (`-p amigo_g5_android`) and for
  `x86_64-pc-windows-gnu` (`--no-default-features`: with `desktop` its build script would build SDL2 with a mingw
  compiler CI does not have; the Rust checked is the same).

## Added by M9 part 2 (29 Oct 2026): the on-screen touch controls
- **Code**: `rust/amigo_g5_play/src/touch.rs` (button choice, layout, fingers to actions, the overlay drawing, the
  checkbox default and storage); the checkbox in `preboot.rs`; the wiring in `lib.rs`.
- **One input path**: a finger arrives as `Host::Finger(id, down|move|up, x, y)` (x and y as fractions of the window).
  When the controls are on, `TouchPad` turns it into `Host::Touch(source, action, held)`, which `Mapper` holds and
  lets go of exactly as it does a pad button's action, so the game reads the same `Input` and the same key presses.
  With the controls on, the mouse events SDL makes from touches (`SDL_TOUCH_MOUSEID`) are dropped in game, so a
  finger is not also the Amiga mouse; with them off, a tap still works as the mouse.
- **Checkbox** "On-screen controls": focus item 4 of the pre-boot (after Quit, right of the countdown bar), toggled
  by Choose, a click or a tap. Shown only on Android, where its value is the stored choice, else on when SDL sees a
  touch device and no connected game controller. The choice is saved as `onscreen-controls` (`on` or `off`) in
  SDL's pref path for org `Ami-Go`, app `G5_<title>` (non-alphanumerics as `_`); on Android that is the app's own
  internal storage, so it is per game.
- **`--touch on|off`** (any platform): shows the checkbox with that value and remembers nothing; with `on` the game
  starts with the controls. For review on a desktop and for the tests. `--preboot-shot` draws the checkbox when given.
- **Script line** `TICK touch down|move|up ID X Y` feeds a finger in a `--ticks` run.
- **Buttons**: `touch.buttons` when given (exactly 4, each `{action, label}`; otherwise the game.json is refused);
  else fire and fire 2 when the pad has them, then the keys of X, Y, B, RB, LB, RT, LT in that order, with the key
  `start_hint` names put in the fourth place if it is not already among them. A start key is found in the hint as a
  function key, a named key (Space, Return/Enter, Esc, Del, Help) or one capital letter or digit. **Decision (29 Oct
  2026)**: when that gives fewer than four (the default mapping gives Fire and Fire 2), the empty places are not
  drawn; a made-up button would do nothing. Positions: bottom, right, left, top of the diamond (A, B, X, Y on a pad).
  Labels: Fire, Fire 2, or the key's name.
- **Keyboard button**: shown when `touch.keyboard` is true or a `key:` action in `pad` or `keys` has no button. The
  pop-up is an Amiga 500 layout in six rows (Esc, F1 to F10, Del, Help; the digit row with Backspace; Tab to
  Return; Ctrl to the second half of Return; Shift to the arrows Left and Right; Alt, Amiga, Space, Amiga, Alt, Up,
  Down) along the bottom 55% of the screen. Each key sends its Amiga raw key down while touched and up when lifted
  or slid off; modifiers are held by a finger, as on a real keyboard. **Decision (29 Oct 2026)**: while the
  keyboard is open, the stick and buttons are hidden and let go (the keyboard needs the width); the key and pause
  buttons stay.
- **Layout** (pixels, from `u` = the shorter of the height and width x 9/16, over 1080): margin 40u, stick radius
  190u in the bottom-left corner with a dead zone of 0.3 of its radius and 8 directions, buttons of radius 78u at
  120u from the diamond's centre in the bottom-right corner, pause and key buttons 96u square at the top right.
  Tested at 2400x1080, 1600x720 (the two phones turned to landscape), 1920x1080 and 1280x720: nothing overlaps,
  everything is on screen, nothing is over the middle third of the picture's width.
- **Drawing**: a see-through overlay drawn on the CPU when a control changes, blended over the picture: metal discs,
  light rim at rest, cyan rim and knob while touched, a visible dead-zone ring, Inter labels; 40% opaque at rest,
  80% touched (the keyboard 80% while open).
- **Pause menu**: a click or tap picks the item under it (a phone has no keys to move the focus); the pointer is
  shown while paused (relative mouse mode is off), so a desktop click lands where it is seen.
- **Fixed on the way**: `rawkey("F11")` returned nothing (the function-key test stopped before the Help entry), so
  F11 never reached the game as Help; it does now.

## M10 (Ryzen only, not for a Claude Code session)
Package games 296, 336 and 608 for Windows and Android, run g6check.py, check that `amigo-g5-play` frames match the
G5 headless frames, capture the pre-boot with `--preboot-shot`, then copy the builds to WinPC
(`F:\Game Source\Amiga\Android` and `\Windows`), install the APKs on the YugeTV Shield, and send them to Karl.

## M11: the multi-game launcher (Karl, 29 Oct 2026; after M10)
Karl wants a launcher for many games, in the same LGP theme, on the Shield and on Windows: a browsable shelf of box
art, each game with its art, a linked manual and instructions, links to videos, and (later) short instruction videos
made from our own play. It hands off to each game's own build, whose pre-boot still shows.
- Media is gathered on Ryzen by `~/bin/amigo-media.py` into `/srv/bulk/amigo-media/games/<id5>/` (art, manuals,
  `manifest.json` with sources, match scores and video links). That media is third-party artwork and scans for
  Karl's personal use: it **never enters this repo**. The launcher code reads a media pack Ryzen builds at packaging
  time, in the same way the game pack works; tests use synthetic images only.
- Not scheduled for an automatic run yet: M11 starts only when the routine's milestone list names it.
