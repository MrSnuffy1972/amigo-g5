# Game 1727 recovery: Volfied (Empire / Arcade Masters, 1991; licensed from Taito)

Written 2 Nov 2026 by the batch 2 agent (p1727). No game bytes, assets or disassembly are in this repository; the
recovery lives on Ryzen (`/srv/fast/native-work/1727/recovery/01727`, a private copy of
`/srv/fast/amigo/recovery/01727` made while the shared queue's own run of 1727 was still in its naming stage).

## Variant: 1727, not 1721

| | 1721 `[cr flt][a2]` | 1727 `[cr]` |
|---|---|---|
| main program | `Volfied`, 131,404 bytes, **packed** (only the 544-byte depacker stub is code until it runs) | `Volfied`, 184,844 bytes, **unpacked** AmigaDOS executable (5 hunks) |
| extras | Fairlight cracktro (`Fairlight`, run before `Boot1`) | none |
| data files | the same `Data/Screen*` and `Data/Sprite*` sets (file names `.cmp` there, `.cnp` here) | |
| first queue run | 35 functions, 336 executed PCs (the reference run never left the cracktro) | 1,246 functions (many false), 756 executed PCs |

1727 was chosen: the whole game's code is present on disk as a normal hunk executable, so every routine can be
recovered statically and checked against recordings, with no depacker or cracktro in the way. 1721 would add a
cracktro and a packer but no game code that 1727 lacks.

## What is on the disk

- `s/startup-sequence` runs `Boot1` then `Exit`. `Boot1` (188 bytes) starts `Boot2` as a process (LoadSeg +
  CreateProc); `Boot2` (164 bytes) opens dos and intuition and runs `Volfied`; `Exit` ends the CLI. `ClickMe` types
  the `ReadMe` (hard-disk installation text) in a RAW: window. These are the publisher's small launchers.
- `Volfied`: code hunk ($14C04 bytes), data hunk 1 ($3F40), BSS ($6108), chip data hunk 3 ($11C70), chip BSS ($800).
  Hand-written 68000 (not compiled): many variables live inside the code hunk next to the routines that use them.
- `Data/Screen@.cnp` (story picture), `ScreenA`-`ScreenP` (the 16 round surfaces), `ScreenQ` (the ending);
  `Data/Sprite@.dat`-`SpriteQ.dat`: one bob set per round (+ the story and ending sets).

## File formats (recovered from the code; re-implemented outside the repo)

- `.cnp` pictures (decoder +1C9A): words width, height and planes; `2 << planes` bytes of 12-bit palette; then each
  plane as bytes with $80 escapes: `$80 $80` is a literal $80, `$80 n` (n >= 0) repeats the previous byte n+2 times,
  `$80 n` (n < 0) copies -n+1 bytes from the line above. The caller's plane mask (d2) selects which planes are written:
  a round loads its own surface with all four planes and the next round's picture with planes 0-1 only (the claimed
  area shows that picture in four greys). All 18 decode to their last byte.
- `.dat` bob sets: a 32-bit length then an LZHUF stream (Okumura's coder: 4,096-byte ring buffer pre-filled with
  spaces, look-ahead 60, threshold 2, adaptive Huffman over 314 symbols; +2E14 with +529A init and +52FC update; the
  position tables sit in the code hunk). The decoded set is parsed by +8A74/+8B0A: entries of width, height and flags,
  then rows of four bitplanes (row-interleaved, 16 pixels a word). Flags: bit 1 mirror-extend horizontally (width
  2w-1), bit 2 vertically, bit 0 build the mask from colour 15 (transparent), bits 3/4/5 append a vertically,
  horizontally and doubly flipped copy. All 18 sets parse to their last byte (round 1: 351 frames).
- Font: 8x8, one bit a pixel, 8 bytes a character from code 0, in data hunk 1 at +277C; strings carry x, y (in cells)
  and colour bytes before the text and end with $FF (+7024).

## Copy protection

None found, after reading the load path and searching the code: the game reads its files through dos.library only
(`Open`/`Read` through +5ACE/+9024, "PLEASE REPLACE DISK IN DRIVE" on a failed read), there are no trackdisk or
custom-disk accesses, no DSKSYNC or CIA-B disk register use, no checksum loops over code, no manual lookup. The only
CIA-A accesses are the filter LED ($BFE001 bit 1) and the fire button (bit 7). The release supports hard-disk
installation (the ReadMe). The `[cr]` tag may refer to a disk-level change by the cracker; nothing in the program
needs removing. `TRAINER` sits in the program's own option table beside EASY, NORMAL and HARD and is handled by the
same menu code; nothing marks it as a cracker's addition, so it is treated as an original difficulty setting.
No cracktro or trainer program on this disk.

