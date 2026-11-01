# Game 2035 (Sensible Soccer, Crack Inc crack): native port notes

Written 1 Nov 2026 by the p2035 agent (batch 2). Status: step 1 (code recovery) done, all four proofs pass (2 Nov 2026), step 2 (understanding)
started, steps 3 and 4 (native rewrite, verification) and the release not started. Recovery summary: [g2035-recovery.md](g2035-recovery.md).

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
| 2 | Language screen and main menu (Marble background, `cjcbits` flags and logo, `cjcgrafs` font) | next |

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

### Part 2 groundwork: the menu tables (measured 2 Nov 2026, not yet ported)
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
