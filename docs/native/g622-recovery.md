# Game 622 (Tower of Babel): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00622). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902496 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 31 raw-word lines |
| Functions named and described | pass, 6 of 6 named (0 author symbols), 6 of 6 described (0 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 88 of 88 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 299 of 299 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 57624 |
| code | 1074 |
| data | 831316 |
| padding | 706 |
| unused | 11776 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Tower of Babel.adf:bootblock | 1 | 57 | 1 |
| logged disk load from Tower of Babel.adf boot-block load | 1 | 2 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 227 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |

## Cases handled

- Tower of Babel/Tower of Babel.adf: custom-format (non-DOS) disk, 1735 non-empty blocks: matched against track-loaded segments
- Tower of Babel/Tower of Babel.adf: the boot block reads 65536 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- runtime segment an address: 80 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Tower of Babel/Tower of Babel.adf: 1 logged loads cover 65536 bytes of the image
- Tower of Babel/Tower of Babel.adf: blocks 169-1334 (bytes 86528-683520) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.30)
- 3 packed track region(s), 225792 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Tower of Babel.adf:bootblock

- `verify_checksum_and_unpack_stage`: Sets up a disk read request, then runs a rotate-and-xor checksum over a block of words loaded from disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_setup_then_idle_forever`: Waits for an input character, calls a sequence of setup routines, clears fields of its state structure, and finally enters an endless idle loop.
- `poll_mouse_and_clamp_cursor`: Reads the mouse port movement register and the button states.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `shift_pending_value_slot`: If the current slot byte is zero and the pending byte is nonzero, it moves the pending value into the current slot and resets the pending slot to four.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_halved_word_value`: Stores half of the low word of a data register into a fixed field of the state structure.
- `shift_buffer_byte`: checks if a certain byte in a buffer is zero and, if a preceding byte is non-zero, shifts a value forward and writes a constant to the next position.
