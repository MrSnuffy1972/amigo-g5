# Game 657 (Dyna Blaster): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00657). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905440 top-level bytes in 18 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 95 of 95 named (0 author symbols), 95 of 95 described (7 by rule, 44 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 569 of 569 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 845248 bytes, 69 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2720 of 2721 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 29301 |
| code | 3080 |
| data | 823513 |
| padding | 906 |
| unused | 48640 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dyna Blaster.adf:bootblock | 1 | 21 | 1 |
| logged disk load from Dyna Blaster.adf boot-block load | 1 | 90 | 3 |
| logged disk load from Dyna Blaster.adf boot-block load | 1 | 132 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 590 | 19 |
| Dyna Blaster.adf:blocks 11-235 (bytes 5632-120832)#unpacked (raw code file, load address unknown) | 1 | 841 | 25 |
| Dyna Blaster.adf:blocks 242-289 (bytes 123904-148480)#unpacked (raw code file, load address unknown) | 1 | 53 | 1 |
| Dyna Blaster.adf:blocks 297-341 (bytes 152064-175104)#unpacked (raw code file, load address unknown) | 1 | 691 | 21 |
| Dyna Blaster.adf:blocks 352-458 (bytes 180224-235008)#unpacked (raw code file, load address unknown) | 1 | 156 | 3 |
| Dyna Blaster.adf:blocks 1100-1372 (bytes 563200-702976)#unpacked (raw code file, load address unknown) | 1 | 1105 | 19 |

## Cases handled

- Dyna Blaster/Dyna Blaster.adf: custom-format (non-DOS) disk, 1658 non-empty blocks: matched against track-loaded segments
- Dyna Blaster/Dyna Blaster.adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- Dyna Blaster/Dyna Blaster.adf: the boot block reads 17408 bytes from disk offset 236544 to an address (found statically); recovered as a unit
- runtime segment an address: 473 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- Dyna Blaster/Dyna Blaster.adf: 2 logged loads cover 28672 bytes of the image
- Dyna Blaster/Dyna Blaster.adf: blocks 2-8 (bytes 1024-4608) outside the filesystem hold data, not code (opcode density 0.059, entropy 6.79)
- Dyna Blaster/Dyna Blaster.adf: blocks 11-235 (bytes 5632-120832) hold a IMP: File Imploder stream (115200 -> 144684 bytes)
- Dyna Blaster/Dyna Blaster.adf:blocks 11-235 (bytes 5632-120832)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- Dyna Blaster/Dyna Blaster.adf: blocks 242-289 (bytes 123904-148480) hold a IMP: File Imploder stream (24576 -> 113460 bytes)
- Dyna Blaster/Dyna Blaster.adf:blocks 242-289 (bytes 123904-148480)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- Dyna Blaster/Dyna Blaster.adf: blocks 297-341 (bytes 152064-175104) hold a IMP: File Imploder stream (23040 -> 209510 bytes)
- Dyna Blaster/Dyna Blaster.adf:blocks 297-341 (bytes 152064-175104)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- Dyna Blaster/Dyna Blaster.adf: blocks 352-458 (bytes 180224-235008) hold a IMP: File Imploder stream (54784 -> 177594 bytes)
- Dyna Blaster/Dyna Blaster.adf:blocks 352-458 (bytes 180224-235008)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- Dyna Blaster/Dyna Blaster.adf: blocks 517-525 (bytes 264704-269312) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.47)
- Dyna Blaster/Dyna Blaster.adf: blocks 1067-1098 (bytes 546304-562688) outside the filesystem hold data, not code (opcode density 0.013, entropy 7.09)
- Dyna Blaster/Dyna Blaster.adf: blocks 1100-1372 (bytes 563200-702976) hold a IMP: File Imploder stream (139776 -> 200000 bytes)
- Dyna Blaster/Dyna Blaster.adf:blocks 1100-1372 (bytes 563200-702976)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- Dyna Blaster/Dyna Blaster.adf: blocks 1375-1405 (bytes 704000-719872) hold a IMP: File Imploder stream (15872 -> 12960 bytes)
- Dyna Blaster/Dyna Blaster.adf: blocks 1408-1480 (bytes 720896-758272) hold a IMP: File Imploder stream (37376 -> 39240 bytes)
- Dyna Blaster/Dyna Blaster.adf: blocks 1485-1535 (bytes 760320-786432) hold a IMP: File Imploder stream (26112 -> 60160 bytes)
- Dyna Blaster/Dyna Blaster.adf: blocks 1540-1557 (bytes 788480-797696) hold a IMP: File Imploder stream (9216 -> 17196 bytes)
- Dyna Blaster/Dyna Blaster.adf: blocks 1562-1580 (bytes 799744-809472) hold a IMP: File Imploder stream (9728 -> 17196 bytes)
- Dyna Blaster/Dyna Blaster.adf: blocks 1584-1604 (bytes 811008-821760) hold a IMP: File Imploder stream (10752 -> 17196 bytes)
- Dyna Blaster/Dyna Blaster.adf: blocks 1606-1626 (bytes 822272-833024) hold a IMP: File Imploder stream (10752 -> 17196 bytes)
- Dyna Blaster/Dyna Blaster.adf: blocks 1628-1647 (bytes 833536-843776) hold a IMP: File Imploder stream (10240 -> 17196 bytes)
- Dyna Blaster/Dyna Blaster.adf: blocks 1650-1667 (bytes 844800-854016) hold a IMP: File Imploder stream (9216 -> 17196 bytes)
- Dyna Blaster/Dyna Blaster.adf: blocks 1672-1686 (bytes 856064-863744) hold a IMP: File Imploder stream (7680 -> 17196 bytes)
- Dyna Blaster/Dyna Blaster.adf: blocks 1694-1710 (bytes 867328-876032) hold a IMP: File Imploder stream (8704 -> 17196 bytes)
- 5 never-loaded region(s), 845248 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 2 packed track region(s), 285184 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Dyna Blaster.adf:bootblock

