# Game 888 (international karate + (1988)(system 3)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00888). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 11 of 11 named (0 author symbols), 11 of 11 described (1 by rule, 7 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 29 of 30 (96.67%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 134 of 134 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 442 |
| data | 254020 |
| padding | 1090 |
| unused | 645632 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| International Karate + (1988)(System 3).adf:add21k | 2 | 26 | 1 |
| International Karate + (1988)(System 3).adf:IK | 1 | 98 | 9 |
| International Karate + (1988)(System 3).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |

## Cases handled

- reference crashes at an address (HLE illegal instruction an address at an address: through vector 0, as before (vector 4 is 0 again)): 109944 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- international karate + (1988)(system 3)/International Karate + (1988)(System 3).adf:IK: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### International Karate + (1988)(System 3).adf:add21k

- `open_exec_and_graphics_libraries`: Clears a register, loads the library base pointer, and calls two library open routines, storing each returned handle into a global variable at offsets 44 and 48.

### International Karate + (1988)(System 3).adf:IK

- `store_next_routine_address`: Loads the address of the next routine into a register and stores it into a global variable at offset 32, then falls through into the following routine.
- `install_interrupt_vector`: Disables interrupts and DMA channels, then copies a jump vector from a table into the interrupt vector table before jumping to the copied entry point.
- `init_vector_and_dispatch_bits`: Fills a memory region with longwords in a loop, then processes a jump vector by computing checksums and branching to the bit reader with different counts depending on the results.
- `set_registers_and_branch`: Loads the value 8 into two working registers and then branches into the middle of another routine, so it only sets up these registers before handing off.
- `read_bits_and_select_path`: Calls the bit reader, then compares the result against two constants to select a branch, calling the reader again with a different count before finally branching onward.
- `copy_buffer_and_set_color`: Computes a loop count from two registers, calls the bit reader, then copies that many bytes from one address to another in a loop before storing the destination into the color register.
- `bk_checksum_error`: Depacker failure exit: the checksum did not cancel to zero, so it writes to COLOR01 sixty-five thousand times as a visible error and returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### International Karate + (1988)(System 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
