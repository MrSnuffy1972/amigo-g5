# Game 2480 (resolution 101 (1990)(millenium)[cr cls][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02480). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904144 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 39 of 39 named (0 author symbols), 39 of 39 described (1 by rule, 18 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 296 of 296 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 732 of 732 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 320 |
| code | 2078 |
| data | 7564 |
| padding | 742 |
| unused | 893440 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Resolution 101 (1990)(Millenium)[cr CLS][a].adf:bootblock | 1 | 20 | 2 |
| logged disk load from Resolution 101 (1990)(Millenium)[cr CLS][a].adf boot-block load | 1 | 89 | 2 |
| logged disk load from Resolution 101 (1990)(Millenium)[cr CLS][a].adf boot-block load | 1 | 89 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 191 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 343 | 25 |

## Cases handled

- resolution 101 (1990)(millenium)[cr cls][a]/Resolution 101 (1990)(Millenium)[cr CLS][a].adf: the boot block reads 29184 bytes from disk offset 346112 to an address (found statically); recovered as a unit
- resolution 101 (1990)(millenium)[cr cls][a]/Resolution 101 (1990)(Millenium)[cr CLS][a].adf: the boot block reads 66048 bytes from disk offset 375296 to an address (found statically); recovered as a unit
- runtime segment an address: 51 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 245 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- resolution 101 (1990)(millenium)[cr cls][a]/Resolution 101 (1990)(Millenium)[cr CLS][a].adf: 2 logged loads cover 0 bytes of the image
- resolution 101 (1990)(millenium)[cr cls][a]/Resolution 101 (1990)(Millenium)[cr CLS][a].adf: blocks 11-21 (bytes 5632-11264) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `set_global_from_masked_byte` (runtime segment an address): extracts a masked byte from a pointer and stores it in a global variable. Kind: track/sector or manual/code-wheel lookup. Pass path: The routine executes a conditional branch or a jump-table-based indirect call (the `u4__d0_03D8` jump) which determines the next valid code/data block or valid lookup entry, effectively acting as a conditional branch or a way to skip/includ.

## Functions


### Resolution 101 (1990)(Millenium)[cr CLS][a].adf:bootblock

- `configure_hardware_and_jump`: Writes several values to offsets within a structure pointed to by a register, calls two subroutines, updates a global variable, and executes a software trap.
- `decode_stream_to_colour_register`: Decodes a bit-stream of literal bytes and back-referenced runs, writing each decoded byte to the background colour register as it goes.

### logged disk load from Resolution 101 (1990)(Millenium)[cr CLS][a].adf boot-block load

- `shift_bitstream_right_flagged`: Returns the longword just before the input pointer shifted right one place, with its top bit set from a flag bit tested in the D7 register.
- `shift_bitstream_count_loop`: Shifts the bit buffer right by one place for a count held in D1, counting down to zero.

### logged disk load from Resolution 101 (1990)(Millenium)[cr CLS][a].adf boot-block load

- `shift_and_store_word`: Loads a word from memory, stores a concatenated value into a memory location, and returns a longword with the most significant bit set.
- `loop_bit_shifts`: Repeats a loop that shifts a value right until it reaches zero, at which point it calls another routine to refresh the value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `conditionally_increment_global`: Compares two unsigned short values and, if the first is smaller than the second, adds a constant to a global variable.
- `read_mouse_button_state`: Reads the fire-button line from the CIA-A port register and the right and middle button lines from the potentiometer registers, updates a stored button state word, and returns the masked button bits.
- `test_pot_and_write_register`: Tests bits in d0 and d1, and if certain conditions are met, stores a pointer to a global variable and writes a value to the custom POTGO register before branching to another routine.
- `set_status_flag`: Sets the carry flag based on the value of a byte at a specific memory location and then returns.
- `check_input_and_set_pot`: Checks bits in the CIAA port and the custom POTGOR register, then conditionally sets bit 1 of d1 or writes a value to the custom POTGO register.
- `process_interrupt_and_input`: Handles an interrupt by checking the CIAA status, performing bitwise operations on a byte from the CIAA SDR, and then uses a table to update a global variable or a pointer based on a value retrieved from another memory location.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `service_bitplane_pointers_on_vblank`: On the vertical-blank interrupt bit, increments a frame counter and sets five bitplane pointers spaced 8000 bytes apart from a base address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `advance_tick_level_toward_target`: Advances an envelope counter on each tick.
- `step_channel_countdown_table`: Decrements a per-channel countdown and, when it expires, loads the next duration from a table.
- `update_channel_pitch_and_volume`: Applies a table-driven envelope and pitch step to a per-channel state, then writes the computed volume and period values to the custom chip registers for that channel.
- `reset_channel_state_and_dma`: Zeroes five per-channel state bytes and writes control values to the audio control and DMA control registers, turning off the four DMA channels.
- `interpret_sequence_stream_bytes`: Reads the next value from a byte sequence.
- `interpret_sequence_stream_from_start`: Copies the first byte of its source into a channel field, then runs the same byte-sequence interpreter as its sibling routine.
- `set_bits_and_branch`: Sets bits 0 and 5 of a byte at the address in a0 and then performs a tail branch to another routine.
- `set_bit_and_branch`: Sets bit 5 of a byte at the address in a0 and then performs a tail branch to another routine.
- `write_byte_to_offset`: stores a byte from a pointer into a global structure at a fixed offset.
- `initialize_structure_bytes`: writes several bytes to a global structure and sets a bit in a flag.
- `set_structure_bytes`: stores two bytes into a global structure and sets a bit in a flag.
- `write_byte_to_offset_alt`: stores a byte from a pointer into a global structure at a different offset.
- `set_flag_and_parse`: sets certain bits in a global structure before starting a parsing loop.
- `clear_structure_byte`: sets a byte in a global structure to zero before starting a parsing loop.
- `set_global_from_masked_byte`: extracts a masked byte from a pointer and stores it in a global variable.
- `tick_channel_countdown_then_read`: Decrements a per-channel countdown and adjusts a modulation byte while it stays non-zero.
- `interpret_sequence_stream_with_header`: Stores the first byte of its source as a channel header field, then runs the byte-sequence interpreter.
- `store_byte_to_offset`: stores a byte from a pointer into a global structure at a specific offset.
- `tick_duration_and_resume_sequence`: Decrements a duration counter and, while it stays non-zero, resumes reading from the saved sequence position; otherwise it continues from the caller's source pointer.
- `start_sequence_with_duration`: Takes the first byte of its source as a duration, stores the source offset as the channel's saved read position, and runs the byte-sequence interpreter.
- `set_structure_value`: sets a value in a global structure and calculates a length or offset.
- `update_structure_from_pointer`: stores bytes from a pointer into a global structure and calculates a value.
- `finish_channel_note_update`: Stores the difference between two offsets as the channel's saved read position and copies a counter field into its working counter.
- `load_channel_from_table_entry`: Looks up a table entry by the given index, copies one byte from it into a channel field, and copies a second value from a paired table into another channel field.
- `update_channel_sample_pointer_length`: Counts down a channel step state, and on each step loads a start offset and a length word from a per-channel table.