- `run_bootblock_dma_library_call`: Boot block routine, entered from the Kickstart strap, that makes one library call at a fixed negative offset and touches the DMA control register.

### logged disk load from Dyna Blaster.adf boot-block load

- `unpack_bitstream_to_palette`: Reads a bitstream from a table, unpacks bits into bytes using a refill loop, and writes the resulting values to a palette buffer and the color register.
- `store_low_byte_and_shift`: Writes the low byte of a longword read just before an address register into a location addressed by another register, combined with the high byte of a second value.
- `shift_bits_with_refill_loop`: Shifts a longword bit buffer right once per iteration for a count taken from a register, calling a routine to refill the buffer whenever it empties.

### logged disk load from Dyna Blaster.adf boot-block load

- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `copy_boot_words_and_jump`: Sets user and supervisor stacks, copies 118 words from a2 into destination memory through a3, then jumps backward relative to the final destination pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `trigger_trap`: Stores a longword into a specific memory location and executes a trap instruction, then returns.
- `initialize_hardware_and_cop`: Writes to CIA and DMA registers, calls another routine, sets timer registers, and enters a loop that checks several global variables to call other routines before finally loading a value into a custom register and enabling DMA.
- `check_interrupt_and_jump`: Checks a bit in a custom register, and if set, calls another routine and checks a value at an offset of a table; if that value is one, it calls another routine, writes a value to a custom register, and jumps to a fixed address.
- `frag_movem_cmpi_bne_move_jmp`: Short code fragment that saves registers on the stack, then compares a fixed memory address with 150, then branches if the last result was not zero, then copies 6 to a fixed memory address, then copies 0 to a fixed memory address, then restores saved registers from the stack, then jumps to another point in the code.
- `configure_cop_registers`: Loads a pointer from a memory location, accesses a library structure, and writes several values to a custom register and a table, then calls a routine at a pointer and returns via an exception return.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `copy_data_and_setup_blt`: Copies bytes from a source table to a destination table using a loop with a nested loop structure, waits for a DMA bit to clear, and then configures several Blitter registers and a memory location before waiting for DMA again and returning.
- `update_joystick_position`: Checks a bit in a CIA register to update a global variable, then calculates a difference between a global variable and a byte from a custom register to update two other global variables, finally performing arithmetic on two words from memory and writing the results to a specific memory location.
- `process_coordinate_bounds`: Checks if four longwords at a table starting at a specific address fall within two ranges defined by global variables, then iterates through a second table to call another routine and update a byte at offset 16 of each entry.
- `copy_memory_blocks`: Copies four longwords from a source pointer to a destination pointer, and then copies four longwords from a second source pointer to a second destination pointer within a loop that runs twelve times.
- `fill_table_with_patterns`: Fills a table with repeating patterns of a byte, a constant, and two signed words, using a nested loop structure to repeat this process seven times.
- `update_counter_and_blt`: Increments a global counter and, if it reaches eight, resets it and copies bytes from a source table to a destination table; otherwise, it waits for a DMA bit to clear, configures Blitter registers, and waits for DMA again before returning.
- `setup_blitter_transfer`: Waits for a bit in a custom register to clear, then writes several values to blitter and DMA registers and calls two other routines.
- `calculate_scaled_offsets`: Increments a global variable, then iterates through a table of data, performing complex multiplication, division, and bit manipulation to calculate and store two new values in global variables at offsets 6 and 8.
- `decrement_table_values`: Iterates through a table of words, checking for specific values, and subtracts constant values from the words at those locations before returning.
- `setup_blt_parameters`: Waits for a DMA bit to clear, then writes values to several custom registers and a memory location before returning.
- `calculate_collision_geometry`: Calculates relative distances and a priority value based on the difference between two input values, then stores these results into a structure pointed to by a6 at offsets 82, 98, 100, 64, 72, 84, and 88.
- `execute_multiple_blt_transfers`: Waits for a DMA bit to clear, then performs two separate Blitter transfer configurations by writing to several custom registers and memory locations, waiting for the DMA bit to clear after each configuration, and finally returning.
- `initialize_data_tables`: Clears two separate memory blocks by iterating through them with a loop; the first block is filled with zeros and the second block is filled with the value six.

