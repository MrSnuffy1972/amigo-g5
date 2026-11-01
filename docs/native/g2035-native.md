# Game 2035 (Sensible Soccer, Crack Inc crack): native port notes

Written 1 Nov 2026 by the p2035 agent (batch 2). Status: step 1 (code recovery) done apart from naming, step 2 (understanding)
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
