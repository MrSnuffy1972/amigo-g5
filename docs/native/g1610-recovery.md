# Game 1610 (rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01610). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 20 of 20 named (0 author symbols), 20 of 20 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 48 of 48 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 94208 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 400 of 400 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 64397 |
| code | 1376 |
| data | 686159 |
| padding | 4 |
| unused | 157184 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf:bootblock | 1 | 8 | 1 |
| logged disk load from Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf boot-block load | 1 | 327 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 40 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 3 |
| Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf bytes 34816-88576 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf bytes 91648-120832 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf bytes 256000-267264 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: custom-format (non-DOS) disk, 1440 non-empty blocks: matched against track-loaded segments
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: the boot block reads 67584 bytes from disk offset 788480 to an address (found statically); recovered as a unit
- runtime segment an address: 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: bytes 34816-88576 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: bytes 91648-120832 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: bytes 256000-267264 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: 4 logged loads cover 161792 bytes of the image
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.65)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 22-29 (bytes 11264-15360) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.12)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 32-39 (bytes 16384-20480) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.95)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 65-66 (bytes 33280-34304) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.29)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 242-498 (bytes 123904-255488) outside the filesystem hold data, not code (opcode density 0.013, entropy 4.45)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 523-523 (bytes 267776-268288) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.61)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 525-525 (bytes 268800-269312) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.48)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 527-551 (bytes 269824-282624) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.26)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 553-727 (bytes 283136-372736) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.77)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 729-823 (bytes 373248-421888) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.17)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 825-925 (bytes 422400-474112) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.53)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 927-1037 (bytes 474624-531456) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.90)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 1039-1149 (bytes 531968-588800) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.05)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 1151-1273 (bytes 589312-652288) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.39)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 1275-1336 (bytes 652800-684544) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.86)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 1347-1347 (bytes 689664-690176) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.79)
- rainbow islands (1987)(ocean)[cr tic - freestyle][t +1 tic - freestyle][a]/Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf: blocks 1349-1400 (bytes 690688-717312) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.33)
- 3 never-loaded region(s), 94208 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf:bootblock

- `issue_boot_block_io_request`: Fills an exec I/O request block with a command, a length and two addresses, issues it through the DoIO library call, then calls a library vector, three unnamed routines, and clears a global before entering a loop.

### logged disk load from Rainbow Islands (1987)(Ocean)[cr TIC - Freestyle][t +1 TIC - Freestyle][a].adf boot-block load

- `update_audio_dma_and_interrupts`: Updates several status bytes using a mask of 1920, which selects the audio interrupt bits, and adds a value to a stored byte with carry.
- `decrement_timer_step_value`: Decrements a countdown byte in an object; when it reaches zero it reloads it from a table, and a zero table entry stops the update.
- `process_byte_block_with_helpers`: A long routine that works on a block of bytes through several pointers and calls three unnamed routines.
- `reset_state_and_control_words`: Clears five state words in program data, then writes two control values to registers listed in the facts as the disk-and-audio control register and the DMA control register.
- `copy_counted_byte_run`: Reads a count from the input bytes and copies data between pointers in a byte loop.
- `advance_counted_byte_copy`: Decrements a count byte in a structure and, while it stays non-zero, copies bytes between pointers, calling one unnamed routine.
- `reset_object_timer_from_input`: When an object's mode byte is clear, it zeroes a counter, sets the mode value, and stores a byte in a global.
- `lookup_table_into_object_fields`: Looks up a byte in a table indexed by a 16-bit value and stores it, along with a second table byte, into two fields of an object.
- `update_value_from_table_record`: When a mode byte is set, it looks up a ten-byte table record by index, decrements a counter, and stores a difference and an offset in the output fields.
- `copy_word_table_passes`: Runs a loop of fifteen passes over a table in program data, copying 16-bit words between pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_indirect_call_by_offset`: Calls a routine at an address formed by adding a 16-bit offset to a base pointer register, then returns.
- `count_and_call_table_handler`: Increments a global counter, then calls a routine whose address is read from a fixed field of a structure that a global points to.
- `dispatch_video_interrupt_handlers`: Reads the interrupt request and enable registers, masks them with a constant covering the copper, vertical blank and blitter bits, writes the result back to acknowledge them, and calls a handler through a pointer for each set bit, checked in order.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_indirect_call_by_offset_2`: Calls a routine at an address formed by adding a 16-bit offset to a base pointer register, then returns.
- `store_word_triple`: Copies three consecutive 16-bit values from a source pointer into three fields of a structure, then returns minus one.
- `execute_callback_chain`: Walks a singly linked list whose head pointer is read from a fixed field of a base structure, calling a function pointer stored in each node.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_bad_instruction_data`: The bytes do not decode to a valid instruction, so there is no real routine.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `clear_bit_range_in_word`: Clears a range of bits in a memory word, with the range computed from a register, then reaches a bad instruction that the decompiler cuts off.
