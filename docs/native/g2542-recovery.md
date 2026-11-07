# Game 2542 (nitro (1990)(psygnosis)[cr hz][t +5 full - evil]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02542). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| data | 598174 |
| unused | 284160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf:bootblock | 1 | 7 | 1 |
| logged disk load from Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf boot-block load | 1 | 127 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 547 | 13 |

## Cases handled

- nitro (1990)(psygnosis)[cr hz][t +5 full - evil]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf: custom-format (non-DOS) disk, 1180 non-empty blocks: matched against track-loaded segments
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf: the boot block reads 20480 bytes from disk offset 606208 to an address (found statically); recovered as a unit
- runtime segment an address: 429 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf: 1 logged loads cover 20480 bytes of the image
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.02)
- 1 packed track region(s), 594944 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf:bootblock

- `init_and_copy_strings`: Initializes a set of fixed values in a memory structure, then calls a startup routine.

### logged disk load from Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil].adf boot-block load

- `tiny_clr`: Small routine of 1 instruction that clears d0.
- `xor_byte_pattern`: Walks through a 32-bit value one byte at a time, toggling a global variable with the XOR of the current byte whenever that byte is zero, then decrements the value until it wraps past zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_coprocessor_dispatch`: Waits for a copper-list interrupt, then calls a series of library functions through pointers.
- `copy_memory_and_dispatch`: Copies a fixed block of memory into the system's global variable area, then reads a byte from a known location and repeatedly dispatches to another routine based on that byte's value while decrementing a counter and copying data through pointers.
- `clear_memory_block`: It clears a block of memory by writing zero bytes to consecutive words until a pointer reaches a limit, then returns.
- `load_table_values`: Casts an index value to a signed byte and stores a byte from a global table into the first byte of a destination buffer.
- `count_null_bytes`: Walks through a string until it reaches a null byte, counting the characters along the way.
- `update_string_index`: Examines a control byte and then scans a string to locate a non-zero byte, moving forward or backward depending on its value.
- `decrement_and_wrap_counter`: Decrements a byte in a structure and, if it wraps to zero, resets it and adjusts a counter up or down based on another byte.
- `process_timer_command`: Decrements a timer byte and, when it reaches zero, resets it to four and prepares a flag.
- `decode_encoded_stream`: Decrements a counter in a data structure and, when it reaches zero, reloads it and invokes four helper routines to decode a length-prefixed string and interpret a stream of encoded bytes while updating channel and instrument state.
- `decode_length_prefixed_string`: Decrements a counter stored in a data structure and then decodes a length-prefixed string from a table, advancing a pointer by the decoded length and updating several associated fields.
- `process_encoded_stream`: Decrements a counter and interprets a stream of encoded bytes, updating channel and instrument state through helper routines.
- `update_table_coordinate_from_entry`: Reads a table entry selected by an offset and updates a coordinate value plus a size value stored through a pointer, then adjusts a direction index from a nibble of the entry.
- `update_channel_period_and_vibrato`: Updates a channel's period and vibrato state from its instrument entry, applying a counter-driven sweep and a sign flip, then stores the new period and volume.
