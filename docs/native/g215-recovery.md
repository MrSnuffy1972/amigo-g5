# Game 215 (Toyota Celica GT Rally): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00215). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901200 top-level bytes in 29 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 15 of 15 named (0 author symbols), 15 of 15 described (1 by rule, 13 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 151 of 151 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 11564 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 419 of 419 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 202250 |
| code | 1152 |
| data | 518970 |
| padding | 6796 |
| unused | 172032 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Toyota Celica GT Rally.adf:pdxintro | 2 | 108 | 4 |
| type | 2 | 117 | 2 |
| pause | 2 | 3 | 1 |
| Toyota Celica GT Rally.adf:Rally | 1 | 178 | 6 |
| Toyota Celica GT Rally.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| Toyota Celica GT Rally.adf:boot (raw code file, load address unknown) | 1 | 0 | 0 |
| Toyota Celica GT Rally.adf:winner (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- Toyota Celica GT Rally/Toyota Celica GT Rally.adf:clouds: high-entropy data file (7.63 bits/byte), classified as packed data
- Toyota Celica GT Rally/Toyota Celica GT Rally.adf:engine: high-entropy data file (7.77 bits/byte), classified as packed data
- Toyota Celica GT Rally/Toyota Celica GT Rally.adf:objects: high-entropy data file (7.82 bits/byte), classified as packed data
- Toyota Celica GT Rally/Toyota Celica GT Rally.adf:gameback: high-entropy data file (7.77 bits/byte), classified as packed data
- Toyota Celica GT Rally/Toyota Celica GT Rally.adf:gears: high-entropy data file (7.64 bits/byte), classified as packed data
- Toyota Celica GT Rally/Toyota Celica GT Rally.adf:speech: high-entropy data file (7.86 bits/byte), classified as packed data
- Toyota Celica GT Rally/Toyota Celica GT Rally.adf:norway: high-entropy data file (7.72 bits/byte), classified as packed data
- Toyota Celica GT Rally/Toyota Celica GT Rally.adf:intro: high-entropy data file (7.80 bits/byte), classified as packed data
- Toyota Celica GT Rally/Toyota Celica GT Rally.adf:music: high-entropy data file (7.85 bits/byte), classified as packed data
- Toyota Celica GT Rally/Toyota Celica GT Rally.adf:menuback: high-entropy data file (7.83 bits/byte), classified as packed data
- Toyota Celica GT Rally/Toyota Celica GT Rally.adf:sc_back: high-entropy data file (7.77 bits/byte), classified as packed data
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Toyota Celica GT Rally/Toyota Celica GT Rally.adf:boot: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Toyota Celica GT Rally/Toyota Celica GT Rally.adf:winner: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Toyota Celica GT Rally/Toyota Celica GT Rally.adf:pdxintro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- Toyota Celica GT Rally/Toyota Celica GT Rally.adf:Rally: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 11564 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Toyota Celica GT Rally.adf:pdxintro

## Copy protection found

- none found

## Functions


### Toyota Celica GT Rally.adf:pdxintro

- `copy_global_to_memory`: Copies bytes from a global variable into the memory region at 32768 until it reaches a zero byte, then returns.
- `resolve_and_shift_push`: Resolves a destination pointer from a table, shifts a 32-bit value right and calls a helper when it becomes zero, then pushes bytes and decrements a counter in a loop.
- `pop_and_shift_return`: Pops a 32-bit value off the stack, stores its low word at offset 384, disables interrupts, and returns the value shifted right by one with bit 31 set.
- `shift_and_call_helper`: Shifts a 32-bit value right one bit per iteration and calls a helper whenever the shifted value becomes zero, then returns.

### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Copies the value from register d1 into register d0 and then branches to another routine without returning.

### pause

- `wait_for_flag_clear`: Repeatedly reads a byte from a global variable and returns while its bit 6 stays set, otherwise it returns.

### Toyota Celica GT Rally.adf:Rally

- `init_custom_and_jump`: Sets up the custom register base, loads a mask value, stores it at four offsets of the custom structure, enables audio and interrupts, and jumps to the next routine.
- `copy_code_to_fast_memory`: Copies 645 bytes of code into fast memory at offset an address, pushes a return address onto the stack, and jumps to the copied routine.
- `expand_repeat_markers`: Copies bytes from a source pointer to a destination at offset 512, expanding a repeat marker whose count follows the marker, and stops when the source reaches offset an address.
- `expand_rle_block`: Decompresses a run-length encoded block from a table at address an address into a destination buffer, copying literal bytes and expanding repeat markers whose count follows the marker, then returns.
- `pop_and_shift_return_2`: Pops a 32-bit value off the stack, stores its low word at offset 384, disables interrupts, and returns the value shifted right by one with bit 31 set, otherwise it calls a helper.
- `shift_and_call_helper_2`: Shifts a 32-bit value right one bit per iteration and calls a helper when the shifted value becomes zero, then returns.

### Toyota Celica GT Rally.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_ciaa_bit6_clear`: Checks bit 6 of the CIA port A byte and, while that bit is set, keeps looping; otherwise it returns.
