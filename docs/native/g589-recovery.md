# Game 589 (Elite v2.0 1572): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00589). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2435901 top-level bytes in 17 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 18 of 18 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 336 of 336 named (0 author symbols), 336 of 336 described (5 by rule, 331 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1838 of 1838 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 69120 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7624 of 7624 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 458360 |
| code | 25740 |
| data | 1951689 |
| padding | 112 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 93 | 10 |
| logged disk load from Disk.1 | 1 | 7316 | 301 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 210 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 4 |
| Disk.2 bytes 61952-63744 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.2 bytes 67584-134912 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Elite_v2.0_1572/Elite_v2.0_1572_hdf/Elite/Disk.1: raw disk image (444928 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Elite_v2.0_1572/Elite_v2.0_1572_hdf/Elite/Disk.2: raw disk image (444928 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Elite_v2.0_1572/Elite_v2.0_1572.hdf: hard-disk image (AmigaDOS OFS, 2624 blocks): its filesystem parses and all 12 files in it are byte-identical to the extracted tree Elite_v2.0_1572_hdf, which is recovered file by file; no block outside the filesystem holds data
- 12 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Elite_v2.0_1572/Elite_v2.0_1572_hdf/Elite/Disk.2: bytes 61952-63744 look like code no run loaded: recovered position-independently (load address unknown)
- Elite_v2.0_1572/Elite_v2.0_1572_hdf/Elite/Disk.2: bytes 67584-134912 look like code no run loaded: recovered position-independently (load address unknown)
- Elite_v2.0_1572/Elite_v2.0_1572_hdf/Elite/Disk.1: 11 logged loads cover 428032 bytes of the image
- Elite_v2.0_1572/Elite_v2.0_1572_hdf/Elite/Disk.1: chunks 0-43 (bytes 0-11264) outside the filesystem hold data, not code (opcode density 0.010, entropy 1.21)
- Elite_v2.0_1572/Elite_v2.0_1572_hdf/Elite/Disk.1: chunks 1716-1737 (bytes 439296-444928) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.17)
- Elite_v2.0_1572/Elite_v2.0_1572_hdf/Elite/Disk.2: 3 logged loads cover 131072 bytes of the image
- Elite_v2.0_1572/Elite_v2.0_1572_hdf/Elite/Disk.2: chunks 249-263 (bytes 63744-67584) outside the filesystem hold data, not code (opcode density 0.000, entropy 7.07)
- Elite_v2.0_1572/Elite_v2.0_1572_hdf/Elite/Disk.2: chunks 527-1737 (bytes 134912-444928) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.78)
- 2 never-loaded region(s), 69120 bytes, recorded as UNVERIFIED DATA (0 bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `copy_ciab_buffer_and_process_input` (Disk.1 +an address loaded at an address (95744 bytes)): The routine copies a 256-byte block from an input area to a buffer, decrypts it, copies it back, stores a checksum byte, waits for button release, sets a flag in a global variable, waits for a target value and for input, disables a joystick button, runs a game loop, and then resets combat or disk menu state depending on a flag. Kind: not judged: Extra data: line 1 column 65 (char 64). Pass path: not determined.
- `read_disk_label` (Disk.1 +an address loaded at an address (95744 bytes)): The routine waits for input and repeatedly reads a byte from disk until a checksum matches, then compares a disk label string byte-by-byte against a reference string. Kind: not judged: Extra data: line 1 column 84 (char 83). Pass path: not determined.
- `initialize_disk_boot` (Disk.1 +an address loaded at an address (95744 bytes)): The routine runs a disk boot sequence. Kind: not judged: Extra data: line 1 column 53 (char 52). Pass path: not determined.
- `initialize_disk_dma` (Disk.1 +an address loaded at an address (95744 bytes)): Fills a table with a constant pattern and derives two words from input bytes, writing them to memory and a global variable; it then performs a patterned xor reduction over a buffer, updates a global variable and spins on hardware status bits before calling another routine and waiting for a hardware flag. Kind: not judged: Extra data: line 1 column 51 (char 50). Pass path: not determined.
- `wait_for_joystick_button` (Disk.1 +an address loaded at an address (95744 bytes)): The routine masks a global variable, sets a bit in it, increments a short through an address derived from an unused register, calls another routine to reset an accumulator, then spins until a bit in the global variable is cleared. Kind: not judged: Extra data: line 1 column 51 (char 50). Pass path: not determined.
- `decrement_counter_and_wait_input` (Disk.1 +an address loaded at an address (95744 bytes)): The routine masks a global variable, decrements a short through an address in a register, calls another routine, then spins until a bit in the global variable clears. Kind: not judged: Extra data: line 1 column 51 (char 50). Pass path: not determined.
- `wait_for_button_release` (Disk.1 +an address loaded at an address (95744 bytes)): Clears a global variable mask and waits for a CIA timer. Kind: not judged: Extra data: line 1 column 70 (char 69). Pass path: not determined.
- `disable_joystick_button` (Disk.1 +an address loaded at an address (95744 bytes)): The routine modifies a global variable by clearing one bit and setting another bit, then returns. Kind: not judged: Extra data: line 1 column 51 (char 50). Pass path: not determined.
- `init_disk_dma_transfer` (Disk.1 +an address loaded at an address (95744 bytes)): The routine sets up a global variable and a pointer to a table, writes values to a global variable, then polls a flag with a decrementing counter. Kind: not judged: Extra data: line 1 column 74 (char 73). Pass path: not determined.
- `trigger_novella_protection_violation` (Disk.1 +an address loaded at an address (95744 bytes)): The routine checks two words from a global variable and, if they differ, clears palette entries, resets game state, parses an input line and reads the input buffer, then enters an infinite loop. Kind: not judged: Extra data: line 1 column 65 (char 64). Pass path: not determined.
- `decrement_joystick_counter` (runtime segment an address): The routine masks a global variable with a constant and sets low bits, decrements a short value through an address derived from an uninitialized register, calls another routine, then loops until a bit in the global variable is clear. Kind: not judged: Extra data: line 1 column 51 (char 50). Pass path: not determined.
- `disable_joystick_input` (runtime segment an address): The routine clears a bit in a global variable, waits for a CIA timer, then repeatedly calls another routine while a bit in the global variable is set, writes zero to a memory location via an uninitialized pointer, and then loops until another bit in the global variable clears. Kind: not judged: Extra data: line 1 column 53 (char 52). Pass path: not determined.
- `read_joystick_input` (runtime segment an address): The routine reads the CIA B port B register, masks a global variable and returns a value derived from the port data and an intermediate value. Kind: not judged: Extra data: line 1 column 51 (char 50). Pass path: not determined.
- `init_disk_dma_transfer_2` (runtime segment an address): A routine called by wait_for_keypress_3 that sets up disk DMA registers, clears a request flag, runs a polling loop and a small data stir loop, and tails to wait_for_input_and_stir_buffer. Kind: not judged: Extra data: line 1 column 51 (char 50). Pass path: not determined.

## Functions


### game.slave

- `return_negative_one`: The routine returns the constant value -1.
- `run_game_loop`: Executes several callbacks from a structure passed in A0, then enters an infinite loop that shows the mission briefing screen and repeatedly updates a counter in the structure while clearing fields and calling support routines until a loop condition is met.
- `verify_routine_entry`: The routine contains nine instructions and performs no observable work; it ends with a return.
- `store_word_to_global`: The routine copies the word value from data register D3 into a global variable.
- `return_immediately`: The routine contains six instructions and ends with a return, so it performs no observable work and returns to its caller immediately.
- `initiate_hyperspace_jump`: The routine contains three 68000 instructions and then returns.
- `wait_for_vblank_sync`: The routine calls another routine to reset video hardware, then checks a global variable and a memory location.
- `set_game_flag`: The routine writes the constant 1 to a global variable and then returns.
- `set_global_state`: The routine writes the constant 2 into a global variable and then returns.
- `call_game_hook`: The routine calls another routine through a pointer stored in a global variable and returns the combined value of the two input registers.

### logged disk load from Disk.1

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `call_indirect_through_table`: The routine calls a function pointer taken from a table and returns the combined value of two input registers.
- `update_mission_timer_and_bitmask`: Updates a mission timer or, if certain conditions are met, performs a bitwise masking or conditional bitwise replacement (bitwise AND/OR or bitwise mux) on a block of memory using a lookup table.
- `set_combat_target`: The routine writes constants and copies a value from a global variable into another global variable, then stores a pointer to a local field and writes a constant to a global variable.
- `store_target_vector`: The routine stores a pointer and a word from its inputs into a global variable and a table, copies a value from a global variable into another field of the table, writes a constant to the global variable, and stores the address of a following field as a pointer in the table.
- `set_target_position`: The routine stores a short from a register into a global variable and a ushort from a register into another global variable.
- `read_input_buffer`: The routine loops while a global flag is set, repeatedly calling another routine via a table and updating a pointer.
- `parse_input_line`: The routine scans a null-terminated input string to its end, calls another routine, then loops while a global flag is set, dispatching through a table and updating a pointer.
- `wait_for_input_and_process`: The routine skips null-terminated strings and then loops, repeatedly calling another routine while a global flag is non-zero, updating a pointer each time.
- `skip_null_terminated_strings`: The routine receives a pointer and a word count in registers and advances the pointer past a null-terminated string, repeating the skip for the given count.
- `set_target_coordinates`: The routine stores two short values from registers into two consecutive short fields of a structure pointed to by a register, and writes two pointers into two consecutive pointer fields of the same structure.
- `format_number_string`: The routine builds a small digit buffer on the stack by repeatedly calling another routine until a zero result is returned, then copies the digits to a destination buffer pointed to by a register, inserting a decimal point before the last digit when a flag is set and optionally prefixing a sign character from a global variable.
- `reset_game_state`: The routine calls another routine to format a number string and then calls another routine to read an input buffer, then returns.
- `wait_for_key_release`: The routine clears a global variable and then spins until that variable becomes non-zero, repeating the wait a fixed number of times.
- `copy_string_to_buffer`: The routine copies bytes from a source pointer in A0 to a destination pointer in A1 until a zero byte is encountered, advancing both pointers each iteration.
- `count_string_length`: The routine receives a pointer in A0 and walks forward byte by byte, incrementing a counter until it reads a zero byte.
- `copy_string_to_buffer_2`: The routine copies a null-terminated string from a source pointer to a destination pointer.
- `compare_strings`: The routine receives two pointers in registers and walks them in lockstep, comparing the byte pointed to by each.
- `wait_for_player_input`: The routine increments a message counter, retrieves ship data, saves game state and resets combat state, then loops reading a message character and optionally running a deferred callback until a non-zero character is obtained.
- `run_deferred_callback`: The routine checks a global flag; if set it clears the flag and then calls a function pointer stored in a global variable, returning its result.
- `return_key_code`: The routine returns a constant value.
- `update_comms_buffer`: The routine updates a circular buffer in a global variable.
- `read_next_message_char`: The routine returns a 16-bit value built from a high byte taken from a global variable and a low byte taken from a table indexed by another global index.
- `clear_targeting_flags`: The routine reads characters from a message stream until a zero terminator is encountered, then clears three 16-bit fields in a global structure by writing zero to each.
- `wait_for_input_or_timeout`: The routine advances a ship list index and clears a combat target, then loops reading the next message character and checking a global flag.
- `wait_for_message_char`: The routine repeatedly reads a character from the message stream until a non-zero character is obtained or a global flag is cleared, then clears the flag and resets combat state before returning the character value combined with the stack contents.
- `set_target_ship`: The routine stores target coordinates in a global structure and updates a combat target buffer using a message counter and input handling.
- `process_input_buffer`: The routine loops waiting for player input and processes characters in a buffer.
- `parse_target_system_id`: The routine increments a message counter, reads an input buffer and resets combat state, then calls a routine to set a target ship and extracts its return value.
- `return_error_code`: The routine returns a constant value with all bits set.
- `set_target_system`: The routine updates a global variable with a computed offset, stores values returned by another routine into globals, writes constant bytes to globals, runs game-state update and combat resets, then loops until a global flag is set, returns a value from a global, and performs additional counters and buffer copies.
- `reset_combat_target`: The routine clears several fields in a global structure and writes a value from a register into one field, then sets a few word fields to constants and writes a byte value to a byte field.
- `increment_message_counter`: The routine reads a short counter from a global variable, stores the current two-byte message length and a four-byte message pointer from two global variables into a table indexed by that counter, increments the counter by six, clears the message length variable, and calls another routine.
- `increment_ship_list_index`: The routine reads a short index from a global variable, stores a word from a global variable into a table entry indexed by that short, stores a long from another global variable into a table entry indexed by that short, then increments the short index by six.
- `reset_combat_state`: The routine clears several global variables, decrements a global index, loads a 32-bit value from a table indexed by that index into a global variable, loads a 16-bit value from a table indexed by that index into a temporary and stores it into a global variable, and if the loaded 16-bit value is zero it calls another routine.
- `select_target_ship`: Checks a global flag and a zero flag, then walks a table of short entries nine elements at a time, testing a pair of coordinates against stored ranges.
- `select_target_system`: If a global flag is set and another flag is clear, it iterates a table of short entries and, when the entry lies within computed bounds, copies fields from the entry into global variables and sets a flag.
- `reset_game_state_2`: The routine clears several memory fields, conditionally clears palette entries, resets collision flags, decodes a crypt message, optionally loads ship data and saves the game state, toggles a palette, waits for a flag, sets palette entries, and initializes a few more fields and a pointer to a global variable.
- `reset_game_state_3`: The routine resets several global variables, updates a message counter and a weapon pointer, conditionally decrypts game data and reloads ship data, then copies a starfield buffer if a short value is zero.
- `get_ship_data`: The routine reads a short from a global variable using an index derived from the input register, then reads a second short from a fixed offset in the same table and returns them combined as a 32-bit value.
- `run_game_loop_2`: The routine increments a message counter, retrieves ship data, saves the game state, reads the input buffer, resets the combat state, and returns a value formed from two input registers.
- `process_text_input`: The routine increments a message counter, checks activity, stores ship data and coordinates, then parses a text buffer word by word using a table-driven indirect call, updating target position and input buffers.
- `reset_game_state_4`: The routine clears a set of flags via a table of offsets, resets several global variables and a small fixed-point value, zeroes a block of ship-damage related entries, and then waits until a particular global counter becomes non-zero.
- `shift_register_checksum_update`: The routine reads three bytes from a three-byte shift register in a global variable, rotates the register left by one byte, and computes a new feedback byte from the previous top byte and the incoming byte using a left shift, a sign test, and a combination of the low bit of the previous byte and the high bit of the incoming byte.
- `scale_fixed_point_value`: The routine obtains two 16-bit values from another routine, combines them into a 32-bit word, multiplies the result by an incoming 16-bit value in D2, then forms a 32-bit fixed-point result by shifting the product and recombining the high and low parts.
- `decode_compressed_data`: The routine repeatedly updates a set of local variables using arithmetic and bit operations on an input value and internal state, forming a small fixed-iteration loop that transforms the input through shifts, masks and conditional selections.
- `reset_game_state_5`: The routine calls another routine and then returns.
- `rotate_weapon_offsets`: The routine reads two short values from a structure pointed to by a register, swaps the first two fields, then updates the third field by adding the original two values to its current contents.
- `compute_checksum`: The routine implements a 32-bit CRC-like checksum over a 32-bit value in D0 using a 32-bit accumulator and a carry flag.
- `generate_random_number`: The routine implements a linear congruential generator using a global variable as seed.
- `insert_commodity_into_trade_list`: The routine clears several bytes in a global structure, checks a flag and a set of conditions involving a second structure, then calls another routine through a function pointer and another two routines.
- `process_trade_list`: The routine checks a global flag.
- `process_player_state`: The routine exits early for paused games by looping a bounded counter while rotating a 4-bit counter and invoking two display helpers.
- `process_message_queue`: The routine repeatedly reads a short value from a pointer in A4 and advances the pointer by one short each iteration.
- `update_starfield_positions`: The routine iterates over a table of star entries and updates each entry’s screen coordinates by transforming a vector with a matrix and scaling the result.
- `update_targeting_lock`: The routine updates per-target lock flags in a table, marks a target as being processed, and for each active target it sets a flag, transforms a vector, and computes scaled screen coordinates using division and remainder, storing the results back to the target entry; it aborts early on negative indices or when clipping or range checks fail.
- `check_targeting_range`: The routine reads two short values from a structure pointed to by a register, computes differences with a base value, and tests the results against signed 8-bit and 32-bit bounds using a compound condition.
- `exit_program`: The routine contains no instructions and performs no operations; it is an empty function that returns immediately.
- `negate_input_value`: The routine receives a 16-bit value in D0, negates it and returns the result in D0.
- `scale_value_to_range`: The routine takes an input value in a register, left shifts it by nine bits, then divides the shifted value by a 16-bit divisor taken from a global variable and obtains the remainder.
- `transform_vector_by_matrix`: The routine reads a 3x2 matrix of 16-bit values from a global variable and a 3-component vector from a pointer, computes two dot products of the vector with the first two rows of the matrix using 32-bit arithmetic, and returns the two results packed into a 64-bit value with a fixed-point scaling applied to each.
- `check_missile_lock`: The routine scales three values and stores them into a structure, then checks a global flag and a bit in a byte.
- `compute_object_orientation`: The routine checks a flag byte via an address in A5 and, if the flag bit is clear, sets the bit and writes three 16-bit values computed from six 16-bit fields in the same structure using cross products scaled and shifted.
- `scale_screen_coords`: The routine calls another routine and then divides three 16-bit values pointed to by a pointer by an uninitialized word, after scaling each value with a fixed shift.
- `reset_orientation_accumulator`: The routine checks a global variable and, if a threshold is exceeded, clears two bytes of a structure, calls another routine twice, clears a flag bit in a byte, calls another routine, and computes three short values from pairs of shorts in the structure using multiply-subtract and shift operations, storing the results back into the structure.
- `calculate_ship_damage`: The routine calls another routine to transform a vector by a matrix, then calls a function pointer taken from a table to obtain a short value.
- `decrement_timer_and_process_events`: The routine decrements a short global variable if it is non-zero, then calls another routine to set a weapon pointer.
- `update_asteroid_velocity`: The routine reads a set of short values from a structure pointed to by a5, computes two fixed-point ratios from the first three fields and a derived field, passes the results to a clipping routine twice, then updates four velocity fields by scaling them with a constant and decrements a counter field.
- `clear_ship_data`: The routine clears a block of word-sized entries in a global structure by writing zeros to a fixed number of consecutive words, then clears a block of byte-sized entries in the same structure by writing zeros to a fixed number of consecutive bytes, and finally writes zero to two separate word fields in the structure.
- `check_fuel_status`: The routine uses a pointer derived from the address register plus an offset to index a table of bytes.
- `update_station_docking_state`: Updates a station docking record by copying bytes from a table, refreshing timers and flags, and incrementing a counter in a global table.
- `decrement_service_flags`: The routine iterates over a table of bytes using a fixed offset and a counter.
- `decrement_station_service`: The routine clears bits in a byte pointed to by a register, then if a short value is below a threshold it decrements a byte in a table indexed by that short, calls another routine, and conditionally decrements two different short globals based on another short flag, and clears a short global when the index equals a specific value.
- `decrement_target_counter`: The routine checks a short value in D0 and, if it equals one, decrements a short stored at an offset from a pointer in A6.
- `decrement_target_counter_2`: The routine checks two short values from a structure pointed to by a register; if both are non-zero it checks a short value from another structure pointed to by a different register.
- `reset_game_timer`: The routine saves a global variable, writes a constant to it, calls another routine three times inserting a colon byte before each call, then restores the saved value.
- `multiply_signed_16x16`: The routine treats the low word of the first argument as a signed 16-bit value and the second argument as a signed 16-bit value, takes absolute values of both, multiplies them using 32-bit arithmetic, and then applies a sign based on the xor of the high word of the first argument with the second argument.
- `compute_abs_product`: The routine takes two 32-bit values from registers, converts each to its absolute value, then returns the product of the high 16 bits of the first value and the low 16 bits of the second value.
- `empty_routine`: The routine contains a series of 12 instructions that perform no observable operation on memory or registers and then returns.
- `store_ship_parameters`: The routine calls another routine and then stores two values from registers into two consecutive words of a structure pointed to by a register.
- `store_target_coordinates`: The routine calls another routine and then stores two values from registers into two consecutive words of a structure pointed to by a register.
- `update_weapon_targeting`: The routine calls another routine and stores two values from registers into a global variable.
- `update_computer_target`: The routine calls another routine, stores two values from registers into a global variable, calls compute_object_orientation, clears a bit in a byte pointed to by a pointer, performs six signed 16x16 multiplications in three pairs, stores the high byte of each difference into three consecutive short fields of the pointed-to structure, and increments a short counter in that structure.
- `reset_game_state_6`: The routine calls another routine four times, then stores the value from a data register doubled into a memory location pointed to by a pointer, and stores zero into a memory location pointed to by another pointer.
- `reset_state_and_update_velocity`: The routine calls another routine to reset game state, then if a flag in a global variable is clear it clears the flag and performs eight signed 16 by 16 multiplications, combines the results with addition and subtraction, shifts the results right by eight bits and stores them as 16-bit values into two pairs of offsets in the same global structure, increments a 16-bit counter in that structure, and returns.
- `reset_and_update_targeting_state`: The routine resets a game state via another routine and, if a flag in a global variable is clear, clears that flag, performs eight signed 16x16 multiplications, stores two derived 16-bit results into two pairs of offsets in the same global structure, increments a 16-bit counter in that structure, and returns.
- `initialize_briefing_state`: The routine resets several game state fields and loads mission data into a table, then runs a loop that clears input, updates ship orientation, commodity prices, trade list and random system flags, updates the mission timer and target angles, and adjusts a scroll offset until a condition on two status words is met, while also updating weapon targeting and computer target.
- `adjust_target_angle`: The routine takes a short from D0 and a short from D1.
- `set_up_target_ship`: This routine clears ship data and initializes a ship structure pointed to by an address register, copies a fixed number of bytes from a global variable into the structure, then repeatedly updates combat targeting for a decreasing counter, updating station docking state and a target pointer each iteration, and waits for key release if the loop exhausts.
- `update_combat_targeting`: This routine loops a fixed number of times, clearing an input buffer, scanning a table of entries and updating weapon targeting and commodity data, then processes trade and communication state and reads messages.
- `copy_palette_block`: The routine copies eight 16-bit words from a global variable to a block pointed to by an argument register plus an offset, then stores a constant 16-bit value at an offset from the same argument base.
- `update_display_mode`: The routine resets docked state, installs exit_program as two function pointers, conditionally resets game state and writes a flag, then updates ship data and a set of flags when a stored mode changes, clears damage entries, selects a display mode by testing bits in global tables, and writes mode-dependent values to a global structure.
- `update_display_mode_logic`: Updates a pointer to a function and a global variable, resets certain flags, and clears a range of memory in a table.
- `reset_game_state_7`: The routine clears palette entries, collision flags and the screen buffer, zeroes several global variables, decodes a crypt message, then copies 8000 words from a table derived by XORing two global values into a destination table pointed to by a global pointer, and finally sets palette entries and clears a global flag.
- `add_input_to_value`: The routine reads an input event when a value in D3 is non-zero, adds the D3 value to the input D0 value, and if D3 is not equal to the ASCII digit '0', it overwrites the sum with another input event.
- `compute_target_offset`: The routine reads a short from a global variable, scales it and adds a constant, combines it with the high word of the input value, and then optionally replaces the result with the value returned by another routine.
- `compute_target_offset_2`: The routine reads a short from a global variable, scales it and adds a constant to form a low word.
- `calculate_target_offset`: The routine reads a short from a global variable, scales it and adds a constant to form a low word, combines it with the high word of the input value, and then optionally overwrites the result with the return value of another routine.
- `get_ship_name`: The routine reads a short from a global variable, combines it with the high word of an input value and an offset, and conditionally calls another routine to obtain an input event, returning the resulting value.
- `compute_target_distance`: The routine reads a short from a global variable, scales it, and combines it with the high word of the input value to form a 32-bit result.
- `get_ship_name_index`: The routine reads a short from a global variable, adds a constant to it, combines the result with the high word of an input value, and if the short is non-zero it calls another routine to obtain input and clears the high word.
- `set_scanline_palette`: The routine writes two short values to a global variable and then loops four times.
- `set_weapon_range_and_input`: The routine reads a 16-bit value from a global variable, clamps it to a range, stores a derived 16-bit flag into another global variable, computes a scaled short value from the clamped input, combines it with the high word of an input register and a constant, and returns the combined value after optionally calling another routine to fetch input events.
- `set_ship_speed`: The routine writes two 16-bit constants to a global variable, computes a scaled value from a global variable and stores the result in another global variable, then calls another routine repeatedly based on the stored value and a comparison against a constant.
- `set_ship_speed_2`: The routine writes two 16-bit constants to a global variable and computes a 16-bit value from another global variable using multiplication and division, storing the result back to a global variable.
- `draw_hud_digit`: The routine reads a value from a global variable, converts it to a scaled index, then loops three times to derive digit indices and updates a global variable by adding a constant offset each iteration.
- `check_game_over_state`: The routine tests a global variable.
- `check_docking_status`: The routine tests a global variable and branches.
- `initiate_combat_sequence`: The routine prepares a combat state by calling another routine to obtain ship data and then writes several fields in a global structure, copies a starfield buffer, checks targeting constraints and conditionally decodes a galactic map or draws a scanline sprite.
- `clear_screen_buffer`: The routine clears a block of memory by writing zero to each 16-bit word.
- `set_random_system_flags`: The routine reads two integers from a global variable and compares them to produce a bit index.
- `empty_routine_2`: The routine contains four 68000 instructions and performs no observable work; it does not access memory, registers or call other code and ends with a return.
- `reset_docked_state`: The routine checks a flag stored in a global variable.
- `update_targeting_state`: The routine toggles bits in two byte pairs using a mask derived from an input register, then either stores the input value at an address derived from a pointer and returns a combined value, or updates a small fixed-size table in a global structure and writes a byte from the input register into that table, marking entries for two specific input values and storing an index in a global variable.
- `set_input_buffer`: The routine copies a byte from a register into a byte in a global buffer and, if the byte is a control character, marks the corresponding entry in a global table and records the index in a global variable.
- `clear_screen_buffer_2`: The routine clears a small byte buffer in the caller’s structure by writing a constant byte value to a few consecutive bytes, then fills a table of 19 words starting from a base value by repeatedly adding the current table entry to the base and storing the result back into the table.
- `store_input_byte`: The routine writes a byte from a register into an input buffer and updates related flags.
- `clear_ship_flags`: The routine stores zero into a byte at an offset from a base pointer and zero into a word at the next offset, then returns.
- `check_asteroid_field`: The routine reads a byte from a global variable and, if the byte is non-zero, it calls another routine.
- `process_combat_targets`: The routine calls a zero-return helper then loops three times, using a global variable and a table entry to decide whether to call a combat target routine or start an audio channel.
- `start_audio_channel`: The routine copies a set of bytes from a table entry indexed by the value in D0 to a per-channel structure pointed to by D1, sets up audio hardware registers AUD0LCH and AUD0LEN via a base address taken from the channel data, writes DMACON to enable DMA, and updates a few control flags in the channel structure.
- `process_combat_target`: Clears a bit in a global variable and updates per-target fields in a table indexed by an input register, handling counters and flags and writing back a saved word to a global variable.
- `reset_audio_channel`: The routine clears the low channel register for audio channel 0 and writes zero to a second audio register derived from a byte in a structure pointed to by A1.
- `configure_audio_channels`: The routine writes values from a table and a global variable to the four audio period and volume registers, and updates a global variable with a masked and inverted value.
- `reset_audio_channels_wrapper`: The routine calls another routine to reset audio channels and then returns.
- `return_zero_constant`: The routine is empty and returns zero.
- `reset_audio_channels`: The routine clears four global variables and writes zero to four memory locations that correspond to the AUD0VOL hardware register.
- `jump_to_reset_game_state_10`: Jump stub: passes control straight on to reset_game_state_10.
- `process_station_docking`: The routine reads a value via another routine and uses it to index a global table, then conditionally resets game state and updates audio.
- `update_audio_channel`: The routine updates a per-channel audio pointer by indexing a table via an offset in a byte field of a structure, increments that byte and wraps it when a sentinel value is seen, then writes the new pointer back to a global variable and a secondary global variable.
- `clear_game_state_flags`: The routine clears a set of memory locations to zero.
- `reset_game_state_8`: The routine calls another routine and then clears a global variable twice.
- `copy_ship_data`: The routine copies a small fixed-size block from a global variable table into a destination table using a bitmask from a global variable, then writes a terminator byte and clears a byte at an offset in the input structure.
- `clear_target_and_zero_counter`: The routine clears a global variable, copies a short value from an offset within that variable to another offset, then decrements a 16-bit value from the global variable to zero using a loop.
- `copy_commodity_fields`: The routine reads a byte from an input structure and uses it as an index into a global table to obtain a pointer, then copies several word fields from a table entry into fixed offsets of the input structure.
- `set_audio_channel`: The routine reads a byte from the argument block, uses it as an index into a table to fetch a sample pointer and length, writes the pointer into the block, selects a length value depending on a flag byte, and writes the length to the audio channel length register and the sample pointer to the audio channel pointer register.
- `copy_ship_data_2`: The routine reads a 16-bit value from a global variable using an index derived from the high byte of a 32-bit input and a byte from an input pointer, then writes that value twice to the same location in a global variable.
- `write_audio_dma_control`: The routine writes four 16-bit values to the four audio period registers and then writes a 16-bit value to the audio DMA control register, followed by reading a global variable, masking and combining its bits with constants and writing the resulting 16-bit value to the DMA control register.
- `process_input_and_flags`: The routine is empty.
- `handle_interrupt_flags`: The routine inspects a global variable and branches based on its bits.
- `set_custom_bit_flag`: The routine loops over a range of bit positions and tests a bit in a combined value formed from an input word and a global table entry.
- `set_target_lock_bit`: The routine loops incrementing a counter in a register until a bit in a register is set, then stores a bit mask derived from the counter into a short at an offset from a pointer.
- `update_ship_weapon_state`: The routine checks a short value in a global variable.
- `reset_state_and_tick_game`: The routine clears a byte in a global variable, increments a global counter, clears another byte, and calls several other routines to read input, update timers and HUD, handle player and station logic, and dispatch input.
- `set_cia_timer_and_flags`: The routine sets a global variable, tests a derived condition from it and conditionally calls another routine, then writes a constant and zero to the global variable.
- `clear_target_flag`: The routine clears a byte in a table pointed to by a register, using a word from a register as an index with a mask in one branch.
- `clear_screen_cell`: The routine stores a zero byte into a byte addressed by a pointer in A0 plus an offset derived from the low byte of the word in D1 with the high bit cleared.
- `build_target_mask`: The routine builds a bit mask from a signed-short terminated table in a pointer.
- `decrement_timer_and_update_hud`: The routine decrements a short timer and, when it reaches zero under a condition, increments a set of counters and resets them on overflow.
- `reset_game_state_11`: The routine stores zero into a 16-bit field at a fixed offset from a register and then performs an indirect call through a table.
- `update_player_state`: The routine updates a player state block when a flag is set.
- `input_dispatch_handler`: The routine checks a global flag and, if clear, indexes a table of function pointers using a global word and calls the selected routine via an indirect jump.
- `update_hyperspace_color`: The routine updates a short value stored in a global variable by adding or subtracting a constant, writes the result to a hardware color register, and toggles a flag in another global variable when the value reaches a limit.
- `increment_hud_timer`: The routine increments a short counter stored in a global variable and resets it when it reaches a threshold.
- `clear_screen_color`: The routine writes a constant value to a global variable that maps to COLOR14 in custom hardware, then returns.
- `read_joystick_and_pot`: The routine reads joystick and potentiometer hardware registers and updates a small set of bytes in a table with delta values for two axes and button states.
- `init_mouse_input`: The routine is empty.
- `return_immediately_2`: The routine contains eight instructions and performs no observable work before returning.
- `check_game_over_state_2`: The routine contains eight instructions but performs no observable work on memory or registers and immediately returns to its caller.
- `clear_input_buffer`: The routine spins until a short at an offset from an uninitialized pointer is non-zero, then writes zero to a word at another offset from that pointer, writes a constant to a word at a further offset, writes zero to the following word, sets a global variable to a constant, and clears a long at another offset from the pointer.
- `reset_game_state_12`: The routine calls another routine to toggle a palette register, clears a 16-bit value in a global variable, calls another routine that may skip execution, and stores a 32-bit value from a register into a global variable.
- `set_comms_channel_length`: The routine copies a string to a buffer, then walks a zero-terminated string at a fixed offset from a base pointer, counting characters while toggling a flag.
- `cache_string_length`: The routine copies a string to a buffer and then scans a null-terminated string at a fixed offset in a global structure, counting its characters with a toggling flag.
- `self_modifying_mission_timer`: The routine reads a global short timer value and updates it.
- `clip_polygon_to_viewport`: The routine clips a line segment to a rectangular viewport using Cohen-Sutherland outcodes, then masks a rectangular region of screen memory.
- `apply_starfield_mask`: The routine computes a mask index from two input values and, when the index is zero, calls another routine to obtain an index, loads a 16-bit mask from a table via a global variable, inverts it, and combines it with four 16-bit words in a buffer pointed to by a register using bitwise AND and OR with data from a pointer obtained from a register.
- `draw_scanlines`: The routine stores two words from registers into a global variable and sets a byte in the same global variable to a constant.
- `update_target_rotation`: The routine reads two short offsets from a global structure, checks they are within a small signed range, and if so calls another routine with the first offset.
- `rotate_palette_word_and_tick_counter`: The routine checks a global flag and, if set, decrements a global counter and reloads it when it reaches zero after calling another routine with combined input words and storing the result in a global variable.
- `copy_starmap_data_logic`: Calls a routine to check a state and copies starmap data.
- `copy_starmap_data`: The routine copies a block of data from a source table to a destination table using a loop that repeats a fixed number of times.
- `apply_mask_to_planet_data`: The routine masks four 32-bit words per iteration using a mask in D0 and its complement, combining the masked source words from a table in A2 with the destination words in A0.
- `xor_decrypt_game_data`: The routine obtains a flag from another routine and uses it together with a global variable to decide whether to call a decryption helper and to set up two pointers.
- `xor_decrypt_block`: The routine XOR-decrypts four interleaved blocks of 32-bit words using a 32-bit key in a register and a 32-bit key table pointed to by a register.
- `set_weapon_pointer`: The routine stores a pointer into a global variable.
- `apply_screen_mask`: Masks four 16-bit words of a screen buffer using a 16-bit mask from a table.
- `apply_scanline_mask`: The routine uses registers and a global variable to select a 32-bit mask and then iterates over a block of memory accessed through two pointers, combining the pointed-to words with the mask and its complement using bitwise AND and OR, advancing the pointers by fixed offsets and rotating the mask each iteration.
- `decode_galactic_coordinates`: The routine repeatedly updates a block of memory using a mask derived from a global variable and two input values, rotating those inputs each iteration and conditionally adjusting a pointer offset based on a comparison of two 16-bit values.
- `apply_mask_to_planets`: The routine copies a mask from a global table into three destination blocks using a source pointer and a destination pointer, applying a bitwise mask to each word.
- `apply_mask_to_planet_data_2`: The routine clamps two word inputs to a range, selects a mask word from a table using a value returned by another routine, then applies the mask to a block of memory accessed via two pointers.
- `apply_scanner_mask`: The routine masks a block of memory using a value in a data register and a table pointed to by a pointer.
- `skip_negative_scores`: The routine increments a pointer by two bytes in a loop and calls apply_scanline_mask and apply_scanner_mask until the word pointed to by the pointer is negative, then returns that word.
- `clear_palette_entries`: The routine writes zero to sixteen consecutive 16-bit locations starting at the hardware register for COLOR00, decrementing a counter from fifteen down to zero.
- `draw_scanline_starfield`: The routine reads a linked list of star entries from a global variable and for each entry computes a screen interval using the current scanline and writes the corresponding X positions into two separate tables.
- `draw_scanline_wireframe`: The routine iterates over a small set of entries in a table and, when a flag is set, calls another routine and updates a global variable.
- `process_combat_targeting`: The routine sets a flag in a byte pointed to by a pointer, writes two 16-bit values to offsets in a base structure, then loops over a linked list of byte blocks.
- `update_scan_result`: The routine reads a short from a function pointer and a short from a register, maps each to a flag byte, stores the flag byte and the two shorts through a pointer, optionally links the pointer into a global structure, and updates a global minimum and maximum short with a pointer to the current record.
- `copy_commodity_record`: The routine copies a byte-masked value and two fields from a source structure to a destination buffer pointed to by a0, and updates two pointer fields in the destination when a condition on d5 holds.
- `decode_crypt_message`: The routine checks a flag in a global variable and returns if it is not set.
- `toggle_cop1lch_palette`: The routine toggles a flag in a global variable and writes four 16-bit values to COP1LCH. It copies data from a table pointed to by a global variable into the COP1LCH palette registers, using a loop that processes four entries, and then inverts the flag.
- `set_palette_entries`: The routine copies sixteen 16-bit palette words from a table pointed to by a register to consecutive hardware color registers starting at COLOR00.
- `reset_video_hardware`: The routine resets Amiga custom chip registers for display timing and bitplane control, clears DMA and interrupt request registers, writes a byte to a global location, toggles a palette routine, shifts a series of short values in a global table left, calls a palette setup routine, then spins on a global variable before writing a new value to it and returning the value read from a custom chip register.
- `empty_routine_5`: The routine contains eight instructions and performs no observable work; it ends with a return.
- `update_commodity_price`: The routine reads an integer from a structure, makes it positive, calls another routine, then stores the value into different fields of a second structure depending on a short flag, updates a damage field with a difference, and writes the value back to the first structure.
- `copy_ship_data_3`: The routine copies a small fixed block of word data from one offset in a structure pointed to by A6 to another offset in the same structure, then clears a word at a third offset in the structure.
- `update_ship_trajectory`: The routine reads a short from a register, and if it is non-zero it performs a short linear recurrence using three short values from a structure pointed to by a register, updating the three values in place.
- `scale_byte_value`: The routine reads a byte from an offset in a global variable, calls another routine, and returns the byte scaled by 255 over 255, which is effectively the original byte value.
- `copy_ship_name`: The routine calls another routine and then copies up to three two-character entries from a table of short indices into a name buffer at an offset from a base pointer, skipping space characters and counting copied characters.
- `update_ship_status`: The routine copies ship data, updates the ship trajectory, copies two words from a source area to a destination area, and computes several short fields from bytes in the ship structure using shifts, masks and arithmetic.
- `copy_ship_data_and_update_combat`: The routine calls another routine, copies twelve words from a source area to a destination area within the same structure, increments a word derived from a masked field, and then processes seventeen entries from a global table into a local buffer using arithmetic and bit operations.
- `run_galactic_chart_screen`: The routine resets game state and copies ship data and strings, formats a number, parses input, then loops over a table of short values, skipping negative entries, setting a weapon pointer, scaling a byte, generating a random color and applying a screen mask for each entry.
- `process_game_loop`: The routine repeatedly calls several helper routines and decrements a stack variable until it reaches -1, then it updates the starfield buffer, resets combat targets, conditionally resets combat state, and writes two words to a structure.
- `reset_game_state_13`: The routine calls another routine to generate a random color and another routine to reset combat state, then stores the value 1 into two words at offsets from an uninitialized address register.
- `update_starfield_buffer`: The routine prepares a starfield display buffer by updating palette masks, copying scanlines, generating a random color offset and copying the starfield buffer.
- `update_star_system_data`: The routine updates a global variable and repeatedly calls another routine, applying a palette mask several times per iteration.
- `update_system_state`: The routine stores a random colour delta and a second delta into a structure, writes a saved register value into a global variable, calls two helper routines, then conditionally decodes a map or draws a sprite and updates flags.
- `apply_palette_mask`: The routine loads a 16-bit mask from a table indexed by the low nibble of a value returned by another routine, inverts it, and combines it with four 16-bit words in a buffer pointed to by a register.
- `reset_combat_targeting_state`: The routine resets several global combat state variables and drives a short targeting loop.
- `update_combat_state`: A routine that updates combat-related state by copying a global word, branching on a global short flag to call navigation or reset helpers, and when another flag is clear it resets game state, increments a message counter, runs a game loop, reads input, copies a ship name and resets combat, then returns a short status.
- `draw_starfield_background`: The routine reads a global variable, scales it, stores the result to a global variable and then runs a loop that calls another routine eight times per iteration while updating a short counter and a signed offset based on the scaled value.
- `update_fleet_status_flags`: The routine obtains an index from another routine, loads a 16-bit mask from a table via a global variable, and if the mask has no bits in common with four 16-bit words in a block of memory, it clears bits in those words where the mask has ones and then ORs in bits from three other global variables masked by the mask.
- `clear_weapon_flags_for_targeting`: The routine resets ship and weapon state and clears a global variable, then sets two short globals to a value derived from a global flag.
- `scan_for_target_ship`: The routine copies a ship name and uses its length together with a table of 17 entries to compute screen coordinates, then checks a scanline bitmap at those coordinates for a non-zero pattern; when a pattern is found it stores a pointer to a scan buffer, clears a word, calls set_target_coordinates and read_input_buffer, and returns the result.
- `scan_sector_for_target`: The routine writes a pair of input values and their ranges into a global structure, clears two counters, copies ship data, then iterates over 256 entries.
- `generate_random_color`: The routine takes two 16-bit inputs, multiplies each by a constant, then forms a 32-bit value by packing the low and high parts of the first product into the upper 16 bits and the low and high parts of the second product (with an offset added to the high part) into the lower 16 bits.
- `generate_random_color_pair`: The routine takes two 16-bit inputs and returns a 32-bit value.
- `update_combat_state_logic`: Updates a ship status, generates a color, and modifies several global variables.
- `find_best_scan_angle`: The routine stores a random color pair and a value from D1 into a global variable, clears two distance fields, copies ship data, then loops 256 times scaling a byte value, computing a squared distance, and updates the minimum and maximum distance values and their associated indices in global variables.
- `compute_screen_offset`: The routine takes two short inputs and two short values from a global variable, scales the differences, and builds a 32-bit value from the remainders and quotients of division by 40 with added constants.
- `compute_starfield_offset`: The routine takes two short inputs, scales each by a constant, adds a global variable offset per component, and combines the two results into a 32-bit value.
- `reset_combat_state_3`: The routine updates ship status and screen offset, increments a message counter and stores adjusted values into two global variables, then processes a game state update.
- `update_best_distance`: The routine stores the result of a starfield offset call into a global variable and clears a distance accumulator, then copies ship data.
- `find_nearest_star_system`: The routine stores two input words into a global structure, initializes two distance globals to extreme values, copies ship data via another routine, then loops over 256 entries.
- `compute_distance_squared`: The routine takes two short inputs and a pointer in a register, reads two short values from a global variable at fixed offsets from that pointer, subtracts each from the corresponding input, squares the two differences and returns their sum as an int.
- `reset_combat_message_state`: The routine increments a message counter and stores adjusted values into two global variables, then calls a game-state update routine and clears several global word variables.
- `reset_game_state_14`: The routine checks a flag in a global variable and either calls two subroutines or clears the flag, then runs a sequence of game-loop and input helpers, clears several words in the global data area, updates an index and copies a short from a table into a global word, and copies a starfield buffer if the copied value is zero.
- `reset_game_state_9`: The routine clears several words and bytes in a global variable area, calls a game loop and several helper routines, reads an input buffer twice and returns its value.
- `reset_state_and_read_input`: The routine updates best distance and combat state, runs the game loop, increments a message counter, copies the ship name, reads input twice and queries weapon energy, then resets game state.
- `animate_target_reticle`: Updates a reticle’s position fields from a global variable and animates four bounded edges by moving them each frame until a mask byte clears.
- `handle_unreachable_code`: The routine contains no instructions and returns immediately.
- `copy_table_and_update_combat`: The routine copies a series of word blocks from a source buffer into a destination buffer using a table of lengths stored in a global variable, then updates combat state.
- `copy_ship_data_4`: The routine calls another routine to copy a ship name and then copies a series of 16-bit words from a table pointed to by an address in A6 into a destination buffer.
- `copy_jameson_text`: The routine copies a block of bytes from a global variable into a buffer passed in a register, then copies a null-terminated string from a zero address into a location offset from that buffer.
- `reset_target_lock`: The routine calls another routine to check game over, then calls another routine to reset game state and another routine to reset disk menu state.
- `validate_commander_name`: The routine runs a game loop step, increments a message counter, reads the input buffer and resets combat state.
- `reset_target_lock_logic`: Validates a name, reads from a disk, and compares a string against a buffer.
- `verify_commander_password`: The routine reads input and a disk label, then compares a string from a global table against a stored password string in memory.
- `check_message_box`: The routine resets combat state, checks a commander name, copies ship data and runs the game loop.
- `save_commander_data`: The routine processes disk label input and commander data.
- `copy_ciab_buffer_and_process_input`: The routine copies a 256-byte block from an input area to a buffer, decrypts it, copies it back, stores a checksum byte, waits for button release, sets a flag in a global variable, waits for a target value and for input, disables a joystick button, runs a game loop, and then resets combat or disk menu state depending on a flag.
- `reset_combat_state_5`: The routine resets weapon state and message counter, copies four words from a table indexed by a global variable into a global buffer, applies a scanline mask three times, clears several global words, updates a global short and copies a word from a table indexed by that short into another global word, then conditionally copies a starfield buffer using values read from globals and the mask result.
- `reset_combat_target_state`: Writes an input value into a global variable and copies a short block from a table indexed by that value into a working area, then clears several global variables and adjusts a pointer-based index.
- `choose_game_mode`: The routine checks a global variable and calls one of two other routines based on its value.
- `reset_disk_menu_state`: The routine calls another routine to reset game state, then parses an input line.
- `xor_decrypt_data_block`: The routine XORs a block of bytes in memory with a descending sequence of byte values starting from a constant.
- `read_disk_label`: The routine waits for input and repeatedly reads a byte from disk until a checksum matches, then compares a disk label string byte-by-byte against a reference string.
- `sum_checksum_bytes`: The routine reads a length from a global variable and then adds together that many bytes starting from the next word in memory, accumulating the sum in a byte variable.
- `initialize_disk_boot`: The routine runs a disk boot sequence.
- `wait_for_input_loop`: The routine repeatedly calls another routine to initialize disk DMA and another routine to wait for a joystick button, decrementing an uninitialized word each iteration until it becomes -1, then returns the joystick result plus two.
- `initialize_disk_dma`: Fills a table with a constant pattern and derives two words from input bytes, writing them to memory and a global variable; it then performs a patterned xor reduction over a buffer, updates a global variable and spins on hardware status bits before calling another routine and waiting for a hardware flag.
- `decode_text_block`: The routine decodes a block of 32-bit words in place using a byte-wide stream cipher derived from each word.
- `wait_for_keypress`: The routine clears a global variable and then repeatedly starts a disk DMA transfer and polls a joystick button routine until the poll returns zero.
- `wait_for_joystick_button`: The routine masks a global variable, sets a bit in it, increments a short through an address derived from an unused register, calls another routine to reset an accumulator, then spins until a bit in the global variable is cleared.
- `decrement_counter_and_wait_input`: The routine masks a global variable, decrements a short through an address in a register, calls another routine, then spins until a bit in the global variable clears.
- `wait_for_button_release`: Clears a global variable mask and waits for a CIA timer.
- `disable_joystick_button`: The routine modifies a global variable by clearing one bit and setting another bit, then returns.
- `wait_for_cia_timer`: The routine stores a byte from a register into a global variable and then waits on a global flag.
- `wait_for_target_value`: The routine loops until a word in a global variable matches a word held in a register.
- `wait_for_combat_dma`: The routine waits in a loop until a flag bit is set, then repeatedly calls a commodity-price routine and copies 128 words from a source pointer using a bit-mask and shift operation into a buffer whose address was stored earlier, finally returning the value from the stack.
- `init_disk_dma_transfer`: The routine sets up a global variable and a pointer to a table, writes values to a global variable, then polls a flag with a decrementing counter.
- `find_commodity_price`: The routine scans a table of 16-bit entries for a specific marker value, then examines the following 32-bit word and extracts a byte from it.
- `reset_accumulator`: The routine calls another routine to wait for a CIA timer and then returns the 16-bit value that was in D0 on entry.
- `clear_collision_flags`: The routine clears a bit in a set of bytes.
- `copy_starfield_buffer`: Clears a flag in a byte pointed to by a register and, if specific bits are set, calls another routine and copies a block of word data from a source pointer to a destination pointer computed from a global value and a global offset, using two nested loops with counters from a global variable, then returns.
- `copy_planet_texture`: The routine copies 16-bit words from a source pointer to a destination pointer using a nested loop controlled by two short values read from a global variable.
- `process_game_state_update`: The routine updates display buffers and copies a block of short words from a table pointed to by a global variable into another area, iterating over a count stored in the same structure.
- `decode_galactic_map`: The routine initializes a set of temporary mask words and a flag from a global table entry, then iterates over four interleaved scanline buffers.
- `apply_scanline_mask_2`: The routine rotates a mask word by a small amount and combines it with a mask word and a source word, writing the result into four separate locations of a destination buffer.
- `apply_scanline_mask_to_planes`: The routine rotates a mask by a shift amount and uses it to mask and accumulate four 16-bit planes in a 320×256 bitmap, updating per-plane mask accumulators in a global variable and copying masked words to a destination buffer when a flag is set.
- `rotate_galaxy_map_bits`: The routine rotates a 16-bit word by a variable amount and applies the rotated mask to three 16-bit words in a table at offsets 4000, 8000 and 12000, updating a set of accumulator words in a global structure.
- `rotate_scanline_pixels`: The routine rotates a 16-bit word by a variable amount and uses the result to mask and accumulate pixel data across four interleaved scanline buffers.
- `rotate_scanline_pixels_2`: The routine rotates a 16-bit mask by a variable amount and uses it to mask and accumulate four 16-bit words per iteration into a destination buffer at four fixed offsets, while also updating a small set of global variables that hold carry and mask state.
- `draw_scanline_sprite`: The routine initializes a set of mask words and sprite pointers from arguments and a global table, then enters a loop that builds per-scanline mask values by shifting and combining sprite data with a colour mask, writes those masks into a work area, and calls several mask and rendering helpers to process the current scanline before advancing the sprite pointers and repeating.
- `rotate_scanline_mask`: The routine rotates a 16-bit mask by a variable amount and combines it with per-scanline mask words stored in a global variable, updating four mask words used for sprite drawing.
- `rotate_and_blit_planet`: The routine rotates a 16-bit mask by a variable amount and uses the result to mask four interleaved word planes in a 16-bit word.
- `apply_graphics_mask`: The routine rotates a mask derived from a source word buffer and combines it with per-plane masks stored in a global variable, writing the results into four interlaced bitplanes pointed to by a parameter.
- `rotate_wireframe_edges`: The routine rotates a 16-bit mask by a variable amount derived from a register and applies the rotated mask to four 16-bit words spaced 4000 words apart in a buffer pointed to by a parameter, using global variables for masks and control flags.
- `apply_scanline_mask_4`: The routine processes four 16-bit mask words from a pointer, rotates them by a shift amount, combines them with a mask word and a global variable, and writes the results to four screen words spaced 4000 words apart, updating a global variable for each plane.
- `apply_scanline_mask_5`: The routine iterates over scanline words, rotating a mask derived from a global variable and combining it with sprite data from a pointer and a global table to mask four interleaved destination words.
- `render_scanline_pixels`: The routine processes a sprite scanline by rotating pixel mask words according to a shift amount and combining them with a mask and an existing pixel buffer.
- `apply_scanline_mask_6`: The routine rotates a mask value by a shift amount and combines it with a mask stored in a global variable and an input word to update four scanline words in a buffer, storing intermediate results back to global variables.
- `draw_planet_sprites`: The routine iterates over a set of sprite words, rotates each word by a shift amount derived from an input value, masks the rotated data with several global variables and combines the results with existing words in a destination buffer at fixed offsets.
- `apply_screen_mask_2`: The routine iterates over a block of screen words using a mask derived from a global variable and a parameter, applying masked writes to four interleaved planes and updating per-plane mask words in a global structure.
- `reset_target_limits`: The routine clears two 16-bit values in a global structure, reads a set of short parameters from a pointer in a4, and uses them to compute signed differences against limits stored in the same structure.
- `copy_scanlines_to_buffer`: The routine copies a rectangular block of 16-bit words from a source buffer pointed to by a global variable to a destination buffer referenced via a global variable.
- `save_game_state`: The routine writes a flag byte, two word inputs and a long input into a global structure, calls a starfield copy routine and a targeting check routine, stores a byte from a register, and conditionally decodes a galactic map or draws a scanline sprite.
- `get_ship_data_2`: The routine takes the value in D0, shifts it left by two bits, adds the result to the base address of a global table, and returns the 32-bit word stored at that computed offset.
- `check_game_over_flag`: The routine reads a short from an address formed from an uninitialized pointer plus an offset, and if the value is zero it calls another routine to set an input buffer then returns.
- `get_weapon_energy`: The routine reads a short from a global variable offset, selects between two offsets based on a flag at another offset, calls another routine to update ship status, calls a routine to decode compressed data, and returns the selected short shifted left by two.
- `scale_ship_size`: The routine reads a short from a global variable, calls another routine to update ship status, calls another routine to decode compressed data, then returns the previously read short shifted left by two bits.
- `reset_game_state_17`: The routine calls another routine to read an input buffer, then calls another routine to wait for a key release and stores its return value.
- `read_random_key_pair`: The routine repeatedly calls another routine to obtain two short values until both are non-zero after subtracting fixed offsets, then returns a 32-bit value formed by packing the adjusted values into the high and low words of a 64-bit result.
- `fill_screen_buffer`: The routine writes a sequence of values to a global buffer using a pointer derived from a register.
- `find_station_by_id`: The routine scans a table of station entries by stepping through fixed-size records starting from a base pointer derived from a register.
- `reset_game_state_18`: The routine resets several game state fields and repeatedly reads input, updates target position and coordinates, and waits for input.
- `set_combat_target_2`: The routine prepares ship data, sets several fields in a ship structure, updates a flag byte, and then copies a starfield buffer and checks targeting constraints.
- `update_game_state_2`: The routine calls another routine twice with a pointer argument, calls a save routine twice, and then calls a routine that skips negative scores.
- `process_commander_input`: The routine copies two strings, scans a global string to its terminator, sets a target position, then loops waiting on a global flag and calling a table routine, and for each character from the input buffer it calls a table routine, optionally writes to an input buffer and waits for a key release before continuing until the buffer ends.
- `jump_to_wait_for_keypress_2`: Jump stub: passes control straight on to wait_for_keypress_2.
- `wait_for_keypress_2`: Checks a condition using two shorts from a global variable and, if true, clears a palette, resets game state, parses an input line, reads an input buffer, and then enters an infinite loop.
- `trigger_novella_protection_violation`: The routine checks two words from a global variable and, if they differ, clears palette entries, resets game state, parses an input line and reads the input buffer, then enters an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_starfield_background`: The routine reads a global variable and a table of bytes, then writes a byte from a register through a pointer to memory.
- `wait_for_keypress_3`: The routine repeatedly calls another routine to initialize a disk DMA transfer and then calls another routine, looping until the second call returns zero.
- `wait_for_ciaa_pra_bit`: The routine polls a global variable until its bit 5 is cleared, then returns.
- `empty_stub_routine`: The routine contains no instructions and returns immediately.
- `decrement_joystick_counter`: The routine masks a global variable with a constant and sets low bits, decrements a short value through an address derived from an uninitialized register, calls another routine, then loops until a bit in the global variable is clear.
- `tiny_rts`: Small routine of 1 instruction that returns.
- `disable_joystick_input`: The routine clears a bit in a global variable, waits for a CIA timer, then repeatedly calls another routine while a bit in the global variable is set, writes zero to a memory location via an uninitialized pointer, and then loops until another bit in the global variable clears.
- `clear_screen_flag`: The routine stores zero to a two-byte location referenced through a pointer and then spins until a bit in a global variable is cleared.
- `empty_routine_3`: The routine contains no instructions and returns immediately.
- `read_joystick_input`: The routine reads the CIA B port B register, masks a global variable and returns a value derived from the port data and an intermediate value.
- `wait_for_cia_timer_2`: The routine clears a global variable and stores two bytes from a register into a global variable.
- `wait_for_input_flag`: The routine spins in a loop until a global variable becomes non-zero.
- `return_immediately_3`: The routine contains no instructions and performs no operations; it returns immediately on entry.
- `init_disk_dma_transfer_2`: A routine called by wait_for_keypress_3 that sets up disk DMA registers, clears a request flag, runs a polling loop and a small data stir loop, and tails to wait_for_input_and_stir_buffer.
- `wait_for_input_and_stir_buffer`: The routine loops ten times, calling another routine each iteration and exiting early if a global variable is non-zero.
- `find_nearest_star_system_2`: The routine scans a table of 16-bit entries for a sentinel value, then examines a pair of 32-bit words at the matching entry and builds a 32-bit mask from the low bits of those words.
- `find_next_target`: The routine walks a table of short entries pointed to by a pointer, using an input word to find matching entries and building a packed result from bits of two words.
- `check_game_state`: The routine calls another routine that waits for a CIA timer and then returns the value it received in D0.
- `empty_routine_4`: The routine contains no instructions and returns immediately.
- `apply_scanline_mask_7`: The routine processes four 16-bit mask words per iteration, combining them with a running accumulator stored in a global variable and with the current pixel data pointed to by A0 and A1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initiate_game_state`: The routine contains no instructions and returns immediately.
- `initialize_game_state_empty`: The routine contains no instructions and returns immediately.
- `empty_stub_routine_2`: The routine contains no instructions and returns immediately.
- `init_game_state_2`: The routine is empty and performs no operations.
