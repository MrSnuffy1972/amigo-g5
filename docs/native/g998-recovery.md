# Game 998 (war machine (1989)(players premier)[cr acc][t acc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00998). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 36 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 16 of 16 named (0 author symbols), 16 of 16 described (5 by rule, 11 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 134 of 134 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 876 of 876 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 253451 |
| code | 2722 |
| data | 103794 |
| padding | 8225 |
| unused | 532992 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| War Machine (1989)(Players Premier)[cr ACC][t ACC].adf:war | 1 | 99 | 3 |
| Disk-Validator | 1 | 386 | 1 |
| War Machine (1989)(Players Premier)[cr ACC][t ACC].adf:clearmem | 1 | 87 | 5 |
| War Machine (1989)(Players Premier)[cr ACC][t ACC].adf:clear | 3 | 292 | 5 |
| War Machine (1989)(Players Premier)[cr ACC][t ACC].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- war machine (1989)(players premier)[cr acc][t acc]/War Machine (1989)(Players Premier)[cr ACC][t ACC].adf:war: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### War Machine (1989)(Players Premier)[cr ACC][t ACC].adf:war

- `copy_block_and_rotate_bits`: Copies a block of bytes from a source table to a destination in the an address data area, then reads a 32-bit value and a pointer from the stack and rotates the value's bits into a byte buffer, filling the shifted-out bits from the buffer.
- `restore_and_rotate_value`: Pops a 32-bit value from the stack into d0, stores its low byte at offset 5 in a5, sets the interrupt mask, and returns d0 rotated right by one bit with bit 31 forced on.
- `shift_and_fill_bits`: Shifts the 32-bit value in d0 right by one bit into d2 and decrements d1, calling a routine to fill d2 with the shifted-out bits whenever d0 becomes zero, repeating until d1 reaches zero.

### Disk-Validator

- `format_digit_buffer_entry`: Writes the decimal digits of a value read from a global variable into two consecutive bytes of a buffer indexed by a computed offset, decrements two counters stored in that global, clears a flag, and then calls another routine.

### War Machine (1989)(Players Premier)[cr ACC][t ACC].adf:clearmem

- `cleanup_and_copy_tables`: Disables interrupts, sets a flag in a global word, and repeatedly calls a cleanup routine until a counter reaches sixteen, then frees two memory blocks and copies two 64-byte tables from a source into freshly allocated memory.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `check_and_replace_key`: Compares the bytes at offsets 0, 1 and 2 of the structure pointed to by a1 against the characters 'K', 'E' and 'V', and if all three match stores the characters 'd', 'o' and 's' at those same offsets.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `tiny_move_rts`: Small routine of 2 instructions that copies 71 to d6, then returns.

### War Machine (1989)(Players Premier)[cr ACC][t ACC].adf:clear

- `initialize_vector_table`: Initializes a global vector table by storing the entry point and a following routine address at offsets 20 and 24, then calls a setup routine.
- `link_structure_chain`: Tests a flag in d5 and increments d6 if it is clear, otherwise walks a table of pointers, computing offsets from d6 and d1 to link together a chain of structures in a loop.
- `jump_to_routine`: Contains a single branch instruction that jumps to another routine, transferring control without returning.
- `store_values_and_call_vector`: Stores a computed value and a fixed constant into two memory locations, calls another routine, and then calls through a fixed low-memory vector.
- `decompress_bit_stream_backwards`: Decodes a bit-packed stream read from the end of a buffer, using lookup tables to drive back-reference copies that write output backwards, and writes a value to the background colour register at a fixed address.

### War Machine (1989)(Players Premier)[cr ACC][t ACC].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
