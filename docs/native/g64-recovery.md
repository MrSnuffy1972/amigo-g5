# Game 64 (Supremacy v1.1 1384): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00064). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1566299 top-level bytes in 22 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 141 of 141 named (0 author symbols), 141 of 141 described (9 by rule, 132 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1060 of 1060 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 661504 bytes, 29 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2608 of 2662 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 895000 |
| code | 10528 |
| data | 660767 |
| padding | 4 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Supremacy.Slave | 1 | 140 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 75 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 855 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 765 | 70 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| DIEM (raw code file, load address unknown) | 1 | 0 | 0 |
| GAM1 (raw code file, load address unknown) | 1 | 0 | 0 |
| GAM2 (raw code file, load address unknown) | 1 | 0 | 0 |
| INT2 (raw code file, load address unknown) | 1 | 0 | 0 |
| Main (raw code file, load address unknown) | 1 | 804 | 29 |

## Cases handled

- runtime segment an address: 21 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 75 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 634 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f150): 286 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Supremacy_v1.1_1384/Supremacy/SupremacyData/DIEM: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Supremacy_v1.1_1384/Supremacy/SupremacyData/GAM1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Supremacy_v1.1_1384/Supremacy/SupremacyData/GAM2: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Supremacy_v1.1_1384/Supremacy/SupremacyData/INT2: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Supremacy_v1.1_1384/Supremacy/SupremacyData/Main: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 4 never-loaded region(s), 657408 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `toggle_ciab_bit_7` (runtime segment an address (f150)): Calls a wait routine, toggles bit 7 of a global variable related to CIAB port B, and calls another routine. Kind: disk hardware check (CIAB PRB bit 7). Pass path: returns 0.
- `toggle_ciab_bits_1_7` (runtime segment an address (f150)): Calls a wait routine, toggles bits 1 and 7 of a global variable related to CIAB port B, and calls another routine. Kind: disk hardware check (CIAB PRB bit timing/state). Pass path: branch taken by btst.b #an address, an address.l (bit 2 of CIA B PRB register checked).
- `verify_and_load_data_blocks_3_2` (runtime segment an address (f150)): Iterates through a linked list of data structures to verify integrity by comparing computed sums against stored values, then configures hard disk paths. Kind: disk hardware timing/checksum verification. Pass path: returns 0.
- `verify_and_load_data_blocks_3` (runtime segment an address (f150)): Iterates through a linked list of data structures to verify integrity by comparing computed sums against stored values, then configures hard disk access parameters. Kind: disk hardware timing/checksum verification. Pass path: returns 0.
- `verify_and_load_data_blocks_2` (runtime segment an address (f150)): Iterates through a linked list of data structures to verify integrity by comparing computed sums against stored values, then configures hard disk parameters. Kind: disk hardware timing/checksum verification. Pass path: returns 0.
- `verify_and_load_data_blocks` (runtime segment an address (f150)): Iterates through a linked list of data structures to verify integrity by comparing computed sums against stored values, then configures hardware registers for block transfer operations. Kind: disk hardware timing/checksum verification. Pass path: returns 0.
- `configure_disk_and_dma` (Main (raw code file, load address unknown)): Conditionally writes to the disk length, sync, DMA control, and interrupt request registers to configure disk drive operations and DMA channels. Kind: disk hardware configuration / DMA setup for protection check. Pass path: returns normally (no explicit branch shown, but routine completes without error).
- `toggle_ciab_port_b_bits` (Main (raw code file, load address unknown)): Xors a global variable with a specific mask to toggle bits, while relying on external routines to wait for flags and configure the CIAB timer. Kind: disk hardware check (CIAB port B). Pass path: btst.b #an address, an address.l returns 0 (bit 2 clear), branch beq.b an address is taken.
- `verify_boot_data_integrity` (Main (raw code file, load address unknown)): The routine scans a memory block for specific marker values and calculates checksums to verify data integrity. Kind: disk hardware timing/checksum verification. Pass path: returns 0.

## Functions