### Dyna Blaster.adf:blocks 11-235 (bytes 5632-120832)#unpacked (raw code file, load address unknown)

- `initialize_dma_and_object_structures`: Sets the DMA control register to enable four channels, clears a series of global variables, and populates a table of four memory addresses with specific values.
- `update_dma_and_object_states`: Checks several global status bytes and, if conditions are met, updates a series of object structures by calling routines to initialize, load parameters, and clamp values.
- `advance_sample_loop_for_channels`: Walks three sound channels and advances each sample pointer by a step, handling one-shot, forward loop and ping-pong reversal modes.
- `load_channel_pattern_fields`: Loads a channel's pattern entry from a table indexed by a row byte, copying a word pointer and two byte parameters into the channel record and clearing a flag.
- `apply_channel_slide_step`: Calls a helper to update a pointer, then when a reset flag is set clears two accumulator words, and otherwise adds a stored delta to a channel output word and advances an accumulator by a step each tick while it is nonzero.
- `read_next_channel_command_byte`: Clears two channel counters, then either counts down a hold delay or reads the next command byte from a stream and stores its parameter bytes, storing a hold count when the byte is negative.
- `start_channel_note_from_table`: Clears the effect state fields of the record when the trigger at offset 46 is set, then loads a sample header from a table row indexed by the trigger, copying the pointer and length into the record and computing the initial period from a lookup table.
- `update_channel_period_with_clamp`: Calls the eight effect helpers for one tick, adds the slide amount read at offset 58 to the channel period at offset 10, clamps the result to a floor of 95 and a ceiling of 5760, and stores the clamped value to the period output at offset 6 of the structure in a6 indexed by the repeat count saved at entry.
- `step_repeat_counter`: Reads a repeat count from offset 12 of the pointed-to record, and when it is nonzero increments the counter at offset 37 unless it already matches the value at offset 13, in which case it adds the count to the counter at offset 38.
- `update_entity_state_attributes`: Extracts a command index from a word at offset 50 of a structure in a2 and performs conditional operations based on that index.
- `slide_period_toward_note_target`: Looks up a target period from a table using a note value at offset 50, then moves the period at offset 10 toward that target by the step stored at offset 62, clearing the step and snapping the period to the target when reached, and returns the current period.
- `set_pitch_from_command_table`: Computes a table index from the command word and a byte in the caller's data, then loads the pitch value at that index into the record.
- `negate_field_and_subtract`: Stores the negated value of one record field into a second field and subtracts that value from a third field.
- `adjust_counter_by_scaling`: When the field at offset 18 is zero and the byte at offset 4 of the record in a0 is zero, updates offset 13 from another record, then either adds a scaled value to or subtracts a scaled value from the counter at offset 12 depending on a bitmask, clamping the result.
- `frag_move_beq_clr_not_and_rts`: Short code fragment that copies the memory at an offset from a2 to d0, then branches if the last result was zero, then clears the memory at an offset from a2, then copies d0 to the memory at an offset from a2, then copies the memory at an offset from a2 to d1, then inverts d1, then masks the memory at a5 with d1, then returns.
- `frag_move_beq_cmp_bne_clr_rts`: Short code fragment that copies the memory at an offset from a2 to d0, then branches if the last result was zero, then compares d0 with the memory at an offset from a0, then branches if the last result was not zero, then clears the memory at an offset from a2 (2 times in a row), then copies 0 to an indexed entry from a6, then returns.
- `frag_move_beq_clr_rts`: Short code fragment that copies the memory at an offset from a2 to d0, then branches if the last result was zero, then clears the memory at an offset from a2, then copies d0 to the memory at an offset from a2, then copies the memory at an offset from a2 to the memory at an offset from a2 (2 times in a row), then returns.
- `decrement_counter`: Loads a byte from memory offset 5 of the structure in a0, stores it to offset 3 of the same structure, then decrements that value and returns.
- `frag_move_cmpi_bcc_add_rts`: Short code fragment that copies the memory at an offset from a2 to d0, then compares d0 with 31, then branches if the carry flag is clear, then adds a fixed memory address to d0, then copies d0 to the memory at an offset from a0, then returns.
- `step_secondary_envelope_pitch`: Advances the secondary envelope counter at offset 43 by comparing it to a segment length, reloading it from the length minus the step and adding a table value to the pitch at offset 10 when the segment ends, otherwise incrementing the counter.
- `step_primary_envelope_pitch`: Advances the primary envelope counter at offset 39 using its own record fields, reloading it and adding a lookup-table value to the pitch at offset 10 when the segment ends, otherwise incrementing the counter.
- `copy_sequence_entry_to_buffer`: Advances the sequence counter at offset 35, and when the segment ends fetches a byte from a table, negating the counter when the byte is negative, otherwise copying the long and word fields of a 64-byte entry from the table into the output buffer at a running offset and recording the entry index.
- `scale_position_into_buffer`: Calls one helper, copies a position word into a second field, and writes the high half of its product with a scale field into an output buffer at a running offset.
- `run_envelope_state_machine`: Advances the record's level envelope one step according to its state: a ramp up, a ramp down, a counted pause, or a timed hold, moving to the next state when each one finishes.
- `write_dma_control_registers`: Checks a flag word and, when clear, clears it, then indexes a table using a scaled value in d0 to fetch a byte that is written to several global variables before setting bit 15 of the DMA control register to enable DMA channels.

