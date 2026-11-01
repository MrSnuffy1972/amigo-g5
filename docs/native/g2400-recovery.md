# Game 2400: Bubble and Squeak (Audiogenic 1994), code recovery

Written 1 Nov 2026 UTC by the batch-2 agent for game 2400. No game bytes, assets or disassembly are in this file;
everything game-derived stays on Ryzen under `/srv/fast/amigo/recovery/02400/` and `/srv/fast/native-work/2400/`.

Status: **stage 1 not finished.** Proofs 1 and 2 pass for disk 1 (shared pipeline run); disk 2 is not yet in the
recovery; proofs 3 and 4 fail. The reference harness now reaches the title screen and level 1 (it did not before),
which is what proof 4 and the stage 4 verification both need. No native code yet.

## Variant: 2400 (disk 1) plus disk 2 of 2399

| project | disk 1 | disk 2 |
|---|---|---|
| 2399 `[cr PDX]` | PARADOX cracktro, then `code` | yes |
| 2400 `[cr PDX][t +3 Trader's Dream]` | PARADOX cracktro, then `Train` (trainer), which loads `code` | **missing** |
| 2401 `[cr PDX][a2]` | same files as 2399 | same files as 2399 |

- Disk 1 holds the program (`code`), the cracktro (`PARADOX`), the title and menu art and the title music. Disk 2
  (volume `bubble2`) holds every level: 48 maps (`Maps/`), 48 object lists (`Objects/`), the section graphics and
  palettes (`sec*.raw`, `*.grad`), the enemy and boss sprites (`*.all`) and the in-game music (`*.MOD`). Without
  disk 2 the game cannot leave the title.
- 2400's `code`, cracktro and data files are byte-identical to 2399's (same file hashes); 2400 only adds `Train`
  and changes `s/startup-sequence` to run it. 2399 and 2401 have identical files on both disks.
- Chosen: **2400 disk 1 + 2399 disk 2** (the same PDX release). It is the only set with both the full game and the
  trainer the process asks to recover and keep as a cheats menu. The id stays 2400.

## What the program is
- Startup: `PARADOX` (cracktro), `closewb`, `Train` (trainer; its menu loads and patches `code`).
- `code`: AmigaDOS executable, 5 hunks: code 188,540 bytes (any memory), code 10,912 (the music replayer),
  code 2,292, BSS 186,060 (chip), BSS 236,228. Hand-written assembler (no compiler runtime).
- Start-up checks (not protection): two labels in the chip BSS must share one 64 KB bank, then `AvailMem(largest
  chip)` must leave at least $41EB0 bytes ("Insufficient Memory" otherwise). The game needs 1 MB of chip RAM.
- Interrupts: a VERTB server ("BubbleVB": frame counters, the music replayer call, a beam wait) and a COPER server
  ("BubbleCop": the main loop's frame flag), plus a PORTS server ("BubbleKeys").
- Input: joystick port 2 (JOY1DAT and CIA-A fire bit 7), one of three readers chosen by an option.
- Files: through dos only (`Bubble1:` and `Bubble2:` paths). Packed files are unpacked by the recovery's
  temisu/ancient step (all 40 containers classified).

## Copy protection (read in the code)
- None found in `code`: no trackdisk, no disk registers, no trace-vector or checksum tricks; the `DoIO`-looking
  call sites are graphics `OwnBlitter`/`WaitBlit`. The release was cracked by PARADOX before the trainer was added.
- Not protection: the level password screen (`DPPASSWORD`, with the hint "TRY TYPING MAXIBABY AS A PASSWORD") is a
  game feature and stays. A `cd.device` string is a CD32-build remnant.

## Cracktro (keep as a native intro)
`PARADOX`: copper-gradient background, scrolling message ("PARADOX cracked: BUBBLE AND SQUEAK, supplied by MIMICOM,
cracked by HARMONICA", a poem, contact details); waits for the left mouse button.

## Trainer (becomes the cheats menu, off by default)
`Train` (Traders Dream, "+3 micro trainer", music by Martin Wall): mouse menu with three toggles, Yes on the left
button and No on the right. On "start game" it waits for both buttons, `LoadSeg`s `code` and patches it before
running it:
- infinite lives: NOP one instruction at code+$27880;
- infinite energy: NOP one instruction at code+$27946;
- invincibility: a `BRA` at code+$20D2C and an `RTS` at code+$2135A.
The native cheats menu reproduces these three effects in the game logic.

## Reference harness (private copy, `/srv/fast/native-work/2400/prekit`)
The project's prekit harness never left the trainer menu (1,089 PCs). Changes, all in the private copy:
1. Disk 2 embedded as Method B multi-disk (`oshle_ndisks = 2`, volumes Bubble1/bubble2).
2. Build fix for current gcc (`-std=gnu17 -Wno-implicit-function-declaration`).
3. Script tokens: `kXX` raw key, `v ^ < >` mouse motion, `r` right mouse button, `f` joystick button 2; script table
   16,384 entries (was 512).
4. Memory: 2 MB chip model (the game's chip check and the fake OS's smaller AllocMem region), and `FreeMem` of the
   topmost block really returns it (the game frees its largest-block probe and allocates $400 less).
5. COPER interrupt servers: the fake OS ran only VERTB servers, so the main loop's frame flag never came. A new
   level 3 handler (vasm source `tools/l3.s`) runs COPER and VERTB servers separately.
6. `CPUSCALE=0.75` for gameplay runs: the intro waits for beam line 75 straight after its line-74 copper interrupt;
   the fake OS's interrupt dispatch is slower than Kickstart's, so at 1.0 the wait missed the line every frame.
   This is a documented reference deviation (frame-locked game logic is unaffected; CPU-bound timing is not exact).
- Result: cracktro, trainer menu, copyright, intro, title, "Get Ready: Stardust, Caverns One" and level 1 drawn
  correctly (checked by eye).
- Measured (4,000-frame run, joystick right held with jump taps): the "Get Ready ... PRESS START" card shows at
  frame ~2,250 and level 1 runs from ~2,400 with the port-2 reader (code+$24526) polled every frame, but the
  player stays at the left edge and the screen does not scroll, so the play recordings do not yet cover gameplay.
  Next step: read the level-start state (player object, start/pause flags) from the RAM dumps.

## Proofs today
| proof | state |
|---|---|
| 1 classification | PASS for disk 1 (905,312 bytes, 0 unknown); disk 2 not yet run |
| 2 reassembly | PASS: 7 of 7 units identical (boot block, closewb, code, PARADOX, Train, two run-time segments) |
| 3 functions | FAIL: 30 of 900 named, 7 described (the model pass was deferred by the GPU-residency rule) |
| 4 executed addresses | FAIL: the canonical run covered only the trainer menu; the new runs reach level 1 |

## What remains
1. Recovery: add disk 2 to the pipeline (private tools copy as b1 did), add the new reference runs (title, level 1,
   and gameplay once input works), run the naming pass, check and fix names by hand, reach proofs 1 to 4.
2. Understanding, the native rewrite (`rust/games/g2400`), verification, play test and release: not started.
