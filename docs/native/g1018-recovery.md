# Game 1018 (Hellbent): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01018). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 10 of 10 named (0 author symbols), 10 of 10 described (2 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 56 of 56 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 244 of 244 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 159450 |
| code | 850 |
| data | 15703 |
| padding | 637 |
| unused | 724480 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Hellbent.adf:Hellbent | 2 | 226 | 9 |
| Hellbent.adf:bootblock | 1 | 18 | 1 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EAE) A7=an address (A7)=an address, slid to exec.library -an address): 628 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Hellbent.adf:Hellbent

- `initialize_and_jump`: Stores the address of a routine into a global vector, executes a system trap, then copies a block of 32-bit words from a source table into a destination block and copies 38 bytes from another table into a buffer before jumping to the code at a fixed global address.
- `store_marker_if_bit_set`: Tests bit 0 of a long variable read from a global location, and if it is set stores the constant 48753 at the long at offset 114 and the word 48753 at the word at offset 118, then jumps to the routine at address 6100.
- `init_and_process_input_buffers`: Clears a 32768-word buffer and copies a fixed table of 16384 words into a second buffer, then repeatedly processes input words by calling helper routines and updating a global counter until the vertical-blank interrupt arrives, at which point it restores the previous interrupt vector and disables DMA.
- `tiny_sub`: Small routine of 1 instruction that subtracts d2 from d1.
- `store_four_word_entry`: Stores the word in d0 and the word in d1 into consecutive 16-bit slots of the buffer pointed to by a2, interleaving the constants -2 and 384 between them, then returns the value of d0 increased by 256.
- `init_buffers_and_run_loop`: Initializes a buffer with fixed and input words, allocates and sets up several structures through library calls, then runs a loop that reads a byte from a table and branches based on its value.
- `call_library_pair_twice`: Calls the library function at offset 240 from a6 twice, alternating with the library function at offset 60 from a6, passing the same register values to each call, then adds 384 to d0 and returns.
- `load_indexed_table_byte`: Reads a bit from a global long variable and loads one of two addresses depending on whether the bit is set, then fetches a byte from a table indexed by a shifted value and calls another routine before restoring the registers and returning.
- `copy_block_and_jump`: Calls a routine through a6, disables DMA and interrupts via the custom and CIA registers, copies 32 bytes from a table into a block of memory, then jumps into that block.

### Hellbent.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
