# Game 2981 (Brides of Dracula): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02981). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903600 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 22 raw-word lines |
| Functions named and described | pass, 45 of 45 named (0 author symbols), 45 of 45 described (15 by rule, 21 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 156 of 156 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 9728 bytes, 9 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1859 of 1868 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 20028 |
| code | 7110 |
| data | 848812 |
| padding | 2 |
| unused | 27648 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Brides of Dracula.adf:bootblock | 1 | 34 | 2 |
| logged disk load from Brides of Dracula.adf boot-block load | 1 | 94 | 3 |
| logged disk load from Brides of Dracula.adf boot-block load | 1 | 92 | 2 |
| logged disk load from Brides of Dracula.adf boot-block load | 1 | 94 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 428 | 26 |
| Brides of Dracula.adf bytes 45056-49152 (code never loaded by a reference run) | 1 | 1126 | 9 |
| Brides of Dracula.adf bytes 375296-380928 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Brides of Dracula/Brides of Dracula.adf: custom-format (non-DOS) disk, 1704 non-empty blocks: matched against track-loaded segments
- Brides of Dracula/Brides of Dracula.adf: the boot block reads 1536 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- Brides of Dracula/Brides of Dracula.adf: the boot block reads 14336 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- Brides of Dracula/Brides of Dracula.adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 156 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Brides of Dracula/Brides of Dracula.adf: bytes 45056-49152 look like code no run loaded: recovered position-independently (load address unknown)
- Brides of Dracula/Brides of Dracula.adf: bytes 375296-380928 look like code no run loaded: recovered position-independently (load address unknown)
- Brides of Dracula/Brides of Dracula.adf: 5 logged loads cover 31232 bytes of the image
- Brides of Dracula/Brides of Dracula.adf: 2 track-loaded blocks found in recovered runtime segments and classified through them
- Brides of Dracula/Brides of Dracula.adf: blocks 5-10 (bytes 2560-5632) hold ASCII text
- Brides of Dracula/Brides of Dracula.adf: blocks 22-22 (bytes 11264-11776) outside the filesystem hold data, not code (opcode density 0.023, entropy 4.61)
- Brides of Dracula/Brides of Dracula.adf: blocks 24-85 (bytes 12288-44032) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.01)
- Brides of Dracula/Brides of Dracula.adf: blocks 112-236 (bytes 57344-121344) outside the filesystem hold data, not code (opcode density 0.003, entropy 7.12)
- Brides of Dracula/Brides of Dracula.adf: blocks 240-456 (bytes 122880-233984) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.97)
- Brides of Dracula/Brides of Dracula.adf: blocks 458-516 (bytes 234496-264704) outside the filesystem hold data, not code (opcode density 0.002, entropy 7.07)
- Brides of Dracula/Brides of Dracula.adf: blocks 521-730 (bytes 266752-374272) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.38)
- Brides of Dracula/Brides of Dracula.adf: blocks 746-904 (bytes 381952-463360) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.18)
- Brides of Dracula/Brides of Dracula.adf: blocks 913-913 (bytes 467456-467968) outside the filesystem hold data, not code (opcode density 0.043, entropy 3.60)
- Brides of Dracula/Brides of Dracula.adf: blocks 917-1127 (bytes 469504-577536) outside the filesystem hold data, not code (opcode density 0.022, entropy 5.31)
- Brides of Dracula/Brides of Dracula.adf: blocks 1129-1129 (bytes 578048-578560) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.98)
- Brides of Dracula/Brides of Dracula.adf: blocks 1132-1590 (bytes 579584-814592) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.33)
- Brides of Dracula/Brides of Dracula.adf: blocks 1592-1639 (bytes 815104-839680) outside the filesystem hold data, not code (opcode density 0.024, entropy 5.05)
- Brides of Dracula/Brides of Dracula.adf: blocks 1650-1715 (bytes 844800-878592) hold ASCII text
- Brides of Dracula/Brides of Dracula.adf: blocks 1744-1759 (bytes 892928-901120) hold ASCII text
- 1 never-loaded region(s), 5632 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Brides of Dracula.adf:bootblock

- `initialize_dma_channels`: Configures DMA control registers and interrupt settings, initializes I/O request structures with specific command codes and memory addresses, then performs multiple I/O operations using the exec library's DoIO function to set up hardware state for boot process execution.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### logged disk load from Brides of Dracula.adf boot-block load

- `write_decrementing_global_until_wrap`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Brides of Dracula.adf boot-block load

- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Brides of Dracula.adf boot-block load

