# Game 2320 (smash tv (1991)(ocean)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02320). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916048 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 16 raw-word lines |
| Functions named and described | pass, 106 of 106 named (0 author symbols), 106 of 106 described (3 by rule, 98 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1328 of 1328 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 272 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2102 of 2102 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 40560 |
| code | 7772 |
| data | 834948 |
| unused | 32768 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Smash TV (1991)(Ocean)[cr SKR].adf:bootblock | 1 | 16 | 1 |
| logged disk load from Smash TV (1991)(Ocean)[cr SKR].adf boot-block load | 1 | 94 | 4 |
| logged disk load from Smash TV (1991)(Ocean)[cr SKR].adf boot-block load | 1 | 94 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1124 | 45 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 687 | 50 |
| logged disk load from Smash TV (1991)(Ocean)[cr SKR].adf | 1 | 69 | 2 |
| Smash TV (1991)(Ocean)[cr SKR].adf bytes 50176-50256 beside a logged load (code never loaded by a reference run, load address unknown) | 1 | 0 | 0 |
| Smash TV (1991)(Ocean)[cr SKR].adf bytes 50496-50688 beside a logged load (code never loaded by a reference run, load address unknown) | 1 | 18 | 1 |

## Cases handled

- smash tv (1991)(ocean)[cr skr]/Smash TV (1991)(Ocean)[cr SKR].adf: custom-format (non-DOS) disk, 1687 non-empty blocks: matched against track-loaded segments
- smash tv (1991)(ocean)[cr skr]/Smash TV (1991)(Ocean)[cr SKR].adf: the boot block reads 22528 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- smash tv (1991)(ocean)[cr skr]/Smash TV (1991)(Ocean)[cr SKR].adf: the boot block reads 11264 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 901 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 315 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- smash tv (1991)(ocean)[cr skr]/Smash TV (1991)(Ocean)[cr SKR].adf: 10 executed PCs whose RAM was overwritten before any snapshot match its bytes 50256-50496: recovered as a unit run at an address
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate+displacement
- Smash TV (1991)(Ocean)[cr SKR].adf bytes 50176-50256 beside a logged load (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 0 of 80 bytes decode)
- Smash TV (1991)(Ocean)[cr SKR].adf bytes 50496-50688 beside a logged load (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 52 of 192 bytes decode)
- smash tv (1991)(ocean)[cr skr]/Smash TV (1991)(Ocean)[cr SKR].adf: 5 logged loads cover 34032 bytes of the image
- smash tv (1991)(ocean)[cr skr]/Smash TV (1991)(Ocean)[cr SKR].adf: blocks 24-97 (bytes 12288-50176) outside the filesystem hold data, not code (opcode density 0.053, entropy 7.15)
- smash tv (1991)(ocean)[cr skr]/Smash TV (1991)(Ocean)[cr SKR].adf: blocks 1661-1662 (bytes 850432-851456) hold ASCII text
- 1 never-loaded region(s), 80 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 794112 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `verify_and_validate_asset_block` (runtime segment an address (play-rs-m1500)): Calculates a checksum of a data block and compares it against a stored value to ensure data integrity. Kind: disk track/checksum validation. Pass path: The branch where uVar6 (the calculated checksum) is compared against *puVar13 (the stored checksum) and the subsequent call to u4__sub_2144() is executed..
- `verify_and_load_asset_block` (runtime segment an address (play-rs-m1500)): Validates a data block by calculating a checksum and comparing it against a stored value. Kind: disk track/sync check and checksum verification. Pass path: The branch where uVar6 (checksum) is compared against *puVar13 and _u4__d0_1FFE == 1, leading to the loading of assets via u4__sub_2144.
- `set_and_wait_flag` (runtime segment an address (play-rs-m1500)): Stores a value into a global variable and then calls another routine. Kind: disk hardware/status check. Pass path: bne.b an address.
- `set_and_wait_flag_alt` (runtime segment an address (play-rs-m1500)): Stores a different value into a global variable and then calls another routine. Kind: disk hardware/sync check. Pass path: bra.b an address.
- `configure_disk_and_dma` (runtime segment an address (play-rs-m1500)): Conditionally writes to the disk length, sync, DMA control, and interrupt request registers to configure disk drive operations and DMA channels. Kind: disk hardware/sync validation. Pass path: the instruction immediately following the call (e.g., st.b an address.l).
- `configure_dma_channels` (runtime segment an address (play-rs-m1500)): Checks a global variable and, if negative, writes specific values to it. Kind: disk hardware/configuration check. Pass path: the branch at an address (bne.b an address is not taken).
- `toggle_ciab_bit` (runtime segment an address (play-rs-m1500)): Calls a wait routine, toggles bit 7 of a global variable related to CIA port B, and calls another routine. Kind: disk hardware/status check (CIAB_PRB). Pass path: the branch/path that does NOT trigger the bit-toggle logic or subsequent failure state.
- `toggle_ciab_bits_1` (runtime segment an address (play-rs-m1500)): Calls a wait routine, toggles bits 1 and 7 of a global variable related to CIA port B, and calls another routine. Kind: disk hardware/CIAB_PRB check. Pass path: the branch following the btst.b #an address, an address.l instruction (the 'beq.b an address' is the failure/exit path).

## Functions


### Smash TV (1991)(Ocean)[cr SKR].adf:bootblock

- `perform_bitstream_decompression`: Reads a bitstream from a source and performs a series of conditional shifts and writes to reconstruct bytes into a destination memory area.

### logged disk load from Smash TV (1991)(Ocean)[cr SKR].adf boot-block load

- `decompress_bitstream_data_2`: Reads a compressed bitstream using bit-level operations to reconstruct data into a destination buffer.
- `write_decrementing_global_until_wrap`: Writes a decrementing word to a global variable in a loop that runs until the counter wraps, then returns the value minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Smash TV (1991)(Ocean)[cr SKR].adf boot-block load

- `write_decrementing_global_until_wrap_2`: Writes a decrementing word to a global variable in a loop that runs until the counter wraps, then returns the value minus one.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `trigger_software_interrupt`: Sets a global variable to a pointer and then executes a software trap.
- `initialize_system_tables`: Clears a large memory range, populates several tables with constant data, and enters a loop that waits for a global variable to change.
- `update_structure_fields`: Increments a byte in a global variable and, if it reaches a specific value, resets it to zero before writing several multi-byte values to a structure.
- `copy_table_data_by_index`: Searches a table for a matching byte, then copies a block of data from a calculated offset to a destination address.
- `store_multiple_values_to_offset`: Stores several constant values into various offsets relative to a pointer.
- `update_memory_table_via_loop`: If a global variable is zero, the routine performs a loop that calculates and stores values into a memory range using data from two separate tables and an offset.
- `initialize_structure_loop`: Writes several constant values to a structure and then executes a loop that calls another routine a fixed number of times.
- `store_registers_to_memory`: Waits for a specific bit in a memory location to clear, then stores several registers into a structure.
- `copy_data_from_lookup_table`: Iterates through a list of bytes, finds matches in a lookup table, and copies the corresponding data to a destination.
- `store_constants_after_bit_check`: Checks a bit in a memory location and, once cleared, writes several constant values to a structure.
- `increment_counter_and_check_global`: Increments a global counter and, upon reaching a threshold, checks a global variable and updates another global pointer.
- `process_table_entries`: Waits for a bit in a byte to clear, modifies several fields in a structure, and then iterates through a table to call another routine based on specific value comparisons.
- `calculate_and_store_distance_metrics`: Calculates horizontal and vertical differences between two points and stores the resulting dimensions and bitmask into a structure.
- `reset_structure_data`: Waits for a bit in a byte to clear and then writes a series of constant values to a structure before calling another routine four times.
- `store_word_to_pointer`: Checks a bit in a byte pointed to by a register and, if not set, stores a 32-bit value and a 16-bit value into a structure at an offset from that pointer.
- `initialize_loop_data`: Waits for a bit to clear, writes constant values to a structure, and then iterates through a table to perform bitwise operations and call another routine.
- `initialize_structure_data`: Checks a bit in a byte pointed to by a register and, if not set, writes several constant values and multiple register values into a structure at an offset from that pointer.
- `increment_table_values`: Iterates through two tables and increments the values in the first table to match the corresponding byte components of the second table.
- `decrement_bits`: Processes a word value through a series of bit tests and decrements, advancing pointers and looping based on a counter.
- `configure_hardware_interrupts`: Manipulates several hardware registers including DMA and interrupt control registers, performing a loop to check a value before setting specific register bits.
- `load_object_properties_2`: Performs complex initialization involving multiple loops to populate structures, manipulate memory, and interact with hardware registers.
- `clear_audio_volumes`: Sets four consecutive memory locations, which correspond to audio volume registers, to zero.
- `manage_state_counters`: Increments and resets two global counters and performs a series of calls to other routines based on their values.
- `process_object_attributes`: Iterates through a list of pointers to update structure members, including bitwise inversions and coordinate adjustments based on internal flags.
- `load_object_properties`: Uses a byte from a source to index into tables and stores the resulting pointer and two bytes into a structure.
- `accumulate_value`: Calls another routine and then adds a value from a structure to a global variable, updating a secondary field in the process.
- `select_highest_byte`: Compares bytes within a list and stores the largest byte found into a structure, while decrementing a counter.
- `setup_object_data`: Clears several fields in a structure and, if a specific value is non-zero, calculates a pointer to load data from a table into the structure.
- `update_clamped_value`: Adds a value to a short integer and clamps the result between two predefined limits before storing it in a structure.
- `increment_counter_on_match`: If a byte from a pointer matches a value in a table, it increments a counter or adds the byte to a short value.
- `update_entity_state_attributes`: Processes an entity state based on a command index extracted from a word.
- `update_value_with_clamping`: Updates a value by adding a signed offset and clamps the result between a minimum and maximum range.
- `fetch_table_value`: Extracts bits from a word to index into a table and uses the result to calculate and store a new value into a structure.
- `negate_value_in_structure`: Reads a 16-bit word from an offset of 52 in a structure, negates it, and stores the result back at an offset of 58 in the same structure.
- `copy_value_in_structure`: Copies a 16-bit word from an offset of 52 in a structure to another offset of 58 in the same structure.
- `update_position_and_flag`: Checks several conditions and updates a value at offset 12 in a structure, setting it to 255 if a calculated sum exceeds 255.
- `copy_byte_to_two_locations`: Copies a single byte from a table into two consecutive byte locations in another structure.
- `store_byte_to_offset`: Stores a byte from a table into a specific offset of another structure.
- `scale_and_store_value`: Stores a short value into a table and then stores its quadruple value, clamped to a maximum, into another location.
- `check_bit_mask`: Checks if the lower four bits of a byte are non-zero and, if so, stores that value into a specific byte location.
- `process_table_data_with_offset`: Compares values from a table, increments a counter, and adds a value retrieved from a secondary table using an index.
- `update_value_from_table_index`: Checks table values, increments a counter, and updates a short value using a calculation involving a table and a base address.
- `copy_data_from_indexed_table`: If table conditions are met, it increments a counter and copies four bytes of data from an indexed table into another location.
- `shift_and_store_value`: Calls another routine and then stores a bit-shifted version of a value from one structure into another.
- `update_state_dependent_value`: Checks a state variable and performs specific arithmetic operations on a target value based on that state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_decompress_graphics`: Allocates memory and configures hardware registers for display and DMA before performing dictionary-based bitstream decompression to populate a memory buffer.
- `decompress_bitstream_data`: Reads a bitstream to perform a dictionary-based decompression, populating memory by copying sequences of previously encountered data or inserting new values based on the decoded bit patterns.
- `verify_and_validate_asset_block`: Calculates a checksum of a data block and compares it against a stored value to ensure data integrity.
- `verify_and_load_asset_block`: Validates a data block by calculating a checksum and comparing it against a stored value.
- `verify_and_decompress_asset_block`: Validates a data block by calculating a checksum and comparing it against a stored value.
- `verify_checksum_and_decompress_data`: Validates a data block by searching for a specific signature and calculating a checksum.
- `set_and_wait_flag`: Stores a value into a global variable and then calls another routine.
- `set_and_wait_flag_alt`: Stores a different value into a global variable and then calls another routine.
- `wait_for_interrupt_flag`: Polls a global variable until a specific bit is set, then clears that bit.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `set_interrupt_flag`: Sets bit 13 of the status word on the stack and returns, effectively enabling an interrupt flag.
- `initialize_hardware_and_decompress_data`: Configures display registers, DMA control, and CIA interrupts before performing a bitstream-based decompression routine to populate memory.
- `tiny_move_rte`: Small routine of 2 instructions that copies d0 to d0, then returns from the exception.
- `update_status_flags`: Checks and modifies bits in a global variable and clears an interrupt request bit.
- `wait_and_set_interrupt_flag`: Polls a global variable until a specific bit is set, then sets that same bit.
- `write_pot_values`: Stores two specific values into a global variable.
- `check_zero_value`: Compares a global variable against zero.
- `check_global_sign`: Checks if a global variable is non-negative and returns.
- `nop_sequence`: This routine is empty and performs no operations.
- `execute_unconditional_call`: Performs a call and then sets a register to one.
- `read_keyboard_serial`: The routine reads from the CIAA serial data register and processes the bit stream to determine keyboard input status, storing the result in a global variable.
- `process_joystick_input`: The routine reads joystick and mouse data from CIAA and POTGOR registers, processes button states and movement deltas, and updates global variables with clamped input values.
- `setup_display_registers`: Stores a series of values into registers to configure display parameters, copper settings, and DMA control.
- `clear_interrupt_request`: This routine clears a bit in the interrupt request register.
- `configure_cia_interrupts`: Sets multiple values into global variables and configures CIA and interrupt enable registers.
- `execute_unreachable_loop`: This routine is unreached and contains a loop that polls a global variable and performs conditional calls.
- `loop_with_constant_assignment`: Assigns a constant to a global variable and enters a loop based on the result of another routine.
- `process_conditional_loop`: Polls a global variable in a loop and performs conditional logic involving a constant and another routine.
- `wait_for_zero`: Polls a global variable until its value becomes zero.
- `clear_interrupt_flags`: The routine checks and clears specific bits in the CIAB interrupt control register and the global interrupt request register, then sets a memory flag.
- `update_ciaa_interrupt_status`: Reads a global variable, clears it, and checks a specific bit to conditionally call another routine.
- `copy_block_data`: The routine copies several words and bytes from a source pointer into global variables, including a calculated value derived from a byte in the source block.
- `parse_and_execute_data_table`: Iterates through a table of data structures to calculate a value based on specific byte offsets and bitwise operations.
- `execute_indirect_calls`: Iteratively calls a routine and performs indirect jumps through a table based on the returned values.
- `verify_data_block_integrity`: Checks if a data pointer is null; if so, it initializes a global variable and calls helper routines.
- `search_and_copy_table_entry`: Searches a linked list or table for a specific identifier and, if found, performs a block memory copy of a data structure to a new destination.
- `configure_disk_and_dma`: Conditionally writes to the disk length, sync, DMA control, and interrupt request registers to configure disk drive operations and DMA channels.
- `configure_dma_channels`: Checks a global variable and, if negative, writes specific values to it.
- `process_pointer_and_decrement`: Reads a value from a pointer, performs bitwise shifts and comparisons, and loops while decrementing a calculated difference.
- `decrement_and_loop`: Stores a word through a pointer, modifies a global variable based on a bit, and loops while decrementing a value.
- `load_constant_two_hundred_forty_two`: Loads the value two hundred forty-two into a register and returns.
- `execute_retry_loop`: Repeatedly calls another routine and checks a flag to determine whether to continue the loop or return a specific error value.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 242 into d0, then returns.
- `toggle_ciaa_bits`: Checks a bit in a global variable and, based on the result, toggles either bits 1 and 7 or bit 7 of a CIA register through a global variable.
- `toggle_flag_when_bit_set`: Tests bit 2 of a global flag byte and, only if it is set, toggles bit 7 of that byte, returning zero; otherwise it returns a fixed error code (-14).
- `toggle_ciab_bit`: Calls a wait routine, toggles bit 7 of a global variable related to CIA port B, and calls another routine.
- `check_button_status`: Tests a specific bit in a hardware register and branches if that bit is not set.
- `toggle_ciab_bits_1`: Calls a wait routine, toggles bits 1 and 7 of a global variable related to CIA port B, and calls another routine.
- `update_global_variable`: Compares a register to a global variable and performs a series of sequential writes to that global variable.
- `execute_unnamed_routine`: An unnamed routine consisting of fourteen instructions.

### logged disk load from Smash TV (1991)(Ocean)[cr SKR].adf

- `wait_for_condition_and_call`: Waits for a global variable to become zero, then waits for another condition before calling a function and returning its result.
- `bcd_add_with_conditions`: Performs conditional operations based on global flags, doubles a value, and adds it to memory locations using BCD arithmetic.

### Smash TV (1991)(Ocean)[cr SKR].adf bytes 50496-50688 beside a logged load (code never loaded by a reference run, load address unknown)

- `bcd_add_with_pointer_copy`: Copies a byte from one location to another, performs conditional operations based on global flags, doubles a value, and adds it to memory locations using BCD arithmetic.
