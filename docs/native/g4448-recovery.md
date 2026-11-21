# Game 4448 (Moochies v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04448). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3642583 top-level bytes in 225 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 56 raw-word lines |
| Functions named and described | pass, 107 of 107 named (0 author symbols), 107 of 107 described (25 by rule, 82 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1390532 bytes, 106 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3939 of 4028 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4527 |
| code | 4 |
| data | 3638048 |
| padding | 4 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| bootfile#unpacked (raw code file, load address unknown) | 1 | 48 | 2 |
| castleanim#unpacked (raw code file, load address unknown) | 1 | 795 | 38 |
| endgame#unpacked (raw code file, load address unknown) | 1 | 27 | 1 |
| game#unpacked (raw code file, load address unknown) | 1 | 11 | 1 |
| highscore#unpacked (raw code file, load address unknown) | 1 | 1200 | 18 |
| loader#unpacked (raw code file, load address unknown) | 1 | 453 | 6 |
| select#unpacked (raw code file, load address unknown) | 1 | 1492 | 40 |

## Cases handled

- Moochies_v1.0/Moochies_v1.0.hdf: hard-disk image (AmigaDOS OFS, 4192 blocks): its filesystem parses and all 114 files in it are byte-identical to the extracted tree Moochies_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/00blocks: Imploder Data (xfd)-packed, unpacked with ancient (6896 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/00doors: Imploder Data (xfd)-packed, unpacked with ancient (350 -> 2080 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/00enemies: Imploder Data (xfd)-packed, unpacked with ancient (2262 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/00level: Imploder Data (xfd)-packed, unpacked with ancient (4766 -> 14000 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/00walls: Imploder Data (xfd)-packed, unpacked with ancient (8388 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/01blocks: Imploder Data (xfd)-packed, unpacked with ancient (4772 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/01doors: Imploder Data (xfd)-packed, unpacked with ancient (932 -> 2080 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/01enemies: Imploder Data (xfd)-packed, unpacked with ancient (3348 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/01level: Imploder Data (xfd)-packed, unpacked with ancient (4632 -> 14000 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/01tune: Imploder Data (xfd)-packed, unpacked with ancient (25582 -> 32732 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/01walls: Imploder Data (xfd)-packed, unpacked with ancient (5968 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/02blocks: Imploder Data (xfd)-packed, unpacked with ancient (5772 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/02enemies: Imploder Data (xfd)-packed, unpacked with ancient (3758 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/02level: Imploder Data (xfd)-packed, unpacked with ancient (4272 -> 14000 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/02tune: Imploder Data (xfd)-packed, unpacked with ancient (22636 -> 29788 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/02walls: Imploder Data (xfd)-packed, unpacked with ancient (2398 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/03blocks: Imploder Data (xfd)-packed, unpacked with ancient (4904 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/03enemies: Imploder Data (xfd)-packed, unpacked with ancient (4386 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/03level: Imploder Data (xfd)-packed, unpacked with ancient (4306 -> 14000 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/03tune: Imploder Data (xfd)-packed, unpacked with ancient (26452 -> 34868 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/04blocks: Imploder Data (xfd)-packed, unpacked with ancient (5316 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/04enemies: Imploder Data (xfd)-packed, unpacked with ancient (4374 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/04tune: Imploder Data (xfd)-packed, unpacked with ancient (29198 -> 40300 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/05blocks: Imploder Data (xfd)-packed, unpacked with ancient (6372 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/05enemies: Imploder Data (xfd)-packed, unpacked with ancient (3378 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/05tune: Imploder Data (xfd)-packed, unpacked with ancient (27842 -> 36280 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/05walls: Imploder Data (xfd)-packed, unpacked with ancient (6080 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/06blocks: Imploder Data (xfd)-packed, unpacked with ancient (5512 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/06enemies: Imploder Data (xfd)-packed, unpacked with ancient (5408 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/06tune: Imploder Data (xfd)-packed, unpacked with ancient (22546 -> 28866 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/06walls: Imploder Data (xfd)-packed, unpacked with ancient (3826 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/07blocks: Imploder Data (xfd)-packed, unpacked with ancient (6326 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/07enemies: Imploder Data (xfd)-packed, unpacked with ancient (4446 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/07tune: Imploder Data (xfd)-packed, unpacked with ancient (16044 -> 27052 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/07walls: Imploder Data (xfd)-packed, unpacked with ancient (14708 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/08blocks: Imploder Data (xfd)-packed, unpacked with ancient (5362 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/08enemies: Imploder Data (xfd)-packed, unpacked with ancient (4750 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/08tune: Imploder Data (xfd)-packed, unpacked with ancient (20880 -> 33500 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/08walls: Imploder Data (xfd)-packed, unpacked with ancient (7136 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/09blocks: Imploder Data (xfd)-packed, unpacked with ancient (5842 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/09enemies: Imploder Data (xfd)-packed, unpacked with ancient (5368 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/09tune: Imploder Data (xfd)-packed, unpacked with ancient (23952 -> 32754 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/10blocks: Imploder Data (xfd)-packed, unpacked with ancient (3466 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/10enemies: Imploder Data (xfd)-packed, unpacked with ancient (4308 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/10tune: Imploder Data (xfd)-packed, unpacked with ancient (30170 -> 38930 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/11blocks: Imploder Data (xfd)-packed, unpacked with ancient (6636 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/11enemies: Imploder Data (xfd)-packed, unpacked with ancient (4414 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/11tune: Imploder Data (xfd)-packed, unpacked with ancient (21122 -> 28556 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/11walls: Imploder Data (xfd)-packed, unpacked with ancient (940 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/12blocks: Imploder Data (xfd)-packed, unpacked with ancient (5118 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/12enemies: Imploder Data (xfd)-packed, unpacked with ancient (7100 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/12tune: Imploder Data (xfd)-packed, unpacked with ancient (23714 -> 35280 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/13blocks: Imploder Data (xfd)-packed, unpacked with ancient (7046 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/13enemies: Imploder Data (xfd)-packed, unpacked with ancient (6792 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/13tune: Imploder Data (xfd)-packed, unpacked with ancient (19664 -> 34518 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/14blocks: Imploder Data (xfd)-packed, unpacked with ancient (7214 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/14tune: Imploder Data (xfd)-packed, unpacked with ancient (23258 -> 34014 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/15blocks: Imploder Data (xfd)-packed, unpacked with ancient (6422 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/15enemies: Imploder Data (xfd)-packed, unpacked with ancient (5508 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/15tune: Imploder Data (xfd)-packed, unpacked with ancient (29618 -> 40738 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/16blocks: Imploder Data (xfd)-packed, unpacked with ancient (6932 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/16enemies: Imploder Data (xfd)-packed, unpacked with ancient (7932 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/16tune: Imploder Data (xfd)-packed, unpacked with ancient (24736 -> 32764 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/17blocks: Imploder Data (xfd)-packed, unpacked with ancient (5314 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/17enemies: Imploder Data (xfd)-packed, unpacked with ancient (6688 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/17tune: Imploder Data (xfd)-packed, unpacked with ancient (23648 -> 32060 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/18blocks: Imploder Data (xfd)-packed, unpacked with ancient (4584 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/18enemies: Imploder Data (xfd)-packed, unpacked with ancient (5696 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/18tune: Imploder Data (xfd)-packed, unpacked with ancient (26478 -> 32752 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/18walls: Imploder Data (xfd)-packed, unpacked with ancient (1778 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/19blocks: Imploder Data (xfd)-packed, unpacked with ancient (6212 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/19enemies: Imploder Data (xfd)-packed, unpacked with ancient (7520 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/19tune: Imploder Data (xfd)-packed, unpacked with ancient (26522 -> 35388 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/19walls: Imploder Data (xfd)-packed, unpacked with ancient (1766 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/20blocks: Imploder Data (xfd)-packed, unpacked with ancient (5362 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/20enemies: Imploder Data (xfd)-packed, unpacked with ancient (4812 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/20tune: Imploder Data (xfd)-packed, unpacked with ancient (22140 -> 34876 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/21blocks: Imploder Data (xfd)-packed, unpacked with ancient (7100 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/21enemies: Imploder Data (xfd)-packed, unpacked with ancient (3250 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/21tune: Imploder Data (xfd)-packed, unpacked with ancient (16482 -> 26174 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/22blocks: Imploder Data (xfd)-packed, unpacked with ancient (6802 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/22enemies: Imploder Data (xfd)-packed, unpacked with ancient (3288 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/22tune: Imploder Data (xfd)-packed, unpacked with ancient (25504 -> 35240 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/23blocks: Imploder Data (xfd)-packed, unpacked with ancient (4276 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/23enemies: Imploder Data (xfd)-packed, unpacked with ancient (4206 -> 19836 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/23tune: Imploder Data (xfd)-packed, unpacked with ancient (27594 -> 36862 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/24blocks: Imploder Data (xfd)-packed, unpacked with ancient (4854 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/24tune: Imploder Data (xfd)-packed, unpacked with ancient (20852 -> 32970 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/24walls: Imploder Data (xfd)-packed, unpacked with ancient (1454 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/25blocks: Imploder Data (xfd)-packed, unpacked with ancient (6570 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/25tune: Imploder Data (xfd)-packed, unpacked with ancient (22690 -> 33658 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/25walls: Imploder Data (xfd)-packed, unpacked with ancient (912 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/26blocks: Imploder Data (xfd)-packed, unpacked with ancient (5780 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/26tune: Imploder Data (xfd)-packed, unpacked with ancient (26746 -> 39516 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/27tune: Imploder Data (xfd)-packed, unpacked with ancient (29332 -> 39740 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/27walls: Imploder Data (xfd)-packed, unpacked with ancient (966 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/28blocks: Imploder Data (xfd)-packed, unpacked with ancient (5166 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/28tune: Imploder Data (xfd)-packed, unpacked with ancient (23072 -> 34108 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/29blocks: Imploder Data (xfd)-packed, unpacked with ancient (5758 -> 9966 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/35walls: Imploder Data (xfd)-packed, unpacked with ancient (1298 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/37walls: Imploder Data (xfd)-packed, unpacked with ancient (1084 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/40walls: Imploder Data (xfd)-packed, unpacked with ancient (10408 -> 50464 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/bootfile: Imploder Data (xfd)-packed, unpacked with ancient (4608 -> 54600 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/castleanim: Imploder Data (xfd)-packed, unpacked with ancient (66256 -> 225348 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/endgame: Imploder Data (xfd)-packed, unpacked with ancient (91738 -> 184692 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/game: Imploder Data (xfd)-packed, unpacked with ancient (88736 -> 425860 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/highscore: Imploder Data (xfd)-packed, unpacked with ancient (39036 -> 129824 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/loader: Imploder Data (xfd)-packed, unpacked with ancient (70010 -> 108872 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/select: Imploder Data (xfd)-packed, unpacked with ancient (82496 -> 261336 bytes)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/bootfile#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/castleanim#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/endgame#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/game#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/highscore#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/loader#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Moochies_v1.0/Moochies_v1.0_hdf/Moochies/data/select#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 7 never-loaded region(s), 1390532 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### bootfile#unpacked (raw code file, load address unknown)

- `initialize_system_display`: Saves registers, reads a display mode register, sets up interrupt enable registers, handles display initialization or restoration logic including saving and restoring register values, then restores registers and returns.
- `setup_display_registers`: Configures multiple display hardware registers including window and bitplane settings, then returns immediately.

### castleanim#unpacked (raw code file, load address unknown)

- `setup_interrupt_handler`: Initializes several system functions, enters a loop waiting for a condition, then calls cleanup routines before returning.
- `frag_addi_btst_bne_move_cmpi_blt_rts`: Short code fragment that adds 1 to a fixed memory address, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then copies 255 to a fixed memory address, then copies a fixed memory address to d0, then compares d0 with 1478, then branches if the comparison was less, then copies 255 to a fixed memory address, then returns.
- `toggle_animation_state`: Tests bits in a global variable and branches based on their state, then calls other routines to update animation data before clearing or setting flags.
- `update_animation_counter`: Checks a global flag, increments an animation counter if not at maximum value, otherwise resets it and processes buffer data.
- `update_animation_state_2`: Updates animation state by incrementing a counter and checking conditions, then modifies global variables based on comparison results.
- `process_blitter_wait`: Waits for blitter readiness and configures multiple blitter registers with specific values before returning.
- `update_global_variable`: Updates a global variable through a loop and conditional checks, then resets it to zero before returning.
- `update_global_variable_2`: Waits for a global variable to equal 32, then increments it in a loop until it reaches 5, after which it resets the variable and increments it by 4 before returning.
- `handle_frame_counter`: Processes frame counter logic by comparing against a constant value and updating state variables accordingly.
- `update_animation_state_3`: Updates animation state variables based on conditional checks and increments, then clears specific flags and resets counters when certain conditions are met.
- `manage_animation_states`: Tests a bit in a global variable and branches if set.
- `frag_lea_bsr_addi_adda_move_cmpi_bne_cl`: Short code fragment that loads the address of a fixed memory address into a6, then calls another routine, then adds 1 to a fixed memory address, then adds 48 to a6, then copies a fixed memory address to d7, then compares d7 with 4, then branches if the last result was not zero, then clears a fixed memory address, then returns.
- `configure_blitter_operation`: Waits for blitter to become idle, sets up blitter source and destination pointers, then configures blitter control registers with fixed parameters.
- `initialize_blitter_dma`: Waits for blitter to become idle, sets up blitter source and destination pointers, then configures blitter control registers with specific parameters.
- `setup_blitter`: Configures multiple blitter registers including source and destination pointers, modulo values, control words, and transfer size using hardcoded constants, then returns.
- `setup_blitter_2`: Configures multiple blitter registers with specific values, including control and size settings, then returns.
- `setup_blitter_3`: Configures multiple blitter registers with specific values, including setting up source and destination pointers, modulo values, and transfer dimensions.
- `setup_blitter_4`: Configures multiple blitter registers with specific values, waits for a bit in a global variable to clear, and then writes data pointers to blitter address registers.
- `setup_blitter_5`: Configures multiple blitter registers with specific values, including setting up source and destination pointers, modulo values, and control words for a graphics operation.
- `setup_blitter_6`: Configures blitter registers for a memory-to-memory transfer operation using specific modulo values and bit plane settings, then returns.
- `wait_for_blitter`: Waits for the blitter to become ready by polling a status register, then configures multiple blitter control and memory address registers with specific values before returning.
- `update_animation_state`: Updates animation state based on data fetched from a table, modifying flags and counters while advancing through animation frames.
- `frag_subi_rts`: Short code fragment that subtracts 40 from the memory at an offset from a6, then returns.
- `frag_addi_rts`: Short code fragment that adds 40 to the memory at an offset from a6, then returns.
- `frag_move_cmpi_bne_addi_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then compares d0 with 15, then branches if the last result was not zero, then copies 1 to the memory at an offset from a6, then adds 2 to the memory at an offset from a6, then returns, then adds 1 to the memory at an offset from a6, then returns.
- `frag_move_cmpi_bne_subi_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then compares d0 with 0, then branches if the last result was not zero, then copies 14 to the memory at an offset from a6, then subtracts 2 from the memory at an offset from a6, then returns, then subtracts 1 from the memory at an offset from a6, then returns.
- `frag_bsr_rts`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_bsr_rts_2`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_bsr_rts_3`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_bsr_rts_4`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_bsr_rts_5`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_bsr_rts_6`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_bsr_rts_7`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_bsr_rts_8`: Short code fragment that calls another routine (2 times in a row), then returns.
- `process_dma_transfer`: Processes DMA transfers by checking completion flags, updating pointers, and managing counter values in a loop structure.
- `update_animation_counter_2`: Increments a global counter and checks if it reaches a specific value, clearing the counter if the condition is met.
- `initialize_dma_operation`: Initializes DMA operation by setting up source and destination pointers, performing arithmetic operations, and updating counter values.
- `configure_blitter_and_dma`: Tests a bit in a global variable and branches if set.

### endgame#unpacked (raw code file, load address unknown)

- `initialize_game`: Sets up hardware registers for display and interrupts, then waits for a condition to be met before proceeding with additional initialization calls.

### game#unpacked (raw code file, load address unknown)

- `copy_game_data`: Copies data from one memory location to another using a series of move operations and calls helper routines to process the copied information.

### highscore#unpacked (raw code file, load address unknown)

- `setup_display_system`: Initializes global variables and system functions, sets up interrupt enable register, enters a loop waiting for conditions, then calls cleanup routines.
- `compare_and_update_scores`: Compares values between memory locations, updates score data when conditions are met, and handles score ranking logic.
- `copy_and_pad_string`: Copies bytes from one memory location to another using a loop, then fills remaining space with spaces and null terminators.
- `convert_digit_to_ascii`: Takes a character value from a register, adds the ASCII value of '0' to it, and stores the result in memory through a pointer.
- `copy_score_data`: Copies score-related data from a global variable to another location and updates multiple global variables with values from tables based on index offsets.
- `update_score_values`: Processes score-related values by performing arithmetic operations on word-sized data, then stores the results in specific memory locations.
- `update_score_entry`: Writes a word value to a structure at specific offsets, updating coordinates and incrementing a counter before returning.
- `store_highscore_entry`: Handles high score entry logic by reading from a data structure, comparing values, and writing updated information to another location.
- `frag_movea_cmpa_bne_rts_addi_move_add`: Short code fragment that copies the memory at an offset from a0 to a5, then compares a5 with 87170, then branches if the last result was not zero, then returns, then adds 2 to the memory at an offset from a0, then copies the memory at a5 to d0, then copies a fixed memory address to d7, then adds d7 to d0, then copies d0 to the memory at an offset from a0, then returns.
- `check_bit_and_branch`: Tests a bit in a global variable and branches to a specific location if the bit is set, otherwise continues execution.
- `load_and_compare`: Loads two address pointers from global memory locations into a1 and a0, then calls three routines before returning.
- `initialize_memory_pointers`: Tests a bit in a global variable and branches if set.
- `manage_game_counters_and_flags`: Tests bits in global variables and performs conditional operations including incrementing counters, calling routines, and setting or clearing flags based on comparison results with specific values.
- `update_position_values`: Updates several word-sized values in memory based on comparisons and arithmetic operations, then performs bit shifting and multiplication before storing the result back to memory.
- `calculate_offset_value`: Performs bit shifting and arithmetic operations on word-sized values to compute an offset address for memory access.
- `frag_move_lsr_or_rts`: Short code fragment that copies 128 to d4, then shifts d4 right by d2, then merges d4 into the memory at a2, then returns.
- `rotate_and_mask_byte`: Rotates a byte value right by a specified amount and applies a bitwise AND operation with a mask, then stores the result back to memory.
- `setup_blitter_operation`: Configures blitter registers with specific values, waits for a hardware condition, then sets up additional display parameters before returning.

### loader#unpacked (raw code file, load address unknown)

- `initialize_system`: Initializes global variables with fixed values, calls several setup routines, waits for a condition to be met, then calls cleanup functions.
- `modify_bit_flags`: Clears a word-sized register, reads a byte from memory, checks its sign bit, and modifies the original byte based on that check.
- `process_bit_field_data`: Takes a byte value from memory, tests if it has its least significant bit set, and branches if not.
- `setup_blitter_registers`: Configures multiple blitter hardware registers including source and destination addresses, moduli, control settings, and waits for a hardware condition before returning.
- `configure_blitter_display`: Waits for a hardware condition, then sets up blitter parameters including source and destination addresses, and various control registers.
- `setup_multi_plane_blit`: Configures multiple blitter planes with identical parameters, waits for hardware readiness, then sets up control registers for a complex graphics operation.

### select#unpacked (raw code file, load address unknown)

- `initialize_system_and_dma`: Stores a register value to a global variable, calls several routines including interrupt and DMA setup, tests flags in memory, and branches based on their states.
- `setup_interrupts_and_dma`: Configures interrupt and DMA settings by writing specific values to hardware control registers, then calls another routine to handle input polling.
- `frag_move_lea_clr_dbra_rts`: Short code fragment that copies 31 to d0, then loads the address of a fixed memory address into a0, then clears the memory at a0 (advancing a0), then loops back while counting down d0, then copies a fixed memory address to a fixed memory address, then copies 66150 to a fixed memory address, then returns.
- `handle_input_polling`: Handles input polling by reading from the keyboard serial register and managing interrupt requests, but does nothing else in this routine.
- `frag_lea_bra`: Short code fragment that loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `loop_until_counter_reaches_four`: Loops while a global variable is less than four, incrementing it each iteration, then sets it to zero before returning.
- `set_pattern_flags`: Compares a word value with specific constants and sets corresponding bits in a global variable.
- `frag_movem_lea_moveq_bsr_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then loads 7 into d2, then calls another routine, then restores saved registers from the stack, then returns.
- `process_joystick_input`: Reads joystick status from a hardware register, performs bitwise operations on multiple input values, and returns a combined result in a register.
- `check_flags_and_set_bits`: Clears a word register and sets specific bits based on the values of several global variables, then returns the modified value.
- `update_display_state`: Updates a global variable bitfield based on conditions from a lookup table, then calls other routines to handle display updates and memory operations.
- `frag_movea_move_bsr_rts`: Short code fragment that copies a2 to a0, then copies 38 to d0, then copies 1793 to d1, then copies 56 to d3, then copies a1 to the stack, then calls another routine, then copies the top of the stack to a1, then returns.
- `frag_bsr_move_cmpi_beq_adda_addi_bra_cl`: Short code fragment that calls another routine, then copies a fixed memory address to d7, then compares d7 with 4, then branches if the last result was zero, then adds d3 to a0, then adds 4520 to a1, then adds 1 to a fixed memory address, then jumps to another point in the code, then clears a fixed memory address, then returns.
- `process_pattern_data`: Processes data in a loop until encountering a termination marker, calling other routines for each iteration while advancing through memory locations.
- `frag_move_clr_lsl_add_rts`: Short code fragment that copies d5 to the stack, then clears d6, then copies a fixed memory address to d6, then copies a fixed memory address to d5, then shifts d6 left by 8, then adds d5 to d6, then copies the top of the stack to d5, then returns.
- `clear_cia_registers`: Clears the CIA TODLOW and TODMID registers and returns immediately.
- `render_graphics_with_dma`: Configures and executes DMA transfers for graphics rendering by setting up hardware registers with values from input pointers and handling interrupt control.
- `copy_pattern_data`: Copies byte data from one memory location to multiple destinations in a pattern, incrementing both source and destination pointers in a loop.
- `lookup_and_adjust_offset`: Uses an input value to calculate an offset into a table, adjusts the table pointer based on a lookup result, and returns the final pointer value.
- `update_selection_loop`: Iterates through a table using an index from a global variable, reads bytes from the table, and calls another routine with calculated parameters.
- `update_frame_counter`: Increments a global variable and performs a blitter setup operation before checking if the variable has reached a specific value, otherwise it returns.
- `update_selection_counter`: Increments a global counter and checks if it reaches a specific value, resetting it when met.
- `configure_blitter_dma_3`: Configures blitter DMA parameters including source and destination pointers, dimensions, and performs a blitter operation before incrementing a counter and checking its value.
- `update_selection_state`: Updates the selection state based on input conditions and modifies flags in a structure, then returns.
- `update_selection`: Updates the selection state based on input conditions and modifies flags in a structure, then returns.
- `update_counter_and_flags`: Increments a byte value at a fixed memory address and checks if it equals five, clearing the counter and returning if so.
- `update_game_state`: Checks a bit in a global variable and updates a counter in a structure based on comparison results with specific values, then calls another routine to process the data.
- `frag_bsr_adda_addi_move_cmpi_bne_clr_rt`: Short code fragment that calls another routine, then adds d1 to a0, then adds d2 to a1, then adds 1 to a fixed memory address, then copies a fixed memory address to d7, then compares d7 with 5, then branches if the last result was not zero, then clears a fixed memory address, then returns.
- `frag_bsr_adda_addi_move_cmpi_bne_clr_rt_2`: Short code fragment that calls another routine, then adds 1122 to a0, then adds d2 to a2, then adds d2 to a3, then adds 1 to a fixed memory address, then copies a fixed memory address to d7, then compares d7 with 5, then branches if the last result was not zero, then clears a fixed memory address, then returns.
- `select_character_data`: Tests a bit in memory, reads a word value, performs comparisons, multiplies by a constant, then calls another routine with indexed data.
- `increment_counter_loop`: Calls another routine and increments a counter in memory until it reaches a specific value, then clears the counter and returns.
- `setup_blitter_dma`: Waits for a hardware DMA status bit to clear, then configures multiple blitter registers including source and destination pointers, moduli, and control words before returning.
- `configure_blitter_dma`: Sets up interrupt enable registers, waits for a hardware DMA status bit to clear, configures blitter registers with various parameters including source and destination pointers, moduli, control words, and size, then enables interrupts before returning.
- `initialize_blitter_dma_2`: Sets up interrupt enable registers, waits for a hardware DMA status bit to clear, configures blitter registers with source and destination pointers, moduli, control words, and size, then enables interrupts before returning.
- `configure_blitter_params`: Waits for a hardware DMA status bit to clear, configures blitter registers with source and destination pointers, moduli, control words, and size, then returns without enabling interrupts.
- `setup_blitter_with_values`: Waits for a hardware DMA status bit to clear, configures blitter registers with source and destination pointers, moduli, control words, and size using register values, then returns.
- `clear_dma_status`: Reads and waits for a specific bit in a global variable until it clears, then writes multiple values to hardware registers controlling bitmap transfer operations, and finally enables interrupts.
- `setup_blitter_dma_2`: Configures blitter registers for direct memory access using parameters from input registers and pointers, then returns.
- `setup_multi_blitter_dma`: Waits for a hardware DMA status bit to clear, configures multiple blitter registers including source and destination pointers, moduli, control words, and size using register values, then returns.
- `configure_blitter_dma_2`: Sets up multiple blitter DMA registers with specific values and returns immediately after configuring the hardware.
