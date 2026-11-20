# Game 3780 (GaryLinekersHotShot v1.0 0883): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03780). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1190077 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 25 of 25 named (0 author symbols), 25 of 25 described (8 by rule, 13 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 398294 bytes, 17 functions |
| Independent decoder cross-check (Ghidra against capstone) | 461 of 461 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 133594 |
| code | 1748 |
| data | 1054723 |
| padding | 12 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 57 | 8 |
| main (raw code file, load address unknown) | 1 | 79 | 5 |
| music (raw code file, load address unknown) | 1 | 325 | 12 |

## Cases handled

- GaryLinekersHotShot_v1.0_0883/GaryLinekersHotShot_v1.0_0883.hdf: hard-disk image (AmigaDOS OFS, 1280 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree GaryLinekersHotShot_v1.0_0883_hdf, which is recovered file by file; no block outside the filesystem holds data
- GaryLinekersHotShot_v1.0_0883/GaryLinekersHotShot_v1.0_0883_hdf/GaryLinekersHotShot/data/main: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- GaryLinekersHotShot_v1.0_0883/GaryLinekersHotShot_v1.0_0883_hdf/GaryLinekersHotShot/data/music: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 2 never-loaded region(s), 398294 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `setup_and_branch`: Initializes pointers to global variables, calls another routine with a parameter, and branches conditionally based on the result of that call.
- `initialize_game_structure`: Sets up a game structure at offset 1024 of a5, checks for specific magic values at offsets 1024 and 140, and branches based on comparison results.
- `frag_movea_lea_move_dbra`: Short code fragment that copies a5 to a0, then loads the address of a fixed memory address into a1, then copies 17660 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1) (4 times in a row), then loops back while counting down d0.
- `frag_lea_suba_movea_jsr_jmp`: Short code fragment that loads the address of a location in the code into a0, then subtracts a1 from a1, then copies a location in the code to a2, then calls another routine, then jumps to another point in the code.
- `frag_movem_movea_jsr_tst_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to a2, then calls another routine, then tests d0, then restores saved registers from the stack, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_pea_move_addq_rts`: Short code fragment that pushes the address of a fixed memory address onto the stack, then copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.

### main (raw code file, load address unknown)

- `store_pointer_and_trap`: Loads an address into a register, stores it to a global variable, makes a system call with trap number one, and returns immediately.
- `initialize_video_registers`: Configures multiple hardware registers including display and DMA settings, performs memory transfers, and sets up interrupt handling for video processing.
- `frag_cmpi_bne_clr_tst_moveq_jsr_bra`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was not zero, then clears a fixed memory address, then tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then calls another routine, then jumps to another point in the code.
- `update_game_counter`: Increments a global counter and performs several system calls including graphics and sound functions, updating state variables at specific memory locations.
- `accumulate_byte_values`: Reads four byte values from fixed offsets, accumulates them with an increment, then stores the result and individual bytes to different memory locations.

### music (raw code file, load address unknown)

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `initialize_audio_parameters`: Sets up audio channel parameters by writing specific values to hardware registers, initializes volume and period settings for four audio channels, and performs arithmetic operations to calculate and store updated parameter values in a data structure.
- `initialize_audio_channels`: Sets up audio channel parameters by writing specific values to hardware registers, initializes volume and period settings for four audio channels.
- `process_audio_channel`: Checks channel state, decrements a counter, processes audio data through a function call, and updates various audio parameters based on comparison results.
- `update_audio_parameters`: Checks channel state conditions, updates audio period register with a flag bit set, decrements counter, and performs memory operations based on the counter value.
- `set_flag_and_accumulate_short`: Writes a constant flag word into one structure and adds a short field from another structure into its short field, then returns.
- `update_audio_channel`: Manages audio channel state by decrementing a counter and updating hardware registers when the counter reaches zero, or setting a flag bit in a register when the counter is positive.
- `decrement_counters_and_flip_sign`: Decrements two counters in a channel structure and accumulates other fields into a running total; it returns early while the first counter is still nonzero, and when the second counter expires it reloads it from another structure and flips the sign of a field.
- `mask_interrupt_status_bits`: Masks a global value with a fixed bit pattern, shifts it right seven bits and stores the result, then returns the input word unchanged; the listed facts also name the DMA and interrupt enable and request registers as touched.
- `decrement_and_check_counters`: Decrements two counters in a structure and updates values based on comparison results, then clears a byte flag after completing the loop iterations.
- `initialize_audio_channel`: Sets up audio channel parameters including volume, frequency, and waveform data based on input values and table lookups, then configures interrupt handling and hardware registers for audio playback.
