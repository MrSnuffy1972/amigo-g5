# Game 2140 (alien breed - tower assault (1994)(team 17)[cr dlt][t +3 dtl](disk 1 of 4)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02140). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 11 of 11 named (0 author symbols), 11 of 11 described (0 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 175 of 175 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 972 of 972 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 8539 |
| code | 2686 |
| data | 883239 |
| unused | 6656 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Alien Breed - Tower Assault (1994)(Team 17)[cr DLT][t +3 DTL](Disk 1 of 4)[a].adf:bootblock | 1 | 33 | 1 |
| logged disk load from Alien Breed - Tower Assault (1994)(Team 17)[cr DLT][t +3 DTL](Disk 1 of 4)[a].adf boot-block load | 1 | 109 | 2 |
| logged disk load from Alien Breed - Tower Assault (1994)(Team 17)[cr DLT][t +3 DTL](Disk 1 of 4)[a].adf boot-block load | 1 | 830 | 8 |

## Cases handled

- alien breed - tower assault (1994)(team 17)[cr dlt][t +3 dtl](disk 1 of 4)[a]/Alien Breed - Tower Assault (1994)(Team 17)[cr DLT][t +3 DTL](Disk 1 of 4)[a].adf: custom-format (non-DOS) disk, 1742 non-empty blocks: matched against track-loaded segments
- alien breed - tower assault (1994)(team 17)[cr dlt][t +3 dtl](disk 1 of 4)[a]/Alien Breed - Tower Assault (1994)(Team 17)[cr DLT][t +3 DTL](Disk 1 of 4)[a].adf: the boot block reads 2048 bytes from disk offset 13824 to an address (found statically); recovered as a unit
- alien breed - tower assault (1994)(team 17)[cr dlt][t +3 dtl](disk 1 of 4)[a]/Alien Breed - Tower Assault (1994)(Team 17)[cr DLT][t +3 DTL](Disk 1 of 4)[a].adf: the boot block reads 10240 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- alien breed - tower assault (1994)(team 17)[cr dlt][t +3 dtl](disk 1 of 4)[a]/Alien Breed - Tower Assault (1994)(Team 17)[cr DLT][t +3 DTL](Disk 1 of 4)[a].adf: 2 logged loads cover 12288 bytes of the image
- alien breed - tower assault (1994)(team 17)[cr dlt][t +3 dtl](disk 1 of 4)[a]/Alien Breed - Tower Assault (1994)(Team 17)[cr DLT][t +3 DTL](Disk 1 of 4)[a].adf: blocks 22-26 (bytes 11264-13824) outside the filesystem hold data, not code (opcode density 0.038, entropy 6.85)
- 1 packed track region(s), 878592 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Alien Breed - Tower Assault (1994)(Team 17)[cr DLT][t +3 DTL](Disk 1 of 4)[a].adf:bootblock

- `read_boot_payload_to_buffer`: Builds an I/O request with a command, a destination buffer, a length and a disk offset, then repeats the request until its status flag clears before jumping to the loaded code.

### logged disk load from Alien Breed - Tower Assault (1994)(Team 17)[cr DLT][t +3 DTL](Disk 1 of 4)[a].adf boot-block load

- `calculate_scaled_bit_shift`: Performs arithmetic operations on registers including subtraction, addition, and bitwise negation, followed by a logical shift left and a swap of the high and low words before returning.
- `clear_cpu_caches_via_exec`: Calls the Exec cache-clear function through a library base, and its address is taken by a PC-relative load.

### logged disk load from Alien Breed - Tower Assault (1994)(Team 17)[cr DLT][t +3 DTL](Disk 1 of 4)[a].adf boot-block load

- `advance_cursor_by_step_loop`: Repeatedly copies one cursor field into a second field, calls an unnamed routine, subtracts the copied value from a count field and adds the difference to the cursor, looping while the count is positive.
- `build_canonical_code_tables`: Counts how many input symbols use each code length, accumulates the offsets for each length and fills lookup tables with codes sized by length.
- `decode_bits_from_stream`: Refills a 32-bit bit buffer from a 16-bit input stream when it runs short, shifts out the requested number of bits and looks up a value in a table.
- `clear_and_fill_decode_buffers`: Refills a bit buffer like its neighbour, clears a 512-word buffer and then a 4096-word buffer, and reads a value from an indexed table.
- `unscramble_block_rotate_xor`: Processes 32 input bytes, rotating each one left by two bits and XORing it with the previous output byte, and writes each result to the output buffer.
- `decode_bitstream_into_output`: Reads bits from an input stream through a bit buffer and writes the decoded values to an output buffer, using the length counters kept in the state block.
- `check_result_or_return_error`: Calls an unnamed routine and returns its register pair when the result is non-negative.
- `call_handler_return_stored_value`: Calls a handler through a pointer stored in the caller's state block and then returns a value kept in that same block.
