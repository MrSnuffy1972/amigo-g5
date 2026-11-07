# Game 607 (f-19 stealth fighter (1988)(microprose)[cr valhalla]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00607). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902000 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 27 of 27 named (0 author symbols), 27 of 27 described (0 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 131 of 131 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 842 of 842 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 22805 |
| code | 2010 |
| data | 828031 |
| padding | 2 |
| unused | 49152 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf:bootblock | 1 | 35 | 1 |
| logged disk load from F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf boot-block load | 1 | 271 | 11 |
| logged disk load from F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf boot-block load | 1 | 271 | 11 |
| logged disk load from F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf boot-block load | 1 | 76 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 105 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 58 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 1 |

## Cases handled

- f-19 stealth fighter (1988)(microprose)[cr valhalla]/F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf: custom-format (non-DOS) disk, 1653 non-empty blocks: matched against track-loaded segments
- f-19 stealth fighter (1988)(microprose)[cr valhalla]/F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf: the boot block reads 1536 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- f-19 stealth fighter (1988)(microprose)[cr valhalla]/F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf: the boot block reads 1536 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- f-19 stealth fighter (1988)(microprose)[cr valhalla]/F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf: the boot block reads 24576 bytes from disk offset 225280 to an address (found statically); recovered as a unit
- runtime segment an address: 71 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 43 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- f-19 stealth fighter (1988)(microprose)[cr valhalla]/F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf: 3 logged loads cover 26112 bytes of the image
- f-19 stealth fighter (1988)(microprose)[cr valhalla]/F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf: blocks 5-179 (bytes 2560-92160) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.90)
- f-19 stealth fighter (1988)(microprose)[cr valhalla]/F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf: blocks 198-236 (bytes 101376-121344) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.30)
- f-19 stealth fighter (1988)(microprose)[cr valhalla]/F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf: blocks 242-286 (bytes 123904-146944) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.26)
- f-19 stealth fighter (1988)(microprose)[cr valhalla]/F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf: blocks 297-338 (bytes 152064-173568) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.17)
- f-19 stealth fighter (1988)(microprose)[cr valhalla]/F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf: blocks 341-381 (bytes 174592-195584) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.17)
- f-19 stealth fighter (1988)(microprose)[cr valhalla]/F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf: blocks 506-819 (bytes 259072-419840) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.83)
- f-19 stealth fighter (1988)(microprose)[cr valhalla]/F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf: blocks 827-829 (bytes 423424-424960) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.09)
- f-19 stealth fighter (1988)(microprose)[cr valhalla]/F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf: blocks 836-1062 (bytes 428032-544256) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.95)
- f-19 stealth fighter (1988)(microprose)[cr valhalla]/F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf: blocks 1067-1725 (bytes 546304-883712) outside the filesystem hold data, not code (opcode density 0.010, entropy 7.06)
- f-19 stealth fighter (1988)(microprose)[cr valhalla]/F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf: blocks 1738-1759 (bytes 889856-901120) hold ASCII text
- 1 packed track region(s), 22528 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `write_port_b_and_wait` (F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf boot-block load +an address to an address (1536 bytes)): Writes a fixed control value to the CIA port B register, waits for a short delay, then polls a flag bit until it clears. Kind: disk hardware/timing check. Pass path: the branch at an address (beq.b an address).
- `write_port_b_and_wait` (F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf boot-block load +an address to an address (1536 bytes)): Writes a fixed control value to the CIA port B register, waits for a short delay, then polls a flag bit until it clears. Kind: disk hardware/timing check. Pass path: the branch at an address (beq.b an address).

## Functions


### F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf:bootblock

- `load_disk_segments_via_io`: Issues a series of device I/O requests through the exec library's DoIO routine to load disk segments into memory, with a control request between reads.

### logged disk load from F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf boot-block load

- `run_counted_helper_loop`: Makes one call, then loops a count held in a register, calling two unnamed routines on each pass, and finally makes one more call.
- `toggle_port_bits_and_wait`: Toggles a set of control bits in a port byte depending on the parity of a counter, waits a fixed delay, then polls a status bit until it clears.
- `wait_bit_then_reset_counter`: Calls an unnamed routine while bit four of a status byte is set, then clears a counter word.
- `clear_step_counter`: Sets a step counter to zero.
- `increment_step_and_pulse_port`: Increments a step counter and sets a control bit in a port byte according to the parity of the new count.
- `decrement_step_and_pulse_port`: Decrements a step counter and sets a control bit in a port byte according to its parity.
- `write_port_b_and_wait`: Writes a fixed control value to the CIA port B register, waits for a short delay, then polls a flag bit until it clears.
- `write_control_byte_sequence`: Writes two control values in sequence to one byte at its pointer, so the second value overwrites the first.
- `read_disk_track_for_sync`: Programs the disk controller registers, then scans memory for the disk sync word while polling the DMA and status bits.
- `stub_empty_return`: The decompiled body is empty, so the routine returns to its caller at once.
- `unpack_backward_lz_stream`: Decodes a packed stream working backwards from its end.

### logged disk load from F-19 Stealth Fighter (1988)(MicroProse)[cr Valhalla].adf boot-block load

- `run_counted_helper_loop`: Makes one call, then loops a count held in a register, calling two unnamed routines on each pass, and finally makes one more call.
- `toggle_port_bits_and_wait`: Toggles a set of control bits in a port byte depending on the parity of a counter, waits a fixed delay, then polls a status bit until it clears.
- `wait_bit_then_reset_counter`: Calls an unnamed routine while bit four of a status byte is set, then clears a counter word.
- `clear_step_counter`: Sets a step counter to zero.
- `increment_step_and_pulse_port`: Increments a step counter and sets a control bit in a port byte according to the parity of the new count.
- `decrement_step_and_pulse_port`: Decrements a step counter and sets a control bit in a port byte according to its parity.
- `write_port_b_and_wait`: Writes a fixed control value to the CIA port B register, waits for a short delay, then polls a flag bit until it clears.
- `write_control_byte_sequence`: Writes two control values in sequence to one byte at its pointer, so the second value overwrites the first.
- `read_disk_track_for_sync`: Programs the disk controller registers, then scans memory for the disk sync word while polling the DMA and status bits.
- `stub_empty_return`: The decompiled body is empty, so the routine returns to its caller at once.
- `unpack_backward_lz_stream`: Decodes a packed stream working backwards from its end.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `step_sound_tick_state`: Runs when a flag bit is set: it steps a counter through a period table, updates several fields with masks, and writes audio channel values when the counter reaches its threshold.
- `pack_offset_values_into_word`: Combines a shifted sum of a global value and a register into the high byte, and a halved, offset register value into the low byte, and returns the resulting 16-bit word.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_joystick_into_position`: Reads the joystick port data and button registers, converting the movement into a scaled position change that is clamped at zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `select_pointer_from_state`: Repeatedly tests a state byte against zero and stores one of three fixed addresses into a pointer slot depending on the test, looping while the state is nonzero.
