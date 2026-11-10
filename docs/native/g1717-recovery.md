# Game 1717 (venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01717). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909968 top-level bytes in 21 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 185 raw-word lines |
| Functions named and described | pass, 109 of 109 named (0 author symbols), 109 of 109 described (5 by rule, 82 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1292 of 1292 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2484 of 2498 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 147512 |
| code | 9046 |
| data | 329840 |
| padding | 2706 |
| unused | 420864 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:Type | 2 | 117 | 2 |
| Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus | 2 | 1318 | 64 |
| Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:krayzi | 1 | 99 | 3 |
| Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 181 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 725 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:00: high-entropy data file (7.87 bits/byte), classified as packed data
- venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:01: high-entropy data file (7.64 bits/byte), classified as packed data
- venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:02: high-entropy data file (7.62 bits/byte), classified as packed data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 163 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 148 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +20A, venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +214
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +20A, venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +214
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +20A, venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +214
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +20A, venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +214
- self-modifying code at an address (63 bytes change between runtime segment an address and runtime segment an address (play-rs-x25k)): both forms recovered; writer: venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +20A, venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +214
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +20A, venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +214
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +20A, venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +214
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +20A, venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +214
- self-modifying code at an address (64 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +20A, venus the flytrap (1990)(gremlin)[cr miracle][t +6 atz](disk 1 of 2)[h krayzi]/Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus +214
- self-modifying code at an address (13 bytes change between runtime segment an address (play-rs-x25k) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialise_channel_record_fields` (venus): Clears several fields of a record, computes a table pointer from a masked index, and when a bit flag is set it stores two shared values into an indexed table. Kind: track/sector validation check. Pass path: bra.b an address.
- `process_hex_string_data` (runtime segment an address (play-rs-x25k)): Calculates ASCII characters from a value and performs a string comparison loop. Kind: checksum/data integrity check. Pass path: The loop that compares memory contents (pcVar6 vs pcVar7) and returns via u12__sub_1D80() after a successful match..

## Functions


### Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If the output pointer argument is non-null, copies sixteen longwords from a fixed offset in a base register into the memory it points to, then calls a function pointer held in a register.

### Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:venus

- `initialize_hardware_and_run`: Program entry that touches the CIA port A, copper pointer, DMA control, interrupt enable and request, joystick data and video position registers, and makes six unnamed calls.
- `install_jump_then_call_library`: Writes a jump-instruction encoding and an address into a global location, calls one unnamed routine, then calls a function through a table offset and returns zero.
- `stub_unanalysed_gap_routine`: A six-instruction routine decoded from a gap in the file, with no pseudo-code and no calls, callers, registers or text listed.
- `advance_state_cycle`: Updates a global state byte by cycling its value through a sequence of conditional writes when a specific flag bit is clear.
- `advance_state_counter_cycle`: Sets a global state byte through a short sequence of conditional writes, and when a flag bit is clear it advances that byte through a cycle that wraps back to zero before setting a fixed value.
- `setup_blitter_and_address_table`: Fills a table of address words by stepping an address forward by a fixed amount, then programs the blitter control, modulo, pointer and size registers and sets the background colour.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `interpret_command_bytes_until_end`: Reads a stream of command bytes until an end marker: a zero command sets two globals from the following bytes, a one command skips its operand, and any other value calls one unnamed routine and advances a counter that wraps at a fixed limit.
- `interpret_command_bytes_until_end_2`: Reads a stream of command bytes until an end marker: a zero command sets two globals from the following bytes, a one command skips its operand, and any other value calls one unnamed routine and advances a counter that wraps at a fixed limit.
- `interpret_command_bytes_until_end_3`: Reads a stream of command bytes until an end marker: a zero command sets two globals from the following bytes, a one command skips its operand, and any other value calls one unnamed routine and advances a counter that wraps at a fixed limit.
- `copy_eight_bytes_with_strides`: Copies eight bytes from a table, indexed by a byte value and stepping by a fixed stride, into a buffer addressed through an offset from a base, stepping the destination by a different fixed stride.
- `recursively_call_self`: This routine performs a conditional recursive call to itself based on the values of two global variables.
- `reset_audio_state`: Clears several global variables, sets one to a constant, and stores a value from a global variable into a calculated offset within a table.
- `store_word_to_fixed_location`: Stores the low word of the data register into a fixed memory location and returns.
- `update_entity_state_from_packed_data_2`: Unpacks a 32-bit value into multiple components and uses a derived index to update various fields within an entity structure.
- `copy_table_pairs_until_negative`: Sets two global flags, then copies pairs of words from two offset tables into global arrays, up to thirty-two entries, stopping early when the value read from the first table is negative.
- `return_empty_routine`: This routine performs no operations and immediately returns to its caller.
- `tick_countdown_and_reload`: Counts down a timer held in a record and sets a state field to one or two while it runs.
- `process_entity_state_logic`: Updates entity properties and state transitions by reading from a data table and applying conditional logic based on packed attribute values.
- `process_channel_data`: Checks a byte at an offset, updates a value in a table based on a comparison, and writes a constant to a memory location.
- `update_entity_animation_frame`: Manages entity animation timing and frame selection by decrementing a counter and updating frame indices from a data table.
- `update_entity_animation_frame_2`: Increments or decrements an entity's frame index based on a state-driven switch statement.
- `increment_and_copy_byte`: Copies a byte from a memory location pointed to by a register into a byte at a different memory location and increments a short integer value at a third location.
- `update_entity_behavior_state`: Processes an entity's state machine by reading a command byte from a data table and executing specific logic branches.
- `update_entity_animation_frame_3`: Updates an entity's current animation frame and sprite index by incrementing a counter and performing a lookup in a data table.
- `load_object_attribute_data`: Reads a sequence of attribute values from a data table and populates a structure with corresponding memory pointers and metadata.
- `load_channel_entries_and_process`: Loads multiple channel entries from a data block into a record and updates associated pointer and status fields.
- `reset_state_flag`: The routine calls another routine and then clears a word at an offset from a register, sets a global variable to a constant, and returns the value originally on the stack.
- `process_instruction_stream`: Reads a sequence of data from a table and executes a series of operations based on a command byte.
- `execute_entity_command_sequence`: Iterates through a table of command data to execute a series of operations based on a command type.
- `process_entity_state_machine`: Updates an entity's internal timer and manages a state machine by branching based on a value retrieved from a data table.
- `update_entity_state_machine`: Increments an internal timer and executes a state machine by branching based on values retrieved from a data table.
- `update_entity_state_machine_2`: Increments an internal timer and executes a switch-case logic based on a value retrieved from a data table.
- `update_entity_state_logic`: Increments an internal counter and fetches a state identifier from a data table to execute a switch-based logic block.
- `update_entity_state_logic_2`: Increments an internal timer and executes a large switch-case block to modify entity attributes such as position, direction, and animation frames based on a data table.
- `update_entity_state_data`: Copies a two-byte value from one memory location to another and increments a short integer at a specific offset within a structure.
- `update_entity_state_logic_6`: Decrements an entity's internal counter and uses the resulting state index to fetch data from a table.
- `update_entity_state_machine_3`: Increments an entity's internal counter and executes a branch based on a state value retrieved from a data table.
- `execute_entity_state_logic`: Processes a switch-case state machine for an entity by retrieving a state index from a data table and performing specific data updates.
- `store_zero_word`: Stores a zero word through a pointer offset by two.
- `calculate_entity_value`: Calculates a sum using values from two different data structures and a table lookup, then stores the result in multiple locations.
- `update_entity_value`: Computes a sum by adding a value from a table to a value from a data structure and stores the result in multiple locations.
- `update_entity_state_machine_4`: Increments an internal counter and executes a switch-case block based on a value retrieved from a data table.
- `execute_entity_state_logic_2`: Increments an internal counter and performs a switch-based update on an entity's properties using a value retrieved from a data table.
- `update_entity_state_logic_3`: Increments an internal timer and executes a branch based on a value retrieved from a data table.
- `update_entity_state_logic_4`: Processes an entity's state machine by incrementing a timer and executing specific data-driven actions.
- `execute_entity_state_actions`: Updates an entity's internal state index and executes specific data manipulations based on the resulting state value.
- `update_entity_state_machine_5`: Updates an entity's internal timer and processes a state machine by branching based on a value retrieved from a data table.
- `update_entity_state_logic_5`: Increments an internal counter and executes a large switch-case block based on a value retrieved from a data table.
- `decrement_timer_and_accumulate`: When a flag byte in the record at A5 is set, adds a byte value to a short accumulator and, if a companion field is zero, mirrors the sum into a second record; it then decrements a counter byte and, at zero, reloads it from a companion byte and inverts and increments a byte field.
- `update_timer_and_accumulator`: Checks a flag byte in a record; if set, it adds a byte value to a short accumulator and, if a companion field is zero, mirrors the sum into a second record.
- `update_timer_accumulator`: Checks a flag byte in a record; if set, it adds a byte value to a short accumulator and, if a companion field is zero, mirrors the sum into another field.
- `ramp_field_toward_target`: When a run flag in the record is set and its companion counter has expired, it moves a byte field toward a stored target by a step each pass.
- `update_entity_state_from_packed_data`: Unpacks a 32-bit value into components and uses a derived index to update various fields within a structure.
- `initialise_channel_record_fields`: Clears several fields of a record, computes a table pointer from a masked index, and when a bit flag is set it stores two shared values into an indexed table.
- `reset_game_state_flags`: The routine sets a global variable to one, clears another global variable, and clears a two-byte field via an uninitialized pointer.
- `store_word_with_bit_six`: Writes the input word with bit six set to a fixed memory location and returns.
- `reset_audio_channels`: The routine clears several global variables, sets a global variable to a constant, stores a value from a global variable into an entry of a table indexed by a masked global variable, and clears four additional global variables.
- `nop_padding_routine`: This routine is empty and performs no operations.
- `decode_bit_flags_into_memory`: Reads flag bits from a longword and, for each set flag, copies or expands bytes into memory through a destination pointer, with the copy count taken from a small bit field.
- `unpack_flagged_runs_body`: A longer routine in the same bit-flag decoder family as the routine before it, reading flagged bytes and expanding runs into memory.
- `unpack_bitstream_with_helpers`: Depacker routine, entered through a handler pointer in data, that calls the bit-reader refill and bit-read helpers and touches the background colour register.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:krayzi

- `copy_global_block_then_run`: Program entry that copies bytes from a global data area into a memory region up to a fixed end pointer, then makes four unnamed calls.
- `store_low_byte_and_shift`: Writes the low byte of a longword read just before an address register into a location addressed by another register, combined with the high byte of a second value.
- `shift_bits_with_refill_loop`: Shifts a longword bit buffer right once per iteration for a count taken from a register, refilling it from memory with the top bit set when it empties.

### Venus the Flytrap (1990)(Gremlin)[cr Miracle][t +6 ATZ](Disk 1 of 2)[h Krayzi].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `find_resident_library`: Locates a library in memory using a system call and executes a function from it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_copy_memory`: Clears a block of memory starting at a hardware register address and then copies a block of data from a table to a destination in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_patch_code`: Copies a block of data to a memory location and then performs a loop to overwrite subsequent code with values from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_pointer_and_return`: Loads a pointer stored at offset 22 from the structure in a0 into a0, clears d0, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block`: Copies a block of memory from a source address to a destination address using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_render_graphics`: Initializes memory buffers, copies a block of code to a new location, and performs a nested loop to calculate screen coordinates and bitmask offsets.
- `update_global_value`: Multiplies a global variable by another global variable and adds a constant to the result.
- `modify_global_value`: Multiplies a global variable by another value and adds a constant to the result.
- `clear_bit_in_tables`: Calculates an offset based on input values and clears a specific bit in two different memory tables.
- `empty_routine`: This routine is empty and contains no executable instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_constants`: Sets a global variable to a constant value and, if a specific bit is set in another global variable, sets that variable to one.
- `store_constant_value`: Stores a constant byte into a global variable.
- `initialize_runtime_environment`: Sets up system libraries, copies a block of code into memory, and performs a series of data decompression and table copying operations.
- `decrement_table_values`: Iterates through a table of words, decrementing the high and middle nibbles and the low nibble of each entry until a zero value is encountered.
- `copy_table_data`: If a bit is set in a global variable, it copies data from a table into several memory locations using two separate loops and then updates global counters.
- `update_global_status`: Checks a bit in a global variable and updates that variable and another global with specific constant values.
- `execute_infinite_loop`: Executes an empty loop that never terminates, so control never returns to the caller.
- `update_status_and_function_pointer`: Sets a global variable to a specific constant value, modifies a bit within a global variable, and stores a function pointer into another global variable.
- `update_hardware_and_status`: Sets a bit in a global variable, updates that variable and another with specific constant values, and stores a function pointer into a global variable.
- `decompress_bitstream_data`: Searches for a specific magic signature in memory before performing a bit-shifting decompression algorithm.
- `shift_bitstream_process_data`: Shifts a value left by one bit repeatedly, reloading the value from a pointer and adjusting the pointer when the value becomes zero.
- `toggle_interrupt_enable`: Modifies the interrupt enable bit in the INTENA register and updates the CIAA_PRA register.
- `save_register_context`: Stores a series of registers into a table in memory and performs conditional logic based on bits in a global variable.
- `process_hex_string_data`: Calculates ASCII characters from a value and performs a string comparison loop.
- `update_global_pointer_data`: Stores a constant value and the address of another routine into a global variable via an offset.
- `assign_routine_result`: Calls another routine and stores its return value into a global variable.
- `interleave_bit_data`: This routine performs bitwise interleaving of two data sources by masking bits and shifting them to create a single stream.
- `decrement_register_loop`: Decrements a value from two hundred fifty-five down to zero using a loop.
- `set_bit_and_loop`: Sets the second bit of a byte pointed to by a register and continues calling itself as long as the fifth bit of a second byte is set.
- `modify_byte_and_wait`: Performs bitwise operations to set and clear specific bits in a byte through a pointer and waits for a bit in another byte to clear.
- `decrement_and_loop`: Decrements a value in a loop while performing bitwise operations on a byte and adding a value to a memory location.
- `write_and_wait_for_bit`: Stores a word into two separate memory locations and then waits in a loop until a specific bit in a third memory location is cleared.
- `manipulate_byte_bits`: Executes a series of bitwise AND and OR operations on a byte through a pointer and waits for a bit in another byte to clear.
- `write_hardware_registers`: Performs bitwise operations on a byte and writes specific values to three different memory locations before waiting for a bit to clear.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
