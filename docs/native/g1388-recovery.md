# Game 1388 (bill's tomato game (1992)(psygnosis)[cr csl][t +2 csl]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01388). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (910160 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 128 of 128 named (0 author symbols), 128 of 128 described (4 by rule, 119 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1002 of 1003 (99.90%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1930 of 1930 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 7328 |
| code | 5642 |
| data | 897128 |
| padding | 62 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Bill's Tomato Game (1992)(Psygnosis)[cr CSL][t +2 CSL].adf:bootblock | 1 | 63 | 5 |
| logged disk load from Bill's Tomato Game (1992)(Psygnosis)[cr CSL][t +2 CSL].adf boot-block load | 1 | 1482 | 103 |
| logged disk load from Bill's Tomato Game (1992)(Psygnosis)[cr CSL][t +2 CSL].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 88 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 281 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |

## Cases handled

- bill's tomato game (1992)(psygnosis)[cr csl][t +2 csl]/Bill's Tomato Game (1992)(Psygnosis)[cr CSL][t +2 CSL].adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 6 of the 773 PCs the canonical run logged (3702 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- bill's tomato game (1992)(psygnosis)[cr csl][t +2 csl]/Bill's Tomato Game (1992)(Psygnosis)[cr CSL][t +2 CSL].adf: the boot block reads 5120 bytes from disk offset 512 to an address (found statically); recovered as a unit
- bill's tomato game (1992)(psygnosis)[cr csl][t +2 csl]/Bill's Tomato Game (1992)(Psygnosis)[cr CSL][t +2 CSL].adf: the boot block reads 1024 bytes from disk offset 900096 to an address (found statically); recovered as a unit
- prefix run f1000 ignored: it logged 2935 PCs the canonical run never executed (not a prefix of it)
- prefix run f2000 ignored: it logged 2935 PCs the canonical run never executed (not a prefix of it)
- runtime segment an address (play-rs-m1500): 47 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 190 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- bill's tomato game (1992)(psygnosis)[cr csl][t +2 csl]/Bill's Tomato Game (1992)(Psygnosis)[cr CSL][t +2 CSL].adf: 2 logged loads cover 6144 bytes of the image
- 1 packed track region(s), 894464 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `generate_inverted_bitmask` (Bill's Tomato Game (1992)(Psygnosis)[cr CSL][t +2 CSL].adf boot-block load +an address to an address (5120 bytes)): Iteratively builds a bitmask based on a bit in a global variable and returns its bitwise complement. Kind: checksum/data integrity trap. Pass path: cmpi.w #an address, d0; bne.b an address.
- `update_state_variables` (Bill's Tomato Game (1992)(Psygnosis)[cr CSL][t +2 CSL].adf boot-block load +an address to an address (5120 bytes)): Updates several global variables based on a comparison between an input value and a stored value. Kind: state-based integrity check. Pass path: in_D0w == sRam00070df6.
- `modify_global_bits` (Bill's Tomato Game (1992)(Psygnosis)[cr CSL][t +2 CSL].adf boot-block load +an address to an address (5120 bytes)): Clears a bit in a global variable, calls another routine, and then clears a different bit based on a returned value. Kind: bitmask-based integrity/hardware check. Pass path: the branch where DAT_00bfe001 & 4 != 0.
- `process_complex_logic` (Bill's Tomato Game (1992)(Psygnosis)[cr CSL][t +2 CSL].adf boot-block load +an address to an address (5120 bytes)): Sets a global variable to a specific value and performs a series of conditional checks and memory operations involving hardware registers and pointer-based loops. Kind: disk/track check. Pass path: if (-1 < unaff_D7w).
- `enter_infinite_loop` (Bill's Tomato Game (1992)(Psygnosis)[cr CSL][t +2 CSL].adf boot-block load +an address to an address (5120 bytes)): The routine enters an infinite loop that prevents further execution. Kind: sabotage path / infinite loop trap. Pass path: the code following the call sites (which typically involves loop decrementing or conditional branching based on hardware status).

## Functions


### Bill's Tomato Game (1992)(Psygnosis)[cr CSL][t +2 CSL].adf:bootblock

- `initialize_boot_sequence`: Performs I/O operations and memory type checks before setting specific hardware or system values.
- `initialize_hardware_registers`: Disables interrupts, sets up stack and custom register pointers, and configures various hardware control registers.
- `call_start_routine`: This routine calls another routine and then returns.
- `perform_io_operation`: Sets up a buffer and calls an I/O routine via a pointer stored in a global variable.
- `reset_global_flag`: Checks a bit in a global variable and, if set, recursively calls itself before clearing that variable.

### logged disk load from Bill's Tomato Game (1992)(Psygnosis)[cr CSL][t +2 CSL].adf boot-block load

- `clear_memory_and_jump`: Clears 439 consecutive longwords starting at address 256, then writes the ASCII string 'trim1' followed by 'intr' into the an address-byte region and finally jumps to the code at offset 2048.
- `branch_to_address`: Performs a short relative branch to a specific memory address.
- `perform_arithmetic_copy`: Performs arithmetic operations on a register and copies a byte from a memory location through a pointer.
- `branch_to_routine`: Loads a byte from a pointer, adds it to a register using the carry flag, and branches to a location stored in another register.
- `increment_byte_with_carry`: Loads a byte from a memory location pointed to by a register into another register, adds that byte to itself using the carry flag, and performs a relative branch.
- `update_register_with_byte`: Loads a byte from a pointer into a register, adds it to itself with extend, and branches to another location.
- `increment_byte_with_carry_2`: Loads a byte from a memory location pointed to by a register and adds it to itself using the carry flag.
- `increment_byte_with_carry_3`: Fetches a byte from a memory location pointed to by a register and adds it to itself using the carry flag.
- `increment_byte_with_carry_4`: Loads a byte from a memory location pointed to by a register and adds it to itself using the carry flag.
- `process_byte_with_carry`: Loads a byte from a memory location pointed to by a register, adds it to itself with the carry flag, and branches to another location.
- `accumulate_byte_carry`: Loads a byte from a memory location pointed to by a register and adds it to itself while propagating the carry bit.
- `copy_bytes_with_loop`: Performs a series of byte transfers from one memory location to another using two nested loops controlled by different registers.
- `process_multi_byte_arithmetic`: Performs complex arithmetic operations on bytes and words while propagating carry bits through multiple stages.
- `copy_memory_block`: Copies bytes from one memory location to another using a loop controlled by a counter and then branches.
- `propagate_carry_and_copy_bytes`: Performs multi-stage arithmetic to propagate carry bits through a sequence of bytes while simultaneously copying data from a source to a destination.
- `update_register_with_carry`: Performs arithmetic operations on registers using the carry flag and executes conditional branches based on the results.
- `accumulate_byte_carry_2`: Fetches a byte from a memory location pointed to by a register and adds it to itself while incorporating the processor carry flag.
- `accumulate_byte_carry_3`: Fetches a byte from a memory location pointed to by a register and adds it to itself while incorporating the carry flag.
- `process_byte_with_carry_2`: Fetches a byte from a memory location pointed to by a register and adds it to itself using the carry flag.
- `add_byte_with_carry_and_branch`: Loads a byte from a memory location pointed to by a register, adds that byte to itself using the carry flag, and performs a relative branch.
- `update_byte_with_carry_alt`: Loads a byte from a pointer into a register, adds it to itself using the carry flag, and branches to a different location.
- `process_byte_loop`: Performs arithmetic on a register and uses conditional branches to either loop or return from the routine.
- `branch_to_offset_2`: Performs a short relative branch to a specific instruction offset.
- `update_global_values`: Assigns values to several global variables based on a comparison of a register and then calls another routine.
- `branch_to_offset_3`: Performs a short relative branch to a location within another routine.
- `update_table_data`: Performs complex logic involving searching, clearing, and updating entries in a table.
- `copy_partial_memory_block`: Checks a value against a threshold and, if conditions are met, copies a specific number of bytes from a source pointer to a destination pointer.
- `execute_complex_table_logic`: Manages multiple global variables and executes a sequence of routines that involve searching, clearing, and updating entries in a table.
- `load_constant_and_branch`: Loads a constant into a register and performs a relative branch.
- `compare_and_branch`: Compares a register to a constant and either branches or loads a different constant into that register.
- `conditionally_call_routine`: Loads a constant into a register, loads a pointer, and calls a routine if a specific word value is zero.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `copy_offset_memory_block`: Calculates a destination address based on an offset and copies a block of memory from a source pointer if certain conditions are satisfied.
- `store_registers_to_memory`: Saves the current stack pointer, loads a pointer, and stores the contents of two registers into memory locations based on a conditional check.
- `copy_memory_and_recurse`: Copies two 32-bit values from one memory location to another and then enters a recursive sequence of calls based on the value of a word in a register.
- `compare_and_conditionally_load_d7`: Compares a word value in a register to a constant and either branches to another routine or loads a new value into that register before branching to another routine.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 252 into d7, then returns.
- `process_table_entries`: Iterates through a structure to compare values and updates a table with new data based on the results of another routine.
- `setup_pointers_and_branch`: Saves registers to the stack, loads two pointers from a table into registers, and performs a relative branch.
- `set_global_flag`: Calls another routine and, depending on the result, writes a specific constant to a memory location pointed to by a register.
- `initialize_data_array`: Stores a value at a pointer location and then fills a subsequent sequence of memory with zeros.
- `find_zero_entry`: Iterates through a table of ten-byte structures to find the index of the first entry containing a zero.
- `clear_table_entry`: Clears the contents of a ten-byte structure within a table based on an index.
- `find_matching_entry`: Searches a table of ten-byte structures for an entry matching a specific value and returns its index.
- `update_table_entry`: Writes multiple values into a ten-byte structure within a table at a specified index.
- `get_table_value`: Retrieves a value from a ten-byte structure within a table using a provided index.
- `wrap_other_routine`: Calls another routine and returns its concatenated results.
- `fill_memory_with_zeros`: Fills a block of memory with zeros using a pointer.
- `execute_loop_with_bit_clear`: Repeats a routine in a loop until a specific value is returned, while calling another routine to clear a bit in a table.
- `calculate_nested_loop_bounds`: Calculates a value based on a division and remainder operation, then executes nested loops that call other routines and manipulate a counter.
- `compare_and_branch_value`: Compares a word value against a constant and branches to another routine if certain conditions are met.
- `loop_and_set_bit`: Contains a loop that calls another routine to set a bit in a table and eventually returns a value.
- `return_stack_value`: Returns a value composed of a constant and the contents of the stack.
- `test_bit_in_table`: Pushes registers to the stack, performs bitwise operations, and tests a bit within a table at an offset determined by a register.
- `return_zero`: This routine is empty and simply returns a zero value to the caller.
- `clear_bit_in_table`: Clears a specific bit within a byte located in a table using a pointer and an index.
- `set_bit_in_table`: Sets a specific bit within a byte located in a table using a pointer and an index.
- `call_offset_calculation`: Calls a routine that performs a calculation to update a global variable.
- `branch_to_offset`: Performs a relative branch to a specific instruction offset.
- `branch_to_address_2`: Performs a relative branch to another location.
- `calculate_offset_value`: Performs conditional checks and updates a global variable using a calculation involving a subtraction and multiplication.
- `update_hardware_registers_and_poll_bit`: Sets a global variable to a specific value and writes three values to memory locations.
- `branch_to_subroutine`: Performs a short relative branch to an offset within another routine.
- `check_input_and_update_status`: Reads the status of the CIAA and CIAB peripheral registers and passes the result to another routine that updates a value at a memory offset.
- `update_status_on_result`: Calls another routine and updates a value at a memory offset based on the returned result.
- `generate_inverted_bitmask`: Iteratively builds a bitmask based on a bit in a global variable and returns its bitwise complement.
- `update_state_variables`: Updates several global variables based on a comparison between an input value and a stored value.
- `poll_hardware_bit_loop`: Contains a loop that checks a specific bit in a hardware register and decrements a register until the bit is set or the counter reaches zero.
- `store_word_to_global`: Stores a 16-bit value into a global variable after performing bitwise operations.
- `modify_global_bits`: Clears a bit in a global variable, calls another routine, and then clears a different bit based on a returned value.
- `modify_bitmask_variable`: Sets a high bit in a global variable and then manipulates specific bits within that variable using a value from a table and another routine's result.
- `calculate_scaled_values`: Saves registers to the stack and performs multiplication on two input values using a constant factor before branching.
- `process_complex_logic`: Sets a global variable to a specific value and performs a series of conditional checks and memory operations involving hardware registers and pointer-based loops.
- `modify_memory_via_subroutine`: Calls another routine and then performs a series of bitwise OR and AND operations on a value at a pointer in a register.
- `return_void`: The routine is empty and performs no operations before returning.
- `modify_bit_and_poll_memory`: Clears a bit in a memory location using a value from another routine, then enters a loop that polls a bit in a memory offset.
- `execute_conditional_subroutine`: Sets a register to a specific value, calls another routine, and then performs a conditional check on a bit within a memory location offset by a register.
- `clear_port_bit`: Clears a specific bit in a memory location pointed to by a register and then performs a relative branch.
- `pulse_port_bit_with_delay`: Sets bit 1, clears bit 0, and then sets bit 0 in a byte through a pointer, followed by a loop that runs ninety times.
- `enter_infinite_loop`: The routine enters an infinite loop that prevents further execution.
- `process_bit_logic_and_wait`: Performs bitwise shifts and logical operations on a value, enters a loop to compare results, and then waits for a bit in a global variable to clear.
- `process_data_table`: Iterates through a table to find a specific word, then copies a sequence of data from a source to a destination using a loop.
- `tiny_move`: Small routine of 1 instruction that copies (a6)+ to d1.
- `merge_masked_bit_values`: Combines a register value with a memory word under a mask: it takes the register bits selected by the mask, ORs in the masked memory bits shifted left by one, and returns the result.
- `calculate_xor_checksum`: Iterates through a memory block to calculate the cumulative bitwise XOR of the values and returns the result masked with another value.
- `sort_and_fill_memory`: Performs a nested loop to sort a list of pointers and then either fills a memory area with data or clears it with zeros.
- `copy_and_reorganize_memory_blocks`: This routine performs complex memory operations including copying data from one location to another, filling memory with a specific four-byte pattern, and reorganizing data based on calculated offsets and pointer lists.
- `check_status_flag`: Checks a bit in a global variable and, if set, performs a series of register writes and waits for a bit in a global variable.
- `fill_memory_pattern`: Fills a memory range with a repeating pattern of words and then fills a subsequent range with a value derived from a shifted register.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d0.
- `shift_and_call_subroutine`: Stores a register value in another register, performs a logical shift right on the original value, and calls another routine twice.
- `set_clock_bit_in_byte`: Sets or clears a single bit in a byte depending on bit zero of the byte before it, returning early when bit six is already set in the no-flag case.
- `apply_bit_manipulation_to_pointer`: Performs bitwise operations and shifts on a value before storing it through a pointer.
- `check_memory_signature`: Compares the contents of a memory location against a specific constant and calls another routine if they match.
- `initialize_and_fill_tables`: Clears a series of memory entries and then executes a complex loop that performs bitwise operations, table lookups, and data copying to populate multiple tables.
- `shift_bit_stream`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `fill_accumulated_code_tables`: Sets the first table entry to zero, then runs fifteen steps that accumulate counts from a source array into running sums.
- `decompress_bitstream_data`: Decompresses a bit-packed data stream into a destination memory buffer using variable-length bit sequences.
- `enable_interrupts`: Writes a value to the interrupt enable register to enable interrupts.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_encoded_data`: Calculates several values based on input registers and stores them as a sequence of bytes through a pointer.
- `bulk_transfer_indexed`: Iteratively transfers data from a source table to various destinations using high and low byte combinations from a source buffer.
- `copy_table_data`: Copies data from a source table to multiple destination locations using indexed lookups.
- `generate_bit_pattern_table`: Calculates a series of bit-shifted values and stores them sequentially into a memory location.
- `update_global_flag`: Stores the bitwise NOT of a byte into a global variable, potentially setting it to a specific value if the result is a certain constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_return_address_and_trap`: Saves registers d1-d7 and a0-a6, computes its own return address and stores it into a global variable, then executes trap zero to enter the operating system.
- `init_system_and_setup_vectors`: Disables interrupts and initialises the custom chip registers, then stores a jump vector table and several routine addresses into global variables and hardware structures.
- `set_flags_and_store_words`: Reads three 16-bit values from global variables at offsets 28, 30 and 32 of the structure in a6, calls a routine that sets bit 15 of each value through a pointer, then stores the results into two consecutive 32-bit words in memory.
- `set_values_to_max`: Stores the maximum signed word value into three consecutive word-sized locations relative to a pointer.
- `initialize_memory_values`: Calls two routines, then stores three maximum signed word values from a table into consecutive locations relative to a structure, copies two longwords into global variables, and clears a word at a fixed offset.
- `set_flag_and_store`: Sets bit 15 of a byte register and stores the result through a pointer, then returns.
- `wait_until_flag_clears`: Tests a byte at offset 6 of a structure and loops until that byte becomes zero, otherwise it returns.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_index_updates_then_write_space`: Calls a routine that processes index-table updates, then writes the value 32 to the byte at offset 156 of the structure in a6, restores the saved registers, and returns.
- `check_hardware_and_set_value`: Checks certain bits in a hardware register and, if a condition is met, writes a value to a memory location relative to a pointer.
- `modify_byte_and_loop`: Performs bitwise operations on a byte in memory, updates a global variable based on another memory value, and executes a fixed-count loop.
- `find_string_null_terminator`: Searches through a string in memory until a null terminator or a specific condition is met, potentially calling another routine during the process.
- `copy_string_to_memory`: Compares a string against a global variable and then copies bytes from a source string to a destination using a fixed offset increment.
- `process_index_table_updates`: Reads a 16-bit index from a global variable and clears it, then for values 66 to 79 except 80 and 81 it increments a second global counter and, if the matching table entry equals 5, sets that entry to 18 and increments it unless the counter is 1, between 10 and 39 with any of bits 0, 1, or 2 set, or above 20.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_encode_data`: Copies a block of data from one location to another, potentially performing a memory shift, and then writes encoded byte values through a pointer.
