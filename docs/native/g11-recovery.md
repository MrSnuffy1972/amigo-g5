# Game 11 (AlteredBeast v2.0 0819): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00011). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4060118 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 115 raw-word lines |
| Functions named and described | pass, 426 of 426 named (0 author symbols), 426 of 426 described (20 by rule, 406 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1261 of 1261 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7090 of 7277 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 225621 |
| code | 30052 |
| data | 3639565 |
| padding | 16 |
| unused | 164864 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 733 | 41 |
| Disk.1:bootblock | 1 | 13 | 1 |
| Disk.2:bootblock | 1 | 107 | 3 |
| logged disk load from Disk.1 boot-block load | 1 | 6095 | 351 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 200 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 109 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |

## Cases handled

- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.1: custom-format (non-DOS) disk, 1614 non-empty blocks: matched against track-loaded segments
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.2: custom-format (non-DOS) disk, 1580 non-empty blocks: matched against track-loaded segments
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819.hdf: hard-disk image (AmigaDOS OFS, 4352 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree AlteredBeast_v2.0_0819_hdf, which is recovered file by file; no block outside the filesystem holds data
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.1: the boot block reads 244736 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- 27 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.1: 1 logged loads cover 244736 bytes of the image
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.1: blocks 2-21 (bytes 1024-11264) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.58)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.1: blocks 500-654 (bytes 256000-335360) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.55)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.1: blocks 663-663 (bytes 339456-339968) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.30)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.1: blocks 666-976 (bytes 340992-500224) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.20)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.1: blocks 978-1123 (bytes 500736-575488) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.42)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.1: blocks 1126-1174 (bytes 576512-601600) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.54)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.1: blocks 1177-1448 (bytes 602624-741888) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.58)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.1: blocks 1451-1529 (bytes 742912-783360) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.85)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.1: blocks 1654-1656 (bytes 846848-848384) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.16)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.2: blocks 22-379 (bytes 11264-194560) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.71)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.2: blocks 381-426 (bytes 195072-218624) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.67)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.2: blocks 430-449 (bytes 220160-230400) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.51)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.2: blocks 452-466 (bytes 231424-239104) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.56)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.2: blocks 500-654 (bytes 256000-335360) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.55)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.2: blocks 663-663 (bytes 339456-339968) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.30)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.2: blocks 666-1006 (bytes 340992-515584) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.93)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.2: blocks 1009-1447 (bytes 516608-741376) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.78)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.2: blocks 1450-1607 (bytes 742400-823296) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.62)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.2: blocks 1609-1625 (bytes 823808-832512) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.09)
- AlteredBeast_v2.0_0819/AlteredBeast_v2.0_0819_hdf/AlteredBeast/Disk.2: blocks 1627-1656 (bytes 833024-848384) outside the filesystem hold data, not code (opcode density 0.020, entropy 4.65)
- 1 packed track region(s), 51200 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `initialize_system_and_dispatch`: Initializes the system by setting up interrupt vectors and DMA/copper control registers, then reads a library call result and returns it.
- `return_d0_value`: Returns the 32-bit value already loaded into the D0 register without modifying it.
- `store_and_advance_pointer`: Checks whether a pointer in an address register holds a value below a threshold, and if so stores that value at an offset in a structure and advances the pointer by one entry.
- `jump_via_indexed_table`: Computes an index from the low bits of the D0 register, multiplies it by four, and jumps through a table of subroutine addresses.
- `store_16bit_field_and_global`: Writes a 16-bit value into a field of a structure and stores the same value into a global variable.
- `store_16bit_alt_field_and_global`: Stores a 16-bit value into a different field of a structure and writes the same value into a separate global variable.
- `store_16bit_field_and_global_variant`: Writes a 16-bit value into a structure field and stores the same value into the same global variable as the first routine, but with a different constant.
- `store_16bit_alt_field_and_global_variant`: Stores a 16-bit value into the alternate structure field and writes the same value into the second global variable, using a different constant.
- `store_16bit_field_and_global_third`: Writes a 16-bit value into the first structure field and stores the same value into the first global variable with yet another constant.
- `store_16bit_alt_field_and_global_third`: Stores a 16-bit value into the alternate structure field and writes the same value into the second global variable with another distinct constant.
- `init_global_constant`: Initializes a global variable with a fixed 32-bit constant and returns.
- `index_and_call_jumptable`: It rounds the input value down to the nearest multiple of four, then uses that result to index into a jump table and calls the routine found there.
- `perform_indirect_jumptable_call`: Loads the address of a jumptable into a register and then performs an indirect call through an entry selected by adding a stack value to that address.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `store_self_referential_pointer`: Writes the address of a global variable into that same global variable and returns the value passed in a data register.
- `copy_d0_to_global`: Copies the value in D0 to a global variable and returns it unchanged.
- `wait_and_return_input_byte`: It waits for a status bit to clear, zeroes a global variable, and then returns the input byte unchanged.
- `setup_interrupt_vectors`: Initializes several interrupt vectors by writing trap and interrupt handler addresses into fixed offsets of a vector table, then returns.
- `initialize_global_pointers`: It initializes a series of global memory locations with a return address and a code pointer, alternating with a different return address for some of them.
- `set_flag_and_call_indirect`: Stores the byte value 38 into a global variable and then makes an indirect call through an entry selected from a table of code pointers.
- `set_flag_and_call_indirect_2`: Stores the byte value 40 into a global variable and then makes an indirect call through an entry selected from a table of code pointers.
- `set_flag_and_call_indirect_3`: Stores the value 2 into a global variable and then makes an indirect call through an entry selected from a table of code pointers.
- `set_value_and_call_indirect`: Stores the value 1400 into a global variable and then makes an indirect call through an entry selected from a table of code pointers.
- `set_value_and_call_indirect_2`: Stores the value 1364 into a global variable and then makes an indirect call through an entry selected from a table of code pointers.
- `store_word_and_call_indirect`: Stores a word value taken from a register into a global variable and then makes an indirect call through an entry selected from a table of code pointers.
- `clear_value_and_call_indirect`: Stores the value 0 into a global variable and then makes an indirect call through an entry selected from a table of code pointers.
- `store_and_call_library`: It stores a 16-bit value carried in an address register into a global variable, then jumps to a routine at a fixed offset in a library.
- `store_constant_and_call`: It writes the constant value 65532 into a global variable, then jumps to a routine at a fixed offset in a library.
- `call_indirect_offset`: Stores a 16-bit value from the D4 register into global memory, then invokes the routine located at the address found 32 bytes past the one at a fixed location, and returns.
- `store_word_and_call`: It stores a word value at a fixed offset within a structure pointed to by a register, then calls another routine and returns.
- `store_button_mask_and_call`: It stores the constant value 2050 at a fixed offset within a structure pointed to by a register, then calls another routine and returns.
- `wait_for_button_release`: It spins in a loop until a bit in a global variable, which reflects a hardware button status, is cleared.
- `read_and_convert_buffer`: It returns the value from a library call when a stored counter is nonzero and the result is valid, otherwise it returns the bitwise complement of that result.
- `toggle_flag_bit`: It flips the lowest bit of a global variable, toggling a stored flag between two states.
- `fetch_and_min_high_score`: It queries a global table for a value, keeps the smallest of the results seen, and returns a combined pair of that value and a library call result.
- `save_high_score_if_empty`: It reads a stored value, then writes a new value into a global table only when a flag there is still zero.
- `configure_and_spin_forever`: It configures the blitter and copper timing registers, then enters an endless loop that never returns.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `return_input_byte`: It returns the byte it received in a register without modifying it.
- `load_address_and_return`: The routine loads an address into a register and then returns without performing any other operation.

### Disk.1:bootblock

- `initialize_system_block`: Initializes a system control block by storing a handler address and several configuration values, then repeatedly calls a routine until a flag indicates completion.

### Disk.2:bootblock

