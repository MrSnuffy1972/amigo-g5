# Game 621 (Battle Squadron - The Destruction of the Barrax Empire! (USA)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00621). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4810958 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 42 of 42 named (0 author symbols), 42 of 42 described (1 by rule, 41 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 227 of 227 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 474 of 474 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1998732 |
| code | 1964 |
| data | 2810202 |
| padding | 60 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Battle Squadron - The Destruction of the Barrax Empire! (USA).adf:bootblock | 1 | 237 | 18 |
| Battle Squadron - The Destruction of the Barrax Empire! (USA).ipf#adf:bootblock | 1 | 237 | 24 |

## Cases handled

- Battle Squadron - The Destruction of the Barrax Empire! (USA)/Battle Squadron - The Destruction of the Barrax Empire! (USA).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- Battle Squadron - The Destruction of the Barrax Empire! (USA)/Battle Squadron - The Destruction of the Barrax Empire! (USA).ipf decoded by disk-analyse to a 901120-byte disk image (tracks: T0.0: AmigaDOS; T0.1-79.1: Unformatted*)
- Battle Squadron - The Destruction of the Barrax Empire! (USA)/Battle Squadron - The Destruction of the Barrax Empire! (USA).ipf#adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- Battle Squadron - The Destruction of the Barrax Empire! (USA)/Battle Squadron - The Destruction of the Barrax Empire! (USA).adf: blocks 2-1759 (bytes 1024-901120) hold ASCII text
- Battle Squadron - The Destruction of the Barrax Empire! (USA)/Battle Squadron - The Destruction of the Barrax Empire! (USA).ipf#adf: blocks 2-1759 (bytes 1024-901120) hold ASCII text

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Battle Squadron - The Destruction of the Barrax Empire! (USA).adf:bootblock

- `copy_and_jump_block`: Copies 242 words from a source pointed to by a0 to a destination at offset 6e000, then jumps to that destination.
- `trigger_trap`: Loads an address into a0, moves it to offset 128, and executes a trap.
- `update_config_and_pointer`: Writes 1023 to offset 150 of a structure in a6, writes 16383 to offset 154 of a6, writes 33394 to offset 150 of a6, writes 127 to offset 3200 of a structure in a5, moves the byte from offset 3200 of a5 into d0, writes 144 to offset 3200 of a5, stores the address of the next instruction into offset 120 of a structure in a6, and writes 57344 to offset 154 of a6.
- `update_hardware_register`: Checks bit 4 of a byte at offset 512 of a structure in a5; if set, it calls another routine, then writes 8192 to offset 156 of a structure in a6 and returns via an exception return.
- `initialize_global_structure`: Sets a 16-bit value of 4 at offset 30 of a structure pointed to by a3, sets a 16-bit value of 667 at offset 34, clears a 16-bit value at offset 8, and returns.
- `wait_for_bit_clear`: Loops until bit 6 of a byte at offset 2 of the structure in a6 is cleared, then returns.
- `search_and_initialize_structure`: Searches a table for a specific 16-bit value, then performs two separate calls to another routine to initialize various fields in a structure in a6 using data from a structure in a3.
- `initialize_blt_structure`: Sets up a structure with various parameters, including bitplane and color data, and calls another routine to perform a Blitter operation.
- `update_structure_from_table`: If a 16-bit value at offset 8 of a structure in a3 is zero, it returns; otherwise, it performs several writes to a structure in a6 using values derived from a table at a3, including a bitwise rotation of a constant, and clears the value at offset 8 of the structure in a3 before returning.
- `delay_loop`: Decrements a counter from 3000 to 0 using a loop and returns the original value of d0.
- `set_bits_and_clear_counters`: Sets bits 0, 1, and 2 of a byte at offset 256 of a structure in a5, then enters a loop that toggles bit 0 and calls another routine while bit 4 of a byte at a pointer in a4 is set, finally clearing two 16-bit values in a structure in a3.
- `toggle_bits_and_decrement`: Sets and clears bits 0 and 1 of a byte at offset 256 of a structure in a5, then decrements a 16-bit value at offset 28 of a structure in a3 and returns.
- `toggle_bits_and_increment`: Clears bit 1, sets bit 0, clears bit 0, and sets bit 0 of a byte at offset 256 of a structure in a5, then increments a 16-bit value at offset 28 of a structure in a3 and returns.
- `set_bits_and_wait`: Sets bits 3, 4, 5, and 6 of a byte at offset 256 of a structure in a5, clears bit 7 and bit 3, then loops until bit 5 of a byte pointed to by a4 is cleared before returning.
- `process_counter_range`: Compares a 16-bit value in d0 with a 16-bit value at offset 28 of a structure in a3, then enters a loop that calls either another routine to decrement or another routine to increment a value and updates a 16-bit value at offset 28 of a structure in a3.
- `update_counter_and_bit`: Compares a 16-bit value in d0 with a 16-bit value at offset 0 of a structure in a3, and if they are not equal, sets or clears bit 2 of a byte at offset 256 of a structure in a5 based on bit 0 of d0, stores d0 into the structure in a3, and calls another routine.
- `copy_and_loop_data`: Sets up a destination address in a1, performs a loop that copies 3072 words from a source at offset 5ad08 to the destination, and calls two other routines while decrementing a counter.
- `initialize_system_state`: Sets several pointers in a3, a4, a5, a6, and a7, calls five other routines, executes a loop using d0, writes a 16-bit value to offset 150 of a structure in a6, and jumps to an address in a1.

