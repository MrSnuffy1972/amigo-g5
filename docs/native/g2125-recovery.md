# Game 2125 (mortal kombat ii (1994)(acclaim)[cr psg][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02125). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3615888 top-level bytes in 22 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 14 raw-word lines |
| Functions named and described | pass, 84 of 84 named (0 author symbols), 84 of 84 described (7 by rule, 77 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 246 of 246 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 5120 bytes, 46 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2268 of 2385 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 23900 |
| code | 7900 |
| data | 2871088 |
| padding | 808 |
| unused | 712192 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 1 of 4).adf:bootblock | 1 | 49 | 2 |
| logged disk load from Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 1 of 4).adf boot-block load | 1 | 116 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1241 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 153 | 6 |
| Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 1 of 4).adf bytes 1024-6144 (code never loaded by a reference run) | 1 | 826 | 46 |

## Cases handled

- mortal kombat ii (1994)(acclaim)[cr psg][a](disk 1 of 4)/Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 1 of 4).adf: custom-format (non-DOS) disk, 1621 non-empty blocks: matched against track-loaded segments
- mortal kombat ii (1994)(acclaim)[cr psg][a](disk 2 of 4)/Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 2 of 4).adf: custom-format (non-DOS) disk, 1536 non-empty blocks: matched against track-loaded segments
- mortal kombat ii (1994)(acclaim)[cr psg][a](disk 3 of 4)/Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 3 of 4).adf: custom-format (non-DOS) disk, 1624 non-empty blocks: matched against track-loaded segments
- mortal kombat ii (1994)(acclaim)[cr psg][a](disk 4 of 4)/Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 4 of 4).adf: custom-format (non-DOS) disk, 860 non-empty blocks: matched against track-loaded segments
- mortal kombat ii (1994)(acclaim)[cr psg][a](disk 1 of 4)/Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 1 of 4).adf: the boot block reads 18944 bytes from disk offset 817152 to an address (found statically); recovered as a unit
- runtime segment an address: 108 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 138 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- mortal kombat ii (1994)(acclaim)[cr psg][a](disk 1 of 4)/Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 1 of 4).adf: bytes 1024-6144 look like code no run loaded: recovered position-independently (load address unknown)
- mortal kombat ii (1994)(acclaim)[cr psg][a](disk 1 of 4)/Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 1 of 4).adf: 2 logged loads cover 24064 bytes of the image
- mortal kombat ii (1994)(acclaim)[cr psg][a](disk 1 of 4)/Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 1 of 4).adf: blocks 22-1595 (bytes 11264-817152) hold a RNC1: Rob Northen RNC1 Compressor stream (805888 -> 32000 bytes)
- mortal kombat ii (1994)(acclaim)[cr psg][a](disk 2 of 4)/Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 2 of 4).adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.46)
- mortal kombat ii (1994)(acclaim)[cr psg][a](disk 2 of 4)/Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 2 of 4).adf: blocks 22-1556 (bytes 11264-797184) hold 13 packed streams (RNC1: Rob Northen RNC1 Compressor) found by scanning, 99% of the bytes
- mortal kombat ii (1994)(acclaim)[cr psg][a](disk 3 of 4)/Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 3 of 4).adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.32)
- mortal kombat ii (1994)(acclaim)[cr psg][a](disk 3 of 4)/Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 3 of 4).adf: blocks 22-1644 (bytes 11264-842240) hold a RNC1: Rob Northen RNC1 Compressor stream (830976 -> 32000 bytes)
- mortal kombat ii (1994)(acclaim)[cr psg][a](disk 4 of 4)/Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 4 of 4).adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.33)
- mortal kombat ii (1994)(acclaim)[cr psg][a](disk 4 of 4)/Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 4 of 4).adf: blocks 22-879 (bytes 11264-450560) hold a RNC1: Rob Northen RNC1 Compressor stream (439296 -> 204106 bytes)
- mortal kombat ii (1994)(acclaim)[cr psg][a](disk 4 of 4)/Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 4 of 4).adf: blocks 881-881 (bytes 451072-451584) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_audio_hardware` (Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 1 of 4).adf bytes 1024-6144 (code never loaded by a reference run)): Checks if a global variable is zero, and if so, writes two values to specific offsets of the Amiga custom chip registers, stores a pointer to itself in a memory location, performs a division to calculate a value, stores that byte in a memory location, and calls several other routines. Kind: disk hardware/sync check. Pass path: the branch where bVar1 is false (the code following the if(bVar1) block).
- `load_address_into_a5` (Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 1 of 4).adf bytes 1024-6144 (code never loaded by a reference run)): Loads a long value into a5 and returns. Kind: loader/address setup for protection check. Pass path: sub_0D6A (D70) -> bsr.b an address.

## Functions


### Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 1 of 4).adf:bootblock

- `configure_boot_memory_2`: The routine calls another routine via a global variable and, if the result is non-zero, writes several words through a pointer in a1, clears a flag in a1 and repeats until a byte in a1 is zero, then calls two routines via global variables and finally calls the routine obtained earlier, returning its result.
- `configure_boot_memory`: The routine calls another routine via a global variable and, if the result is non-zero, writes several words through a pointer in a1, clears a global variable, calls another routine via the global variable, calls a fixed address routine, and calls another routine via the global variable.

### logged disk load from Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 1 of 4).adf boot-block load

- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `apply_bitmask_to_word`: Calculates an index based on a register, fetches a word from a table using a pointer in a2, and returns the result of a bitwise AND between that word and a value derived from a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_from_stack`: Restores d0, a0, and a1 from the stack and returns.
- `process_input_and_audio_dma`: Updates display window and DMA settings, manipulates flags in a global variable and a byte pointed to by a pointer, and conditionally calls another routine while updating audio-related custom registers.
- `mask_multiple_bits`: Clears bit 2 of a global variable, clears bit 6 of another global variable, and tests a longword in a global variable before returning.
- `call_and_branch_conditionally`: Calls another routine and then either branches to a different location or returns.
- `set_global_flags_and_process_audio`: The routine stores two input words into a global variable, conditionally calls another routine and writes a byte derived from the global variable to memory, then sets bits in the global variable.
- `branch_to_address`: Performs a short branch to another routine.
- `branch_to_middle_of_routine`: Performs a short branch into the middle of another routine.
- `branch_to_subroutines`: Performs two relative branches to other locations.
- `update_audio_dma_structures`: Iterates through a table of pointers to update audio hardware registers and DMA controller settings by performing arithmetic on values stored in structures.
- `update_sprite_attribute_structure`: Updates a sprite or object structure by calculating differences between current and target values, then writes these values to a table and modifies hardware registers including the DMA controller and interrupt enable register.
- `set_global_and_or_byte`: Stores 6 into a global variable and returns the input byte ORed with 254.
- `update_audio_and_dma_structures`: Iterates through a table of pointers to update audio hardware registers and DMA controller settings by performing arithmetic on values stored within various data structures.
- `initialize_audio_hardware_2`: The routine writes zero to a memory location, sets a bit in a custom chip register, and performs several loops to clear memory areas and manipulate a table of pointers.
- `copy_table_and_set_flags`: Copies 64 bytes from a global variable to a table, then sets a global variable based on several sentinel checks and finally stores a constant to a global variable.
- `clear_d0_register`: Performs multiple bitwise OR operations with zero on the d0 register before returning.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `process_data_stream_2`: Performs bitwise OR operations on d4 and d0, calls two subroutines, and then iterates through a table of four bytes, calling a subroutine for each non-negative byte and finally calling another subroutine.
- `save_registers_and_call_subroutines`: Checks a byte at a PC-relative address, then calls two subroutines, saves registers to a stack, calls a third subroutine, restores registers, and saves several registers to a PC-relative location before returning.
- `update_audio_channel_parameters`: Iterates through a table of structures to update audio parameters, modifying a 16-bit value at offset 18 of a structure via a call to update_structured_pointers_2 and writing to the first audio channel register.
- `update_audio_value`: Subtracts a byte from an offset of a5 from zero, compares the result to a constant, and if the comparison fails, updates an offset of a5 and an offset of a4 before branching.
- `move_and_branch`: Copies a 16-bit word from an offset of a5 to an offset of a4 and branches to another routine.
- `update_structured_pointers_2`: Decrements a 16-bit value at offset 18 of a structure in a3 and returns; otherwise, it iterates through a table of four structures to update internal pointers and values by reading from a secondary table based on bitmasking and index offsets.
- `update_structured_pointers`: Checks a global variable, copies 64 bytes from a table to another location, and then conditionally updates several pointers and values based on a series of sentinel checks.
- `update_audio_dma_registers`: Checks a value in a global variable and updates several minimum byte values in RAM, then copies 64 bytes from a global variable to a table.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `reset_hardware_and_ram`: Negates a word at a pointer in a0, moves a byte from a pointer in a2 to a specific memory location, writes several words to a table, and updates several hardware registers including DMA and interrupt enable registers.
- `conditional_ram_update`: Checks a RAM flag and, if set, clears several RAM locations and stores an input word into one of them before returning.
- `process_data_stream`: Checks several global flags and, if conditions are met, fetches bytes from a table using an index and pointer, updates a counter, and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `build_bitmask_lookup_table`: Calls parse_bit_table to scan a table for matching conditions, calls generate_bitmask_table to calculate and store bitmask values and metadata into a table, and calls mask_and_conditional_call to perform masked comparisons and conditional execution.
- `parse_bit_table`: Scans a table of words for a matching condition, then calculates a bit mask based on byte values and register contents.
- `mask_and_conditional_call`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `perform_arithmetic_and_swaps`: Performs several arithmetic operations on d1, d6, and d7, including shifts and swaps, using values fetched from a table via a pointer in a3, and returns.
- `concatenate_memory_bytes`: The routine reads three consecutive bytes from a pointer and concatenates them into a single value.
- `generate_bitmask_table`: Iterates through a list to calculate and store bitmask values and related metadata into a table.

