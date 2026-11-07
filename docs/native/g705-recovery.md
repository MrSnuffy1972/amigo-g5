# Game 705 (F1): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00705). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901312 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 43 of 43 named (0 author symbols), 43 of 43 described (2 by rule, 41 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 15 of 15 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1480 of 1480 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12195 |
| code | 5568 |
| data | 675164 |
| padding | 1 |
| unused | 208384 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| F1.adf:bootblock | 1 | 124 | 10 |
| logged disk load from F1.adf boot-block load | 1 | 1325 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 4 |

## Cases handled

- F1/F1.adf: custom-format (non-DOS) disk, 1350 non-empty blocks: matched against track-loaded segments
- F1/F1.adf: the boot block reads 22528 bytes from disk offset 788480 to an address (found statically); recovered as a unit
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- F1/F1.adf: 1 logged loads cover 22528 bytes of the image
- F1/F1.adf: blocks 2-2 (bytes 1024-1536) hold ASCII text
- F1/F1.adf: blocks 1757-1759 (bytes 899584-901120) outside the filesystem hold data, not code (opcode density 0.044, entropy 3.99)
- 2 packed track region(s), 667136 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### F1.adf:bootblock

- `initialize_hardware_and_copy_instructions`: Sets values in certain hardware registers, calls a routine, calls another routine, and then loops to copy a sequence of instructions to a destination pointed to by a register.
- `call_search_and_reorder_buffer_data`: Saves registers to the stack, calls a routine that searches for a marker and reorders pointers, restores registers, and returns.
- `search_and_reorder_buffer_data`: Searches for a specific marker in memory, then recursively calls itself to populate a stack-based array with pointers, sorts those pointers in ascending order, and finally initializes a memory structure with a bitmask and a function pointer before copying the sorted pointers into a destination buffer.
- `initialize_and_reorder_buffer_data`: Initializes a memory structure with a specific bitmask and a function pointer, then copies a block of data from a source table to a destination, while also performing a nested loop that swaps or copies longwords between adjacent pointers in a buffer if certain pointer-order conditions are met.
- `swap_and_copy_buffer`: Copies a longword from a source to a destination, then loops to swap or copy data between two buffers using a loop that terminates when d4 reaches zero.
- `tiny_move`: Small routine of 1 instruction that copies (a6)+ to d1.
- `merge_masked_bit_values`: Combines a register value with a memory word under a mask: it takes the register bits selected by the mask, ORs in the masked memory bits shifted left by one, and returns the result.
- `xor_accumulate_longword_table`: Walks a table of 32-bit words a certain number of times, XORing each into an accumulator, then returns the accumulator masked by a register value.
- `return_failure_code`: Returns the all-ones value, used as a failure result.
- `update_conditional_tables`: Performs conditional checks on a value and a global, and if certain conditions are met, writes a word to a global and a byte to another global, or writes different values to a third global and a fourth global before returning.

### logged disk load from F1.adf boot-block load

- `process_string_property_table`: If a global counter is non-zero, it decrements the counter and enters a loop that reads a sequence of words from a table, sums adjacent words, and writes a bit-manipulated result into a destination table.
- `initialize_graphics_buffer_table`: Initializes a data structure by setting various memory offsets and values, then iterates through a list of values to populate a table with calculated memory addresses and parameters.
- `set_register_bits`: Sets a bit in a register and performs a logical OR on a register with a constant.
- `decrement_and_populate_table`: Decrements a global counter and, if it reaches zero, resets it to a specific value and performs a loop to populate a table with values derived from a register.
- `set_bitmask_buffer`: Sets specific bits in a memory buffer by iterating through a table of offsets and values.
- `clear_memory_word`: Writes a zero value to a specific offset within a structure and then enters an infinite loop.
- `initialize_structure_from_string`: Initializes a structure with several constant values, then parses a string to populate a table of properties by calculating an offset from each character's value.
- `scatter_bytes_from_table`: Subtracts thirty-two from a byte in d0, shifts it left by three, and then copies a sequence of bytes from a table at a calculated offset into a destination buffer at a fixed stride of thirty-two bytes, then branches to another routine.
- `process_input_and_update_structure`: Iterates based on a value in a structure, updates a field with an incrementing value, and then performs several structure assignments and calls another routine.
- `check_interrupt_request_bit`: Saves registers to the stack, checks if bit 5 of a hardware register is set, and if not, calls another routine before returning via rte.
- `clear_global_and_set_register`: Clears a global variable, calls another routine, and writes a value to the certain hardware register before returning via rte.
- `zero_data_area`: Performs a series of logical OR operations with zero on a register.
- `initialize_all_subroutines`: Calls a sequence of nine different subroutines that initialize or manipulate various data tables, pointers, and hardware or software settings, then returns.
- `initialize_data_tables`: Populates two separate tables with incrementing values and clears several fields in a structure.
- `populate_function_pointer_table`: Populates a table with a specific routine address by iterating through a memory range, writing the address into consecutive double-word slots within the table, and then returns.
- `load_global_pointers`: Stores several memory addresses into a series of global variables.
- `copy_and_scale_data`: Copies data between memory locations, scales bytes from a table by a constant, and then copies the results to a new destination using a loop.
- `set_global_byte`: Sets a byte in a global variable to one and then returns.
- `setup_blitter_graphics_tables`: Copies a source table into a larger destination buffer and then configures a series of blitter operations.
- `expand_bitmask_to_buffer`: Expands bit-packed values from a source table into a destination buffer by setting bytes to either zero or maximum value, then updates a series of pointers and hardware-related parameters in a structure, waiting for a specific bit flag in a register to clear before proceeding with further pointer updates.
- `reformat_data_table`: Iterates through a table of words and performs bitwise shifts and divisions to store transformed values into a new memory location.
- `copy_data_blocks`: Copies two blocks of data from a source table to a destination table using a fixed stride.
- `initialize_audio_and_random_tables`: If a certain bitmask check passes, it initializes a sequence of random numbers into a table, sets up several pointers or table offsets, and calls initialize_channel_structure and generate_pseudo_random_sequence multiple times.
- `initialize_channel_structure`: Clears several fields of a structure pointed to by a6, copies a 32-bit pointer from another structure, and sets a fixed longword, a mask byte, and a flag byte.
- `generate_pseudo_random_sequence`: Performs a loop that updates a global variable using a mathematical transformation and writes the results to a memory location, then decrements a global byte and performs conditional checks on several other global counters, calls a music sequence handler, and sets bit 15 of the DMA controller or a global variable.
- `step_music_channel_sequence`: Reads the next sequence event for one channel, loading its instrument and state from data tables, and updates the channel's pitch and volume counters.
- `set_register_bits_2`: Sets or clears specific bits in register d0 using a series of or instructions with immediate values.
- `mask_and_store_byte`: Masks the lower 4 bits of register d1 and stores the result into a byte at an offset of 47 from the address in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_argument_into_global`: Stores one argument into a shared global variable, choosing the source by the sign of a third argument.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block`: Copies a block of memory from the address in a1 to the address in a0 one longword at a time, using a dbra loop counter, and returns.
- `clear_long_buffer`: Zeroes 8000 consecutive 32-bit words starting at the address held in a shared pointer.
- `dispatch_then_spin`: Passes a global variable as an argument to a subroutine and then enters an endless loop that never returns.
- `toggle_index_and_load_global`: Flips bit two of a stored word, then loads a shared global from a table entry at an offset given by that word.
