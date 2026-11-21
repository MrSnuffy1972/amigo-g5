# Game 3572 (exodus 3010 (1992)(demonware)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03572). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1804064 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 55 of 55 named (0 author symbols), 55 of 55 described (24 by rule, 31 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 662 of 662 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 35328 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1292 of 1292 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2566 |
| code | 4258 |
| data | 1516189 |
| padding | 475 |
| unused | 280576 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf:bootblock | 1 | 34 | 2 |
| logged disk load from Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf boot-block load | 1 | 869 | 33 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 172 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 100 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 61 | 7 |
| Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf bytes 1024-3072 (code never loaded by a reference run) | 1 | 0 | 0 |
| Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf bytes 10240-13824 (code never loaded by a reference run) | 1 | 56 | 1 |
| Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf bytes 874496-876544 (code never loaded by a reference run) | 1 | 0 | 0 |
| Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf bytes 723456-751104 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: custom-format (non-DOS) disk, 1390 non-empty blocks: matched against track-loaded segments
- exodus 3010 (1992)(demonware)(disk 2 of 2)/Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf: custom-format (non-DOS) disk, 1578 non-empty blocks: matched against track-loaded segments
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: the boot block reads 6144 bytes from disk offset 852480 to an address (found statically); recovered as a unit
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-rs-m1500-unpk01): 155 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 100 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 34 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: bytes 1024-3072 look like code no run loaded: recovered position-independently (load address unknown)
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: bytes 10240-13824 look like code no run loaded: recovered position-independently (load address unknown)
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: bytes 874496-876544 look like code no run loaded: recovered position-independently (load address unknown)
- exodus 3010 (1992)(demonware)(disk 2 of 2)/Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf: bytes 723456-751104 look like code no run loaded: recovered position-independently (load address unknown)
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: 4 logged loads cover 13824 bytes of the image
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: blocks 28-101 (bytes 14336-52224) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.03)
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: blocks 104-110 (bytes 53248-56832) outside the filesystem hold data, not code (opcode density 0.006, entropy 1.42)
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: blocks 116-137 (bytes 59392-70656) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.69)
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: blocks 140-466 (bytes 71680-239104) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.33)
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: blocks 589-1064 (bytes 301568-545280) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.47)
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: blocks 1259-1446 (bytes 644608-740864) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.36)
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: blocks 1449-1453 (bytes 741888-744448) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.80)
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: blocks 1455-1459 (bytes 744960-747520) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.19)
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: blocks 1462-1509 (bytes 748544-773120) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.76)
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: blocks 1530-1664 (bytes 783360-852480) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.47)
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: blocks 1677-1704 (bytes 858624-872960) outside the filesystem hold data, not code (opcode density 0.059, entropy 5.71)
- exodus 3010 (1992)(demonware)(disk 1 of 2)/Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf: blocks 1712-1759 (bytes 876544-901120) outside the filesystem hold data, not code (opcode density 0.010, entropy 4.83)
- exodus 3010 (1992)(demonware)(disk 2 of 2)/Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf: 1 logged loads cover 27648 bytes of the image
- exodus 3010 (1992)(demonware)(disk 2 of 2)/Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf: blocks 2-594 (bytes 1024-304640) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.11)
- exodus 3010 (1992)(demonware)(disk 2 of 2)/Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf: blocks 597-744 (bytes 305664-381440) outside the filesystem hold data, not code (opcode density 0.012, entropy 2.28)
- exodus 3010 (1992)(demonware)(disk 2 of 2)/Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf: blocks 761-798 (bytes 389632-409088) outside the filesystem hold data, not code (opcode density 0.028, entropy 5.21)
- exodus 3010 (1992)(demonware)(disk 2 of 2)/Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf: blocks 800-911 (bytes 409600-466944) outside the filesystem hold data, not code (opcode density 0.026, entropy 5.90)
- exodus 3010 (1992)(demonware)(disk 2 of 2)/Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf: blocks 957-981 (bytes 489984-502784) outside the filesystem hold data, not code (opcode density 0.033, entropy 4.74)
- exodus 3010 (1992)(demonware)(disk 2 of 2)/Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf: blocks 1029-1173 (bytes 526848-601088) outside the filesystem hold data, not code (opcode density 0.011, entropy 3.52)
- exodus 3010 (1992)(demonware)(disk 2 of 2)/Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf: blocks 1177-1308 (bytes 602624-670208) outside the filesystem hold data, not code (opcode density 0.016, entropy 5.46)
- exodus 3010 (1992)(demonware)(disk 2 of 2)/Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf: blocks 1310-1374 (bytes 670720-704000) outside the filesystem hold data, not code (opcode density 0.045, entropy 5.28)
- exodus 3010 (1992)(demonware)(disk 2 of 2)/Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf: blocks 1487-1505 (bytes 761344-771072) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.47)
- exodus 3010 (1992)(demonware)(disk 2 of 2)/Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf: blocks 1508-1510 (bytes 772096-773632) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.36)
- exodus 3010 (1992)(demonware)(disk 2 of 2)/Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf: blocks 1515-1708 (bytes 775680-875008) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.48)
- exodus 3010 (1992)(demonware)(disk 2 of 2)/Exodus 3010 (1992)(Demonware)(Disk 2 of 2).adf: blocks 1710-1759 (bytes 875520-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.14)
- 4 never-loaded region(s), 35328 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `initialize_dma_transfer`: Sets up a DMA transfer by configuring the copper list pointer and allocating memory for an I/O request structure, then performs the I/O operation with error handling.

