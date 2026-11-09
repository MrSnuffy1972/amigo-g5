# Game 37 (BloodMoney v1.4 0302): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00037). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4407059 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 155 raw-word lines |
| Functions named and described | pass, 109 of 109 named (0 author symbols), 109 of 109 described (4 by rule, 105 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 356 of 367 (97.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2048 bytes, 4 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2378 of 2874 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 296927 |
| code | 10654 |
| data | 3735527 |
| padding | 71 |
| unused | 363880 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 151 | 12 |
| logged disk load from Disk.1 | 1 | 1532 | 49 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 27 | 1 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 841 | 42 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| Disk.1 bytes 489728-491776 (code never loaded by a reference run) | 1 | 320 | 4 |

## Cases handled

- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.1: raw disk image (979600 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: raw disk image (979600 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302.hdf: hard-disk image (AmigaDOS OFS, 4736 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree BloodMoney_v1.4_0302_hdf, which is recovered file by file; no block outside the filesystem holds data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 5 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.1: bytes 489728-491776 look like code no run loaded: recovered position-independently (load address unknown)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.1: 6 logged loads cover 305848 bytes of the image
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.1: chunks 1922-1976 (bytes 492032-506112) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.29)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.1: chunks 1980-2589 (bytes 506880-663040) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.69)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.1: chunks 2591-3213 (bytes 663296-822784) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.72)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.1: chunks 3221-3385 (bytes 824576-866816) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.21)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.1: chunks 3387-3574 (bytes 867072-915200) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.42)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.1: chunks 3581-3584 (bytes 916736-917760) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.40)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 0-61 (bytes 0-15872) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.69)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 72-78 (bytes 18432-20224) outside the filesystem hold data, not code (opcode density 0.008, entropy 3.91)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 80-90 (bytes 20480-23296) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.20)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 96-165 (bytes 24576-42496) outside the filesystem hold data, not code (opcode density 0.015, entropy 7.01)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 169-263 (bytes 43264-67584) outside the filesystem hold data, not code (opcode density 0.006, entropy 4.94)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 267-287 (bytes 68352-73728) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.69)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 290-314 (bytes 74240-80640) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.61)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 460-1118 (bytes 117760-286464) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.38)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 1124-1182 (bytes 287744-302848) outside the filesystem hold data, not code (opcode density 0.007, entropy 3.52)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 1186-1844 (bytes 303616-472320) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.24)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 1850-1908 (bytes 473600-488704) outside the filesystem hold data, not code (opcode density 0.011, entropy 3.94)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 1913-2524 (bytes 489728-646400) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.07)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 2577-2635 (bytes 659712-674816) outside the filesystem hold data, not code (opcode density 0.019, entropy 4.19)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 2639-3265 (bytes 675584-836096) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.22)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 3303-3361 (bytes 845568-860672) outside the filesystem hold data, not code (opcode density 0.006, entropy 4.34)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 3366-3412 (bytes 861696-873728) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.16)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 3414-3460 (bytes 873984-886016) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.02)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 3462-3505 (bytes 886272-897536) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.04)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 3507-3512 (bytes 897792-899328) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 3514-3569 (bytes 899584-913920) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.56)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 3571-3643 (bytes 914176-932864) outside the filesystem hold data, not code (opcode density 0.005, entropy 2.92)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 3645-3650 (bytes 933120-934656) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.88)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 3652-3655 (bytes 934912-935936) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- BloodMoney_v1.4_0302/BloodMoney_v1.4_0302_hdf/BloodMoney/Disk.2: chunks 3657-3681 (bytes 936192-942592) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.25)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `wait_for_port_match` (Disk.1 +an address loaded at an address (12400 bytes)): Toggles bits in a global port image and repeatedly calls another routine until the port value matches a passed value. Kind: CIAB parallel-port disk match (wait_for_port_match). Pass path: Loop exits when in_D0b == u1__d0_291E; the matching branch sets DAT_00bfd100 /= 1 (bit 0) and increments u1__d0_291E, then returns to caller sub_2360 which arms the disk controller (an address(a6) = an address, sets an address(a6) = an address)..

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `patch_code_and_dispatch`: Sets a bit in a global variable, saves an incoming value, and writes jump-and-move instruction words plus routine addresses into several structure locations to patch code, then invokes a routine stored in that structure.
- `wait_flag_then_enable_dma`: Spins until a status byte's low bit becomes set, then spins until it clears, and finally stores a control word through a pointer to enable several channels.
- `wait_for_counter_threshold`: Repeatedly reads a value from a helper until it is at least 150, then returns.
- `decompress_byte_pattern`: Waits for a counter to reach 100, then repeatedly decompresses a byte pattern by subtracting the number of set nibble groups and stores the result back until no bits remain.
- `patch_code_and_start_blit`: Writes jump and move instruction words into several structure locations to patch code, then invokes a routine stored in that structure.
- `toggle_low_bits_global`: Toggles the low two bits of a global byte using a constant mask and stores the result back.
- `start_blit_and_wait`: Writes a 16-bit value to a hardware register, then repeatedly reads a status register until its busy bit clears, blocking until the operation finishes.
- `apply_conditional_code_patches`: Selects a jump target based on a passed-in value, calls a routine stored in a structure, then patches code locations with branch instructions when values match fixed constants.
- `copy_byte_into_word_field`: Copies a byte from one structure into a 16-bit field of another, zero-extending it.
- `clear_global_variable`: Assigns zero to the same global variable three times in a row.
- `replicate_global_byte_pattern`: Reads a global byte and returns it replicated across the low three bytes of a 32-bit result.

