# Game 1281 (dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)[t +1 golden betrayal]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01281). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901536 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 14 of 14 named (0 author symbols), 14 of 14 described (1 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 18 of 18 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 425 of 425 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 14435 |
| code | 1300 |
| data | 884265 |
| unused | 1536 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7)[t +1 Golden Betrayal].adf:bootblock | 1 | 14 | 1 |
| logged disk load from Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7)[t +1 Golden Betrayal].adf boot-block load | 1 | 163 | 2 |
| logged disk load from Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7)[t +1 Golden Betrayal].adf boot-block load | 1 | 163 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 85 | 9 |

## Cases handled

- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)[t +1 golden betrayal]/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7)[t +1 Golden Betrayal].adf: custom-format (non-DOS) disk, 1755 non-empty blocks: matched against track-loaded segments
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)[t +1 golden betrayal]/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7)[t +1 Golden Betrayal].adf: the boot block reads 2048 bytes from disk offset 512 to an address (found statically); recovered as a unit
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)[t +1 golden betrayal]/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7)[t +1 Golden Betrayal].adf: the boot block reads 14336 bytes from disk offset 2560 to an address (found statically); recovered as a unit
- runtime segment an address: 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)[t +1 golden betrayal]/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7)[t +1 Golden Betrayal].adf: 2 logged loads cover 16384 bytes of the image
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)[t +1 golden betrayal]/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7)[t +1 Golden Betrayal].adf: blocks 33-214 (bytes 16896-110080) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.80)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)[t +1 golden betrayal]/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7)[t +1 Golden Betrayal].adf: blocks 216-304 (bytes 110592-156160) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.92)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)[t +1 golden betrayal]/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7)[t +1 Golden Betrayal].adf: blocks 306-880 (bytes 156672-451072) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.47)
- dragon's lair 2 - time warp (1990)(readysoft)[cr skr](disk 1 of 7)[t +1 golden betrayal]/Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7)[t +1 Golden Betrayal].adf: blocks 882-1759 (bytes 451584-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.60)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7)[t +1 Golden Betrayal].adf:bootblock

- `init_exec_library`: Loads the base address of the exec library into a6, sets a few fields of a structure pointed to by a1, then calls the exec library and jumps to the handler at the library base.

### logged disk load from Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7)[t +1 Golden Betrayal].adf boot-block load

- `load_and_rotate_value`: Pops a long from a stack pointer into a register, stores a byte of it into a structure, clears the condition codes, rotates the value right, and returns.
- `process_and_copy_data`: Repeatedly shifts a value right and, when a bit clears, calls a routine to process a chunk, then copies a run of bytes from one table to another and finally jumps to the destination address.

### logged disk load from Dragon's Lair 2 - Time Warp (1990)(ReadySoft)[cr SKR](Disk 1 of 7)[t +1 Golden Betrayal].adf boot-block load

- `shift_and_copy_byte`: Loads a long word from the address in a0 into d0, stores its low byte into d7, writes d7 as a 16-bit value to the structure pointed to by a5, shifts d0 right by one bit through the condition codes, and returns; otherwise it sets d3 to 4, d1 to 1, calls process_and_copy_data, adds d2 to d3, sets d1 to 9, and branches to 354.
- `process_and_copy_data_2`: Loops d1 times, shifting a 32-bit value in d0 right by one each iteration and calling a routine when a bit clears, accumulating the carry into d2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_global_counter`: Adds a value from a register to a global variable, and when that variable reaches 512 it resets it to zero and increments the following variable, otherwise it returns.
- `compare_and_call_routine`: Computes a difference between two global variables, and if the result is not less than a threshold it calls a routine at a fixed address, otherwise it returns.
- `loop_and_compare_status`: Calls a routine via a pointer, updates two global variables using a value from d1, and if the difference between them is zero, calls another routine; otherwise, it enters a loop that increments d0 and calls another routine until d0 equals 44, then calls another routine and returns.
- `frag_clr_move_lea_moveq_rts`: Short code fragment that clears a fixed memory address, then copies 2 to a fixed memory address, then loads the address of a location in the code into a0, then loads 0 into d0, then returns.
- `process_address_offset`: Calculates a value from d2, compares it to 1600, stores a pointer in a global variable, and if a comparison with 84320 matches, subtracts 86464 from a pointer, stores the result in a global variable, clears a 32-bit and a 16-bit global variable, and calls another routine.
- `increment_and_check_status_counters`: Increments a 16-bit value and a 32-bit value in global memory, then checks a status byte to conditionally call another routine and update a function pointer.
- `setup_globals_and_jump`: Tests a byte in a global variable, and if it is set it returns; otherwise it copies two longs between globals, clears a flag byte, and jumps to a fixed address.
- `conditionally_copy_globals`: Tests a byte in a global variable, and if it is zero it copies two longs from one pair of globals to another then jumps to a fixed address, otherwise it returns.
- `clear_and_jump_if_zero`: Clears a byte in a global variable, tests another byte, and if that byte is zero it clears it and jumps to a fixed address, otherwise it returns.
