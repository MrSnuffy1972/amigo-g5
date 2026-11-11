# Game 2540 (nitro (1990)(psygnosis)[cr hz][a5]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02540). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905556 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 20 raw-word lines |
| Functions named and described | pass, 33 of 33 named (0 author symbols), 33 of 33 described (0 by rule, 30 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1184 of 1195 (99.08%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1275 of 1284 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 20155 |
| code | 4188 |
| data | 597051 |
| padding | 2 |
| unused | 284160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nitro (1990)(Psygnosis)[cr HZ][a5].adf:bootblock | 1 | 7 | 1 |
| logged disk load from Nitro (1990)(Psygnosis)[cr HZ][a5].adf boot-block load | 1 | 86 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 60 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 384 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 298 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 427 | 4 |

## Cases handled

- nitro (1990)(psygnosis)[cr hz][a5]/Nitro (1990)(Psygnosis)[cr HZ][a5].adf: custom-format (non-DOS) disk, 1203 non-empty blocks: matched against track-loaded segments
- nitro (1990)(psygnosis)[cr hz][a5]/Nitro (1990)(Psygnosis)[cr HZ][a5].adf: the boot block reads 20480 bytes from disk offset 606208 to an address (found statically); recovered as a unit
- runtime segment an address: 41 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 1066 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 3 first-run image unit(s) in 1 version group(s); 628 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- nitro (1990)(psygnosis)[cr hz][a5]/Nitro (1990)(Psygnosis)[cr HZ][a5].adf: 1 logged loads cover 20480 bytes of the image
- nitro (1990)(psygnosis)[cr hz][a5]/Nitro (1990)(Psygnosis)[cr HZ][a5].adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.02)
- 1 packed track region(s), 594944 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Nitro (1990)(Psygnosis)[cr HZ][a5].adf:bootblock

- `execute_library_call`: Performs a single library call or internal function call using a hardcoded library or address.

### logged disk load from Nitro (1990)(Psygnosis)[cr HZ][a5].adf boot-block load

- `decompress_bitstream_to_memory`: Reads a bitstream using a bit buffer to decompress data into a memory range starting at a specified address.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_decrement`: Decrements a global variable and calls another routine before returning a concatenated value from registers.
- `modify_global_bitmask`: Performs a series of bitwise checks and logical or operations to modify a global variable.
- `write_to_hardware_registers`: Writes specific values to a memory-mapped or hardware-related address and updates a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_structure_from_bitfield`: Iterates through a bitfield in a data buffer using nested loops, and when a bit is set, it writes several multi-byte values and constants into a structure pointed to by a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `trigger_trap`: Pushes a longword onto the stack and calls a trap.
- `initialize_bitstream_buffer`: Tests bit 14 of a byte at offset 2 of a structure in a6; if set, it skips three longword and two pointer writes, otherwise it performs them, then calls several other routines including one that increments a byte at a global address in a loop.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `initialize_structure_fields`: Tests bit 14 of a byte at offset 2 of a structure in a6; if set, it skips the following writes, otherwise it stores three longwords, two pointers, a longword zero, and a word 1046 into the structure at offsets 64, 64, 80, 84, 100, and 88 respectively.
- `increment_and_test_pointer`: Increments a longword at a location relative to the program counter, loads the value from that location into a1, and tests the byte at the address in a1.
- `update_structure_data`: If a zero flag is set, it writes a value to a location pointed to by a0; otherwise, it performs a calculation using a value from a pointer, enters a loop that checks bit 6 of a byte at offset 2 of a structure in a6, and writes several longwords and words to offsets of a structure in a6.
- `update_sprite_attribute_table`: Iterates through a table of data to update multiple sprite attributes, including position and color, by writing values to a structure pointed to by a register.
- `increment_table_values`: Increments a byte at a global address by 1, a byte at offset 8 by 2, and a byte at offset 16 by 4, repeating this process 25 times in a loop.
- `copy_and_reorder_data`: Copies data from one memory location to another using a loop, performs a second loop to reverse the order of words, and then copies data from a third location into a final destination using two separate loops.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_structure_from_bitfield`: Iterates through a bitfield to check specific bits, and if a bit is set, it populates a structure at an offset from a6 with several longwords, words, and a pointer.
- `multiply_table_entries`: Multiplies each word in a table by 42 and stores the result back in the same location, repeating for 386 entries.
- `generate_alternating_sequence`: Writes a sequence of bytes and words to a global table, where the bytes are incremented by 2 and the words are generated by swapping and XORing a counter.
- `populate_data_tables`: Fills several memory regions with constant longwords, copies words from a source table to a destination, and performs a loop to write specific values into a structure.
- `setup_global_state`: Writes values to hardware registers, sets a global variable to 32767, and performs a series of assignments to a global variable, then calls another routine seven times and sets a value at an offset of a6.
- `clear_memory_range`: Clears a range of memory by writing zeros to it, starting from a specific address and continuing until the pointer reaches a target address.
- `set_hardware_registers`: Writes 7fff to two specific hardware register offsets and sets two global bytes to 1 and 0 respectively.
- `update_global_pointers`: Copies words from a source table to a destination, updates several global word values using swaps and additions, and clears a word at an offset in a6.
- `populate_memory_tables`: Clears a block of memory at a specific address in a loop, writes a series of values to a structure in a6, calls another routine, and then fills a memory range with a repeating pattern of bytes and words using two nested loops.
- `load_data_via_descriptors`: Reads descriptors from a table to determine offsets and counts, then either populates a structure with specific values or copies bytes from a source table into a destination structure.
- `update_swapped_values`: If a value at a specific location is 150, it swaps two 32-bit values in a global table, calculates two new 16-bit values based on the first value, stores them at offsets 4 and 12 of a structure in a0, and performs a swap on those values before storing them at offsets 0 and 8 of the same structure; otherwise it returns.
- `handle_interrupt_return`: Saves registers, reads a byte from a hardware register, calls another routine, restores registers, writes 8192 to a hardware register, and returns from interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_offset`: Performs a short branch to an instruction at a fixed offset within another routine.
- `process_bytecode_stream`: Sets several global pointers, configures hardware registers, and enters a loop that calls several routines to process data, then performs several conditional checks to update hardware registers and global values based on decremented counters.
- `process_data_structure`: Reads a byte from a table via a pointer derived from a structure in a0, and if certain bit conditions are met, updates several fields in the structure, modifies a value in a global variable via a pointer in a5, writes to a hardware register, and copies multiple bytes from a calculated table offset into the structure.
- `parse_bytecode_stream`: Parses a stream of encoded instructions from a buffer, performing various data movements and pointer updates based on specific opcode values.