### logged disk load from Disk.1

- `decrement_and_validate_value`: Decrements a word value by three and checks the result, then reads a pointer chain to obtain a value that is validated and clamped against a minimum before being stored.
- `load_byte_increment_pointers`: Loads a byte from a table via a pointer register and stores it into a byte at a pointer register.
- `empty_routine`: This routine is empty, containing no instructions and performing no operations on any data.
- `setup_video_and_audio`: Initializes the video and sound hardware by clearing a large work buffer, configuring DMA and interrupt registers, and setting up a list of blitter command structures that reference a repeating image block, then repeatedly invokes a command-processing routine.
- `process_table_records`: Walks a table of records, decrementing counters and translating type codes, then computes and stores adjusted coordinate values into a parallel structure while linking records together.
- `iterate_and_call_records`: Iterates over a list of records, searching for a matching field and invoking a routine stored in each matching entry until either the list is exhausted or a limit is reached.
- `program_blit_commands`: Reads fields from a command structure, adjusts pointers and accumulators, and programs a series of blitter command blocks with computed offsets and sizes.
- `setup_blit_area_and_start`: Sets the blitter's first and last word masks, computes an area size from two inputs, writes modulo and offset registers, and calls helpers to start the blit.
- `wait_for_busy_flag_clear`: Polls a status byte's busy flag until it clears, then returns.
- `setup_blit_area_from_pointer`: Programs the blitter's mask and modulo registers from a computed area size, then fills a loop with successive transfer descriptors and calls a helper to launch each blit.
- `manage_counters_and_call`: Decrements two counters, calls a helper routine, and then branches based on whether a third counter reaches a specific value.
- `clip_rectangle_to_bounds`: Clips a rectangle's horizontal and vertical positions against fixed screen bounds, advancing values in sixteen-unit steps.
- `clear_work_buffer`: Calls a helper for the buffer address, then writes zero to eleven thousand consecutive longwords to clear a large work area.
- `wait_for_frame_counter`: Waits until a counter reaches four, then resets it to zero.
- `swap_bitplane_pointers`: Flips a global flag and selects one of two 32-bit values based on its state, then writes that value into four consecutive 16-bit slots of a table, advancing each slot by a fixed stride.
- `return_if_global_set`: Returns early when a global variable holds a nonzero value, otherwise falling through to return.
- `handle_tick_interrupt`: Acknowledges an interrupt by writing an acknowledge value to the interrupt request register, then increments a shared tick counter that a wait routine counts.
- `decode_signed_sample_stream`: Reads words from a packed stream through a helper and converts their low bits into signed values, storing them in two output locations per pass until the loop counter reaches zero.
- `advance_random_seed`: Adds a constant to the low word of a stored 32-bit value, shifts and exclusive-ores it, then swaps its two halves to produce the next seed.
- `plot_pixel_four_bitplanes`: Verifies a point lies within a 320 by 200 area, then sets the matching colour-index bit in each of four bitplanes whose rows are a fixed distance apart.
- `cycle_table_bytes`: Walks one hundred words in a table after calling a helper, adding eight to each value and keeping only its low byte.
- `set_bounds_and_return`: Writes fixed limit values into four fields of a structure, mirroring the void bounds setup but returning a modified byte.
- `set_bounds_limits`: Stores fixed limits of 63 and 400 into four fields of a structure, forming the void form of the limits setup.
- `set_audio_channel_pair`: Stores a start address and length into audio channels 0 and 2, then writes a DMA control word that enables them to start a pair of samples.
- `set_hardware_parameters`: Stores several constant values into a table using a pointer held in a register.
- `setup_dual_sample_channels`: Writes a 32-bit start address and a 16-bit length into two channel slots of a sound table, then stores a DMA control word that enables both channels.
- `wait_audio_sample_end`: Acknowledges the audio interrupt bits, waits until the first audio channel signals an interrupt, then acknowledges again and turns off DMA for two channels to stop a sound once its sample has finished.
- `wait_for_status_bits`: Spins until at least one of two status bits in a global variable is clear, then returns those two bits inverted.
- `manage_counter_reset`: Sets a global flag, increments a counter and decrements another counter, then either resets the counter to six or invokes another routine before re-arming the flag.
- `initialize_global_counters`: Initializes a global variable by storing the value six into a one-byte field and setting another global variable to all ones.
- `execute_infinite_loop`: Executes an empty loop that never terminates, so control never returns to the caller.
- `run_infinite_loop`: Loops forever by repeatedly calling another routine and discarding its result, never returning to the caller.
- `expand_bitmask_stream`: Expands a stream of sixteen-bit mask words into output words: each set mask bit copies the next source word, and each clear bit writes zero.
- `pulse_port_bit_then_delay_2`: Runs a hardware sequence, clears a bit in a CIAB port image, then runs another hardware sequence before returning.
- `execute_hardware_sequence`: Writes a sequence of values to hardware registers, then spins a countdown loop until a status bit is set before clearing the flag and returning.
- `search_disk_sync_words`: Scans a buffer of longwords for a disk sync pattern, counts the data words that follow, and stores a result flag depending on whether the count is below a threshold.
- `verify_block_checksums`: Repeats a disk read into successive buffers, then sums the words of each block and compares that sum with a following word, advancing by the block size for the requested number of blocks.
- `decode_mfm_words_to_buffer`: Fills a destination buffer from a source table by combining alternate bits with a mask and exclusive-oring the result with a supplied value.
- `read_track_blocks_until_marker`: Repeats a disk read into successive buffers, starting each DMA transfer with the same length word, until the buffer's first longword equals a fixed marker value, then stops the DMA.
- `acknowledge_disk_block_done`: Waits for the disk block-done interrupt bit to be set, then writes the interrupt acknowledge for that bit.
- `delay_then_wait_port_bit`: Runs a fixed countdown of three thousand iterations, then spins until bit five of a global variable that holds the CIA port A status is clear.
- `wait_for_port_match`: Toggles bits in a global port image and repeatedly calls another routine until the port value matches a passed value.
- `wait_drive_status_then_clear`: Spins calling another routine while a status bit stays set, then clears the global flag before returning.
- `wait_for_status_bit_set`: Spins until bit four of a global status byte is set.
- `pulse_port_bit_then_delay`: Clears a bit in a global port image, runs a fixed countdown that clears another bit on each pass, then calls one further routine.
- `set_port_bit_with_delay`: Sets bit seven of a global variable, runs a fixed countdown of one hundred steps that clears bit three, then sets bit three.
- `fade_palette_toward_target`: Repeatedly steps each colour word in a table toward a target, adjusting each 4-bit component until the entries match, calling helpers between passes.
- `fade_palette_entries_down`: Repeatedly steps each colour entry's three 4-bit components down by one until all entries reach zero, calling helpers between passes.
- `process_string_uppercase`: Reads a character from a string, converts it to uppercase, and stores the result through a pointer.

