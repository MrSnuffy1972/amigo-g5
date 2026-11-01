# Game 1868: Flashback (Delphine Software / U.S. Gold, 1992), native port

Batch 2 game, owned by the b2-1868 agent (Claude, p1868). Variant 1868: the French release in the Interpol crack (the
plain dump; 1870 and 1871 are alternate dumps of the same crack, 1872 adds a two-option trainer).

## Status (2 Nov 2026)

| stage | state |
|---|---|
| 1. Recover 100% of the code | proofs 1 and 2 pass; proofs 3 and 4 wait for the naming pass (see g1868-recovery.md) |
| 2. Understand | started: program shape, timing, input, screens, file formats, level loading, protection |
| 3. Native rewrite | not started (no crate yet) |
| 4. Verify | reference harness fixed and recording the real game (cracktro, protection, cutscenes, title, level 1) |
| Release | none yet |

## What the game is (from the recovered code)

- Hand-written 68000 assembly that takes the machine over (its own interrupt handlers, blitter, copper and keyboard)
  and hands it back to the operating system only to read a file. Global state lives in one large block of memory;
  tables and texts sit inside the code.
- The engine is Delphine's Flashback engine: a level is a set of rooms; every actor and switch is a scripted object
  with its own behaviour; four sprite lists are drawn back to front each frame; story scenes are polygon cutscenes.
- Timing: the game logic runs every second vertical blank, 25 Hz on PAL, measured in the reference runs; the frame
  wait swaps two screen buffers. Random numbers come from a 32-bit shift-and-XOR generator.
- Screens are 320 by 224 pixels: 16 colours in play, 32 for the title picture. Text uses an 8 by 8 font.
- Files: the data files are packed with a ByteKiller-style coder that the game depacks itself; music is 21 ProTracker
  modules (so the engine's tracker plays them); samples come in per-level sample banks; 33 cutscene files are named
  (four of them are not on the disks); 7 level slots: the jungle, New Washington (two parts), the Death Tower, two
  Earth levels and two parts on the alien planet.
- The game is French (texts in frcine.txt, AZERTY key tables).

## Copy protection and the cracktro

- Protection: a code-wheel check before the logos. The game shows a random wheel shape and a six-character code and
  expects the answer from the wheel; three wrong answers end in a deliberate crash. The Interpol crack turned the
  comparison into a no-op, so any six characters pass. The native build removes the check completely (no screen).
- No disk-based protection: every file is read through the operating system by name.
- Cracktro: Interpol's intro (logo blits and an 8 by 8 scroller) runs before the game from the startup-sequence. It
  is packed in its own way; the code was unpacked and recovered as its own unit. It will be ported natively as the
  intro piece. There is no trainer in this variant, so there is no cheats menu.

## Reference harness

The project's reference harness could not start the game. Fixes in a copy outside the repo (Ryzen,
/srv/fast/native-work/1868/prekit): the packed game is loaded in its unpacked form (the harness's stand-in
operating system cannot host the depacking library's patch), the keyboard interrupt is enabled as the Amiga's
start-up code does, files of disks 2 to 4 are served without a disk prompt, and scripts can press any key. With
these the reference runs reach the cracktro, the protection screen, the logos and intro cutscenes, the title menu,
the options and password screens and level 1 play.

## Port plan (each part committed when it builds, passes its checks and matches the reference)

1. Assets and pack: an extractor outside the repo writes the native asset pack (font, title picture, sprites, level
   rooms and objects, cutscenes, texts, music, samples).
2. Front end: the cracktro as the intro piece, the title, options and password screens, and the cutscene player.
3. The level engine: rooms, sprite lists, Conrad's movement and actions, collisions, object scripts, inventory, texts,
   sound effects and music.
4. All levels, endings, game over and passwords.
5. Verification against the reference runs (frames and named state per 25 Hz tick, the random seed), a real play
   test, then the Windows and Android releases.
