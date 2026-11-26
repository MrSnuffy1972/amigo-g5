# Game 943 (cadaver (1991)(imageworks)(m3)(disk 1 of 2)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00943). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903264 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 22 of 22 named (0 author symbols), 22 of 22 described (6 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 105 of 105 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 148992 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 557 of 557 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 3154 |
| data | 872971 |
| padding | 3 |
| unused | 27136 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf:bootblock | 1 | 54 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 397 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 68 | 4 |
| Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf bytes 431104-505856 (code never loaded by a reference run) | 1 | 0 | 0 |
| Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf bytes 664576-738816 (code never loaded by a reference run) | 1 | 19 | 0 |

## Cases handled

- cadaver (1991)(imageworks)(m3)(disk 1 of 2)/Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf: custom-format (non-DOS) disk, 1705 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 106 of the 106 PCs the canonical run logged (325 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 8 64 KB pages each with >= 29491 distinct PCs executed, 171890 of its 171894 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 8 64 KB pages each with >= 29491 distinct PCs executed, 171890 of its 171894 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 8 64 KB pages each with >= 29491 distinct PCs executed, 171890 of its 171894 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 8 64 KB pages each with >= 29491 distinct PCs executed, 171890 of its 171894 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- cadaver (1991)(imageworks)(m3)(disk 1 of 2)/Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf: bytes 431104-505856 look like code no run loaded: recovered position-independently (load address unknown)
- cadaver (1991)(imageworks)(m3)(disk 1 of 2)/Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf: bytes 664576-738816 look like code no run loaded: recovered position-independently (load address unknown)
- cadaver (1991)(imageworks)(m3)(disk 1 of 2)/Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf: 2 logged loads cover 148992 bytes of the image
- cadaver (1991)(imageworks)(m3)(disk 1 of 2)/Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf: 2 track-loaded blocks found in recovered runtime segments and classified through them
- cadaver (1991)(imageworks)(m3)(disk 1 of 2)/Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf: blocks 2-37 (bytes 1024-19456) outside the filesystem hold data, not code (opcode density 0.049, entropy 4.87)
- cadaver (1991)(imageworks)(m3)(disk 1 of 2)/Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf: blocks 39-82 (bytes 19968-42496) outside the filesystem hold data, not code (opcode density 0.019, entropy 5.12)
- cadaver (1991)(imageworks)(m3)(disk 1 of 2)/Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf: blocks 84-747 (bytes 43008-382976) outside the filesystem hold data, not code (opcode density 0.021, entropy 7.07)
- cadaver (1991)(imageworks)(m3)(disk 1 of 2)/Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf: blocks 764-840 (bytes 391168-430592) outside the filesystem hold data, not code (opcode density 0.028, entropy 6.93)
- cadaver (1991)(imageworks)(m3)(disk 1 of 2)/Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf: blocks 1004-1296 (bytes 514048-664064) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.52)
- cadaver (1991)(imageworks)(m3)(disk 1 of 2)/Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf: blocks 1459-1752 (bytes 747008-897536) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.52)
- cadaver (1991)(imageworks)(m3)(disk 1 of 2)/Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf: blocks 1755-1756 (bytes 898560-899584) outside the filesystem hold data, not code (opcode density 0.049, entropy 4.72)
- cadaver (1991)(imageworks)(m3)(disk 1 of 2)/Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf: blocks 1758-1759 (bytes 900096-901120) hold ASCII text
- 2 never-loaded region(s), 148992 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Cadaver (1991)(ImageWorks)(M3)(Disk 1 of 2).adf:bootblock

- `boot_probe_then_load_stage`: Makes exec library calls to probe for a resource, and if none is available copies a fallback loader into low memory; otherwise it issues a disk read through the exec library to load the next program stage and jumps to it.
- `boot_fallback_load_stage`: Performs the same resource check as the boot entry, copying a fallback routine into low memory when no resource is found, then issues a disk read request through the exec library and jumps to the loaded stage.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_ori_movem_rts`: Short code fragment that merges 156 into d6, then restores saved registers from the stack, then returns.
- `conditionally_call_routine`: Tests a word at a global address and calls another routine if that word is zero.
- `frag_lea_bsr`: Short code fragment that loads the address of a fixed memory address into a4, then calls another routine, then loads the address of a fixed memory address into a4, then calls another routine, then loads the address of a fixed memory address into a4, then calls another routine, then loads the address of a fixed memory address into a4.
- `update_hardware_configuration`: Updates several fields in a structure and modifies hardware registers related to interrupts and DMA based on conditional logic.
- `load_and_call`: Loads addresses from global variables and calls a routine with each address in a4, repeating this process four times.
- `tick_countdown_and_call`: When a field is nonzero, decrements a countdown byte in the record and calls an unnamed routine when the countdown reaches zero.
- `process_state_transitions`: Manages complex state changes within a structure by updating various word and byte fields based on comparisons with a table of values.
- `initialize_audio_and_structures`: Sets up audio hardware registers, copies a block of data using a loop, and populates several structures with pointers, constants, and hardware register addresses.
- `iterate_object_list`: Saves all general purpose and address registers to the stack, calculates an offset into a table using a word value, loads a byte from that table location, calls another routine with the byte value, increments a counter, and loops until the counter reaches zero.
- `initialize_structure_and_audio`: Clears a block of memory using a loop, populates a structure with pointers and constants, and updates audio hardware registers and a target structure.
- `update_game_state_and_hardware`: Updates hardware registers for interrupts and DMA while managing state counters and structure fields.
- `frag_move_bra`: Short code fragment that copies the memory at a2 (advancing a2) to d0, then copies d0 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies the memory at a2 (advancing a2) to d0, then jumps to another point in the code.
- `frag_movea_move_bra`: Short code fragment that copies the memory at an offset from a4 to a2, then copies a2 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_move_mulu_bra`: Short code fragment that copies the memory at a2 (advancing a2) to d0, then copies the memory at a6 to d1, then multiplies d0 by d1 (unsigned), then copies d0 to the memory at an offset from a4, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_modify_flags`: Copies data backwards from one memory location to another while processing character codes from a table, modifying a status byte based on specific character values, and updates a global variable with the last processed character.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `handle_interrupt_and_cop`: Checks bit 5 of a custom chip register, increments a global variable, performs several updates to a structure, updates custom chip registers, calls two routines, and executes a trap instruction.
- `update_status_byte`: Checks bit 7 of a byte from the CIAA port, sets bit 7 of d1 if set, reads a word from a custom chip, calls another routine, and updates a byte at offset 569 of a structure in a5 by ORing it with bits 6 and 7 of d1.
- `decode_joystick_word_to_direction_bits`: Takes a joystick word in d0 and sets direction bits in d1 for up, down, left and right by testing bit patterns and an xor of shifted copies.
