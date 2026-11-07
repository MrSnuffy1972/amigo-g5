# Game 7 (3D Galax): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00007). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904864 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 137 of 137 named (0 author symbols), 137 of 137 described (16 by rule, 100 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 477 of 477 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 408000 bytes, 35 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2453 of 2503 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 32232 |
| code | 9478 |
| data | 441815 |
| padding | 1499 |
| unused | 419840 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| 3D Galax.adf:3DGalax | 2 | 267 | 8 |
| 3D Galax.adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1075 | 93 |
| 3D Galax.adf:3DGalax.game (raw code file, load address unknown) | 1 | 575 | 6 |
| 3D Galax.adf:3DGalax.boot (raw code file, load address unknown) | 1 | 568 | 29 |

## Cases handled

- runtime segment an address: 270 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 3D Galax/3D Galax.adf:3DGalax.game: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 3D Galax/3D Galax.adf:3DGalax.boot: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 402000 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### 3D Galax.adf:3DGalax

- `process_hunk_blocks`: Parses a stream of AmigaDOS hunk type codes, relocating and copying the referenced data blocks into a working buffer while tracking their sizes and offsets.
- `spin_delay_loop`: Repeatedly decrements a value held in a register until it reaches zero, discarding the result each time.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `accumulate_shifted_bits_2`: It tests whether a shifted version of a global variable is non-zero and returns early if it is, otherwise it shifts bits out of a global variable into a bit accumulator while pulling new bits from a source word.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `accumulate_shifted_bits`: Shifts bits out of a global variable into a bit accumulator while pulling new bits from a source word, accumulating them into a destination word.
- `invoke_cpu_trap`: It invokes a CPU trap and then falls through into unreachable instruction data.

