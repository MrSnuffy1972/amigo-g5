# Game 1677 (super cars ii (1991)(gremlin)[cr skr](disk 1 of 2)[t +9 setrox]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01677). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909136 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 97 raw-word lines |
| Functions named and described | pass, 45 of 45 named (0 author symbols), 45 of 45 described (2 by rule, 35 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 738 of 738 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1185 of 1185 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 28150 |
| code | 4106 |
| data | 869200 |
| unused | 7680 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Super Cars II (1991)(Gremlin)[cr SKR](Disk 1 of 2)[t +9 Setrox].adf:bootblock | 1 | 106 | 5 |
| logged disk load from Super Cars II (1991)(Gremlin)[cr SKR](Disk 1 of 2)[t +9 Setrox].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 47 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 83 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 146 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 803 | 32 |

## Cases handled

- super cars ii (1991)(gremlin)[cr skr](disk 1 of 2)[t +9 setrox]/Super Cars II (1991)(Gremlin)[cr SKR](Disk 1 of 2)[t +9 Setrox].adf: custom-format (non-DOS) disk, 1743 non-empty blocks: matched against track-loaded segments
- super cars ii (1991)(gremlin)[cr skr](disk 1 of 2)[t +9 setrox]/Super Cars II (1991)(Gremlin)[cr SKR](Disk 1 of 2)[t +9 Setrox].adf: the boot block reads 25600 bytes from disk offset 872448 to an address (found statically); recovered as a unit
- runtime segment an address: 33 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 81 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 43 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 581 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- super cars ii (1991)(gremlin)[cr skr](disk 1 of 2)[t +9 setrox]/Super Cars II (1991)(Gremlin)[cr SKR](Disk 1 of 2)[t +9 Setrox].adf: 1 logged loads cover 25600 bytes of the image
- 2 packed track region(s), 866816 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Super Cars II (1991)(Gremlin)[cr SKR](Disk 1 of 2)[t +9 Setrox].adf:bootblock

- `install_copper_list_then_halt`: Issues a library request at offset 456, writes copper list words and the copper location and jump registers, and writes the interrupt enable register to disable all interrupts.
- `unpack_pp20_stream`: Checks for the PP20 signature tag and, when it is present, unpacks the packed data backwards with a bit-stream decoder, copying literal and back-reference runs into the output.
- `test_upper_bits_no_effect`: Tests whether a register has any bits above its lowest bit, then returns in both cases.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `shift_bit_buffer_with_carry`: Shifts a 32-bit bit buffer right by one step per count, moving the carried bit into the top bit and reloading the buffer from memory when it empties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `select_table_entry_or_default`: Calls a helper and returns early if a flag stays unset.
- `update_table_and_register`: Loads a value from a table using an index and stores a register value into a global variable.
- `clear_cia_control_bit`: Clears bit six of the first CIA control register, writes an all-ones value to a shared register, and saves the previous byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decode_joystick_direction_bits`: Reads the joystick port data register and decodes its bits into direction flags using the exclusive-or of adjacent bits, then stores the flags and their change from the previous state.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_empty_return`: Returns immediately with no operations, so it is an empty stub reached only as unreached code.
- `service_movement_interrupt_request`: Interrupt handler that reads the interrupt request register and, when bit four is set, clears that request, reads counter values, updates a state word and calls the matching handler from a function table.
- `stub_empty_body_routine`: The decompiler gave an empty body for these 81 instructions, so no behaviour is recovered from the evidence.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_and_copy_resource_header`: Checks a header or resource identifier and, if a match is found, copies a block of data from a source pointer to a destination, while updating a global counter or pointer.
- `process_table_with_shifts`: Iteratively shifts a value and updates it using a value from a table when the value reaches zero, controlled by a counter.
- `shift_and_decrement_counter`: Repeatedly shifts a value and decrements a counter, potentially loading a new value from a pointer if the value becomes zero.
- `looped_shift_with_table_lookup`: The routine repeatedly shifts an input value, using a table pointer when the value becomes zero, decrementand a short counter derived from an input register until it reaches minus one.
- `copy_memory_backwards_decrement_counter`: Copies a block of memory backwards using a pointer while decrementing a counter initialized from a global variable.
- `copy_memory_backwards_no_offset`: Copies a block of memory backwards using a pointer without adjusting the initial counter value.
- `copy_memory_backwards_increment_counter`: Copies a block ofmemory backwards using a pointer while incrementing the initial counter value.
- `copy_memory_backwards_increment_two`: Copies a block of memory backwards using a pointer while incrementing the initial counter by two.
- `copy_memory_backwards_increment_three`: Copies a block of memory backwards using a pointer while incrementing the initial counter by three.
- `copy_memory_backwards`: Decrements a counter and copies a block of memory backwards using a pointer.
- `copy_memory_backwards_offset_5`: Decrements a counter and copies a block of memory backwards using a pointer, starting with an offset of five.
- `copy_memory_backwards_offset_7`: Decrements a counter and copies a block of memory backwards using a pointer, starting with an offset of seven.
- `copy_memory_backwards_offset_9`: Decrements a counter and copies a block or memory backwards using a pointer, starting with an offset of nine.
- `copy_memory_backwards_offset_13`: Decrements a counter and copies a block of memory backwards using a pointer, starting with an offset of thirteen.
- `copy_memory_backwards_offset_21`: Decrements a counter and copies a block of memory backwards using a pointer, starting with an offset of twenty-one.
- `copy_ring_run_offset53`: Sets a counter to a certain value plus a constant, then copies a block of memory backwards using a pointer.
- `fill_ring_buffer_decrement`: Writes a byte value repeatedly backwards through a ring buffer using a loop counter that is decremented by one.
- `fill_ring_buffer_run`: Writes a byte value repeatedly backwards through a ring buffer.
- `fill_memory_with_value_2`: fills a block of memory with a specific byte value using a decrementing counter.
- `fill_memory_with_value`: Fills a block of memory with a specific byte value using a decrementing counter.
- `fill_memory_with_value_loop`: Fills a block of memory with a specific byte value using a loop that decrements a counter.
- `transpose_bitplane_words`: Reads sixteen bytes and gathers each bit position across them into a 16-bit word, producing certain bit-plane words that are stored in a fixed output area.
- `partial_transpose_bitplane_words`: Reads a sequence of bytes and extracts certain bit positions into a set of destination words.
- `init_eight_table_entries`: Initialises eight consecutive eight-byte table entries, writing a fixed word and the value seven into each.
- `call_subroutines_if_flagged`: Checks if a global flag is non-zero, then calls a routine to reset audio channel state and another to reset global counters.
- `set_global_value`: Writes a constant value to a global variable and clears another global variable.
- `reset_audio_channel_state`: Scans a table of values for the first entry above the current one, then clears a block of channel records and links their pointers.
- `reset_global_counters`: Resets a global variable to zero and increments a global pointer or value until a certain limit is reached.
- `set_four_audio_pointers`: Runs a helper routine four times, once for each audio channel.
- `start_audio_dma_channels`: Clears a global, performs delays, enables audio DMA channels, and loads four audio location pointers from a table.
- `parse_music_row_entry`: Parses one four-byte entry of music data for an audio channel.
- `apply_channel_effect_tick`: Updates a music channel's period or pitch using a predefined effect type, such as a periodic pitch cycle, a linear period slide, or a table-driven frequency change, while clamping the resulting period to a minimum value or a target limit.
