# Game 1992 (midwinter (1990)(microprose)(en)[cr defjam][f pdx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01992). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904480 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 54 of 54 named (0 author symbols), 54 of 54 described (3 by rule, 36 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 283 of 283 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 832 of 832 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 5214 |
| data | 884928 |
| padding | 2 |
| unused | 14336 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf:bootblock | 1 | 180 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 410 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 237 | 22 |

## Cases handled

- midwinter (1990)(microprose)(en)[cr defjam][f pdx]/Midwinter (1990)(MicroProse)(En)[cr Defjam][f PDX].adf: custom-format (non-DOS) disk, 1730 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 223 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
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

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_vertical_blank_sequence`: Waits for the vertical blank to start, then clears a global flag and increments two 16-bit counters before calling a handler.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `always_branches`: Performs an unconditional branch to a fixed address.
- `init_captains_from_table`: Returns immediately unless a global longword matches a fixed address, then returns when a byte at offset 59 is nonzero, and otherwise when a byte at offset 60 is nonzero and a global word is at least four, it indexes a table by a tripled value and stores three bytes into three global words before setting several constants.
- `update_global_pointer_via_table`: Reads a global word, transforms it with a rotate and bit change, and loads a value from a table into a structure, setting a flag word to all ones or clearing it depending on the sign, then clears the flag word when it was zero.
- `update_pointer_with_calculated_value`: If a value at offset 0 of a global pointer is not 2, it combines a shifted value from one global with a scaled value from another global, stores the sum at the pointer, writes a carry-adjusted result two bytes later, and clears the global at offset 0.
- `clamp_two_values`: Returns early unless two global words equal one and eight, then shifts and subtracts from two global words, clamps each result to a range, increments a counter for each clamp, and stores all ones into a global word only when no clamp occurred.
- `set_global_to_max`: Writes a maximum unsigned word value to a global variable and returns.
- `accumulate_modified_pointer_values`: Sets bits in a structure based on the state of bytes indexed by two global pointers, then, when a flag word is nonzero, clears one of those bytes and adds the signed or unsigned version of a global pointer to two counters.
- `accumulate_joystick_input_to_counters`: When a flag word is nonzero, it reads a joystick register, derives a gray-code difference, and based on the state of specific bits adds a signed or unsigned version of two global pointers to two counters.
- `dispatch_via_table_index_2`: Reads a byte from a pointer, updates several global words, and if certain conditions are met, jumps to a destination address retrieved from a table.
- `dispatch_via_table_index`: Reads a byte from a pointer, updates several global words, and if certain conditions are met, jumps to a destination address retrieved from a table indexed by a scaled version of a global variable.
- `add_to_global_counters`: Adds the contents of two registers to two global longwords and returns.
- `clamp_global_variable_range`: Adds the value in d0 to a 32-bit global variable at offset 21588, clamps it to a minimum of zero and a maximum of 326, then stores the result back.
- `clamp_global_variable_bounds`: Adds the value in d0 to a 32-bit global variable at offset 21588, clamps it to a minimum of three and a maximum of 325, then stores the result back.
- `load_table_pair_values`: Clears two 32-bit global variables at offsets 21522 and 21526, then, when two flag bytes at offsets 2c002 and 2c004 are both zero, reads a pair of 32-bit values from a table indexed by a shifted global variable and stores them into those two variables.
- `enable_interrupts_set_vectors`: Points a register at the custom chip, clears both interrupt mask words at offsets 154 and 156 to enable all interrupts, stores seven routine addresses into the interrupt vector table, sets the DMA enable word at offset 154, and restores the saved registers before returning.
- `update_custom_register_mask`: Loads the address of a custom register into a6, checks a bit in a custom register, and if certain conditions are met, masks a value and stores it into a custom register.
- `set_color_register`: Writes a value to the custom color register and returns immediately.
- `handle_cia_port_toggle`: Checks whether a specific interrupt bit and a status bit are both set, and if so saves a CIA port byte, runs a 100-iteration delay loop, clears that status bit, and toggles a bit in a global byte before setting or clearing a table entry based on the toggled value.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `stub_infinite_loop`: Stores the value negative one into a global short at offset 2c0ac, tests that same location and branches back to the test when it is nonzero, so the loop never exits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_global_state`: Loops until a global variable is non-zero, then loops until it becomes zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_complex_index`: Performs a loop that updates a global variable and returns a value composed of a remainder and a byte from a table-based lookup.
- `reset_pointer_and_execute`: If a global variable is zero, it assigns a specific address to that variable, executes the routine at that address, and then executes several other routines or calls via the pointer.
- `set_audio_parameters`: Sets a global variable to a constant value and then updates it with its own address.
- `setup_audio_channels_two_three`: Stores a routine address in a global variable, then copies channel, length, volume, and period data from a table into audio channels two and three and enables DMA channels two and three.
- `copy_audio_volume_registers`: Copies four consecutive words from global variables into the four audio volume registers, then returns.
- `configure_audio_channels`: Writes a value into the second audio volume register, clears the third and fourth audio volume registers, stores a pointer into a global vector table, and returns.
- `clear_audio_and_settings`: Sets the DMA controller, clears the volume of all four audio channels, stores several routine addresses in global variables, and clears six bytes in a global table.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_from_subroutine_2`: the routine is empty and performs no operations before returning.
- `find_first_set_bit`: Checks a bitmask derived from a register and a global variable, then loads a value from a table using a calculated offset.
- `load_table_data`: Loads several values from a table into a global variable using an offset calculated from a constant.
- `load_table_data_2`: Sets two hardware registers to 1, calculates an offset from a value in a global variable, and loads five values from a table into a series of global variables using that offset.
- `configure_audio_channel_zero`: Copies a channel, length, and volume from a table, sets the period of audio channel zero, and enables DMA channel one.
- `modify_global_variable`: Performs bitwise operations on a global variable and, if a certain condition is met, resets a global variable to a specific value and a routine address.
- `set_bit_in_variable`: Sets a specific bit in a global variable based on a bitmask derived from that variable's own value.
- `find_bit_and_load`: Searches for a bit in a mask and loads a value from a table using a calculated offset.
- `update_audio_dma_parameters`: calculates an offset into a table and loads several values into a global variable, which includes updating audio or DMA-related registers.
- `load_audio_table_pointer`: Enables an audio channel and DMA request, then reads a counter, masks its low three bits, multiplies by fourteen and uses that offset to copy two longwords and three words from a table into a global variable, finally storing a routine address there.
- `configure_audio_channel_zero_2`: Copies a channel, length, and volume from a table, sets the period of audio channel zero, and enables DMA channel one.
- `reset_global_variable`: Performs bitwise logic on a global variable and, if a condition is met, resets a global variable to a specific constant and a routine address.
- `update_table_data`: Sets a bit in a global variable based on a bitmask derived from that variable's own value.
- `do_nothing_routine`: the routine is empty and performs no operations.
