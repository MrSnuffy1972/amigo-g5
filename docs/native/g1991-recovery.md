# Game 1991 (midwinter (1990)(microprose)(en)[cr defjam - ccs - acc][h timecircle]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01991). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904048 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 30 of 30 named (0 author symbols), 30 of 30 described (2 by rule, 27 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 223 of 223 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 523 of 523 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 422 |
| code | 3540 |
| data | 879094 |
| unused | 20992 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf:bootblock | 1 | 52 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 307 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 159 | 15 |

## Cases handled

- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc][h timecircle]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf: custom-format (non-DOS) disk, 1717 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 163 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 55 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc][h timecircle]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf: 3 track-loaded blocks found in recovered runtime segments and classified through them
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc][h timecircle]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.038, entropy 6.27)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc][h timecircle]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf: blocks 22-135 (bytes 11264-69632) outside the filesystem hold data, not code (opcode density 0.014, entropy 6.54)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc][h timecircle]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf: blocks 137-996 (bytes 70144-510464) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.68)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc][h timecircle]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf: blocks 999-1084 (bytes 511488-555520) outside the filesystem hold data, not code (opcode density 0.034, entropy 5.59)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc][h timecircle]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf: blocks 1087-1152 (bytes 556544-590336) outside the filesystem hold data, not code (opcode density 0.054, entropy 5.68)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc][h timecircle]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf: blocks 1154-1279 (bytes 590848-655360) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.46)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc][h timecircle]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf: blocks 1284-1304 (bytes 657408-668160) outside the filesystem hold data, not code (opcode density 0.023, entropy 6.07)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc][h timecircle]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf: blocks 1306-1457 (bytes 668672-746496) outside the filesystem hold data, not code (opcode density 0.026, entropy 6.01)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc][h timecircle]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf: blocks 1459-1658 (bytes 747008-849408) outside the filesystem hold data, not code (opcode density 0.022, entropy 6.63)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc][h timecircle]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf: blocks 1660-1671 (bytes 849920-856064) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.06)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc][h timecircle]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf: blocks 1683-1694 (bytes 861696-867840) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.16)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc][h timecircle]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf: blocks 1704-1759 (bytes 872448-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.91)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC][h Timecircle].adf:bootblock

- `load_boot_disk_blocks`: Boot-block entry that copies a small routine into low memory, opens a library by name, and issues three disk read requests with fixed lengths and offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_and_reset_pointers`: Checks a global flag, then loops to write multiplied values from a pointer-based table into a hardware register range.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `initialize_global_with_table_values`: If a certain condition is met, it copies a sequence of bytes from a table into a memory range and sets a global variable to a constant; otherwise, it fills a memory range with an incrementing sequence of bytes.
- `update_global_pointer_via_table`: Calculates a new index using bitwise shifts and an XOR mask, then loads a value from a table into a global variable.
- `update_pointer_with_calculated_value`: Calculates a new value based on a global pointer and writes it to the memory location pointed to by that global pointer.
- `clamp_global_value`: Checks a global variable and, if certain conditions are met, clamps its value within a specific range or sets it to a constant.
- `set_global_to_max`: Sets a global variable to a constant maximum value.
- `accumulate_modified_pointer_values`: Checks a byte at an offset from a global pointer, modifies that byte, and then adds the signed or unsigned version of that pointer to two different global counters.
- `accumulate_joystick_input_to_counters`: Reads certain bits from a joystick or mouse register and, based on the state of those bits, adds a modified version of the input value to two global counters.
- `update_input_flags`: Checks a global variable and sets specific bits in another global variable based on certain bit flags.
- `execute_indirect_call_from_table`: Reads a byte from a pointer, uses its lower bits to select a table entry, and executes a routine at an address retrieved from that table if certain pointer-based conditions are met.
- `add_registers_to_global`: Adds the contents of two registers to a global variable.
- `update_global_with_clamping`: Adds a value to a global variable and clamps the result so it does not drop below zero or exceed a certain threshold.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_global_state`: Loops until a global variable is non-zero, then loops until it becomes zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_complex_index`: Performs a loop that modifies a global variable and returns a value composed of a modulo operation and a byte from a table or memory.
- `execute_conditional_routines`: Checks if a global variable is zero, then sets it to a specific address and executes a sequence of routines or indirect calls via that address.
- `set_global_pointer`: Sets a global variable to a constant and then sets it to point to itself.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_void`: The routine is empty and performs no operations before returning.
- `find_first_set_bit`: Checks a bitmask and searches downwards for a bit, then loads a value from a table using a calculated offset.
- `load_table_data`: Loads several values from a table into a global variable using an offset calculated from a constant.
- `modify_global_variable`: Performs bitwise operations on a global variable and conditionally resets it to a specific value or a routine address.
- `set_bit_in_global`: Sets a bit in a global variable where the bit position is determined by a bitmask or a bit in the same variable.
- `find_bit_and_load`: Searches for a bit in a bitmask and loads a value from a table using a calculated offset.
- `update_audio_table_pointer`: calculates an offset into a table and updates a global variable with a sequence of values or pointers loaded from that table.
- `modify_global_variable_alt`: Performs bitwise negation and conditional assignment to a global variable, potentially setting it to a routine address.
- `update_table_bitmask`: Sets a bit in a global variable based on a bitmask or a bit in the same variable using a table-based offset.
- `do_nothing`: the routine is empty and performs no operations before returning.
- `do_nothing_2`: the routine is empty and performs no operations.
