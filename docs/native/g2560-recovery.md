# Game 2560 (odyssey (1995)(audiogenic)(disk 1 of 2)[cr][t +2 ins]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02560). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (914192 top-level bytes in 58 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 77 of 77 named (0 author symbols), 77 of 77 described (8 by rule, 69 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1706 of 1706 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2462 of 2462 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 587734 |
| code | 9052 |
| data | 222828 |
| padding | 13170 |
| unused | 81408 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Odyssey (1995)(Audiogenic)(Disk 1 of 2)[cr][t +2 INS].adf:Ins.exe | 2 | 196 | 7 |
| Odyssey (1995)(Audiogenic)(Disk 1 of 2)[cr][t +2 INS].adf:cracktro.exe | 2 | 28 | 2 |
| Odyssey (1995)(Audiogenic)(Disk 1 of 2)[cr][t +2 INS].adf:Odyssey | 2 | 28 | 2 |
| Odyssey (1995)(Audiogenic)(Disk 1 of 2)[cr][t +2 INS].adf:CJM1004 | 2 | 28 | 2 |
| Odyssey (1995)(Audiogenic)(Disk 1 of 2)[cr][t +2 INS].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1626 | 41 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 546 | 22 |

## Cases handled

- runtime segment an address: 980 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 344 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- odyssey (1995)(audiogenic)(disk 1 of 2)[cr][t +2 ins]/Odyssey (1995)(Audiogenic)(Disk 1 of 2)[cr][t +2 INS].adf:Ins.exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- odyssey (1995)(audiogenic)(disk 1 of 2)[cr][t +2 ins]/Odyssey (1995)(Audiogenic)(Disk 1 of 2)[cr][t +2 INS].adf:cracktro.exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- odyssey (1995)(audiogenic)(disk 1 of 2)[cr][t +2 ins]/Odyssey (1995)(Audiogenic)(Disk 1 of 2)[cr][t +2 INS].adf:Odyssey: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- odyssey (1995)(audiogenic)(disk 1 of 2)[cr][t +2 ins]/Odyssey (1995)(Audiogenic)(Disk 1 of 2)[cr][t +2 INS].adf:CJM1004: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Odyssey (1995)(Audiogenic)(Disk 1 of 2)[cr][t +2 INS].adf:cracktro.exe

## Copy protection found

- none found

## Functions


### Odyssey (1995)(Audiogenic)(Disk 1 of 2)[cr][t +2 INS].adf:Ins.exe

