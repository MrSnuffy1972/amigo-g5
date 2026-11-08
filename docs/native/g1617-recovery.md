# Game 1617 (rick dangerous (1989)(micro style)[cr qtx][t +5 trilogy]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01617). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902352 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 21 of 21 named (0 author symbols), 21 of 21 described (1 by rule, 18 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 295 of 295 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 457 of 457 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 1726 |
| data | 181042 |
| padding | 1016 |
| unused | 718336 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rick Dangerous (1989)(Micro Style)[cr QTX][t +5 Trilogy].adf:load | 2 | 208 | 4 |
| Rick Dangerous (1989)(Micro Style)[cr QTX][t +5 Trilogy].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 189 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |

## Cases handled

- runtime segment an address: 143 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 27 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (1 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- rick dangerous (1989)(micro style)[cr qtx][t +5 trilogy]/Rick Dangerous (1989)(Micro Style)[cr QTX][t +5 Trilogy].adf:load: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rick Dangerous (1989)(Micro Style)[cr QTX][t +5 Trilogy].adf:load

- `decode_rle_stream_7_2`: Decodes a bit-packed run-length-encoded stream, reading a control bit per output byte and either copying a previously produced byte a variable number of times or emitting a fresh byte read from the compressed data, writing the results into an output buffer.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `decode_rle_stream_7`: Decodes a bit-packed run-length stream, expanding each run into repeated bytes and writing them to an output buffer.

### Rick Dangerous (1989)(Micro Style)[cr QTX][t +5 Trilogy].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_graphics`: Clears a block of memory, interacts with DMA, video, and joystick registers, and performs several library calls and global variable increments.
- `toggle_and_loop`: Sets a global variable to a boolean state and then calls another routine three times in a loop.
- `update_flag_and_loop`: Compares a global variable to zero, stores the result in another global variable, and then calls another routine three times.
- `update_flag_and_loop_2`: Sets a global variable based on whether another global variable is zero and executes a loop that calls another routine three times.
- `set_flag_and_loop`: Sets a global variable based on a comparison and then calls another routine three times in a loop.
- `increment_global_with_loop`: Increments a global variable and, if it does not equal a specific value, executes a loop that calls another routine three times.
- `decrement_and_loop`: Decrements a global variable and, if it is not zero, performs a loop that calls another routine three times.
- `update_state_and_loop`: Modifies a global variable based on comparisons or arithmetic and executes a loop that calls another routine three times.
- `update_blt_graphics_state`: Updates a global counter and performs conditional bitwise operations on memory locations.
- `empty_routine`: This routine contains three instructions but no decompiled logic is available.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_sync_state`: Adds a value to a memory location, performs conditional bitwise XOR operations on a table, and copies the result to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sync_state_on_zero`: Checks if a memory value is zero, performs conditional bitwise XOR operations on a table, and copies the value to another location.
- `nested_loop_execution`: Executes a nested loop structure that calls another routine multiple times.
- `copy_memory_block`: Performs a loop to copy bytes from a source address to a destination address with a fixed stride.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_and_update_pointer`: Calls a subroutine at a long address and then stores a long constant into a memory location pointed to by register 20.
- `enable_dma_channel`: Sets bit 15 of the DMA control register to enable a DMA channel.
