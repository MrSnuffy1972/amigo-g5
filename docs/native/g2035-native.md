# Game 2035 (Sensible Soccer, Crack Inc crack): native port notes

Written 1 Nov 2026 by the p2035 agent (batch 2). Status: step 1 (code recovery) done, all four proofs pass (2 Nov 2026); native parts 1 (intro), 2 (language
screen, main menu) and 3a (pitch, camera) done and verified; the rest of the match engine and the release not started. Recovery summary: [g2035-recovery.md](g2035-recovery.md).

## Measured so far

### Boot and program structure
- Disk 1 boot block (crack) reads the crack loader to $60000; the loader copies a StoneCracker 3.10 depacker to $100 and unpacks the
  Crack Inc intro to $30000 (a static credits text screen, shown for about 70 frames in the reference boot), then the boot block reads
  the start-up program `soccer` to $70000.
- `soccer` checks memory, shows the loading pictures and credits (IFF ILBM files `loader1-3`, `loaderbg`, `credit3-13`), and loads one of
  four builds of the main program by name with its own MFM loader (standard AmigaDOS sectors, sync $4489, Sensi disk file system):
  `soccers` ($C00000, slow RAM, the reference build), `soccerc` ($80000, 1 MB chip), `soccerf` ($200000, fast RAM, the same program
  relocated) and `socceru` (disk 2, $4000, a separate 512K build).
- Disk 2 holds the run-time data: team databases (`Clubs`, `Nats`, `Cust`), pitch maps, menu and team graphics, crowd and whistle
  samples (IFF 8SVX), the menu music, cup and league templates and round-name texts. The game asks for disk 2 when it needs it.

### Game flow (reference harness, scripted joystick)
- Language screen (English, Deutsch, Francais, Italiano) -> main menu (Options, Friendly, Edit Teams, Cup, Load/Save Data, League,
  Highlights, Specials). Menus are joystick driven: a flashing white border marks the selected button; one stick push moves one button,
  fire activates. The cursor starts on Options in the main menu and on "Choose friendly match teams" in the Friendly screen.
- Team selection: fire on a team cycles off -> computer (red C) -> player (blue P). With a player team, the team management screen
  (squad list, formations 4-4-2, 5-4-1, 4-5-1, 5-3-2, 3-5-2, 4-3-3, attack, defend; Play Game, View Oppo, Undo) comes before the match.
- Match: "Friendly" title card, then the scrolling pitch (overhead view, 50 Hz), automatic replays ("R"), half time, full time. A
  3-minute CPU v CPU friendly runs past 16,000 frames in the reference harness.

### Copy protection (to be removed in the port)
- Rob Northen Copylock, removed by the crack and replaced by code that plants the passed-check values; five key-decoded jumps in the main
  program (target = key XOR constant). The port calls the routines the correct key selects; nothing reads a key.

## Plan for the rewrite (rust/games/g2035 on rust/native_engine)
1. Pack extractor (outside the repo): IFF ILBM pictures, 8SVX samples, the two modules (Sensible's own replayer format, not ProTracker:
   needs its own replayer port), team databases and pitch maps into native-asset-pack/1.
