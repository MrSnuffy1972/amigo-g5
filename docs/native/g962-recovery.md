# Game 962 (killing game show, the (1990)(psygnosis)(disk 1 of 2)[intro only]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00962). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901200 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 23 of 23 named (0 author symbols), 23 of 23 described (1 by rule, 22 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 13 of 13 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 197 of 197 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 597596 |
| code | 546 |
| data | 286272 |
| padding | 914 |
| unused | 15872 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:kgs_intro | 1 | 181 | 21 |
| Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |

## Cases handled

- killing game show, the (1990)(psygnosis)(disk 1 of 2)[intro only]/Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:prt2: high-entropy data file (7.62 bits/byte), classified as packed data
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- killing game show, the (1990)(psygnosis)(disk 1 of 2)[intro only]/Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:kgs_intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:kgs_intro

## Copy protection found

- none found

## Functions


### Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:kgs_intro

- `copy_table_to_global`: Copies the 16-byte table located at offset 16 from the program counter into the 16 bytes of a global variable at offset 128, then returns.
- `init_and_decompress_stream`: Enables the CPU and the CIA, resets the blitter and both copper lists, then decompresses a bit-packed stream from the source buffer into the destination buffer, copying the decompressed data until a terminator is reached.
- `compare_then_rotate`: Compares d1 with the constant 6 and branches away when the carry is set, branches away when d1 is zero, otherwise rotates d3 left through the carry after loading a value from the stack and jumps through a jumptable in a4.
- `decrement_then_dispatch`: Decrements d1 and branches away when the carry is clear, otherwise loads a library base into a6 and jumps through a jumptable in a4.
- `load_base_then_branch`: Loads a library base address into register a5 and then branches to the routine at offset 152, falling through into it.
- `dispatch_via_jumptable`: Loads the base address of a library into a6, then jumps through a jumptable in a4.
- `rotate_store_byte_jump`: Adds a constant to a register and rotates a bit through another while incrementing a counter in a loop, then stores a byte onto the stack in a second loop and compares pointers before jumping to another routine.
- `rotate_store_byte_loop`: Pops a 32-bit value from the stack into a register and rotates a bit through a register while incrementing a counter in a loop, then stores a byte onto the stack in a second loop counting down another register.
- `load_base_then_fall`: Loads a library base into a5, then falls through into a following routine.
- `index_jumptable_via_d2`: Sets d0 to 1 and loads the address of a table into a6, then jumps through that table using an index in d2, which is set to 3 when d1 equals 2 and otherwise left as passed, returning via the jumptable.
- `select_and_rotate`: Sets d0 and d2 to small constants, then branches away when d1 is zero, sets d2 to 3 when d1 equals 2, and calls a jumptable when d1 equals 3, otherwise rotating d3 left through the carry after loading a value from the stack.
- `dispatch_via_jumptable_2`: Loads a library base into a6, then jumps through a jumptable in a4.
- `compare_then_adjust`: Compares d1 with the constant 2 and branches away when they are not equal, otherwise adds 4 to d1 and falls through into a following routine.
- `compare_then_dispatch`: Compares d1 with the constant 3 and branches away when they are equal, otherwise loads a library base into a6 and jumps through a jumptable in a4.
- `add_branch_into_routine`: Adds a constant to a register and then unconditionally branches into the middle of another routine.
- `jump_selected_routine`: Loads a constant into a register, points a register at another routine, and jumps to a routine selected by a value in a register.
- `select_rotation_sequence`: Adds a constant to a register and copies it to another, then selects one of several rotation sequences based on a flag bit, decrementing a counter and restoring the status register between each branch.
- `shift_bytes_left_loop`: Shifts a block of bytes left by one position in a loop counting down a register, then compares the source and destination pointers and clears a word in the structure in a3 when the source is not behind the destination.
- `wait_cia_display_loop`: Waits by clearing bit 6 of the CIA display register in a loop that counts down a 16-bit counter from 999, then stores the value 32767 at offset -228 of the structure in a3.
- `rotate_bit_return_zero`: Clears a register and rotates a bit through it, returning to an earlier routine when the register is zero, otherwise looping a fixed number of times before jumping to another routine.
- `pop_stack_rotate_loop`: Pops a 32-bit value from the stack into a register and stores it as a word in the structure in a3, then rotates a bit through a register while incrementing a counter in a loop that counts down another register.

### Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_words_repeat_loop`: Copies 16-bit words from the source table at offset 16 to the destination table at offset 432 in a loop, then jumps back to the start of the table copy to repeat it forever.