### Dyna Blaster.adf:blocks 242-289 (bytes 123904-148480)#unpacked (raw code file, load address unknown)

- `update_cia_registers`: Writes specific values to CIA register addresses, performs bit manipulation on a register, and updates another register based on a condition before returning.

### Dyna Blaster.adf:blocks 297-341 (bytes 152064-175104)#unpacked (raw code file, load address unknown)

- `copy_pixel_data`: Loads addresses from memory locations 24660 and 2e160 into a0 and a1 respectively.
- `copy_and_or_data`: Copies data between memory locations while performing bitwise OR operations on 16-bit values in a loop structure, then returns after completing the operation.
- `initialize_data_blocks`: Initializes two data blocks with specific 16-bit values and their swapped counterparts, then increments pointers and adds a constant to the values in a loop before returning.
- `frag_not_bne_lea_moveq_cmpi_beq_addq_br`: Short code fragment that inverts a fixed memory address, then branches if the last result was not zero, then loads the address of a location in the code into a0, then loads the address of a location in the code into a1, then loads 7 into d7, then compares a fixed memory address with 16, then branches if the last result was zero, then adds 1 to a fixed memory address, then jumps to another point in the code.
- `check_pixel_data`: Loads addresses from memory locations 32f52 and 33150 into a0 and a1, then calls another routine with a loop that processes 31 color components.
- `process_pixel_data`: Processes pixel color components by extracting and scaling them using multiplication and bit shifting operations, then combines the results into a single 16-bit value before storing it.
- `set_graphics_parameters`: Checks a bit in memory, and if set, returns immediately; otherwise, it writes several long values to memory locations relative to a6 and stores a word value there before returning.
- `initialize_block_data`: Sets multiple global variables with specific values, including a 32-bit value at offset 64, and returns immediately.
- `configure_render_state`: Checks a bit in memory, and if set, returns immediately; otherwise, it writes several long values and a pointer to memory locations relative to a6, then stores a word value there before returning.
- `manage_render_queue`: Decrements a counter in memory, and if not zero, increments another counter and updates memory values; otherwise, it processes data from memory locations and updates several registers before returning.
- `manage_render_queue_2`: Decrements a word value at memory location 4e5da, and if not zero, branches to another routine.
- `update_render_parameters`: Increments a word value at memory location 4e824, loads a word from a table into d0, and if that value is negative, branches to another routine.
- `setup_render_parameters`: Loads a long value from memory, checks a bit in memory, and if not set, writes several long values and a pointer to memory locations relative to a6, then stores a word value there before returning.
- `load_render_data`: Decrements a word value at memory location 4e880, and if not zero, branches to another routine.
- `update_animation_counter`: Decrements a word value at memory location 4e8fc, and if not zero, branches to another routine.
- `update_render_state`: Decrements a word value at a fixed memory location and branches to another routine if the result is not zero.
- `manage_render_state`: Decrements a word value at memory location 4eb86, and if not zero, branches to another routine.
- `update_render_data`: Decrements a word value in a global variable, and if not zero, updates multiple global variables with data from a table.
- `process_render_data`: Decrements a word value at memory location 4edaa, and if not zero, branches to another routine.
- `configure_dma_channels_2`: Writes specific values to CIA control registers and manipulates a DMA control register bit.
- `configure_dma_channels`: Writes specific values to CIA control registers and manipulates a DMA control register bit.

