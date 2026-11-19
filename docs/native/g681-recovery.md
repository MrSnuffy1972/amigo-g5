# Game 681 (super cars ii (1991)(gremlin)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00681). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1805520 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 81 raw-word lines |
| Functions named and described | pass, 22 of 22 named (0 author symbols), 22 of 22 described (5 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 372 of 372 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 761 of 761 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 25553 |
| code | 2964 |
| data | 1753449 |
| padding | 2 |
| unused | 23552 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Super Cars II (1991)(Gremlin)[cr SKR](Disk 1 of 2).adf:bootblock | 1 | 117 | 5 |
| logged disk load from Super Cars II (1991)(Gremlin)[cr SKR](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 83 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 161 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 394 | 9 |

## Cases handled

- super cars ii (1991)(gremlin)[cr skr](disk 1 of 2)/Super Cars II (1991)(Gremlin)[cr SKR](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1732 non-empty blocks: matched against track-loaded segments
- super cars ii (1991)(gremlin)[cr skr](disk 2 of 2)/Super Cars II (1991)(Gremlin)[cr SKR](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1738 non-empty blocks: matched against track-loaded segments
- super cars ii (1991)(gremlin)[cr skr](disk 1 of 2)/Super Cars II (1991)(Gremlin)[cr SKR](Disk 1 of 2).adf: the boot block reads 25600 bytes from disk offset 872448 to an address (found statically); recovered as a unit
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 81 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 42 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 243 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- super cars ii (1991)(gremlin)[cr skr](disk 1 of 2)/Super Cars II (1991)(Gremlin)[cr SKR](Disk 1 of 2).adf: 1 logged loads cover 25600 bytes of the image
- 2 packed track region(s), 1751040 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Super Cars II (1991)(Gremlin)[cr SKR](Disk 1 of 2).adf:bootblock

- `setup_coprocessor`: Initializes coprocessor registers with specific values and sets up interrupt enable bits, then enters an infinite loop.
- `unpack_pp20_stream`: Checks for the PP20 signature tag and, when it is present, unpacks the packed data backwards with a bit-stream decoder, copying literal and back-reference runs into the output.
- `frag_lsr_beq_rts_move_roxr`: Short code fragment that shifts d5 right by 1, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (subq.w) and then falls through into the code that follows, with no return of its own.
- `decode_bitstream`: Decodes a bit stream by shifting bits right through a loop and handling carry flags, then returns after processing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_cia_control_bit`: Clears bit six of the first CIA control register, writes an all-ones value to a shared register, and saves the previous byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decode_joystick_port_state`: Reads the mouse and joystick port movement registers, the right and middle button register and the fire-button register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_rol_rts`: Short code fragment that rotates d2 left by d2, then returns.
- `frag_move_rts`: Short code fragment that copies 15 to the memory at an offset from a6, then returns.
- `service_movement_interrupt_request`: Interrupt handler that reads the interrupt request register and, when bit four is set, clears that request, reads counter values, updates a state word and calls the matching handler from a function table.
- `copy_memory_block_3`: Copies 765 32-bit words from a source table to a destination table using a loop.
- `xor_global_byte`: Performs a bitwise XOR between a byte in d0 and a byte at a specific memory address and returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_if_flagged`: Checks if a global flag is non-zero, then calls another routine.
- `frag_move_clr_rts`: Short code fragment that copies 15 to the memory at an offset from a6, then clears a fixed memory address, then returns.
- `set_global_flag`: Sets a global variable to a constant value.
- `initialize_audio`: Initializes audio registers by clearing volume levels and setting specific hardware control values, then returns.
- `update_music_sequence`: Increments a counter and handles music sequence progression, including resetting counters and updating sequence data based on comparison results.
- `update_audio_channels`: Processes audio channel updates by calling a helper routine four times, once for each of the four audio channels.
- `start_audio_dma`: Clears a global variable, performs delay loops, enables audio DMA channels, and loads four audio location pointers from a table.
- `parse_music_entry`: Extracts instrument number and period from a four-byte music data entry, looks up sample pointer and applies effects, then updates channel pointers and length.
- `update_position`: Processes movement commands and updates a position value in a structure, with bounds checking and clamping.