- `initialize_system_structure`: Calls a routine via a pointer in a table, then if the return value is non-zero, performs a series of memory operations involving a library call and updates a structure at a calculated offset.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_compressed_data_stream`: Reads a sequence of values from a source pointer, performs bitwise operations and arithmetic to determine a length, and writes the resulting bytes to a destination buffer.
- `calculate_bit_shifted_value`: Performs arithmetic on registers and calls another routine, then uses a loop to manipulate a value from a pointer in a1 and a global variable.
- `unpack_and_process_data_stream`: Copies a block of data from a source pointer to a destination pointer, then iterates through the data to perform memory allocations, bitwise shifts, and arithmetic operations on values stored in a destination buffer.
- `process_data_buffer`: Reads bytes from a pointer in a2 to calculate an offset, adds a value to a memory location, and calls a library routine if a condition is met.
- `spin_countdown_delay`: Runs an empty countdown loop from a value passed in a register, then returns.

### Odyssey (1995)(Audiogenic)(Disk 1 of 2)[cr][t +2 INS].adf:cracktro.exe

- `call_and_update_table`: Calls a routine via a pointer derived from a local address, then if the return value is non-zero, calls a library function and updates a table entry by shifting a value and storing it at a calculated offset.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### Odyssey (1995)(Audiogenic)(Disk 1 of 2)[cr][t +2 INS].adf:Odyssey

- `execute_and_update_pointer`: Calls a function via a pointer derived from a local address, then if the return value is non-zero, calls a library function and updates a global pointer with a value from a stack-based source and a modified version of a value from a structure.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### Odyssey (1995)(Audiogenic)(Disk 1 of 2)[cr][t +2 INS].adf:CJM1004

- `call_and_update_table_2`: Calls a function via a pointer derived from a local address, then if the return value is non-zero, retrieves a value from a stack-based pointer and a library function, and finally updates a table entry by calculating a new address and storing the retrieved value at an offset within that table.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.

### Odyssey (1995)(Audiogenic)(Disk 1 of 2)[cr][t +2 INS].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_and_jump`: Calls another routine and then performs an indirect jump to an address stored in a global variable.
- `allocate_and_copy_memory`: Clears a global variable, calls another routine to allocate memory, and if successful, copies 184 bytes from a routine to the allocated memory before performing an indirect jump to the allocated address; otherwise, it sets a global variable to 65535 and checks a bit in a CIA register.
- `update_pointer_table`: Copies a longword from a global variable to a destination address and then updates that same global variable with a new address.
- `copy_pattern_bytes`: Compares a byte at a negative offset from a pointer against a sequence of four specific values, and if they match, copies six bytes of alternating values from two different source locations to two different destinations based on two conditional checks.
- `initialize_hardware_and_copy_data`: Calls several library routines to manage system state, modifies hardware registers at a fixed address, and copies ten words from a source table to a destination table using a loop.
- `check_flag_and_return_exception`: Tests bit 0 of a 16-bit word located at offset 296 from a6; if the bit is set, it returns from an exception, otherwise it returns.
- `copy_and_configure_hardware`: Copies ten words from a source to a destination, modifies several custom chip registers, and calls three library routines.
- `acknowledge_interrupt_and_return`: Saves registers to the stack, reads a byte from a hardware register, writes 8192 to an offset of 154 from the custom register, calls another routine, and returns from an interrupt.
- `expand_audio_sample_data`: Processes a list of audio sample descriptors by performing bitwise operations and arithmetic to expand or transform sample data into a target buffer.
- `reset_hardware_and_registers`: Clears four 16-bit words in a structure, modifies a bit in a CIA register, and writes several bytes to specific offsets in the hardware registers.
- `increment_and_dispatch_data_command`: Increments a counter in a global variable and, if it reaches a limit, iterates through a list of structures to perform various bitwise manipulations, pointer updates, and arithmetic operations based on a command identifier, then jumps to a command handler via a jump table.
- `process_variable_length_data_stream`: Iterates through a data buffer using a switch-based decoder that performs arithmetic operations, clamping, and pointer updates on structure members.
- `clamp_and_store_value`: Reads a byte from an offset of 2 from a structure in a5, subtracts it from a word at an offset of a from the same structure, and if the result is greater than 113, stores 113 in both an offset of a from the structure and an offset of 6 from a structure in a4; otherwise it stores the result in the offset of a from the structure and branches to another location.
- `branch_to_offset`: Moves a word from an offset of a5 to an offset of a4, then branches to a relative address.
- `decode_and_update_data_structures`: Iterates through a list of structures, decoding variable-length data from a buffer based on bitmask checks and updating pointers and counters within each structure.
- `transfer_byte_and_branch`: Copies a byte from offset 2 of a structure in a5 to an offset in a5, then branches to another routine.
- `update_index_and_bit_flip`: Updates a byte at an offset in a structure using a value from a global table, and if a signed check passes, updates a pointer in the structure and flips the bits of the byte at that pointer's location.
- `configure_dma_and_interrupts`: Writes values to DMA and interrupt control registers, then stores a value from a table into a location specified by another table entry.
- `update_global_data_tables`: Sets a global variable to 8192 and copies four pairs of values from four different source locations into a series of global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `modify_hardware_and_call_subroutines`: Sets and clears bit 6 of a global variable, checks if a byte from a keyboard register equals 65 or 117 to update two global words, calls four different routines, writes 32 to a hardware register, and returns from an interrupt.
- `populate_data_tables`: Uses two loops to write longwords and words to a destination table starting at a fixed address, including a call to another routine and specific constant values at the end.
- `update_hardware_registers`: Sets d0 to 17, stores 51 at offset 69 of the structure in a0, stores 117 at offset 1255 of the structure in a2, stores 85 in d4, stores 119 at offset 1985 of the structure in a6, stores d2 at offset 1757 of the structure in a4, and clears bit 0 of the byte at offset 1291 of the structure in a2.
- `update_sprite_data_from_tables`: Iterates through a nested structure of control bytes to copy blocks of data from various tables into a destination memory area.
- `copy_conditional_values`: Loops through a table starting at a pointer in a0, and if the value at the current pointer is 0 or 1 and does not match the value at offset 66 of the current structure, it copies the value at offset 66 to the location pointed to by a pointer in a1.
- `copy_block_data`: Calculates a destination address using a word from a pointer, then uses nested loops to copy bytes from one memory location to another.
- `clear_d0_register`: Performs an OR operation on d0 with zero.
- `manage_input_state`: Updates a global variable using a value from a structure, performs a loop to clamp a value within a range based on a table, and conditionally increments counters or calls another routine based on the status of CIAA and POTGOR registers.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `clears_data_register_zero_2`: Performs a logical OR operation on register d0 with zero.
- `zeroes_data_byte`: Performs a logical OR operation on a byte in register d0 with zero.
- `update_state_and_copy_bytes`: Checks a value at an offset from a pointer, and if conditions are met, performs a nested loop to copy bytes from a table to a destination based on values in a1 and a2.
- `manage_conditional_copy`: Checks a value at an offset from a pointer and, if it is zero, performs a nested loop to copy bytes from a table to a destination using values from a1 and a2.
- `fill_data_buffer`: Runs a nested loop that writes a constant value and copies data from a source table into a destination table, while updating a global pointer that wraps around a specific address range.
- `process_division_table`: Performs a loop that divides a word from a0 by 100, then uses the quotient to fetch a byte from a table in a1 and stores it in a2, updating a counter in a3.
- `execute_conditional_logic`: Checks a global word and a second global word, then calls another routine and updates the second global word with either one or two depending on the state of a third global variable.
- `clears_data_d0_register`: Performs a logical OR operation on the d0 register with zero.
- `update_scaled_value`: Increments or decrements a global word and calculates a new value using multiplication and division, then stores the result in a memory location.
- `apply_relocation_codes`: Iterates through a structure using nested loops to read bytes, subtract 32, and use the result as an offset to copy data from one memory area to another, while incrementing a global counter that wraps at 3.
- `update_sprite_data_and_configure_dma`: Copies a series of bytes from a source table to a destination table using a loop, then updates a structure pointed to by a6 with various values including coordinates and control words, while checking the DMA controller status register.
- `mask_and_modify_data`: Waits for bit 6 of a register to clear, then performs two loops that bitwise AND values from a table into a structure at an offset from a global variable, and finally performs bitwise OR operations on specific offsets within that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_pointer`: Stores two longwords into a memory location using an offset from the program counter.
- `copy_pattern_data`: Compares a sequence of bytes at an offset from a pointer against a specific pattern, and if they match, writes two different sets of byte patterns to two different memory locations before jumping to a fixed address.
- `initialize_subsystem_pointers`: Calls a library function to obtain a pointer, configures several global pointers and constants, and enters a loop that increments a pointer through a table until a zero-value is encountered.
- `update_global_pointer_2`: Decrements a global word and resets it to sixteen if it reaches zero or a borrow occurs, then updates a global pointer by adding fifty or subtracting sixteen thousand seven hundred and eight based on its current value.
- `poll_button_transition`: Loops while bit 6 of the CIAA port A peripheral register is clear, calls another routine, and then loops while bit 6 is set before returning.
- `execute_sequential_subroutines`: Calls six different routines in sequence and then returns.
- `wait_for_global_pattern`: Polls a register until the value at an offset of 8 bits from the register's content matches a specific constant.
- `tiny_ori_2`: Small routine of 1 instruction that merges 64 into d0.
- `initialize_global_values`: Stores or calculates several values and writes them into a series of global variables.
- `select_global_value`: Compares two global variables and updates one with the value of another based on that comparison.
- `update_blt_parameters`: Calculates new blitter dimensions and source/destination addresses based on input values and updates several hardware registers and global memory locations.
- `calculate_register_params`: Calculates several values based on a global variable and a routine address, then stores them into the hardware registers at offsets 64 to 102.
- `set_blt_registers`: Waits for a specific bit in a certain memory location to clear, then writes several constant or fixed values into a set of bitplane and DMA registers.
- `calculate_blt_parameters`: Waits for a bit in a memory location to clear, then calculates a value using a table and a register, and stores the result and other values into a set of registers.
- `sync_parameter_tables`: Updates a series of global variables with new values and clamps a value from a pointer-based table between zero and two hundred while iterating through a list.
- `calculate_register_params_alt`: Calculates several values based on a global variable and a routine address, then stores them into the hardware registers at offsets 64 to 102 using a different constant for the value at offset 64.
- `update_blt_and_data`: Checks a value, potentially increments a global variable, and if a condition is met, waits for a bit or register to clear before writing a series of values to memory and registers.
- `set_blt_bounds`: Waits for a bit in a memory location to clear, then writes a sum of two certain values and several constants into a set of registers.
- `process_instruction_stream`: Iterates through a stream of bytes, updating a pointer and certain memory locations based on the value of the byte read, or resetting a pointer to a fixed address.
- `apply_bit_mask`: Performs a bitwise OR on a word at the address held in a register with the constant 616.
- `tiny_ori_3`: Small routine of 1 instruction that merges 200 into (a3)+.
- `do_nothing`: Performs a bitwise OR on a byte in a register with zero.
