# Game 1617 (rick dangerous (1989)(micro style)[cr qtx][t +5 trilogy]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01617). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902336 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 32 of 32 named (0 author symbols), 32 of 32 described (1 by rule, 29 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 239 of 239 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 454 of 454 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 1710 |
| data | 181042 |
| padding | 1016 |
| unused | 718336 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rick Dangerous (1989)(Micro Style)[cr QTX][t +5 Trilogy].adf:load | 2 | 208 | 15 |
| Rick Dangerous (1989)(Micro Style)[cr QTX][t +5 Trilogy].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 189 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |

## Cases handled

- runtime segment an address: 134 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 27 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
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
- `decode_rle_stream`: Decodes a bit-packed stream by expanding each run-length value into repeated bytes and writing them to an output buffer.
- `decode_and_move_rle_block`: Reads a run-length-encoded stream and expands it into a destination buffer, then scans the produced data to locate a marker byte and copy a surrounding block of bytes to a new location.
- `decode_rle_stream_6`: Decodes a run-length-encoded stream, expanding back-references and copying literal bytes into a destination buffer.
- `decode_rle_stream_5`: Decodes a run-length-encoded stream by copying literal bytes and expanding back-references into a destination buffer.
- `decode_rle_stream_4`: Decodes a run-length-encoded stream by copying literal bytes and expanding back-references into a destination buffer.
- `decode_rle_stream_3`: Decodes a run-length-encoded stream by copying literal bytes and expanding back-references into a destination buffer.
- `decode_rle_block`: Decodes a run-length-encoded block by copying literal bytes and expanding back-references into a destination buffer.
- `copy_rle_run_3`: Copies a run of bytes from a source table into a destination buffer, decrementing the destination pointer and counter for each byte written.
- `copy_rle_run_2`: Copies a run of bytes from a source table into a destination buffer, decrementing the destination pointer and counter for each byte written.
- `decode_rle_stream_2`: Decompresses a run-length-encoded byte stream, copying literal bytes and expanding repeated runs into a destination buffer.
- `copy_rle_run`: Copies a run of bytes from a source table into a destination buffer, decrementing the destination pointer and counter for each byte written.

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

- `enable_dma_channels_2`: A very short routine that enables DMA channels by writing to the DMA control register with bit 15 set, then returns.
- `enable_dma_channels`: Writes to the DMA control register to enable DMA channels, as indicated by the DMACON register being touched with bit 15 set.
