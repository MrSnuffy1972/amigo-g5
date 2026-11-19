# Game 969 (midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00969). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904480 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 54 of 54 named (0 author symbols), 54 of 54 described (10 by rule, 9 by the local model) |
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
| Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf:bootblock | 1 | 180 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 410 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 237 | 22 |

## Cases handled

- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: custom-format (non-DOS) disk, 1730 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 223 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 55 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: 3 track-loaded blocks found in recovered runtime segments and classified through them
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: blocks 2-135 (bytes 1024-69632) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.59)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: blocks 137-996 (bytes 70144-510464) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.68)
- midwinter (1990)(microprose)(en)[cr defjam - ccs - acc]/Midwinter (1990)(MicroProse)(En)[cr Defjam - CCS - ACC].adf: blocks 999-1084 (bytes 511488-555520) outside the filesystem hold data, not code (opcode density 0.034, entropy 5.59)
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

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_vertical_blank_sequence`: Waits for the vertical blank to start, then clears a global flag and increments two 16-bit counters before calling a handler.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `init_captains_from_table`: Returns immediately unless a global longword matches a fixed address, then returns when a byte at offset 59 is nonzero, and otherwise when a byte at offset 60 is nonzero and a global word is at least four, it indexes a table by a tripled value and stores three bytes into three global words before setting several constants.
- `update_global_pointer_via_table`: Reads a global word, transforms it with a rotate and bit change, and loads a value from a table into a structure, setting a flag word to all ones or clearing it depending on the sign, then clears the flag word when it was zero.
- `update_pointer_with_calculated_value`: If a value at offset 0 of a global pointer is not 2, it combines a shifted value from one global with a scaled value from another global, stores the sum at the pointer, writes a carry-adjusted result two bytes later, and clears the global at offset 0.
- `clamp_global_value`: Checks a global variable and, if certain conditions are met, clamps its value within a specific range or sets it to a constant.
- `frag_move_rts`: Short code fragment that copies 65535 to a fixed memory address, then returns.
- `accumulate_modified_pointer_values`: Sets bits in a structure based on the state of bytes indexed by two global pointers, then, when a flag word is nonzero, clears one of those bytes and adds the signed or unsigned version of a global pointer to two counters.
- `accumulate_joystick_input_to_counters`: When a flag word is nonzero, it reads a joystick register, derives a gray-code difference, and based on the state of specific bits adds a signed or unsigned version of two global pointers to two counters.
- `accumulate_mouse_delta`: Returns early unless a global word is nonzero, sets two bits in a structure based on button registers, then reads a mouse port, shifts both halves, masks them, and adds them into offsets 2 and 6 of the structure.
- `dispatch_via_table`: Updates several global words based on bits in a byte from a0, and if two 32-bit values at offsets 2 and 6 of a0 are zero, it performs an indirect jump through a table.
- `frag_add_rts`: Short code fragment that adds d1 to a fixed memory address, then adds d0 to a fixed memory address, then returns.
- `update_global_with_clamping`: Adds a value to a global variable and clamps the result so it does not drop below zero or exceed a certain threshold.
- `clamp_global_variable_bounds`: Adds the value in d0 to a 32-bit global variable at offset 1FF3E, clamps it to a minimum of three and a maximum of 325, then stores the result back.
- `load_table_pair_values`: Clears two 32-bit global variables at offsets 1FED8 and 1FEDC, then, when two flag bytes at offsets 2A9B8 and 2A9BA are both zero, reads a pair of 32-bit values from a table indexed by a shifted global variable and stores them into those two variables.
- `enable_interrupts_set_vectors`: Points a register at the custom chip, clears both interrupt mask words at offsets 154 and 156 to enable all interrupts, stores seven routine addresses into the interrupt vector table, sets the DMA enable word at offset 154, and restores the saved registers before returning.
- `frag_movem_lea_move_btst_beq_and_andi`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a6, then copies the memory at an offset from a6 to d0, then tests bit 14 of d0, then branches if the last result was zero, then masks d0 with the memory at an offset from a6, then masks d0 with 16383, then copies d0 to the memory at an offset from a6, then restores saved registers from the stack.
- `frag_move_rte`: Short code fragment that copies 3840 to a fixed memory address, then returns from the exception.
- `handle_cia_port_toggle`: Checks whether a specific interrupt bit and a status bit are both set, and if so saves a CIA port byte, runs a 100-iteration delay loop, clears that status bit, and toggles a bit in a global byte before setting or clearing a table entry based on the toggled value.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `stub_infinite_loop`: Spins forever in an empty loop, so the routine never returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_global_state`: Spins in a loop until the global variable at offset 1FF38 becomes non-zero, then spins in a second loop until the global variable at offset 1FF52 becomes zero, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_table_index`: Sets a global variable to a constant and returns a value calculated using a modulo and a lookup in a table based on the global variable.
- `execute_conditional_routines`: Checks if a 16-bit word in a global variable is zero; if so, it sets a hardware register to 8200, executes a sequence of indirect calls and branch routines, otherwise it sets four consecutive words to zero and returns.
- `init_audio_registers_self`: Writes the constant 12 to the second audio period register, sets bit 0 of the DMA control register, sets bit 0 of the blit control register, and stores the address of itself at offset 63784 of a global structure, then returns.
- `setup_audio_channels_two_three`: Stores a routine address in a global variable, then copies channel, length, volume, and period data from a table into audio channels two and three and enables DMA channels two and three.
- `copy_audio_volume_registers`: Copies four consecutive words from global variables into the four audio volume registers, then returns.
- `frag_move_rts_2`: Short code fragment that copies 12 to a fixed memory address, then copies 0 to a fixed memory address (2 times in a row), then copies 61630 to a fixed memory address, then returns.
- `clear_audio_and_settings`: Sets the DMA controller, clears the volume of all four audio channels, stores several routine addresses in global variables, and clears six bytes in a global table.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `find_first_set_bit`: Reads a flag byte and returns immediately if it is zero, otherwise clears it, scans bits seven down to zero of a value for the first set bit while decrementing a counter, stores the counter, and loads a longword from a table using an offset derived from the bit index, then returns.
- `load_table_data`: Sets two hardware registers to 1, calculates an offset from a value in a global variable, and loads five values from a table into a series of global variables using that offset.
- `load_table_data_2`: Sets two hardware registers to 1, calculates an offset from a value in a global variable, and loads five values from a table into a series of global variables using that offset.
- `configure_audio_channel_zero`: Copies a channel, length, and volume from a table, sets the period of audio channel zero, and enables DMA channel one.
- `modify_global_variable`: Performs bitwise operations on a global variable and conditionally resets it to a specific value or a routine address.
- `set_bit_from_low_three_bits`: Reads the low three bits of a global word, sets the corresponding bit in a global byte, then calls another routine and returns.
- `find_bit_and_load`: Searches for a bit in a bitmask and loads a value from a table using a calculated offset.
- `load_audio_table_pointer`: Enables an audio channel and DMA request, then reads a counter, masks its low three bits, multiplies by fourteen and uses that offset to copy two longwords and three words from a table into a global variable, finally storing a routine address there.
- `load_audio_table_pointer_2`: Enables an audio channel and DMA request, then reads a counter, masks its low three bits, multiplies by fourteen and uses that offset to copy two longwords and three words from a table into a global variable, finally storing a routine address there.
- `configure_audio_channel_zero_2`: Copies a channel, length, and volume from a table, sets the period of audio channel zero, and enables DMA channel one.
- `modify_global_variable_alt`: Performs bitwise negation and conditional assignment to a global variable, potentially setting it to a routine address.
- `set_bit_from_low_three_bits_2`: Reads the low three bits of a global word, sets the corresponding bit in a global byte, then calls another routine and returns.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