## Region analysis (code by region, never by file density)

The first queue run seeded code from Ghidra's guesses and decoded many variables and tables inside the code hunk as
instructions (1,246 functions; vasm stopped on a bogus `sbcd ... ,#imm`). The p1727 pass (`tools/regions.py` on
Ryzen) decides code by reachability: executed PCs from the recordings, the entry, relocated code pointers (dispatch
tables such as the 16 per-round handlers at +E91A) and `lea label,a6` state-machine continuations (each round's enemy
logic stores its next state that way), following branch, call and jump targets and fall-through. Decoded
instructions no flow reaches and that every reference accesses as data (tst, move, clr, st, addq/subq, lea for a data
access) became 30 data ranges (5,428 bytes: main-loop and option variables, the power-up box variables, the sprite-set
variables, the LZHUF tables, small tables) in `tools/p1727.json`, each with its evidence; no recording executes any of
them. Unreached code reached only by computed jumps (the 16 text-colour plotters at +7170, the bra table at +66C4)
is
seeded from the recordings.

Tool fix (in the private copy `tools-p1727`, general): capstone decodes two consecutive `sbcd -(a1),-(a0)` ($8109
$8109) as one 4-byte instruction with an immediate; `dis68k.decode` now keeps the 2-byte form (the second one is
executed, +5468). Other private-copy changes: paths off the offline /srv/fast2, own locks
(`.pipeline-p1727.lock`, `.llm-p1727*.lock`), config `tools/p1727.json`.

## Reference recordings

The canonical run (200 frames) stops on the title menu. The reference harness (project kit copy at
`/srv/fast/native-work/1727/hw`, with a per-frame state log, a frame interval and a PC log added) plays the game:
nine recordings (`orig/ref/play-p1`..`p9`, 20,000 to 40,000 frames, seeded random joystick scripts, plus one idle
run) cover the title menu and its options (difficulty incl. TRAINER and HARD, players, title sequence), the story,
round starts, cutting and claiming, deaths, game over, the high-score table and two-player turns. 7,743 distinct
executed PCs against 756 for the canonical run. The canonical re-run with the copy reproduces the canonical PCs
exactly.

## Naming pass, batch 2 continued (2 Nov 2026)

- The naming pass now runs on two GPUs: the forward pass on Ryzen's Glimmer and four shards on IntelPC's RTX 3090
  (qwen3.6:27b, `tools-p1727/describe_shard.py`, a copy of the reverse shard split four ways; IntelPC's Ollama
  serves one request at a time, so the four shards only remove the gaps between calls: about 3 to 4 routines a
  minute in all).
- Eighteen key routines were checked by hand against the code and the reference runs and carry hand names
  (`tools/p1727-names.json`, merged last by `tools-p1727/p1727_merge.py`): program start, wait for the vertical
  blank, main program, random step, round set-up, the player routine and its four direction steps, the claim fill,
  print string, the glyph plotter, the bob-set loader, the percentage format and draw, the round enemy dispatch and
  round 1's enemy logic. The model names they replace were wrong for most of them (for example the player routine
  was "process_game_state", the round set-up "initialize_copper_list" and the direction steps "empty_routine").
- `tools-p1727/finish-p1727.sh` (a background process, no timer or unit) waits for the shards, stops the forward
  pass, merges, runs describe (leftovers and retries on IntelPC), emit, build and report, then installs the root
  over `/srv/fast/amigo/recovery/01727` with a verified backup (`tools-p1727/install-p1727.sh`). Its log is
  `/srv/fast/native-work/1727/finish.log`. Proofs 3 and 4 are reported as they stand when it ends; the result table
  above is not updated until then.

## Result so far (2 Nov 2026)

| proof | result |
|---|---|
| 1. bytes classified | PASS: 100% of 901,184 top-level bytes (49 containers), 0 unknown |
| 2. byte-identical reassembly (vasm) | PASS: 7 of 7 units identical |
| 3. functions named and described | in progress (naming pass on Ryzen and IntelPC) |
| 4. executed addresses in recovered, described code | waits for proof 3 |

The table is updated when the naming pass ends; the current state is in `coverage.md` of the recovery folder.