### Dyna Blaster.adf:blocks 352-458 (bytes 180224-235008)#unpacked (raw code file, load address unknown)

- `update_color_registers`: Updates color register values in hardware using a lookup table based on input data from a global variable, then performs bit manipulation on the result before returning.
- `update_palette_entry`: Modifies a global variable's palette entry based on input data, then returns.
- `initialize_render_buffers`: Loads addresses from memory locations 57188 and 5737c into a0 and a1 respectively.

### Dyna Blaster.adf:blocks 1100-1372 (bytes 563200-702976)#unpacked (raw code file, load address unknown)

- `initialize_system_and_tables`: Sets hardware registers, calls three routines, clears a block of memory using a loop, and processes a data table to populate various memory locations based on specific byte values.
- `calculate_table_offset`: Calculates two different offsets based on the input in d0 and adds them to a base address in a5, then returns.
- `clear_memory_blocks`: Calls a helper routine, then uses a loop to fill two memory blocks starting at offsets of a5 and a5 plus 8400 with zeros.
- `update_table_values`: Calculates a value by reading from a table using an offset derived from a byte at a specific offset in a structure, then performs a bitwise OR to store this value into two 32-bit words at offsets 0 and 8400 of a destination structure in a5 through a loop.
- `fill_table_from_source`: Copies bytes from a source table to a destination table, incrementing a byte value for each entry until a byte of all ones is encountered.
- `initialize_hardware_and_tables`: Sets a CIA register, configures several hardware registers, and populates a table with specific values derived from a calculation involving a nibble-based bit manipulation.
- `generate_lookup_tables`: Fills a table starting at offset 192 with values calculated via division, populates a second table starting at offset 8116 with bit-rotated bytes, and populates a third table starting at offset 8772 with incrementing 16-bit values.
- `perform_bit_manipulation_and_scaling`: Performs a loop of bitwise XOR operations and shifts on a value, then returns a result that is either a bitwise mask or a scaled product of the input.
- `find_block_index_and_clear_audio`: Finds the smallest byte greater than the running minimum in the table at 952 bytes after the pointer stored at offset a fixed address, then uses that byte as an index to zero-fill and populate a table of longwords, clears several structure fields and custom-chip registers, enables DMA, and returns.
- `init_audio_and_load_block_entry`: Initializes the four audio channels by loading their volume tables and then loads a block entry from a table using an index in the structure in a6, copying its fields into offsets 4 through 40 of that structure.
- `load_and_apply_block_entry`: Loads a block entry from a table using an index in the structure in a6, copies its fields into the structure at offsets 4 through 40, and writes the resulting position values into the structure in a5.
- `dispatch_block_byte_values_2`: Reads the low nibble of the byte at offset 2 of the structure in a6 and dispatches on it, using offset 9 of that structure to index a table of 72-bit entries and offset 8 as a 16-bit value, while writing a 16-bit result to offset 6 of the structure in a5.
- `dispatch_block_byte_values_3`: Copies the 16-bit value at offset 16 of the structure in a6 to offset 6 of the structure in a5 and returns.
- `search_clamp_table`: Searches a table for the first entry greater than the value at offset 0 in a6, adjusts the index by the byte at offset 9, stores the found value at offset 24, and sets the byte at offset 11 to 1 when the value is smaller.
- `clamp_and_search_table`: Clamps the 16-bit value at offset 16 of the structure in a6 toward the value at offset 24, then reads byte 31 and the low nibble of byte 18, multiplies it by 72, and scans a table for the first entry greater than the clamped value, advancing by 2 each step until reaching 72, then stores that index in the structure in a5.
- `apply_signed_delta_value`: Computes a signed value from a byte and a table lookup, applies its sign to a value at offset 16 of a structure in a5, and adds a scaled nibble to a counter at offset 27 before returning.
- `dispatch_block_byte_values`: Reads the low nibble of the byte at offset 2 in the structure in a6 and dispatches on its value, with 9 scaling a value in a5, 12 writing a nibble into a byte in a5, and 13 returning zero or the byte at offset 3.
- `store_low_nibble_byte`: Extracts the low nibble of byte 3 in the structure in a6 and stores it at offset 18, then returns.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