2. Phases from the recovered main loop: intro (the Crack Inc screen), loading/credit pictures, language, menus, team selection and
   management, match, replays, competitions, editor, load/save (to the engine's save store instead of disks).
3. Match engine from the recovered routines: players, ball physics with after-touch, referee and rules, tactics tables, computer play,
   then the competitions and the editor.
4. Verification: the scripted inputs above through the reference harness copy and the headless build; named state from the routines' memory.

## What remains
- Naming: 2470 functions; 2.5k routines minus inherited ones (soccerc/soccerf take soccers names, 406 socceru functions match soccers
  instruction for instruction) leave about 900 for the local models; the naming pass is slow because several agents share the one Ryzen GPU.
- Steps 2 to 4 and the release as above.

## Port parts (rust/games/g2035)
| Part | What | State |
|---|---|---|
| 1 | Extractor and asset pack (outside the repo), crate, the Crack Inc intro | done 2 Nov 2026 |
| 2 | Language screen and main menu: sprite cut lists, fonts, language texts, menu engine, fades | done 2 Nov 2026 |
| 3a | Match pitch, pitch types, random numbers, camera | done 2 Nov 2026 |
| 3b | Ball physics and after-touch, players, referee, set pieces, computer play, timing, sound; match flow from the menu | next |

### Part 1 (2 Nov 2026)
- Extractor `/srv/fast/native-work/2035/tools/extract2035.py FILES PACK` (pack at `/srv/fast/native-work/2035/pack`, 101 assets):
  IFF ILBM pictures to image + palette (25), IFF 8SVX samples to s8 with rate and loop (21), the two FLT4 modules as
  ProTracker modules (tag set to M.K.; the layout is the same), every other file as a raw `blob` (team databases, which
  are packed, pitch maps, cup and league templates, round-name texts, the program files for their data tables). The menu
  graphics are not in the program: `cjcbits` holds the flags, the pitch radar and the Sensible SOCCER logo, `cjcgrafs` the
  kit sprites, clock digits, HALF/FULL TIME and the small fonts, `cjcteam1-3`/`cjcteamg1` the player sprites.
- Intro: the text and layout of the Crack Inc page (lines separated by zero bytes, ended by $FF; line i at y = 23 + 10i,
  centred; background #550044, ink #FF0077; about 70 frames). The original drew with the Kickstart topaz font, which is not
  shipped: the pack carries an 8x8 font rendered from the repo's VT323 (SIL OFL). Checked against reference boot frame 50:
  every text row matches except the dashed line (topaz dashes are two pixels high, ours one).
- After the intro the original showed a black screen while it read the main program and asked for disk 2; the port shows
  black until part 2's language screen replaces it.
- Checks: `cargo test -p g2035 --no-default-features` (intro timing, skip, centring on a synthetic pack); headless run on
  the real pack (80 ticks): phase 0 for ticks 1-69, phase 1 from tick 70.

### Part 2 groundwork: the menu tables (measured 2 Nov 2026; ported in part 2 below)
- The front end is data driven. A menu descriptor (main program) is: items pointer (long, so far always descriptor + 20),
  two words (1, 1), two handler routine pointers and a third long (zero in both menus found), read by `open_menu_descriptor` ($C1E656), then
  16-byte items until a $FFFF word, then a navigation table (bytes: neighbour item per direction, $FF = none).
- Item: kind (word), x, y, w, fifth word (height for boxes; for text buttons it is $24 on every main-menu button, so not
  the drawn height: still to decode), attribute word (high byte flags, low byte colour, e.g. $8F06), data (long: a text
  pointer for kind $0B, a sprite number in the high word for kind $04, zero for kind $0A).
- Kinds seen: $0A plain box, $04 sprite (numbers $231-$234 the four flags, $235 the Sensible logo, $236 the SOCCER logo, all
  in `cjcbits`), $0B text button.
- Language menu at program address $C1D154: four boxes 80 x 79 at (11,39), (247,39), (11,142), (247,142); the two logo
  sprites; four flags; four labels ENGLISH, DEUTSCH, FRANCAIS, ITALIANO.
- Main menu at $C1CC3C: eight text buttons 130 wide in two columns at x 29 and 179, y 56, 96, 136, 176 (OPTIONS, EDIT
  TEAMS, LOAD/SAVE DATA, HIGHLIGHTS; FRIENDLY, CUP, LEAGUE, SPECIALS) and the two logo sprites.
- Descriptors whose items pointer is descriptor + 20 (found by scanning): $C1CC3C, $C1D154, $C203DE, $C22B5E, $C24B10,
  $C26096. Other menus (options, friendly, team selection) use another layout or are built at run time; still to find.
- Attribute word, matched against the reference menus: bit 15 a flag (set on boxes and buttons), bits 8-11 the border and
  text colour, the low byte the fill colour, in the shared 16-colour menu palette (`Marble` and `cjcbits` carry the same
  CMAP: 5 dark orange, 6 orange, 10 red, 12 dark red, 15 yellow ...). Language boxes $8F06 = orange with a yellow border
  and yellow labels; OPTIONS $8605 = dark orange with orange border and text; EDIT TEAMS $8A0C = dark red with red text.
- Sprite items carry their size in w and h. In `cjcbits` (320 x 256): flags 36 x 24 at Italy (0,0), France (48,0),
  Germany (0,114), England (48,132); the logo strip at (0,156) 255 x 37 (Sensible 128 wide, then SOCCER). The sprite
  number to rectangle table itself is not found yet; until it is, these rectangles are measured from the picture.
- Text and sprites share one system: a run-time table of sprite header pointers (long at $C2642C; header word 8 = width,
  word 10 = height) and `draw_sprite_by_number` ($C042BE). A font is a table whose first word is its base sprite number,
  followed by a glyph index per character code minus 32 (a space advances half the width of entry 33);
  `draw_text_centred_on_x` ($C0A496) and `draw_text_from_x` ($C0A4F6) draw proportional text up to x 336. The glyphs are
  the small fonts in `cjcgrafs` below y 192. The table pointer is set to $708D6 (chip RAM) by $C263A0; the table is
  built at run time (the reference RAM dumps in runs/m12 hold it: entry n = header pointer; header = graphics pointer,
  second pointer, then words 8 and 10). Entries $231-$236 there read 16 x 19, 12 x 15, 13 x 15, so the menu item sprite
  numbers are not plain indices into it: to resolve next, with the routine that draws kind $04 items.
- Plan: the extractor parses the descriptors into JSON (`menu_*` data assets), the crate draws items by kind (box and text
  in the colour from the attribute, sprites cut from `cjcbits` by a sprite table still to locate) and moves the selection
  by the navigation table; each menu's handlers become typed Rust actions.

### Part 2 (2 Nov 2026): language screen and main menu
- Extractor addition `tools/menus2035.py` (outside the repo, run by `extract2035.py`) reads the front end's tables out of the
  main program file: the three sprite cut lists (`sprites`: base set $C131CC, match set $C136CC, menu set $C16FF0; ops
  $FFF9 picture by name, $FFF8 picture via a pointer, $FFFF/$FFFB/$FFFE/$FFFD cut n x y w h [dx dy [mode]], $FFF7 blank work
  sprite, $FFF6 end; interpreter $C08C96), the language and main menu descriptors (`menus`), and the menu tables
  (`menu_ui`: font index table and height $C192D4, colour -> font base $C1F468, bottom-edge shades $C1EC52, cursor flash
  colours $C1E556). 104 assets in the pack now.
- The sprite-number-to-picture table is resolved: it is not a table but these cut lists, run at start-up into chip RAM
  (header: graphics, mask, width, height, words per row, ..., own number; interleaved planes). The menu sprites
  $231-$236 are the flags (England $231 at `cjcbits` 48,132; Germany $232 0,114; France $233 48,0; Italy $234 0,0) and the
  logos (Sensible $235 0,156 128 x 37; SOCCER $236 128,156 124 x 42). Checked against a RAM dump of the language screen.
- The coloured fonts are one glyph set (`cjcgrafs` from y 192, face colour 10, shadow colour 8) cut once per text colour
  (bases $92, $C5, $F8, $12B, $15E, $191, $1C4, $1F7 for colours 1, 2, 5, 6, 7, 10, 13, 15) and recoloured: the port draws
  the one set with colour 10 replaced.
- Language texts: the four `textN` files are the program's English block at $C1B750 (2844 bytes) in each language; the
  original copied the chosen one over it. Main menu labels are offsets into that block, so all four languages work
  (German shown natively; lower-case codes are the accented capitals through the font index table).
- Menu engine ported from the code: item drawer $C1EC62 (fill unless bit 7 of the low byte; framed items: ring 2 pixels
  in, top edge in the frame colour, bottom edge in the shade; kind $04 sprite at its byte offsets, $0B text centred
  vertically by (h - 11) / 2 rounded up, in the frame colour's font; glyphs advance width - 1, a space half an `A`); grid
  walk $C1EA4E with the walk codes -2 to -10; menu loop $C1E2EE (cursor = one-pixel outline in flash colour
  [3,3,3,1,1,2,1,1] by counter / 2; 4-frame move wait; fire wait set to $FF on entry, 8 after a press).
- Measured on the reference and ported: the display starts one pixel right; colours are the IFF CMAP's high nibbles
  (#101020 shows as #111122); sprites blit whole 16-pixel words (the SOCCER logo shows 4 more columns of its picture);
  after a move the loop skips one pass (held stick repeats every 6 frames) and the cursor is erased for one frame;
  menus open with 7 set-up frames (fade-in 0, 0, 0, 2, 4, 8, 16 sixteenths, cursor from the 7th); after an activation the
  screen fades out (16, 16, 8, 4, 2, then black) and the next menu opens on the 9th frame; the frame counter $C0DE6A is
  cleared only at $C1D2AA and $C1D2CA, so the main menu's flash carries on from the language screen's counter.
- Front-end flow $C1D25E: attract mode with 1500-frame (then 375) time-outs, language -> main menu, main menu choices
  dispatch by $C1D332 (Options, Edit Teams, Load/Save, Highlights, Friendly, Cup, League, Specials); the choices are a
  black placeholder until their parts (back or fire returns, the cursor kept as the original keeps it).
- Verification (harness snapshot runs with scripted joystick, every frame; native headless with the same input,
  native tick = reference frame + 71): walk3 (language moves, English, main menu moves) 435 of 435 frames pixel-exact;
  walk5 (repeats, diagonal, main menu walk) 368 of 368 up to the Friendly choice; walk4 (Deutsch) 88 of 88 up to the
  choice, after which the reference harness never loads the German text from disk 2 (stalls at $C020E8).
- Not verified: the attract path. The snapshot starts with attract mode off ($C1D732 = 0 in its RAM), so its language
  screen never times out; the port follows the code (no fade on the attract changes). An idle reference run also shows
  the language cursor frozen from frame 1510 to about 1890 with the menu still current (counter running): cause unknown,
  not ported. The attract demo match needs the match engine; until then the attract main menu's time-out goes back to
  the language screen. The loading pictures and credits shown while the original loads are not ported (the cold-boot
  recordings do not reach them).
- Checks: `cargo test -p g2035 --no-default-features` (11 tests on a synthetic pack: timing, repeats, fire release,
  fades, attract time-outs, German text, item drawing, grid walk); verification tools outside the repo
  (`tools/cmp2035.py`, `vfy2035.py`, `scr2035.py`).

### Part 3a (2 Nov 2026): the pitch, pitch types, random numbers and the camera
- Pitch files (`pitch1.map`, `pitch2.map`): a map of 38 x 46 big-endian longs (byte offsets of 128-byte tiles), then the
  tiles (16 x 16, four bitplanes interleaved by row; 128 tiles in pitch1, 160 in pitch2). Measured: every marking is in
  the tiles (the RAM copy at frame 3000 of a CPU v CPU match is the file plus $5CA8, tiles unchanged); nothing is drawn
  over them. The original scrolled a 352 x 272 ring buffer ($C0343A, $C0371C blit a tile row or column as the camera
  crosses a tile edge); natively the view is read from the map. `src/pitch.rs`.
- Pitch type word $C0DF72: low byte = type 0 to 6, high byte = pitch file. Chosen before the match ($C0B3FA): month row of
  chances ($C09962, 12 x 7 percentages; -1 = default row $C099B6; -2 to -8 = type month + 8), the random number scaled
  to 0..99 walks the row; bit 4 of the frame counter picks the file. The type sets palette colours 0, 7 and 9 from
  $C09946 ($C09BAC) and three ball-physics words ($C0D438, $C0D41C, $C0D42A; to decode with the ball).
- Random numbers ($C0DFAC): seed byte = seed * 5 + $81, XOR a byte of the program's own code at $C0F2B6 indexed by
  (frame counter + calls) & $3FF. The pack carries those 1024 bytes. `src/rng.rs`.
- Camera ($C090C6, `src/camera.rs`): target = ball - (168, 136) plus a look-ahead that moves 2 pixels a frame up to 40
  towards the ball's velocity (ball loose, $C0E8AA = 100) or the controlled player's facing; fixed view, forced focus and
  set-piece events ($15-$18, $19-$1C) override; eases (distance / 4) * $4000 per frame in 16.16, at most 5 pixels;
  x 16..256 (0..272 without margin), y 9..452.
- Display: screen columns 1 to 336 show pitch column camera x - 1 + column (column 0 and 337-351 colour 0); all 272
  rows; the shown frame uses the previous frame's camera (double buffering), measured over frames 1991-2010.
- Extractor addition `tools/match2035.py` (outside the repo) writes `match_tables` (palette, grass sets, chances,
  random table, facing table, the per-type physics words, the two map names). 105 assets.
- Reference harness finding: on the match screen the harness shows bitplane 4 wrongly (grey bands; planes 0-2 match,
  plane 4 is the map's in the game's own RAM buffer), so frame checks compare planes 0-2 until the harness is fixed.
- Verification (`p3/vfy3a.py` outside the repo, recording runs/cvc1 with RAM dumps at frames 1990-2010, through the
  `matchcheck` example): camera 20 of 20 frames exact in 16.16 position and look-ahead from the previous frame's ball
  state; pitch view planes 0-2 equal on 99.33% of 1.9 M pixels (the rest are players, ball and shadows, not yet ported).
  Not verified yet: the pitch-type choice on a recording (the choice runs before the match; needs a RAM dump there).
- Checks: `cargo test -p g2035 --no-default-features` adds 5 tests (random routine, pitch decoding, type choice,
  camera, view mapping) on synthetic data.

## Naming pass (proof 3), 2 Nov 2026
- 902 routines needed names (1568 inherit from the identical soccers routine). The naming ran on Ryzen's muse-glimmer:30b and,
  from 2 Nov, three shards on IntelPC's RTX 3090 (qwen3.6:27b); a last round one routine per request. 14 answers rejected
  only for a name already taken were accepted with the address appended (tools-p2035/accept2035.py). The 11 largest routines
  the models never answered (interrupt handlers, the main loop step, the start-up load sequence, the DOS helpers) and the
  small leftovers were named by hand from the code (hand.json, 54 entries).
- Quality: a spot check found several model names plainly wrong (the IFF picture loader was named as a menu flag setter,
  the sprite drawer as a team pointer fetch, the file cache routines as team look-ups); those are corrected in hand.json. A
  deterministic check of names against hardware use (tools-p2035/check2035.py) flagged 18 of 547, mostly harmless. Treat
  model names outside hand.json as hints until the routine is read for the port.
