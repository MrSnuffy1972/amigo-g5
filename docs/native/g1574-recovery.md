# Game 1574 (no second prize (1990)(thalion)[cr ministry][t +2 supplex]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01574). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 25 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 10 of 10 named (0 author symbols), 10 of 10 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1 of 1 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 275 of 275 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 42424 |
| code | 902 |
| data | 850626 |
| unused | 7168 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf:bootblock | 1 | 36 | 2 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load | 1 | 94 | 3 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load | 1 | 57 | 2 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load | 1 | 88 | 3 |
| logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load | 1 | 0 | 0 |

## Cases handled

- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: custom-format (non-DOS) disk, 1741 non-empty blocks: matched against track-loaded segments
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: the boot block reads 16896 bytes from disk offset 805376 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: the boot block reads 2560 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: the boot block reads 19456 bytes from disk offset 867328 to an address (found statically); recovered as a unit
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: 4 logged loads cover 44544 bytes of the image
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: blocks 22-1564 (bytes 11264-801280) hold 24 packed streams (IMP: File Imploder, TPWM: Turbo Packer) found by scanning, 98% of the bytes
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: blocks 1606-1693 (bytes 822272-867328) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.70)
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: blocks 1732-1756 (bytes 886784-899584) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.07)
- no second prize (1990)(thalion)[cr ministry][t +2 supplex]/No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf: blocks 1758-1758 (bytes 900096-900608) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `disable_chips_then_idle_loop` (No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load +an address to an address (2560 bytes)): Entry of thirty-five instructions that loops forever if its argument is zero; otherwise it writes masks that clear the top bit, which disable DMA and interrupts, runs a delay countdown, and loops forever writing a fixed value to a global variable. Kind: checksum/data integrity trap. Pass path: The routine performs a bit-shifting loop that reconstructs a value (uVar3) from a data stream, likely a checksum or a signature embedded in the loaded data. If the reconstructed value matches the expected state (the `uVar2 == 0` branch lead.

## Functions


### No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf:bootblock

- `run_library_request_then_call`: Boot block entry of twenty-one instructions that makes one library request through a stored pointer and calls one unnamed routine.
- `issue_io_then_reset_global`: Fills in two Exec request blocks, issues each through a stored pointer, and calls a fixed-address routine between them, then clears a global variable before returning.

### logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load

- `write_decrementing_global_until_wrap`: Writes a decrementing word to a global variable in a loop that runs until the counter wraps, then returns the value minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load

- `disable_chips_then_idle_loop`: Entry of thirty-five instructions that loops forever if its argument is zero; otherwise it writes masks that clear the top bit, which disable DMA and interrupts, runs a delay countdown, and loops forever writing a fixed value to a global variable.
- `read_capped_entry_field`: Reads a field from a linked structure referenced by a global pointer.

### logged disk load from No Second Prize (1990)(Thalion)[cr Ministry][t +2 Supplex].adf boot-block load

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `shift_packed_longword_right`: Returns the longword four bytes before the address register shifted right one bit, with the top bit set when a flag bit in a data register is set.
- `shift_bits_from_packed_longwords`: Shifts bits out of a register for a count taken from its argument, reloading from the previous packed longword whenever the register empties and feeding a flag into the top bit.