### Battle Squadron - The Destruction of the Barrax Empire! (USA).ipf#adf:bootblock

- `copy_data_to_memory_and_trap`: Copies 243 words from a table into a memory location starting at offset an address using a loop, then calls a routine at that memory location.
- `trigger_trap_2`: Loads an address into a0, moves it to offset 128, and executes a trap.
- `write_values_to_structure`: Writes 1023 to offset 150 of a structure in a6, writes 16383 to offset 154 of a6, writes 33394 to offset 150 of a6, writes 127 to offset 3200 of a structure in a5, moves the byte from offset 3200 of a5 into d0, writes 144 to offset 3200 of a5, stores the address of the next instruction into offset 120 of a structure in a6, and writes 57344 to offset 154 of a6.
- `update_hardware_register_copy`: Checks bit 4 of a byte at offset 512 of a structure in a5; if set, it calls another routine, then writes 8192 to offset 156 of a structure in a6 and returns via an exception return.
- `initialize_global_structure_2`: Sets a 16-bit value of 4 at offset 30 of a structure pointed to by a3, sets a 16-bit value of 667 at offset 34, clears a 16-bit value at offset 8, and returns.
- `wait_for_bit_clear_2`: Loops until bit 6 of a byte at offset 2 of the structure in a6 is cleared, then returns.
- `search_table_and_set_flag`: Saves registers, loads a pointer into a0 from a memory address, compares a 16-bit word in d0 with a 16-bit word at the pointer, and if they are not equal, enters a loop that decrements d1 until it reaches zero before setting the overflow flag and returning.
- `initialize_structure_from_pointer`: Stores a pointer from a0 into offset 24 of a structure in a3, then performs two calls to another routine to initialize multiple fields in a structure in a6 using data derived from the stored pointer.
- `initialize_blt_transfer_structure`: Sets up a structure with Blitter parameters and calls another routine to perform a Blitter transfer, then compares the resulting data against a stored value to verify the operation.
- `check_table_byte`: Loads a pointer into a3 from a memory address, checks bit 1 of a byte at offset 8 of that pointer, and otherwise returns.
- `update_structure_from_table_2`: Writes several 16-bit and 32-bit values to a structure in a6, including a rotated constant based on a value from a table in a3, and clears a 16-bit value at offset 8 of the structure in a3 before returning.
- `run_delay_loop`: Decrements a 16-bit value from 3000 to 0 using a loop and returns.
- `modify_status_bits`: Sets bits 0, 1, and 2 of a byte at offset 256 of a structure in a5, checks bit 4 of a byte at a pointer in a4, and if set, clears bit 0 of the byte at offset 256 of the structure in a5, sets bit 0 of that same byte, and calls another routine.
- `clear_structure_fields`: Calls another routine, clears a 16-bit word at offset 0 of a structure in a3, clears a 16-bit word at offset 28 of the same structure, and restores registers before returning.
- `toggle_bits_and_decrement_2`: Sets and clears bits 0 and 1 of a byte at offset 256 of a structure in a5, then decrements a 16-bit value at offset 28 of a structure in a3.
- `toggle_bits_and_increment_2`: Clears bit 1, sets bit 0, clears bit 0, and sets bit 0 of a byte at offset 256 of a structure in a5, then increments a 16-bit value at offset 28 of a structure in a3.
- `modify_status_and_wait`: Sets bits 5, 6, and 7 of a byte at offset 256 of a structure in a5, clears bit 7 and bit 3, then waits for bit 5 of a byte at a pointer in a4 to be set.
- `compare_and_branch_counter`: Compares a 16-bit value in d0 with a 16-bit value at offset 28 of a structure in a3, then calls another routine and branches based on the comparison results.
- `increment_and_compare_counter`: Calls another routine, increments a 16-bit value in d1, compares d0 with d1, and if they are not equal, calls another routine and branches back to the start of the routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `test_and_set_bit`: Compares a 16-bit value in d0 with a 16-bit value in a structure in a3, tests bit 0 of d0, and if bit 0 is clear, sets bit 2 of a byte at offset 256 of a structure in a5.
- `update_counter_and_bit_2`: Clears bit 2 of a byte at offset 256 of a structure in a5, stores a 16-bit value from d0 into a structure in a3, and calls another routine before returning.
- `copy_and_loop_data_2`: Initializes a destination address in a1, copies 3072 words from a source at offset 5ad08 to the destination in a loop, and calls two other routines.
- `initialize_system_state_2`: Loads several pointers into registers, calls five other routines, performs a loop based on d0, writes a 16-bit value to offset 150 of a structure in a6, and jumps to an address in a1.
