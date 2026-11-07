# Game 1678 (super cars ii (1991)(gremlin)[cr skr][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01678). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1810256 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 97 raw-word lines |
| Functions named and described | pass, 46 of 46 named (0 author symbols), 46 of 46 described (3 by rule, 35 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 738 of 738 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1186 of 1186 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 28150 |
| code | 4078 |
| data | 1764716 |
| unused | 13312 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Super Cars II (1991)(Gremlin)[cr SKR][a](Disk 1 of 2).adf:bootblock | 1 | 97 | 5 |
| Super Cars II (1991)(Gremlin)[cr SKR][a](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| logged disk load from Super Cars II (1991)(Gremlin)[cr SKR][a](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 47 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 83 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 146 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 803 | 32 |

## Cases handled

- super cars ii (1991)(gremlin)[cr skr][a](disk 1 of 2)/Super Cars II (1991)(Gremlin)[cr SKR][a](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1732 non-empty blocks: matched against track-loaded segments
- super cars ii (1991)(gremlin)[cr skr][a](disk 2 of 2)/Super Cars II (1991)(Gremlin)[cr SKR][a](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- super cars ii (1991)(gremlin)[cr skr][a](disk 1 of 2)/Super Cars II (1991)(Gremlin)[cr SKR][a](Disk 1 of 2).adf: the boot block reads 25600 bytes from disk offset 872448 to an address (found statically); recovered as a unit
- runtime segment an address: 33 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 81 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 43 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 581 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- super cars ii (1991)(gremlin)[cr skr][a](disk 1 of 2)/Super Cars II (1991)(Gremlin)[cr SKR][a](Disk 1 of 2).adf: 1 logged loads cover 25600 bytes of the image
- 2 packed track region(s), 1761280 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Super Cars II (1991)(Gremlin)[cr SKR][a](Disk 1 of 2).adf:bootblock

- `clear_interrupts_then_halt`: Sets up a parameter block, calls a library routine through a negative vector offset, writes a mask that clears all interrupt enables, calls one more routine, and then enters an endless loop.
- `depack_powerpacker_block`: After checking the PP20 signature, which is the ASCII text of the PowerPacker format, it unpacks the block backwards.
- `test_upper_bits_no_effect`: Tests whether a register has any bits above its lowest bit, then returns in both cases.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `read_bits_backwards_from_stream`: Loops over a count, shifting bits out of a register one at a time.

### Super Cars II (1991)(Gremlin)[cr SKR][a](Disk 2 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `select_table_entry_or_default`: Calls a helper and returns early if a flag stays unset.
- `update_table_and_register`: Loads a value from a table using an index and stores a register value into a global variable.
- `clear_cia_control_bit`: Clears bit six of the first CIA control register, writes an all-ones value to a shared register, and saves the previous byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decode_joystick_port_state`: Reads the mouse and joystick port movement registers, the right and middle button register and the fire-button register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_without_action`: Returns straight away with no work.
- `acknowledge_copper_interrupt`: When the copper interrupt request bit is set, it writes that bit back to acknowledge it, calls two helpers, and updates a frame counter that resets to a fixed value when it passes a limit, along with some global flags.
- `return_without_visible_work`: The decompiler shows only a return for this routine, although the listing has 81 instructions, so its body is not reliably visible.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_and_copy_asset_header`: Checks a header magic number and, if a match is found, copies a block of data from a source pointer to a destination, while calculating a total size or offset.
- `process_table_with_shifts`: Repeatedly shifts a value and updates it using a table lookup while decrementing a counter initialized from a global variable.
- `shift_and_decrement_counter`: Performs a series of bit shifts on a value and decrements a counter, potentially loading a new value from a table if a zero-check fails.
- `looped_shift_with_table_lookup`: The routine repeatedly shifts an input value, using a table pointer when the value becomes zero, decrementing a short counter derived from an input register until it reaches minus one.
- `copy_ring_run_decrement`: Decrements a global variable and copies a block of memory backwards using a pointer.
- `copy_ring_run_no_counter`: Copies a block of memory backwards using a pointer without using a counter-based loop termination.
- `copy_ring_run_increment`: Increments a global variable and copies a block of memory backwards using a pointer.
- `copy_ring_run_increment_two`: Increments a global variable by two and copies a block of memory backwards using a pointer.
- `copy_ring_run_increment_three`: Increments a global variable by three and copies a block or byte backwards using a pointer.
- `copy_memory_backwards`: Copies a block of memory backwards using a pointer and a counter initialized from a global variable.
- `copy_memory_backwards_offset_five`: Copies a block of memory backwards using a pointer and a counter initialized from a global variable plus a constant.
- `shift_and_copy_memory_block`: Calculates a source and destination index, then copies a block of memory backwards using a decrementing counter.
- `copy_memory_backwards_offset_nine`: Copies a blockly of memory backwards using a pointer and a counter initialized from a global variable plus a constant.
- `shift_and_copy_memory_block_2`: Calculates a source and destination address, then copies a block of memory backwards using a decrementing counter.
- `shift_and_copy_memory_backwards`: Calculates a source and destination index, then copies a block of memory backwards using a decrementing counter.
- `copy_ring_run_offset53`: Sets a counter to a certain value plus a constant, then copies a block of memory backwards using a pointer.
- `fill_ring_buffer_decrement`: Writes a byte value repeatedly backwards through a ring buffer.
- `fill_ring_buffer_run`: Writes a byte value repeatedly backwards through a ring buffer.
- `write_byte_to_buffer_loop`: Writes a byte value to a memory location while decrementing a pointer and a counter.
- `write_byte_to_buffer`: Writes a byte value to a memory location while decrementing a counter and a pointer.
- `write_buffer_with_decrement`: Writes a byte to a memory location using a pointer and a counter that decrement or mask during each iteration.
- `transpose_bitplane_words`: Reads sixteen bytes and gathers each bit position across them into a 16-bit word, producing certain bit-plane words that are stored in a fixed output area.
- `partial_transpose_bitplane_words`: Reads a sequence of bytes and extracts certain bit positions into a set of destination words.
- `init_eight_table_entries`: Initialises eight consecutive eight-byte table entries, writing a fixed word and the value seven into each.
- `call_subroutines_if_flagged`: Checks if a global flag is non-zero, then calls a routine to reset audio channel state and another to advance a sequence step counter.
- `reset_audio_state`: Writes a constant value to a memory location and clears a global variable.
- `reset_audio_channel_state`: Scans a table of values for the first entry above the current one, then clears a block of channel records and links their pointers.
- `advance_sequence_step_counter`: Increments a global variable and resets it to zero if a certain threshold is reached, while also incrementing another global offset.
- `set_four_audio_pointers`: Runs a helper routine four times, once for each audio channel.
- `start_audio_dma_channels`: Clears a global, performs delays, enables audio DMA channels, and loads four audio location pointers from a table.
- `parse_music_row_entry`: Parses one four-byte entry of music data for an audio channel.
- `apply_channel_effect_tick`: Updates a music channel's period or pitch using a predefined effect type, such as a periodic pitch cycle, a linear period slide, or a table-driven frequency change, while clamping the resulting period to a minimum value or a target limit.
