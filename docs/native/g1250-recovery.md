# Game 1250 (PitFighter v1.1 0196): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01250). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4089151 top-level bytes in 14 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 28 raw-word lines |
| Functions named and described | pass, 395 of 395 named (0 author symbols), 395 of 395 described (19 by rule, 258 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 8641 of 8648 (99.92%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14656 of 14820 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 321721 |
| code | 60120 |
| data | 3694981 |
| padding | 41 |
| unused | 12288 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 80 | 10 |
| Disk.1:bootblock | 1 | 8 | 2 |
| Disk.2:bootblock | 1 | 8 | 2 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 2375 | 136 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11955 | 235 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 29 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 31 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 31 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 31 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| logged disk load from Disk.1 | 1 | 270 | 4 |

## Cases handled

- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.1: custom-format (non-DOS) disk, 1736 non-empty blocks: matched against track-loaded segments
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: custom-format (non-DOS) disk, 1741 non-empty blocks: matched against track-loaded segments
- PitFighter_v1.1_0196/PitFighter_v1.1_0196.hdf: hard-disk image (AmigaDOS OFS, 4320 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree PitFighter_v1.1_0196_hdf, which is recovered file by file; no block outside the filesystem holds data
- 4 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address (play-long-joy): 7372 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.1: 243 executed PCs whose RAM was overwritten before any snapshot match its bytes 326064-327328: recovered as a unit run at an address
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.1: 5 logged loads cover 359528 bytes of the image
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.1: blocks 11-21 (bytes 5632-11264) hold ASCII text
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.1: blocks 561-584 (bytes 287232-299520) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.97)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.1: blocks 587-595 (bytes 300544-305152) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.25)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.1: blocks 601-635 (bytes 307712-325632) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.36)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.1: blocks 640-812 (bytes 327680-416256) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.26)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.1: blocks 964-1759 (bytes 493568-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.03)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 2-1380 (bytes 1024-707072) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.68)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1382-1384 (bytes 707584-709120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.37)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1386-1388 (bytes 709632-711168) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.92)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1390-1392 (bytes 711680-713216) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.82)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1395-1499 (bytes 714240-768000) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.02)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1501-1503 (bytes 768512-770048) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.25)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1505-1507 (bytes 770560-772096) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.07)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1509-1511 (bytes 772608-774144) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.42)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1513-1515 (bytes 774656-776192) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.22)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1517-1519 (bytes 776704-778240) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.15)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1521-1523 (bytes 778752-780288) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.55)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1525-1527 (bytes 780800-782336) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.21)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1529-1531 (bytes 782848-784384) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.33)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1533-1535 (bytes 784896-786432) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.47)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1537-1539 (bytes 786944-788480) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.06)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1541-1543 (bytes 788992-790528) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.53)
- PitFighter_v1.1_0196/PitFighter_v1.1_0196_hdf/PitFighter/Disk.2: blocks 1545-1759 (bytes 791040-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.60)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `set_interrupt_and_dma_state` (Disk.1 +an address loaded at an address (270000 bytes)): Writes the interrupt enable, DMA enable, interrupt request and disk control registers, and reads the CIA port registers that carry the fire buttons and disk status. Kind: interrupt/DMA state management for custom disk access. Pass path: The branch following the routine that continues normal data loading/execution.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `execute_shared_vectors`: Calls three different routines through function pointers stored in a shared global block and returns the value from a data register.
- `call_three_subroutines`: Six-instruction routine that is referenced by address and calls three other routines, one of which it reaches through a pointer.
- `forward_single_call_stub`: Four-instruction routine that is referenced by address and forwards control to one other routine, and is reached from one caller.
- `merge_constant_then_call`: Eleven-instruction routine that merges a constant into the data register and then calls three other routines.
- `clear_selected_bit_across_planes`: Clears one bit, chosen from the low bits of a data register, in bytes at fixed offsets from two base pointers, and then zeroes a byte at a third pointer.
- `set_word_bits_then_clear`: Sets the two high-order bits of a 16-bit word at an offset computed from a data register, then clears one bit selected by the low bits of that register in several byte locations.
- `invoke_shared_vector_slot`: Calls a routine through a function pointer at a fixed offset in a shared global block, then returns the value the data register D1 holds afterwards.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 113 into d0.

### Disk.1:bootblock

- `forward_boot_entry_call`: Single-instruction boot block entry that the Kickstart strap calls when the disk starts, and it transfers control to one other routine.
- `load_boot_address_then_call`: Seven-instruction boot block routine that is referenced by an address load, reached from two callers, and calls one other routine.

### Disk.2:bootblock

- `load_boot_routine`: A one-instruction routine that calls another routine to load a boot address.
- `perform_boot_call`: A seven-instruction routine called by two different locations that executes a routine to load a boot address.

### logged disk load from Disk.1