- `write_decrementing_global_until_wrap_2`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer_3`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_3`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `transfer_and_process_data`: Transfers data between memory locations using multiple copy operations, calls other routines to process the data, and performs loop control based on a counter variable.
- `process_tile_data`: Processes tile data by reading bytes from two memory locations, performing bit manipulation and comparisons, then storing modified values back to the first location.
- `frag_cmp_beq_tst_bne_jsr_bra_dbra_rts`: Short code fragment that compares d0 with 0, then branches if the last result was zero, then tests a fixed memory address, then branches if the last result was not zero, then calls another routine, then jumps to another point in the code, then calls another routine, then loops back while counting down d0, then returns.
- `initialize_memory_blocks`: Initializes memory blocks at specific addresses by clearing multiple registers and writing zero values to arrays of memory locations.
- `frag_clr_move_mulu_addq_rts`: Short code fragment that clears d0, then copies a fixed memory address to d0, then multiplies d0 by 5 (unsigned), then adds 1 to d0, then copies d0 to a fixed memory address, then returns.
- `frag_lea_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 6466 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `copy_and_decode_data`: Copies data from one memory location to another while decoding compressed data, handling both direct copies and repeated byte sequences based on bit testing.
- `frag_move_dbra_moveq_bra`: Short code fragment that copies d2 to the memory at a2 (advancing a2), then loops back while counting down d1, then loads 0 into d1, then jumps to another point in the code.
- `frag_move_dbra_moveq_bra_2`: Short code fragment that copies the memory at a1 (advancing a1) to d2, then copies d2 to the memory at a2 (advancing a2), then loops back while counting down d0, then loads 0 into d0, then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_moveq_move_beq_cmp_dbra_rts`: Short code fragment that loads 0 into d1, then copies the memory at a1 (advancing a1) to d2, then copies the memory at a1 (advancing a1) to d0, then branches if the last result was zero, then compares d0 with 255, then branches if the last result was zero, then copies d0 to the memory at a3 (advancing a3), then loops back while counting down d2, then returns.
- `frag_move_dbra_moveq_rts`: Short code fragment that copies the memory at a1 (advancing a1) to d1, then copies d0 to the memory at a3 (advancing a3), then loops back while counting down d1, then loads 0 into d1, then loops back while counting down d2, then returns.
- `setup_graphics_registers`: Sets up various graphics hardware registers including display dimensions, color settings, and DMA control registers for video output.
- `handle_color_data`: Handles color data by copying values from memory to hardware registers, with conditional branches that process different data sets based on a global variable value.
- `frag_move_dbra_rts`: Short code fragment that copies 0 to the memory at a1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then copies 0 to the memory at a2 (advancing a2), then loops back while counting down d7, then returns.
- `distribute_data_blocks`: Distributes data blocks across multiple memory locations using a loop that copies words from source addresses to destination addresses with specific offset calculations.
- `update_bit_flags`: Updates a global variable's bits based on conditions derived from another global variable's value, then performs conditional data transfers using a lookup table.
- `frag_move_ori`: Short code fragment that copies d0 to the memory at a0 (advancing a0), then merges 196 into d5, then merges 4 into d5, then merges 8 into d5.
- `copy_data_loop`: Copies word values from two source pointers to two destination pointers in a loop, processing fifteen iterations before returning.
- `copy_data_block_a`: Copies data between memory locations using a transfer routine, performing two separate copy operations with different source and destination addresses.
- `copy_data_block_b`: Copies data between memory locations using a transfer routine, performing two separate copy operations with different source and destination addresses.
- `frag_cmpi_bne_adda_dbra_rts`: Short code fragment that compares the memory at a0 with 0, then branches if the last result was not zero, then adds 16 to a0, then loops back while counting down d7, then returns.
- `advance_channel_period_chain`: Updates a chain of four-long channel records in sequence, replacing each value and adjusting its accumulator by the difference.
- `frag_lea_move_clr_adda_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a1, then copies 3 to d1, then clears the memory at an offset from a1, then adds 16 to a1, then loops back while counting down d1, then copies 15 to a fixed memory address, then returns.
- `frag_tst_beq_btst_bsr_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then tests bit 5 of a fixed memory address, then calls another routine, then returns.
- `increment_and_compare`: Increments a global variable, compares it with another value, and branches based on the comparison result to control program flow.

### Brides of Dracula.adf bytes 45056-49152 (code never loaded by a reference run)

- `setup_hardware_and_interrupts`: Sets up hardware registers including interrupt control, color settings, and memory mapping, then performs a series of operations involving keyboard input and system state management.
- `frag_move_not_lea_lsl_bsr_addq_lsr`: Short code fragment that copies a fixed memory address to d3, then inverts d3, then copies a fixed memory address to d0, then loads the address of a fixed memory address into a0, then shifts d3 left by 1, then calls another routine, then copies a fixed memory address to d0, then adds 1 to a0, then shifts d3 right by 1.
- `calculate_and_store_byte`: Performs bitwise calculations on register values to generate a byte result, then stores this value in memory using a pointer address.
- `lookup_byte_value`: Iterates through a byte array and performs conditional operations based on byte values, then returns a lookup result from another array using calculated offset.
- `process_sprite_dma`: Reads sprite data from a global table, calculates DMA transfer parameters including address and count values, writes these to hardware registers for DMA setup, and then calls another routine to finalize the DMA configuration.
- `setup_sprite_data`: Calculates and stores sprite position and attribute data from register values and global variables, then calls another routine to process DMA operations.
- `initialize_render_state`: Sets up rendering state data including position coordinates, color values, and pointer addresses, then performs multiple iterations of DMA processing with incrementing pointers.
- `wait_for_dma_ready`: Waits for DMA to become ready by continuously checking a hardware register bit until it clears, then returns immediately.
- `loop_counter_decrement`: Performs a simple decrement loop operation on a register value, executing the loop three times before returning.
