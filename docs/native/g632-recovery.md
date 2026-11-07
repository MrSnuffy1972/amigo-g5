# Game 632 (Pac-Mania): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00632). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (927632 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 39 raw-word lines |
| Functions named and described | pass, 81 of 81 named (0 author symbols), 81 of 81 described (5 by rule, 49 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 990 of 990 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1584 of 1594 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12636 |
| code | 6510 |
| data | 9961 |
| padding | 477 |
| unused | 898048 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Pac-Mania.adf:bootblock | 1 | 159 | 6 |
| logged disk load from Pac-Mania.adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 786 | 47 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 454 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 181 | 8 |

## Cases handled

- Pac-Mania/Pac-Mania.adf: the boot block reads 512 bytes from disk offset 0 to an address (found statically); recovered as a unit
- runtime segment an address: 494 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 169 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 173 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Pac-Mania/Pac-Mania.adf: 1 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- `modify_input_and_set_registers` (runtime segment an address): Updates a bit in a global variable based on a masked bit from a pointer-referenced memory location and writes a constant value to a custom register. Kind: disk hardware/sync check. Pass path: bsr.w an address (if result is non-zero/not zero flag set).
- `start_disk_dma_and_wait` (runtime segment an address): Sets the disk sync word, the disk pointer and the ADKCON control word, then polls the disk block completion bit of the interrupt request register with a timeout before writing the disk length register. Kind: disk sync/track check. Pass path: the branch following the call where the code continues normal execution (the 'after' instructions indicate the routine is used to synchronize or wait for a disk operation to complete, and the loop structure suggests it is polling a status b.

## Functions


### Pac-Mania.adf:bootblock

- `issue_io_write_dma_control`: Boot block entry of twenty-nine instructions that issues an Exec DoIO request and writes the DMA control register, where a set top bit enables the DMA channels.
- `update_dma_control_state`: Checks a byte at a specific address to determine a state, scales that state by sixteen, and writes the result into four fixed memory cells.
- `check_byte_and_branch`: Calls another routine, checks if the byte at the address in a5 is positive, and otherwise returns.
- `calculate_string_length`: Calls a function via a pointer at an offset from a structure in a6, increments a counter in a loop until a null byte is found at an offset from a pointer in a5, and then calls another function via a pointer at a different offset from the structure in a6 before returning.
- `derive_state_from_counter`: Takes the top two bits of a 16-bit counter as a state value, scales that state by sixteen, and writes the result into four fixed memory cells.
- `copy_four_bytes_from_table`: Copies four consecutive bytes from a table entry, selected by a data register and scaled by four, into the buffer that the address register points to.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_entity_coordinate_tables`: Iterates through a linked list of structures to update coordinate values by adding an offset from a base pointer.
- `update_memory_constants_2`: Updates several memory locations and global variables using values from a1 and calls several other routines.
- `update_global_with_offset_address`: Calculates a new address by adding an offset to a pointer and stores the result in a global variable.
- `read_joystick_status`: Clears register d0 and then loads a longword from the joystick port into d0 before returning.
- `update_memory_constants`: Updates a series of memory locations relative to a pointer with a sequence of constant values.
- `dispatch_command_via_pointer`: Loads a pointer from a global variable into a7, calls another routine, and then jumps to a location stored in a pointer at the same global variable address.
- `clear_d0_and_update_pointer`: Compares a byte in a data register with a global variable, performs a series of bitwise OR operations to clear the data register, adds a word from an address register to another address register, and clears the data register again.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `update_mouse_and_memory`: Reads JOY0DAT, performs bitwise operations on memory locations calculated from a3 and a6, modifies a value at a pointer in a2, and increments a value at a pointer in a1.
- `update_conditional_bitmask`: Updates a 16-bit global value using a byte from a register, then checks bits 1 and 3 of a global byte to conditionally perform bitwise manipulations and bit-clearing on other global variables.
- `store_byte_and_branch`: Stores the byte from d0 into a memory location and then branches.
- `test_and_store_byte`: Tests a word at a global address; if it is non-zero, it stores the byte from d0 into a memory location and then branches, otherwise it branches to a different location.
- `process_conditional_data`: Tests a word at a global address, and if non-zero, performs a loop that writes d0 to a memory location, masks a bit at an offset from a1, writes a word to an offset from a0, restores registers, and returns from interrupt.
- `set_word_and_branch`: Stores the word one into a memory location and then branches.
- `clear_bit_and_compare`: Clears bit 7 of d0, compares d0 with a byte at a global address, and if they are not equal, stores the value ff into that global address and branches to another routine, otherwise it branches to a different location.
- `compare_and_set_byte_2`: Compares a byte in d0 with a byte at a global address; if they are not equal, it stores the value ff into that global address and branches to another routine, otherwise it branches to a different location.
- `compare_and_set_byte`: Compares a byte in a global variable with the contents of d0 and, if they are not equal, stores the value ff into that global variable and branches to another location.
- `set_byte_and_branch`: Stores the byte ff into a memory location and then branches.
- `mask_interrupt_requests`: Saves the current stack pointer, reads the interrupt request register, masks bits 0 through 8, writes the result to the interrupt mask registers, restores the stack pointer, and returns from interrupt.
- `initialize_system_tables`: Clears a block of memory, updates several memory locations with constant values, performs a bitwise rotation on a value, calls several routines, and writes specific 16-bit values to offsets of a structure in a0.
- `set_blt_registers`: writes specific constant values to several memory-mapped or global registers, including certain bitplane or blt-related registers.
- `tiny_andi_ori`: Small routine of 2 instructions that masks d0 with 0, then merges 0 into d0.
- `set_and_test_byte`: Sets bit 5 of register d0 and tests a word at a global address, returning if the word is non-zero.
- `update_global_from_table`: Tests two bytes at a global address, decrements a word at a global address if the first byte is non-zero, and if the second byte is zero, loads a byte from a table into a global variable using a value from that global variable as an index.
- `save_registers_and_rotate_d2`: Saves d0 through d7 and a0 through a3 to the stack, clears d1, moves the byte from d4 to d1, clears d4, and performs a two-bit left rotation on the word in d2.
- `return_stack_value`: This routine calls a subroutine and returns the value that was present on the stack at the start of the routine.
- `apply_mask_to_memory`: Iterates through a sequence of instructions, applying bitwise masks or negations to certain bytes and storing the results into a memory table at large offsets.
- `tiny_ori`: Small routine of 1 instruction that merges 100 into d2.
- `calculate_and_store_table_values`: Calculates a value based on a 16-bit word and a byte from a structure in a6, then writes several 16-bit results to specific offsets within a table calculated from a byte in the structure.
- `update_table_constants`: Performs a loop five times that reads a byte from a table and updates a structure in a6, calculating two 16-bit values based on the table data and a global variable, or setting specific constant values if certain conditions are not met.
- `write_table_constants`: Writes two 16-bit constants to offsets 4 and 6 of a structure in a5 and branches to an earlier instruction.
- `store_count_results`: Calls another routine four times and stores each returned 16-bit value into consecutive 16-bit locations pointed to by a pointer in a1.
- `count_matching_bytes`: Iterates through a buffer pointed to by a0 for 383 iterations, incrementing d0 for every byte that equals 1, and returns the count.
- `update_memory_pointers`: Calculates an offset based on a bitwise operation on a value in d0, then if a longword at that offset is non-zero, updates several global variables and potentially sets them to new values based on comparisons of words at offsets 408 and 40a.
- `store_four_longwords`: Compares a word at offset 2 of a0 to 272; if it is less, it adds 63424 to d2, then stores d0, d1, d2, and d3 into four consecutive longword locations starting at a global address.
- `clear_color_register`: Sets a range of memory locations, starting from a hardware register, to zero.
- `call_subroutines_sequentially`: Saves registers to the stack, calls one routine, restores the registers, calls another routine, and then returns.
- `process_input_loop`: Stores three values from registers into global variables, then enters a loop that increments a global variable and checks if it equals eleven, while also performing bitwise operations on a global variable and potentially calling another routine.
- `modify_input_and_set_registers`: Updates a bit in a global variable based on a masked bit from a pointer-referenced memory location and writes a constant value to a custom register.
- `process_data_stream`: Searches a memory range for a specific word, calls another routine, and then processes a sequence of bytes by performing bitwise rotations and storing them into a table.
- `combine_masked_pointer_bits`: Returns the data register ANDed with the longword at the second pointer, ORed with the data register ANDed with the first pointer's longword shifted left by one bit.
- `modify_cia_register_and_call`: Reads a byte from a CIA register, clears bit 0, writes it back, increments the value, writes it again, and calls another routine.
- `write_word_bytes_then_poll`: Writes the low and high bytes of the data register word to two consecutive memory locations, then polls a status byte until its low bit is set.
- `mask_byte_and_store_value`: Keeps the top two bits of one memory byte, sets its bit 3, and writes the value 127 to a second memory byte.
- `decrement_and_branch_loop`: Decrements a register while checking a bit in a memory location, then performs a secondary decrementing loop.
- `adjust_global_range`: This routine compares two global variables and, depending on which is larger, modifies a memory location and then enters a loop that increments or decrements a global variable while calling another routine.
- `decrement_and_mask_register`: Decrements a register until a bit in a global variable is cleared, then masks a bit and writes a value to a hardware register or memory, followed by a recursive call and a fixed-count loop of another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fetch_sprite_attribute_data`: Checks if a global variable equals 19; if so, it clears the variable, calls another routine, and enters an infinite loop.
- `set_blt_registers_2`: Sets several hardware registers or memory locations to specific values based on a global variable comparison.
- `load_address_and_branch`: Loads a pointer from a global address into a4 and branches to another routine.
- `load_and_branch`: Loads the word 3610 into d4 and branches to another routine.
- `fill_memory_with_routine`: Executes a routine repeatedly while decrementing a value, effectively performing a loop-based memory write.
- `configure_memory_block`: Writes a sequence of constant values and calculated pointer or register values to a memory area using a pointer.
- `setup_blt_registers`: Sets certain hardware registers or custom memory-mapped registers to specific constant or calculated values.
- `save_registers_and_branch`: Saves registers d0-d7 and a0-a6 to the stack, then performs a series of moves and branches to other locations.
- `load_pointer_and_branch`: Loads a pointer from a global address into a4, loads the word 5147 into d4, and branches to another routine.
- `format_numeric_string`: Calculates digits and stores them as characters into a series of memory locations.
- `load_word_from_table`: Loads a word from a table using an offset calculated from a0 and d3 into d0.
- `set_blitter_parameters`: Sets several values in memory, including bitwise modifications to a byte at an offset of a0, and updates multiple hardware-related registers.
- `initialize_and_process_game_loop`: Initializes several global variables and memory locations, then enters a loop that repeatedly calls various subroutines, updates a counter, and performs conditional table lookups to write values into memory.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `enter_infinite_loop`: The routine enters an infinite loop that does not exit.
- `write_constant_sequence`: Writes a sequence of constant values to a global variable.
- `load_jump_parameters`: Loads a pointer into a0, two 16-bit constants into d1 and d2, and jumps to a long address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_store_byte`: Compares a byte from a memory location to one, then either stores a longword at a specific address or stores the byte into another address before branching.
- `copy_data_block`: Copies 512 words from a source table to a destination address using a loop, then performs a bitwise operation on a global variable and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_execute_sequence`: Copies a block of memory from one location to another using a loop, sets a global variable, modifies a byte in memory, and then executes a series of loops that call other routines based on various counter and bit conditions.
- `increment_and_check_counter`: Calls another routine, increments a byte at offset 0 of a6, and if it reaches eleven, increments a word at offset 6 of a6 and checks bit 0 of d0 to decide whether to return or branch.
- `start_disk_dma_and_wait`: Sets the disk sync word, the disk pointer and the ADKCON control word, then polls the disk block completion bit of the interrupt request register with a timeout before writing the disk length register.
- `scan_sync_word_read_block`: Searches memory from a fixed start address for the disk sync word, skipping repeated sync words, and checks an unnamed routine's result until it matches.
- `combine_masked_pointer_bits_2`: Returns the data register ANDed with the longword at the second pointer, ORed with the data register ANDed with the first pointer's longword shifted left by one bit.
- `toggle_bit_and_call`: Clears bit 0 of a byte at an offset from a pointer in a1, increments that byte, calls another routine, and loads a 16-bit constant into d0.
- `write_word_bytes_then_poll_2`: Writes the low and high bytes of the data register word to two consecutive memory locations, then polls a status byte until its low bit is set.
- `modify_structure_bits`: Clears bits 6 and 7 of a byte at offset 3584 of a structure in a1 and sets bit 3, then writes 127 to offset 3328 of the same structure.