### 3D Galax.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `forbid_and_invoke_routine`: It calls exec.Forbid to disable the system, then invokes a routine twice, calls another routine through an offset in a global variable, and returns a value passed in a register.
- `enable_interrupts_dma`: It writes to the color and interrupt-enable registers, using bit 15 to enable interrupts and DMA channels, and calls one unnamed routine.
- `call_six_instruction_block`: It references a six-instruction block by address and calls three unnamed routines.
- `six_instruction_block`: It consists of six instructions that are referenced by address but whose exact operations could not be recovered.
- `loop_on_global_flag`: Reads a value from a global variable and returns it; if that value is zero the routine exits, otherwise it enters an endless loop.
- `wait_nonzero_loop`: Reads a global variable and returns it; if the value is zero the routine exits, otherwise it enters an endless loop.
- `loop_color_register`: Reads a value from a global variable and returns it; if that value is zero the routine exits, otherwise it enters an endless loop that continuously writes to the color register.
- `loop_writing_color_register`: It writes to the color register in an endless loop without ever leaving.
- `seed_random_64bit`: It obtains a seed value from another routine and, if a register is zero, replaces it with a 64-bit value split into high and low 32-bit parts.
- `copy_struct_block`: Reads a value from a structure field, zeroes another field, and copies a computed number of bytes from one location to another using a word-sized loop with a leftover byte copy.
- `copy_byte_block`: Reads a value from a global variable, increments a counter, and copies a computed number of bytes from one location to another using a word-sized loop with a leftover byte copy.
- `copy_memory_block_3`: Copies four bytes from a source location to a destination using a word-sized loop counter, then copies any remaining leftover bytes one at a time.
- `copy_memory_block_byte`: It copies a block of memory one byte at a time using a double-register loop counter.
- `copy_memory_block`: Copies a block of memory from the address in a0 to the address in a5 one byte at a time, using a dbra loop counter, and returns.
- `compare_structure_fields`: Compares two fields of a structure pointed to by a register and, if they match and a flag field is non-zero, obtains an identifier from another routine and returns it.
- `compare_and_return_code`: Checks whether a pointer is null, stores a shifted value into a global variable, then compares two strings byte by byte and returns a constant depending on the comparison outcome.
- `compare_string_pair_2`: It returns a constant unless a byte in a register differs from a specific value and a pointed-to string is empty, in which case it returns a different constant.
- `compare_string_pair`: Compares two strings byte by byte, decrementing a counter until they differ or the counter runs out.
- `parse_drive_path`: It reads characters from another routine to detect a drive letter pattern, specifically checking for 'D' and 'F' followed by a digit and a colon.
- `tiny_move`: Small routine of 1 instruction that copies (a0)+ to d0.
- `empty_routine`: It consists of seven instructions that perform no observable work; it does not access memory, registers, or call other routines, and ends with a return.
- `copy_path_segment_to_buffer`: It scans a null-terminated string from a base pointer, advancing until it reaches a null terminator or a slash character.
- `check_extraout_flag`: Calls another routine and stores its result, then tests a flag and two fields of a pointer passed in a register.
- `return_value_or_default`: It calls a routine to obtain a value, then calls it again if that value is zero, substituting a fixed constant when the second call also returns zero, and finally returns the resulting value.
- `sum_array_or_default`: It sums twelve consecutive 16-bit words pointed to by a register and returns the accumulated total.
- `sum_negate_array_words`: It adds 128 consecutive 16-bit words pointed to by a pointer in A0, accumulating the sum in a register.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_and_return`: It calls another routine and then returns.
- `empty_routine_2`: This routine is empty and performs no operations.
- `store_register_global`: It loads the address of a global variable into a register and then stores the value of another register at that address.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d2.
- `tiny_moveq_2`: Small routine of 1 instruction that loads 0 into d3.
- `combine_divide_values`: It calls another routine that divides a register value, accumulating a product while looping, and combines the resulting quotient and remainder into a single value.
- `compute_and_combine_values`: Checks a register value, computes a quotient and remainder, loops while decrementing a counter and accumulating a product, then combines the results into a 32-bit value before returning.
- `store_and_loop_divide`: Stores several incoming register values into a structure, computes a quotient and remainder, then loops while decrementing a counter and accumulating a product into a global variable.
- `concatenate_d0_and_a6_word`: It calls a routine and returns a 32-bit value formed by concatenating the D0 register with a word read from an address computed from A6.
- `loop_toggle_cia_bit_2`: Loops a fixed number of times, calling a routine that toggles a CIA port bit and accumulating the results in a register.
- `loop_toggle_cia_bit`: It loops thirty times, each pass clearing a CIA port bit and recording the result.
- `clear_cia_port_bit`: It derives a bit index from a register and applies it to a global variable, clearing the associated CIA port bit.
- `configure_and_check_dma`: Clears several fields of a structure, checks a flag and a DMA-related global variable, writes constants into hardware registers, and compares two structure fields before returning.
- `poll_routine_with_counter`: Repeatedly calls a routine while decrementing a counter, returning its result as soon as a flag clears.
- `store_computed_structure_value`: It calls a routine and, when the result and two stored structure values satisfy the comparisons, it writes a computed value and a constant into the structure, otherwise it returns a fixed error code.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 24 into d0, then returns.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 27 into d0, then returns.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 25 into d0, then returns.
- `loop_and_check_table_entries`: It loops over a table, polling a status byte and indexing into a large array, performing several checks and calls, incrementing a counter until it reaches a limit, and returning different codes depending on the outcome.
- `tiny_moveq_rts_4`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `empty_stub_routine`: It performs no observable work and simply returns.
- `set_flag_and_increment_counter`: It sets a bit in a 16-bit word located a few bytes before a pointer in A6, using a bit index derived from D3.
- `clear_table_entries`: It clears a table by writing zero to successive entries.
- `extract_high_byte_pair`: It calls another routine to obtain a 32-bit value and reassembles it so the low 16 bits come from the result and the high 8 bits come from the original high byte.
- `interleave_word_bits`: It reads two consecutive words from a pointer and returns a value formed from their low bits.
- `apply_interleave_xor_mask`: It calls a routine to interleave word bits and obtains a 32-bit value.
- `xor_checksum_masked`: It XORs a run of nine words from a pointer into an accumulator, advancing the pointer each pass.
- `compute_xor_checksum`: It iterates through a sequence of words pointed to by an input pointer, XORing each into an accumulator.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `interleave_word_bits_2`: It interleaves the low and high bits of two 32-bit word streams pointed to by two input pointers.
- `configure_dma_registers`: It writes specific values to hardware registers associated with DMA control, including ADKCON, DMACON, DSKLEN, DSKPTH, and DSKSYNC. It enables DMA channels by setting bit 15 in the DMACON register.
- `poll_status_and_set_globals`: It polls a status byte in a loop, writing two constants to a global variable in different orders depending on the flag, and returns a success or failure code.
- `write_constants_return`: It writes a constant to a global variable, then writes another constant to the same global variable, and returns.
- `set_adkcon_flag`: It stores a constant into a global flag, then, when a signed value is negative, it derives a bit index from a register and writes a masked result back to that flag.
- `tiny_moveq_3`: Small routine of 1 instruction that loads 255 into d1.
- `set_bit_in_global_flag`: It reads a 16-bit value from a table pointed to by a register, adds a constant, masks the result to 5 bits, and uses it as a bit index.
- `wait_for_flag_and_result_2`: It writes a constant to a global variable, calls a routine, then waits in a loop for a status bit to clear and for another routine to return a specific value before returning.
- `update_table_entry`: Reads a word from a global table using an index, conditionally calls another routine if the value is negative, computes the absolute difference against a register, and writes the result back to the table.
- `wait_status_bit_clear`: Waits for a status bit to clear in a loop, using a short value stored at an offset from a register to index a global variable and storing zero there when the bit is clear.
- `lb_c000_b42`: It computes an index from a short value stored at an offset from a register, uses that index to address a global variable, and stores zero into the selected 16-bit slot.
- `modify_byte_and_wait_for_flag`: It calls a routine to read a byte, adjusts its bits based on the result, stores the modified byte into a global variable, then waits for a status bit to clear and for another routine to return a specific value.
- `store_masked_input_byte`: It calls a routine that masks an input byte using a global mask and a byte from a structure, then stores the resulting byte into a global variable.
- `mask_input_byte`: It builds a mask from a global variable and a byte taken from a structure, clears one bit in that mask based on a byte from the structure, and optionally clears another bit if a flag in a table indexed by the structure is set.
- `wait_for_flag_and_result`: It repeatedly sets a CIA timer until the routine returns one.
- `configure_ciab_timer`: It checks whether a global flag bit is clear and a passed value is not one.
- `set_ciab_timer`: It writes three constant values to a global variable in sequence.
- `invoke_and_return_value`: Calls a routine twice, then invokes a routine through an offset in a global variable, calls another routine, and returns a value passed in a register.
- `invoke_vtable_sequence`: It calls a routine via a vtable and, if the result is non-zero, calls two more routines through subsequent offsets.
- `init_custom_and_wait_beam`: Initializes custom chip registers and global variables, then waits for a beam flag in a loop while toggling a global variable.
- `wait_for_flag_and_set_flag`: It spins until a specific bit in a word at a certain offset from a pointer in a register is set, then writes a constant word to another offset from that same pointer and returns the observed word.
- `wait_for_flag_toggle`: It spins on a flag byte in a global structure until the flag is set, then spins until the flag is cleared again, and then returns.
- `loop_until_flag_set`: It repeatedly calls another routine while incrementing a global word by a constant until that word reaches a specific value.
- `loop_until_flag_clear`: It repeatedly calls another routine while decrementing a global word by a constant until that word wraps to zero.
- `wait_for_register_match`: It loads an address into a register and then loops while the high byte of a word at offset six from that address is equal to itself, effectively creating an infinite loop.
- `check_serial_input_and_wait`: It tests a flag in a structure via a pointer and, if set, toggles a global variable while repeatedly calling another routine until a short condition holds, then restores the global variable.
- `call_vtable_sequence`: It calls a routine via a vtable at an offset from a global variable, and if the result is non-zero it calls two more routines via offsets from that result and from the global variable.
- `call_supervisor`: Wrapper that calls the exec library function Supervisor and returns its result to the caller.
- `write_custom_registers`: It stores a constant word into a global variable, then stores zero into it.
- `call_graphics_subroutine`: It calls a routine through a pointer in a register and, if that routine returns non-zero, calls another routine through a second pointer before returning.
- `init_and_store_struct`: Clears a register, inspects a byte in a global variable to set a flag, then reads a series of fields from a structure pointer and stores them into global variables before returning.
- `execute_indirect_table_sequence`: It reads a global variable, calls three routines via offsets from that value and from a second global, then stores a different global variable into the first global variable.
- `process_null_terminated_strings`: It scans a null-terminated string counting entries, and when it finds a marked terminator it measures the length of a following string and copies a computed block of memory, repeating until the string is exhausted.
- `copy_struct_field`: It copies one byte from a source pointer to a destination pointer and stores a value returned by another routine into a global variable.
- `copy_struct_fields`: Copies one byte from the source pointer to the destination pointer, then copies seven more bytes from successive source offsets to fixed destination offsets.
- `store_two_constants`: Stores a 16-bit value into a global variable and then stores a small constant into an adjacent global variable.
- `clear_zero_table`: Clears a block of memory by writing zero to a sequence of longwords, iterating a fixed number of times while decrementing a counter and advancing a pointer.

### 3D Galax.adf:3DGalax.game (raw code file, load address unknown)

- `initialize_display_and_copy_data`: Initializes the display by configuring the bit-plane registers, palette colors, and joystick ports, then copies and rearranges several data tables into fast memory and enables interrupts before handing off to the main game logic.
- `initialize_and_copy_memory_blocks`: It fills a memory region with a constant, toggles a global flag, calls several routines in a loop that program timer registers and copy a block of memory, and waits for a status bit to change before continuing.
- `clear_and_copy_tables`: Clears a table of 16-bit values, then copies two blocks of 16-bit data from one table to another, and finally clears five consecutive global variables.
- `clear_and_copy_tables_2`: It clears a table of 16-bit values, copies several blocks of 16-bit data from one table to another, clears consecutive global variables, and finally stores a constant into a global variable.
- `adjust_table_values`: Iterates over a table of paired values, and whenever one value is less than another minus a running total, it subtracts a constant from the second value and sets the total to its negative one.
- `read_and_transform_inputs`: Reads the joystick port movement and the fire-button and disk-status bits, then combines and shifts those bits to build a transformed status byte.

### 3D Galax.adf:3DGalax.boot (raw code file, load address unknown)

- `init_and_boot_disk`: It initializes a set of custom and CIA registers, stores several constants into global variables, and invokes a sequence of disk-related routines.
- `init_registers_and_call`: Initializes a set of custom and CIA registers, stores several constants into global variables, and invokes two other routines in sequence.
- `init_disk_and_call`: Programs a set of custom and DMA registers, stores constants into global variables, and calls three disk-related routines in sequence.
- `manage_table_scan`: Initializes a table of ten entries with the values 0, 5 and 10, then repeatedly scans that table while decrementing a counter, comparing each entry against a value returned by another routine and clearing or marking entries based on the result.
- `scan_and_update_table`: Initializes a ten-entry table with the values 0, 5 and 10, then repeatedly scans that table while decrementing a counter and comparing each entry, and finally writes a masked result back through a pointer.
- `configure_disk_registers`: It writes a series of constants into hardware registers and global variables, then calls a routine before returning.
- `copy_memory_block_2`: Copies a block of memory from the address in a0 to the address in a1 one longword at a time, using a dbra loop counter, and returns.
- `encode_word_to_mfm_2`: It repeatedly calls another routine to encode the two bytes of a 16-bit word into MFM clock and data bits, storing each 32-bit result into a global variable, with a fixed delay between encodings.
- `check_and_clear_bit`: Checks a bit in a byte pointed to by an offset, and if that bit is set it clears the corresponding bit in the following byte.
- `start_disk_read_when_flagged`: Checks a shared status flag and, only when it is set, waits a short fixed delay, programs the disk DMA pointer and disk length registers, calls two routines and then clears the flag.
- `start_disk_transfer_if_flagged`: Checks the same shared status flag and, when set, calls routines and programs the disk DMA pointer and control registers before clearing the flag.
- `wait_for_interrupt_request_bit`: Polls the interrupt request register until one particular bit is set, then clears that request by writing the register again and returns the bit's value.
- `step_disk_head_to_track`: Compares a requested track number, which must be below 160, with the last track stored, then pulses the drive step and direction control bits once per track of difference, with a delay for each step.
- `step_head_to_next_track`: Increments the stored current track number, stopping at the last track, and sends one head step pulse through the drive control bits with a delay.
- `recalibrate_disk_head`: Sets a drive control bit, waits with delays while a drive status line stays active, then resets the stored track position to zero and clears a related control bit.
- `set_drive_select_bit`: Sets the high bit of a shadow control byte together with the bit indexed by the byte's own low three bits, which selects one drive line.
- `clear_drive_select_bit`: Clears the high bit and the bit indexed by the shadow control byte's low three bits, which deselects the drive line, then calls one routine.
- `wait_for_disk_status_clear`: Waits a fixed delay, then busy-waits while a disk status bit in the shadow control byte remains set.
- `wait_for_status_bit_edge`: Waits while a status bit is set and then waits until it is set again, so it returns on the next rising edge of that status line.
- `decode_mfm_words_to_bytes`: Loops over a count of source words, shifting each word through a 32-bit register to pick out its data bits and writes one assembled byte per source word to the destination buffer.
- `extract_odd_bits_byte`: Shifts a 32-bit value eight times, gathering the spread-out data bits into one byte, and returns that value with one byte position masked out.
- `encode_bytes_to_mfm_words`: Takes each source byte, shifts its eight bits out one at a time and inserts clock bits between data bits where neighbours are zero, writing one encoded word per byte to the destination.
- `encode_word_to_mfm`: Encodes the two bytes of a 16-bit word into MFM clock and data bits using the same shift-and-insert-clock steps, leaving the 32-bit result in a register.
- `fill_buffer_alternating_pattern`: Fills a block of 4000 long words in memory, starting at a fixed buffer address, with an alternating bit pattern.
- `clear_block_and_stamp_entries`: Clears a block of 1750 long words, then writes a marker byte and an offset value derived from the argument into three entries at fixed spacing.
- `write_mfm_sync_marks`: Calls one routine and then stores the standard Amiga disk sync word at six fixed memory locations spaced through the track buffer.
- `apply_track_and_run_sequence`: Copies the stored track number into the shadow control byte, then calls four routines in sequence.
- `delay_until_counter_wraps`: Counts a 16-bit register down in a tight loop until it wraps past zero, creating a fixed delay with no other effect.
- `copy_track_chunks_to_output`: Copies a 16-byte record from a table into working variables, then copies the track buffer to an output pointer in chunks, calling routines between chunks until the remaining count reaches zero.
