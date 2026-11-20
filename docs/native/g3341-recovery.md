# Game 3341 (Deep v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03341). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2052494 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 35 of 35 named (0 author symbols), 35 of 35 described (20 by rule, 12 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 435 of 435 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12392 |
| code | 1384 |
| data | 2038204 |
| padding | 2 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 7 | 3 |
| Disk.1:bootblock | 1 | 145 | 5 |
| logged disk load from Disk.1 boot-block load | 1 | 283 | 27 |

## Cases handled

- Deep_v1.0/Deep_v1.0_hdf/Deep/Disk.1: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- Deep_v1.0/Deep_v1.0.hdf: hard-disk image (AmigaDOS OFS, 2208 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree Deep_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- Deep_v1.0/Deep_v1.0_hdf/Deep/Disk.1: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- Deep_v1.0/Deep_v1.0_hdf/Deep/Disk.1: 1 logged loads cover 1024 bytes of the image
- Deep_v1.0/Deep_v1.0_hdf/Deep/Disk.1: blocks 4-1412 (bytes 2048-723456) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.15)
- Deep_v1.0/Deep_v1.0_hdf/Deep/Disk.1: blocks 1414-1759 (bytes 723968-901120) outside the filesystem hold data, not code (opcode density 0.059, entropy 6.20)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_io_ports` (Disk.1 boot-block load +an address to an address (1024 bytes)): Modifies I/O port bits to set specific values, then reads and processes a word from memory before returning the computed result. Kind: disk hardware check. Pass path: branch taken when cmpi.w #an address, d0 is not equal to zero (bne.b an address).
- `frag_move_or_eori_bsr_lea_subi_rts` (Disk.1 boot-block load +an address to an address (1024 bytes)): Short code fragment that copies 137 to d0, then merges a location in the code into d0, then exclusive-ors 255 into d0, then copies d0 to a fixed memory address, then exclusive-ors 1 into d0, then copies d0 to a fixed memory address, then calls another routine, then loads the address of a location in the code into a0, then subtracts 1 from the memory at a0, then returns. Kind: disk hardware check. Pass path: The routine modifies CIAB_PRB to check disk hardware state, and if successful, continues execution via u2__sub_00DA() which likely proceeds with loading. The 'bra.b an address' and 'bsr.w an address' instructions suggest branching or calling into other .
- `frag_move_or_eori_bsr_lea_addi_rts` (Disk.1 boot-block load +an address to an address (1024 bytes)): Short code fragment that copies 139 to d0, then merges a location in the code into d0, then exclusive-ors 255 into d0, then copies d0 to a fixed memory address, then exclusive-ors 1 into d0, then copies d0 to a fixed memory address, then calls another routine, then loads the address of a location in the code into a0, then adds 1 to the memory at a0, then returns. Kind: disk track check. Pass path: CIAB_PRB writes followed by u2__sub_00DA() call, with d0_00E6 incremented on success.
- `configure_dma_and_disk` (Disk.1 boot-block load +an address to an address (1024 bytes)): Sets up DMA channels and disk registers, performs conditional operations based on comparison results, then calls other routines to handle disk data processing. Kind: disk track sync check. Pass path: the routine proceeds through the loop checking for valid disk sync patterns and data, and only returns successfully if all checks pass (i.e., no early termination due to mismatched sync or data)..
- `process_disk_data` (Disk.1 boot-block load +an address to an address (1024 bytes)): Handles disk data processing by configuring hardware registers, reading status bytes, performing bit operations, and writing processed data to memory locations. Kind: disk track check. Pass path: The routine loads data from disk into memory at an address, and only continues if the checksum (bVar5) matches expectations after processing. A successful check means bVar5 == 0 at the end of the loop, allowing execution to proceed past the fin.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.

### Disk.1:bootblock

- `setup_memory_structure`: Initializes memory structures by clearing locations, setting up pointers, and performing arithmetic operations on data values before calling other initialization routines.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `store_four_word_entry`: Stores two input words from registers d0 and d1 into consecutive 16-bit slots of a buffer pointed to by a2, interleaving two fixed 16-bit constants, then returns the first input word increased by 256.
- `initialize_boot_block`: Sets up boot block data structures by storing fixed words and variables into a buffer, then calls multiple library functions to configure hardware registers and perform initialization routines.
- `call_library_pair_twice`: Calls two library functions at fixed offsets from a base, alternating between them twice.

### logged disk load from Disk.1 boot-block load

- `setup_custom_register`: Configures a custom hardware register with a fixed value, followed by a jump to another routine that continues execution flow.
- `frag_move_lea_dbra_jmp`: Short code fragment that copies 9984 to the status register, then loads the address of a fixed memory address into a0, then loads the address of a location in the code into a1, then copies 255 to d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0, then jumps to another point in the code.
- `frag_clr_move_bra`: Short code fragment that clears a fixed memory address (2 times in a row), then copies 399 to a fixed memory address, then jumps to another point in the code.
- `setup_and_execute`: Sets up stack pointer and several address pointers, writes a value to memory, calls another routine with specific parameters, stores a byte result from that call, and conditionally jumps to a specific location based on the result.
- `combine_alternating_bits`: Processes two word streams using a bit mask of 21845, combining data from source pointers into a destination block in a loop until counter reaches zero.
- `frag_move_andi_beq_rts`: Short code fragment that copies a fixed memory address to d0, then masks d0 with 2, then branches if the last result was zero, then copies 32770 to a fixed memory address, then copies 2 to a fixed memory address, then returns.
- `frag_move_dbra_rts`: Short code fragment that copies 3000 to d0, then loops back while counting down d0, then returns.
- `configure_io_ports`: Modifies I/O port bits to set specific values, then reads and processes a word from memory before returning the computed result.
- `frag_move_or_eori_bsr_lea_subi_rts`: Short code fragment that copies 137 to d0, then merges a location in the code into d0, then exclusive-ors 255 into d0, then copies d0 to a fixed memory address, then exclusive-ors 1 into d0, then copies d0 to a fixed memory address, then calls another routine, then loads the address of a location in the code into a0, then subtracts 1 from the memory at a0, then returns.
- `frag_move_or_eori_bsr_lea_addi_rts`: Short code fragment that copies 139 to d0, then merges a location in the code into d0, then exclusive-ors 255 into d0, then copies d0 to a fixed memory address, then exclusive-ors 1 into d0, then copies d0 to a fixed memory address, then calls another routine, then loads the address of a location in the code into a0, then adds 1 to the memory at a0, then returns.
- `frag_move_andi_beq_bsr_bra`: Short code fragment that copies a fixed memory address to d0, then masks d0 with 16, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `frag_lea_clr_rts`: Short code fragment that loads the address of a location in the code into a0, then clears the memory at a0, then returns.
- `configure_dma_and_disk`: Sets up DMA channels and disk registers, performs conditional operations based on comparison results, then calls other routines to handle disk data processing.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `process_disk_data`: Handles disk data processing by configuring hardware registers, reading status bytes, performing bit operations, and writing processed data to memory locations.
- `frag_move_bpl_subq_and_rts`: Short code fragment that copies the memory at an offset from a3 to d0, then branches if the last result was not negative, then subtracts 1 from d1, then branches if the last result was not negative, then masks d0 with 85, then returns.
- `compare_ascii_strings`: Compares characters from two string buffers, converting lowercase letters to uppercase before comparison, returning a flag when strings match or end marker is found.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `parse_and_initialize`: Processes a string buffer until encountering a colon or null terminator, then initializes global variables based on conditional checks involving memory addresses and counters, before calling other routines to handle disk operations and data processing.
- `calculate_division_results`: Performs division and arithmetic operations on memory values, stores results in global variables, then calls another routine to continue processing.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `read_disk_byte`: Handles disk byte reading by decrementing a counter, checking conditions, incrementing an index, and returning the byte value from memory at calculated offset.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a6, then returns.
- `frag_tst_bne_subq_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then subtracts 1 from an indexed entry from a0, then returns.
- `frag_cmpi_beq_tst_rts`: Short code fragment that compares a fixed memory address with 2, then branches if the last result was zero, then tests an indexed entry from a0, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
