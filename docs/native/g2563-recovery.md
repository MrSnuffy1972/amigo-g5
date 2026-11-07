# Game 2563 (odyssey (1995)(audiogenic)[cr lsd][t +6 pdy]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02563). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1810592 top-level bytes in 103 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 68 of 68 named (0 author symbols), 68 of 68 described (6 by rule, 59 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1605 of 1605 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 48818 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2634 of 2745 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1352485 |
| code | 9866 |
| data | 349141 |
| padding | 25372 |
| unused | 73728 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 1 of 2).adf:TRAINER!.EXE | 2 | 267 | 8 |
| Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 1 of 2).adf:cracktro.exe | 2 | 194 | 7 |
| Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 1 of 2).adf:Odyssey | 2 | 267 | 8 |
| Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 1 of 2).adf:CJM1004 | 2 | 28 | 2 |
| Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 658 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 815 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 506 | 20 |
| Map1.CMP (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 421 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-x25k): 337 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 400 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- odyssey (1995)(audiogenic)[cr lsd][t +6 pdy](disk 2 of 2)/Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 2 of 2).adf:FinalMaps/Demo/Map1.CMP: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- odyssey (1995)(audiogenic)[cr lsd][t +6 pdy](disk 2 of 2)/Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- odyssey (1995)(audiogenic)[cr lsd][t +6 pdy](disk 1 of 2)/Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 1 of 2).adf:cracktro.exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- odyssey (1995)(audiogenic)[cr lsd][t +6 pdy](disk 1 of 2)/Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 1 of 2).adf:Odyssey: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- odyssey (1995)(audiogenic)[cr lsd][t +6 pdy](disk 1 of 2)/Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 1 of 2).adf:CJM1004: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 48818 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 1 of 2).adf:TRAINER!.EXE
- Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 1 of 2).adf:cracktro.exe

## Copy protection found

- `initialize_system_interrupts_and_tables` (runtime segment an address): Registers an interrupt server, clears several memory buffers, and populates lookup tables. Kind: disk hardware/track check. Pass path: the loop continues or exits normally after the DAT_00bfe001 check, allowing the code to proceed to the next initialization stage.

## Functions


### Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 1 of 2).adf:TRAINER!.EXE

- `parse_amigados_hunk_relocation`: Iterates through a memory block to identify and process AmigaDOS hunk type codes, performing memory relocation and updating pointer tables or block sizes.
- `decrement_register_to_zero`: Decrements a register until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_memory_block`: Uses a count and a pointer to copy or move bytes within a memory range, potentially involving conditional logic or clearing certain bytes.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_streams`: Iteratively processes a value by checking and shifting bits, using a pointer to read or write data based on certain bit patterns.
- `execute_trap_instruction`: Executes a trap instruction and stores several bytes from certain registers into a memory location relative to a register.

### Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 1 of 2).adf:cracktro.exe

- `execute_program_entry`: Executes a routine called via a pointer, and if a certain value is non-zero, updates a pointer and a memory location with a value from a source address.
- `do_nothing`: The routine is empty and performs no operations before returning.
- `expand_memory_list`: Iterates through a linked list of memory blocks to check for available space, extending or updating block pointers and copying or allocating new memory blocks if a certain size or boundary is exceeded.
- `empty_routine`: This routine is empty and performs no operations.
- `allocate_and_copy_memory_list`: Allocates a block of memory and copies a range of bytes backwards or forwards, then traverses a list of pointers to update or extend memory-resident data structures.
- `update_memory_from_pointer`: Calculates an offset, adds a register value to a memory location, and iterates through a list of pointers, potentially calling other routines or executing a delay loop.
- `spin_countdown_delay`: Runs an empty countdown loop from a value passed in a register and returns.

### Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 1 of 2).adf:Odyssey

- `parse_amigadp_hunk_relocation`: Parses a list of executable hunk types and performs memory relocation or block copying.
- `decrement_loop_counter`: Decrements a register or a global variable and loops until it reaches zero.
- `do_nothing_routine`: the routine is empty and performs no operations before returning.
- `expand_bit_stream`: Iteratively expands or copies data from a source pointer to a destination pointer based on bit-pattern-driven logic and a counter.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `process_bit_stream`: Performs complex bitwise shifts and logical operations while iterating through a memory buffer.
- `extract_bit_stream`: Processes a bit stream by reading a pointer, checking bit patterns, and updating a bit-shifted value in a register or a global variable.
- `update_memory_and_trap`: Executes a trap instruction, writes register values to a memory offset, and modifies a global byte.

### Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 1 of 2).adf:CJM1004

- `update_pointer_and_value`: Checks a function pointer and, if the return value is non-zero, updates a global pointer and a memory location with a value from a register.
- `nop_routine`: An empty routine that performs no operations and returns immediately.

