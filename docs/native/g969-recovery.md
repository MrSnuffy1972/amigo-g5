# Game 969 (midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00969). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903728 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 36 of 36 named (0 author symbols), 36 of 36 described (6 by rule, 30 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 187 of 187 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 617 of 617 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 422 |
| code | 3276 |
| data | 885694 |
| unused | 14336 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf:bootblock | 1 | 189 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 285 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 140 | 14 |

## Cases handled

- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: custom-format (non-DOS) disk, 1730 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 138 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 47 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: 2 track-loaded blocks found in recovered runtime segments and classified through them
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: blocks 2-135 (bytes 1024-69632) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.59)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: blocks 137-996 (bytes 70144-510464) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.68)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: blocks 998-1084 (bytes 510976-555520) outside the filesystem hold data, not code (opcode density 0.035, entropy 5.60)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: blocks 1087-1152 (bytes 556544-590336) outside the filesystem hold data, not code (opcode density 0.054, entropy 5.69)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: blocks 1154-1279 (bytes 590848-655360) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.46)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: blocks 1284-1304 (bytes 657408-668160) outside the filesystem hold data, not code (opcode density 0.023, entropy 6.07)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: blocks 1306-1457 (bytes 668672-746496) outside the filesystem hold data, not code (opcode density 0.026, entropy 6.01)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: blocks 1459-1658 (bytes 747008-849408) outside the filesystem hold data, not code (opcode density 0.022, entropy 6.63)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: blocks 1660-1672 (bytes 849920-856576) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.92)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: blocks 1682-1694 (bytes 861184-867840) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.92)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: blocks 1704-1759 (bytes 872448-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.91)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf:bootblock

- `initialize_system_and_call_disk_routine`: Calls a routine via a pointer from a register, populates a table with addresses through a loop, and calls a routine to perform disk operations before potentially jumping to an address or writing a value to a hardware register.
- `store_pending_interrupt_mask`: If the master interrupt-enable bit of the interrupt enable register is set, stores the bits set in both the enable and request registers, limited to the low 14 bits, in a global variable.
- `configure_disk_dma_and_wait`: Configures a hardware register with a specific bitmask based on values in a structure, sets three bytes in a memory buffer, and enters a loop that performs a large multiplication calculation before checking a bit in the buffer to decide whether to return or branch to another routine.
- `pulse_control_port_bits`: Writes six byte values to a control port, each clearing one of five bits in turn and then restoring all bits.
- `wait_port_bit_clear_word`: Calls a routine repeatedly while bit four of a byte at a fixed offset from the register pointer stays set, then clears a word field in the structure addressed by the second register.
- `modify_structure_field`: Stores a word from d0 into a structure field, then checks a word at an offset in a structure; if that word is negative, it calls a routine, otherwise it subtracts the value from d0 and potentially negates d0.
- `pulse_control_port_count`: For each count in its input, writes a pair of control-port values one bit apart so that each pair produces a single pulse, then waits a fixed delay loop before the next pair.
- `store_params_and_loop_calls`: Stores two register inputs into structure fields, then calls subroutines once per count in a loop, setting a field to two on each pass.
- `read_mfm_sector_with_retries`: Starts a disk read DMA with the MFM sync word and a fixed buffer address, waits for the read to finish, then searches for the sync word and decodes an MFM sector while checking its checksum.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_and_reset_pointers`: Checks a global flag, then loops to write multiplied values from a pointer-based table into a hardware register range.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `initialize_global_with_table_values`: If a certain condition is met, it copies a sequence of bytes from a table into a memory range and sets a global variable to a constant; otherwise, it fills a memory range with an incrementing sequence of bytes.
- `update_global_pointer_via_table`: Calculates a new index using bitwise shifts and an XOR mask, then loads a value from a table into a global variable.
- `update_pointer_with_calculated_value`: Calculates a new value based on a global pointer and writes it to the memory location pointed to by that global pointer.
- `clamp_global_value`: Checks a global variable and, if certain conditions are met, clamps its value within a specific range or sets it to a constant.
- `set_global_constant`: Stores the value 65535 in a global variable and returns.
- `accumulate_modified_pointer_values`: Checks a byte at an offset from a global pointer, modifies that byte, and then adds the signed or unsigned version of that pointer to two different global counters.
- `accumulate_joystick_input_to_counters`: Reads certain bits from a joystick or mouse register and, based on the state of those bits, adds a modified version of the input value to two global counters.
- `update_joystick_structure`: Checks a global variable, sets bits in a structure at a fixed address based on the state of specific bits in a hardware register, and adds processed values from the mouse port to two fields in that structure.
- `dispatch_jump_table_entry`: Checks a byte from a pointer in a0, updates several global words based on its bits, and performs an indirect jump to an address from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_non_zero_word`: Loops until the word at a specific global address is non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_conditional_routines`: Checks if a global variable is zero, then sets it to a specific address and executes a sequence of routines or indirect calls via that address; otherwise it sets four consecutive words to zero.
- `set_global_pointer`: Sets three global words to specific constants and then sets a global variable to point to itself.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `find_first_set_bit`: Checks a bitmask and searches downwards for a bit, then loads a value from a table using a calculated offset.
- `load_table_data`: Loads several values from a table into a global variable using an offset calculated from a constant.
- `modify_global_variable`: Performs bitwise operations on a global variable and conditionally resets it to a specific value or a routine address.
- `set_bit_in_global`: Sets a bit in a global variable where the bit position is determined by the lower three bits of another global variable.
- `find_bit_and_load`: Searches for a bit in a bitmask and loads a value from a table using a calculated offset.
- `update_audio_table_pointer`: Calculates an offset into a table and updates a global variable with a sequence of values or pointers loaded from that table, while also modifying audio period and DMA control registers.
- `modify_global_variable_alt`: Performs bitwise negation and conditional assignment to a global variable, potentially setting it to a routine address.
- `update_table_bitmask`: Sets a bit in a global variable based on a bitmask or a bit in the same variable using a table-based offset.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
