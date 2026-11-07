# Game 2052 (speedball 2 - brutal deluxe (1990)(imageworks)[cr cls - rzr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02052). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907136 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 43 raw-word lines |
| Functions named and described | pass, 33 of 33 named (0 author symbols), 33 of 33 described (1 by rule, 28 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1039 of 1074 (96.74%) |
| Recovered without load address (counted in proof 1, flagged) | 3072 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1185 of 1185 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 19241 |
| code | 3950 |
| data | 876776 |
| padding | 1 |
| unused | 7168 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CLS - RZR].adf:bootblock | 1 | 65 | 3 |
| logged disk load from Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CLS - RZR].adf boot-block load | 1 | 89 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1013 | 27 |
| Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CLS - RZR].adf bytes 1024-4096 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- speedball 2 - brutal deluxe (1990)(imageworks)[cr cls - rzr]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CLS - RZR].adf: custom-format (non-DOS) disk, 1719 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 60 of the 674 PCs the canonical run logged (121382 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4E75) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- speedball 2 - brutal deluxe (1990)(imageworks)[cr cls - rzr]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CLS - RZR].adf: the boot block reads 17920 bytes from disk offset 883200 to an address (found statically); recovered as a unit
- runtime segment an address (play-rs-m1500): 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 915 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- speedball 2 - brutal deluxe (1990)(imageworks)[cr cls - rzr]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CLS - RZR].adf: bytes 1024-4096 look like code no run loaded: recovered position-independently (load address unknown)
- speedball 2 - brutal deluxe (1990)(imageworks)[cr cls - rzr]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CLS - RZR].adf: 2 logged loads cover 20992 bytes of the image
- speedball 2 - brutal deluxe (1990)(imageworks)[cr cls - rzr]/Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CLS - RZR].adf: blocks 22-1724 (bytes 11264-883200) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.68)
- 1 never-loaded region(s), 3072 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CLS - RZR].adf:bootblock

- `call_two_boot_routines`: A six-instruction boot-block entry that only calls two unnamed routines.
- `run_boot_io_with_dma`: Issues an exec DoIO request, makes further library calls, and writes the background color register and the DMA control register with the enable bit set.
- `configure_hardware_registers`: Writes specific values to three consecutive memory-mapped hardware registers and then executes a function via a pointer stored in a library structure.

### logged disk load from Speedball 2 - Brutal Deluxe (1990)(ImageWorks)[cr CLS - RZR].adf boot-block load

- `write_packed_byte_pair`: Reads the longword just before its input pointer and writes a two-byte value into the output, combining the low byte of that longword with the high byte of a register.
- `shift_bit_buffer_by_count`: Shifts a 32-bit working value right repeatedly for a count held in a register, calling a refill helper whenever the value reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block`: Copies a block of data from a source address to a destination address using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_state`: Performs a sequence of global variable assignments, copies data to the autovector table, and executes a function via a pointer.
- `call_old_open_library`: Library stub: loads its arguments and the exec library base, calls OldOpenLibrary and returns its result to the caller.
- `program_and_wait_blitter`: Waits until the blitter is idle, programs its control, modulo, pointer and size registers, waits for it to finish, then clears the blitter interrupt request.
- `wait_and_clear_interrupt_requests`: Reads a CIA port value and a control register, clears all interrupt request bits, waits on a status bit, and clears the requests again.
- `build_copper_list_and_palette`: Builds a copper list in the output buffer by copying a template, writing bitplane pointer moves, writing 32 palette color register moves, and adding wait and end-of-list entries, while also filling a large table by copying and negating a sample sequence.
- `install_interrupt_vectors`: Copies a block of longwords from a table into a destination and installs six handler addresses into the autovector table.
- `generate_transformed_data`: Calculates a series of values using arithmetic transformations and writes them into a destination buffer in a structured format.
- `process_table_data_transform`: Iterates through a table of values to perform arithmetic transformations and bitwise manipulations on each entry.
- `clear_table_entries`: Iterates through a table of values and clears specific memory locations based on conditional checks of the table data.
- `update_physics_and_collision_states`: Updates a set of coordinate and velocity values by applying increments and decrements, then performs scaling calculations based on the resulting values.
- `toggle_state_and_update_pointers`: Toggles a global byte and updates a table of pointers and values based on the new state.
- `update_blitter_transformation_parameters`: Adjusts a set of coordinate values using fixed-point arithmetic and updates Blitter registers.
- `apply_complex_transformation`: Iterates through a table of values and performs fixed-point arithmetic to update a destination table with transformed coordinates.
- `calculate_vector_magnitude_and_transform`: Calculates the difference between two coordinate pairs and performs a series of conditional arithmetic operations to derive new values.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `update_structure_fields`: loops until a bit is cleared in a memory location and then writes several values to a structure pointed to by a register.
- `increment_table_values`: Iterates through a table of short integers, incrementing each value by three and performing a conditional negation via another routine if a value exceeds a threshold.
- `parse_string_data`: Parses a string to extract specific characters or sequences, updating a global pointer and writing multiple bytes to a destination based on the characters found.
- `toggle_global_value`: sets a global variable to one value, calls another routine multiple times, and then sets the global variable to a different value.
- `set_global_values`: Sets a global variable to specific values and calls another routine multiple times.
- `set_interrupt_request_seven`: stores the value seven into a global variable and touches the interrupt request register.
- `set_interrupt_request_eight`: stores the value eight into a global variable and touches the interrupt request register.
- `set_interrupt_request_high`: stores a large value into a global variable and touches the interrupt request register.
- `set_interrupt_request_very_high`: stores a larger value into a global variable and touches the interrupt request register.
- `set_interrupt_request_maximum`: stores a maximum value into a global variable and touches the interrupt request register.
- `configure_hardware_and_copy_data`: modifies a global variable, performs a block copy of data, and interacts with various hardware registers including DMA, interrupts, and custom chips.
- `execute_unspecified_routine`: an instruction sequence of twelve instructions that is called by two other routines.