- `load_copper_list_and_jump`: Writes a pointer into the copper list location register and then strobes the copper jump register, which starts a copper program.
- `set_copper_list_pointer`: Stores an address register value into the first copper list location register, which points the copper at a new display list.
- `set_interrupt_and_dma_state`: Writes the interrupt enable, DMA enable, interrupt request and disk control registers, and reads the CIA port registers that carry the fire buttons and disk status.
- `forward_two_routine_calls`: Three-instruction routine that is referenced by an address load, reached from two callers, and calls two other routines.
- `spin_forever_idle_loop`: Four-instruction routine whose only body is an endless loop, so it never returns to its callers once entered.
- `decode_bitplane_draw_commands`: Reads command bytes from a stream and dispatches on each through a large switch, including a case that writes a bit-masked pattern into five bitplane-style regions spaced one row of 40 bytes apart.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `dispatch_draw_command_bytes`: Starts a command-byte dispatcher with the same case structure as its neighbour, including a bit-masked write into five plane-style regions, and makes one call through a dispatch table.
- `return_immediately_stub`: Single-instruction routine that only returns to its caller and does no other work.
- `return_immediately_stub_2`: Single-instruction routine that only returns to its caller and does no other work.
- `copy_row_bytes_into_planes`: Loops over eight rows at a stride of 40 bytes, copying each source byte into four plane regions and then clearing the destination bits that the data register's low bits do not select.
- `clear_and_set_plane_bits`: Loops over eight rows at a stride of 40 bytes, clearing one bit position in five plane regions and then setting it from the source byte where the matching mask bit is on.
- `copy_row_bytes_masked`: Loops over eight rows at a stride of 40 bytes, writing each source byte into four plane regions and then clearing the bits that the mask in the data register does not select.
- `blend_source_bits_into_planes`: Loops over eight rows at a stride of 40 bytes, clearing a bit position in five plane regions and then setting it from the source byte where the matching mask bit is on.
- `init_state_wait_for_zero`: Stores two starting values into shared state words, then calls a routine in a loop and repeats until the word it returns is zero.
- `init_alt_wait_for_zero`: Stores a different pair of starting values into the shared state words than its neighbour, then calls a routine in a loop until the returned word reaches zero.
- `clear_colour_register_block`: Writes zero into a block of 32 consecutive colour registers, four per iteration over eight iterations, starting at the first colour register.
- `scale_colors_into_palette`: Writes 30 colour entries from a table into the palette registers, scaling each red, green and blue nibble by a shared brightness value with rounding.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `save_registers_and_copy_trampoline`: Waits until a status byte clears, copies a block of code words into a RAM area, then writes the current data and address register values into a save area after it.
- `raise_trap_and_set_status`: Raises the trap exception with vector number one and then sets a status byte to all ones.
- `store_d7_and_set_status`: Stores the seventh data register into a shared global and then sets a status byte to all ones.
- `copy_block_and_count_bytes`: Copies a block of words from a source pointer into a destination buffer, with the count taken from the sixth data register.
- `store_low_word_global`: Stores the low word of the data register into a shared global variable and returns.
- `halt_in_endless_loop`: Does nothing except loop forever, so it never returns once entered.
- `wait_for_zero`: Enters a loop that waits until a specific global variable becomes zero, while assigning a constant value to another global variable.
- `start_idle_pending_slot`: Checks a shared enable value and, when a pending slot is idle, calls one routine, marks the slot busy by setting its high byte, and clears a companion flag.
- `arm_pending_slot_when_idle`: When a status word is zero, calls one routine, sets the high byte of the word to ones, and clears a companion flag.
- `arm_cia_timer_countdown`: When a pending word is set and its companion flag is clear, configures a CIA control register, loads a 16-bit count into the timer registers and sets the interrupt control value.
- `arm_cia_timer_when_idle`: Performs the same timer-arming step as its neighbour with the early return removed, so it arms the timer whenever its companion flag is clear.
- `clear_pending_slot_state`: Stores a pointer and then zero into a shared pending slot, clearing its pending state.
- `program_cia_timer_count`: Sets the run-mode bits of a CIA control register, writes the interrupt control value, and loads a 16-bit count from the data register into the timer low and high registers.
- `call_if_flag_set`: Calls one routine only when a shared flag byte is non-zero, and returns otherwise.
- `forward_call_then_return`: Calls one other routine and then returns, adding no other work.
- `handle_cia_a_interrupt_status`: Reads the CIA-A interrupt control and port registers, and when a specific status bit is set it updates a port control bit, computes a byte from another CIA-A port into a lookup location, and reprograms the interrupt control register.
- `dispatch_cia_b_pending_bits`: Reads a pending-bit byte from the CIA-B interrupt control register, clears the bits it handles, and calls one routine when the lowest bit is set and another when bit three is set.
- `service_pending_bit_tail`: Checks two pending bits, sets a flag byte when the second is set, clears the handled bits, and calls one routine when the third bit is set.
- `advance_button_poll_counter`: Masks a state byte, calls one routine, saves the result into a shared slot, and advances a poll counter.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `reset_port_bytes_state`: Zeroes four byte-wide hardware registers at fixed addresses, sets a shared state value and clears a counter.
- `tiny_ori_3`: Small routine of 1 instruction that merges 2 into d6.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `enter_port_reset_sequence`: Single-instruction entry into the port reset sequence, which zeroes four byte-wide hardware registers and sets a shared state value.
- `tiny_ori_4`: Small routine of 1 instruction that merges 94 into d0.
- `reset_port_bytes_long_entry`: Longer entry point into the port reset routine, recorded at 129 instructions, where only the opening part is visible in the decompiled output.
- `reset_port_state_and_read`: Opens by zeroing four byte-wide hardware registers and setting a shared state value, and the facts also show a read of the CIA-B port register and one call.
- `cycle_indexed_port_bytes`: Sets the top bit of one hardware byte, clears another, and writes two bytes taken from a table at a rotating index into two more hardware bytes.
- `advance_port_index_and_call`: Returns when a flag byte is zero; otherwise it increments a shared index byte and calls one routine, setting the byte to a fixed value on one branch.
- `init_audio_channels_and_table`: Builds a table of pointers from a block of byte values, then clears the four audio channel volume registers and sets a state word that marks the setup as active.
- `silence_audio_channels`: Writes zero to all four audio channel volume registers and resets a shared audio state byte to zero.
- `step_index_when_enabled`: Returns when a flag byte is zero; otherwise it steps a shared index byte and calls one routine.
- `reset_index_and_call`: Sets a shared index byte to zero, increments it, and calls one routine.
- `set_status_and_call`: Sets a shared status byte to all ones and calls one routine, then returns its stack value unchanged.
- `load_audio_channel_pointers`: Masks a shared byte and calls a routine six times, then two more times when a flag is set, and the facts show it writes the four audio channel location registers.
- `set_dma_request_word`: Stores a value with its top bit set into a shared word when a status nibble is not full or a companion flag is clear.
- `load_sequence_entry_record`: Loads a four-byte entry from an input block into a record and, when the entry's high nibble is non-zero, copies a parameter row into the record's pointer, count and step fields.
- `copy_sequence_entry_fields`: Sets a record's address field from a supplied offset and copies its values into an output buffer when the entry is active, returning the stored value.
- `copy_record_to_buffer`: Copies a record's address and value fields into an output buffer, masks a length value to twelve bits, and ORs a bit from the record into a shared word.
- `compare_value_with_limit`: Masks a record's value to twelve bits, stores it in the record and clears a flag.
- `clear_record_pending_word`: Zeroes a 16-bit pending field in a record and returns.
- `decode_effect_command_nibble`: Reads the low nibble of a command byte in a record and applies one effect: setting or clearing a shared flag bit, setting a status byte, loading a counter from the parameter, clamping a value to a maximum, or storing the low five bits of the parameter as a shared mode value.
- `set_flag_bit_from_parameter`: Sets or clears bit one of a shared flag byte depending on whether a parameter byte in the record is zero.
- `set_flag_bit_one`: Sets bit one of a shared flag byte and returns.
- `set_status_all_ones`: Sets a shared status byte to all ones and returns.
- `store_param_then_set_status`: Stores a record's parameter byte minus one into a shared byte, then overwrites that byte with all ones and returns.
- `clamp_parameter_to_limit`: Clamps a parameter byte to a maximum of 64 and to a shared limit, then stores the result in a record field.
- `store_low_bits_as_mode`: Takes the low five bits of a record's parameter byte and, when they are non-zero, stores them into a shared byte after first clearing it.
- `write_audio_channel_values`: Writes four values, each obtained from a routine call, into the four audio channel registers at fixed spacing, and writes a fourth pair only when a flag byte is non-zero.
- `clamp_and_store_record_field`: Clamps a record's value field to a shared limit and copies it into a second record field.
- `apply_effect_command_value`: Applies one effect chosen by the low nibble of a record's command word: a threshold table scan, a subtraction with a floor, an addition with a ceiling, a stepped ramp toward a target, a scaled decay, or a bounded counter adjustment.
- `lookup_threshold_table_entry`: Chooses one nibble of the record's parameter byte, high or low depending on a shared mode, then scans a table of 36 thresholds against the record's value and returns the matching entry.
- `lookup_threshold_table_low_nibble`: Scans a table of 36 threshold entries against the record's value, using the low nibble of the parameter byte as the column, and returns the first entry whose threshold does not exceed the value.
- `return_record_value_field`: Returns the record's current 16-bit value field without changing it.
- `lookup_threshold_table_by_column`: Scans a table of 36 threshold entries against the record's value, using a column index passed in a data register, and returns the first entry whose threshold does not exceed the value.
- `lookup_threshold_table_mapped_column`: Looks up a column index from a shared index table, then scans 36 threshold entries against the record's value using a nibble of the parameter byte as the column.
- `find_table_entry_by_threshold`: Iterates through a table of short integers to find the first value less than or equal to a target threshold.
- `fetch_offset_word`: Returns a two-byte value from an offset of sixteen bytes relative to a pointer held in a register.
- `search_threshold_table`: Iterates through a table of words to find the first entry where a value is less than or equal to a specified threshold.
- `step_value_toward_target`: Stores a non-zero parameter byte as a step, then moves a record value toward its target field by that step, snapping to the target when it is passed.
- `subtract_delta_snap_target`: Subtracts a delta from a record's value and, when the value reaches its target field, sets it to that target and clears the target.
- `apply_table_scaled_decrement`: Stores a non-zero parameter byte into a record field, computes a scaled amount from a shared table indexed by bits of another field and the low nibble of that parameter, then advances that field and subtracts the amount from the record value.
- `adjust_counter_from_nibble`: Adds the high nibble of a parameter byte to a counter capped at 64 when that nibble is non-zero, otherwise subtracts the low nibble with a floor at zero.
- `decrement_counter_floor_zero`: Subtracts the low nibble of a parameter byte from a record counter, floored at zero, and returns the record's value field.
- `subtract_param_clamp_floor`: Subtracts a record's parameter byte from its value, and when the twelve-bit part falls below a fixed floor it resets that part to the floor.
- `add_param_clamp_ceiling`: Adds a record's parameter byte to its value, and when the twelve-bit part exceeds a fixed ceiling it resets that part to the ceiling.
- `tiny_addi`: Small routine of 1 instruction that adds 0 to d0.
- `cycle_channel_index_and_run`: Returns early when a shared flag byte is set, otherwise advances a rotating channel index when a shared value is in range, updates the channel state word, and runs a routine for each of three channel slots.
- `update_channel_state`: Modifies a global state variable and performs several conditional routine calls based on the value of a shared variable.
- `advance_channel_index_and_run`: Advances a rotating channel index when a shared value is non-zero and in range, updates the channel state word, and runs a routine for each of three channel slots.
- `advance_channel_counter_step`: Increments a per-channel counter byte and, when it reaches a shared threshold, resets it and runs routines for the active channel.
- `set_channel_bit_store_pointer`: Sets a shared bit for the current channel and, when the flag byte is then zero, stores a pointer value into that channel's audio location register and calls one routine.
- `clear_channel_bit_and_run`: Clears a shared bit that belongs to the current channel index, then runs two routines for that channel.
- `write_channel_pointers_when_idle`: Returns when a shared byte is set; otherwise, for each of three channels whose bit is clear, it writes two pointer words into that channel's audio registers and advances a per-channel offset.
- `write_channel_pointers_unguarded`: Performs the channel pointer writes of its neighbour for each channel whose bit is clear, without the leading early return.
- `reset_word_and_step_index`: When the low ten bits of a word are non-zero it jumps past the reset; otherwise it zeroes the word and steps a shared index around a length taken from a table, repeating until the index wraps to zero.
- `resolve_sprite_attribute_data`: Extracts a nibble-based index from a source word to look up values in a table.
- `update_dma_buffer_parameters`: Calculates an offset by adding two values and stores the result in a structure.
- `configure_dma_transfer`: Sets up several memory values and updates a global variable, including a write to the DMA control register.
- `update_status_flags`: Compares a value from a pointer against a threshold and updates a byte and a word at specific offsets within a structure.
- `clear_memory_word`: Stores a zero word at an offset of twenty-four bytes relative to a pointer held in a register.
- `process_command_byte`: Evaluates a nibble from a byte to perform various updates to a global variable or a structure based on specific bit patterns.
- `update_ciaa_bit`: Sets or clears the second bit of a global variable based on the value of a byte in a structure.
- `set_ciaa_bit`: Performs a bitwise OR on a global variable to set the second bit, affecting the CIAA port register.
- `set_global_byte`: Stores the value two hundred and fifty-five into a global variable.
- `decrement_and_set_byte`: Decrements a byte from a memory location pointed to by a register and stores it in a global variable before overwriting that variable with two hundred and fifty-five.
- `clamp_value_to_range`: Reads a byte from a memory location, caps it at a maximum value if it exceeds a specific threshold, and ensures it does not fall below another value before storing the result in a table.
- `update_masked_value`: Extracts a five-bit value from a memory location and, if non-zero, clears an entry in a table and updates a global variable.
- `write_channel_register_value`: When a shared flag byte is clear, selects one audio channel register block from the current channel index, writes a value computed by a routine into a register six bytes in, and then runs one routine.
- `apply_command_effect_logic`: Processes a command value by performing one of several operations on a record's field, such as threshold table lookups, clamped addition or subtraction, or scaled decrements.
- `dispatch_draw_command`: Calls another routine to process bitplane drawing commands.
- `forward_call_then_return_2`: Calls one other routine and then returns, adding no other work.
- `dispatch_bitplane_draw_command`: Executes a specific drawing operation by processing a command byte from a data stream.
- `load_iff_ilbm_picture`: Parses an IFF FORM container by searching for its bitmap header, colour map and body chunks, reads the picture dimensions, and copies up to 32 colour entries from the colour map into shared memory.
- `call_hook_and_check_status`: Calls a routine through a function pointer, then returns zero when a status byte has its top bit set and the idle check passes.
- `call_constant_loader_routine`: Eight-instruction routine that is referenced by address and calls a two-instruction routine which loads 255 into the data register.
- `init_text_entries_from_string`: Walks a text string until its terminator, and for each non-space character clears two record fields, sets two to all ones and two to thirty-eight, then calls five routines.
- `load_blitter_registers_and_wait`: Stores a control word chosen by a flag bit, two pointer values and a size value into a block of blitter registers, then waits until the blitter busy flag clears.
- `run_two_blitter_passes`: Sets the blitter mask and control registers from a record value, then runs two blit passes of a fixed size, advancing the source and destination pointers by one plane-sized step between passes and waiting for the blitter to finish each time.
- `run_two_blitter_passes_2`: Sets the blitter mask and control registers from a record value, then runs two blit passes of a fixed size, advancing the source and destination pointers by one plane-sized step between passes and waiting for the blitter to finish each time.
- `forward_single_routine_call`: Single-instruction routine reached from an unreached gap that calls one other routine.
- `run_four_blitter_plane_copies`: Sets the blitter mask and control registers, then runs four blits of a fixed size, each aimed at the next plane-sized offset and waiting for the blitter to finish before the next one starts.
- `start_fixed_blit_and_wait`: Clears the mask and offset fields in a record, sets the blitter control word to a fixed value, points the blitter at a fixed address with a fixed size, and then waits for it to finish.
- `start_fixed_blit_and_wait_2`: Performs the same blit setup as its neighbour with a different count field, and waits for the blitter to finish.
- `step_blitter_row_copy`: When a row counter is below a limit, sets the blitter control word and performs four blit passes for the current row, each waiting for the blitter to finish, then advances the row counter.
- `step_blitter_row_copy_capped`: Sets the blitter control word and runs up to four blit passes for the current row depending on the row counter, then advances that counter and caps it at five.
- `run_two_fixed_row_blits`: Sets a fixed blitter control word and clears the offset registers, then runs two blit passes of a fixed size against a plane offset, waiting for the blitter after each one.
- `reset_display_and_sprites`: Waits through a fixed delay loop, clears the sprite control registers, and writes the display window, fetch, playfield control and plane modulo registers.
- `set_copper_list_pointer_2`: Stores a pointer into the first copper list location register, pointing the copper at a display list.
- `set_cop_pointer`: Stores the address of a global variable into a coprocessor register.
- `store_pointer_count_table`: Stores a fixed list of pointer values, each paired with a count of seven, into shared table entries.
- `copy_words_into_record`: Copies 32 words from a source pointer into a record field starting at a fixed offset.
- `fade_colour_table_toward_source`: Fills a block from a source value, sets 32 entries to a fixed value, then for each colour slot lowers each nibble of a working colour table toward the matching source nibble when that nibble is larger.
- `halt_in_endless_loop_2`: Loops forever with no other work, so it never returns once entered.
- `halt_in_endless_loop_3`: Loops forever with no other work, so it never returns once entered.
- `modify_memory_word`: Performs a bitwise OR operation on a word at a location pointed to by a register.
- `run_blitter_fill_sequence`: Starts several blitter operations on a shared buffer, waiting for the blitter to finish after each write.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_value_on_mismatch`: Stores a word into a memory location if two input bytes are not equal.
- `calculate_accumulated_values`: Uses a value from a table to update several memory locations through arithmetic operations.
- `modify_hardware_status`: Checks a bit in a global variable, performs a bitwise OR on a memory location, executes another routine, runs a decrementing loop, and clears a bit in a memory location.
- `modify_bit_field`: Performs bitwise operations on a value to create a mask and then either writes a byte to a calculated memory offset or clears a byte at a specific location.
- `execute_function_table`: Iterates through a table of pointers and calls each function if its address is not zero.
- `iterate_function_pointer_table`: Iterates through a table of data structures, checking if a specific offset contains a non-zero value before calling the function pointer located at the subsequent offset.
- `update_table_and_loop`: Updates a global variable by fetching values from a table using a multiplier, sets several global values, and enters a loop that calls other routines.
- `fetch_table_values`: Calculates an offset using a global variable, fetches three consecutive words from a table into a global variable, and enters a loop that calls other routines.
- `clear_color_registers`: Sets a series of consecutive color registers to zero using a loop.
- `update_color_palette`: Calculates and writes new values to a series of color registers based on a global multiplier.
- `tiny_ori_5`: Small routine of 1 instruction that merges 255 into d0.
- `calculate_and_call`: Performs complex bitwise arithmetic on a global variable to derive two values, which are then passed as arguments to another routine twice.
- `clear_memory_block_2`: Sets four consecutive longwords in memory to zero and clears a global variable.
- `update_indexed_data`: Updates a byte using a bitwise OR, copies data from a table using an index, and increments a global variable with a wrap-around check.
- `increment_conditional_byte`: Increments a byte in memory and resets it to zero or sets another byte to a specific value based on a comparison with a threshold.
- `initialize_system_state`: Resets audio volumes, processes an input command, finds a maximum value in a table, and initializes several global variables and memory blocks.
- `reset_audio_volumes`: Sets four audio volume registers to zero and modifies two global variables.
- `conditional_toggle_byte`: If a global variable is non-zero, it increments a byte and conditionally resets it or sets another global variable to zero-ff.
- `toggle_global_byte`: Increments a global byte and conditionally resets it or sets another global variable to zero-ff based on a comparison.
- `set_audio_byte`: Stores the value zero-ff into a global variable.
- `update_audio_channels_2`: Modifies several audio hardware registers and updates various global variables and tables based on structure data.
- `set_conditional_flag`: Sets a global variable by combining a value from a memory location with a bitmask if certain conditions are met.
- `process_structure_data`: Copies data from one memory location to another, performs calculations based on a nibble value to fetch data from a table, and updates various fields in a structure.
- `update_structure_fields`: Updates various fields within a structure based on values from other structures and modifies a global variable using a bitwise OR operation.
- `update_dma_and_pointers`: Updates several memory locations using values from a pointer and modifies a global variable by performing a bitwise OR with a value from a table.
- `update_offset_value`: Extracts a lower word from a pointer, stores it at an offset, and updates a byte field based on a comparison with another value in the structure.
- `clear_memory_offset`: Stores a zero value into a memory location at a specific offset from a pointer.
- `update_threshold_settings`: Modifies a global variable or a threshold table index based on a command byte extracted from a structure.
- `set_ciaa_fire_bit`: Modifies a bit in the CIAA Port A register to enable or disable a specific bit based on a byte value in a structure.
- `set_ciaa_bit_2`: Performs a bitwise OR on a global variable to set a specific bit.
- `set_threshold_maximum`: Stores a maximum byte value into a global variable.
- `decrement_threshold_value`: Decrements a byte value from a pointer and stores it in a table, while also setting a global variable to a maximum byte value.
- `clamp_parameter_value`: Clamps a value between a specific threshold and a global variable, then stores the result in a structure.
- `set_threshold_index`: Extracts a five-bit value from a structure and stores it into a global variable.
- `update_audio_channels`: Updates several audio channel registers by calling another routine multiple times.
- `sync_audio_parameter`: Updates a structure field with a value from a global variable if it exceeds a specific threshold.
- `update_clamped_parameter_state`: Updates a value within a structure based on a mode selected from a bitmask.
- `search_threshold_table_2`: Uses a value from a structure to select a starting point in a table and iterates through the table to find the first entry that is less than or equal to a specified threshold.
- `search_table_entry`: Iterates through a table of words to find the first entry where the table value is less than or equal to a value from a structure.
- `get_offset_word`: Returns a word from a specific offset within a structure pointed to by a register.
- `search_table_for_value`: Iterates through a table of words to find the first entry that is less than or equal to a value from a structure.
- `lookup_table_value`: Determines a table offset using a byte from a global variable and then searches a table for a word that satisfies a comparison against a structure member.
- `find_table_value`: Performs a loop through a table of words to return a value based on a comparison with a structure member.
- `fetch_structure_word`: Returns a word located at a specific offset within a structure.
- `find_table_entry`: Searches through a list of words to find an entry that satisfies a comparison against a value in a structure.
- `adjust_clamped_value`: Updates a structure member by adding or subtracting a value and clamps the result between zero and a specified threshold.
- `subtract_and_clamp_value`: Subtracts a value from a structure member and clamps it to a minimum threshold if it falls below a specified limit.
- `calculate_scaled_offset`: Calculates a value by multiplying a byte from a table by another byte, scales it, and subtracts the result from a value in a structure.
- `modify_clamped_counter`: Increments or decrements a short value in a structure using bits from a byte, clamping the result between zero and a maximum limit.
- `decrement_clamped_value`: Subtracts a byte from a short value in a structure and clamps the result to zero if it becomes negative.
- `decrement_with_lower_bound`: Subtracts a byte from a short value and ensures the result does not fall below a specific constant threshold.
- `increment_with_upper_bound`: Adds a byte to a short value and ensures the result does not exceed a specific constant threshold.
- `process_input_command`: Updates a command state and processes a value from a global variable through several conditional routine calls.
- `update_command_state`: An instruction-minimal routine that performs a single operation to update a command state or global variable.
- `check_command_condition`: Performs the same logic as routine R3 but is specifically entered via absolute moves and comparisons.
- `update_command_state_2`: Checks a condition to reset a byte, updates a command state variable, and performs conditional updates to a global variable and a bitmask.
- `process_command_input`: Increments a byte through a pointer, applies bitwise masks to a global variable based on a command type, and updates a structure field.
- `modify_command_state`: Updates a bitmask in a global variable based on a command byte and stores a value into a global structure.
- `mask_global_variable_by_command`: Checks a command byte and applies a bitwise AND mask to a global variable.
- `increment_audio_counters_2`: Checks a bitmask in a global variable and, if specific bits are not set, updates other global variables and increments three separate counters by sixteen.
- `increment_audio_counters`: Checks a bitmask and, if bits are not set, updates global variables and increments specific counters by sixteen.
- `reset_loop_counter`: Clears a value in a structure and increments a counter that wraps around at a specific limit.
- `update_structure_data`: Copies data from a source to a destination, performs bitwise masking, and updates several fields in a structure based on values retrieved from a table.
- `transfer_structure_data`: Copies data between structures, performs bitwise masking on a value, and updates a global variable and a memory location.
- `transfer_data_block`: Copies multiple fields from one structure to another and updates a global variable and a hardware DMA control register.
- `reset_offset_data`: Copies a masked value into a structure offset and sets a status byte based on a comparison with a threshold.
- `clear_structure_field`: Sets a two-byte field at a specific offset within a structure to zero.
- `process_input_command_byte`: Evaluates a command byte from a structure to update hardware register flags, decrement counters, or clamp a value within a specific range.
- `toggle_ciaa_bit`: Sets or clears the second bit of a global variable representing a hardware register based on a structure byte.
- `set_bit_in_register`: Performs a bitwise OR operation to set a specific bit in a global variable associated with hardware input.
- `set_global_byte_2`: Stores the value 255 into a global byte.
- `decrement_and_set_byte_2`: Decrements a byte from a memory location pointed to by a register and stores it in a global variable, then sets another global variable to 255.
- `clamp_value_to_limit`: Loads a byte from a pointer, caps it at a maximum value if it exceeds a certain threshold, and then ensures it does not exceed a value stored in a global variable before storing the result in a table.
- `mask_and_store_byte`: Masks a byte from a pointer with a bitmask and, if the result is non-zero, clears a global variable and stores the masked value in another global variable.
- `update_audio_channel_parameters`: Updates specific hardware audio channel registers by retrieving a value from another routine and storing it into a global structure.
- `update_clamped_parameter_state_2`: Modifies a numeric value within a structure using one of several modes determined by a bitmask.
- `copy_memory_block`: Copies a block of 31 words from a source table to a destination address.
- `copy_memory_block_2`: Copies a block of data from a source table to a destination location.
- `copy_memory_block_3`: Copies a block of data from a source table to a destination address.
- `tiny_ori_ori_ori_2`: Small routine of 3 instructions that merges 8 into d0, then merges 0 into d0, then merges 1 into d0.
- `tiny_ori_6`: Small routine of 1 instruction that merges 244 into d1.
- `copy_memory_block_5`: Copies a block of data from a source address to a destination address using a loop.
- `set_global_state`: Checks specific conditions involving a global variable and a local value, then sets a global variable to a specific constant before calling another routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `check_state_condition`: Checks if a global variable is zero and a second value is non-zero, then sets a global variable to a specific value and calls another routine.
- `update_sprite_attributes`: Updates a sprite attribute table by copying data from a source table and modifying several global variables and hardware registers.
- `toggle_bit_and_update_state`: Toggles a specific bit in a byte via a pointer and performs complex conditional updates to several global variables based on their current values and bitwise checks.
- `update_state_logic`: Performs conditional updates to multiple global variables based on a countdown of a global value and bitwise checks of other global variables.
- `update_audio_registers`: Updates various audio channel length and location registers and enables DMA channels via the DMA controller.
- `transfer_memory_block`: The routine performs a memory copy operation using a loop.
- `mask_and_copy_bytes`: Copies a block of bytes to four different memory locations while applying bitwise masks to each destination.
- `call_mask_copy`: A four-instruction routine that calls a routine to copy and mask bytes.
- `call_mask_copy_short`: A two-instruction routine that calls a routine to copy and mask bytes.
- `do_nothing`: The routine is empty and performs no operations before returning.
- `nop_padding`: The routine is empty and contains no executable instructions.
- `call_mask_copy_2`: Calls a routine to copy and mask bytes.
- `call_mask_copy_short_2`: This routine consists of a single instruction that invokes another routine to perform a masked byte copy operation.
- `copy_and_mask_bytes`: Copies a block of bytes to four different memory locations and applies bitwise masks to each destination based on a bitmask.
- `conditionally_mask_bytes`: Iterates through a source block to selectively set or clear bits in multiple destination memory locations using a bitmask.
- `copy_and_mask_data`: Copies bytes from one memory location to another in a loop, applying bitwise masks to specific offsets within each destination block.
- `execute_infinite_loop`: The routine enters an endless loop that prevents further execution of the program flow.
- `update_scrolling_text_buffer_2`: Manages a character buffer by shifting data and modifying individual bytes to create a scrolling or cycling text effect.
- `find_minimum_entry`: Iterates through a table to find an entry that minimizes a calculated value, then stores the resulting minimum and the corresponding address in global variables.
- `tiny_ori_7`: Small routine of 1 instruction that merges 6 into d0.
- `search_table_minimum`: Loops through a memory range to find an entry that satisfies specific comparison and flag conditions, updating a global variable with the minimum value found.
- `calculate_weighted_difference`: Calculates the absolute difference between two sets of values stored in a table and returns a weighted sum based on those differences.
- `find_nearest_value`: Searches through a table to find a pointer to an entry that minimizes a weighted difference calculated by another routine.
- `find_minimum_value`: Iterates through a table to find a value that minimizes a difference, storing the result in a global variable.
- `compare_coordinate_differences`: Compares the absolute differences between two sets of values and returns zero if specific threshold conditions are met.
- `modify_video_pointer`: Performs arithmetic on a global variable and uses the result to influence the value of a video hardware register.
- `update_entity_state_and_animation`: Manages entity state transitions by decrementing internal timers and updating animation frame pointers based on specific status flags.
- `update_entity_state_2`: Sets a word value at an offset within a structure based on the lowest bit of a register.
- `update_entity_data`: Updates specific fields within a structure by writing word and long values and performing arithmetic on coordinates.
- `update_sprite_animation_state`: Updates animation frame indices and sprite pointer offsets based on movement delta and state counters.
- `modify_object_properties`: Updates multiple fields within a structure, including coordinate offsets and status flags, and performs conditional resets of structure data.
- `update_entity_collision_and_state`: Checks horizontal coordinate bounds between two entities and updates their respective sprite or attribute data based on the result.
- `update_entity_attributes`: Updates various fields in a structure, including setting word values, performing arithmetic on coordinates, and calling a routine to find a nearby value.
- `update_scrolling_text_buffer`: Manipulates a character buffer by shifting data and modifying individual bytes to create a scrolling or cycling text effect.
- `handle_game_state`: Checks a global variable and calls another routine, potentially resetting a state structure and toggling a global state.
- `reset_state_structure`: Calls another routine using parameters derived from a table and then clears several fields within a structure.
- `update_entity_state`: Iterates through a list of entities to update their properties and executes a function pointer associated with each entity.
- `initialize_data_structure`: Sets several fields in a data structure to zero or specific constant values and waits for a bit in a byte to be cleared.
- `reset_game_state_and_hardware`: Synchronizes the system state by waiting for player input, toggling a global state variable, reconfiguring hardware registers, and repopulating a data table.
- `update_game_state_and_timers`: Updates several global variables and memory locations based on conditional checks and arithmetic operations.
- `update_score_display`: Checks specific global status flags and calculates a numeric value based on a global variable.
- `translate_rows_via_lookup_table`: Walks a 1000-entry block of 20-byte rows, 40 bytes apart, and for each source byte stores a value fetched from a 256-entry byte table into the destination row in reverse order.
- `clear_data_register_zero`: Performs a logical OR operation on a byte with zero in a data register.
- `clear_data_register_zero_2`: Applies a logical OR operation with zero to a byte in a data register.
- `copy_data_table`: Copies data from a source table to a destination table using a nested loop structure.
- `call_motion_calculation`: Calls a routine to calculate and store motion-related properties into an object structure.
- `process_recursive_motion_2`: Modifies values within a structure and updates a global variable.
- `process_recursive_motion`: Performs a recursive check on a value derived from a structure offset and a global variable against a constant limit.
- `wait_for_player`: A large routine that manages system state, blitter transfers, and entity updates while waiting for a specific event.
- `process_string_characters`: Iterates through a null-terminated string and, for every non-space character, writes several values to a structure and calls another routine multiple times.
- `process_string_data`: Iterates through a null-terminated string and performs multiple writes to a data structure for every non-space character found.
- `process_string_data_2`: Iterates through a null-terminated string and performs multiple writes to a structure whenever a non-space character is encountered.
- `initialize_entity_state_and_loop`: Initializes several global variables and entity properties by reading values from various pointers, followed by a series of conditional setup routines and a loop that iterates through a memory range to update specific data structures.
- `tiny_ori_8`: Small routine of 1 instruction that merges 16 into d0.
- `update_global_pointer`: Sets a global variable to two different constant values sequentially.
- `set_global_values`: Sets a global variable to two different word values sequentially while calling another routine between the assignments.
- `execute_conditional_setup`: Performs a series of conditional checks on a global variable to trigger various setup and hardware parameter routines.
- `setup_hardware_parameters`: Calculates offsets and writes multiple values to a data structure, including a loop that performs repeated writes to specific memory locations.
- `configure_audio_hardware`: Writes values to memory locations and hardware registers to set specific parameters and then updates a global variable.
- `apply_bitwise_mask`: Traverses a list of values to perform bitwise AND and OR operations on large blocks of memory using a mask table.
- `apply_bitwise_masks`: Iterates through a list of pointers to perform bitwise AND and OR operations on large blocks of memory using values from a table.
- `clear_memory_blocks`: Clears various fields within two distinct sets of memory blocks by writing zero to specific offsets.
- `initialize_and_update_object_state`: Sets up hardware parameters by writing offsets to a data structure and then updates object properties by combining data and writing it to a memory structure.
- `display_knockout_score`: Initializes a data structure for graphical display and uses blitter operations to clear memory areas.
- `apply_bitmask_to_memory`: Uses a small table of bytes to perform bitwise AND and OR operations on a destination buffer in increments of eight thousand bytes.
- `setup_blitter_area_tables`: Initializes a series of global variables with specific values and then enters a nested loop to populate a table of memory addresses.
- `update_entity_timers_and_recycle_list`: Iterates through a list of entity structures to decrement active timers and transition states when timers reach zero.
- `traverse_and_update_object_list`: Iterates through a linked list of objects to perform conditional updates and property synchronization.
- `find_nearest_opponent_target`: Scans the table of up to 32 object pointers, skipping inactive entries and the caller itself, to select the candidate whose vertical position lies within a window around the caller and is closest to it.
- `find_target_collision_object`: Iterates through a list of objects to find the closest match within a specific coordinate bounding box relative to a primary object.
- `tiny_ori_9`: Small routine of 1 instruction that merges 0 into d6.
- `find_closest_match`: Iterates through a list of objects to find the one whose coordinates most closely match a target area, storing the result in a provided location.
- `toggle_byte_flag`: Compares two bytes at a specific offset within two different structures.
- `update_object_spatial_bounds`: Calculates and updates the positional coordinates and bounding box dimensions for an object.
- `iterate_and_update_objects`: Repeatedly calls a routine that updates object state fields until that routine returns zero.
- `update_object_state`: Updates several fields in a structure, including coordinate offsets and status flags, based on the values of another structure.
- `calculate_minimum_distance`: Searches a list for an object that satisfies specific criteria and calculates a value based on its distance from a target.
- `configure_blitter_parameters`: Sets various Blitter hardware registers and updates a structure with calculated values after checking a condition.
- `update_blitter_parameters`: Checks a global variable and a pointer to determine if it should update several memory locations and Blitter hardware registers.
- `initialize_hardware_parameter_blocks`: Populates a series of data structures with calculated memory addresses and constant values.
- `setup_blitter_transfer_2`: Writes constant values and calculated addresses to a block of memory and waits for a hardware status bit to clear.
- `initialize_blitter_transfer`: Writes specific memory addresses and constant values to a block of memory and waits for a hardware status bit to clear.
- `configure_memory_parameters`: Writes a series of constant values, calculated addresses, and repeated byte patterns to a memory block.
- `initialize_hardware_parameters`: Writes a series of values to specific memory locations and waits for a hardware status bit to clear multiple times.
- `set_memory_parameters`: Writes several constant values and calculated addresses to a block of memory, then waits for a hardware status bit to clear.
- `populate_data_structure`: Writes a series of values and pointers into a structure pointed to by a register.
- `update_object_attributes`: Repeatedly calls a routine to update object properties and sprite attribute tables.
- `decrement_counter_loop`: Initializes a register with a constant value and executes a loop that decrements the value until it reaches negative one.
- `sort_list_descending`: Performs a bubble sort on a list of pointers based on a value stored at an offset within each pointed-to structure.
- `convert_integer_to_string`: Converts a decimal integer into a sequence of ASCII characters stored in a memory buffer and calls another routine.
- `write_structure_fields`: Writes several constant values into a structure at specific offsets.
- `write_structure_data`: Calculates a bitwise value and writes several fields into a structure at a specific offset, then waits for a status bit to clear.
- `clear_memory_tables`: Fills several large blocks of memory with zeros or specific byte patterns using nested loops.
- `initialize_game_state_tables`: Sets a global state variable and populates multiple memory tables with configuration values, timing parameters, and pointers.
- `render_sprite_sequence_2`: Sequentially calls other routines to update global memory values and then writes specific data to various structures.
- `render_sprite_sequence`: Populates several data structures with source and destination addresses and parameters before executing a series of blitter operations.
- `execute_blitter_sequence`: Performs a sequence of operations including configuring Blitter registers, writing multiple address pairs, and waiting for hardware status bits.
- `write_multiple_offsets`: Writes a sequence of four different address pairs into a structure and waits for a hardware status bit to clear after each write.
- `store_and_wait_input`: Stores a pointer into a global variable and waits for a specific bit to clear in a memory location after an interrupt.
- `wait_for_input_bit`: Polls a bit in a memory location until it is cleared after an interrupt.
- `configure_blitter_registers`: Writes specific values to several Blitter hardware registers.
- `execute_blitter_operation`: Sets Blitter hardware registers and updates several memory locations, including a value from a global variable, while waiting for a hardware status bit.
- `update_data_table_entries`: Writes four sets of values to a table at specific offsets and waits for a bit to clear between each write.
- `setup_blitter_parameters`: Updates a global variable and populates several memory locations with specific values before waiting for a bit in a memory location to clear.
- `set_global_value`: Sets a global variable to a specific value and calls another routine.
- `execute_conditional_subroutines`: Updates a global variable and populates several memory locations before checking the value of that variable to determine how many times another routine should be called.
- `initialize_object_attribute_table`: Initializes a data structure by clearing several fields and populating a series of offsets with incrementing values and calculated memory pointers.
- `unpack_compressed_data_block`: Searches through memory for a specific multi-byte signature to locate a data header.
- `configure_blt_registers`: Sets several Blitter hardware registers and waits for a specific bit in a global variable to clear.
- `configure_blitter_parameters_2`: Sets multiple Blitter control and mode registers and waits for the Blitter to become idle by polling a status bit.
- `calculate_object_motion_parameters`: Calculates and stores motion-related properties including position, velocity, and timing offsets into an object structure.
- `generate_lookup_table`: Calculates a series of values using a mathematical loop and stores them in a table in memory.
- `fill_character_buffers`: Populates two separate memory areas with repeating sequences of characters and null terminators.
- `fill_character_buffers_2`: Writes a pattern of characters and specific byte values into two distinct memory locations using a loop.
- `fill_memory_blocks`: This routine performs a series of repetitive memory writes using values derived from input registers.
- `return_zero_stub`: The routine is a single instruction, entered only through address-taken references, so it does nothing beyond returning control to its caller.
- `bulk_fill_memory_blocks`: Clears a memory range and then performs a series of repetitive, multi-word block writes to various memory locations using values derived from input registers and existing table data.
- `initialize_and_populate_data_structures`: Clears a memory range and then performs a series of repetitive, multi-word block writes to various memory locations using values derived from input registers and a table.
- `clear_memory_block`: Initializes a global variable with two words and then clears a sequence of memory locations using a loop.
- `calculate_and_blit_sprite_data`: Calculates sprite dimensions and bitplane offsets using mathematical transformations, then performs a series of Blitter transfers to populate memory blocks with processed graphical data.
- `setup_blt_parameters`: Configures several memory locations and hardware bitplane/size registers before waiting for a DMA completion flag.
- `execute_blt_transfer`: Updates specific memory locations and hardware bitplane/size registers and waits for a DMA completion flag.
- `execute_masked_clears`: Calls another routine twice with different values stored in a global variable.
- `clear_and_mask_bitmaps`: Resets specific memory blocks to zero or all ones and applies a bitmask to a series of data buffers.
- `update_pointer_table_entries`: Iterates through a loop to update a series of multi-byte values in a structure using offsets and increments derived from a global variable and a table.
- `update_sprite_attribute_table`: Iterates through a table of data to calculate and update sprite position and attribute values.
- `update_blitter_object_attributes`: Updates object properties and sprite attribute tables by calculating new positions and bitwise combinations of data.
- `empty_routine`: This routine is empty and contains no executable instructions.
- `toggle_global_state`: inverts a global variable and updates a memory location with different values based on the result.
- `tiny_ori_10`: Small routine of 1 instruction that merges 104 into d1.
- `initialize_data_table`: stores a series of constant values and paired integers into a global variable sequentially.
- `calculate_offset_sequence`: performs a series of arithmetic operations and bit shifts on a value to update a global variable.
- `update_pointer_arithmetic`: performs a sequence of additions and bit shifts on a value and stores the result in a global variable.
- `update_object_properties`: writes multiple values to a structure pointed to by a register, including bitwise combinations of other data.
- `generate_bit_pattern_table`: Calculates a series of bit-shifted values based on an incrementing counter and stores them in a memory sequence.
- `perform_indirect_table_substitution`: Reads a value from a pointer, uses it as an index to fetch a byte from a table, and writes that byte back to memory.
- `update_blitter_source_table`: Constructs a source data table by fetching values from a lookup table based on a specific pattern size determined by a global variable.
- `store_byte_and_loop`: Stores a byte from a register into a memory location pointed to by a register and enters a loop that checks a bit in a byte at a specific offset.
- `clear_memory_range`: fills a specific range of memory with zeros using a loop.
- `set_configuration_parameters`: initializes a large number of global variables with specific constant values and conditional logic.
- `populate_attribute_table`: fills a large table in memory with various constant values, pointers, and configuration settings.
- `execute_loop_count`: Executes another routine three times using a decrementing counter.
- `process_attribute_list`: iterates through a list of data to calculate values and update a structure in memory.
- `update_object_properties_2`: Iterates through a list of structures, performing bitwise shifts and multiplications on data before writing the results back to each structure via a routine.
- `configure_hardware_registers`: Clears several memory locations and writes values to various hardware registers including display, DMA, and sprite control registers.
- `populate_data_table`: Writes a large sequence of hardcoded words into a memory range and then fills a specific memory range with zeros.
- `copy_data_block`: Copies thirty-two words from a source table to a destination located at an offset from a pointer.
- `copy_data_from_pointer`: Copies thirty-two words from a source pointer to a destination located at an offset from another pointer.
- `copy_memory_block_4`: Copies a block of thirty-two bytes from a fixed source address to a destination address offset from a register value.
- `setup_blitter_transfer`: Sets several blitter registers and waits for a specific bit in a global variable to clear.
- `tiny_ori_11`: Small routine of 1 instruction that merges 20 into d2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_indirect_data`: Traverses a data structure using indirect pointers and performs zero-fills on a memory range when a null byte is encountered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_pointer_table`: Traverses a table of pointers to update memory locations and clears a range of memory when a null terminator is encountered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `resolve_relative_pointers`: Iterates through a list of relative offsets to update a table with absolute addresses and clears a memory range upon finding a zero byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_indirect_pointer_table`: Iterates through a table of pointers to update memory locations based on values found in a linked list or nested pointer structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `nop_placeholder`: This routine is empty and performs no operations.

### logged disk load from Disk.1

- `initialize_and_loop`: Sets several global variables to specific values, calls another routine, and then enters an infinite loop.
- `setup_blt_registers`: Sets several Blitter registers and waits for a specific bit in a global variable to clear.
- `execute_sequence_of_calls`: Sets global variables, calls two specific routines, and then enters an infinite loop.
- `copy_data_blocks`: Copies blocks of data from one memory location to another based on the value of a global variable.
