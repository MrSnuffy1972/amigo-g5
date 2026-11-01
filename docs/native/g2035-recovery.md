# Game 2035 (Sensible Soccer, Renegade / Sensible Software 1992, Crack Inc crack): code recovery summary

Step 1 of the native-port process. Source, disassembly, data and assets stay on Ryzen (/srv/fast/amigo/recovery/02035); this page has numbers and names only. Recovered by the p2035 agent on 1 November 2026 with per-game tools (tools-p2035), region based: containers come from the boot structure and the game's own file system, never from opcode density.

## Variant

2035 ([cr Crack Inc], two disks) is kept. 2036 ([cr FLT]) is another crack of the same release; 437 is the later International Edition and 203 the 92/93 WHDLoad update, both different programs. 2035 boots, plays full matches in the reference harness and holds every build of the program (slow, chip, fast RAM and 512K) plus the DOS helper code.

## Proofs

| proof | result |
|---|---|
| Bytes classified | PASS: 100.000% of 1802240 disk bytes (2 disks) classified, 0 unknown; 9 code containers |
| Byte-identical reassembly with vasm | PASS: 9 of 9 units identical |
| Functions named and described | PASS: 2470 / 2470 named and described (902 by a model or by hand, 1568 inherited from the same routine in soccers) |
| Executed addresses (reference recordings) in recovered code | PASS: 13668 / 13668 (100.00%) inside recovered code (11 stand-in OS addresses of the harness left out); 0 of them in functions not yet described |

Names and descriptions so far: 1568 inherited, 419 muse-glimmer:30b, 415 qwen3.6:27b, 54 hand, 7 muse-glimmer:30b (name made unique), 7 qwen3.6:27b (name made unique).

## Disk bytes by class

| disk | class | bytes |
|---|---|---|
| d1 | asset | 376678 |
| d1 | code | 264912 |
| d1 | data | 234626 |
| d1 | padding | 116 |
| d1 | unused | 24788 |
| d2 | asset | 528702 |
| d2 | code | 80720 |
| d2 | data | 199634 |
| d2 | unused | 92064 |

## Code units

| unit | what | bytes | instructions | code bytes | functions | vasm |
|---|---|---|---|---|---|---|
| bootblock | disk 1 boot block (crack): reads the crack loader, then the start-up program file | 1024 | 23 | 118 | 2 | identical |
| crackloader | crack loader: hardware set-up, StoneCracker 3.10 depacker (copied to $100), packed intro | 2560 | 210 | 476 | 2 | identical |
| cracktro | Crack Inc intro: credits text screen (unpacked image) | 2684 | 301 | 1456 | 5 | identical |
| soccer | start-up program: memory check, loading and credit pictures, Sensi disk loader, picks the build | 20948 | 3344 | 14320 | 124 | identical |
| soccers | main program, slow-RAM build (the one the reference runs) | 156722 | 20733 | 83328 | 581 | identical |
| soccerc | main program relocated for 1 MB chip RAM | 156722 | 20738 | 83336 | 583 | identical |
| soccerf | main program relocated for fast RAM | 156722 | 20737 | 83334 | 583 | identical |
| socceru | main program, separate build for 512K machines (disk 2) | 153654 | 19500 | 77508 | 533 | identical |
| doscode | AmigaDOS helper code for saving to a DOS disk (disk 2) | 3266 | 1043 | 3212 | 57 | identical |

## Disk layout (both disks: "Sensi disk" file system)

- Sector = 510 data bytes + a 2-byte link (track * 256 + sector, 0 = last). Directory at track 2 sector 2 ("Sensi disk Directory": 32-byte entries with name, length, load address and first track/sector). Disk 1: 21 files; disk 2: 53 files.
- Disk 1 track 0: the crack's boot block, the crack loader and the packed intro (the rest of track 0 is zero). The boot block reads the start-up program file by disk offset; from then on the game's own MFM loader reads files by name (it re-reads the directory track before each file).
- Files: the four builds of the main program and the start-up program (code); IFF ILBM pictures (loading and credit screens, menu and team graphics); IFF 8SVX crowd and whistle samples; two music modules; pitch maps; team databases (clubs, nations, custom); cup and league templates; round-name texts.
- Bytes after each file end in its last sector, the link words, the directory, a sector allocation table and the free sectors are classified as such.

## Copy protection found (read from the code)

- Rob Northen Copylock (original): the crack removed its body. In the main program the call is replaced by a routine that plants the values a passed check leaves (the serial key in the spurious-interrupt vector and two key longwords); in the start-up program the call site is padded out and the key written directly. Native port: not ported.
- Key-decoded jumps (checksum traps): five places in each 1 MB build (two found so far in the 512K build) compute their next jump target as key XOR constant, so a wrong key jumps into nowhere. In the port these are plain calls to the routine the correct key selects.
- Searched and not found: manual or word lookups (no such text or screen), other disk-track checks (the loader reads only standard sectors with sync $4489), checksums over code.

## Cracktro and trainer

- Cracktro: the Crack Inc credits text screen, unpacked from a StoneCracker 3.10 stream (model checked against RAM). To be kept as a native intro.
- No trainer in this variant.

## Reference recordings

A private copy of the project reference harness (frozen G5 build untouched) with both disks attached to the G0 snapshot boot, plus a cold-boot recording from the boot block (crack loader, intro, start-up program): 14 recordings (boot, menus, CPU v CPU friendlies, player v CPU friendlies with random play, random menu walks) give 13668 distinct executed game addresses, all inside recovered code.

## Functions

2470 functions; table (unit, address, name, description) on Ryzen in /srv/fast/amigo/recovery/02035/functions.md. soccerc and soccerf functions take the name of the same-offset soccers function; socceru functions that match a soccers function instruction for instruction (addresses and branch distances aside) take its name; the rest are named by the local models (tools-p2035/describe2035.py, Ryzen muse-glimmer) and checked by hand.

