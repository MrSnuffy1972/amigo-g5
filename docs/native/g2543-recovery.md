# Game 2543 (nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02543). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906480 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 16 of 16 named (0 author symbols), 16 of 16 described (1 by rule, 15 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 429 of 429 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 681 of 681 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 21930 |
| code | 2216 |
| data | 603294 |
| unused | 279040 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a2].adf:bootblock | 1 | 7 | 1 |
| logged disk load from Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a2].adf boot-block load | 1 | 127 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 547 | 13 |

## Cases handled

- nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a2]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a2].adf: custom-format (non-DOS) disk, 1190 non-empty blocks: matched against track-loaded segments
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a2]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a2].adf: the boot block reads 20480 bytes from disk offset 606208 to an address (found statically); recovered as a unit
- runtime segment an address: 429 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a2]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a2].adf: 1 logged loads cover 20480 bytes of the image
- 1 packed track region(s), 600576 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a2].adf:bootblock

- `init_and_copy_table`: Initializes a set of fixed values in a data structure, then invokes a startup routine.

### logged disk load from Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a2].adf boot-block load

- `tiny_clr`: Small routine of 1 instruction that clears d0.
- `xor_byte_pattern`: Walks through a 32-bit value one byte at a time, flipping the matching byte of a global variable with an XOR whenever that byte is zero, then decrements the value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_copper_and_registers`: Waits for the copper to reach a specific line, then programs several hardware registers.
- `process_byte_table`: Copies a block of words from a source to a destination in chip memory, then walks a table of byte values, dispatching to different routines for each value and copying a variable-length run of bytes for the ones it recognizes.
- `zero_memory_range`: It zeroes a run of consecutive memory words by writing 0 to each location in turn, advancing one word at a time until it reaches the end address.
- `load_table_values`: Uses an index to fetch a byte from a table and then copies several specific bytes from a memory range into a destination buffer at fixed offsets.
- `count_null_bytes`: Iterates through a string until a null byte is found and then stores a calculated value into a destination offset.
- `update_string_index`: Checks a character value and iterates through a string to find a non-zero byte, then updates a global table with calculated values based on the resulting index.
- `decrement_and_wrap_counter`: Decrements a byte in a structure and, if it reaches zero, performs logic to increment or decrement a counter and update a status byte.
- `manage_timer_and_mode`: Decrements a countdown timer and, when it reaches zero, resets it and updates a status byte.
- `decode_command_sequence_2`: Decrements a counter in a global structure and, when it reaches zero, decodes a sequence of command bytes from a table by repeatedly decoding length-prefixed strings.
- `decode_length_prefixed_string`: Decrements a counter stored in a data structure and then decodes a length-prefixed string from a table, splitting it into a text pointer and a byte count while advancing past the encoded bytes.
- `decode_command_sequence`: Decrements a counter stored in a global structure and, when it underflows, decodes a sequence of command bytes from a table.
- `apply_movement_update`: Reads a movement table entry selected by an offset and updates a structure's coordinates and a timer value, then applies a direction derived from a nibble of a table byte.
- `update_channel_period_and_vibrato`: Updates a channel's period and vibrato state from its instrument entry, applying a counter-driven sweep and a sign flip, then stores the new period and volume.
