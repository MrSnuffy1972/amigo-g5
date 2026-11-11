# Game 698 (zool (1992)(gremlin)[cr csl][t +1 dc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00698). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1804912 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 19 of 19 named (0 author symbols), 19 of 19 described (3 by rule, 13 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 277 of 277 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 461 of 461 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12656 |
| code | 1484 |
| data | 1714996 |
| unused | 75776 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf:bootblock | 1 | 13 | 1 |
| logged disk load from Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf boot-block load | 1 | 163 | 2 |
| logged disk load from Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| logged disk load from Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 285 | 16 |

## Cases handled

- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1651 non-empty blocks: matched against track-loaded segments
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 2 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1717 non-empty blocks: matched against track-loaded segments
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: the boot block reads 5632 bytes from disk offset 512 to an address (found statically); recovered as a unit
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: the boot block reads 6144 bytes from disk offset 12288 to an address (found statically); recovered as a unit
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: the boot block reads 1536 bytes from disk offset 897024 to an address (found statically); recovered as a unit
- runtime segment an address: 277 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: 3 logged loads cover 13312 bytes of the image
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: blocks 12-23 (bytes 6144-12288) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.04)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: blocks 36-36 (bytes 18432-18944) outside the filesystem hold data, not code (opcode density 0.020, entropy 4.55)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: blocks 568-570 (bytes 290816-292352) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.31)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: blocks 572-580 (bytes 292864-297472) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.01)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: blocks 582-586 (bytes 297984-300544) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.39)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: blocks 588-820 (bytes 301056-420352) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.01)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: blocks 1612-1616 (bytes 825344-827904) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.01)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: blocks 1622-1630 (bytes 830464-835072) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.24)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: blocks 1632-1635 (bytes 835584-837632) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.13)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: blocks 1642-1651 (bytes 840704-845824) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.18)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: blocks 1659-1663 (bytes 849408-851968) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.06)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: blocks 1679-1683 (bytes 859648-862208) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.05)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: blocks 1699-1702 (bytes 869888-871936) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.87)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: blocks 1738-1751 (bytes 889856-897024) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.99)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 1 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf: blocks 1755-1759 (bytes 898560-901120) hold ASCII text
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 2 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 2 of 2).adf: blocks 2-41 (bytes 1024-21504) hold ASCII text
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 2 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 2 of 2).adf: blocks 1098-1100 (bytes 562176-563712) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.17)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 2 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 2 of 2).adf: blocks 1102-1112 (bytes 564224-569856) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.83)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 2 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 2 of 2).adf: blocks 1118-1118 (bytes 572416-572928) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.02)
- zool (1992)(gremlin)[cr csl][t +1 dc](disk 2 of 2)/Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 2 of 2).adf: blocks 1146-1759 (bytes 586752-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.39)
- 3 packed track region(s), 1203200 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf:bootblock

- `load_stages_and_unpack_bootblock`: Reads three disk segments into memory with device I/O and copies a block of boot code to its destination.

### logged disk load from Zool (1992)(Gremlin)[cr CSL][t +1 DC](Disk 1 of 2).adf boot-block load

- `emit_word_and_shift_bitstream`: Reads a longword from the stream behind the input pointer, stores a 16-bit word combining its low byte with the high byte of a register, and returns the longword shifted right with its top bit set.
- `shift_bit_buffer_by_count`: Shifts a bit buffer right one bit per loop iteration for the count held in a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics_and_palette`: Initializes several data structures through a loop, updates the hardware custom chip registers with specific values, sets a series of palette entries, and finally performs a memory copy before jumping to a fixed address.
- `level5_interrupt_handler`: Saves all registers and status, checks INTREQ bit 5, sets the level 5 flag byte and calls the game interrupt routine, then acknowledges INTREQ with 112 and returns from exception.
- `clear_and_branch_on_flag`: Clears a byte in a global variable, tests it, and branches if it is zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_packed_tilemap_to_bitplanes`: For each of 16 bitplanes and 30 words, reads a packed word from a1 and increments bytes in the four-byte group at a3 to set bits, advancing a3 by four.
- `decrement_palette_components_until_zero`: For each of sixteen levels and thirty palette entries, reads a packed colour word, decrements each non-zero red, green and blue nibble, repacks the value and stores it back before advancing the pointer.
- `call_unpack_bitstream`: Saves registers to the stack, calls another routine, restores the registers, and returns.
- `unpack_fun_gs_bitstream_to_buffer`: Scans a0 for the FUN*GUS* signature, then decodes the following bit-packed data into the buffer at a1 using a shift register in d0-d7 and returns.
- `lz_bitstream_shift_refill_words`: Shifts the d0 bitstream left, refilling from d4-d7 and pulling a longword from -(a0) when it empties, and repeats d1 times before returning.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `check_bit_and_clear_a4`: Branches to another routine, checks bit 4 of the byte at offset 4097 in a5, and if set, loops 5999 times, calls another routine, and branches back to the bit check; otherwise it clears a word at the address in a4, clears bit 2 of the byte at offset 256 in a5, and returns.
- `pulse_status_bits_and_wait_blitter`: Calls a routine, sets then clears bits 3 and 7 of the byte at offset 256 in a5, loops 39999 times, then polls bit 5 of the byte at offset 4097 in a5 until it is clear before returning.
- `pulse_audio_filter_bit_and_delay`: Calls a routine, sets then clears bits 3 and 7 of the byte at offset 256 in a5, then loops 39999 times before returning.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `toggle_led_wait_2000_then_poll_bit5`: Sets bit 0 of the byte at offset 256 in a5, clears it, sets it again, delays 2000 iterations, then waits until bit 5 of the word at offset 4097 in a5 is clear and returns.
- `load_custom_ciab_and_base_pointers`: Loads the custom chip base into a6, the CIAB base into a5 and the address a fixed address into a4, then returns to the caller.