### Supremacy.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `configure_slave_entry_point`: Sets up a function pointer table and decrypts a code block by XORing it with a key to prepare the slave process for execution.
- `setup_slave_entry_point_2`: Configures a function pointer table and decrypts a block of code by XORing it with a key, preparing the slave process for execution.
- `setup_slave_entry_point`: Configures a function pointer table and decrypts a block of code by XORing it with a key, preparing the slave process for execution.
- `return_immediately`: The routine performs a return instruction to exit execution.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `query_ssg_status`: References the text string 'SSG?' and calls a routine to return a sign-extended word.
- `return_sign_extended_word`: The routine takes a 32-bit parameter, extracts the lower 16 bits, sign-extends them to 32 bits, and returns the result.
- `call_function_pointer_table`: It adds the ASCII value of '1' to a byte in a register, uses the result to index a table of function pointers, and calls the selected function.
- `handle_win_magic`: Checks if a register value matches a specific magic number (a fixed address).
- `return_immediately_4`: This routine is empty and consists solely of a return instruction, transferring control back to the caller without modifying any data or registers.
- `return_immediately_2`: The routine consists solely of a return instruction.
- `initialize_hardware_and_decode`: The routine writes various values to global variables and memory locations, calls several other routines, and then enters a loop that decodes data by XORing bytes from a source buffer with a target buffer based on offset tables.
- `tiny_move`: Small routine of 1 instruction that copies d0 to -(a7).

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decode_xor_data_block`: It iterates through a data structure, using offset values to calculate target addresses, and XORs bytes from a source pointer with the target memory locations until a counter reaches zero.
- `setup_dma_and_decode`: The routine writes to global variables, calls another routine, and then performs the same data decoding operation as R6 by XORing source bytes with target memory based on offset tables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_decode_buffer`: Calculates a buffer size based on input parameters, clears the buffer with zeros, and calls a routine to decode data into the prepared memory area.
- `decode_huffman_data`: Initializes a buffer with zeros and then enters a loop to decode data.
- `compare_register_values`: Compares two register values, likely acting as a bounds check or conditional guard before returning control to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_dma_and_cia_2`: Sets a value in a global structure and configures DMA channels by writing to the DMACON register, while also accessing CIAA port A data.
- `decode_music_and_copper`: Parses a music module format by interpreting command bytes and note values from a data stream.
- `decode_music_module_data`: Parses a music module format by interpreting command bytes and note values from a data stream.
- `process_copper_list_entries`: It waits for a global variable to equal the tilde character, then iterates through a list of structures, writing values to the Copper list memory at a fixed address based on flags within each structure.
- `configure_copper_dma`: Calculates offsets and selects data pointers from global tables based on object state flags.
- `check_flag_and_set_byte`: Checks if a specific bit in a byte at a global pointer is set and if a global variable is non-zero, while also verifying a short at an offset in a structure is zero.
- `update_structure_from_table`: Conditionally sets a byte in a structure based on a flag.
- `process_hunk_header_data`: The routine interprets AmigaDOS hunk header fields stored in a structure, performing arithmetic on size and offset values.
- `calculate_scaled_value`: Reads a short from a struct at offset 112, shifts it left by 2, multiplies it by a value derived from a byte at offset 4 of another struct, shifts the result right by 6, and stores the final short at offset 114 of the first struct.
- `initialize_timer_fields`: Checks a byte at offset 5 of a source struct.
- `adjust_state_counter_2`: Loops while certain flags are set.
- `calculate_scaled_difference`: Computes a scaled difference between two values from a table, using offsets derived from struct fields and conditional logic.
- `adjust_state_counter`: Increments or decrements a byte field in a struct based on comparisons with other fields and specific flag values.
- `extract_nibble_value`: Shifts the upper nibble of a byte from one struct field into a lower position and stores it in another struct field.
- `decrement_state_counter`: Decrements a byte-sized counter at a specific offset in memory; if the counter reaches zero, it calls another routine to handle the state change.
- `initialize_timer_values`: Sets multiple byte fields in a struct based on the value of another byte, including shifting and masking operations.
- `compute_clamped_sum`: Calculates a sum involving several byte fields, applies sign logic, and clamps the result before storing it.
- `decode_pointer_data`: Reads bytes from a memory address pointed to by a struct field and distributes them into various struct fields.
- `update_progress_state`: Updates state variables and returns a value based on comparisons between progress counters and limits.
- `process_hunk_relocation`: Processes data related to AmigaDOS hunk types.
- `configure_dma_and_cia`: Sets a value in a global structure and configures DMA channels by writing to the DMACON register, while also accessing CIAA port A data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_interrupt_service`: Polls for interrupt flags, checks joystick input, triggers potentiometer reads, and configures CIA timers and display hardware.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `empty_stub_routine`: This routine is empty and performs no operations.
- `check_joystick_input_flags`: Checks specific bits in a global variable related to joystick input, updating the state flags and calling a routine to read joystick data if necessary.
- `wait_for_interrupt_flag`: Polls a global variable until a specific bit is set, then clears that bit.
- `trigger_pot_go`: Writes to the POTGO hardware register to initiate a potentiometer read cycle.
- `validate_global_state`: Checks if a global variable is zero or negative, returning immediately if the condition is met, otherwise proceeding without further action.
- `empty_stub_return_2`: The routine is empty and immediately returns without performing any operations.
- `empty_stub_return`: The routine is empty and immediately returns without performing any operations.
- `loop_while_carry_set`: Repeats a call to another routine as long as the carry flag is set, clearing the flag after each iteration.
- `read_keyboard_serial`: The routine reads from the CIAA serial data register and processes the bit stream to determine keyboard input status, storing the result in a global variable.
- `process_joystick_input`: The routine reads joystick and mouse data from CIAA and POTGOR registers, processes button states and movement deltas, and updates global variables with clamped input values.
- `configure_cia_timers`: Writes specific values to CIAA and CIAB control, interrupt, and timer registers to initialize hardware timing and interrupt handling.
- `wait_for_global_zero_loop`: Repeats a call to another routine while a global variable is non-zero, setting the variable to a negative constant between iterations.
- `wait_for_global_zero`: Sets a global variable to a negative value and loops until that variable becomes zero, calling another routine in between checks.
- `wait_for_global_zero_2`: Loops until a global variable becomes zero.
- `configure_ciab_timer`: The routine writes configuration values to CIAB control registers and timer registers to set up a hardware timer interval.
- `wait_for_flag_clear`: The routine loops continuously until a specific global variable becomes zero, effectively waiting for a condition to be met.
- `clear_interrupt_flags`: The routine checks and clears specific bits in the CIAB interrupt control register and the global interrupt request register, then sets a memory flag.
- `process_ciaa_interrupts`: Reads keyboard serial data and manipulates global variables based on bit flags in the CIAA interrupt control register.
- `initialize_display_hardware`: The routine configures multiple copper list registers, display window registers, and DMA control registers to set up the graphics display and enable interrupts.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `halt_cpu_execution`: The routine enters an infinite loop that performs no operations, effectively stopping the processor from executing further instructions.
- `trap_and_wait_loop`: Executes a trap instruction and then enters a loop that checks a global variable, calling a stop routine if the variable is non-zero.
- `read_joystick_state`: Processes joystick and mouse input data from hardware registers to update global state variables.
- `copy_block_data`: The routine copies several words and bytes from a source pointer into global variables, including a calculated value derived from a byte in the source block.
- `process_table_entries`: Iterates through a table of entries, copying blocks of data and performing calculations based on byte values and global variables.
- `do_nothing_stub`: The routine contains no instructions and returns immediately.
- `dispatch_indirect_jump_3`: Calls a helper routine to compare memory blocks and checks its return value.
- `compare_memory_blocks_2`: Compares eight bytes from two memory locations, one offset by negative 512 from a global variable and the other at the global variable address.
- `compare_memory_blocks`: Compares eight bytes from two memory locations, one offset by negative 512 from a global variable and the other at the global variable address.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 223 into d0, then returns.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `copy_chunk_data`: Searches a linked list of chunk headers for a matching identifier and copies the associated data block to a destination pointer.
- `copy_word_blocks_2`: Copies four words from a source pointer to a destination pointer in a loop controlled by a register.
- `copy_word_blocks`: Copies four words from a source pointer to a destination pointer in a loop controlled by a register.
- `process_buffer_loop_2`: Iteratively processes a buffer by calling a helper routine and checking its return value, adjusting a counter until a condition is met.
- `process_buffer_loop`: Iteratively processes a buffer by calling a helper routine and checking its return value, adjusting a counter until a condition is met.
- `dispatch_indirect_jump_2`: Performs an indirect jump through a jumptable pointer after calling another routine and checking its return value.
- `dispatch_indirect_jump`: Performs an indirect jump through a jumptable pointer after calling another routine and checking its return value.
- `empty_stub_routine_2`: This routine is empty and immediately returns without performing any operations or modifying any data.
- `return_immediately_3`: The routine is empty and returns immediately without performing any operations or modifying data.
- `configure_dma_registers`: Conditionally writes specific values to global variables associated with DMA control, disk length, and CIA port B registers based on a register value check.
- `poll_cia_port_a_bit`: Checks a bit in the CIAA port A register, performing arithmetic on pointer data and looping while the bit remains set.
- `poll_cia_port_loop_2`: Repeatedly calls another routine and checks a global variable representing a CIA port bit.
- `wait_for_cia_bit_alt`: Loops calling a helper routine and checking a CIA port bit, returning a specific error code if the bit is set or zero otherwise.
- `poll_cia_port_loop`: Repeatedly calls another routine and checks a global variable representing a CIA port bit.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 242 into d0, then returns.
- `toggle_ciab_bits_conditional`: Checks a bit in a global variable to decide whether to toggle bits 1 and 7 or only bit 7 of a CIAB-related global variable.
- `toggle_ciab_port_bits`: Calls a wait routine, toggles bit 7 of a global variable related to CIAB port B, and calls another routine.
- `check_cia_port_bit`: Checks if a specific bit is set in a global variable related to CIAA port A. If set, it calls another routine, toggles bits in the global variable, and calls another routine again before returning zero; otherwise returns an error code.
- `toggle_ciab_bit_7`: Calls a wait routine, toggles bit 7 of a global variable related to CIAB port B, and calls another routine.
- `check_cia_port_bit_alt`: Checks if a specific bit is set in a global variable related to CIAA port A. If set, it calls another routine, toggles different bits in the global variable, and calls another routine again before returning zero; otherwise returns an error code.
- `toggle_ciab_bits_1_7`: Calls a wait routine, toggles bits 1 and 7 of a global variable related to CIAB port B, and calls another routine.
- `configure_ciab_timer_3`: Writes configuration values to CIAB control registers and timer registers to set up a hardware timer interval.
- `configure_ciab_timer_2`: Writes specific configuration values to CIAB control and timer registers to set up a hardware timer interval.
- `copy_table_entries`: Copies blocks of 128 bytes from source addresses defined in a table to a destination pointer, iterating through multiple entries.
- `copy_byte_blocks`: Copies blocks of 128 bytes from source addresses pointed to by a table to a destination pointer, iterating through multiple entries.
- `copy_byte_blocks_indirect`: Copies 128 bytes from a source address pointed to by a table entry to a destination pointer, repeating this process for multiple entries in the table controlled by a counter.
- `verify_and_load_data_blocks_3_2`: Iterates through a linked list of data structures to verify integrity by comparing computed sums against stored values, then configures hard disk paths.
- `verify_and_load_data_blocks_3`: Iterates through a linked list of data structures to verify integrity by comparing computed sums against stored values, then configures hard disk access parameters.
- `verify_and_load_data_blocks_2`: Iterates through a linked list of data structures to verify integrity by comparing computed sums against stored values, then configures hard disk parameters.
- `verify_and_load_data_blocks`: Iterates through a linked list of data structures to verify integrity by comparing computed sums against stored values, then configures hardware registers for block transfer operations.
- `validate_data_checksum`: Iterates through a linked list of data structures to verify integrity by comparing a computed sum against a stored value.
- `tiny_moveq_rts_4`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `set_global_to_ff`: Sets a global variable to 255 and returns a specific error code.
- `verify_data_integrity_and_load`: Iterates through a linked list of data blocks to verify checksums and magic numbers, then configures hardware registers for DMA and interrupts before loading validated data.
- `decode_xor_data_stream`: Iterates through a data stream, skipping zero bytes while adjusting a target pointer based on subsequent values, then XORs the target memory with the current data byte until a counter reaches zero.
- `decode_huffman_stream`: Initializes a buffer with zeros and decodes a compressed data stream using a Huffman tree structure, writing decompressed bytes to memory.
- `check_address_range`: Compares a value in register A1 against a value in register A4 and returns if the first is less than the second.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_routine_3`: This routine is empty and performs no operations before returning.
- `do_nothing`: The routine is empty and returns immediately without performing any operations.

### Main (raw code file, load address unknown)

- `initialize_system_hardware`: Configures CIA timers, display hardware, and DMA control registers, then verifies data block integrity before entering a trap-and-wait loop.
- `wait_clear_interrupt_flag`: Polls the interrupt request register until a specific bit is cleared, then writes to the register to acknowledge or clear the interrupt state.
- `trigger_pot_go_read`: Writes to the POTGO register to initiate a potentiometer or joystick port read cycle.
- `check_global_state_validity`: Evaluates a global variable to determine if it is zero or negative, returning immediately if either condition is met.
- `loop_on_carry_flag`: Recursively calls itself while the carry flag is set, setting the carry flag to 1 in each iteration.
- `read_serial_keyboard_data`: Reads a byte from the CIA serial data register, processes its bits to determine status, and stores the result into a global variable.
- `process_joystick_input_2`: Reads joystick and mouse data from hardware registers, processes button states and movement deltas, clamps the values against global limits, and updates global input variables.
- `configure_cia_timer_registers`: Writes specific values to CIAA and CIAB timer and control registers to configure timer intervals and interrupt settings.
- `wait_for_idle_state`: Polls a global variable in a loop until it becomes zero, effectively waiting for a specific system state to clear.
- `initialize_copper_and_dma`: Configures multiple copper list registers, display window dimensions, DMA control, and interrupt enable registers to set up the graphics and interrupt system.
- `enable_dma_channels`: Sets bit 15 of the DMACON register to enable DMA channels.
- `trap_and_wait_loop_2`: Executes a trap instruction and then loops until a global variable becomes zero, calling a stop routine if the variable is non-zero.
- `copy_block_data_2`: Copies several words and bytes from a source pointer into global variables, including a calculated value derived from a byte in the source.
- `jump_vector_entry_2`: Acts as a jump-table entry that calls a helper routine to compare memory blocks.
- `verify_data_block_integrity`: Checks if a data pointer is null; if so, it initializes a global variable and calls helper routines.
- `parse_and_copy_data_block`: Iterates through a linked list of data structures to find a matching entry, then calculates offsets and copies block data to a destination buffer or processes it via a jump table.
- `configure_disk_and_dma`: Conditionally writes to the disk length, sync, DMA control, and interrupt request registers to configure disk drive operations and DMA channels.
- `configure_dma_channels`: Checks a global variable and, if negative, writes specific values to it.
- `process_cia_port_data`: Reads and processes data from the CIAA port A register, performing bitwise operations and conditional checks on the input values.
- `toggle_ciab_port_bits_2`: Checks a bit in the CIAA port A register and toggles specific bits in a global variable related to CIAB port B based on the result.
- `toggle_global_flag_on_condition`: Checks if a specific bit is set in a global variable.
- `toggle_ciab_portb_bit7`: Toggles the most significant bit of a global variable and configures the CIAB timer.
- `toggle_cia_flag_and_poll`: Checks a specific bit in a global variable and, if set, calls a polling routine, toggles bits in the global variable, and calls the polling routine again.
- `toggle_ciab_port_b_bits`: Xors a global variable with a specific mask to toggle bits, while relying on external routines to wait for flags and configure the CIAB timer.
- `initialize_cia_timers`: Compares an input byte with a global variable, sets the global variable to specific constant values, and calls a routine to configure CIA timers.
- `copy_data_blocks`: Copies multiple 128-byte blocks from source addresses defined in a table to a destination pointer, iterating through entries.
- `verify_boot_data_integrity`: The routine scans a memory block for specific marker values and calculates checksums to verify data integrity.
- `fill_buffer_with_shifted_data`: Initializes a buffer with zeros and then populates it by shifting bits from a global variable, copying values based on bit patterns, and performing circular buffer updates.
- `check_index_bound`: Compares a value in register A1 against a value in register A4 and returns if A1 is less than A4.
