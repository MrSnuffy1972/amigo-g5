# Game 1707 (thunderhawk (1991)(virgin)[cr skr][h pns]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01707). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (915904 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 34 of 34 named (0 author symbols), 34 of 34 described (2 by rule, 29 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 457 of 457 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 635 of 2009 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 64935 |
| code | 7868 |
| data | 834491 |
| padding | 930 |
| unused | 7680 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| ThunderHawk (1991)(Virgin)[cr SKR][h PNS].adf:bootblock | 1 | 14 | 1 |
| logged disk load from ThunderHawk (1991)(Virgin)[cr SKR][h PNS].adf boot-block load | 1 | 0 | 0 |
| logged disk load from ThunderHawk (1991)(Virgin)[cr SKR][h PNS].adf boot-block load | 1 | 22 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 42 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 50 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 53 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1722 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 106 | 5 |

## Cases handled

- thunderhawk (1991)(virgin)[cr skr][h pns]/ThunderHawk (1991)(Virgin)[cr SKR][h PNS].adf: custom-format (non-DOS) disk, 1743 non-empty blocks: matched against track-loaded segments
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 191 64 KB pages each with >= 29491 distinct PCs executed, 6254292 of its 6254294 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-joy slid through memory (an address: 191 64 KB pages each with >= 29491 distinct PCs executed, 3142718 of its 3142719 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-keys slid through memory (an address: 191 64 KB pages each with >= 29491 distinct PCs executed, 3142718 of its 3142719 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-mixed slid through memory (an address: 191 64 KB pages each with >= 29491 distinct PCs executed, 6231507 of its 6231509 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 191 64 KB pages each with >= 29491 distinct PCs executed, 4462946 of its 4462948 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 191 64 KB pages each with >= 29491 distinct PCs executed, 4462946 of its 4462948 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 191 64 KB pages each with >= 29491 distinct PCs executed, 4462949 of its 4462951 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 191 64 KB pages each with >= 29491 distinct PCs executed, 4462946 of its 4462948 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- thunderhawk (1991)(virgin)[cr skr][h pns]/ThunderHawk (1991)(Virgin)[cr SKR][h PNS].adf: the boot block reads 2048 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- thunderhawk (1991)(virgin)[cr skr][h pns]/ThunderHawk (1991)(Virgin)[cr SKR][h PNS].adf: the boot block reads 58368 bytes from disk offset 838144 to an address (found statically); recovered as a unit
- prefix run f1000 ignored: it logged 9210 PCs the canonical run never executed (not a prefix of it)
- prefix run f2000 ignored: it logged 9210 PCs the canonical run never executed (not a prefix of it)
- runtime segment an address (play-rs-m1500-unpk03): 40 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 50 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 50 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk03): 179 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 104 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- thunderhawk (1991)(virgin)[cr skr][h pns]/ThunderHawk (1991)(Virgin)[cr SKR][h PNS].adf: 2 logged loads cover 60416 bytes of the image
- thunderhawk (1991)(virgin)[cr skr][h pns]/ThunderHawk (1991)(Virgin)[cr SKR][h PNS].adf: blocks 11-20 (bytes 5632-10752) outside the filesystem hold data, not code (opcode density 0.037, entropy 2.87)
- thunderhawk (1991)(virgin)[cr skr][h pns]/ThunderHawk (1991)(Virgin)[cr SKR][h PNS].adf: blocks 22-1636 (bytes 11264-838144) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.17)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `copy_loader_and_jump_2` (ThunderHawk (1991)(Virgin)[cr SKR][h PNS].adf boot-block load +an address to an address (58368 bytes)): Writes stored values to the interrupt and DMA control registers, sets the disk control register, and writes a drive control value. Kind: loader-based integrity/environment check. Pass path: the branch where sRam01000574 != 0 (skipping the manual copy loop).
- `set_global_bytes` (runtime segment an address (play-rs-m1500-unpk03)): Stores four constant bytes into the CIAB Port B register. Kind: disk hardware/track validation check. Pass path: the instruction at 17E8 (bsr.w an address) or the branch following the hardware check at 17ED (btst.b #an address, an address.l).
- `set_port_and_call_routine` (runtime segment an address (play-rs-m1500-unpk03)): Writes 127 to the CIA B port, clears bit 3 of the CIA B port, calls another routine, and then loads a value from a global pointer into d0. Kind: checksum or hardware validation trap. Pass path: the branch where _u6__d0_1BDE != 0 (the 'else' block).
- `process_joystick_input` (runtime segment an address (play-rs-m1500-unpk03)): Saves registers, clears bit 1 of the CIA B port, and if d0 is non-zero, updates a global variable and a value at an offset in d0, then enters a loop calling another routine if certain conditions are met. Kind: disk hardware/timing check. Pass path: d0 != 0.
- `step_port_toward_target_value` (runtime segment an address (play-rs-m1500-unpk03)): Clears bit 0 and sets bit 1 of the CIA B port, then enters a loop that decrements d1 from a constant value until it reaches negative one. Kind: checksum/integrity trap. Pass path: the branch following the dbra loop (sub_18EC) which continues normal execution.
- `generate_bitmapped_data` (runtime segment an address (play-rs-m1500-unpk03)): Clears a global word and bit 2 of the CIA B port, sets up several hardware registers, and enters a nested loop that performs bitwise operations and XORs to populate a destination table based on source data. Kind: bitmapped data decompression/decryption with integrity check. Pass path: the loop continues and eventually returns via the sVar3 == -1 branch (successful decompression/validation).
- `initialize_hardware_and_tables` (runtime segment an address (play-rs-m1500-unpk02)): Sets two words in a structure via a6 using values from a5, writes three words of 32767 to offsets 150, 154, and 156 of a5, writes three bytes to the CIA B port, and fills two separate memory ranges with incrementing values using loops. Kind: initialization and table setup for protection/decryption. Pass path: the code continues to the next instruction after the call (lea.l an address(pc), a0).

## Functions


### ThunderHawk (1991)(Virgin)[cr SKR][h PNS].adf:bootblock

- `read_loader_and_relocate`: Fills in an I/O request's length, data address and offset fields with a read command, then sends it through the library's do-I/O entry.

### logged disk load from ThunderHawk (1991)(Virgin)[cr SKR][h PNS].adf boot-block load

- `copy_loader_and_jump_2`: Writes stored values to the interrupt and DMA control registers, sets the disk control register, and writes a drive control value.
- `relocate_words_and_jump`: Copies a block of words from a source pointer into low memory starting at a fixed address, then jumps into the copied code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clamp_and_store_values`: If a global byte is non-zero, it returns; otherwise, it calls another routine thirteen times with varying increments to d1, then clamps the resulting value between 43 and 244 and stores it at the address in a1.
- `clamp_and_store_value`: Clamps the word in d1 between 43 and 244, stores the resulting byte at the address in a1, and increments a1 by 8 before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_table_values`: Writes a sequence of incrementing values into a table at a fixed address using a loop that runs 199 times.
- `populate_memory_table`: Iteratively writes an incrementing value into a table, adjusting the destination pointer and size or offset based on a loop counter.
- `swap_bytes_at_offset`: Writes two bytes to a destination address, where the offsets are calculated from a register and a third register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_bitmapped_table`: Checks a global variable and, if non-zero, performs a loop that reads bytes from a global variable and writes a reordered or permuted bit-pattern or byte-pattern into a destination table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_trap`: Tests a global variable and, if non-zero, performs a trap and jumps to a specific address; otherwise it returns.
- `call_and_mask`: Calls a routine and returns the result bitwise-ORed with 70.
- `process_null_data_table`: This routine consists of a sequence of instructions that perform no functional operation on the data registers, effectively acting as a data table of zeroed bytes.
- `decrement_loop_counter`: Checks if bit 5 of a global variable is set; if so, it decrements a local 16-bit value from 32767 down to 0 in a loop, provided the bit remains set, otherwise it returns.
- `wait_for_button_release`: Checks bit 5 of the CIAA Port A register in a loop, saving and restoring d0, and returns when the bit is cleared.
- `process_data_stream`: Enters a loop that copies a null-terminated string into a local buffer, calls a routine, and then searches a table for a matching 16-byte sequence to execute a routine and return a value.
- `update_and_jump`: Clears a 16-bit global variable, moves a byte from a1 to a global variable, calls several routines, and then jumps to an address in a table.
- `run_sequence_of_calls`: If a global variable is zero, sets it to one, executes two loops of 39 calls to another routine, writes a value to a coprocessor register, and returns.
- `set_global_bytes`: Stores four constant bytes into the CIAB Port B register.
- `set_port_and_call_routine`: Writes 127 to the CIA B port, clears bit 3 of the CIA B port, calls another routine, and then loads a value from a global pointer into d0.
- `process_joystick_input`: Saves registers, clears bit 1 of the CIA B port, and if d0 is non-zero, updates a global variable and a value at an offset in d0, then enters a loop calling another routine if certain conditions are met.
- `step_port_toward_target_value`: Clears bit 0 and sets bit 1 of the CIA B port, then enters a loop that decrements d1 from a constant value until it reaches negative one.
- `check_and_copy_data`: Compares d0 to a global word, and if equal, checks a global byte and calls a routine to copy data from a table if d0 is non-zero.
- `copy_bitmapped_data`: Calls two routines, then if a value in d0 is non-zero, copies 128 words from a table at an offset calculated from d1 to a destination pointed to by a0, and finally clears a byte in a global variable.
- `perform_infinite_jump`: Performs a short branch to itself.
- `generate_bitmapped_data`: Clears a global word and bit 2 of the CIA B port, sets up several hardware registers, and enters a nested loop that performs bitwise operations and XORs to populate a destination table based on source data.
- `increment_and_check_status`: Increments a 16-bit word in a global variable, compares it to 5, and if not equal, returns; otherwise it clears d0 and returns.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `copy_data_table`: Checks bit 2 of the CIAA PRA register, and if set, performs several routine calls and copies 3103 words from a table at a fixed address to a destination pointed to by a global pointer.
- `copy_conditional_data`: Compares d0 to a global word, and if equal and a global byte is non-zero, copies a block of data from a table to a destination in a0, where the number of bytes copied is determined by a value in d2 or a value from the table if d0 is negative.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_trap`: Loads a value from a global variable into d0, performs a trap, and returns.
- `initialize_hardware_and_tables`: Sets two words in a structure via a6 using values from a5, writes three words of 32767 to offsets 150, 154, and 156 of a5, writes three bytes to the CIA B port, and fills two separate memory ranges with incrementing values using loops.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `render_tile_data`: Calls another routine, sets up hardware registers, processes a list of bytes to calculate offsets, and uses a loop to copy bytes from a source table to a destination based on calculated indices.
- `update_structure_from_library`: Sets three words at offsets 150, 154, and 156 of a5 to 32767, then copies two words from a6 into those same offsets, calls a routine via a pointer in a system library, and stores a value from the returned pointer into offset 128 of a5.
