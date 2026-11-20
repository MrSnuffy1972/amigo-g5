# Game 4003 (Indy Heat): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04003). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906912 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 40 of 40 named (0 author symbols), 40 of 40 described (20 by rule, 13 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 332 of 332 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 73834 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 915 of 915 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 15226 |
| code | 3468 |
| data | 884634 |
| unused | 3584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Indy Heat.adf:bootblock | 1 | 21 | 1 |
| logged disk load from Indy Heat.adf boot-block load | 1 | 94 | 2 |
| logged disk load from Indy Heat.adf boot-block load | 1 | 0 | 0 |
| logged disk load from Indy Heat.adf boot-block load | 1 | 0 | 0 |
| logged disk load from Indy Heat.adf boot-block load | 1 | 53 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 747 | 32 |
| Indy Heat.adf:blocks 22-92 (bytes 11264-47616)#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- Indy Heat/Indy Heat.adf: custom-format (non-DOS) disk, 1751 non-empty blocks: matched against track-loaded segments
- Indy Heat/Indy Heat.adf: the boot block reads 10752 bytes from disk offset 512 to an address (found statically); recovered as a unit
- Indy Heat/Indy Heat.adf: the boot block reads 2048 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- Indy Heat/Indy Heat.adf: the boot block reads 512 bytes from disk offset 893952 to an address (found statically); recovered as a unit
- Indy Heat/Indy Heat.adf: the boot block reads 1024 bytes from disk offset 894464 to an address (found statically); recovered as a unit
- runtime segment an address: 332 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate
- Indy Heat/Indy Heat.adf: 4 logged loads cover 14336 bytes of the image
- Indy Heat/Indy Heat.adf: blocks 22-92 (bytes 11264-47616) hold a IMP: File Imploder stream (36352 -> 73834 bytes)
- Indy Heat/Indy Heat.adf:blocks 22-92 (bytes 11264-47616)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- Indy Heat/Indy Heat.adf: blocks 1742-1745 (bytes 891904-893952) hold a IMP: File Imploder stream (2048 -> 1276 bytes)
- Indy Heat/Indy Heat.adf: blocks 1749-1753 (bytes 895488-898048) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.71)
- Indy Heat/Indy Heat.adf: blocks 1755-1759 (bytes 898560-901120) outside the filesystem hold data, not code (opcode density 0.012, entropy 2.91)
- 1 never-loaded region(s), 73834 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 839168 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Indy Heat.adf:bootblock

- `initialize_dma_transfer`: Configures DMA transfer parameters through a series of memory writes to specific offsets, then invokes a library routine to execute the transfer before proceeding with additional setup operations.

### logged disk load from Indy Heat.adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Indy Heat.adf boot-block load

