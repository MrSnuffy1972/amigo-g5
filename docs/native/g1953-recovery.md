# Game 1953 (Kick Off 2 - The Final Whistle (Europe) (v1.9g)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01953). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4862809 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 17 of 17 named (0 author symbols), 17 of 17 described (2 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 30 of 30 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 216 of 216 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2250206 |
| code | 832 |
| data | 2610187 |
| padding | 1584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Kick Off 2 - The Final Whistle (Europe) (v1.9g).adf:bootblock | 1 | 46 | 3 |
| Kick Off 2 - The Final Whistle (Europe) (v1.9g).ipf#adf:bootblock | 1 | 46 | 3 |
| logged disk load from Kick Off 2 - The Final Whistle (Europe) (v1.9g).adf boot-block load | 1 | 13 | 0 |
| logged disk load from Kick Off 2 - The Final Whistle (Europe) (v1.9g).ipf#adf boot-block load | 1 | 13 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 124 | 11 |

## Cases handled

- Kick Off 2 - The Final Whistle (Europe) (v1.9g)/Kick Off 2 - The Final Whistle (Europe) (v1.9g).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- Kick Off 2 - The Final Whistle (Europe) (v1.9g)/Kick Off 2 - The Final Whistle (Europe) (v1.9g).ipf decoded by disk-analyse to a 901120-byte disk image (tracks: T0.0-79.1: AmigaDOS)
- Kick Off 2 - The Final Whistle (Europe) (v1.9g)/Kick Off 2 - The Final Whistle (Europe) (v1.9g).ipf#adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- Kick Off 2 - The Final Whistle (Europe) (v1.9g)/Kick Off 2 - The Final Whistle (Europe) (v1.9g).adf: the boot block reads 120320 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- Kick Off 2 - The Final Whistle (Europe) (v1.9g)/Kick Off 2 - The Final Whistle (Europe) (v1.9g).ipf#adf: the boot block reads 120320 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Kick Off 2 - The Final Whistle (Europe) (v1.9g)/Kick Off 2 - The Final Whistle (Europe) (v1.9g).adf: 1 logged loads cover 120320 bytes of the image
- Kick Off 2 - The Final Whistle (Europe) (v1.9g)/Kick Off 2 - The Final Whistle (Europe) (v1.9g).adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.006, entropy 4.52)
- Kick Off 2 - The Final Whistle (Europe) (v1.9g)/Kick Off 2 - The Final Whistle (Europe) (v1.9g).adf: blocks 246-1759 (bytes 125952-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.16)
- Kick Off 2 - The Final Whistle (Europe) (v1.9g)/Kick Off 2 - The Final Whistle (Europe) (v1.9g).ipf#adf: 1 logged loads cover 120320 bytes of the image
- Kick Off 2 - The Final Whistle (Europe) (v1.9g)/Kick Off 2 - The Final Whistle (Europe) (v1.9g).ipf#adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.006, entropy 4.52)
- Kick Off 2 - The Final Whistle (Europe) (v1.9g)/Kick Off 2 - The Final Whistle (Europe) (v1.9g).ipf#adf: blocks 246-1759 (bytes 125952-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.16)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Kick Off 2 - The Final Whistle (Europe) (v1.9g).adf:bootblock

- `boot_alloc_absolute_and_io`: Boot block that calls the exec library to allocate memory at a fixed address and to run an I/O request through DoIO, and writes the colour, DMA and interrupt control registers.
- `copy_block_to_low_memory`: Copies longwords in a counted loop from a start label in the program to a fixed low memory address, then calls one other routine.
- `store_boot_state_values`: Writes a fixed constant into a global variable, decrements a value stored in low memory using a register, and copies a longword from a fixed low memory address into a nearby byte location.

### Kick Off 2 - The Final Whistle (Europe) (v1.9g).ipf#adf:bootblock

- `init_memory_and_io`: Runs a 30-instruction setup sequence that allocates a fixed memory block and performs an input/output request through the exec library, while toggling interrupt and DMA enable bits in the hardware control registers.
- `call_single_helper`: An eleven-instruction routine that invokes one unnamed routine and is reached by address taken, delegating its work to that other routine.
- `toggle_dma_control`: A five-instruction routine that touches the DMA control register and is reached by address taken, performing a short hardware register manipulation before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `pulse_bit_around_calls`: Loops indefinitely, setting bit 2 of a byte at one pointer, calling one routine, clearing the same bit at a second pointer, and calling that routine again.
- `wait_for_flag_clear`: Polls a byte pointer for a specific bit to clear, using a decrementing counter loop.
- `search_byte_pattern_in_stream`: Waits for the top bit of a word at a pointer to be set, then compares the following bytes against the character D and a second fixed byte value, with a counter that restarts the search.
- `wait_for_word_top_bit`: Two-instruction routine that polls a word through a pointer until its top bit is set, repeating the poll several times in a row.
- `compare_multiple_pointer_values`: Compares a byte in d1 against four consecutive signed words pointed to by a3, returning if any value is negative or if any value fails to match its corresponding byte in d1, d2, d3, or d4.
- `repeat_control_write_and_poll`: Repeats 120 times: writes three fixed byte values to three offsets from a base pointer, then polls the first offset until its lowest bit is clear.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (bset.b, subq.w) and then falls through into the code that follows, with no return of its own.
- `pulse_control_bit_zero_then_continue_alt`: Invokes a control entry, clears and then sets bit zero through a0 around a second call, sets d2 to thirteen, and branches onward.
- `compare_words_repeat_control_alt`: Tests d0 against d1, branches to completion if equal or to increment logic if greater, otherwise calls a control entry and repeats the comparison.
- `clear_bit_increment_retry_alt`: Clears bit one at a0, adds one to d0, calls the alternate bit-zero pulse entry, and returns to the shared compare loop by branching.
