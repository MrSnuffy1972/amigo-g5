# Game 2446 (links - the challenge of golf v1.53 (1990)(access)[cr opium][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02446). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802656 top-level bytes in 32 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 23 of 23 named (0 author symbols), 23 of 23 described (5 by rule, 9 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 299 of 299 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 751 of 751 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 501208 |
| code | 1978 |
| data | 1216824 |
| padding | 9942 |
| unused | 72704 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Links - The Challenge of Golf v1.53 (1990)(Access)[cr Opium][a](Disk 1 of 2).adf:GOLF | 2 | 267 | 8 |
| run | 2 | 117 | 2 |
| Links - The Challenge of Golf v1.53 (1990)(Access)[cr Opium][a](Disk 1 of 2).adf:LINKS.OPM | 2 | 267 | 8 |
| Links - The Challenge of Golf v1.53 (1990)(Access)[cr Opium][a](Disk 1 of 2).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 82 | 4 |

## Cases handled

- links - the challenge of golf v1.53 (1990)(access)[cr opium][a](disk 1 of 2)/Links - The Challenge of Golf v1.53 (1990)(Access)[cr Opium][a](Disk 1 of 2).adf:INTROSND.LZ: high-entropy data file (7.85 bits/byte), classified as packed data
- links - the challenge of golf v1.53 (1990)(access)[cr opium][a](disk 1 of 2)/Links - The Challenge of Golf v1.53 (1990)(Access)[cr Opium][a](Disk 1 of 2).adf:SOUNDS.LZ: high-entropy data file (7.74 bits/byte), classified as packed data
- links - the challenge of golf v1.53 (1990)(access)[cr opium][a](disk 1 of 2)/Links - The Challenge of Golf v1.53 (1990)(Access)[cr Opium][a](Disk 1 of 2).adf:TOPVIEW.LZ: high-entropy data file (7.76 bits/byte), classified as packed data
- links - the challenge of golf v1.53 (1990)(access)[cr opium][a](disk 1 of 2)/Links - The Challenge of Golf v1.53 (1990)(Access)[cr Opium][a](Disk 1 of 2).adf:GOLF1.LZ: high-entropy data file (7.72 bits/byte), classified as packed data
- links - the challenge of golf v1.53 (1990)(access)[cr opium][a](disk 1 of 2)/Links - The Challenge of Golf v1.53 (1990)(Access)[cr Opium][a](Disk 1 of 2).adf:SCORECARD.LZ: high-entropy data file (7.90 bits/byte), classified as packed data
- links - the challenge of golf v1.53 (1990)(access)[cr opium][a](disk 2 of 2)/Links - The Challenge of Golf v1.53 (1990)(Access)[cr Opium][a](Disk 2 of 2).adf:COURSES/PRACTICE.CRX: high-entropy data file (7.77 bits/byte), classified as packed data
- links - the challenge of golf v1.53 (1990)(access)[cr opium][a](disk 2 of 2)/Links - The Challenge of Golf v1.53 (1990)(Access)[cr Opium][a](Disk 2 of 2).adf:COURSES/bountifu.crs: high-entropy data file (7.95 bits/byte), classified as packed data
- runtime segment an address (play-long-joy): 49 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- links - the challenge of golf v1.53 (1990)(access)[cr opium][a](disk 1 of 2)/Links - The Challenge of Golf v1.53 (1990)(Access)[cr Opium][a](Disk 1 of 2).adf:LINKS.OPM: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Links - The Challenge of Golf v1.53 (1990)(Access)[cr Opium][a](Disk 1 of 2).adf:GOLF

- `load_amigados_hunk_executable`: Program entry point that loads an AmigaDOS executable: it allocates memory for each hunk, reads hunk type codes, handles relocation entries and frees memory through the exec release call.
- `decrement_word_until_wrap`: Decrements a 16-bit register in a loop until it wraps past zero, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `shift_bit_buffer`: Shifts a multi-word bit buffer by a specified amount using memory pointers and bitwise logic.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_multiword_bit_buffer`: Shifts a multi-word bit buffer by a count taken from a register, one bit position at a time, reading new words from memory when the buffer empties and updating a carry bit.
- `execute_trap`: Triggers a software interrupt via a trap instruction.

### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_then_call_pointer`: If the destination pointer is set, copies sixteen longwords from a fixed offset in a base register into it, then calls the routine stored in an address register.

### Links - The Challenge of Golf v1.53 (1990)(Access)[cr Opium][a](Disk 1 of 2).adf:LINKS.OPM

- `parse_amigados_hunk_table`: Iterates through a memory structure to identify and process AmigaDOS hunk type codes.
- `decrement_register_loop`: Decrements a value in a register until it reaches zero.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `shift_bit_buffer_2`: Performs a bitwise shift on a multi-word buffer by moving data between memory locations based on a count.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `shift_multiword_bit_buffer_2`: Shifts a multi-word bit buffer for a count derived from the difference between two registers, one bit position at a time, reading new words from memory when the buffer empties.
- `shift_multiword_bit_buffer_3`: Shifts a multi-word bit buffer by a specified number of bits.
- `check_range_and_trap`: Raises trap number six, writes a word at an offset computed from a register, and raises trap six again when a value lies outside a two-longword range read through a pointer.

### Links - The Challenge of Golf v1.53 (1990)(Access)[cr Opium][a](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_result_storage_and_cleanup`: Stores several values from registers into global variables, calls a routine to process a result, and then conditionally executes a series of library functions to release resources or clear flags based on the state of a structure.
- `call_open_library`: Wrapper that calls the exec library function OpenLibrary and returns its result to the caller.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `store_result_and_branch`: Loads a pointer from a relative offset, calls another routine, stores the returned value in a global variable, and branches to a location based on the result.
