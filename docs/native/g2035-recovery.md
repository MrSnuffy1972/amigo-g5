# Game 2035 (sensible soccer (1992)(renegade)(m4)[cr crack inc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02035). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1814112 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 54 of 54 named (0 author symbols), 54 of 54 described (0 by rule, 54 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 567 of 567 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 494080 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1390 of 1432 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2625 |
| code | 5960 |
| data | 1707925 |
| padding | 1346 |
| unused | 96256 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 1 of 2).adf:bootblock | 1 | 23 | 2 |
| logged disk load from Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| logged disk load from Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 466 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 463 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 480 | 20 |
| Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 1 of 2).adf bytes 393728-887808 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- sensible soccer (1992)(renegade)(m4)[cr crack inc](disk 1 of 2)/Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1718 non-empty blocks: matched against track-loaded segments
- sensible soccer (1992)(renegade)(m4)[cr crack inc](disk 2 of 2)/Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1610 non-empty blocks: matched against track-loaded segments
- sensible soccer (1992)(renegade)(m4)[cr crack inc](disk 1 of 2)/Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 1 of 2).adf: the boot block reads 512 bytes from disk offset 393216 to an address (found statically); recovered as a unit
- sensible soccer (1992)(renegade)(m4)[cr crack inc](disk 1 of 2)/Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 1 of 2).adf: the boot block reads 2560 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 154 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 204 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 209 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- sensible soccer (1992)(renegade)(m4)[cr crack inc](disk 1 of 2)/Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 1 of 2).adf: bytes 393728-887808 look like code no run loaded: recovered position-independently (load address unknown)
- sensible soccer (1992)(renegade)(m4)[cr crack inc](disk 1 of 2)/Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 1 of 2).adf: 3 logged loads cover 497152 bytes of the image
- sensible soccer (1992)(renegade)(m4)[cr crack inc](disk 1 of 2)/Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 1 of 2).adf: blocks 12-25 (bytes 6144-13312) outside the filesystem hold data, not code (opcode density 0.005, entropy 2.00)
- sensible soccer (1992)(renegade)(m4)[cr crack inc](disk 1 of 2)/Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 1 of 2).adf: blocks 35-767 (bytes 17920-393216) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.67)
- sensible soccer (1992)(renegade)(m4)[cr crack inc](disk 2 of 2)/Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 2 of 2).adf: blocks 10-21 (bytes 5120-11264) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.96)
- sensible soccer (1992)(renegade)(m4)[cr crack inc](disk 2 of 2)/Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 2 of 2).adf: blocks 24-27 (bytes 12288-14336) outside the filesystem hold data, not code (opcode density 0.021, entropy 3.82)
- sensible soccer (1992)(renegade)(m4)[cr crack inc](disk 2 of 2)/Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 2 of 2).adf: blocks 32-32 (bytes 16384-16896) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.70)
- sensible soccer (1992)(renegade)(m4)[cr crack inc](disk 2 of 2)/Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 2 of 2).adf: blocks 35-1627 (bytes 17920-833536) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.77)
- 1 never-loaded region(s), 494080 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Sensible Soccer (1992)(Renegade)(M4)[cr Crack Inc](Disk 1 of 2).adf:bootblock

