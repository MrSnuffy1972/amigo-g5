# Game 1290 (robocop (1987)(ocean)[cr oracle]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01290). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 24 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 6 of 6 named (0 author symbols), 6 of 6 described (1 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 128 of 129 (99.22%) |
| Recovered without load address (counted in proof 1, flagged) | 113201 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 163 of 163 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 267348 |
| code | 460 |
| data | 279875 |
| padding | 5277 |
| unused | 348160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Robocop (1987)(Ocean)[cr Oracle].adf:robocop | 1 | 145 | 5 |
| Robocop (1987)(Ocean)[cr Oracle].adf:bootblock | 1 | 18 | 1 |
| Robocop (1987)(Ocean)[cr Oracle].adf:T (raw code file, load address unknown) | 1 | 0 | 0 |
| Robocop (1987)(Ocean)[cr Oracle].adf:S1 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- robocop (1987)(ocean)[cr oracle]/Robocop (1987)(Ocean)[cr Oracle].adf:T: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- robocop (1987)(ocean)[cr oracle]/Robocop (1987)(Ocean)[cr Oracle].adf:S1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- robocop (1987)(ocean)[cr oracle]/Robocop (1987)(Ocean)[cr Oracle].adf: blocks 954-955 (bytes 488448-489472) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.07)
- robocop (1987)(ocean)[cr oracle]/Robocop (1987)(Ocean)[cr Oracle].adf: blocks 967-967 (bytes 495104-495616) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.80)
- robocop (1987)(ocean)[cr oracle]/Robocop (1987)(Ocean)[cr Oracle].adf: blocks 972-972 (bytes 497664-498176) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.81)
- robocop (1987)(ocean)[cr oracle]/Robocop (1987)(Ocean)[cr Oracle].adf: blocks 983-983 (bytes 503296-503808) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.79)
- robocop (1987)(ocean)[cr oracle]/Robocop (1987)(Ocean)[cr Oracle].adf: blocks 986-987 (bytes 504832-505856) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.21)
- robocop (1987)(ocean)[cr oracle]/Robocop (1987)(Ocean)[cr Oracle].adf: blocks 992-993 (bytes 507904-508928) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.36)
- robocop (1987)(ocean)[cr oracle]/Robocop (1987)(Ocean)[cr Oracle].adf: blocks 995-995 (bytes 509440-509952) outside the filesystem hold data, not code (opcode density 0.008, entropy 1.76)
- robocop (1987)(ocean)[cr oracle]/Robocop (1987)(Ocean)[cr Oracle].adf: blocks 1005-1005 (bytes 514560-515072) hold ASCII text
- robocop (1987)(ocean)[cr oracle]/Robocop (1987)(Ocean)[cr Oracle].adf: blocks 1008-1010 (bytes 516096-517632) outside the filesystem hold data, not code (opcode density 0.005, entropy 3.68)
- 2 never-loaded region(s), 113201 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Robocop (1987)(Ocean)[cr Oracle].adf:robocop

- `allocate_memory_copy_block`: Calls a library routine to allocate memory, and if the returned pointer is zero loads an alternate address from the library vector, then copies a block of words from a source table into that address.
- `enable_ints_copy_code`: Enables interrupts and DMA channels by writing masks to the custom registers, clears a copper list register, and copies a four-word block from a source table into a destination address.
- `store_address_load_value`: Loads an address into a register, stores it into a global variable at offset 16, and loads the value 8192 into register d7.
- `save_stack_copy_words`: Saves the stack pointer into a global variable and copies a sequence of words from one table into another until the source pointer passes the destination, then jumps through a vector.
- `copy_words_decrement`: Copies words from a source block pointed to by a1 into a destination block at a2 while both pointers decrement, then branches past the copy when the counter in d0 reaches zero.

### Robocop (1987)(Ocean)[cr Oracle].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
