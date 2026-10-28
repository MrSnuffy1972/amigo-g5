
## Lessons, 15 Oct 2026 evening (Cowork; measured unless marked)
Clean-sheet runner (/srv/fast/cleansheet, tools v0d/v0e):
- Busy-wait yields must only go on loops that WAIT (poll and compare). Yields on work loops turn milliseconds of work into hundreds
  of ticks. v0d rejected loops that store to memory; v0e also rejects dbcc-counted loops, shift/rotate/addx (decruncher bit loops
  store outside the inner loop), subroutine calls and address-register advances (pointer scans). Tested on real snapshots: all 5
  Gemini-named work loops rejected, every known wait loop kept (IK+ $774A, Arkanoid $673AE, Wizball $5E560, Kid Gloves $1902C).
- WHDLoad snapshots go after the slave's startup DiskLoad/LoadFile/Decrunch/Patch burst (WHDTIME=1 on the ref run gives the frame of
  each call; harness SNAP_FRAME). A blanket "frame 12" fallback broke IK+ (snapshot mid-routine at frame 149) and was reverted.
- Snapshot timing is a stopgap: code that appears at run time (boot copies, decrunchers, overlays, resload_Patch) is solved properly
  by instruction-level code versions (FIRSTBYTES (pc,10 bytes)@frame, guarded labels, FINDING + rebuild), design checked 23/23.
- resload slots beyond LoadFile/SaveFile/GetFileSize need native versions (DiskLoad +$28, LoadFileOffset +$4C, Control +$34,
  CRC16 +$30, Patch +$64); the resload base is slave - $100 ($C7E000 for 512 KB slow, $CFF900-$CFFB00 for 1 MB). DiskLoad's disk
  number is the LOW BYTE of D2 (Disk.65537 = $10001 aborts).
- Picture-fail causes so far (44 games): snapshot before loads, yields on work, blitter/sprites/hires/AGA not drawn, level 3
  interrupt never delivered or gated by INTENAR/INTREQR shadows, COPER-only interrupts and INTREQR polls waiting for the copper,
  COP2LC/COPJMP2 display, Kickstart ROM font scans.
Trackloader boot (scratch kit /srv/fast/amigo/_scratch/kit-trackboot, not merged):
- Harness CIA-A PRA cleared then re-set /DSKRDY and /TK0 (chipset.c lines 182-183): no trackloader could ever see a ready drive.
  ADKCON was stored raw instead of set/clear. Both real, both fixed in the scratch kit.
- Gemini's MFM encoder decoded fine but had wrong clock bits (2936 illegal "11" pairs on one track): always check MFM legality,
  not only round-trip decode. CIA-B DDRB/PRB must reset to $FF; DSKBLK must come after the transfer, not at once; the user stack must
  not sit just under the supervisor stack; load the bootblock at $4000 (games clear $64000-$80000); an OS-stub trap must check the
  stub is still there (games copy code over $1000).
- "Settled" must not trigger inside cracktros (Hybris settled in THE LIGHTFORCE): needs a picture check; trainer menus need mouse input.
Gemini and free models:
- Ask Gemini for its WHOLE answer in ONE code block (one copyable window); answers copied via Word collapse runs of spaces, so check
  quotes with whitespace normalised; replaced "old" lines may span several lines. Gemini share links cannot be fetched.
- Gemini quota runs out without warning (Karl, 15 Oct); Flash Lite answers must be re-run on a stronger model before relying on them.
- Free OpenRouter thinking models (Nemotron Ultra/Super) spend the whole budget reasoning on code jobs: max_tokens 2048 gives an
  empty body; at 24000 Ultra wrote 71 KB of reasoning and no code in 831 s. Gemma/Laguna hit HTTP 429. Use them for short source
  reviews, not for writing code.
Operations:
- PowerShell mangles inline python, quotes, pipes into head/grep and $variables: write a script, scp it, run it.
- A planned container stop gets undone by oc-eventwatch (container.start rail): rename the container to retire it.
- pgrep -f inside docker exec sh -c matches itself ("no process?" was a false alarm; the save happened).

## 18 Oct 2026 (Cowork kd2): keyboard-only deploy and keyed discovery
- Filtering a diff by dropping whole hunks can drop wanted lines that share a hunk. The first keyboard-only cut dropped intentrt.c
  chip_reset's icrmask 0x08 with the chipset_restore_cia hunk, so the port never took the level-2 keyboard interrupt while the
  reference did (114: reference left the title, port did not; port read SDR 0 times). Caught by a positive input check, not by the
  no-input check (no input cannot see a missing keyboard path). Always run the positive check before calling a deploy done.
