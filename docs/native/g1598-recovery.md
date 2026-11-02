# Game 1598 (PGA Tour Golf, Electronic Arts 1991): code recovery

Native batch 2, written 2 Nov 2026 by the p1598 agent. Numbers and names only: the source, the disassembly and every
game byte stay on Ryzen (`/srv/fast/native-work/1598/recovery/01598`, tools in `tools-g1598/`).

## Which variant, and why 1598
Ami-Go holds five PGA Tour Golf variants. 1592, 1593, 1594 and 1595 are each **disk 1 of 2 only** (the program disk);
the courses disk is missing from the corpus for all of them, so none of them can play a hole. **1598 ("[cr AGL][a]")
is a one-disk release: the program and two courses (TPC Sawgrass and TPC Avenel) on a single disk**, with the real
music (GMUSIC). Its program is the same build as 1593's: once unpacked, every byte of the code and data hunks and the
full relocation sets are identical to 1593's unpacked Golf. 1596 (not in this batch's list) is another one-disk
release with all four courses but with the music file replaced by a copy of the fly-by tune; its two extra courses
(PGA West Stadium, Sterling Shores) use the same file formats and can be added to the pack later.

## The four proofs (state on 2 Nov 2026)
| proof | result |
|---|---|
| 1. every byte classified (901,248 top-level bytes, 44 containers) | **PASS**: 100.000%, 0 unknown |
| 2. byte-identical vasm reassembly | **PASS**: 15 of 15 units identical (3 raw-word fallback lines) |
| 3. every routine named and described | **PASS** (2 Nov 2026): all 1,409 routines; EA author symbols name 608, the local models (Glimmer on Ryzen, qwen3.6 on IntelPC) wrote the rest, 37 the models could not get past the quality check were named and described by hand from the code |
| 4. every executed address in recovered, described code | **PASS** (2 Nov 2026): all 2,475 executed PCs of the reference run fall in recovered, described routines |

The full generated summary (proofs, units, every routine name) is in [g1598-recovery-summary.md](g1598-recovery-summary.md).

## How the code was recovered
- **Packed program.** `Golf` (72,296 bytes, 7 hunks) is crunched with an Imploder-style executable cruncher that
  ancient does not open. Hunk 0 (80 bytes) walks the seglist to hunk 4 and calls it; hunk 4 (496 bytes) is the
  decruncher: an "explode" decoder (literal runs, four length classes, three offset tables) reading the packed data in
  hunk 5 backwards through an $4900-byte ring buffer (BSS hunk 6), writing hunks 1 to 3 and applying the RELOC32 groups
  stored in the stream; then it frees hunks 4 to 6 and returns into hunk 1. The decruncher was ported to Python
  (`explode_g1598.py`) from the recovered code; it consumes the whole stream (the read pointer ends at offset 0) and
  gives a 114,564-byte code hunk, 11,728 bytes of data (53,036-byte hunk) and a 4-byte BSS hunk with 5,673 + 435
  relocations: identical to the 1593 unpacked Golf (one relocation group is stored in a different order; the sets are
  equal). The unpacked executable is recovered as its own container; the packed file's depacker hunks are code, the
  payload is packed data, hunk 6 is the work buffer.
- **Names.** The 1593 program disk carries the unpacked Golf with its HUNK_SYMBOL blocks (611 code and 544 data
  symbols, the Aztec C build's own names such as `_play_hole`, `_shoot`, `_putt`, `_ball_interrupt`, `_draw_grid`).
  Because the hunks are byte-identical they are attached to 1598's unpacked image as author symbols.
- **Out-of-line blocks.** The compiler placed loop tails and switch arms after a function's return; the recovery splits
  them at their branch targets. 525 such blocks, entered only by branches from one named function, are named
  `<function>_partN` and described deterministically from their calls (rung 3, `frag_g1598.py`), before the model.
- **Code by region.** The sound and music files (GMUSIC, FLYMUS, FXLARGE, FXSMALL) open with a jump table and hold an
  830 to 930-byte player each, followed by music and sample data. The private tools adopt the shared tools' region
  test, so they are recovered as position-independent code units (the game loads them into an allocated buffer and
  calls offsets 0 and 4 from its vertical-blank server). The graphics banks GR2 to GR5 and TR1 to TR4 trip that test on
  pixel data; they are typed as data by the program's use of them (`graf_read` loads them, only the blitter object code
  reads them).
- **Other units.** c/Type, c/endcli, c/loadwb, l/Disk-Validator, libs/icon.library, libs/diskfont.library and the boot
  block are recovered as the system code they are; two 64-byte run-time segments are harness and library stubs.

## Copy protection
None found in the recovered program, checked by reading the code: no disk-register or CIA drive access (the only
custom-chip writes are the blitter object routine, INTENA and the blitter-nasty DMACON bit), no trackdisk.device, no
manual or word lookup (the `menu_word` routines are keyboard navigation of the menu bar), no colour wheel, no checksum
loops over code, no trap-vector tricks (`__trapaddr`/`__oldtrap` belong to the Aztec C maths runtime). The only disk
check is `get_program_disk`, which asks for the PGA TOUR Program disk by volume name when a file is missing; the native
build has no disks. Whether the uncracked original had a disk check that AGL removed before crunching cannot be told
from this release.

## Cracktro
1598's startup-sequence types `s/text` (a 12,456-byte ANSI text page by the cracking group) before running the game. It
is classified as cracktro text (no code); the native build is to show it as its intro.

## Formats found (for the rewrite)
- Full-screen pictures (TITLE.EGA after a 4-byte 320x200 header, PROSHOP.EGA, TENT.EGA): four 8,000-byte bitplanes
  one after another; palettes are 16 12-bit words in the data hunk (`__title_pallette`, `__proshop_pallette`,
  `__new_pallette`, `__old_pallette`).
- Object banks (GR2, GR3, GR4, TR1 to TR4), read by `graf_read`: u16 LE count, u32 LE total size, u16 LE largest object,
  count u16 LE sizes, then objects of u16 LE height, width and bytes per row, rows of 4-bit pixels two to a byte (high
  nibble first). GR2 holds the golfer's swing (10 frames, 56x74) and putting stroke (8 frames, 67x37) and the pro shop
  backdrop; TR banks hold four tree kinds in 12 distance sizes each. GR5 is a planar bank addressed by the object table.
- Mountains (`<course>.MTN`): u16 width, u16 height, then three 4-plane strips drawn at the horizon by view angle.
- Courses (`<course>.CRS`, little-endian, PC layout): 208-byte header (course name, tree set, tournament name, per
  hole par and two tee yardages, 19 hole offsets), then per hole a tip text and a record list: types 0 to 9 ground
  polygons by surface (6 is water, drawn by `clip_ocean`), $10 to $12 tee, pro tee and reference point, $13 to $16 the
  pin for each tournament day, $20 to $23 trees, $30 and up contour lines (green slopes for `get_contour`), -1 end.
- Rendering: `transform` rotates by the view's heading (1.15 fixed point) and a fixed pitch of about 11 degrees;
  `perspect` projects with x*480/z + 150 and y*545/z + 149 onto the 320x200 view, sky above line 44.
- `<course>.PAK` is LZSS-packed (4 KB window, `decode`/`lzw_picture`), 4-byte length first.