### Odyssey (1995)(Audiogenic)[cr LSD][t +6 PDY](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_interrupts_and_tables`: Registers an interrupt server, clears several memory buffers, and populates lookup tables.
- `update_object_kinematics`: Updates a table of motion parameters by adding or subtracting a value from various components of each entry based on a state machine.
- `clear_blitter_data_table`: Waits for the blitter to be idle and then zeroes out eight 28-byte blocks in a table.
- `update_data_table`: Searches a table for a matching byte, then copies six 28-byte blocks from a source table to a destination table based on an index.
- `copy_table_entry`: Searches a table for a matching byte and copies six 28-byte blocks from a source table to a fixed memory location.
- `transform_coordinate_table`: Iterates through a table of coordinate pairs and performs a linear transformation on each pair using a multiplication and addition algorithm, storing the results in a second table.
- `reset_blitter_masks_and_modulo`: Waits until the blitter is idle, then writes full first and last mask words, a control word and two modulo values for the blitter.
- `fill_offset_table`: Fills 256 consecutive words with offsets that increase by forty for each row, forming a lookup table of row start positions.
- `build_row_offset_table`: Populates a table of two hundred fifty-six words where each entry increments the previous value by forty.
- `draw_line_with_blitter`: Orders two endpoints, computes the step counts and octant of a line, toggles one pixel bit at the start position, and programs the blitter with the modulo, control and size values to draw the line.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_and_input_state`: Sets up audio volume registers and initializes a table of pointers and values.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `increment_and_conditional_audio_update`: Increments a global byte and compares a value from a local table against a threshold.
- `update_audio_channel_parameters_2`: Parses a command byte to modify audio channel properties such as pitch, period, and volume.
- `update_audio_channel_parameters`: Parses a command byte to update audio channel properties including pitch, period, and volume.
- `update_parameter_state`: Updates a control structure by modifying a counter value based on a command byte.
- `search_parameter_table`: Iterates through a table of values to find an entry that satisfies a comparison against a masked value.
- `update_counter_and_lookup`: Modifies a value in a data structure by adding or subtracting a byte based on a flag, then performs a table lookup using a calculated index.
- `calculate_scaled_offset`: Extracts a value from a data structure using bitmasking and performs a lookup in a table or applies a multiplier.
- `process_control_command_byte`: Parses a command byte from a data structure to perform various state updates, pointer arithmetic, and value scaling.
- `store_masked_byte`: Extracts a four-bit value from a byte in a data structure and stores it into another location within the same structure.
- `update_pointer_based_on_flag`: The routine checks a high bit in a structure field; if set, it clears the field, adjusts a pointer based on a table lookup and offset, and negates the byte at the new pointer location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_subsystem_pointers`: Calls a library function to obtain a pointer, configures several global pointers and constants, and enters a loop that increments a pointer through a table until a zero-value is encountered.
- `update_global_pointer`: Decrements a global variable and resets it to a fixed value if it reaches zero or fails a bit-check, then updates a pointer to a new memory location.
- `poll_button_state`: Loops while a specific bit in a global variable is clear, then calls another routine and loops while that bit is set.
- `execute_empty_sequence`: The routine is empty and performs no operations.
- `wait_for_global_pattern`: Polls a global variable until a specific bitmask or pattern is detected.
- `tiny_ori`: Small routine of 1 instruction that merges 64 into d0.
- `initialize_global_values`: Stores or calculates several values and writes them into a series of global variables.
- `select_global_value`: Compares two global variables and updates one with the value of another based on that comparison.
- `update_blt_parameters`: Calculates and writes a new set of bitplane-based or blitter-related parameters, including modified register values, to a series of global memory locations and bitplane/blitter-related registers.
- `calculate_register_params`: Calculates and stores several calculated values into a block of RAM, including a value derived from a routine address and a value derived from a subtraction.
- `set_blt_registers`: Waits for a specific bit in a certain memory location to clear, then writes several constant or fixed values into a set of bitplane and DMA registers.
- `calculate_blt_parameters`: Waits for a bit in a memory location to clear, then calculates a value using a table and a register, and stores the result and other values into a set of registers.
- `sync_parameter_tables`: Updates a series of global variables with new values and clamps a value from a pointer-based table between zero and two hundred while iterating through a list.
- `calculate_register_params_alt`: Calculates and stores certain values into a block of RAM, using a different constant for a certain memory write compared to another routine.
- `update_blt_and_data`: Checks a value, potentially increments a global variable, and if a condition is met, waits for a bit or register to clear before writing a series of values to memory and registers.
- `set_blt_bounds`: Waits for a bit in a memory location to clear, then writes a sum of two certain values and several constants into a set of registers.
- `process_instruction_stream`: Iterates through a stream of bytes, updating a pointer and certain memory locations based on the value of the byte read, or resetting a pointer to a fixed address.
- `update_register_value`: The routine performs a single operation that modifies a register or memory location using an absolute comparison or arithmetic instruction.
- `tiny_ori_2`: Small routine of 1 instruction that merges 200 into (a3)+.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d0.
