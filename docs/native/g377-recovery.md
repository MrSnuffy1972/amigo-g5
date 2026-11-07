# Game 377 (R-Type II): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00377). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909568 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 36 of 36 named (0 author symbols), 36 of 36 described (1 by rule, 22 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 436 of 436 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1258 of 1258 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 218897 |
| code | 3892 |
| data | 678075 |
| unused | 8704 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| R-Type II.adf:bootblock | 1 | 61 | 4 |
| logged disk load from R-Type II.adf boot-block load | 1 | 92 | 2 |
| logged disk load from R-Type II.adf boot-block load | 1 | 0 | 0 |
| logged disk load from R-Type II.adf boot-block load | 1 | 116 | 2 |
| logged disk load from R-Type II.adf boot-block load | 1 | 99 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 131 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 298 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 461 | 3 |

## Cases handled

- R-Type II/R-Type II.adf: custom-format (non-DOS) disk, 1626 non-empty blocks: matched against track-loaded segments
- R-Type II/R-Type II.adf: the boot block reads 18944 bytes from disk offset 822272 to an address (found statically); recovered as a unit
- R-Type II/R-Type II.adf: the boot block reads 56320 bytes from disk offset 844800 to an address (found statically); recovered as a unit
- R-Type II/R-Type II.adf: the boot block reads 147456 bytes from disk offset 1536 to an address (found statically); recovered as a unit
- R-Type II/R-Type II.adf: the boot block reads 5120 bytes from disk offset 895488 to an address (found statically); recovered as a unit
- runtime segment an address: 103 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 289 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 44 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- R-Type II/R-Type II.adf: 4 logged loads cover 222720 bytes of the image
- R-Type II/R-Type II.adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.004, entropy 1.99)
- R-Type II/R-Type II.adf: blocks 755-762 (bytes 386560-390656) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- R-Type II/R-Type II.adf: blocks 764-1436 (bytes 391168-735744) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.84)
- R-Type II/R-Type II.adf: blocks 1439-1439 (bytes 736768-737280) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.12)
- R-Type II/R-Type II.adf: blocks 1444-1445 (bytes 739328-740352) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.37)
- R-Type II/R-Type II.adf: blocks 1448-1605 (bytes 741376-822272) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.71)
- 1 packed track region(s), 237056 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### R-Type II.adf:bootblock

- `read_disk_blocks_via_doio`: Issues a sequence of disk requests through exec DoIO into several memory destinations, including a request with command nine and zero length between reads, then calls a fixed routine and writes the colour and interrupt registers; the decompiler reports bad instruction data at the end, so the tail is unreliable.
- `wait_until_flag_clear`: Busy-waits while bit six of a global is set, then returns; the listed facts also name the DMA control read register as touched.
- `read_payload_then_relocate_code`: Sends a read request through DoIO for five kilobytes from a disk offset into a destination, then copies a 33-byte block of its own code into low memory.
- `execute_trap_zero`: Invokes trap zero and returns the combined 64-bit value of the D0 and D1 registers.

### logged disk load from R-Type II.adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from R-Type II.adf boot-block load

- `select_half_or_input`: Returns the input word unchanged unless the D7 word equals one, in which case it returns the D6 word shifted right by one bit, so half its value.
- `lookup_masked_table_word`: Reads a word from a table indexed by twice the D1 word, ANDs it with the D6 word, and returns the result.

### logged disk load from R-Type II.adf boot-block load

- `fetch_bitstream_longword`: Reads a longword just before the current input pointer, stores its low byte into an output word, and returns the value shifted right one bit with the top bit set, the pattern of a bit-stream reader.
- `shift_bits_with_refill`: Shifts a bit buffer right once per count taken from the D1 register, refilling the buffer from another routine whenever it becomes zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_button_state`: Checks a bit in a global variable and a character in a structure to modify a value, then increments a pointer and updates a status flag if a certain limit is reached.
- `initialize_data_tables`: Copies a block of data from a source buffer to a destination buffer and then performs a secondary, interleaved copy of a larger data block into multiple separate memory buffers.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `traverse_and_update_list`: Iterates through a list of bytes, updating a global variable based on the value or presence of a certain byte, and calls another routine if a certain condition is met.
- `execute_repeated_function`: Executes a function pointer a fixed number of times.
- `write_multiple_offsets`: Writes a byte from one register and another byte from a second register to several different offsets within a structure.
- `write_different_offsets`: Writes a byte from one register to certain offsets and a byte from a different register to another offset within a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_input_and_audio`: Checks a global variable, updates certain memory locations based on bit-tests, increments a global counter, and calls routines for sound sequencing and music playback.
- `decrement_and_update_button_timers`: Decrements a global timer and, if the timer reaches zero, updates a table of button-related values by subtracting a constant or decrementing bit-masked nibbles/bits.
- `copy_memory_block`: Copies a fixed number of words from a source pointer to a destination pointer.
- `update_global_memory_block`: Stores a register value plus a constant into a global variable and writes a sequence of constant values to a block of memory or a set of global variables.
- `store_register_to_globals`: Stores the contents of certain registers or modified register values into a sequence of global variables and sets a global variable to a constant value.
- `check_global_bit`: Returns the least significant bit of a global byte.
- `wait_for_memory_change`: Enters an infinite loop that waits until a value at a memory location, offset by a constant, no longer equals itself.
- `process_bit_manipulation`: Performs complex bitwise masking, shifting, and concatenation on register values and returns a computed result.
- `copy_memory_block_with_decrement`: Copies a block of memory backwards from a source pointer to a destination pointer, then performs a second, larger block copy in reverse, and finally copies a block of memory forwards or backwards depending on the pointer direction.
- `copy_and_decrement_memory_block`: Copies a block of memory backwards from a source pointer to a destination pointer, while decrementing a counter and performing conditional checks on register or memory-based values.
- `decrement_counter_loop`: Decrements a register value in a loop until it reaches a specific negative value.
- `test_register_bits`: Tests if a specific bit in a register is clear and if a bit in another register is set without any side effects.
- `process_bit_shifts`: Performs bitwise shifts and conditional checks on register values and memory contents while decrementing a counter.
- `load_from_indexed_address`: Loads a longword from a memory address calculated using a shifted register and a fixed offset, then calls another routine.
- `invoke_subroutine`: Calls another routine and returns.
- `call_subroutine`: Executes another routine located at a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `step_four_channel_sound_sequencer`: Steps through four sound channels on each call: for each active channel it advances a note timer, and on expiry reads the next note's pitch, volume and sample address from tables and writes them to the audio channel registers, with delay loops in between.
- `run_four_channel_music_player`: Steps four sound channels on each call: for each active channel it advances a timer and, on expiry, reads the next entry from a stream table to set the pitch and sample address, then writes the volume registers of all four channels.
- `update_slot_bit_table`: Advances a per-slot index into a table and either sets one bit in a mask buffer and copies a longword, or emits a stored byte, returning when a terminator byte is reached; the decompiler reports bad instruction data, so the tail is unreliable.