- native_discover "rc 1" with no new round line is the round that found 0 interpreted addresses (grep -c exits 1 under set -e):
  discovery converged, it did not fail. probe.txt with interp_pc=0 confirms it.
- Keyed coverage (cs_game.sh v1b): games with a cs_inputs.json sequence also run the reference with the script and append the
  first-execution bytes of PCs only the keyed run reached to first_bytes.txt (no-input lines first and unchanged), and run
  native_discover twice (no input, then keyed) into the same seed file. 196: untranslated 0608A2 at tick 27 -> 600 frames, picture
  and sound PASS (JEV dissent). 261: untranslated 028022 at tick 577 (code loaded after fire, translated from snapshot bytes) -> 600
  frames, picture diverges from tick 579. Games without a sequence take the old path unchanged (23, 119, 266 identical).
- 78 (Budokan) CFFF2A is WHDLoad SLAVE code at the top of the 1 MB slow RAM ($CFFC00-), not ExpMem (ExpMem $81000 at $C00000).
  cs_snapcfg.py clips code ranges to $C80000, so the translator calls it "out of code range". Needs the clip raised where
  m68k2c's memory window allows (mem_sizes() now follows SLOW_SIZE); cs_snapcfg.py was left untouched on Karl's instruction.

## 18 Oct 2026 (Cowork filedboot): filed games black from frame 0
- Clean-sheet snapshots keep the RAM but not oshle.c's C statics, and cs_snapcfg.py renames oshle's game_boot_extra out of the
  snapshot build, so in every filed native exe chip_next and slow_next started at 0: AllocMem returned address 0 and its clear wiped
  $0-$size (AbsExecBase, vectors), then Alert or a spin before the first custom-chip write. Measured with --log on 7, 41, 412, 498,
  507 ("HLE AllocMem $78 flags $10000 -> $000000", "AbsExecBase ($4) is $00000000"). Fix in oshle.c (deployed 15:03): the
  allocator pointers are mirrored in emulated RAM at $3FE0/$3FE4 with magic 'OSHL' at $3FE8, read back on the first alloc; with no
  mirror they start at oshle_chip_lo/slow_lo. Harness RAM changes only in those 12 bytes (measured on 147 at frame 300: 7 bytes, all
  in $3FE0-$3FEF); 19 passes and 10 WHDLoad non-passes identical in frames and RAM. Staged before/after: 498 picture 9.3% -> 99.7%
  of 342 frames (then the 900 s run timeout), 507 45.0% -> 75.7%, 7 black -> 23 perfect frames then $80D0C (the cs_snapcfg code
  clip at $80000), 490 0 -> 1779 custom writes then a jump to $DFF000, 412 0 -> 8 writes (yields "ACCEPT: unverified" on its text
  loop at $4966 slow it). Other oshle statics (open files, locks, disk libraries) are still lost at a snapshot taken after F=0.
- intent.c: a copper INTREQ move before the list's first WAIT has line 0 and was never delivered (tick_line_step checks after
  line++). Only Cannon Fodder (80, MOVE $8004 = SOFT, its frame flag $8060E) and Kid Gloves (275, $8014) have one in their
  snapshot lists. Fixed in intentrt.c (deployed 15:07, fires them with VERTB): 80 picture 13.3% -> 88.3% of ticks, active ticks
  18.2% -> 75.0%, custom writes 0 -> 6845; its next blocker is resload asking for '1.RAW', not embedded in the native.
- cs_snap_override matters for comparisons: 449 was judged with "frame 150"; a staged run without it snapshots at the boot entry.
- Picture-fail causes found in this group that are not the allocator: 449 hires (BPLCON0 $A200, not composed), 80 the line-0
  copper interrupt, 124 old run (yields on decruncher bit loops, since rejected by v0e), 69 now 96.5% picture with few active ticks.

- 19 Oct 2026 (c19r): a workspace option file's MERE PRESENCE is a value. cs_game.sh treats any /w/cs_input_mode content (even "null") as attract-only and cs_pickpc.py any /w/cs_snap_override content as an override, so never write "null" placeholder files there; keep null in the identity instead. Also: importlib.reload() re-finds a module by sys.path, so a test that loads a COPY must put the copy first. See C19-INSTALL-NOTES.txt.
