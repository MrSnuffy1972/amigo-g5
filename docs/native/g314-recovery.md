# Game 314 (War Machine): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00314). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 37 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 16 of 16 named (0 author symbols), 16 of 16 described (5 by rule, 11 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 134 of 134 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 876 of 876 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 323457 |
| code | 2722 |
| data | 108274 |
| padding | 8491 |
| unused | 458240 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| War Machine.adf:war | 1 | 99 | 3 |
| Disk-Validator | 1 | 386 | 1 |
| War Machine.adf:clearmem | 1 | 87 | 5 |
| War Machine.adf:clear | 3 | 292 | 5 |
| War Machine.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- War Machine/War Machine.adf:war: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### War Machine.adf:war

- `copy_block_and_shift_value`: Copies a block of 25 bytes from a global variable into RAM, then reads a 32-bit value and a pointer from the stack and shifts the value right, calling a helper routine each time it becomes zero, and returns.
- `pop_and_shift_word`: Pops a 32-bit word from the stack into d0, stores its low byte at a fixed offset in a structure, clears the interrupt mask, and returns the word shifted right by one bit with bit 15 forced high.
- `shift_and_accumulate_right`: Shifts the 32-bit input in d0 right by one bit in a loop, calling a helper routine each time the shifted value becomes zero, and returns the accumulated rotated result in d2.

### Disk-Validator

- `initialize_validation_table`: Initializes a validation table by writing two ASCII digit bytes for a counter value, adjusts two counters in a structure, clears a flag, and then repeatedly calls a helper routine while filling the table with entries until a stop condition is met.

### War Machine.adf:clearmem

- `cleanup_and_copy_tables`: Disables interrupts, sets a flag in a global word, and repeatedly calls a cleanup routine until a counter reaches sixteen, then frees two memory blocks and copies two 64-byte tables from a source into freshly allocated memory.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `check_and_write_dos`: Compares the bytes at offsets 0, 1, and 2 of the structure pointed to by a1 against the characters 'K', 'E', and 'V', and if all three match, stores the characters 'd', 'o', and 's' at those same offsets.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `tiny_move_rts`: Small routine of 2 instructions that copies 71 to d6, then returns.

### War Machine.adf:clear

- `process_clear_routine`: Sets up a jump table in a global structure, calls the memory manager to allocate a buffer, then iterates over a table of pointers calling a decompression routine for each entry and a cleanup routine for the final one.
- `link_table_entries`: Increments a counter in d6 when a flag in d5 is clear, otherwise walks a table of pointers, resolving nested entries and linking them together in a loop until it reaches a null entry.
- `branch_into_routine`: Contains only a single unconditional branch, so it does nothing but jump into the middle of another routine.
- `store_values_and_call_vector`: Stores a computed value and a fixed constant into two memory locations, calls another routine, and then calls through a fixed low-memory vector before restoring the registers.
- `decompress_bit_stream_backwards`: Decodes a bit-packed stream read from the end of a source buffer, using lookup tables to drive back-reference copies that write the output backwards into a destination buffer, and writes a value to the background colour register.

### War Machine.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