### Mortal Kombat II (1994)(Acclaim)[cr PSG][a](Disk 1 of 4).adf bytes 1024-6144 (code never loaded by a reference run)

- `unpack_and_execute_jump_table`: Writes a byte to a global variable, copies three longwords from a source to a destination in a loop, modifies custom chip registers, and then copies a block of 1151 longwords to a new memory location before performing an indirect jump using an offset calculated from the destination address.
- `initialize_system_and_load_data`: Clears a large block of memory starting at a global address, configures several custom chip registers via a structure in a6, and performs a series of conditional data loading and decompression operations before either entering an infinite loop or jumping to a location determined by a value in a table.
- `process_disk_data`: The routine calls another routine, enters a loop based on a value from the stack, and either calls another routine or writes bytes from a global variable to a memory location.
- `check_input_status`: The routine reads a global variable to determine the state of specific bits, returning a byte that combines the results of these checks.
- `wait_dma_and_halt`: Polls a status value until a condition is met, writes constants to a global variable, calls a routine, and then enters an infinite loop.
- `clear_color_registers`: The routine iterates through a range of memory addresses associated with the custom chip's color registers, writing zero to each location to clear the palette.
- `call_memory_allocator`: The routine calls the memory allocation library stub to request a block of memory and then returns the result provided by that stub.
- `mask_first_result_high_bits`: Calls one routine, then a second routine, and returns the first result masked with a fixed bit mask that clears the low address bits.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `decode_rle_data_block`: The routine iterates through a source buffer pointed to by a global variable and decodes run-length encoded data into a fixed destination area, repeating or copying bytes based on the sign of control values.
- `increment_and_set_interrupt`: The routine increments a global counter variable and sets another global variable to a specific constant value, likely related to interrupt handling.
- `infinite_loop_halt`: The routine executes an unconditional infinite loop, causing the processor to stall indefinitely.
- `generate_data_table`: Fills a memory area starting at a specific address with a sequence of calculated values using nested loops, then copies a block of data from a table to the end of the generated sequence.
- `copy_word_array`: The routine copies a sequence of 16-bit words from a source pointer to a destination pointer, decrementing a counter until the operation is complete.
- `calculate_offset_and_branch`: Loads a byte from a pointer, modifies it based on several comparisons, and either calls another routine or branches to a different location.
- `copy_structured_data`: If the input word is non-negative, the routine computes a source address from the input and a constant base, then copies six 40-byte blocks from that source to a destination pointer, advancing both pointers by 40 each iteration.
- `process_and_loop_data`: Performs bitwise shifts and rotations on a register, then enters a loop that calls another routine and decrements a counter.
- `call_other_routine_twice`: The routine calls another routine and then returns the address of that routine.
- `calculate_offset_and_branch_2`: The routine compares a word against several constants, calls another routine, and then uses a value from a table to call a routine at a calculated address in a loop.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `check_magic_number_call`: The routine compares an input word with its own address.
- `verify_and_load_data`: Checks a memory location for specific constant values, and if they match, enters a loop that increments a counter and calls another routine until a limit is reached.
- `load_address_and_constants`: Loads a base address into a register and sets two long integers to specific constant values.
- `call_reset_disk_controller`: The routine calls another routine to reset the disk controller.
- `initialize_audio_hardware`: Checks if a global variable is zero, and if so, writes two values to specific offsets of the Amiga custom chip registers, stores a pointer to itself in a memory location, performs a division to calculate a value, stores that byte in a memory location, and calls several other routines.
- `tiny_moveq`: Small routine of 1 instruction that loads 135 into d1.
- `write_hardware_byte`: Loads a hardware address into a5 and writes the byte from d1 to that address, flanked by two writes of the value 255 to the same location.
- `load_address_into_a5`: Loads a long value into a5 and returns.
- `wait_for_memory_flag_change`: The routine calls another routine, writes a masked byte to a memory location pointed to by a register and writes a constant byte to the same location, then checks a byte at an offset from that pointer.
- `clear_memory_bit`: Clears bit 1 of a byte at the address in a5 and branches to another routine.
- `wait_for_memory_change`: Sets bit 1 and then clears bit 0 of a byte at the address in a5, calls another routine, sets bit 0 of the byte at a5, and then loops 90 times calling that same routine.
- `compare_and_return_if_equal`: Compares a byte at an offset of 6 from a pointer in a6 with a byte stored on the stack, returning immediately if they are equal; otherwise, it restores d7 from the stack and returns.
- `process_bit_and_loop`: Checks a global variable, enters a loop that calls another routine, and performs bitwise operations on a value before entering a second loop that calls another routine.
- `process_input_and_xor_data`: The routine polls a hardware status byte in a loop and, when cleared, processes input data by calculating masks and XORing memory blocks, updating global variables and pointers in a loop.
- `tiny_move`: Small routine of 1 instruction that copies (a5)+ to d1.
- `calculate_masked_sum`: Computes a value by adding the result of two bitwise AND operations between d4 and d1, and d4 and a long word from a pointer in a2, then doubling the second operand's contribution.
- `xor_memory_block_and_mask`: The routine iterates through a block of memory, XORing each word into an accumulator, then combines the final accumulator with a global mask and an input register to form a 64-bit return value.
- `copy_audio_buffer_blocks`: Iterates through a table of pointers to copy blocks of data from source addresses to a destination buffer, resetting offsets after each block transfer.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d7.
- `decrement_input_value`: The routine takes an input value, subtracts 2 from it, and returns the result.
- `validate_header_and_decompress`: Validates a data header signature, allocates a buffer for decompression, reconstructs a Huffman tree structure from the input stream, and processes encoded data blocks to generate output bytes.
- `parse_bit_table_2`: Loops through a table of words by masking a word in d6 with a word from a0 and subtracting another word from a0 until a condition is met, then performs arithmetic on bytes at offsets 60, 61, and 62 of a0 to conditionally call another routine and set a bit in d2.
- `mask_and_conditional_call_2`: Masks a word in d0 with a word in d6 and, if the byte in d1 is greater than or equal to the byte in d7, calls another routine and shifts d6 right by the value in d1; otherwise it returns the masked word.
- `perform_arithmetic_on_registers`: Performs a series of arithmetic operations and bit shifts on registers d1, d6, and d7 using values fetched from a pointer in a3, then returns.
- `accumulate_32bit_from_bytes`: The routine reads four consecutive bytes from a pointer in a register and builds a 32-bit value in a register by concatenating each byte into the result.
- `build_huffman_tree_nodes`: Iterates over a buffer to match bytes against a running value, writing masks, shifted values, and node properties into a destination array to build a tree structure.
