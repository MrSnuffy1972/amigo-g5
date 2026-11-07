# Game 1992 (midwinter (1990)(microprose)(en)[cr defjam][f pdx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01992). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904048 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 39 of 39 named (0 author symbols), 39 of 39 described (2 by rule, 28 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 223 of 223 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 660 of 660 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 422 |
| code | 3942 |
| data | 885348 |
| unused | 14336 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf:bootblock | 1 | 189 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 307 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 159 | 15 |

## Cases handled

- midwinter (1990)(microprose)(en)[cr defjam][f pdx]/Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf: custom-format (non-DOS) disk, 1730 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 163 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 55 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- midwinter (1990)(microprose)(en)[cr defjam][f pdx]/Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf: 3 track-loaded blocks found in recovered runtime segments and classified through them
- midwinter (1990)(microprose)(en)[cr defjam][f pdx]/Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf: blocks 2-135 (bytes 1024-69632) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.59)
- midwinter (1990)(microprose)(en)[cr defjam][f pdx]/Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf: blocks 137-996 (bytes 70144-510464) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.68)
- midwinter (1990)(microprose)(en)[cr defjam][f pdx]/Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf: blocks 999-1084 (bytes 511488-555520) outside the filesystem hold data, not code (opcode density 0.034, entropy 5.59)
- midwinter (1990)(microprose)(en)[cr defjam][f pdx]/Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf: blocks 1087-1152 (bytes 556544-590336) outside the filesystem hold data, not code (opcode density 0.055, entropy 5.69)
- midwinter (1990)(microprose)(en)[cr defjam][f pdx]/Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf: blocks 1154-1279 (bytes 590848-655360) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.46)
- midwinter (1990)(microprose)(en)[cr defjam][f pdx]/Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf: blocks 1284-1304 (bytes 657408-668160) outside the filesystem hold data, not code (opcode density 0.023, entropy 6.07)
- midwinter (1990)(microprose)(en)[cr defjam][f pdx]/Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf: blocks 1306-1457 (bytes 668672-746496) outside the filesystem hold data, not code (opcode density 0.026, entropy 6.01)
- midwinter (1990)(microprose)(en)[cr defjam][f pdx]/Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf: blocks 1459-1658 (bytes 747008-849408) outside the filesystem hold data, not code (opcode density 0.022, entropy 6.63)
- midwinter (1990)(microprose)(en)[cr defjam][f pdx]/Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf: blocks 1660-1672 (bytes 849920-856576) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.92)
- midwinter (1990)(microprose)(en)[cr defjam][f pdx]/Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf: blocks 1682-1694 (bytes 861184-867840) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.92)
- midwinter (1990)(microprose)(en)[cr defjam][f pdx]/Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf: blocks 1704-1759 (bytes 872448-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.91)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf:bootblock

- `boot_block_setup_interrupts`: The decompiled code is unavailable, so this describes the listed facts only.
- `capture_pending_interrupt_bits`: When the interrupt master-enable bit is set, it stores the bits common to the interrupt enable and request registers, masked to their low fourteen bits, in a global variable.
- `read_mfm_track_sectors`: Sets a bit mask and a drive port byte, then decodes sector words by combining alternating bits with a mask for MFM decoding.
- `pulse_control_port_bits`: Writes six byte values to a control port, each clearing one of five bits in turn and then restoring all bits.
- `wait_status_bit_then_clear`: Loops while a status bit on a port is set, calling a subroutine on each pass.
- `step_position_toward_target`: Moves a stored position toward the input value, one step per unit of difference, by writing alternating control port values with a fixed delay between steps.
- `pulse_control_port_repeatedly`: Repeats a two-stage port write for the count in its input, with a fixed delay after each pair.
- `store_params_and_loop_calls`: Stores two register inputs into structure fields, then calls subroutines once per count in a loop, setting a field to two on each pass before the calls.
- `read_mfm_sector_with_retries`: Starts a disk read DMA with the MFM sync word and a fixed buffer address, waits for the read to finish, then searches for the sync word and decodes an MFM sector while checking its checksum.
- `return_from_subroutine`: The routine is empty and consists of a single return instruction.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_complex_state`: Checks a bit in a global variable, and if set, performs a loop that doubles values pointed to by a global pointer, clears several global or remote memory locations, and executes indirect calls or other routines.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `update_global_from_table`: If a certain condition is met, it calculates an offset, reads a byte from a table, and overwrites a global variable with a sequence of constant values.
- `modify_global_using_table_lookup`: Checks a global variable and, if non-zero, uses a bit-manipulated version of that variable to fetch a byte from a table and assign it to a global variable.
- `update_pointer_with_calculated_value`: Calculates a new value based on a global pointer and writes it to the memory location pointed to by that global pointer.
- `clamp_global_value`: Checks a global variable and, if certain conditions are met, clamps its value within a specific range or sets it to a constant.
- `set_global_to_max`: Sets a global variable to a constant maximum value.
- `accumulate_modified_pointer_values`: Checks a byte at an offset from a global pointer, modifies that byte, and then adds the signed or unsigned version of that pointer to two different global counters.
- `accumulate_joystick_input_to_counters`: Reads certain bits from a joystick or mouse register and, based on the state of those bits, adds a modified version of the input value to two global counters.
- `update_input_state`: Checks a global variable and, if certain bits are not set, modifies a byte in a global variable.
- `execute_indirect_call_from_table`: Reads a byte from a pointer, updates a global variable based on certain bits, and if a condition is met, calls a routine from a table using a calculated address.
- `add_registers_to_global`: Adds the contents of two registers to a global variable.
- `update_global_with_clamping`: Adds a value to a global variable and clamps the result so it does not drop below zero or exceed a certain threshold.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_global_state`: Loops until a global variable is non-zero, then loops until it becomes zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_complex_index`: Performs a loop that updates a global variable and returns a value composed of a remainder and a byte from a table-based lookup.
- `reset_pointer_and_execute`: If a global variable is zero, it assigns a specific address to that variable, executes the routine at that address, and then executes several other routines or calls via the pointer.
- `set_audio_parameters`: Sets a global variable to a constant value and then updates it with its own address.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_from_subroutine_2`: the routine is empty and performs no operations before returning.
- `find_first_set_bit`: Checks a bitmask derived from a register and a global variable, then loads a value from a table using a calculated offset.
- `load_table_data`: Loads several values from a table into a global variable using an offset calculated from a constant.
- `modify_global_variable`: Performs bitwise operations on a global variable and, if a certain condition is met, resets a global variable to a specific value and a routine address.
- `set_bit_in_variable`: Sets a specific bit in a global variable based on a bitmask derived from that variable's own value.
- `find_bit_and_load`: Searches for a bit in a mask and loads a value from a table using a calculated offset.
- `update_audio_dma_parameters`: calculates an offset into a table and loads several values into a global variable, which includes updating audio or DMA-related registers.
- `reset_global_variable`: Performs bitwise logic on a global variable and, if a condition is met, resets a global variable to a specific constant and a routine address.
- `update_table_data`: Sets a bit in a global variable based on a bitmask derived from that variable's own value.
- `do_nothing_routine`: the routine is empty and performs no operations.
- `do_nothing`: this routine is empty and performs no operations before returning.
