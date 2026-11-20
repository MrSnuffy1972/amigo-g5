# Game 4007 (inferior (1996)(ego software)[cr znt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04007). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1828480 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 54 raw-word lines |
| Functions named and described | pass, 139 of 139 named (0 author symbols), 139 of 139 described (61 by rule, 75 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2504 of 2504 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2560 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4307 of 4307 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 22032 |
| code | 24082 |
| data | 1695944 |
| padding | 918 |
| unused | 85504 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Inferior (1996)(Ego Software)[cr ZNT](Disk 1 of 2).adf:bootblock | 1 | 17 | 1 |
| logged disk load from Inferior (1996)(Ego Software)[cr ZNT](Disk 1 of 2).adf boot-block load | 1 | 116 | 2 |
| logged disk load from Inferior (1996)(Ego Software)[cr ZNT](Disk 1 of 2).adf boot-block load | 1 | 616 | 64 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 747 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 775 | 29 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 35 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1915 | 24 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 86 | 4 |
| Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf bytes 762880-763392 (code never loaded by a reference run) | 1 | 0 | 0 |
| Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf bytes 764416-765440 (code never loaded by a reference run) | 1 | 0 | 0 |
| Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf bytes 772096-773120 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- inferior (1996)(ego software)[cr znt](disk 1 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1620 non-empty blocks: matched against track-loaded segments
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1728 non-empty blocks: matched against track-loaded segments
- inferior (1996)(ego software)[cr znt](disk 1 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 1 of 2).adf: the boot block reads 2560 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- inferior (1996)(ego software)[cr znt](disk 1 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 1 of 2).adf: the boot block reads 4608 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 347 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 592 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 22 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1470 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 73 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: bytes 762880-763392 look like code no run loaded: recovered position-independently (load address unknown)
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: bytes 764416-765440 look like code no run loaded: recovered position-independently (load address unknown)
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: bytes 772096-773120 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying code at an address (50 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- inferior (1996)(ego software)[cr znt](disk 1 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 1 of 2).adf: 2 logged loads cover 7168 bytes of the image
- inferior (1996)(ego software)[cr znt](disk 1 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 1 of 2).adf: blocks 1730-1730 (bytes 885760-886272) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.71)
- inferior (1996)(ego software)[cr znt](disk 1 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 1 of 2).adf: blocks 1757-1757 (bytes 899584-900096) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.28)
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: 3 logged loads cover 2560 bytes of the image
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: 28 track-loaded blocks found in recovered runtime segments and classified through them
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: blocks 1380-1488 (bytes 706560-762368) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.43)
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: blocks 1496-1498 (bytes 765952-767488) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.21)
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: blocks 1500-1500 (bytes 768000-768512) outside the filesystem hold data, not code (opcode density 0.027, entropy 5.90)
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: blocks 1503-1505 (bytes 769536-771072) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.32)
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: blocks 1519-1519 (bytes 777728-778240) outside the filesystem hold data, not code (opcode density 0.016, entropy 3.85)
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: blocks 1523-1523 (bytes 779776-780288) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.82)
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: blocks 1528-1529 (bytes 782336-783360) outside the filesystem hold data, not code (opcode density 0.031, entropy 5.46)
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: blocks 1532-1543 (bytes 784384-790528) outside the filesystem hold data, not code (opcode density 0.008, entropy 4.39)
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: blocks 1547-1682 (bytes 792064-861696) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.04)
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: blocks 1684-1695 (bytes 862208-868352) outside the filesystem hold data, not code (opcode density 0.025, entropy 4.72)
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: blocks 1705-1705 (bytes 872960-873472) outside the filesystem hold data, not code (opcode density 0.008, entropy 3.73)
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: blocks 1709-1715 (bytes 875008-878592) outside the filesystem hold data, not code (opcode density 0.045, entropy 6.55)
- inferior (1996)(ego software)[cr znt](disk 2 of 2)/Inferior (1996)(Ego Software)[cr ZNT](Disk 2 of 2).adf: blocks 1718-1759 (bytes 879616-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.08)
- 3 never-loaded region(s), 2560 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 4 packed track region(s), 1520640 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_and_loop_2` (Inferior (1996)(Ego Software)[cr ZNT](Disk 1 of 2).adf boot-block load +an address to an address (4608 bytes)): Initializes a counter to zero, calls several subroutines, loads addresses into register pairs, and performs conditional operations based on a global variable's state. Kind: disk hardware check. Pass path: The check passes if CIAB_PRB is read and matches expected values; the routine continues normally without triggering sabotage or aborting..

## Functions


### Inferior (1996)(Ego Software)[cr ZNT](Disk 1 of 2).adf:bootblock

- `initialize_io_request`: Sets up an I/O request structure with command code 2 and specific data values, then calls the DoIO function to execute it.

### logged disk load from Inferior (1996)(Ego Software)[cr ZNT](Disk 1 of 2).adf boot-block load

- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `fetch_and_mask_value`: Reads a word value from a table using a calculated index and performs a bitwise AND operation with another value before returning the result.

### logged disk load from Inferior (1996)(Ego Software)[cr ZNT](Disk 1 of 2).adf boot-block load

- `setup_hardware_registers`: Initializes multiple hardware registers including interrupt enable, DMA control, and coprocessor settings, then enters an infinite loop updating color values.
- `process_array_elements`: Processes multiple elements of an array by loading values, calling another routine to process them, and storing the results back to memory locations.
- `frag_cmpi_bge_rts`: Short code fragment that compares d1 with an address, then branches if the comparison was greater or equal, then returns.
- `frag_subi_add_rts`: Short code fragment that subtracts an address from d1, then adds a fixed memory address to d1, then returns.
- `process_dma_entries_3`: Processes DMA control entries by loading data from memory locations, calling other routines to handle elements, and storing results back to global variables.
- `process_dma_entries_2`: Processes a sequence of DMA control entries, updating global variables and executing function pointers while managing hardware register writes for DMA operations.
- `process_dma_entries`: Processes a sequence of DMA control entries, updating global variables and executing function pointers while managing hardware register writes to enable DMA channels.
- `copy_memory_blocks`: Copies memory blocks in chunks of 10 longwords followed by remaining bytes, using division and modulo operations to determine copy counts.
- `decode_bitstream_3`: Extracts bits from a compressed data stream using bitwise operations and a lookup table to generate output bytes, while updating internal state variables.
- `frag_move_roxr_bra`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_2`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_3`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_4`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_5`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_6`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `decode_stream`: Processes a compressed data stream by extracting bits and applying bitwise operations to generate output bytes, using a lookup table for control flow.
- `decode_bitstream`: Processes a bitstream from a buffer, performing bitwise operations and table lookups to decode data, while updating internal state variables and pointers during decompression.
- `frag_move_roxr_bra_7`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_8`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_9`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `decode_bitstream_2`: Processes a bitstream from a buffer, performing bitwise operations and table lookups to decode data, while updating internal state variables.
- `frag_move_roxr_bra_10`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_11`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_12`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `rotate_and_extract`: Extracts bits from a memory location through the extend flag and performs bit rotation operations on data.
- `frag_move_roxr_bra_13`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_14`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_15`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_moveq_sub_move_add_jmp`: Short code fragment that loads 13 into d0, then subtracts an indexed entry from a2 from d0, then copies d0 to d1, then adds d0 to d0, then adds d1 to d0, then adds d0 to d0, then jumps to another point in the code.
- `rotate_and_copy`: Rotates bits from a memory location through the extend flag and copies bytes from a table to a destination buffer based on the rotated values.
- `frag_move_roxr_bra_16`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_17`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_18`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_19`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_20`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_ori_bra`: Short code fragment that merges an address into d0, then jumps to another point in the code.
- `frag_lea_move_bsr_ori_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies the memory at an offset from a0 to a fixed memory address, then copies 32767 to the memory at an offset from a0, then copies 17545 to a fixed memory address, then calls another routine, then copies a location in the code to d0, then merges 32768 into d0, then copies d0 to the memory at an offset from a0, then returns.
- `initialize_and_loop_2`: Initializes a counter to zero, calls several subroutines, loads addresses into register pairs, and performs conditional operations based on a global variable's state.
- `initialize_and_loop`: Initializes a counter to zero, calls several subroutines, loads addresses into register pairs, and performs conditional operations based on a test result.
- `frag_addi_addq_subq_bgt_bsr_rts`: Short code fragment that adds 5632 to a fixed memory address, then adds 1 to a fixed memory address, then subtracts 1 from a fixed memory address, then branches if the comparison was greater, then calls another routine, then returns.
- `frag_tst_bra`: Short code fragment that tests a fixed memory address, then jumps to another point in the code.
- `process_data_block`: Reads words from a source pointer until a matching value is found, then processes a fixed number of iterations using another routine's output to update a global variable.
- `frag_move_and_lsl_or_rts`: Short code fragment that copies the memory at a1 (advancing a1) to d0, then copies the memory at a1 (advancing a1) to d1, then masks d0 with d4, then masks d1 with d4, then shifts d1 left by 1, then merges d1 into d0, then returns.
- `validate_and_process_data`: Validates data by comparing values against expected patterns, processes data through multiple operations including XOR and bit manipulation, and updates flags accordingly.
- `frag_move_rts`: Short code fragment that copies 255 to a fixed memory address, then returns.
- `frag_and_lsl_or_rts`: Short code fragment that masks d0 with d4, then masks d1 with d4, then shifts d0 left by 1, then merges d1 into d0, then returns.
- `update_control_registers`: Reads values from memory locations, performs arithmetic operations on them, updates control bits based on comparison results, and calls other routines in a loop.
- `frag_move_bsr_rts`: Short code fragment that copies d2 to the memory at a5, then calls another routine, then copies a fixed memory address to a fixed memory address, then returns.
- `frag_bset_nop_btst_beq_bsr_bra`: Short code fragment that sets bit 1 of the memory at a5, then pauses for one no-operation step (2 times in a row), then tests bit 4 of the memory at a4, then branches if the last result was zero, then calls another routine (2 times in a row), then jumps to another point in the code.
- `frag_clr_rts`: Short code fragment that clears a fixed memory address, then returns.
- `frag_bclr_nop_bset_rts`: Short code fragment that clears bit 0 of the memory at a5, then pauses for one no-operation step (2 times in a row), then sets bit 0 of the memory at a5, then returns.
- `frag_bsr_btst_bne_rts`: Short code fragment that calls another routine, then tests bit 5 of the memory at a4, then branches if the last result was not zero, then returns.
- `frag_move_btst_bne_rts`: Short code fragment that copies 0 to a fixed memory address (2 times in a row), then copies 14 to a fixed memory address, then copies 25 to a fixed memory address, then tests bit 0 of a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_move_moveq_and_addq_bclr_bsr_rts`: Short code fragment that copies 255 to the memory at a5, then loads 3 into d0, then masks d0 with a fixed memory address, then adds 3 to d0, then copies 127 to the memory at a5, then clears bit d0 of the memory at a5, then calls another routine, then returns.
- `frag_moveq_and_addq_move_bclr_bset_rts`: Short code fragment that loads 3 into d0, then masks d0 with a fixed memory address, then adds 3 to d0, then copies 255 to the memory at a5, then clears bit d0 of the memory at a5, then sets bit d0 of the memory at a5, then returns.
- `configure_hardware_settings`: Writes various configuration values to hardware registers, reads a status bit, and modifies settings based on the bit's value before returning.
- `frag_subi_ori_asr_bne_move_rts`: Short code fragment that subtracts 57 from d4, then merges 191 into d6, then shifts d1 right by 8, then branches if the last result was not zero, then copies an address to a fixed memory address, then returns.
- `frag_ori_move_rts`: Short code fragment that merges 28 into d7, then copies a fixed memory address to a fixed memory address, then returns.
- `frag_move_rts_2`: Short code fragment that copies the memory at an offset from a0 to a fixed memory address, then returns.
- `frag_cmpi_bne_move_rts`: Short code fragment that compares a fixed memory address with 255, then branches if the last result was not zero, then copies an address to a fixed memory address, then compares a fixed memory address with 254, then branches if the last result was not zero, then returns.
- `frag_move_btst_bne_not_beq_rts`: Short code fragment that copies a fixed memory address to a fixed memory address, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then copies a fixed memory address to d0, then inverts d0, then copies d0 to a fixed memory address, then tests bit 6 of a fixed memory address, then branches if the last result was zero, then returns.
- `frag_btst_bne_cmpi_beq_rts`: Short code fragment that tests bit 7 of a fixed memory address, then branches if the last result was not zero, then compares a fixed memory address with 255, then branches if the last result was not zero, then compares a fixed memory address with 254, then branches if the last result was not zero, then tests bit 7 of a fixed memory address, then branches if the last result was zero, then returns.
- `frag_cmpi_beq_move_rts`: Short code fragment that compares a fixed memory address with an address, then branches if the last result was zero, then copies an address to a fixed memory address, then returns.
- `frag_move_rts_3`: Short code fragment that copies 1 to a fixed memory address, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_clr_move_rts`: Short code fragment that clears the memory at an offset from a0, then copies 15 to the memory at a0, then returns.
- `update_audio_dma`: Updates audio DMA configuration by managing global counters, calling other routines for audio processing, and writing values to hardware registers.
- `initialize_display`: Initializes display hardware by setting up register pointers, calling initialization routines multiple times, and clearing a specific memory location.
- `process_object_data`: Reads object properties from a structure, performs bit manipulation and arithmetic operations on coordinates, then stores computed values back into the same structure.
- `process_input_value`: Reads a value from a structure, performs bitwise operations and conditional jumps based on its value, updates fields in another structure, and returns a modified value.
- `update_input_value`: Reads a value from a structure, performs bitwise operations and conditional jumps based on its value, updates fields in another structure, and returns the result.
- `process_input_range`: Processes values from a structure located at a6, performs arithmetic operations including addition and subtraction with values from the structure, and updates fields based on comparisons.
- `calculate_and_update_2`: Processes values from a structure located at a6, performs arithmetic operations including addition and subtraction with values from the structure, updates counters based on comparisons, and calculates a result value through bit manipulation and table lookups before storing it in memory.
- `calculate_and_update`: Processes byte values from the stack, performs bit manipulation and arithmetic operations including shifts, multiplications, and additions.
- `process_input_value_2`: Processes input data from a structure, extracts nibbles, updates fields in the structure based on the input, and manipulates hardware register CIAA_PRA to control fire buttons and disk status.
- `store_input_nibble`: Reads a byte from a structure, extracts the lower four bits, and stores the result at a specific offset in the same structure.
- `negate_and_advance_string`: The routine updates a signed byte field using a nibble from a byte at offset 1F and a table of values, then clears the field if its high bit is set.
- `frag_moveq_move_cmp_beq_dbra_rts`: Short code fragment that loads 3 into d0, then copies a fixed memory address to d1, then compares d1 with a fixed memory address, then branches if the last result was zero, then loops back while counting down d0, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (bchg.b) and then falls through into the code that follows, with no return of its own.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_ori_bsr_rts`: Short code fragment that merges 136 into d1, then calls another routine, then returns.
- `decode_and_copy`: Decodes data by shifting bits, performs conditional operations based on bit values, and copies bytes between memory locations using decrementing counters.
- `frag_lsr_beq_rts_move_roxr`: Short code fragment that shifts d5 right by 1, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then returns.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (subq.w) and then falls through into the code that follows, with no return of its own.
- `shift_and_loop`: Performs bit shifting operations on a data value, handles loop iterations with decrementing counters, and manages memory pointer adjustments based on shift results.
- `process_data_blocks_2`: Processes data blocks by reading from memory locations, performing bit tests, calling other routines for processing, and updating global variables with calculated values.
- `process_data_blocks`: Processes data blocks by reading indices from memory, performing OR operations on multiple longword values, and writing results to output locations in a nested loop structure.
- `copy_block_data`: Copies 5 words from one memory location to two different locations, advancing pointers after each copy operation, and repeats this process 4 times.
- `process_encoded_data`: Processes encoded data by reading word values, performing conditional operations based on specific word values, and copying data from a lookup table to output locations.
- `initialize_table_values`: Initializes a table of 32 short integers with multiplication results of a counter value and a global variable, incrementing the counter by 2 each iteration.
- `initialize_globals`: Initializes multiple global variables to zero or specific values, copies data between memory locations, and sets up various hardware register configurations.
- `clear_memory_block`: Clears a memory block by writing zero values to 10 consecutive long words pointed to by a register, repeating this process 10 times.
- `wait_vertical_sync`: Waits for vertical blanking periods by polling a hardware register until specific bits indicate the start and end of vertical blanking intervals.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (movea.w, ori.b) and then falls through into the code that follows, with no return of its own.
- `populate_offset_table`: Populates two tables with calculated short integer values derived from an input value, incrementing an offset by 44 each iteration for a total of 4 iterations.
- `clear_large_buffer`: Clears a large memory buffer by writing zero values to 10 consecutive long words, repeating this process 3401 times.
- `calculate_and_store_values`: Calculates and stores integer values in a table by adding an offset to data from another location, then storing the results with specific offsets applied.
- `copy_and_modify_values`: Copies integer values from one memory location to another, applying specific offset calculations and storing modified results in a table with fixed increments.
- `process_and_update_flags`: Initializes a memory area with zeros, compares values from memory locations, and updates flags in a table based on comparison results.
- `decode_and_update_data`: Decodes data by reading word values, performing arithmetic operations with lookup table values, and updating memory locations based on conditional checks and bit rotation.
- `configure_audio_dma`: Configures audio DMA parameters for a sound channel by writing various register values including length, period, and volume, and manages a global counter that controls which channel is configured next.
- `setup_audio_dma`: Configures audio DMA parameters for a sound channel by writing various register values including length, period, and volume.
- `frag_lea_move_rts`: Short code fragment that loads the address of a fixed memory address into a1, then copies 21320 to the memory at a1 (advancing a1), then copies 6187 to the memory at a1 (advancing a1), then copies 33694 to the memory at a1 (advancing a1), then copies 1264 to the memory at a1 (advancing a1), then copies 36224 to the memory at a1 (advancing a1), then copies 2879 to the memory at a1 (advancing a1), then returns.
- `frag_movea_move_rol_sub_rts`: Short code fragment that copies a fixed memory address to a0, then copies the memory at a0 (advancing a0) to a fixed memory address, then copies the memory at a0 (advancing a0) to d2, then rotates d2 left by 8, then copies d2 to a fixed memory address (2 times in a row), then copies 18 to d2, then subtracts the memory at a0 (advancing a0) from d2, then copies d2 to a fixed memory address, then returns.
- `setup_interrupts`: Configures interrupt handling by setting up hardware registers, waiting for vertical sync periods, enabling interrupts and DMA channels, and managing coprocessor pointers.
- `swap_and_wait`: Exchanges values between global variables and memory locations, waits for a hardware register bit to clear, then enables and requests interrupts.
- `clear_dma_buffers`: Initializes multiple memory buffers by setting their contents to zero, including specific offsets within each buffer.
- `frag_movea_tst_bne`: Short code fragment that copies 80890 to a1, then tests the memory at an offset from a1, then branches if the last result was not zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `start_dma_blit`: Enables DMA blitting by setting up display and bitplane registers, waits for blitter idle, then configures additional blitter parameters based on a comparison result before proceeding with further graphics operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_lea_move_addi_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 91 to d7, then adds 6777 to the memory at a0 (advancing a0), then loops back while counting down d7, then returns.
- `process_input_data`: Reads input bytes from a buffer and processes them through multiple conditional checks against data tables, updating counters and modifying memory locations based on comparison results.
- `update_position_and_input`: Updates position coordinates based on joystick input, clamps values within specified ranges, and handles input state changes including mouse button detection and screen boundary checks.
- `update_projectile_data`: Processes projectile movement and rendering data, updating positions and handling collision detection for projectiles stored in a global table.
- `decrement_table_entries`: Decrements specific byte entries in two memory tables through iterative pointer arithmetic, using a loop counter to control the number of iterations.
- `process_joystick_input`: Reads joystick port 1 data, processes movement directions and button states into signed 16-bit values stored in global variables, then updates a counter and performs interrupt-related operations including setting up coprocessor pointers and clearing interrupt flags.
- `fill_pattern_data`: Processes byte data from a source table and writes 32-bit values to two destination buffers while updating a counter.
- `copy_word_pairs`: Copies 32-bit values from a source table to two separate word-aligned destinations, incrementing pointers after each copy operation.
- `setup_blitter_registers`: Initializes multiple blitter hardware registers with specific values and calculates a difference between two global variables to store in another register.
- `execute_blit_operations`: Processes a series of blitter operations stored in memory, updating hardware registers for each operation and handling conditional logic based on data values.
- `update_coordinates`: Calculates and stores coordinate values based on input data and performs bit manipulation operations on a counter before returning.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs one instruction (movea.l) and then falls through into the code that follows, with no return of its own.
- `process_command_sequence`: Reads command data from a sequence, updates global variables based on command values, and modifies memory locations according to conditional logic.
- `process_input_byte`: Reads bytes from a buffer, applies conditional logic based on marker values, and updates memory locations with computed results.
- `modify_and_store_values`: Processes byte values through subtraction and addition operations, stores modified values in arrays, and performs bit flag manipulations.
- `frag_divu_ori`: Short code fragment that divides d2 by the memory at a0 (advancing a0) (unsigned), then merges 224 into d1, then merges 156 into d1, then merges 216 into d1, then merges 28 into d1, then merges 100 into d1, then merges 172 into d1, then merges 244 into d1, then merges 252 into d1, then merges 208 into d1.
- `frag_move_andi_lsr_rts`: Short code fragment that copies a fixed memory address to d0, then masks d0 with an address, then shifts d0 right by 1, then shifts d0 right by 7, then returns.
- `copy_byte_pairs`: Copies 16 bytes from one memory location to another using a loop that decrements a word register and advances pointers.
- `initialize_word_table`: Writes different word values to a memory table based on conditional checks of a global variable, then performs arithmetic operations on values in another table.
- `copy_word_pairs_2`: Copies word values from one memory location to another, performing arithmetic operations on addresses and storing results in sequential memory locations.
- `update_pointer_and_call`: Updates a pointer value based on conditional checks, modifies data at the pointer location, and calls another routine with updated parameters.
- `compute_and_store_words`: Performs division operations on input values, updates bit flags in a table, computes word values from multiple sources, and stores results in sequential memory locations.
- `handle_frame_sync`: Synchronizes frame updates with vertical blanking intervals by polling video position registers and managing interrupt flags.
- `frag_movem_jsr_move_rte`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then copies 32 to a fixed memory address, then returns from the exception.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_graphics_registers`: Sets multiple graphics hardware registers to specific values, performs memory lookups and updates, waits for vertical blanking intervals, and handles joystick input during a loop.
- `fill_table_entries`: Writes word values to two memory locations in a loop, incrementing a counter and pointer addresses each iteration.
- `update_status_flags`: Waits for a specific bit in memory to become clear, then writes multiple word and long values to fixed offsets within a structure pointed to by a register.
- `initialize_coprocessor`: Sets up coprocessor registers and enters an infinite loop that manages memory allocation and interrupt handling while continuously updating video register values.
