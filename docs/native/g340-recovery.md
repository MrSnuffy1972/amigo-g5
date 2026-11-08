# Game 340 (hellbent (1988)(novagen)[cr qtx][t +1 mfc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00340). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| Hellbent (1988)(Novagen)[cr QTX][t +1 MFC].adf:Hellbent | 2 | 226 | 9 |
| Hellbent (1988)(Novagen)[cr QTX][t +1 MFC].adf:bootblock | 1 | 18 | 1 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EAE) A7=an address (A7)=an address, slid to exec.library -an address): 628 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Hellbent (1988)(Novagen)[cr QTX][t +1 MFC].adf:Hellbent

- `install_boot_vector_and_copy_tables`: Stores the address of the boot routine into the vector table at offset 128 and calls the system trap.
- `check_bit_and_init_table`: Tests bit 0 of a global long variable and, if that bit is clear, stores the constant 20081 as two 32-bit words and one 16-bit word into a global table, then jumps to a routine at a fixed address.
- `setup_blitter_buffer_and_process_table`: Clears a 4096-word block of memory, sets the DMA enable register to enable the blitter channel, copies a 4096-word table into that block, then reads a sequence of 16-bit words from it and processes each one, stopping at the end marker.
- `tiny_sub`: Small routine of 1 instruction that subtracts d2 from d1.
- `store_four_word_entry`: Stores two input words from registers d0 and d1 into consecutive 16-bit slots of a buffer pointed to by a2, interleaving two fixed 16-bit constants, then returns the first input word increased by 256.
- `init_buffers_and_run_loop`: Initializes a buffer with fixed and input words, allocates and sets up several structures through library calls, then runs a loop that reads a byte from a table and branches based on its value.
- `call_library_pair_twice`: Calls two library functions at fixed offsets from a base, alternating between them twice, passing the same register values to each call, then adds a constant to a register and returns.
- `load_indexed_table_byte`: Reads a bit from a global long variable and loads one of two addresses depending on whether the bit is set, then fetches a byte from a table indexed by a shifted value and calls another routine before restoring the registers and returning.
- `enable_interrupts_copy_table`: Enables interrupts and disables DMA channels via the custom registers, then copies twenty bytes from a source table into a fixed destination using a loop that decrements a counter, and finally jumps to that destination.

### Hellbent (1988)(Novagen)[cr QTX][t +1 MFC].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