### logged disk load from Disk.1

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_string_and_clear_globals`: Copies a null-terminated string from a source buffer into a global variable, stopping early if the terminator is reached, then fills a fixed-size block of memory with a computed value and resets several global fields before returning.
- `advance_and_link_records`: Walks an array of six-word records, decrementing a counter in each until it reaches zero, then applies a transformation to the following record by adding stored offsets to its coordinates and linking it into a chain.
- `iterate_table_slots`: Walks a table of seven-word entries, comparing a running counter against each slot's first value and recursing into another routine for matching slots until the counter is exhausted or the table is fully scanned.
- `build_blit_command_table`: Walks a linked list of command blocks, advancing each pointer and summing a size field, then aborts early if the current block's marker is not the expected value.
- `setup_blit_area_and_start_2`: Initializes a blitter command block by storing the word mask and color registers, then computes a size from two input values and derives width, height, and offset fields.
- `wait_for_bit_clear`: The routine loops until a bit in a byte pointed to by a register becomes clear.
- `setup_blit_area_from_pointer_2`: Initializes the blitter's mask and modulo registers from a computed area size, then repeatedly programs the source and destination pointers and sizes to start a series of blits.
- `tick_dual_countdown_timers`: Maintains two independent countdown timers stored in global memory, decrementing each by one on every invocation, and advances a separate counter by two until a second timer reaches a fixed threshold, at which point it stops.
- `clip_coordinates_to_bounds`: The routine clamps two coordinates against fixed limits, adjusting each in sixteen-unit steps and returning a failure code when out of range.
- `clear_work_area_if_allowed`: Reads a global variable and returns early when it is non-zero, otherwise clears a large block of eleven thousand longwords to zero, freeing a work area.
- `wait_for_frame_counter_2`: The routine waits until a shared counter reaches four, then resets it to zero.
- `swap_bitplane_pointers_2`: The routine toggles a flag and writes four bitplane addresses into a pointer table, stepping each by a fixed block size.
- `check_global_flag`: It reads a global variable and returns early if that value is not zero.
- `handle_tick_interrupt_2`: The routine acknowledges a hardware interrupt by writing a request value, then increments a shared tick counter.
- `decode_signed_sample_stream_2`: Reads words from a stream through a helper and converts their low bits into signed values, storing them in three output locations until the loop counter reaches zero.
- `advance_random_seed_2`: The routine advances a stored 32-bit value by adding a constant to its low word, shifting and exclusive-oring it, then swapping its halves.
- `plot_pixel_four_bitplanes_2`: The routine checks that a point lies inside a 320 by 200 area, then sets the matching bit in each of four bitplanes.
- `increment_table_words`: Loops one hundred times, each time adding eight to a word in a table and masking the result to a single byte, then advances to the next table entry.
- `set_bounds_limits_2`: The routine stores fixed limit values into four fields of a structure, setting two pairs of bounds.
- `set_bounds_limits_void`: The routine stores fixed limits into four fields of a structure, setting two pairs of bounds.
- `set_audio_channel_pair_2`: Stores a start address and length into the registers of two audio channels, then writes a DMA control word that enables both channels.
- `set_hardware_parameters_2`: Stores a set of fixed constant values into distinct fields of a hardware parameter table via a base pointer held in a register.
- `set_audio_channel_pair_3`: Stores a different start address and length into the registers of two audio channels, then writes a DMA control word that enables both channels.
- `wait_audio_sample_end_2`: Acknowledges the audio interrupt bits, waits until the first audio channel signals an interrupt, then acknowledges again and turns off DMA for two channels.
- `wait_for_status_bits_2`: Spins until at least one of two status bits in a global variable is clear, then returns those two bits inverted.
- `initialize_counters_and_call`: Initializes a global variable to a constant value and increments another global counter, decrementing a third counter that resets to a fixed value when it reaches zero.
- `initialize_global_counters_2`: Initializes a global variable to a constant value and sets another global variable to all ones.
- `execute_infinite_loop_2`: Spins forever with no work inside the loop, so control never returns to the caller.
- `spin_infinite_loop`: This routine contains a single infinite loop that repeatedly reads a global variable and compares it against zero, never performing any work and never returning to its caller.
- `expand_bitmask_stream_2`: Expands a stream of 16-bit mask words into output words: each set mask bit copies the next source word, and each clear bit writes zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_table_entry_and_decode`: This routine is a jump-table entry that performs a single absolute jump to a fixed address elsewhere in memory.
- `decode_mfm_words_to_buffer_2`: Fills a destination buffer from a source table by combining alternate bits with a mask and exclusive-oring the result with a supplied value.
- `delay_and_check_bit`: Performs a countdown loop and then waits until a specific bit in a global variable is cleared.
- `set_flag_via_byte`: If the incoming byte is nonzero, it sets two bits in a global variable and then calls another routine before returning.
- `clear_global_variable_2`: Writes a zero value into a global variable before returning.
- `do_nothing`: The routine contains no instructions and returns immediately without reading or writing any data.
- `clear_flag_bit_and_call`: Runs a 100-pass loop that clears bit 3 of a shared byte, then calls another routine and returns.
- `do_nothing_2`: The routine contains no instructions and returns immediately without modifying any data or registers.
- `pulse_flag_bit_low_high`: Runs a 100-pass loop that clears bit 3 of a shared byte, then sets that bit again before returning.
- `set_flag_bit`: Sets a single bit inside a byte located through a computed pointer address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_then_branch`: It returns immediately, then branches to two other routines.

### Disk.1 bytes 489728-491776 (code never loaded by a reference run)

- `configure_audio_channels`: Stores a fixed set of start addresses and lengths into the registers of four audio channels, writes a DMA control word to enable all of them, and references a text string during setup.
- `apply_mode_to_record_field`: It reads a mode nibble from a record: for modes 1 and 2 it adds or subtracts a four-bit step and clamps the result, while for the remaining modes it indexes a small table by another field and stores the value into the record.
- `stub_redundant_pointer_comparisons`: Compares one address register with a shared block address three times in a row and returns each time, so the decoded body performs no real work.
- `load_channel_descriptor_fields`: Copies a long word from a source table into a record, then uses the high nibble of a second byte to index a per-entry table that fills the record's pointer and length fields.
