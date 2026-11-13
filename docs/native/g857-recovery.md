# Game 857 (adventures of robin hood, the (1991)(millennium)(m4)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00857). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901280 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 18 of 18 named (0 author symbols), 18 of 18 described (2 by rule, 14 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3 of 3 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 201 of 201 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 22036 |
| code | 654 |
| data | 785918 |
| unused | 92672 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf:bootblock | 1 | 8 | 1 |
| logged disk load from Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf boot-block load | 1 | 159 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 34 | 1 |

## Cases handled

- adventures of robin hood, the (1991)(millennium)(m4)[cr skr]/Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf: custom-format (non-DOS) disk, 1577 non-empty blocks: matched against track-loaded segments
- adventures of robin hood, the (1991)(millennium)(m4)[cr skr]/Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf: the boot block reads 23040 bytes from disk offset 878080 to an address (found statically); recovered as a unit
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- adventures of robin hood, the (1991)(millennium)(m4)[cr skr]/Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf: 1 logged loads cover 23040 bytes of the image
- adventures of robin hood, the (1991)(millennium)(m4)[cr skr]/Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf: blocks 294-300 (bytes 150528-154112) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.42)
- adventures of robin hood, the (1991)(millennium)(m4)[cr skr]/Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf: blocks 490-519 (bytes 250880-266240) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.09)
- adventures of robin hood, the (1991)(millennium)(m4)[cr skr]/Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf: blocks 527-556 (bytes 269824-285184) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.10)
- adventures of robin hood, the (1991)(millennium)(m4)[cr skr]/Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf: blocks 564-565 (bytes 288768-289792) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.07)
- 25 packed track region(s), 749056 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf:bootblock

- `load_disk_data_and_init`: Fills a disk read request for 23040 bytes from a fixed offset into a load address, submits it through a device call, then zeroes 81 longwords, copies 31 longwords into place and calls a routine.

### logged disk load from Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf boot-block load

- `initialize_system_and_jump`: Opens a library, performs several library calls, clears a memory block via a loop, copies a block of data via a loop, configures custom hardware registers, waits for a bit in a hardware register to clear, and jumps to another routine.
- `initialize_hardware_and_trap`: Loads the custom hardware base into a6, sets several custom register values to 32767, sets a bit in a hardware register, and calls a trap.
- `set_status_and_call_twice`: Sets the processor status register, sets a7 to a specific address, loads a6 from a pointer in a5, and calls another routine twice using values from a6 at specific offsets before clearing a location in a5 and jumping to the address in a4.
- `compare_and_store_pointer`: Compares a long value in d1 against a constant, and if it is greater than or equal to that constant, stores d0 into a location pointed to by a5 and jumps to the address in a4; otherwise it returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `load_custom_base`: Loads the address of the custom hardware registers into a6 and branches to another routine.
- `write_jump_target`: Writes the word 20083 to a global variable, writes the byte 8 to another global variable, and jumps to a long address.
- `setup_and_jump`: Loads a pointer into a0, sets d0 to a constant, calculates an address in a1, sets a2 to a constant, calls another routine, and jumps to the address in a2.
- `parse_and_copy_data`: Checks if a long value at a0 matches a constant, then uses d1 and d5 to calculate offsets, calls another routine multiple times, and performs a loop to copy bytes from a source to a destination based on values in d1 and d2.
- `shift_and_branch`: Performs a logical shift right on d5 by one bit and returns if the result is not zero; otherwise it calls another routine.
- `rotate_and_return`: Loads a long value from the address in a0 into d5, rotates the bits of d5 left by one, and returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (subq.w) and then falls through into the code that follows, with no return of its own.
- `loop_and_clear_word`: Sets d1 to zero, shifts d5 right by one, and if d5 is not zero, rotates d1 left by one and uses d0 as a loop counter to repeat; it then clears a word at an offset of a6 and returns.
- `rotate_and_branch`: Loads a long value from the address in a0 into d5, rotates the bits of d5 left by one, and branches to another routine.
- `execute_conditional_logic`: Calls another routine, loads a byte from a table using a5 and d1, compares d1 to d6, and if they are not equal, calls another routine and performs a series of conditional calls and data movements involving d1, d2, d3, and d7.
- `move_and_decrement_loop`: Calls another routine, moves d1 to d3, increments d2, and uses d2 as a loop counter to move bytes from a table at a3 into a location at a3, then clears a word at an offset of a6, compares a3 to a2, and returns if a3 is not less than a2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decode_table_coded_rows`: When a block counter expires, unpacks 16 rows of words from a byte stream, each value being a byte plus or minus a lookup-table entry, written at a fixed row stride.
