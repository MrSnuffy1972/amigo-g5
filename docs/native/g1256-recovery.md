# Game 1256 (power drift (1989)(activision)[cr defjam - ccs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01256). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802304 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 22 of 22 named (0 author symbols), 22 of 22 described (3 by rule, 18 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 104 of 105 (99.05%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 704 of 704 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 2014 |
| data | 1110542 |
| padding | 4692 |
| unused | 685056 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:pdintro | 1 | 46 | 3 |
| Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:add21 | 2 | 26 | 1 |
| Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:runcheck | 2 | 13 | 2 |
| Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:intro | 1 | 155 | 3 |
| Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:memfix | 2 | 39 | 1 |
| Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:failat | 2 | 117 | 2 |
| Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:GAME | 1 | 178 | 6 |
| Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:endif | 1 | 2 | 1 |
| Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:if | 2 | 117 | 2 |
| Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:bootblock | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |

## Cases handled

- power drift (1989)(activision)[cr defjam - ccs](disk 2 of 2)/Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1592 non-empty blocks: matched against track-loaded segments
- reference crashes at an address (HLE illegal instruction an address at an address: through vector 0, as before (vector 4 is 0 again)): 109944 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- power drift (1989)(activision)[cr defjam - ccs](disk 2 of 2)/Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 2 of 2).adf: blocks 11-328 (bytes 5632-168448) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.85)
- power drift (1989)(activision)[cr defjam - ccs](disk 2 of 2)/Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 2 of 2).adf: blocks 332-571 (bytes 169984-292864) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.50)
- power drift (1989)(activision)[cr defjam - ccs](disk 2 of 2)/Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 2 of 2).adf: blocks 715-868 (bytes 366080-444928) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.26)
- power drift (1989)(activision)[cr defjam - ccs](disk 2 of 2)/Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 2 of 2).adf: blocks 880-1759 (bytes 450560-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.93)
- power drift (1989)(activision)[cr defjam - ccs](disk 1 of 2)/Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:pdintro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- power drift (1989)(activision)[cr defjam - ccs](disk 1 of 2)/Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- power drift (1989)(activision)[cr defjam - ccs](disk 1 of 2)/Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:GAME: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:pdintro
- Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:intro

## Copy protection found

- none found

## Functions


### Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:pdintro

- `disable_audio_and_dma`: Sets up the custom chip base pointer, clears a scratch register, and disables audio and DMA by writing the enable pattern to the control registers, then stores a boot vector address into the system vector table.
- `copy_and_execute_block`: Copies a block of code from a routine to a fixed memory area, sets up a trap frame on the stack, and jumps to the copied code at an offset from the destination.
- `decode_and_copy_stream`: Copies a run of bytes from the input pointer to a fixed output area, expanding a repeat opcode by writing the following byte a counted number of times, and stores each decoded byte to both the output pointer and a memory-mapped location until the input reaches the end marker.

### Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:add21

- `initialize_libraries_and_cleanup`: Opens two libraries via base pointers and saves their handles into a global variable, sets a byte in a structure reached through a pointer chain, then calls four more library functions before returning.

### Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:runcheck

- `reset_memory_check`: Checks a global variable for a magic value and clears it, otherwise it writes a sequence of values to the global variable and returns a constant.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:intro

- `copy_block_and_decode_stream`: Copies a block of bytes from a source table to a destination table, choosing which table to copy from based on their relative addresses, then decodes a compressed data stream by shifting a 32-bit value and storing selected bytes from a lookup table.
- `store_byte_and_rotate`: Copies the low byte of a 32-bit value from a0 into d7, stores d7 as a 16-bit value at a5, clears the condition code register, then rotates the value right by one bit and returns.
- `shift_and_rotate_loop`: Shifts the 32-bit value in d0 right by one bit on each iteration, calling another routine and rotating the result into d2 when the shifted bit is set, repeating d1 times, then returns.

### Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:memfix

- `traverse_linked_list_and_store`: Follows a linked list via a pointer stored in a global variable, checks a flag in each node, extracts a field from the matching node, stores a byte to a memory-mapped location if the field is zero, and writes masked values to a global variable before returning.

### Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:failat

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Copies the contents of one register to another and then branches to a later point in another routine.

### Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:GAME

- `init_custom_and_install_vector`: Initializes the custom chip registers by disabling audio and DMA channels, then installs a boot handler into the system vector table.
- `copy_table_and_jump`: Copies 645 bytes from a data table into the memory area at an address, then restores the interrupt mask and resets the manual stack pointer before jumping to the routine at address an address.
- `decode_and_copy_stream_2`: Copies a run of bytes from the input pointer to a memory area starting at a global variable, expanding a repeat opcode by writing the following byte a counted number of times, and stores each decoded byte to both the output pointer and a memory-mapped location until the input reaches the end marker.
- `decode_huffman_stream`: Decodes a Huffman-style compressed stream by reading a bit at a time and copying literal bytes or expanding repeat codes into a fixed output buffer, then stores the decompressed data to a global variable.
- `load_shift_and_return`: Pops a longword off the stack, saves its low byte to a scratch register, stores that byte to a structure field, and arithmetic-shifts the value right before returning.
- `shift_and_loop_call`: Loops decrementing a counter while shifting a value, calling a routine when the value becomes zero and using its result, and returns when the counter reaches zero.

### Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:endif

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:if

- `find_max_and_prepare_buffer_2`: Walks a linked list of table entries to find the maximum value in a field, then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_d1_to_d0`: Copies the value in register d1 into register d0 and then branches to the routine that opens two libraries and stores their handles.

### Power Drift (1989)(Activision)[cr Defjam - CCS](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
