# Game 901 (rampage (1989)(activision)[cr band][t +1 band]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00901). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903888 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 6 of 6 named (0 author symbols), 6 of 6 described (0 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 42 of 42 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 5632 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 189 of 189 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1808 |
| code | 414 |
| data | 9574 |
| padding | 188 |
| unused | 891904 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rampage (1989)(Activision)[cr Band][t +1 Band].adf:bootblock | 1 | 21 | 2 |
| logged disk load from Rampage (1989)(Activision)[cr Band][t +1 Band].adf boot-block load | 1 | 97 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 71 | 2 |
| Rampage (1989)(Activision)[cr Band][t +1 Band].adf bytes 5632-11264 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- rampage (1989)(activision)[cr band][t +1 band]/Rampage (1989)(Activision)[cr Band][t +1 Band].adf: file            (header 5124 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- rampage (1989)(activision)[cr band][t +1 band]/Rampage (1989)(Activision)[cr Band][t +1 Band].adf: the boot block reads 6656 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- runtime segment an address: 42 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rampage (1989)(activision)[cr band][t +1 band]/Rampage (1989)(Activision)[cr Band][t +1 Band].adf: bytes 5632-11264 look like code no run loaded: recovered position-independently (load address unknown)
- rampage (1989)(activision)[cr band][t +1 band]/Rampage (1989)(Activision)[cr Band][t +1 Band].adf: 2 logged loads cover 5632 bytes of the image
- 1 never-loaded region(s), 5632 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rampage (1989)(Activision)[cr Band][t +1 Band].adf:bootblock

- `copy_words_and_jump`: Calls another routine, sets a2 to a stack address, copies 128 long words from a0 to a2 using a dbra loop, and jumps to a Kickstart vector.
- `init_struct_and_clear_colors`: Initializes a structure pointed to by a1 with a couple of words and long values, calls another routine through a vector in a global variable, then calls a Kickstart vector and clears four color registers in the custom chip.

### logged disk load from Rampage (1989)(Activision)[cr Band][t +1 Band].adf boot-block load

- `shift_and_copy_data`: Pops a long word from a0, stores its high byte into a word, sets a condition code bit, rotates the value right, and returns, with a following section that shifts and copies data in a loop.
- `loop_copy_and_dispatch`: Loops decrementing a counter while shifting a value, calling a routine when a bit becomes clear, and otherwise dispatches based on a counter value to copy bytes from a source into a destination until a range is reached.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_and_clear_flags`: Waits in a loop until a word at offset -31368 of a1 is nonzero and then has bit 7 clear, clears that word, zeroes eleven consecutive words starting at offset -31426, and sets bit 7 of six more words before returning.
- `read_inputs_and_set_audio`: When bit 5 of the input port register is set, increments a counter, stores several processed values of the input and joystick data into a structure, and sets audio volume registers based on button state, then returns via a jump.