### logged disk load from Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf boot-block load

- `initialize_system`: Reads a byte from the CIAA_SDR keyboard register, processes it through bitwise operations, and compares it with a fixed value to determine initialization flow.
- `frag_lea_tst_beq_movea_move_jsr_clr_rts`: Short code fragment that loads the address of a location in the code into a0, then tests the memory at a0, then branches if the last result was zero, then copies a fixed memory address to a6, then copies the memory at a0 to a1, then copies 24576 to d0, then calls another routine, then loads the address of a location in the code into a0, then clears the memory at a0, then returns.
- `set_cia_port_bit`: Sets a specific bit in the CIA port register and calls a routine to handle system initialization or hardware setup.
- `manage_dma_transfer`: Controls DMA transfer operations by configuring I/O request structures, managing memory addresses, and handling completion status flags during data transfers.
- `setup_and_execute`: Prepares data structures by calculating offsets, setting up pointers, and calling routines before returning a conditional result based on execution status.
- `frag_movem_movea_move_clr_jsr_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to a6, then copies a location in the code to a1, then copies 9 to the memory at an offset from a1, then clears the memory at an offset from a1, then calls another routine, then restores saved registers from the stack, then returns.
- `setup_io_request`: Configures an I/O request structure with command, data pointers, and completion status before initiating a DMA operation.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_bra_lea`: Short code fragment that jumps to another point in the code, then loads the address of a location in the code into a4, then jumps to another point in the code, then loads the address of a location in the code into a4, then jumps to another point in the code.
- `allocate_and_process_memory`: Allocates memory blocks based on calculated sizes, processes data through a loop, and manages memory pointers for subsequent operations.
- `process_data_block`: Processes a block of data by reading from a source location, calculating offsets, and writing values to destination locations in a loop.
- `clear_memory_block`: Clears memory bytes starting at a calculated address by decrementing a counter and setting each byte to zero until completion.
- `frag_move_subq_bcc_rts`: Short code fragment that copies d1 to the memory at a1 (advancing a1), then subtracts 1 from d0, then branches if the carry flag is clear, then returns.
- `accumulate_values`: Accumulates values from one memory location to another through a loop, adding corresponding elements together while managing counters.
- `frag_movem_bra`: Short code fragment that saves registers on the stack, then jumps to another point in the code.
- `frag_moveq_bra`: Short code fragment that loads 0 into d7, then jumps to another point in the code.
- `process_hunk_data_loop`: Executes a loop that calls functions repeatedly while managing counters and performing conditional jumps based on comparison results.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_3`: Short code fragment that calls another routine, then jumps to another point in the code.
- `process_hunk_data`: Processes hunk data by calling a subroutine, checking values against specific constants, managing counters, and handling memory allocation and deallocation based on conditional logic.
- `initialize_table_values`: Initializes a table of word values by copying data from a source location and setting multiple entries to the same value.
- `frag_lea_move_clr_rts`: Short code fragment that loads the address of a fixed memory address into a5, then copies a6 to the memory at an offset from a5, then clears the memory at an offset from a5, then returns.
- `initialize_input_system`: Sets up input system components by calling initialization routines, configures I/O request structures with command parameters, clears memory buffers, and manages keyboard input checking with specific conditions.
- `allocate_and_process_buffer`: Allocates memory using the Exec library, initializes a buffer with calculated addresses, and processes data in a loop.
- `calculate_data_length`: Reads two words from a buffer, combines them into a long integer with bit shifts, and stores the result in a global variable.
- `decode_bitstream`: Processes a bitstream to decode data, performing bit shifting operations and conditional subroutine calls based on bit values, while managing register states and memory writes through a loop structure.
- `frag_bsr_eor_move_roxr_rts_moveq_subq_b`: Short code fragment that calls another routine, then exclusive-ors d0 into d5, then copies 16 to the condition codes, then rotates d0 right through the extend flag by 1, then returns, then loads 255 into d0, then copies d0 to a fixed memory address, then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `bit_shift_and_execute`: Performs bit shifting operations on data, executes subroutine calls when conditions are met, and manages register values through a loop structure.
- `validate_and_process_data`: Checks a condition based on a global variable and pointer comparison, then modifies a memory location and calls another routine before returning a value from memory.
- `frag_lea_tst_beq_movea_move_jsr_clr_rts_2`: Short code fragment that loads the address of a location in the code into a6, then tests the memory at a6, then branches if the last result was zero, then copies a fixed memory address to a6, then copies a location in the code to a1, then copies an address to d0, then calls another routine, then loads the address of a location in the code into a6, then clears the memory at a6, then returns.
- `allocate_and_copy_data`: Allocates memory blocks using system calls, copies data between locations, and manages counters while handling completion status flags.
- `free_allocated_memory`: Frees allocated memory blocks by iterating through a list, checking for valid pointers, and calling the memory deallocation function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_movea_jsr_moveq_rts`: Short code fragment that copies a fixed memory address to a6, then calls another routine (3 times in a row), then copies a fixed memory address to a6, then calls another routine, then loads 0 into d0, then returns.
- `frag_move_and_cmp_bne_jsr_tst_bmi_rts`: Short code fragment that copies a fixed memory address to d0, then masks d0 with an address, then compares d0 with 12288, then branches if the last result was not zero, then calls another routine, then tests a fixed memory address, then branches if the last result was negative, then returns.
- `process_global_data`: Processes data from a global table by iterating through entries until a negative value is encountered, calling routines with parameters loaded from the table.
- `process_tile_data`: Processes tile data by reading words from input buffer, applying bit manipulation to extract and transform components, then storing results to output buffers while managing a counter variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_tile_values`: Reads words from a buffer, extracts high, middle, and low nibble components, decrements each component when non-zero, and stores the modified value back to the buffer location.
- `frag_sub_ori`: Short code fragment that subtracts the memory at an offset from a4 from d4, then subtracts 0 from d4, then subtracts the memory at a4 from d4, then merges 162 into d0, then merges 54 into d0, then merges 56 into d0, then merges 54 into d0.
- `frag_ori_sub`: Short code fragment that merges 0 into d0, then subtracts d5 from the memory at a2 (advancing a2), then merges 18 into d0, then merges 74 into d0, then merges 138 into d0, then merges 236 into d0, then merges 60 into d0, then merges 255 into d4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_input_state`: Processes input state by checking hardware registers, managing counters, and executing function pointers while handling data movement and conditional operations.
- `check_input_state`: Checks multiple hardware input registers including joystick buttons, mouse fire status, and keyboard serial data to determine if a specific input condition is met, returning -1 or 1 based on the result.
- `read_vposr_and_call`: Reads a 16-bit value from a hardware register, shifts it right by 8 and 7 bits respectively, tests bit 0 of the result, and if the bit is set, calls another routine with predefined parameters.
- `check_and_update_counter`: Reads a global variable, compares it with a constant value, and updates two counters based on conditional logic.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_swap_move_add_addq_dbra_rts`: Short code fragment that swaps the halves of d0, then copies d0 to the memory at an offset from a0, then swaps the halves of d0, then copies d0 to the memory at an offset from a0, then adds d1 to d0, then adds 8 to a0, then loops back while counting down d2, then returns.
- `frag_move_swap_addq_dbra_rts`: Short code fragment that copies the memory at a1 (advancing a1) to d1, then swaps the halves of d1, then copies d1 to the memory at an offset from a0, then swaps the halves of d1, then copies d1 to the memory at an offset from a0, then adds 8 to a0, then loops back while counting down d0, then returns.
- `initialize_dma_registers`: Waits for DMA to be ready, sets up DMA control registers with specific values, opens a graphics library, configures coprocessor registers with library addresses, and closes the library.
- `frag_move_lea_clr_movea_rts`: Short code fragment that copies a5 to the stack, then loads the address of a fixed memory address into a5, then copies a6 to the memory at an offset from a5, then clears the memory at an offset from a5, then copies the top of the stack to a5, then returns.
- `write_pattern_data`: Writes a specific 16-bit value to multiple word offsets within a structure, first setting even offsets to 192 and then odd offsets to 384.
- `frag_btst_beq_moveq_rts`: Short code fragment that tests bit 6 of a fixed memory address, then branches if the last result was zero, then loads 255 into d0, then returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### Exodus 3010 (1992)(Demonware)(Disk 1 of 2).adf bytes 10240-13824 (code never loaded by a reference run)

- `install_interrupt_handler`: Installs an interrupt handler by calling a system function with a specific interrupt number and vector address, then returns.
