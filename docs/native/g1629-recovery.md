# Game 1629 (james pond 2 - codename robocod (1991)(millennium)[cr skr][t +3 pns]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01629). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908016 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 45 of 45 named (0 author symbols), 45 of 45 described (3 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1080 of 1080 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2053 of 3397 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 49201 |
| code | 10056 |
| data | 733047 |
| unused | 115712 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +3 PNS].adf:bootblock | 1 | 28 | 2 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +3 PNS].adf boot-block load | 1 | 1438 | 5 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +3 PNS].adf boot-block load | 1 | 0 | 0 |
| logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +3 PNS].adf boot-block load | 1 | 107 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1824 | 36 |

## Cases handled

- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +3 pns]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +3 PNS].adf: custom-format (non-DOS) disk, 1522 non-empty blocks: matched against track-loaded segments
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +3 pns]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +3 PNS].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +3 pns]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +3 PNS].adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +3 pns]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +3 PNS].adf: the boot block reads 40960 bytes from disk offset 833536 to an address (found statically); recovered as a unit
- runtime segment an address: 1080 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +3 pns]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +3 PNS].adf: 3 logged loads cover 53248 bytes of the image
- james pond 2 - codename robocod (1991)(millennium)[cr skr][t +3 pns]/James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +3 PNS].adf: blocks 4-1431 (bytes 2048-733184) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.54)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +3 PNS].adf:bootblock

- `run_io_and_enable_dma`: Boot-block entry that fills an I/O request structure with fixed sizes and offsets, calls exec DoIO through the library base, and sets DMA control bits.
- `perform_boot_io_request`: Sets up an I/O request structure with a size and an offset, calls the exec library routine through the global base, then calls two further routines and returns the first routine's result.

### logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +3 PNS].adf boot-block load

- `unpack_backward_bitstream_block`: Decodes a bit-packed stream read from the end of the input backwards, writing bytes toward the start of the destination.
- `test_bit_register_no_effect`: Tests whether the shifted bit register is non-zero and returns on either branch, so it has no visible effect.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `read_bits_from_word_stream`: Shifts bits out of a bit register one at a time for a count held in a data register, reading a new word from memory whenever the register runs out.
- `copy_marker_bytes_then_call`: A 1360-instruction routine whose pseudo-code sets bit patterns in two registers, waits for a flag, and copies bytes selected by a marker-coded string into memory.

### logged disk load from James Pond 2 - Codename Robocod (1991)(Millennium)[cr SKR][t +3 PNS].adf boot-block load

- `write_byte_return_halved_value`: Writes the low byte of a length-like value into a destination word, keeping the high byte of another register.
- `shift_bit_register_by_count`: Shifts a bit register right one step per count held in a data register, calling an unnamed routine to refill it whenever it empties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_and_display`: Initializes hardware registers including bitplane controls, DMA controllers, interrupt enables, and joystick/mouse input registers.
- `initialise_bitplanes_and_draw_tiles`: Clears a set of state words and writes the DMA control register with a fixed value.
- `clear_state_and_draw_tiles`: Adjusts a state value by a fixed multiply, clears ten state words, then walks a marker-coded string and copies an eight-row tile for each character into a 40-column bitmap.
- `draw_marker_string_tiles`: Walks a marker-coded string and copies an eight-row tile for each character into a bitmap, starting at the destination passed in and wrapping to the next band after forty characters.
- `copy_table_to_low_memory`: Copies 256 bytes from a fixed source table into a fixed low-memory destination, one byte at a time, with no return value.
- `tick_countdown_and_scale_value`: Decrements a countdown byte while it is non-zero, calling an unnamed routine when it reaches zero, then rescales a value by adding its low byte times sixteen.
- `draw_digit_tiles_from_text`: When a flag bit is set, it clears state fields and waits for the blitter to go idle, then reads a marker-coded string whose digit markers are converted to values.
- `read_mouse_position_delta`: Reads the mouse port movement register, computes the change from the previous reading and clamps it, then stores three values derived from the position divided by sixteen and shifted by three.
- `divide_offset_byte_by_eight`: Adds 126 to a byte, wraps the result to eight bits, divides it by eight and stores it back in the same location.
- `adjust_counter_on_button_input`: Reads the fire button and the right button state and, when a button state changes, increments or decrements a stored two-digit counter and redraws its digit tiles.
- `copy_strided_bytes_to_bitmap`: Copies eight bytes spaced forty bytes apart from a source to a destination computed from row and column values.
- `wait_with_countdown_loop`: Runs a short countdown loop from a start value taken from a global, returning when it reaches minus one.
- `wait_for_blitter_idle`: Polls the blitter-busy bit in the custom chip status word until it clears, then returns.
- `process_conditional_logic`: Checks a bit in a word at an offset from a pointer, masks that word, and then executes a sequence of calls to another routine based on the result.
- `program_blitter_registers_for_blit`: Briefly sets and then clears the blitter DMA enable bit around a wait for the blitter to go idle.
- `store_offset_address_pairs`: Stores two addresses derived from a base value, one offset by a fixed amount and one by a second amount, into two pairs of word locations, and returns the second address.
- `update_state_structure`: Checks if a global variable is non-zero, then writes several values to a structure pointed to by registers, including two word values and two longwords.
- `compute_masked_position_offsets`: Adjusts a stored coordinate by a fixed step when it is below a threshold, then computes differences from two fields of an object record and adds three fixed offsets.
- `transform_vertices_by_rotation`: Looks up sine and cosine style values from a table and builds a three-by-three rotation matrix in fixed-point.
- `draw_line_segments_with_blitter`: Reads pairs of point indices from a table until a negative terminator, looks up each pair of endpoints and orders them by vertical position.
- `compute_bounds_and_program_fill`: Scans forty-five pairs of projected coordinates for their extreme values, computes a start address and width, and programs the blitter with a fill control word after waiting for it to go idle.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 180 into d3, then merges 196 into -(a0).
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 180 into d4, then merges 4 into -(a0).
- `fade_colour_channels_toward_target`: Derives a brightness-like level from a cross product of coordinate differences and clamps it.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `reload_audio_channel_pointers`: Advances a counter and, when it wraps, calls four unnamed routines, waits in two countdown loops and loads the four audio channel location registers from a table of pointers.
- `run_channel_effect_tick`: Runs four unnamed routines, then interprets one effect command from a channel's state word, covering slide, set and scale operations, and writes the resulting value to the audio channel registers.
- `update_audio_channel_from_pattern`: Updates one audio channel from a note entry in a pattern table, looking up a period and sample length by note and copying the location and length registers.
- `apply_channel_modulation_effect`: Dispatches a channel effect from a command word and its parameter byte.
- `copy_channel_period_word`: Copies a word-sized value from a channel state field into the matching audio register field and returns.
- `lookup_period_for_target_value`: Searches a 36-entry table of 16-bit values for the first entry not below the current value, with a one-step adjustment when a flag bit is set.
- `slide_value_toward_target`: Moves a stored value toward a target by a per-tick step in the up or down direction, stopping at the target.
- `apply_waveform_offset_to_base`: Looks up a waveform value from a table at the current phase, scales it by a depth nibble, adds it to a base value and advances the phase by a rate.
- `dispatch_effect_command_with_input`: Dispatches a channel effect command from a command word, covering the same parameter families as the other effect dispatcher, and touches the CIA port A register for its input.
- `store_low_nibble_parameter`: Stores the low nibble of a parameter byte into a state field of the channel block.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
