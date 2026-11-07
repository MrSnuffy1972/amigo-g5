# Game 2422 (caesar deluxe (1993)(impressions)[cr atx][fd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02422). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1809648 top-level bytes in 185 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 27 of 27 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 95 of 95 named (2 author symbols), 95 of 95 described (4 by rule, 89 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1333 of 1333 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3768 of 3768 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1227542 |
| code | 11458 |
| data | 498089 |
| padding | 57199 |
| unused | 15360 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:play | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:assign | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:echo | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:lab | 1 | 2 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:skip | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:CSR.EXE | 8 | 29 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:failat | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:Anthrox | 6 | 251 | 6 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:cd | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:endif | 1 | 2 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:caesar.boot | 1 | 31 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:if | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:play | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:assign | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:echo | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:lab | 1 | 2 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:skip | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:CSR.EXE | 8 | 29 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:failat | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:Anthrox | 6 | 45 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:cd | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:endif | 1 | 2 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:caesar.boot | 1 | 31 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:if | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:bootblock | 1 | 18 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1670 | 47 |

## Cases handled

- caesar deluxe (1993)(impressions)[cr atx](disk 1 of 2)[fd]/Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf: file caeswa32.apx (header 27953 bytes, blocks hold 976): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 1085 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- caesar deluxe (1993)(impressions)[cr atx](disk 1 of 2)[fd]/Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:CSR.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- caesar deluxe (1993)(impressions)[cr atx](disk 2 of 2)[fd]/Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:CSR.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 packed track region(s), 29696 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:play

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_words_then_call`: Copies a fixed run of sixteen words from a source table offset by a constant into a destination pointer, then invokes a routine pointed to by a global slot.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:assign

- `find_max_and_prepare_header`: Walks a linked list of entries to find the maximum value in a field, then writes that value and a run of sixteen words into a header buffer.
- `conditionally_copy_words`: Copies a fixed number of words from a source table into a destination buffer pointed to by a parameter, then invokes a routine stored in a global slot.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:echo

- `find_max_and_copy_blocks`: Walks a linked list of entries to find the maximum value in a field, then writes that value and a small header into a buffer.
- `copy_block_and_call_3`: Copies a fixed-size block of data from a source address to a destination address when the destination is valid, then invokes another routine.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:skip

- `find_max_and_prepare_buffer_10`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block into a table and copies sixteen words from a source table into a destination pointer before invoking another routine.
- `copy_words_and_call`: Copies a fixed number of words from a source offset into a destination buffer when the destination is non-null, then invokes a routine through a stored pointer.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:CSR.EXE

- `run_setup_loop`: Performs a small sequence of setup operations, repeatedly invokes another routine and a function stored in a global variable table while decrementing a counter, then writes zero through a pointer.
- `decrement_to_negative_one`: Repeatedly subtracts one from a signed 16-bit variable until it becomes negative one, then exits.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:failat

- `find_max_and_prepare_buffer_11`: Walks a linked list of entries to find the largest value in a field, then, if a condition is met, writes that value and a small header into a buffer and sets up a callback.
- `copy_words_and_call_2`: Copies twelve words from a source table into a destination buffer when the buffer pointer is valid, then invokes the routine whose address is held in a register.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:Anthrox

- `initialize_aga_patch`: Invokes a couple of other routines, then runs a four-iteration loop that repeatedly calls another routine and dereferences a pointer stored in a global structure to invoke a function through it.
- `decrement_until_minus_one_3`: Repeatedly decrements a signed 16-bit local variable by one until it reaches negative one, then returns.
- `process_cached_data`: Executes a routine twice, then runs a two-iteration loop that repeatedly calls another routine and branches on the sign and zero status of its result to perform either an accumulating pointer update or a single call.
- `copy_byte_block`: Copies a block of bytes from one memory location to another using a length counter.
- `copy_memory_and_process_bits`: Copies a block of bytes from one memory location to another, decrementing both pointers as it goes.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:cd

- `find_max_and_prepare_buffer_12`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header of sixteen words into a destination buffer and invokes another routine.
- `copy_values_and_call`: Copies a fixed count of four-byte values from a source table, located at a fixed offset from a base pointer, into a destination buffer when that buffer is not null.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:endif

- `return_zero_3`: Returns a single zero value and performs no other operation.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:caesar.boot

- `ProgStart`: Opens the graphics and intuition libraries, sets a flag in the intuition display structure when it is still clear, and in that case frees a graphics raster and remakes the display, then closes both libraries and returns zero.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:if

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block of sixteen words into a destination buffer and invokes another routine pointed to by a value.
- `copy_buffer_and_call`: Copies a fixed-size block of words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:play

- `find_max_and_prepare_buffer_3`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block and copies sixteen words from a source table into a destination buffer before invoking a routine pointed to by the caller.
- `copy_buffer_and_call_2`: Copies a fixed block of 15 words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:assign

- `find_max_and_prepare_buffer_4`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header table filled with a repeating pattern and copies sixteen words from a source table into a destination buffer.
- `copy_block_and_call`: Copies a fixed-size block of words from a source table into a destination buffer when that buffer is not null, then invokes another routine.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:echo

- `find_max_and_prepare_buffer_5`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block and copies sixteen words from a source table into a destination.
- `copy_block_and_call_2`: Copies a fixed-size block of words from one memory location to another when the destination is valid, then invokes another routine.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:lab

- `return_zero`: Empty routine that returns zero without touching any data.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:skip

- `find_max_and_prepare_buffer_6`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header of sixteen words into a buffer and copies sixteen words from a source table into a destination pointer.
- `copy_words_then_call_2`: Copies fifteen consecutive 32-bit words from a source table pointed to by a register into the block of memory addressed by a stack parameter, then invokes the routine referenced by another register.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:CSR.EXE

- `run_startup_cleanup`: Performs a small cleanup sequence at program startup by invoking another routine several times and clearing a four-byte location pointed to by a stack value.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:failat

- `find_max_and_prepare_buffer_7`: Walks a linked list of entries to find the maximum value in a field, then writes a small header block containing that maximum and a run of sixteen words into a destination buffer.
- `copy_words_then_call_pointer`: Copies a fixed number of four-byte words from a table into a caller-provided buffer when that buffer is not null, then invokes a pointer stored in a register.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:Anthrox

- `initialize_patch_dispatch`: Performs a fixed sequence of setup calls, then runs a four-iteration loop that invokes another routine and calls through a pointer stored in a global variable.
- `decrement_until_minus_one_2`: Repeatedly subtracts one from a local word until it becomes negative one, then returns.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:cd

- `find_max_and_prepare_buffer_8`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes that value and a small header into a buffer.
- `copy_block_then_call`: Copies a fixed-size block of 32-bit words from one location to another when the destination is not null, then calls another routine.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:endif

- `return_zero_2`: An empty routine that immediately returns zero without touching any data.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:caesar.boot

- `ProgStart`: Opens the graphics and intuition libraries, sets a flag in the intuition display structure when it is still clear, and in that case frees a graphics raster and remakes the display before closing both libraries and returning zero.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:if

- `find_max_and_prepare_buffer_9`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block into a buffer and copies sixteen words from a source table into it before invoking another routine.
- `copy_buffer_and_call_3`: Copies a fixed run of 15 words from a source table into a destination buffer pointed to by a parameter, but only when that parameter is not null.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:bootblock

- `open_libs_report_status`: Opens a library and sets a flag in it, then opens a second library and reports whether that second open succeeded.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_graphics_copper_sequence`: Opens the graphics library, saves interrupt and DMA state, waits on the raster beam, clears sprite control registers and fills a table of sprite values, then runs sixty passes that wait for raster lines while stepping a counter and two pointers through fixed tables, and finally restores the saved interrupt and library state.
- `swap_buffer_pair_store_offset`: Calls four helper routines in sequence, then stores a value 350 bytes past the second saved buffer address into a shared pointer and swaps the two saved buffer addresses.
- `process_counter_table_lookup`: Checks a global counter and, when it is nonzero, either decrements it recursively or indexes into a table using a scaled offset and copies several bytes from it.
- `swap_buffer_pair_back_offset`: Calls another routine twice, then swaps two adjacent global variables by XORing a flag bit and using a temporary value.
- `set_table_pointers`: Stores an incoming address into a global variable and writes a computed address into another global variable.
- `store_scaled_position_pair`: Stores the incoming value shifted right by three bits into one shared variable, then stores the value shifted right by six bits into a second shared variable.
- `combine_and_mask_coordinate_pair`: Adds each of two word pairs to its partner and masks both results to clear the low bit and the top four bits, keeping the values even and within a limited range.
- `init_blitter_registers`: Saves a caller-supplied value into a global variable, then writes several blitter control, address and size registers and waits for the blitter to become idle.
- `configure_blitter_registers`: Writes several blitter control, address and size registers, then waits for the blitter to become idle.
- `manage_flag_state_machine`: Checks a flag bit and, when set, queries a routine whose result controls a small state machine that counts down and eventually resets several fields.
- `configure_blitter_registers_2`: Writes several blitter control, address and size registers with a different height value, then waits for the blitter to become idle.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_blitter_idle`: Spins while the blitter busy flag in the DMA control status register is set, then returns.
- `reset_text_stream_state`: Points a text stream read position at a fixed start address, zeroes the column counter, sets the line counter to one, and clears a byte in shared memory.
- `advance_string_pointer_2`: Reads a character from a string and advances a pointer, skipping past leading zeros to the next non-zero entry.
- `advance_string_pointer_step`: Increments a counter and wraps it when it reaches a fixed limit, then advances a string pointer.
- `advance_string_pointer`: Initialises a counter and advances a pointer past a leading zero, then scans a table for the next non-zero entry, storing it along with a fixed length.
- `setup_blitter_aligned_copy`: Sets blitter source masks and modulo values, then chooses the control word and size depending on whether two address values are odd or even, offsetting the source and destination by fixed amounts in the odd case.
- `build_copper_list_from_pointer_2`: Writes two fixed copper instruction words at a destination pointer, then copies sixty-four word pairs from a source table into the list.
- `build_copper_list_from_pointer_3`: Writes two fixed copper instruction words at a destination pointer located further along, then copies sixty-four word pairs from a different source table into the list.
- `build_copper_list_from_pointer`: Writes two fixed copper instruction words at the destination pointer, then copies 64 word pairs from the source table into the list, advancing six words per source entry.
- `split_base_address_words`: Splits the low and high words of five base addresses, each formed by adding a fixed offset to a shared base, into paired global variables, then loads a value into a global variable.
- `encode_sprite_position_words`: Encodes a stored position into the words of two sprite control entries: the start position is halved, a fixed offset is added to the stop position, and a bit field derived from a second stored value is merged in.
- `store_address_and_offset`: Stores the incoming address into a global variable and then writes a computed address into another global variable.
- `call_and_toggle_flag`: Calls another routine and then flips the lowest bit of a global variable by XORing it with one.
- `store_arguments_run_helpers`: Saves the incoming address and two register values into shared variables, then calls three helpers in order.
- `test_memory_bit`: Polls a specific bit within a byte at an offset from a pointer until that bit is cleared.
- `setup_blitter_rect_copy`: Reads two corner coordinate pairs from shared storage, computes the blit width in words and the height in rows from their differences, sets a full-mask destination-only blit with a modulo, and starts it using an address supplied by a helper.
- `start_blitter_masked_rect_copy`: Computes the blit width and height from two stored corner coordinate pairs, sets a full-mask blit with a combined source and destination control word and modulo values, stores the destination address and starts the blit.
- `project_points_scaled_to_screen`: Resets a rectangle record to default bounds, then walks a table of points scaling each coordinate pair by a stored factor and adding fixed centre offsets before clamping the results to the bounds.
- `process_indexed_point_pairs`: Walks a list of index pairs, looks up two words for each index in a table, and passes them with the paired value to a helper, once per pair.
- `reset_default_clip_rect`: Stores four default bounds values, 160, 160, 128 and 128, into a shared rectangle record.
- `clamp_values_to_bounds`: Clamps a pair of coordinate values into the stored bounds of a rectangle, updating each edge only when the incoming value falls outside it.
- `setup_blitter_line_octant`: Initializes the blitter control and data pointers, then computes the source address, step counts, an octant selector from the coordinate signs, and a control word for drawing a line between two points, storing all of these values into the blitter registers.
- `find_max_byte_and_reset`: Scans a zero-terminated string from a base address to find its longest byte value, then uses that value to index a fixed table.
- `tick_counter_and_update_registers`: Increments a counter and resets a group of globals when a limit is reached, copying several values into a hardware register block.
- `clear_state_globals`: Clears five consecutive global variables to zero and sets the last one to a fixed value, then returns.
- `process_input_nibble_2_2`: Reads a nibble from a structure field and dispatches based on its value, scaling and clamping a counter up or down against stored limits.
- `setup_and_play_sound`: Copies a sound entry from a lookup table into a structure, setting its length, period, volume and pointer fields.
- `process_input_nibble_2`: Reads a nibble from a structure field and dispatches based on its value, scaling and clamping a counter up or down against stored limits.
- `copy_structure_word`: Copies a word from one structure field into another structure field at a fixed offset, using two passed base addresses.
- `search_table_and_set_flag`: Searches a table for a value within a limit, adjusting the search index and handling a sign flag.
- `clamp_and_lookup_value`: Clamps a structure field between zero and a stored maximum, adjusting it up or down and resetting the maximum when the limit is reached.
- `adjust_struct_value_from_table`: Reads a couple of fields from a structure, applies a sign and a scaling factor to a value selected from a table, then stores the adjusted result back into the structure and updates another of its fields.
- `process_input_nibble`: Reads a nibble from a joystick/mouse input byte and dispatches based on its value, adjusting structure fields by scaling, clamping, or storing them.
- `store_low_nibble_value`: Stores the low nibble of one structure field into another field of the same structure.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
