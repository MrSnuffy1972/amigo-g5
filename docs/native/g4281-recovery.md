# Game 4281 (Lollypop v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04281). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (8025794 top-level bytes in 228 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 5 of 5 named (0 author symbols), 5 of 5 described (1 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 477050 bytes, 3 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8731 of 8731 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11421 |
| code | 16 |
| data | 8014352 |
| padding | 5 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 6 | 2 |
| load (raw code file, load address unknown) | 1 | 0 | 0 |
| 1C#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| 6B#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| 6D#unpacked (raw code file, load address unknown) | 1 | 4141 | 1 |
| 6F#unpacked (raw code file, load address unknown) | 1 | 4141 | 1 |
| 70#unpacked (raw code file, load address unknown) | 1 | 443 | 1 |

## Cases handled

- Lollypop_v1.0/Lollypop_v1.0.hdf: hard-disk image (AmigaDOS OFS, 8864 blocks): its filesystem parses and all 118 files in it are byte-identical to the extracted tree Lollypop_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/01: Imploder Data (xfd)-packed, unpacked with ancient (8994 -> 18284 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/02: Imploder Data (xfd)-packed, unpacked with ancient (20314 -> 54484 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/03: Imploder Data (xfd)-packed, unpacked with ancient (22676 -> 54484 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/04: Imploder Data (xfd)-packed, unpacked with ancient (89288 -> 154700 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/05: Imploder Data (xfd)-packed, unpacked with ancient (51178 -> 88816 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/06: Imploder Data (xfd)-packed, unpacked with ancient (54636 -> 75792 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/07: Imploder Data (xfd)-packed, unpacked with ancient (42258 -> 50166 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/08: Imploder Data (xfd)-packed, unpacked with ancient (7216 -> 8484 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/09: Imploder Data (xfd)-packed, unpacked with ancient (27636 -> 33526 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/0A: Imploder Data (xfd)-packed, unpacked with ancient (24056 -> 53952 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/0B: Imploder Data (xfd)-packed, unpacked with ancient (45088 -> 89508 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/0C: Imploder Data (xfd)-packed, unpacked with ancient (29486 -> 59638 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/0D: Imploder Data (xfd)-packed, unpacked with ancient (11820 -> 23392 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/0E: Imploder Data (xfd)-packed, unpacked with ancient (12204 -> 23392 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/0F: Imploder Data (xfd)-packed, unpacked with ancient (11492 -> 23392 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/10: Imploder Data (xfd)-packed, unpacked with ancient (12468 -> 23392 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/11: Imploder Data (xfd)-packed, unpacked with ancient (11054 -> 23392 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/12: Imploder Data (xfd)-packed, unpacked with ancient (11254 -> 23392 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/13: Imploder Data (xfd)-packed, unpacked with ancient (10302 -> 23392 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/14: Imploder Data (xfd)-packed, unpacked with ancient (11642 -> 23392 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/15: Imploder Data (xfd)-packed, unpacked with ancient (99176 -> 155410 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/16: Imploder Data (xfd)-packed, unpacked with ancient (20264 -> 27440 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/17: Imploder Data (xfd)-packed, unpacked with ancient (3268 -> 25632 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/1B: Imploder Data (xfd)-packed, unpacked with ancient (86700 -> 126454 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/1C: Imploder Data (xfd)-packed, unpacked with ancient (13762 -> 34824 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/1D: Imploder Data (xfd)-packed, unpacked with ancient (115142 -> 173580 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/1E: Imploder Data (xfd)-packed, unpacked with ancient (47272 -> 91666 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/1F: Imploder Data (xfd)-packed, unpacked with ancient (48590 -> 93944 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/20: Imploder Data (xfd)-packed, unpacked with ancient (13324 -> 15220 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/21: Imploder Data (xfd)-packed, unpacked with ancient (10284 -> 14980 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/22: Imploder Data (xfd)-packed, unpacked with ancient (20156 -> 47656 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/23: Imploder Data (xfd)-packed, unpacked with ancient (17278 -> 36840 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/24: Imploder Data (xfd)-packed, unpacked with ancient (23048 -> 57406 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/25: Imploder Data (xfd)-packed, unpacked with ancient (3034 -> 24896 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/26: Imploder Data (xfd)-packed, unpacked with ancient (95036 -> 154764 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/27: Imploder Data (xfd)-packed, unpacked with ancient (35104 -> 52270 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/28: Imploder Data (xfd)-packed, unpacked with ancient (43928 -> 57584 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/29: Imploder Data (xfd)-packed, unpacked with ancient (35694 -> 99386 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/2A: Imploder Data (xfd)-packed, unpacked with ancient (44628 -> 59128 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/2B: Imploder Data (xfd)-packed, unpacked with ancient (23736 -> 34550 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/2C: Imploder Data (xfd)-packed, unpacked with ancient (31154 -> 42066 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/2D: Imploder Data (xfd)-packed, unpacked with ancient (26788 -> 59404 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/2E: Imploder Data (xfd)-packed, unpacked with ancient (47774 -> 137762 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/2F: Imploder Data (xfd)-packed, unpacked with ancient (110210 -> 193490 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/30: Imploder Data (xfd)-packed, unpacked with ancient (23276 -> 46014 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/31: Imploder Data (xfd)-packed, unpacked with ancient (15524 -> 33854 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/32: Imploder Data (xfd)-packed, unpacked with ancient (16474 -> 34698 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/33: Imploder Data (xfd)-packed, unpacked with ancient (25338 -> 40192 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/34: Imploder Data (xfd)-packed, unpacked with ancient (20848 -> 28422 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/35: Imploder Data (xfd)-packed, unpacked with ancient (28436 -> 41052 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/36: Imploder Data (xfd)-packed, unpacked with ancient (41530 -> 58288 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/37: Imploder Data (xfd)-packed, unpacked with ancient (19976 -> 48928 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/38: Imploder Data (xfd)-packed, unpacked with ancient (34800 -> 87042 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/39: Imploder Data (xfd)-packed, unpacked with ancient (33618 -> 65968 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/3A: Imploder Data (xfd)-packed, unpacked with ancient (73842 -> 122884 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/3B: Imploder Data (xfd)-packed, unpacked with ancient (34552 -> 56790 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/3C: Imploder Data (xfd)-packed, unpacked with ancient (33158 -> 51352 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/3D: Imploder Data (xfd)-packed, unpacked with ancient (23766 -> 37006 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/3E: Imploder Data (xfd)-packed, unpacked with ancient (53008 -> 78664 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/3F: Imploder Data (xfd)-packed, unpacked with ancient (38362 -> 56362 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/40: Imploder Data (xfd)-packed, unpacked with ancient (10492 -> 14340 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/41: Imploder Data (xfd)-packed, unpacked with ancient (12460 -> 19724 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/42: Imploder Data (xfd)-packed, unpacked with ancient (16610 -> 23140 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/43: Imploder Data (xfd)-packed, unpacked with ancient (30526 -> 61166 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/44: Imploder Data (xfd)-packed, unpacked with ancient (73908 -> 155238 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/45: Imploder Data (xfd)-packed, unpacked with ancient (52588 -> 92032 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/46: Imploder Data (xfd)-packed, unpacked with ancient (45360 -> 59452 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/47: Imploder Data (xfd)-packed, unpacked with ancient (11678 -> 31494 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/48: Imploder Data (xfd)-packed, unpacked with ancient (18884 -> 42664 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/49: Imploder Data (xfd)-packed, unpacked with ancient (2220 -> 5536 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/4A: Imploder Data (xfd)-packed, unpacked with ancient (1884 -> 4856 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/4B: Imploder Data (xfd)-packed, unpacked with ancient (2672 -> 7192 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/4C: Imploder Data (xfd)-packed, unpacked with ancient (2936 -> 8556 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/4D: Imploder Data (xfd)-packed, unpacked with ancient (3014 -> 7510 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/4E: Imploder Data (xfd)-packed, unpacked with ancient (1356 -> 3266 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/4F: Imploder Data (xfd)-packed, unpacked with ancient (2292 -> 5618 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/50: Imploder Data (xfd)-packed, unpacked with ancient (1742 -> 4498 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/51: Imploder Data (xfd)-packed, unpacked with ancient (16770 -> 29590 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/52: Imploder Data (xfd)-packed, unpacked with ancient (14600 -> 29060 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/53: Imploder Data (xfd)-packed, unpacked with ancient (14786 -> 28142 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/54: Imploder Data (xfd)-packed, unpacked with ancient (6568 -> 10872 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/55: Imploder Data (xfd)-packed, unpacked with ancient (12442 -> 19592 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/56: Imploder Data (xfd)-packed, unpacked with ancient (3834 -> 6440 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/57: Imploder Data (xfd)-packed, unpacked with ancient (11754 -> 16832 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/58: high-entropy data file (7.86 bits/byte), classified as packed data
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/59: Imploder Data (xfd)-packed, unpacked with ancient (26716 -> 44042 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/5A: Imploder Data (xfd)-packed, unpacked with ancient (19958 -> 54484 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/5B: Imploder Data (xfd)-packed, unpacked with ancient (100038 -> 163154 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/5C: Imploder Data (xfd)-packed, unpacked with ancient (36288 -> 72572 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/5D: Imploder Data (xfd)-packed, unpacked with ancient (53650 -> 89184 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/5E: Imploder Data (xfd)-packed, unpacked with ancient (47042 -> 59998 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/5F: Imploder Data (xfd)-packed, unpacked with ancient (9972 -> 19224 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/60: Imploder Data (xfd)-packed, unpacked with ancient (8594 -> 14260 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/61: Imploder Data (xfd)-packed, unpacked with ancient (33450 -> 79434 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/62: Imploder Data (xfd)-packed, unpacked with ancient (33384 -> 82972 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/63: Imploder Data (xfd)-packed, unpacked with ancient (83022 -> 151426 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/64: Imploder Data (xfd)-packed, unpacked with ancient (8698 -> 16204 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/65: Imploder Data (xfd)-packed, unpacked with ancient (27454 -> 41932 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/66: Imploder Data (xfd)-packed, unpacked with ancient (45056 -> 59962 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/67: Imploder Data (xfd)-packed, unpacked with ancient (25484 -> 32940 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/68: Imploder Data (xfd)-packed, unpacked with ancient (19012 -> 43688 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/69: Imploder Data (xfd)-packed, unpacked with ancient (28582 -> 59750 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/6A: Imploder Data (xfd)-packed, unpacked with ancient (19364 -> 48000 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/6B: Imploder Data (xfd)-packed, unpacked with ancient (33226 -> 65536 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/6C: Imploder Data (xfd)-packed, unpacked with ancient (92654 -> 136756 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/6D: Imploder Data (xfd)-packed, unpacked with ancient (45386 -> 112684 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/6E: high-entropy data file (7.80 bits/byte), classified as packed data
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/6F: Imploder Data (xfd)-packed, unpacked with ancient (45392 -> 112684 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/70: Imploder Data (xfd)-packed, unpacked with ancient (35364 -> 106998 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/71: Imploder Data (xfd)-packed, unpacked with ancient (47532 -> 59060 bytes)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/load: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/1C#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/6B#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/6D#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/6F#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Lollypop_v1.0/Lollypop_v1.0_hdf/Lollypop/data/70#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 4 never-loaded region(s), 251682 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_hardware` (6D#unpacked (raw code file, load address unknown)): Sets up multiple Amiga hardware registers including DMA control, interrupt enable, and copper list pointers. Kind: disk hardware check. Pass path: ADKCON register setup and hardware initialization continues normally.
- `setup_display_system` (6F#unpacked (raw code file, load address unknown)): Configures the Amiga's custom hardware for graphics rendering by setting up bitplane pointers, display modes, and color registers. Kind: disk hardware check. Pass path: ADKCON register read/modify operations succeed without triggering protection traps.
- `handle_input_loop` (70#unpacked (raw code file, load address unknown)): Processes user input events in a continuous loop by calling input detection functions and updating global state variables based on key press/release conditions. Kind: disk hardware check. Pass path: The routine checks for specific input conditions (key presses) and modifies a counter variable (_DAT_000401c2). If the correct sequence of inputs is detected (specifically, pressing 'L' to decrease and 'M' to increase the counter, with a fi.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `empty_routine`: the routine performs no observable work and ends without modifying registers or memory.

### 6D#unpacked (raw code file, load address unknown)

- `initialize_hardware`: Sets up multiple Amiga hardware registers including DMA control, interrupt enable, and copper list pointers.

### 6F#unpacked (raw code file, load address unknown)

- `setup_display_system`: Configures the Amiga's custom hardware for graphics rendering by setting up bitplane pointers, display modes, and color registers.

### 70#unpacked (raw code file, load address unknown)

- `handle_input_loop`: Processes user input events in a continuous loop by calling input detection functions and updating global state variables based on key press/release conditions.