- `jump_through_pointer`: Loads a 32-bit address from offset 2 of the data at a0 into a0 and jumps to that address.
- `frag_moveq_bra`: Short code fragment that loads 16 into d7, then loads 0 into d0, then jumps to another point in the code.
- `frag_move_movem_bsr_addq_dbra_rts`: Short code fragment that copies the memory at a1 (advancing a1) to d1, then saves registers on the stack, then calls another routine, then restores saved registers from the stack, then adds 1 to d0, then loops back while counting down d7, then returns.
- `call_os_function`: Performs a library call through a function pointer located at offset 12 of the structure in a6, with an offset of -288 from that pointer.
- `copy_and_process_data`: Copies 2048 bytes from a source address to memory location 4096, then processes a series of data structures by updating their fields and calling a library routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_game_state`: Updates global variables and performs audio hardware configuration, including setting timer periods and sample pointers, then copies data from a table to multiple memory locations.
- `frag_move_jmp`: Short code fragment that copies 20037 to a fixed memory address, then jumps to another point in the code.
- `frag_move_rte`: Short code fragment that copies an address to the memory at an offset from a0, then copies an address to the memory at an offset from a0, then copies a0 to the memory at an offset from a7, then returns from the exception.
- `update_position_counters`: Updates four 16-bit counters by adding three to each, then decrements them while checking against a threshold value.
- `increment_and_conditionally_update`: Increments two word-sized counters by two each, checks if one reaches a specific value, and updates a global variable when the condition is met.
- `update_counters_and_copy`: Increments four word-sized counters by two each, checks if one reaches a specific value, and updates a global variable when the condition is met.
- `update_and_calculate_coordinates`: Checks a condition based on a global byte value, updates two word-sized counters, and calculates coordinate offsets by loading values from tables.
- `frag_not_rts`: Short code fragment that inverts a fixed memory address, then returns.
- `initialize_structure_parameters`: Sets a 32-bit value at offset 64 of a structure in a6, copies a pointer to a6 offset 84, clears a 16-bit word at offset 102, and writes 276 and 33248 to offsets 88 and 150 respectively.
- `batch_calculate_coordinate_offsets`: Loads four 16-bit values from a source list into registers and calls another routine to process them into a destination structure pointed to by a1.
- `frag_ext_sub_bmi_cmp_bgt_moveq_bra`: Short code fragment that sign-extends d0, then subtracts d0 from d2, then branches if the last result was negative, then subtracts d1 from d3, then branches if the last result was negative, then compares d3 with d2, then branches if the comparison was greater, then loads 17 into d4, then jumps to another point in the code.
- `frag_exg_moveq_bra`: Short code fragment that exchanges d2 and d3, then loads 1 into d4, then jumps to another point in the code.
- `frag_neg_cmp_bgt_moveq_bra`: Short code fragment that negates d3, then compares d3 with d2, then branches if the comparison was greater, then loads 25 into d4, then jumps to another point in the code.
- `frag_exg_moveq_bra_2`: Short code fragment that exchanges d2 and d3, then loads 5 into d4, then jumps to another point in the code.
- `frag_neg_sub_bmi_cmp_bgt_moveq_bra`: Short code fragment that negates d2, then subtracts d1 from d3, then branches if the last result was negative, then compares d3 with d2, then branches if the comparison was greater, then loads 21 into d4, then jumps to another point in the code.
- `frag_exg_moveq_bra_3`: Short code fragment that exchanges d2 and d3, then loads 9 into d4, then jumps to another point in the code.
- `frag_neg_cmp_bgt_moveq_bra_2`: Short code fragment that negates d3, then compares d3 with d2, then branches if the comparison was greater, then loads 29 into d4, then jumps to another point in the code.
- `calculate_coordinate_offset`: Processes coordinate data from input registers and stores the results into various global variables at fixed offsets.
- `update_bitfield_from_tables`: Updates a global variable by XORing it with a constant, then enters a loop that reads values from four tables to set bits in a bitfield at a memory location pointed to by a2.
- `initialize_audio_and_sample_data`: Sets audio hardware registers to specific period and volume values, configures a DMA-driven structure with sample pointers and metadata, and calls another routine four times to process individual sample data entries.
- `initialize_structure_fields`: Clears multiple fields in a structure pointed to by a0, sets a byte at offset 29 to 63, and populates several other fields using values from a table indexed by a calculated offset.
- `update_dma_and_return`: Modifies a global byte and a counter, calls four routines with different pointer offsets, writes a word to the DMA controller register, and returns via an exception return.
- `update_entity_dma`: Updates DMA-related fields in a structure based on entity data and clears several counters.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `frag_neg_move_bra`: Short code fragment that negates d1, then copies d1 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies d1 to the memory at an offset from a0, then jumps to another point in the code, then copies d1 to the memory at an offset from a0, then jumps to another point in the code, then copies d1 to the memory at an offset from a0, then jumps to another point in the code.
- `update_entity_state_and_counters`: Iterates through a table to find a valid entry, updates a pointer and a counter in a structure, and performs several arithmetic operations on various counters and values within the same structure, including bitwise inversion and clamping values to a maximum of twelve or fifteen.
