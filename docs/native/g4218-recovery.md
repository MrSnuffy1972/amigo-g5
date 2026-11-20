# Game 4218 (LastActionHero v1.1): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04218). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3165856 top-level bytes in 153 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 45 raw-word lines |
| Functions named and described | pass, 130 of 130 named (0 author symbols), 130 of 130 described (25 by rule, 100 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 137122 bytes, 129 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6613 of 6641 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10527 |
| code | 4 |
| data | 3155325 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| LAH.RNC#unpacked (raw code file, load address unknown) | 1 | 6639 | 129 |

## Cases handled

- LastActionHero_v1.1/LastActionHero_v1.1.hdf: hard-disk image (AmigaDOS OFS, 3552 blocks): its filesystem parses and all 78 files in it are byte-identical to the extracted tree LastActionHero_v1.1_hdf, which is recovered file by file; no block outside the filesystem holds data
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/AGAMEBY.BIN: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (10824 -> 40960 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BADMAN.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (33971 -> 44218 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BENEDICT.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (24275 -> 82198 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BIN.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (2134 -> 6202 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BLOOD.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (255 -> 2470 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BOLO.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (21442 -> 57740 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BOSS1.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (11562 -> 54400 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BOSS1.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (2712 -> 51200 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BOSS2.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (1890 -> 10720 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BOSS2.MAP: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (239 -> 680 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BOSS2.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (3619 -> 51200 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BOSS3.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (3534 -> 20320 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BOSS3.MAP: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (365 -> 680 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BOSS3.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (1930 -> 51200 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BOSS4.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6791 -> 28480 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BOSS4.MAP: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (430 -> 680 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BOSS4.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (2463 -> 51200 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BOSS5.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (8611 -> 46240 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BOSS5.MAP: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (641 -> 680 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/BOSS5.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4569 -> 51200 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/CHAIR.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (1993 -> 6132 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/CRATE.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (2170 -> 6162 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/DANIM.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (24239 -> 44866 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/DEALER.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5874 -> 13884 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/FIRE.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (968 -> 2160 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/FIREMAN.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (12849 -> 30272 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/FLASH.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (330 -> 2450 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/FONT8.BIN: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (389 -> 768 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/FRANCO.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (12345 -> 29738 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/GINELLI.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (19527 -> 51792 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/HOCKEY.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (26729 -> 63290 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/ICE.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (13093 -> 31990 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/ILLBEBAK.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (11881 -> 22086 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/INGAME1.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (31357 -> 43980 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/INGAME2.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (43490 -> 57368 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/INGAME3.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (30412 -> 45180 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/INGAME4.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (32205 -> 56096 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/INGAME5.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (31182 -> 52370 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/JET.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (15240 -> 33522 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/LAH.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (81192 -> 137122 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/LAH_INTV.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (250947 -> 489566 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/LEVEL1.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (21195 -> 72320 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/LEVEL1.MAP: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (1588 -> 4080 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/LEVEL2.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (12771 -> 54720 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/LEVEL2.MAP: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (1719 -> 4080 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/LEVEL3.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (13935 -> 67040 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/LEVEL3.MAP: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (1279 -> 4080 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/LEVEL4.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (25988 -> 126240 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/LEVEL4.MAP: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (2052 -> 4080 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/LEVEL5.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6311 -> 25440 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/LEVEL5.MAP: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (417 -> 680 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/LIFTBRIK.BIN: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (106 -> 5440 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/METAL.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (15861 -> 38404 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/OPTIONSC.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (10252 -> 51200 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/OUT1.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (27558 -> 52602 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/OUT2.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (151650 -> 230644 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/POLE.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (295 -> 1800 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/POWERUPS.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (1350 -> 4022 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/PSYLOGO.BIN: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4807 -> 40960 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/RIPPER.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (28593 -> 79738 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/SLATER.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (49221 -> 102912 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/STARK.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (13303 -> 33538 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/TABLE.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (2668 -> 10112 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/TITLE.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (51521 -> 60110 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/TITLEPIC.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (25966 -> 51200 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/TONI.GFX: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (15648 -> 43530 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/TWEEN1.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5252 -> 51200 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/TWEEN2.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6661 -> 51200 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/TWEEN3.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6249 -> 51200 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/TWEEN4.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4988 -> 51200 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/TWEEN5.RNC: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (5802 -> 51200 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/WANDD.LBM: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4177 -> 8288 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/WINANIM.BMF: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (25089 -> 158400 bytes)
- LastActionHero_v1.1/LastActionHero_v1.1_hdf/LastActionHero/data/LAH.RNC#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_audio_channels` (LAH.RNC#unpacked (raw code file, load address unknown)): Initializes audio DMA channels by setting up hardware registers including volume, period, length, and channel pointers, then enables DMA and interrupts. Kind: disk hardware check. Pass path: CIAB_PRB register read value indicates valid hardware configuration.
- `update_joystick_status` (LAH.RNC#unpacked (raw code file, load address unknown)): Updates joystick input status by reading a counter value, adding it to a fixed byte, writing the result to a memory location, and calling other routines to handle input processing. Kind: joystick status check. Pass path: the routine updates joystick status and continues execution normally when check passes.
- `handle_input_processing` (LAH.RNC#unpacked (raw code file, load address unknown)): Processes input by setting a memory flag, reading a counter value, adding it to a fixed byte, writing the result to a memory location, and calling another routine to handle input processing. Kind: disk hardware check. Pass path: The routine writes to CIAB_PRB and calls u1__sub_0A1A(), suggesting a hardware-based check (likely involving port polling or disk drive status). The call to sub_0A1A() is followed by a jump to an address, which may be a sabotage or continuation .
- `wait_for_joystick_input` (LAH.RNC#unpacked (raw code file, load address unknown)): Waits for specific joystick button states by polling a hardware register until a bit is clear, then sets and clears flags in memory while calling other routines to process input. Kind: joystick input wait routine. Pass path: The routine waits for joystick input and then continues execution. The check passes when the joystick is pressed (bit 5 of CIAA_PRA is cleared), allowing execution to proceed to u1__sub_0A1A()..
- `process_joystick_movement` (LAH.RNC#unpacked (raw code file, load address unknown)): Processes joystick movement by reading a counter value, comparing it with another value, and either decrementing or incrementing a counter before calling other routines to handle movement. Kind: disk hardware check. Pass path: The routine checks joystick movement via CIAB_PRB and performs bit manipulation on it, likely to verify a specific hardware state or pattern. A successful check would pass through the branch where the joystick movement is detected and proce.
- `initialize_graphics_and_input` (LAH.RNC#unpacked (raw code file, load address unknown)): Initializes graphics hardware registers including DMA control, interrupt settings, and disk drive parameters. Kind: disk track sync check. Pass path: the routine proceeds past the sync check and loads data, with the successful path being the branch where `CUSTOM_INTREQR` shows interrupt flag 2 (DSKINT) cleared after a valid disk sync pattern is detected.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### LAH.RNC#unpacked (raw code file, load address unknown)

- `frag_move_addi_lea`: Short code fragment that copies 65535 to a fixed memory address, then adds 1 to a fixed memory address, then loads the address of a fixed memory address into a6, then copies 32767 to the memory at an offset from a6 (2 times in a row), then loads the address of a fixed memory address into a7, then loads the address of a location in the code into a0, then copies a0 to a fixed memory address.
- `initialize_audio_channels`: Initializes audio DMA channels by setting up hardware registers including volume, period, length, and channel pointers, then enables DMA and interrupts.
- `frag_move_addi_andi_cmp_bne_rts`: Short code fragment that copies the memory at an offset from a6 to d1, then adds 512 to d1, then masks d1 with 65280, then copies the memory at an offset from a6 to d2, then masks d2 with 65280, then compares d2 with d1, then branches if the last result was not zero, then returns.
- `initialize_dma_channels`: Sets up DMA and interrupt registers, copies data from multiple source locations to destination addresses, and configures various hardware control registers including coprocessor pointers and interrupt masks.
- `initialize_joystick_input`: Initializes joystick input handling by reading a counter value, using it as an index to access a table, storing the result in a global variable, and setting multiple hardware registers with specific values.
- `frag_move_lsr_cmp_beq_dbra_rts`: Short code fragment that copies a fixed memory address to d1, then shifts d1 right by 8, then copies d1 to d2, then copies a fixed memory address to d1, then shifts d1 right by 8, then compares d2 with d1, then branches if the last result was zero, then loops back while counting down d0, then returns.
- `frag_addi_movem_move_jsr_subi_bne_rts_b`: Short code fragment that adds 5 to a fixed memory address, then saves registers on the stack, then copies a fixed memory address to d0, then calls another routine, then subtracts 1 from d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns, then calls another routine, then jumps to another point in the code.
- `update_joystick_status`: Updates joystick input status by reading a counter value, adding it to a fixed byte, writing the result to a memory location, and calling other routines to handle input processing.
- `handle_input_processing`: Processes input by setting a memory flag, reading a counter value, adding it to a fixed byte, writing the result to a memory location, and calling another routine to handle input processing.
- `wait_for_joystick_input`: Waits for specific joystick button states by polling a hardware register until a bit is clear, then sets and clears flags in memory while calling other routines to process input.
- `process_joystick_movement`: Processes joystick movement by reading a counter value, comparing it with another value, and either decrementing or incrementing a counter before calling other routines to handle movement.
- `frag_move_andi_divu_swap_rts`: Short code fragment that copies d0 to d1, then masks d1 with 65535, then divides d1 by 11 (unsigned), then copies d1 to d2, then masks d1 with 255, then masks d2 with an address, then swaps the halves of d2, then returns.
- `frag_lsr_subq_moveq_move_eor_dbra_andi`: Short code fragment that shifts d1 right by 2, then subtracts 1 from d1, then loads 0 into d0, then copies the memory at a0 (advancing a0) to d2, then exclusive-ors d2 into d0, then loops back while counting down d1, then masks d0 with an address, then returns.
- `rotate_and_index_bitstream`: Rotates a 32-bit value left by four bits eleven times, uses the low nibble of the rotated value as an index into a lookup table, combines the fetched byte with a shifted input value, and returns the result.
- `process_input_sequence`: Reads input data from a global buffer, processes each byte by applying a transformation routine, and updates a global counter.
- `convert_to_uppercase`: Converts lowercase letters to uppercase by subtracting 32 from their ASCII values when they fall within the lowercase range, otherwise it does nothing.
- `initialize_graphics_and_input`: Initializes graphics hardware registers including DMA control, interrupt settings, and disk drive parameters.
- `handle_table_operations`: Handles table operations by reading from indexed memory locations, comparing values, and copying data between tables based on conditional checks and loop iterations.
- `interleave_bit_pairs`: Interleaves bit pairs from two memory locations by performing bitwise AND operations with a mask, left-shifting one value, ORing the results, and storing the combined value in another location.
- `sum_memory_values`: Iterates through a table of 256 values, accumulating their sum with carry, then returns the bitwise NOT of the final sum.
- `update_game_state`: Updates multiple global variables with values from memory locations, processes input data through a routine, sets a bit in a hardware register, and handles conditional logic for different game states.
- `update_timer_data`: Loads a value from a global word to an address register, copies it to another global location, calls a routine with the value, updates a global pointer, and returns immediately.
- `update_and_clear_structure`: Updates two global variables with values from registers, clears a block of memory starting at a computed address, and writes structured data to offsets within a parameter, then sets a status byte.
- `copy_and_transform_data`: Copies data from one memory location to another, processes the copied data by performing arithmetic operations and table lookups, and stores transformed values in a destination buffer.
- `initialize_game_state`: Initializes various game state variables and data structures, including input handling, graphics buffers, and animation tables.
- `setup_blitter_dma`: Configures blitter registers for DMA transfer operations and performs multiple blitter operations using data from tables, with checks on DMA status before each operation.
- `decode_rnc_packed_block_to_buffer`: Saves all registers, allocates 384 bytes on the stack, verifies the RNC signature, unpacks the compressed block into the destination buffer using bit-stream reads, then restores registers and returns.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a condition on a global mask is met, then reads a byte from a table entry and uses it as an index into a global mask to extract a bit, calling another routine if a borrow condition is detected.
- `frag_and_sub_bge_bsr_lsr_rts`: Short code fragment that masks d0 with d6, then subtracts d1 from d7, then branches if the comparison was greater or equal, then calls another routine, then shifts d6 right by d1, then returns.
- `frag_add_lsr_swap_addq_move_rol_sub_mov`: Short code fragment that adds d1 to d7, then shifts d6 right by d7, then swaps the halves of d6, then adds 4 to a3, then copies the memory just below a3 (moving a3 down) to d6, then rotates d6 left by 8, then copies the memory just below a3 (moving a3 down) to d6, then swaps the halves of d6, then subtracts d7 from d1, then loads 16 into d7, then subtracts d1 from d7, then returns.
- `frag_moveq_lsl_move_dbra_rts`: Short code fragment that loads 3 into d1, then shifts d0 left by 8, then copies the memory at a0 (advancing a0) to d0, then loops back while counting down d1, then returns.
- `build_huffman_table`: Constructs a Huffman decoding table by processing bit patterns and building lookup entries for decompression routines.
- `initialize_rendering`: Calls eight different routines in sequence, sets two words at fixed memory locations to specific values, tests two global bytes for zero condition, and returns immediately if either is non-zero.
- `update_blitter_config`: Configures blitter parameters for sprite rendering by setting up source and destination pointers, size, and control registers based on sprite data, then updates internal structure pointers.
- `update_sprite_animation`: Updates sprite animation frame counters and handles blitter operations for sprite rendering, including checking frame conditions and calculating addresses for graphics data.
- `animate_sprite_frame`: Manages sprite animation frame updates by calculating addresses, performing blitter operations, and handling frame counter logic.
- `manage_random_sequence`: Manages random number sequences by checking game state flags, updating counters, and calling random generation routines to produce output values for various game systems.
- `frag_movea_tst_bne_sf_bsr_rts`: Short code fragment that copies a fixed memory address to a0, then tests the memory at an offset from a0, then branches if the last result was not zero, then clears a fixed memory address, then calls another routine (4 times in a row), then returns.
- `process_active_entries`: Iterates through a global table of 16 entries, checking each entry for a non-zero value and calling a function pointer found at offset 6 of the entry structure.
- `store_entity_data`: Stores word-sized values from registers into a table, searching for an empty slot by checking longword values at table entries.
- `validate_object_pointer`: Checks if a pointer in a global table matches a fixed address, clears the table entry if it does, and returns immediately otherwise.
- `frag_move_lea_andi_movea_rts`: Short code fragment that copies a1 to the stack, then loads the address of a fixed memory address into a1, then masks d1 with 15, then copies an indexed entry from a1 to d1, then masks d1 with 255, then copies the top of the stack to a1, then returns.
- `update_object_state_2`: Updates object state information including position, velocity, and animation frame data based on various conditions and comparisons with stored values.
- `reset_animation_frame`: Clears a 16-bit counter and a byte flag in a structure, reads a frame address from a table, checks a bit in the frame data, and stores the result as a signed byte in the structure.
- `decrement_timer`: Decrements a timer value at offset 78 of a structure, clears a flag at offset 106 if set, and updates a counter at offset 60.
- `update_object_state_3`: Updates the state of objects in a global table by checking conditions and modifying their properties, including color register updates and memory management.
- `load_animation_frame`: Loads animation frame data into an object structure based on bit flags and performs related initialization operations for animation playback.
- `update_frame_sequence`: Updates frame sequence by comparing current and next frame values, setting flags, and managing frame counter with boundary checks.
- `generate_random_value`: Generates random values by incrementing counters, masking with bitmasks, and indexing into lookup tables to produce output values.
- `update_animation_state`: Updates animation frame counters and flags based on conditional checks against global variables and structure data, then calls another routine to process changes.
- `update_sprite_coordinates`: Updates two global variables with incremented values, masks them with hexadecimal constants, and invokes a rendering routine three times with different parameters.
- `update_sprite_data`: Writes sprite coordinates and properties from a source structure to a target structure, updates global variables with calculated values, and performs conditional adjustments based on frame data.
- `allocate_entity_slot`: Finds an empty slot in a table and initializes it with entity data, copying input parameters and setting default values for new entries.
- `store_object_data`: Stores word-sized data from registers into a structure at a pointer, initializes specific fields including a flag and a shift-processed value, and updates a global table with the structure pointer.
- `update_object_state`: Updates the state of objects in a global table by checking conditions and modifying their properties, including color register updates and memory clearing operations.
- `check_object_collisions`: Performs detailed collision detection between pairs of objects from two global tables, evaluating various conditions including object states, coordinates, and pointers before calling specific handler routines.
- `check_collision_and_update`: Checks for specific collision conditions between two objects and updates the velocity of one object based on distance and direction calculations.
- `activate_object`: Checks conditions based on object properties and coordinates, then activates an object by setting specific flags and calling another routine.
- `evaluate_attack`: Checks conditions between two game entities to determine if an attack should be triggered, including distance, orientation, and state flags.
- `frag_movem_move_subi_add_bsr_rts`: Short code fragment that saves registers on the stack, then copies the memory at a1 to d0, then copies the memory at an offset from a1 to d2, then copies the memory at an offset from a2 to d1, then subtracts 1 from d1, then adds d1 to d1, then copies an indexed entry from a1 to d1, then calls another routine, then restores saved registers from the stack, then returns.
- `mark_object_active`: Scans through a global object table to find inactive objects and marks them as active by setting a flag byte at a specific offset.
- `process_input_loop`: Initializes a counter to zero, calls two routines with the counter on the stack, increments the counter, compares it with four, and returns early if not equal; otherwise continues with global variable updates.
- `validate_position_range`: Checks if two coordinate pairs are within specific ranges of each other and calls another routine if the conditions are met.
- `update_and_clear_entries`: Processes a sequence of entries by calling functions at calculated addresses, updating a global table entry with zero when a condition is met, and returning an index value.
- `update_counter_value`: Updates a counter value in a table based on a lookup index, incrementing the value at an offset calculated from the index and returning the updated value.
- `lookup_object_data`: Looks up object data in a table using an index and returns the data pointer or -1 if the entry is empty.
- `compare_positions`: Compares positions of objects against a reference point, setting flags in global variables based on whether the comparison results in positive or negative differences.
- `initialize_render_data`: Initializes multiple data structures with values from a lookup table and performs bit manipulation on global variables before copying sprite data from one location to another.
- `add_bcd_values`: Performs binary-coded decimal addition on a 32-bit value stored in a global variable, updating multiple bytes with carry propagation and BCD adjustment.
- `adjust_counter`: Adjusts a counter value in a global object table by incrementing or decrementing it based on comparisons with another value, iterating through the table entries.
- `sort_and_process_objects`: Copies object data between two memory regions, clears the source region, processes objects to sort them by a property, and calls routines on sorted objects.
- `update_sprite_dma`: Updates DMA transfer parameters for sprite rendering by calculating addresses from tables and setting hardware registers, then performs multiple DMA operations with incremental address adjustments.
- `compute_collision_data`: Calculates collision boundaries and offsets based on input coordinates and stores the results in a structured data block.
- `setup_bitmap_dma`: Configures multiple DMA bitplane transfer parameters including source and destination addresses, size, and control words using values derived from structure offsets.
- `draw_bitmap_pattern`: Calculates and draws a bitmap pattern by manipulating coordinates and memory addresses, setting bits in a graphics buffer using shift and add operations.
- `render_sprite_tile`: Processes sprite dimensions and coordinates to calculate tile positions and performs bitplane rendering operations using custom hardware registers for display.
- `update_boundary_values`: Updates boundary values based on comparisons with another structure's data and modifies related counters.
- `update_collision_bounds`: Adjusts collision boundary values based on comparisons with another structure's data and updates related counters.
- `update_collision_data`: Updates position and velocity data based on collision detection with another object, modifying coordinates and counters according to directional comparisons and threshold checks.
- `update_object_data`: Copies a value from a global memory location to another global word, processes a linked list of structures by calling a routine for each node, clears a field in each structure, and updates a global pointer.
- `store_active_entry`: Finds the first empty slot in a global array and stores a pointer to it, then initializes two word values at specific offsets from that pointer.
- `clear_active_entry`: Searches through a global array for a matching pointer value and clears that entry by setting it to zero.
- `check_collision_bounds`: Tests if a structure's bounds intersect with a global time value and updates flags based on the comparison results.
- `update_animation_frame`: Iterates through a linked list of structures, increments a counter in each, accesses a table using that counter, and updates flags and values based on the table results.
- `handle_input_conditions`: Checks various global flags and input states, performs conditional operations based on joystick and mouse status, and manages DMA control settings.
- `handle_dma_control`: Manages DMA channel enabling and disables specific interrupt flags while updating frame counters and setting up color registers for display.
- `initialize_game_state_2`: Initializes game state variables and hardware registers including interrupt and DMA control, then copies data tables based on a state counter.
- `manage_game_state`: Manages game state by checking various flags and counters, updating values based on conditions, and calling other routines to handle specific operations like rendering or sound processing.
- `frag_move_or_bne_rts`: Short code fragment that copies the memory at an offset from a5 to d0, then merges the memory at an offset from a5 into d0, then branches if the last result was not zero, then returns.
- `initialize_data_tables`: Sets up three data tables by calling another routine with specific parameters for character conversion and processing.
- `frag_move_bsr_rts`: Short code fragment that copies a fixed memory address to a fixed memory address, then calls another routine (6 times in a row), then returns.
- `frag_moveq_move_jsr_dbra_rts`: Short code fragment that loads 15 into d0, then copies d0 to the stack, then calls another routine (3 times in a row), then copies the top of the stack to d0, then loops back while counting down d0, then returns.
- `frag_lea_move_moveq_dbra_adda_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 256 to d1, then loads 0 into d7, then loads 9 into d0, then copies d7 to the memory at a0 (advancing a0), then loops back while counting down d0, then adds 160 to a0, then loops back while counting down d1, then returns.
- `fill_tile_buffer`: Calculates memory addresses in a tile buffer using multiplication and addition, then writes a specific value to a series of locations in that buffer.
- `check_interrupt_status`: Checks interrupt status flags in hardware registers and waits for specific interrupt conditions before returning.
- `frag_movem_moveq_lea_move_dbra_rts`: Short code fragment that saves registers on the stack, then loads 15 into d0, then loads 0 into d1, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies d1 to the memory at a0 (advancing a0), then copies d1 to the memory at a1 (advancing a1), then loops back while counting down d0, then restores saved registers from the stack, then returns.
- `frag_moveq_lea_jsr_addi_cmpi_bne_rts`: Short code fragment that loads 0 into d0, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then calls another routine, then adds 2 to d0, then compares d0 with 64, then branches if the last result was not zero, then returns.
- `frag_lea_moveq_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then loads 15 into d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `copy_word_array`: Copies 16 words from one memory location to another using a loop with decrementing counter.
- `copy_color_table`: Copies 32 words from one memory location to a hardware register area, then calls three routines with different parameters, copies a value from a table based on a byte offset, and sets multiple audio registers.
- `frag_jsr_rts`: Short code fragment that calls another routine (2 times in a row), then returns.
- `clear_input_table`: Initializes a table of 16 long words to zero values, then checks bit 7 of a hardware register and returns.
- `convert_and_process_char`: Converts character data from a source buffer into numeric values, calls a function for specific characters, and updates a counter.
- `setup_graphics_registers`: Initializes multiple graphics hardware registers including DMA control, interrupt enable, sprite controls, and coprocessor settings.
- `frag_move_moveq_lea_dbra_rts`: Short code fragment that copies 0 to the memory at an offset from a6, then copies 2559 to d7, then loads 0 into d0, then loads the address of a fixed memory address into a1, then copies d0 to the memory at a1 (advancing a1), then loops back while counting down d7, then returns.
- `copy_tile_data`: Reads character values from a source buffer, uses them as offsets into another data table, and copies 8 bytes from that table to a destination buffer.
- `frag_lea_movea_bra`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies a fixed memory address to a2, then jumps to another point in the code.
- `frag_movea_lea_bra`: Short code fragment that copies a fixed memory address to a0, then loads the address of a fixed memory address into a1, then copies a fixed memory address to a2, then jumps to another point in the code.
- `frag_move_moveq_bsr_addi_dbra_rts`: Short code fragment that copies 0 to d1, then loads 15 into d7, then calls another routine, then copies d1 to the memory at an offset from a6, then adds 273 to d1, then loops back while counting down d7, then returns.
- `frag_btst_bne_rts`: Short code fragment that tests bit 7 of a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_move_moveq_bsr_subi_dbra_rts`: Short code fragment that copies 4095 to d1, then loads 15 into d7, then calls another routine, then copies d1 to the memory at an offset from a6, then subtracts 273 from d1, then loops back while counting down d7, then returns.
- `convert_hex_to_ascii`: Converts a 32-bit value into a 5-character ASCII hex string and stores it at a memory location.
- `configure_audio_dma`: Configures audio DMA channels by setting up register values for audio playback, including channel enable bits and sample addresses, then synchronizes with vertical position changes.
- `setup_audio_dma`: Configures audio DMA channels by setting up register values for audio playback, including channel enable bits and sample addresses.
- `synchronize_vertical_position`: Waits for a vertical position value to change, then sets up DMA control registers with specific values before returning.
- `update_random_seed`: Updates a sequence of 16 random seeds using a linear congruential generator algorithm with bit manipulation operations.
- `clear_and_setup`: Clears memory areas and sets up data structures while configuring hardware registers, then returns.
- `reset_audio_volumes`: Clears volume registers for four audio channels, checks a global flag, clears a fifth volume register if the flag is set, sets specific values to a hardware control register, and returns immediately.
- `update_audio_channels`: Configures four audio channels by setting up pointers to data structures and calling a routine with different channel parameters.
- `process_input_data`: Processes input data from a structure, updates velocity components and direction flags based on table lookups, and performs conditional operations including DMA configuration and bit flag clearing.
- `configure_audio_channels`: Sets up audio channel parameters including sample addresses and lengths, configures DMA control registers, and manages audio playback timing.
- `frag_bsr_move_andi_beq_lsl_movea_jmp`: Short code fragment that calls another routine, then copies the memory at an offset from a6 to d0, then masks d0 with 4095, then branches if the last result was zero, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then shifts d0 left by 2, then copies a location in the code to a0, then jumps to the address in a0.
- `copy_word_value`: Copies a 16-bit word value from one memory location to another using pointer arithmetic and register operations.
- `update_velocity_with_clamp`: Updates a velocity component by looking up a value in a table based on input data, clamps it against a limit, and sets a direction flag.
- `update_velocity_component`: Updates a velocity component by adding or subtracting a byte value, clamps it against a limit, and performs a lookup in a table based on the clamped value to determine a final result stored in a global variable.
- `scale_velocity_component`: Calculates a scaled velocity component using a lookup table and bit manipulation, then stores the result in a global structure.
- `process_input_byte`: Processes a byte value from a structure, extracts nibbles, performs bit operations on global variables, and updates memory locations based on conditional checks.
- `extract_nibble_and_store`: Extracts the low nibble of a byte value, stores it at offset 18 in the structure pointed to by a6, and returns.
- `advance_pointer_and_invert_byte`: When the high nibble of a parameter is nonzero, adds a table value to a counter.