- `boot_block_entry`: The routine is the boot block entry point called by the Kickstart strap.
- `initialize_memory_block`: The routine writes a word and three long words into a structure pointed to by a1, then calls another routine via a pointer stored in a6 and finally invokes a library routine with the value returned in a1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_global_counter`: The routine updates a global variable under a guard condition, calls several other routines, and conditionally invokes two further routines and a function pointer with values derived from the global state.
- `write_player_sprite_table`: The routine copies a global variable to several RAM locations, updates the global variable with a small arithmetic sequence, computes two base offsets from the global variable, then writes four pairs of 16-bit words into two tables in RAM for three iterations.
- `swap_global_pointer_values`: Checks a global variable against a specific constant and swaps its value with another constant if the condition is met.
- `wait_for_vertical_blank`: The routine clears a hardware register, then spins until the vertical position register indicates the display has advanced past a threshold, and finally writes a constant value to a global variable four times.
- `check_joy_and_buttons`: The routine reads the joystick port movement register and the CIAA port A register, then performs a series of bitwise tests and assignments on a global variable, ending with a conditional write of a constant to that variable.
- `read_joystick_and_mouse_input`: Reads hardware registers for joystick fire buttons and mouse port movement to determine input states.
- `synchronize_two_indices`: The routine adjusts a global variable toward a first short value in D1 and then toward a second short value in D2 by stepping the variable by two each iteration, wrapping it at the limits implied by the code.
- `copy_masked_word_table`: The routine copies 16 masked word values from a source table into two fixed destination tables.
- `fill_table_with_masked_value`: The routine repeatedly calls another routine and writes a masked value from a register to a global variable and to successive entries of a table, stopping when a register value becomes zero.
- `scale_table_index_by_factor`: The routine reads a byte from a pointer, masks the low four bits, multiplies the result by an unsigned word from D2, and returns a 32-bit value formed by the high 16 bits of the product and the product shifted right by seven bits.
- `reset_global_accumulator`: The routine checks a global variable and, if it is not a sentinel value, clamps a negative value, recurses into another routine, then clears the variable and, if the result is negative, writes a sentinel and zero to the global variable.
- `clamp_and_double_global`: The routine checks a global variable and, if it is not equal to a sentinel value, clamps it to zero when negative, recursively calls another routine, doubles the variable, and if the result exceeds a threshold it sets the variable to the sentinel value and then to one.
- `reset_interrupt_flag`: The routine writes a constant to a global variable and then, if a flag is set, clears the flag and stops execution because the next instruction is invalid.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_global_if_nonzero`: The routine checks a global variable and, if it is non-zero, subtracts one from it.
- `apply_volume_scaling`: The routine scales a global variable by four successive table lookups, each multiplied and shifted right by six, and returns the combined input registers.
- `reset_audio_dma_state`: The routine sets a global variable to a constant, stores the incoming pointer in a global variable, calls another routine, then clears the same global variable several times and returns the combined input registers.
- `decrement_counter_with_wrap`: The routine calls another routine, then checks a global variable for a flag.
- `empty_routine`: The routine contains no instructions and performs no operations.
- `initialize_table_pointers`: The routine walks a byte table starting at an offset from a base pointer, repeatedly comparing and decrementing a counter, and on a specific condition it clears a set of global variables and another routine's state, then builds a table of zeroed pointers by writing nulls into a destination area and storing the addresses into a separate pointer table, advancing both tables in lockstep.
- `reset_game_state`: The routine searches a table for a byte value and, if not found, clears a set of global variables and a table of pointers, then returns.
- `reset_game_state_2`: The routine walks a byte table starting at an offset from a base pointer, using a counter and a comparison loop, and if the loop exhausts it clears a set of global variables and a table of pointers built from the base pointer, then returns.
- `clear_game_state`: The routine walks a byte table starting at an offset from a base pointer, repeatedly comparing successive bytes and decrementing a counter.
- `clear_team_tables`: The routine walks a byte table starting at an offset from a base routine, repeatedly selecting the maximum byte within a sliding window and using the resulting index to clear a set of pointer tables and reset several global variables.
- `clear_game_state_table`: The routine walks a byte table starting at an offset from a base pointer, repeatedly selecting the smallest remaining byte and using it as an index to clear a block of 32-bit entries in a table at a fixed address.
- `reset_game_state_3`: The routine walks a byte table from a fixed base, performing a nested loop that decrements counters and compares bytes.
- `reset_game_state_4`: The routine walks a byte table starting at an offset from a base pointer, comparing bytes in a nested loop and decrementing counters.
- `initialize_audio_channels`: The routine walks a byte table derived from another routine and, when the walk ends, clears a block of pointers in a table, zeroes several global variables and two other routines, and writes zero to four audio volume hardware registers.
- `reset_audio_dma`: The routine clears four audio volume registers and four two-byte globals, then writes a value to a global variable and returns.
- `reset_audio_channels`: The routine clears a global flag and then programs the four audio channel registers with values from a table in memory, performing two short delays between operations.
- `update_sound_channel_state`: The routine reads a sound-channel descriptor via a pointer and updates fields by indexing a table using a nibble extracted from the descriptor.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `process_timer_state`: The routine reads a control word from a structure pointed to by a register, dispatches on low nibble flags, updates fields in the same structure and writes a result word to another structure pointed to by a different register, returning a masked control value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_input_and_dispatch`: The routine repeatedly calls another routine and checks a set of global variables and a function pointer, decrementing counters and returning early when conditions on those globals are met.
- `process_input_loop`: Executes a polling loop that calls a dispatch routine and checks global state variables to determine if input processing is complete.
- `read_keyboard_state`: The routine reads a byte from a hardware register and updates five global byte variables with key state values.
- `scan_table_terminated_by_minus_one`: The routine copies fields from a pointer in A5 into several global variables and then walks a short table pointed to by the same structure, advancing by eight bytes per entry until a sentinel value of minus one is found.
- `store_table_pointers`: The routine copies several fields from a pointer passed in a register into global variables and then scans a short table pointed to by one of those fields until it finds a sentinel value.
- `load_table_from_pointer`: The routine copies several fields from a pointer in A5 into a set of global variables and then scans a short table pointed to by the structure for a sentinel value of -1, advancing by eight bytes per entry.
- `set_global_variable`: The routine adds a constant to an uninitialized value and stores the result in a global variable.
- `decode_table_entry`: The routine walks a table pointed to by a global variable using two indices and a stride, skipping sentinel bytes and adjusting the indices for specific marker values, returning when it encounters an end marker or an unhandled marker.
- `parse_message_table`: The routine walks a table pointed to by a global variable using two index variables and a stride, skipping entries marked with sentinel values and adjusting the indices on encountering control markers.
- `decode_table_entry_2`: The routine reads a table indexed by a global variable and a counter, walks through entries using sentinel bytes and adjusts indices based on marker values.
- `skip_table_rows`: The routine walks a table pointed to by a global pointer and indexed by a global offset and a counter, using two global variables as row and column steps.
- `decode_tile_map`: The routine walks a byte table pointed to by a global pointer using two index registers and a stride value, interpreting signed marker bytes to move the indices, skip, or compute a linear offset from two coordinates.
- `self_calling_loop`: The routine calls another routine twice and then enters an infinite loop that repeats while a global variable is zero.
- `call_other_routine_and_wait`: The routine calls another routine and then spins in a loop until a global variable becomes non-zero, then returns.
- `wait_for_global_nonzero`: The routine loads a global variable and repeats until the value is not zero, then returns.
- `empty_loop_routine`: The routine contains no reachable instructions and consists of an infinite loop that never executes any operation.
- `draw_sprite_quad`: The routine reads four 16-bit values from a structure pointed to by A5, loads a 16-bit value from D0, and calls another routine several times with those values and the A5 pointer.
- `set_custom_chip_registers`: The routine computes an address from its inputs and writes several values to custom chip registers, using a table lookup and a global variable as a flag.
- `set_sprite_animation`: The routine writes animation parameters into a structure pointed to by a register, using values from registers and from a table.
- `set_bob_dma_parameters`: The routine prepares a structure in memory for a blitter operation by writing colour and control words from tables and registers into fixed offsets of a structure pointed to by a register, and it sets a global variable and waits for a hardware flag before each write.