- `install_trap_and_copy_table`: Saves the trap vector address into a global variable, then issues a trap and copies a table of bytes into memory while polling a CIA port register until it changes.
- `initialize_and_copy_table`: Initializes a CIA port register and a global variable, then repeatedly calls another routine and copies a block of bytes from a table into memory.
- `wait_xor_flag_byte`: Waits in a loop until the low bit of the byte pointed to matches the low bit of a register value, then XORs that byte with 2 and returns the original byte value.

### logged disk load from Disk.1 boot-block load

- `initialize_system_state`: Copies the processor's status register into a memory byte by packing the condition-code flags, then repeatedly calls another routine while writing a sequence of control values into a hardware register and initializing several global variables and tables.
- `initialize_copper_and_palette`: Writes initial values into the copper list registers, then repeatedly clears a memory table and repeatedly updates a palette table until a counter reaches its limit.
- `program_blitter_window`: Programs the blitter window registers by writing the horizontal and vertical start and stop values into the video controller registers, then stores a couple of word values into a buffer.
- `clear_interrupts_and_enable`: Clears the interrupt request register, enables the interrupts and DMA channels, then stores a constant into a global variable and overwrites several word-sized fields of a structure with fixed values.
- `process_cia_status_and_decrement`: Reads a CIA status register and derives a flag byte from its bits, then decrements two countdown variables and invokes a couple of other routines.
- `process_cia_status_byte`: Reads a CIA register value, combines its bits into a status byte, and stores that byte into a global variable while clearing a pending flag, then decrements two counters and returns a status byte.
- `update_status_from_pointer`: Tests a status bit at a fixed offset from a pointer and, when set, updates a global variable using shifted values and stores a fixed constant at another offset before clearing the variable.
- `fold_flag_bits`: Reads four bits from a register value and combines them into a single byte by repeatedly doubling and adding one, toggling two of the bits as it goes.
- `shift_and_copy_tables`: Copies a block of words from a source table into a destination table while doubling each value, then shifts a second block of words left by one bit.
- `double_and_store_words`: Reads words from a source pointer, doubles each value, and writes the results into two destination locations offset from each other before shifting a second block left by one bit.
- `double_source_words`: Loops over a fixed number of entries, doubling each word read from a source pointer and storing the result into a destination table.
- `clear_word_blocks`: Clears a block of words in a destination table and zeroes out two related locations offset from each other, repeating the operation for a fixed count.
- `adjust_table_entries`: Fills a series of four-byte slots with a value, decrements a counter, and when that counter underflows adjusts a second table by nudging each entry toward a matching value in another table.
- `swap_value_pairs`: Swaps pairs of values stored in a small structure, toggles a flag bit, and writes the swapped values into two offset locations within that structure.
- `store_pointer_and_word`: Stores a pointer to a global structure into a slot of a target structure and writes a word value into an adjacent slot.
- `wait_and_reset_counter`: Waits in a loop until a global counter reaches a supplied value, then resets that counter to zero.
- `initialize_and_spin_loops`: Clears a global variable, loads a large value into another global, sets a counter, and then enters two loops that spin until a condition is met.
- `save_registers_and_walk_table`: Writes a constant into a global variable and a value into the video controller register that enables DMA channels, then repeatedly calls another routine that saves incoming register values and jumps to a fixed address until a stored value reaches a threshold.
- `store_table_index_result`: Multiplies a value from a register by ten and stores the result in a global variable.
- `store_scaled_value_and_dispatch`: Multiplies a value from a register by ten and stores the result into a global variable.
- `store_table_via_register`: Multiplies a value from a register by ten and stores the result in a global variable.
- `store_scaled_table_entry`: Scales a register value by ten to index into a table of four-word entries, storing an incoming value at the matching slot when the referenced entry is non-negative, then saves the value and pointer to the table and loops until another routine reports completion.
- `store_and_advance_pointer_2`: Saves several incoming register values into a global variable, then, when a condition holds, divides a value into quotient and remainder and drives another routine, repeatedly adjusting a counter and a pointer while advancing the pointer by a computed amount and resetting the counter.
- `fill_longword_sentinel`: Fills a run of longwords in a global variable with a fixed sentinel value, decrementing a counter each iteration and skipping the work when the first slot already holds that value.
- `fill_table_slots_when_flag_clear`: Loops ten times, checking a global flag and reading a counter, and writes the value 65535 into a two-byte slot of a table indexed by that counter when the flag is clear.
- `run_setup_loop`: Runs a fixed helper routine four times in a countdown loop, then repeatedly invokes another routine until it returns a specific value or a counter underflows, using the result to decide whether to stop early.
- `blit_mfm_decode_pass`: Programs the blitter with source, mask and destination pointers plus a size computed from a sector offset, then waits for the blitter to finish running one bit-mask pass over a track buffer.
- `copy_table_bytes_wrapped`: Computes a wrapped index into a table from two input values and then copies one byte per loop iteration, resetting the index whenever it passes a newline marker.
- `start_blitter_copy_pass`: Computes three blitter pointer values from its input and an offset, programs the blitter control and size registers for a two-row copy, then waits for the blitter to complete.
- `verify_and_decode_interleaved`: Verifies a bit-interleaved data block by computing its checksum, then programs the blitter to decode the verified data into a destination while validating interleaved word patterns along the way.
- `clear_flag_byte`: Reads a byte pointed to by a pointer and a flag from the preceding byte.
- `check_bit_interleave_checksum`: Walks a sequence of 32-bit values from consecutive memory locations, XORing alternating masked bits into an accumulator, then compares the result against a recomputed value and returns a fixed error code when they differ.
- `interleave_two_bitplane_words`: Takes two longvalues from consecutive memory locations and combines them by interleaving their bits, returning the merged longvalue.
- `xor_longword_mask`: XORs a run of longwords together, decrementing a counter derived from a shift amount each pass, and returns the low alternating-bit mask of the accumulated result.
- `shift_bits_across_long_pair`: Shifts a 32-bit value and the word after it by the same count and combines the shifted parts into one 32-bit result, realigning bitstream data.
- `call_three_routines_in_sequence`: Invokes three unnamed routines in sequence and returns with no result.
- `store_masked_bitplane_word`: Builds a word from the alternating bit pattern of the input and its shifted neighbours, clears the top bit when a flag byte before the target is set, and stores the result through the pointer.
- `search_tagged_list`: Walks a linked list of tagged entries, matching a search value against consecutive slots and returning a per-tag code, or returning negative one when the end of the bounded region is reached.
- `program_blitter_decode_shift`: Writes two words of shifted and masked data into fixed custom-register offsets, then programs the blitter for two decode passes using a shifted parameter and a computed size.
- `start_blitter_decode_pass`: Programs the blitter control, size and pointer registers for a decode pass using a shifted parameter and a computed size, then waits for the blitter to finish.
- `wait_status_bit_clear`: Loops until a status byte at a fixed offset from a pointer has a specific bit cleared, spinning on that memory location.
- `init_memory_block`: Initializes a block of memory by storing a sentinel value and several fixed constants at fixed offsets from a pointer.
- `start_disk_dma_read`: Checks a flag bit and, when set, programs the disk control register with a buffer pointer and transfer length, enables the disk DMA channel, and starts a read of a fixed length.
- `start_disk_dma_write`: Programs the disk control registers for an MFM write by setting precompensation, enabling disk DMA, and starting a write of a fixed length, then waits for the transfer to finish.
- `wait_disk_transfer_done`: Polls a status word at a fixed offset from a pointer for a fixed number of passes, clears a register at another offset, and returns zero when a flag bit is set or minus one on timeout.
- `tiny_moveq`: Small routine of 1 instruction that loads 255 into d1.
- `deselect_drive_bit`: Clears one bit in a global port register, with the bit position chosen from a stored drive index.
- `poll_disk_status_with_timeout`: Clears a global port register, drives two unnamed routines, then polls a status bit in a loop with a bounded counter, returning early when the bit clears or a fixed code when the timeout elapses.
- `apply_absolute_difference`: Loads a value from a table indexed by a register, takes its absolute magnitude, then subtracts a second value from the first and takes that absolute difference.
- `poll_drive_select_flag`: Spins a bounded loop that polls a status flag, clearing a drive-select port entry when the flag is set and calling a helper on each pass, and returns a fixed value when the loop exhausts.
- `configure_drive_select_port`: Reads a port value, clears a couple of bits based on a flag, stores the assembled value into a global register, then triggers a drive selection routine.
- `store_drive_select_and_delay`: Stores a drive selection value into a global register and then spins a fixed-count loop as a short delay.
- `decrement_counter_delay`: Decrements a counter in a loop until it reaches zero, acting as a simple delay.
- `build_drive_select_bits`: Builds a port value by clearing one bit chosen from a stored index and clearing a second bit when a flag is set, then returns the assembled value.
- `process_buffer_loop`: Stores incoming register values into global variables, then walks a byte buffer, branching on a range check and stopping when it encounters a terminator byte.
- `process_buffer_dispatch`: Stores incoming register values into global variables, then walks a byte buffer dispatching on each byte to one of two routines until it hits a terminator or a sentinel value.
- `do_nothing`: This routine contains no instructions and performs no operations; it is completely empty.
- `copy_bytes_advance_stride`: Copies three source bytes into a destination on each of seven iterations, widening the destination stride as it advances, then increments a global counter and returns the incoming value plus one.
- `blend_byte_into_buffers`: It reads a byte from a source buffer through one pointer and ORs it into three fixed locations of a destination buffer, then combines the inverted and raw byte into the current destination location.
- `increment_counter_and_return`: It increments a global counter by one and returns the incoming data register value incremented by one.
- `decrement_and_restore_byte`: Decrements a global counter, wrapping it back to a fixed value when it goes negative, then saves and restores a byte at an offset before invoking another routine.
- `decrement_and_restore_buffer`: Decrements a global counter, wraps it to a fixed value when it underflows, temporarily overwrites a byte in a buffer, and then restores that byte.
- `apply_bitmask_table`: Reads a byte from a table indexed by a register, then repeatedly applies bitwise AND and OR operations across several spaced offsets of a buffer, looping a fixed number of times.
- `copy_table_rows_scatter`: Reads a value from a global variable selected by a register and copies three consecutive words from a source table into three scattered destinations within a destination table, repeating the copy for a fixed number of source rows.
- `initialize_system_and_clear_table`: Initializes the hardware by enabling interrupts and DMA, then clears and sets up a block of memory through pointers before entering a timed loop that repeatedly zeroes a table and updates a counter.
- `build_structures_from_table_2`: Increments a global counter, then builds destination structures in a nested loop by reading an index from a table and filling consecutive fields with computed offsets and control words.
- `store_relative_offsets`: Adds a register value to a base stored in a global variable and writes the resulting offset into four consecutive fields of a structure, each offset increasing by a fixed amount.
- `initialize_stride_tables`: Initializes a table of words by writing a running value that increments by a fixed stride, then initializes a second table by writing another running value with a different fixed stride.
- `fill_stride_tables`: It fills a table of words with a running value that increments by a fixed stride, then fills a second table of words with a different running value using another stride.
- `program_blitter_fill_structures`: Programs the blitter with source and destination masks, modulo values, and a control word, enabling DMA channels, then fills destination structures from a table and stores a routine address into each.
- `configure_blitter_transfer`: Programs the blitter with source and destination word masks, modulo values, and a control word, then enables the DMA channels by writing to the DMA control register.
- `build_structures_from_table`: It subtracts two stored values with wraparound handling, then repeatedly builds destination structures by reading an index from a table and writing pointer and constant fields into them, wrapping the index when it exceeds a limit.
- `populate_structures_from_table`: Reads a pointer from a table indexed by a register and, for each entry, writes fields into a destination structure, choosing one branch when the entry is negative and another when it is non-negative.
- `populate_structures_from_table_2`: It reads a word from a source table and, depending on whether it is negative, writes either a pointer to another routine or an adjusted address into a destination structure.
- `setup_blitter_copy_rows`: Adjusts a source pointer and length based on signed bounds checks, then programs a series of blitter copy operations using the computed address and stride values.
- `setup_blitter_and_walk_table`: Loops over a table of signed words calling another routine for each entry, then writes a constant into a global variable and a value into the video controller register that enables DMA channels.
- `update_global_value_conditional`: It stores the value six into a global variable, then indexes into that variable with a shifted register to read a byte and, if a condition holds, overwrites the variable with fourteen before invoking another routine twice and performing a pair of word swaps using a temporary location.
- `copy_interleaved_rows`: Computes a blitter copy length from a signed bound check, then copies interleaved words from a source table into a destination area while swapping two global pointers.
- `copy_interleaved_words_index_2`: It computes a source address from an input value and a table, then copies four interleaved words from each source row into a destination area, repeating the copy for a fixed number of rows.
- `copy_table_rows_broadcast`: Copies three consecutive words from a source table into four scattered destinations within a destination table, repeating the copy for a fixed number of source rows.
- `clear_memory_block`: Clears a block of memory by writing zero words to it in a loop controlled by a counter.
- `setup_coprocessor_chain`: Stores pointers and constants into several fields of a structure and writes a value to a register, then sets two control words that enable the coprocessor chain.
- `initialize_system_and_wait`: Clears several memory blocks and tables, initializes audio channel state, sets up the coprocessor chain and interrupts, and then waits in a loop until a global counter reaches a supplied value before resetting that counter.
- `copy_table_rows_scatter_loop`: Copies three consecutive words from a source table into three scattered destinations within a destination table, repeating the copy for a fixed number of source rows.
- `copy_table_rows_scatter_counted`: Copies three consecutive words from a source table into three scattered destinations within a destination table, repeating the copy for a number of source rows given by a register.
- `setup_table_range`: Clears a block of memory and updates a global counter that selects a code path, then reads a table entry and computes pointer offsets and lengths from its fields to set up a range of destination slots.
- `tiny_ori_2`: Small routine of 1 instruction that merges 254 into d0.
- `setup_blitter_copy_rows_2`: Clears a large block of memory, then updates a global counter that selects one of several code paths.
- `wrap_byte_counter_down`: It decrements a byte stored in a global variable and, when that value underflows below zero, resets it to a fixed maximum and increments an adjacent byte in the same global area.
- `run_state_machine`: Runs a multi-stage state machine driven by a counter, clearing and filling tables, copying data blocks, and calling helper routines to reset and reinitialize game structures.
- `clear_word_block_and_store`: Clears a block of ten consecutive words in a global variable, then stores a constant into another global variable before returning.
- `setup_blitter_row_copy`: Runs two nested timing loops, then sets up blitter parameters and copies rows of data using computed source pointers and lengths.
- `decrement_counter_fill_table`: Decrements a global counter and returns while it stays nonzero, otherwise performs bookkeeping and, when a second counter is below a threshold, fills a run of words in a table with a fixed value before returning.
- `init_globals_copy_table_entries`: Initializes a series of global variables and pointers, copies a small block of words, then repeatedly compares a counter and copies or adjusts table entries while decrementing that counter until it reaches zero.
- `copy_nested_table_blocks`: It copies a repeating block of words from a source table into a destination area, advancing both pointers through nested loops and writing the data at several spaced offsets within the destination.
- `setup_sprite_data`: Sets up a sprite drawing routine by computing sprite coordinates and sizes from a table, adjusting for screen boundaries, and storing the sprite data including pointers, sizes, and positions.
- `setup_blitter_copy_rows_3`: Clips a source pointer and length against signed bounds, then programs a series of blitter row-copy operations using the adjusted address and a computed length.
- `tiny_ori_3`: Small routine of 1 instruction that merges 218 into d0.
- `setup_blitter_copy_rows_3_2`: Reads a pointer table and a source buffer, then programs a series of blitter row-copy operations using the adjusted address and length.
- `transform_copy_word_block`: It copies a block of words from a source table to a destination table while applying a bitwise AND/XOR transformation using several other words from the source table, decrementing a counter until the block is fully copied.
- `copy_masked_byte_table`: It reads a pointer table and a source buffer, then copies masked bytes from the source into indexed destination locations while advancing through rows for a nested loop.
- `adjust_stored_values`: Fills eight consecutive words in a global variable with the value minus 48, then compares two stored values and, when one is greater than zero and the other is not greater than the first, decrements one and increments the other.
- `adjust_table_and_values`: Fills a six-entry table by subtracting 48 from two bytes at each entry, then compares two stored values and adjusts them if the comparison succeeds.
- `decode_and_adjust_values`: Decodes a run of six bytes by subtracting 48 from each, then compares two stored values and adjusts them when the second falls within a range.
- `adjust_table_and_values_2`: It subtracts 48 from six pairs of bytes in a global table, then compares two stored values and, when one is greater than zero, decrements one and increases the other.
- `compare_and_insert_block`: It compares a block of bytes against a reference block, and if the reference is smaller it shifts stored entries and copies the reference values into a table slot.
- `store_constants_and_loop`: Stores two constants into global variables, then runs a countdown loop that repeatedly calls helper routines and decrements a counter until it goes negative or a condition stops holding.
- `copy_word_through_indexed_table_2`: It masks a stored value and, while a flag is positive, calls another routine and copies a word from an indexed location into a target address, resetting the flag under specific conditions.
- `copy_word_through_indexed_table`: It loads a word from a table using an offset and stores it into a table entry whose index is computed from another value.
- `decrement_and_update_flags`: It decrements a counter, returns an input value when the counter is positive, otherwise resets the counter and modifies byte and word fields based on several flag comparisons.
- `adjust_table_byte_copy_block`: Checks a flag and returns early when set, otherwise increments or decrements a byte in a table based on another flag, clamps a related value, and copies a block of words from one table to another.
- `adjust_and_shift_table`: Adjusts a byte stored in a table by incrementing or decrementing it and clamping the result to a valid range, then shifts a block of table entries down by one slot and repeats this for several consecutive slots.
- `copy_table_entries`: Copies a fixed-size block of data from a table indexed by a global counter into a destination buffer, repeating the copy for each of several entries.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 128 into d0.
- `copy_interleaved_words_index_3`: It calls another routine while a flag is positive and stores a value into a location, then copies four interleaved words from a source block selected by a counter into a destination area for a fixed number of entries.
- `copy_interleaved_words_index`: It computes a destination address from a table index, then copies four interleaved words from a source table into a destination block for a fixed number of iterations.
- `setup_game_tables`: Initializes a table of long words by writing a repeating pattern of values, then clears the lower half of a second table and adjusts the top value based on a flag.
- `reset_blitter_state_flag`: Sets a global flag to zero.
- `setup_game_tables_2`: It initializes a block of memory locations and stores a computed value into a table through a pointer, then branches based on whether the source buffer contains a nonzero entry.
- `initialize_long_table`: Initializes a table of longs by writing pattern values through a pointer, then stores a computed byte into a memory region via two pointers using fixed offsets.
- `store_table_values`: Writes values into a fixed memory table through a pointer, then reads from it to compute a doubled value and store it in two locations.
- `tiny_ori_4`: Small routine of 1 instruction that merges 255 into d0.
- `initialize_score_tables`: Initializes a table of 3000 words by writing zeros to it, then sets up a second table of 30 words by doubling each of its values and storing the results in two related tables.
- `initialize_game_structures`: Initializes a set of global variables and clears two tables in memory, then writes a computed value into a third table indexed by a mask.
- `initialize_game_state`: Initializes a block of memory by writing a fixed pattern into its entries, then clears a second table and sets a group of global variables to their starting values.
- `initialize_memory_tables`: Initializes a block of memory by writing a fixed value into each of its slots, then copies a transformed value from one table into another.
- `initialize_memory_blocks`: Initializes a block of memory by filling two tables with the value 96, then writes a value through a pointer and increments a counter.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `initialize_game_state_and_loop`: Initializes a large block of global state by writing fixed values into many fields and clearing two tables, then runs a timed loop that repeatedly zeroes a buffer and derives per-entry values from a source table.
- `reset_palette_buffer`: Initialises a large zeroed work area by filling a word table, clears a screen buffer through a pointer, and then writes a repeating fixed pattern into a colour palette table.
- `initialize_memory_block`: Initializes a block of memory by writing a repeating pattern and then clears a table through a pointer.
- `init_move_tables`: Initializes a table of 15 words by copying a doubled value from a source table into a parallel table.
- `initialize_game_state_2`: Initializes a block of global state by writing fixed values into its fields, then clears a large table to zero.
- `initialize_and_scale_table`: Initializes a block of memory by filling it with repeated values, then runs a timed delay loop and clears a table.
- `initialize_memory_structure`: Initializes a block of memory by writing a repeating pattern into a table and then sets up a series of configuration values in a global structure.
- `initialize_and_refresh_tables`: Initializes a block of memory by filling it with a repeated value, then runs a timed wait loop while repeatedly clearing a table and updating entries derived from another table.
- `tiny_ori_5`: Small routine of 1 instruction that merges 176 into d4.
- `clear_and_prepare_words`: Clears a table of words by zeroing its entries and then copies each word length into two parallel tables, one for the counts and one for the words themselves.
- `tiny_ori_6`: Small routine of 1 instruction that merges 202 into d1.
- `initialize_and_update_table`: Initializes a block of memory by filling it with repeated values, then runs a timed loop that clears a table and derives new values from existing ones until a counter reaches its limit.
- `write_scaled_table_entry`: Calculates a scaled value by masking a word and multiplying it by two, then stores the result into a table at a specific offset.
- `initialize_memory_and_table`: Initializes a block of memory by filling it with a repeated pattern, then runs a timed loop that clears a table and derives values from another table.
- `initialize_state_and_loop`: Initializes a set of global state and tables, then waits for a flag to change before looping while periodically clearing a buffer and updating per-slot values.
- `init_sprite_movement`: Initializes a block of sprite data by zeroing a table and computing per-sprite movement values, then waits for a flag before repeating.
- `initialize_palette_setup`: Initializes the palette by writing color entries to a screen data table, then sets up the two-player palette slots and clears the sprite attributes list for one screen field.
- `clear_memory_block_2`: Clears a block of memory by writing zeros to it, then increments a counter stored in a global variable.
- `init_score_tables`: Initializes a block of 16-bit words in a table by doubling a masked value from another table, then advances a pointer by a fixed amount.
- `initialize_channels_and_palette`: Initializes a block of channel structures by filling them with a repeated value, then repeatedly clears a palette table and rebuilds channel entries from a source table until a mode counter reaches a limit.
- `configure_blitter_window`: Waits for a vertical-blank interrupt flag to clear, then programs the blitter's start and stop horizontal window registers and points them at a global buffer.
- `process_record_sequence_2`: Invokes a record-sequence walker twice, passing the value returned by the first invocation as the initial argument to the second, then stores that returned value into a global variable.
- `process_record_sequence`: Walks a sequence of variable-length records pointed to by a buffer, advancing a destination pointer past each one until it reaches an end marker.
- `program_blitter_and_loop_routines`: Programs a series of blitter control words, then repeatedly calls a set of routines in two loops until counters reach target values, and finally restores the saved blitter state.
- `advance_sprite_animation`: Counts down a timer that toggles a flag and resets animation offsets while the game state is active, then sets up animation data by indexing a per-entity table and computing per-frame offsets and pointers into sprite data.
- `setup_blitter_copy_rows_6`: Runs a short startup sequence that flips a toggle bit and sets a timer, then indexes into a per-sprite table to compute source and destination pointers and lengths.
- `run_triple_loop`: Executes a fixed loop body three times by decrementing a counter from three to negative one.
- `decrement_counter_and_return`: Sets a bit pattern in a global variable through a pointer, decrements a counter, and returns a value derived from a register.
- `update_screen_timer`: Sets a couple of bits in a memory location pointed to by a register, flips a global counter, and then decrements or divides that counter while wrapping a second counter back to a starting value once it underflows.
- `advance_timer_sequence`: Sets a bit pattern in a memory location pointed to by a register, toggles a global counter, and when that counter is negative while another is positive it decrements a stored value or derives a new one, then decrements a fourth counter that wraps to a fixed value and advances a fifth.
- `update_screen_timer_2`: Toggles a global flag and, while a game state is active, decrements a byte counter or resets it from a shifted value.
- `copy_table_blocks`: Copies a block of words from one table to another using nested loops, filling the destination sequentially.
- `find_closest_table_entry`: Clears a block of words and then scans a table of pairs, comparing each entry against a running value to find the closest match.
- `select_and_copy_record`: It zeroes a table of words, scans another table of six-byte records to find the one with the smallest value in its first field (skipping records that fall outside a range), then copies fields from that record into a destination buffer and adjusts a value until it reaches a threshold.
- `select_and_store_table_entry`: Clears a block of short values in a table, then scans another table of short entries to find the smallest one that satisfies range checks, and writes its fields into a destination buffer pointed to by a register.
- `select_and_store_table_entry_2`: Finds the table entry with the smallest first field that satisfies range checks, then stores its values into a buffer while incrementing a counter until the buffer value exceeds a threshold and a nearby matching record is found.
- `setup_timers_and_init_registers`: It invokes another routine to set up timer registers using a doubled word as an offset into a hardware register block, then writes the original word back.
- `setup_timer_registers`: It doubles the incoming word and uses that doubled value as an offset into a fixed hardware register block, writing the original word back to one register and presetting several neighboring registers.
- `process_event_table`: It walks a table of word entries, skipping any whose first field is negative or whose value falls outside a global range, and invokes the routine indexed by the absolute value of a field shifted left by two bits.
- `process_event_table_entries`: It walks a table of two-word entries, skipping any whose first field is negative or whose value falls outside a global range, and invokes the referenced routine for each valid entry.
- `save_and_call_table_routines`: Saves a global word and clears it, then repeatedly calls a routine via a table of addresses indexed by a running value, and finally updates several global flags and counters.
- `set_global_counter`: It saves the current value of a global counter, replaces that counter with the value one hundred and ninety-nine, runs another routine, and then restores the original counter value.
- `save_restore_variable`: Saves the current contents of a global variable, sets that variable to a fixed value, invokes another routine, then restores the original contents of the variable.
- `configure_blitter_sequence`: Selects a routine from a table based on a decremented counter and a value that must not be five or one, then clears a block of memory and configures the blitter to copy a sequence of data.
- `setup_blitter_copy_sequence`: Decrements a global counter and, when it is nonzero and a certain value is not five or one, indexes a table to select a routine, clears a block of four-word slots, then programs a series of blitter copy operations and enables the DMA channels.
- `check_and_loop_byte`: It reads a single byte from a memory location, compares it against the value 47, and returns early if the two do not match.
- `clear_memory_block_3`: It decrements a pointer starting from a location derived from a global counter and clears each pointed-to long word until the counter reaches its initial value, effectively zeroing a block of memory.
- `advance_screen_counter`: Decrements a counter and shifts two screen-position values by a page, then clears a block of entries from a table.
- `manage_screen_transitions`: Manages a set of pending screen-transition tasks by decrementing counters, adjusting screen positions, and clearing the relevant memory blocks, while also writing a numeric value as ASCII characters into a buffer.
- `process_buffer_dispatch_2`: Converts a small counter value into its decimal digit characters and stores two register values into global variables, adjusting one when a flag bit is set.
- `clear_memory_and_reset_globals`: Clears a 512-byte region by writing zero words at consecutive addresses, then resets four global variables to zero or all-ones values.
- `rotate_table_bytes_with_input`: Walks four entries of a global table, combining each stored byte with a running value and an input byte, then shifts the values through the loop.
- `init_table_blocks_flag_2`: It invokes another routine to obtain a value, then stores that value into a block of words in a table.
- `compare_and_update_tables`: It compares two tables of words across a fixed number of entries, and for matching pairs it either stores a fixed value or decrements a value in a target table.
- `compare_and_update_tables_2`: Stores a decremented word read from one table into another table, then scans matching entries and writes a fixed word into a third location whenever a comparison condition holds.
- `setup_blitter_copy_rows_7`: Increments a global counter and, based on its value, looks up a table entry that describes a block of data.
- `apply_bitmask_table_update`: Reads a width and height from a source entry, then processes a block of paired short values, masking and setting words in a lookup table based on two input bitmasks.
- `dispatch_and_clear_palette_table`: Dispatches on the value of a global variable to load specific word constants into a table and invoke comparison and palette-copy routines with different arguments for each case.
- `fill_palette_slots`: Writes a register value into ten consecutive word slots of a table indexed by a base pointer.
- `initialize_palette_setup_2`: Sets up a palette by writing color entries to a screen data table and configuring the two-player palette slots, then compares two byte streams while advancing their pointers and invoking a helper routine when they differ.
- `compare_and_advance_pointers_3`: Compares two byte streams pointed to by incrementing pointers, advancing both while the bytes match and invoking a helper routine when they differ until an end marker is reached.
- `copy_length_prefixed_stream`: Copies a length-prefixed byte stream from one global buffer to another, advancing past the length byte once the source and destination pointers have caught up to the remaining count.
- `copy_palette_words`: Clears a global variable, stores a value into another global, invokes a comparison routine, then copies two words from one location into a second location.
- `store_pointer_and_copy_words`: Stores a routine address into a global variable, stores a value into another global, invokes a comparison routine, copies two words into a second location, then leaves a global variable unchanged.
- `compare_and_advance_pointers`: Compares two bytes pointed to by incrementing pointers, advancing both when they match and calling another routine when they differ until a counter is exhausted.
- `compare_and_advance_pointers_2`: It compares two byte streams pointed to by incrementing pointers, advancing both while the bytes match and invoking another routine when they differ.
- `mask_index_and_adjust_short`: Combines an input with a mask to index a table, then either clears a byte and sets a flag, adjusts a short through a pointer, or looks up a slot, before looping decrementing table entries.
- `branch_adjust_short_table`: Branches on a flag to either clear a byte and set a flag or to adjust a short through a pointer and store it, then loops decrementing table entries.
- `adjust_short_and_loop_table`: Clears a byte and sets a flag when a value is negative, otherwise decrements or increments a short through a pointer and stores the adjusted result, then loops decrementing table entries.
- `decrement_and_loop_table`: Increments a short stored through one pointer and decrements a short stored through another pointer, then repeatedly decrements entries in a table until a condition stops the loop.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `rotate_table_bytes_with_input_2`: It invokes another routine to rotate bytes in a global table using an input byte and a running value, then returns without performing any further operations.
- `setup_pointers_and_normalize`: Writes several fixed values into memory locations addressed through supplied pointers offset by a register value, then performs the same six-word global-table nibble reduction as another routine and stores a fixed value into a global variable.
- `process_command_sequence_2`: Clears a flag in a hardware register and, if a certain condition is not met, loads a command index from a lookup table and processes a sequence of commands stored in a data block.
- `process_command_sequence`: Loads a value from a lookup table using an index, then walks through a sequence of commands stored in a data block.
- `scatter_copy_record_fields`: Clears a record selected by a register value and marks it with sentinel flags, then copies three consecutive words from a source record into three scattered slots of that record.
- `setup_blitter_copy_rows_4`: Reads a pointer and two lengths from a table indexed by the input value, then adjusts those pointers and lengths using signed bounds checks.
- `clear_global_word`: Clears a global word by writing zero to it and then returns.
- `store_wrapped_index_and_return`: Computes a wrapped index from an input value, stores its low nibble into a global variable, and returns the value shifted right by three bits.
- `adjust_record_slot`: Reads a slot value from a table indexed by a masked input and, when that value is negative, writes the maximum word into the corresponding structure slot.
- `adjust_slot_values`: Reads a slot value from a table indexed by a masked input, and if that value is negative writes the maximum word into a second structure.
- `set_slot_to_max_value`: Writes the word 65535 into a slot of a structure selected by adding an input value to a fixed offset.
- `adjust_slot_counter_and_flag`: Adjusts a slot's counter by a computed amount and sets a flag based on the slot's current value, then updates a related slot depending on its marker and table contents.
- `adjust_flag_slot_value`: Decrements a counter stored through a pointer and adjusts a flag slot based on its current value and related structure fields.
- `select_flag_slot_value`: Reads a flag word from a structure and, depending on its value, stores one of two constant words into a nearby structure slot.
- `copy_global_into_slot`: Copies a global word into a structure slot chosen by subtracting a fixed offset and adding an input value.
- `update_record_slot`: Loads a value from a global variable table, calls another routine to obtain an index, and uses that index to either clear and adjust a record's fields or add a value to a slot, applying a cap and special handling when the result is large.
- `clear_structure_slot`: Writes zero into a structure slot selected by subtracting a fixed offset and adding an input value.
- `decrement_slot_value`: Invokes another routine to obtain an index, then uses it to locate a slot in a structure and decrements that slot's value by one.
- `normalize_and_update_slots`: Checks a flagged field and forces it to a sentinel when set, stores a value into one slot, rounds another short up to the next multiple of eight, and clears a fourth slot.
- `store_conditional_flag_value`: Tests two bits of an input value and, when set, stores either zero or a fixed flag into a structure slot selected by an input offset.
- `apply_command_updates_3`: Reads a command code and its parameter from registers, looks up a matching entry in a command table, and applies the corresponding update to a data table by adding or subtracting values, copying fields, or adjusting pointers and lengths with signed bounds checks.
- `apply_command_updates_2`: Reads a command code from a register and applies the matching update to a data table, adding or subtracting values, copying fields, and adjusting a table index, with several commands dispatching to another routine.
- `apply_command_updates`: Reads a command code from a register and applies the matching update to a data table, adding or subtracting values, copying fields, and applying masks.
- `dispatch_special_commands_3`: Reads a command code from a register and applies the matching update to a data table, adding or subtracting values, copying fields, masking bits, and calling helper routines for special command types.
- `dispatch_special_commands_2_2`: Reads a command code from a register and dispatches to specialized handlers that modify a data table.
- `dispatch_special_commands_2`: Reads a command code from a register and dispatches to specialized handlers that modify a data table.
- `dispatch_special_commands`: Dispatches special commands based on a command code stored in a register, updating game state through a series of routine calls and adjusting stored values in a table.
- `write_constant_slots`: Writes the constant value 65528 into one structure slot and the constant value 65535 into another structure slot, then returns.
- `handle_special_commands`: Reads a slot record and adjusts a stored value based on a command code, then dispatches to a command handler that updates game state through a pointer.
- `process_special_commands`: Handles special command processing by reading a slot record, invoking a command dispatcher, and then manipulating game state through pointer-based writes and global variable updates.
- `dispatch_command_marker`: Dispatches a command index by looking up a slot record and branching on its type, storing a clamped value into a table and then reading a pointer and two lengths to copy a block of data.
- `setup_blitter_copy_rows_9`: Reads a pointer and two lengths from a table indexed by an input value, then adjusts those pointers and lengths using signed bounds checks before handing them off to blitter copy routines.
- `setup_blitter_copy_rows_8`: Reads a pointer and two lengths from a table indexed by an input value, then adjusts those pointers and lengths using signed bounds checks.
- `handle_command_marker`: Handles a special command index by storing a marker value into a slot record and then loading and dispatching another command index, clamping and adjusting pointer and length values read from a table.
- `process_command_index`: Loads a command index and its parameters from a structure, then dispatches based on the index: rejects out-of-range or invalid values, stores the command into a table while updating counters and state fields, and clears or sets flag bits in a companion table.
- `apply_command_sequence`: Validates a command index against a small range, then stores a byte into a command table and updates a per-command counter and state field based on the current index and a lookup value.
- `process_command_sequence_2_2`: Adds a byte to a stored value and then walks through a command table, applying offsets and flags to a block of memory based on the current index.
- `process_event_script_state`: Validates entity state fields against a small set of allowed values, then increments a counter or resets it and stores a computed index into a table.
- `apply_event_script_2`: Loads a scripted event table entry using an index and applies its commands to per-entity state fields, decrementing timers, adding to counters and pointers, and storing values into the entity's data.
- `apply_event_script`: Reads a scripted event table using an index and applies its commands to a set of per-entity state fields, updating counters, pointers, and derived values according to each command type.
- `apply_command_marker`: Dispatches on a signed marker value to update two buffers, clamping a value read from a slot record, and restoring or saving state via a stack.
- `process_table_markers`: Reads a sequence of small signed markers from a table and applies each one to values stored in two buffers.
- `restore_slot_marker_state_2`: Restores a value from a slot record into a memory location, adds a marker delta to it with range clamping, and dispatches to other routines based on the marker type.
- `restore_slot_marker_state`: Restores a value from a slot record into a memory location, adds a marker delta to it with range clamping, and dispatches to other routines based on the marker value before restoring the saved return address and continuing.
- `resolve_slot_by_marker`: Loads a pointer from the process vector table, then reads a marker value and uses it to index into a parallel table of slot records.
- `self_modify_code_buffer_2`: Patches its own following instructions by writing new mnemonics into a code buffer, then restores a saved return address and continues.
- `process_command_entry`: Loads a command entry from a table using an index, adjusts a pointer and length by a clamped offset, and bails out when the resulting length is zero or negative.
- `self_modify_code_buffer`: Patches its own following instructions by writing new mnemonics into a code buffer, then restores a saved return address and continues.
- `apply_command_sequence_2`: Validates a command index against a small range, then stores a byte into a command table and updates a per-command counter and state field.
- `process_command_dispatch_2`: Reads a command byte from a table and dispatches to a state handler, updating a running index.
- `process_command_dispatch`: Reads a command byte from a table and dispatches to a state handler, updating a running index.
- `do_nothing_routine`: This routine is empty and performs no operations.
- `setup_blitter_copy_rows_4_3`: Reads a pointer and two lengths from a table indexed by an input value, then adjusts those pointers and lengths using signed bounds checks.
- `store_indexed_word_with_high_byte`: Writes a word to a location derived from a base pointer and an index, combining a passed value with the high byte of an unmodified register.
- `load_add_store_short`: It loads a short from one table using a base pointer and offset, then adds a second value to it when a condition in another table is non-zero.
- `add_byte_mask_store_code`: It reads a byte from a table, adds a carry flag to it, and stores the result back.
- `add_carry_to_table_byte`: Reads a byte from a table using an index, adds a carry flag to it, and stores the result back through the same pointer.
- `store_indexed_clear_small`: Writes the constant five into a global variable, clears a word at a base address offset by an index, and when that index is below four also clears a word at another base address and records the constant five into a global variable.
- `store_fourteen_clear_small`: Stores the constant fourteen into a global variable, and when an index is below four clears a word at a base address offset by that index and records the constant fourteen into a global variable.
- `clear_indexed_words_when_small`: Clears a word at an indexed location, and when the index is below four also clears a word at another indexed location and records a routine address into a global variable.
- `store_two_values_to_global`: Loads a 32-bit value from a fixed address into a register, stores that value into a global variable, loads another 32-bit value from a fixed address, and stores that value into the same global variable.
- `validate_indexed_offset`: Computes a signed difference between two indexed values, then scans a table of words for a flagged entry whose bounds stay within range, adjusting a stored value and reading a byte from another table when the entry matches.
- `scan_table_validate_offset_2`: Checks whether a difference stays within signed bounds, then scans a table of words for a flagged entry.
- `scan_table_validate_offset`: Reads a byte from a global table, scans a table of words for an entry marked with a flag within a value range, and once a matching slot is found loads a word from an indexed location into a global variable.
- `scan_table_and_validate_offset`: Scans a table of words looking for an entry marked with a specific flag, then computes a difference between two indexed values and validates the result against a global table before reading a byte from a second table.
- `adjust_slot_value_by_table`: Checks whether a slot at a given index holds a specific marker, then scans a parallel table of slots to find one that satisfies a set of conditions.
- `find_and_adjust_slot`: Scans a table of slots looking for one that matches a set of conditions, then adjusts a value using a passed index and clamps it against table-derived limits.
- `clamp_and_scan_table_2`: Computes a signed difference and clamps it to zero when negative, then scans a table of words for a valid slot.
- `clamp_and_scan_table`: Clamps a computed difference to zero when negative, scans a table of words for a flagged entry, and once a valid slot is located loads a word from an indexed location into a global variable.
- `find_matching_entry_and_load_pointer`: Iterates over a table of words to find a matching entry, computes a signed difference between indexed values, and once a valid slot is located loads a pointer from a table and derives a length from two indexed words.
- `find_slot_and_adjust_value`: Walks a table using an incrementing index until it finds an entry matching a set of conditions, then reads a value from a parallel table through a pointer and stores it into a global variable.
- `recursively_increment_and_clear_data`: Checks a value in a table and, if non-zero, increments a byte at an offset and enters a recursive loop to decrement a counter.
- `increment_byte_and_loop_counter`: Increments a byte at an indexed location, then repeatedly calls another routine while decrementing a global counter until it reaches zero.
- `store_three_words_advance`: It stores three incoming values into consecutive words at a target address, setting the middle word to a fixed bit pattern.
- `tiny_ori_7`: Small routine of 1 instruction that merges 56 into d1.
- `store_three_words_plain`: It stores three incoming values into consecutive words at a target address without altering any of them.
- `loop_call_blitter_setup`: Sets a global variable to a fixed pattern, then repeatedly calls two other routines until a pointer reaches a target address.
- `init_table_blocks_flag`: It initializes a block of words in two tables to fixed values, including a flag and a large constant.
- `tiny_ori_8`: Small routine of 1 instruction that merges 1 into d1.
- `normalize_string_carry`: It selects a destination table based on a condition, storing a code into a global variable.
- `process_string_counters`: Decrements two countdown counters and then walks a null-terminated string, invoking a per-character routine for each non-space byte and a different routine for each space byte while adjusting a second counter.
- `loop_index_reset_sum`: Compares a clamped value against a stored global and, when they differ, updates it and runs a small loop that decrements an index, resetting it to zero when negative while calling another routine and returning a sum.
- `loop_with_index_reset_and_sum`: Runs a small loop that decrements an index, resetting it to zero when it goes negative, calls another routine on each pass, and returns that routine's result plus one.
- `save_and_clear_global_word`: Saves the current value of a global word, clears it to zero, runs another routine, then restores the saved value.
- `adjust_record_slot_2`: Reads a slot value from a table indexed by a masked input and, when that value is negative, writes the maximum word into the corresponding slot.
- `handle_indexed_table_update`: Computes an index from a value in a table and, when a paired entry is zero, branches based on that index to read and modify several related memory entries, sometimes looping over a table to record results.
- `adjust_indexed_scan_table_2`: Validates an index against a bound and retrieves another index from a routine when a value is non-negative.
- `store_max_word_then_call`: Stores the value six thousand five thousand five hundred thirty-five into a word at an indexed location and then calls another routine.
- `adjust_indexed_scan_table`: Checks whether an indexed word equals a constant, obtains an index from another routine, and when a global variable is nonzero adjusts two indexed words and scans a small table.
- `adjust_indexed_words_and_scan_table`: Calls another routine to obtain an index, returns early if a global variable is zero, then adjusts two indexed words and scans a small table of shorts for a negative entry to update several related locations.
- `find_negative_adjust_copy`: It scans a small table of shorts looking for a negative entry.
- `initialize_score_tables_2`: Iterates over a table of words, clearing or incrementing each entry through bit operations and calls to another routine, and records a marker into a global variable.
- `scan_entries_lookup_offset`: It walks backward through a table of entries, comparing position and offset values against a global table.
- `manage_flagged_table_entry`: Checks a byte in a global table, incrementing it and setting a global when it is not a specific marker, and otherwise records a marker, copies a global value, clears several indexed locations, and selects one of two values for another global.
- `store_differences_set_flag`: Clears a word at an indexed location, stores computed differences into consecutive global words, and records a constant into a global variable depending on a global flag.
- `set_global_to_thirty_eight`: Assigns the value thirty-eight to a global variable and returns immediately.
- `dispatch_table_indexed`: Reads a 32-bit value from a jump table indexed by a small signed number, multiplies that index by four, and jumps to the routine stored at the resulting table slot.
- `jump_table_dispatch`: Reads a 32-bit entry from a jump table indexed by a small signed number and jumps to the routine stored there.
- `update_slot_from_counter`: Walks a table of short slots looking for a negative counter, then updates a stored offset, a nearby flag word, and a derived value before returning.
- `compute_struct_offset_and_flags`: Computes a signed offset from a base register and stores it into a slot of a structure, then clears a nearby flag word.
- `update_indexed_word_flags`: Adds a global value to a series of consecutive words, then inspects each entry's low nibble to decide whether to increment it or reset it to a maximum pattern.
- `copy_and_initialize_structures`: Calls another routine to obtain a value, then copies several words from a source structure into offset locations of a destination structure while adding the returned value.
- `advance_phase_timer`: Maintains a countdown counter and, once it reaches zero, advances a phase index through a small table of states, invoking a routine chosen by that index and updating associated per-state fields.
- `manage_phase_state_machine`: Manages a multi-phase state machine that initializes game tables and, once a setup loop completes, advances through phase states and dispatches to the routine for the current phase.
- `tiny_ori_9`: Small routine of 1 instruction that merges 20 into d1.
- `loop_copy_table_entries`: Runs a loop that decrements an index and a counter, calling another routine while an index stays below a threshold and a global value exceeds a limit, then advances the global value.
- `copy_indexed_pair`: Reads a four-word entry from a table indexed by an offset, then copies two of its words into consecutive four-byte slots pointed to by the first word of that entry.
- `mix_and_copy_table`: Combines bytes of a global table with a running value and an input byte, then copies pairs of words from an indexed table into consecutive four-byte slots.
- `setup_dma_and_save_registers`: Writes a constant into a global variable, then writes a value into the video controller register that enables DMA channels, and calls another routine to save incoming register values and jump to a fixed address.
- `clear_and_normalize_tables`: Writes a fixed pattern into a global variable and a location derived from a passed-in value, then zeroes a block of memory when a global does not match an expected value.
- `normalize_table_nibbles`: Walks six consecutive words of a global table, reducing each nibble of every word to the largest nonzero nibble it contains, then rewrites each word with that nibble replicated in all four positions, and finally stores a fixed value into a global variable.
- `execute_scheduled_string_update`: Runs a scheduled one-time action by decrementing a timer and returning early until it expires.
- `normalize_string_carry_2`: Selects a destination table based on a condition and stores a code into a global variable, then increments a string until a character falls below a threshold.
- `process_buffer_by_routine`: Reads a flag byte and, when it indicates an active state, sets up a counter and a limit, then repeatedly reads a null-terminated buffer and calls one of two routines for each byte depending on whether it is a space.
- `dispatch_byte_buffer`: Initializes a counter and a length value, then repeatedly reads a null-terminated byte buffer, dispatching each byte to one of two routines based on whether it is a space, and advancing through the buffer until a terminator is reached.
- `run_startup_sequence_2`: Runs a one-shot startup sequence when a countdown reaches zero, enabling interrupts and DMA channels and initializing audio and score tables.
- `compare_and_select_status`: Compares a global counter against zero to choose between two branches, then computes the absolute difference between a live value and a stored value and compares it against a threshold to select a small numeric code.
- `run_startup_sequence`: Runs a one-shot startup sequence when a countdown reaches zero, enabling interrupts and DMA channels and initializing audio and score tables.
- `setup_blitter_copy_rows_5`: Clamps a source pointer and length against signed bounds, then programs a series of blitter copy operations using the computed address.
- `find_matching_record`: Walks backward through a table of three-word records, comparing each record's fields against a pair of values, and stores the offset of the first matching record into a fixed location or a sentinel when none match.
- `adjust_matching_records`: Walks forward through a table of three-word records and, for each record whose first field is positive and whose last two fields sum below a threshold, adds a fixed value to the middle field.
- `scan_for_negative_entry`: Walks forward through a table of six-word records and returns as soon as it finds a record whose first field is negative, otherwise exhausting the fixed number of records.
- `scan_for_zero_entry`: Walks forward through a table of two-word entries and returns as soon as it finds a zero entry, otherwise exhausting the fixed number of entries.
- `copy_table_chunk`: Sets several flag bits in hardware and global registers, then indexes a table using a passed value to compute a pointer and length, skipping the copy when the source is empty or out of range.
- `setup_clipped_blitter_row`: Looks up a per-frame entry in a table and, based on its value, either clears some state and returns, or adjusts a source pointer and length to clip a row against a range before filling a blitter-copy structure with the corrected values.
- `setup_clipped_blitter_copy`: Reads a descriptor from a table and adjusts a source pointer and length to clip it against a range, then fills a blitter-copy structure with the resulting source, destination, and row stride.
- `setup_blitter_copy_rows_4_2`: Looks up a blitter-copy descriptor in a table using the input value, then adjusts the source pointer, length, and destination offset using signed bounds checks.
- `adjust_and_store_offset`: Computes a pointer from a register offset and a table index, decrements the short stored there, and writes the values 60, 0, 0, 0 into four nearby short locations.
- `store_registers_and_jump`: Copies fifteen incoming register values into a contiguous block of global variables, then jumps to a fixed address.
- `tiny_ori_10`: Small routine of 1 instruction that merges 64 into d0.
- `dispatch_queued_value`: Subtracts a global from 64, moves a pending value into a queue slot, and dispatches based on several status flags to load data from a table.
- `route_pending_values`: It swaps the sign of a stored counter and then moves pending values between two small queues, routing some of them through a lookup table indexed by the value.
- `process_item_queues`: It swaps the sign of a stored counter and then drains pending items from two queues, dispatching each to a per-item routine.
- `dispatch_queued_input`: Processes a queued input command by decoding its type and target, then dispatches it to the appropriate handler table while storing the result.
- `process_indexed_table_load`: Loads a 32-bit value from a table indexed by a stored number and writes it to a global variable, then clears that number.
- `apply_queued_move_values`: It moves a pending value into a holding slot and then applies queued values from two separate slots to a per-entry table using the slot's index as a multiplier.
- `dispatch_queued_sound_command`: It subtracts a value from 64, then stores pending command data into a couple of slots depending on status flags.
- `load_indexed_value`: Reads a four-byte value from a table indexed by an incoming register offset and returns it.
- `dispatch_queued_input_2`: Processes one queued command by decoding its type and target, then dispatches it to the appropriate handler table while storing the result.
- `process_command_counter`: Copies a value through a pointer, stores a pointer and a global address into a structure, then decrements and clamps a stored counter based on a command code.
- `return_immediately`: Contains no instructions and simply returns to its caller.
- `process_queued_command`: Updates a linked list node by copying fields from a source record, then inspects a stored command type to adjust a counter and pointer.
- `dispatch_queued_command`: Pops the next queued command from a circular buffer, advancing the read pointer, then decodes its type and target and dispatches it to the appropriate handler while storing the result.
- `store_and_dispatch_command`: Copies a four-byte value from the memory pointed to by the first pointer into a field of the structure addressed by the second pointer, then advances that pointer and processes a queued command based on a type byte.
- `store_pair_and_dispatch`: Copies two four-byte values from consecutive memory locations into fields of a structure, advances the source pointer, and then processes a queued command based on a type byte.
- `reset_audio_channel_defaults`: Writes fixed default values to four sets of channel fields, with period entries taken from a table, then clears the shared state variables.
- `initialize_channel_tables`: Clears a global block of 205 words, loads four pointers from a table selected by an index, points eight channel fields at the same block, and writes six fixed index values.
- `is_empty_routine`: This routine is empty, containing no instructions and performing no operations on data or registers.
- `mark_empty_routine`: This routine contains no instructions, so it performs no operation itself; it exists only as a jump target that another routine branches to.
- `tiny_ori_ori_ori_2`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `copy_byte_via_registers`: Loads a byte from the memory location pointed to by the address register and stores that byte into the memory location pointed to by the second address register.
- `increment_and_index_byte`: Loads a 32-bit value from a global variable into a register, adds a constant to it, stores the result back to the same global variable, then loads a byte from a table indexed by a register and stores that byte through a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `enable_interrupts_and_dma`: Enables the CPU interrupts and the DMA channels, then stores a constant into a global variable and overwrites several word-sized fields of a structure with fixed values.
- `read_intreq_select_flag`: Reads the interrupt request register and inspects three of its bits to select one of two constant values, storing that value into a global variable and incrementing a shared counter when appropriate.
- `set_flag_and_increment`: Stores a fixed value into a structure slot and increments a shared counter before returning.
- `set_flag_value`: Stores a fixed value into a structure slot before returning.
- `init_and_decrement_counters`: Initializes a set of global state variables, then repeatedly decrements two countdown counters and branches based on the result of a called routine.
- `toggle_flag_and_decrement`: Sets a flag, writes a value, calls another routine twice, then decrements two shared counters before returning.
- `conditionally_rotate_and_store`: Checks a flag bit in a structure and conditionally rotates a global variable, then zeroes a distant memory field and stores a constant word into another structure field.
- `compute_status_byte`: Transforms a status register value by testing several flag bits and accumulating a computed result through shifts and additions before returning the derived byte.
- `tiny_ori_11`: Small routine of 1 instruction that merges 2 into d0.
- `tiny_ori_12`: Small routine of 1 instruction that merges 178 into d0.
- `double_and_store_values`: Doubles sixteen short values into two destination tables using shifted writes, then advances a table pointer and returns a constant.
- `update_channel_tables`: Copies a word into five consecutive four-word slots and decrements a counter, then compares nibbles of two tables and adjusts the second table toward the first by one or sixteen.
- `shifted_subtract_store_loop`: Repeatedly subtracts a value from a word pointed to by a register, then shifts two working values left by four bits while decrementing a counter.
- `swap_and_store_globals`: Reads four consecutive global words, toggles a flag, swaps paired values, and writes the relocated values into two adjacent memory locations.
- `store_function_result`: Invokes a function to read a value and stores the returned word into a fixed offset of a structure.
- `return_immediately_2`: The routine executes four instructions and then returns.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_set_bits`: Writes a value into a function and then checks a value in a register.
- `calculate_and_store_values`: Performs a series of calculations on values in registers and stores the results in various memory locations.
- `clear_memory_block_4`: Clears a block of memory by writing zeros to a series of locations.
- `store_value_and_call_routines`: Retrieves a value from a function and stores it in memory, then calls two other routines.
- `reset_system_state`: It copies a value from a routine call into a fixed offset, then shifts a small linked list of pointers stored in a global structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine`: This routine is empty and does nothing.
- `process_table_byte`: Loads a byte from a table through a pointer and stores a computed value through another pointer, decrementing the table pointer when the byte is 255.
- `reset_counters_and_tables`: Decrements a counter stored at a pointer, then repeatedly clears a small table and writes a series of values into a global structure through a helper routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `do_nothing_2`: The routine contains no instructions and performs no operations.
- `do_nothing_3`: This routine performs no operations and returns immediately without modifying any data or registers.
- `do_nothing_4`: The routine contains no instructions and does nothing.
- `do_nothing_5`: The routine contains no instructions and returns immediately without performing any operation.
- `do_nothing_6`: The routine contains no instructions and does nothing; it simply returns.
