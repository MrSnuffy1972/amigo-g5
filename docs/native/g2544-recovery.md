# Game 2544 (nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02544). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906480 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 15 of 15 named (0 author symbols), 15 of 15 described (0 by rule, 15 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 429 of 429 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 610 of 610 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12132 |
| code | 2162 |
| data | 608026 |
| unused | 284160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a].adf:bootblock | 1 | 63 | 2 |
| logged disk load from Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 547 | 13 |

## Cases handled

- nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a].adf: custom-format (non-DOS) disk, 1203 non-empty blocks: matched against track-loaded segments
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a].adf: the boot block reads 10752 bytes from disk offset 716800 to an address (found statically); recovered as a unit
- runtime segment an address: 429 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a].adf: 1 logged loads cover 10752 bytes of the image
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a].adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.02)
- 1 packed track region(s), 604672 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a].adf:bootblock

- `setup_display_hardware`: Configures various display registers including bitplane modes, copper control, and video timings, then enables DMA. It performs three library calls and swaps the high and low bytes of each word in a table.
- `initialize_and_swap_data`: Performs three library calls to set up parameters and then loops through a table to swap the high and low bytes of each word.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_main_loop`: Enters a loop that repeatedly calls four different routines based on pointers stored in a structure.
- `copy_memory_block`: Copies a large block of data from a global variable to a destination starting at a fixed address.
- `clear_memory_range`: Iterates through a memory range starting at a given pointer and sets each word to zero until a specified limit is reached.
- `load_table_values`: Uses an index to fetch a byte from a table and then copies several specific bytes from a memory range into a destination buffer at fixed offsets.
- `count_null_bytes`: Iterates through a string until a null byte is found and then stores a calculated value into a destination offset.
- `update_string_index`: Checks a character value and iterates through a string to find a non-zero byte, then updates a global table with calculated values based on the resulting index.
- `decrement_and_wrap_counter`: Decrements a byte in a structure and, if it reaches zero, performs logic to increment or decrement a counter and update a status byte.
- `update_score_display`: Decrements a counter and, upon reaching zero, updates a numeric value in a data structure and converts it into a three-character ASCII string for display.
- `update_dma_control_register`: Updates a global variable and modifies a DMA control register to enable DMA channels.
- `parse_compressed_data_stream`: Decodes a custom bitstream or compressed data format by reading bytes and extracting multi-bit values.
- `parse_sample_sequence`: Reads and processes a sequence of sample data from a table to update object properties.
- `update_object_kinematics`: Calculates and updates the position and velocity of an object based on data retrieved from a table.
- `update_channel_period_and_vibrato`: Updates a channel's period and vibrato state from its instrument entry, applying a counter-driven sweep and a sign flip, then stores the new period and volume.
