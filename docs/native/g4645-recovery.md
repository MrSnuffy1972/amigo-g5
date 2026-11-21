# Game 4645 (paragliding (1991)(loriciel)(m4)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04645). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 150 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 74 of 74 named (0 author symbols), 74 of 74 described (23 by rule, 35 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 15 of 15 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1843 of 1843 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 298612 |
| code | 5292 |
| data | 552295 |
| padding | 24441 |
| unused | 20480 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 458 | 12 |
| RunBack | 3 | 163 | 2 |
| EndCLI | 2 | 182 | 3 |
| powerpacker.library | 2 | 788 | 51 |
| Paragliding (1991)(Loriciel)(M4).adf:para | 1 | 242 | 5 |
| Paragliding (1991)(Loriciel)(M4).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P7.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (7556 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P21.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (7012 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/PTime.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (6872 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P3.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (8224 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P28.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (11228 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P1.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (23656 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P26.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (10284 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P24.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (8336 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P8.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (9368 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P22.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (9924 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P4.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (12072 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P2.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (7512 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P27.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (10144 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/Pdivers.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (4776 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P17.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (7956 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P25.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (7492 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P15.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (9012 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/LORI.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (2444 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:P.finaux/P23.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (10096 -> 32032 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:FromAtari/GUSCR.raw.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (4680 -> 23040 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:FromAtari/GUSDEP.raw.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (5868 -> 25600 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:tab1/Blocs.raw.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (12048 -> 30720 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:tab1/Fond.raw.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (3088 -> 15360 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:tab1/Sprites2.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (2972 -> 9248 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:tab2/Blocs.raw.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (17540 -> 30720 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:tab2/Fond.raw.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (3004 -> 15360 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:tab2/Sprites2.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (3416 -> 9248 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:tab3/Blocs.raw.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (13640 -> 30720 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:tab3/fond.raw.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (2196 -> 15360 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:tab3/Sprites2.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (3316 -> 9248 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/CAMPC3.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (3964 -> 32000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/PRACTIS3.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (1100 -> 6400 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/COMPET3.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (5292 -> 32000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/BALLADE2.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (4496 -> 32000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/CAMPA3.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (4712 -> 32000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/CAMPC2.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (3860 -> 32000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/PRACTIS2.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (1056 -> 8000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/COMPET2.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (4916 -> 32000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/BALLADE1.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (3712 -> 32000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/CAMPA2.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (4600 -> 32000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/CAMPC1.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (4000 -> 32000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/PRACTIS1.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (956 -> 6400 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/COMPET1.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (4264 -> 32000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/CAMPA1.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (3700 -> 32000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/BALLADE4.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (2652 -> 16000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/CAMPC4.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (2672 -> 16000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/COMPET4.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (4380 -> 32000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/BALLADE3.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (4712 -> 32000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:TABLEAUX/CAMPA4.TAB.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (2688 -> 16000 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:tab4/Blocs.raw.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (8772 -> 30720 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:tab4/fond.raw.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (3960 -> 15360 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:tab4/Sprites2.rawp.pp: PowerPacker Data (xfd)-packed, unpacked with ancient (3352 -> 9248 bytes)
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf:para: Titanics Cruncher 1.2 (xfd) stream found by ancient scan (111560 -> 520220 bytes) but only 0 of 78 sampled windows are in reference RAM: not used
- paragliding (1991)(loriciel)(m4)/Paragliding (1991)(Loriciel)(M4).adf: blocks 1737-1759 (bytes 889344-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.64)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `frag_movem_andi_moveq_lea_clr_jsr_bra` (powerpacker.library): Short code fragment that saves registers on the stack, then masks d1 with 65535, then loads 0 into d3, then loads the address of a location in the code into a6, then clears the memory at a1, then calls another routine, then jumps to another point in the code. Kind: password check. Pass path: tst.l d0; bne.b an address; lea.l an address(a7), a7.

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `initialize_paraglider`: Initializes paraglider data structure fields including position, velocity, and state variables.
- `update_paraglider_data`: Reads a byte from a table using a structured index, stores it in a data structure, then updates multiple fields in that structure including position and velocity data.
- `update_paraglider_position`: Updates paraglider position and velocity data including flags stored in a structure referenced by a1, and writes a byte from a buffer to a memory location calculated from the updated data.
- `update_paraglider_state`: Updates paraglider state data including position, velocity, and flags stored in a structure referenced by a1.
- `frag_moveq_add_bra`: Short code fragment that loads 1 into d1, then adds the memory at an offset from a1 to d1, then jumps to another point in the code.
- `frag_tst_bne_bra`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_move_and_tst_beq_ori_moveq_cmp_blt`: Short code fragment that copies 128 to d1, then masks d1 with the memory at an offset from a1, then tests d1, then branches if the last result was zero, then merges an address into the memory at an offset from a1, then loads 133 into d1, then compares d1 with the memory at an offset from a1, then branches if the comparison was less, then jumps to another point in the code.
- `update_object_fields`: Updates multiple fields in a data structure based on values from another structure, including clearing and setting specific offsets, and performs conditional operations based on values stored at specific offsets.
- `update_object_data`: Updates multiple fields in a data structure based on values from another structure, including clearing and setting specific offsets, performing arithmetic operations, and handling conditional logic based on stored values.
- `initialize_and_process`: Initializes multiple data fields in a structure and performs conditional operations based on values stored at specific offsets.

### RunBack

- `parse_command_line`: Parses command line arguments to extract numeric values and string parameters, updating global state based on parsed data.
- `scan_and_dispatch`: Scans a null-terminated string byte by byte until it reaches the end marker.

### EndCLI

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a subsequent call.
- `frag_move_bra_2`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `update_game_state`: Updates various game state variables including counters and flags, performs memory operations on arrays, and handles conditional logic based on comparison results.

### powerpacker.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `save_state_and_init_library`: As part of the PowerPacker shared library, saves a pointer argument into the state block at offset 36 and records the input in a global.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `frag_moveq_subq_bne_btst_rts`: Short code fragment that loads 0 into d0, then subtracts 1 from the memory at an offset from a6, then branches if the last result was not zero, then tests bit 3 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `frag_movem_movea_tst_beq_bset_moveq_bra`: Short code fragment that saves registers on the stack, then copies a6 to a5, then copies a fixed memory address to a6, then tests the memory at an offset from a5, then branches if the last result was zero, then sets bit 3 of the memory at an offset from a5, then loads 0 into d0, then jumps to another point in the code.
- `finish_and_return_saved_value`: As part of the PowerPacker shared library, reads the value saved at offset 36 of its argument, makes three library calls to release resources, and returns that value.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `frag_movem_move_jsr_rts`: Short code fragment that saves registers on the stack, then copies a2 to d2, then copies a3 to d3, then calls another routine, then restores saved registers from the stack, then returns.
- `decode_data_stream_2`: Processes a data stream by reading bytes from memory, performing arithmetic operations on counters, and copying data between memory locations.
- `frag_lsr_beq_rts`: Short code fragment that shifts d5 right by 1, then branches if the last result was zero, then returns.
- `frag_move_roxr_rts`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (subq.w) and then falls through into the code that follows, with no return of its own.
- `frag_moveq_lsr_beq_roxl_dbra_rts`: Short code fragment that loads 0 into d1, then shifts d5 right by 1, then branches if the last result was zero, then rotates d1 left through the extend flag by 1, then loops back while counting down d0, then returns.
- `frag_move_roxr_bra`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `decode_data_stream`: Processes a data stream by reading bytes from memory, performing arithmetic operations on counters, and copying data between memory locations with conditional branching.
- `frag_bsr_move_addq_dbra_cmpa_bcs_movem`: Short code fragment that calls another routine, then copies d1 to d3, then adds 1 to d2, then copies an indexed entry from a1 to the memory just below a1 (moving a1 down), then loops back while counting down d2, then copies a1 to the memory at a3, then compares a4 with a1, then branches if the carry flag is set, then restores saved registers from the stack, then returns.
- `compute_rotating_word_checksum`: Iterates over a zero-terminated byte string at a0, rotating a 16-bit accumulator in d0 by the low bits of a running value and adding the current byte to it, then returns the accumulator as a 16-bit value in d0.
- `compute_checksum`: As part of the PowerPacker shared library, computes a checksum over a zero-terminated byte stream pointed to by a0.
- `xor_block_with_key`: Computes a longword count from the length in d0, then XORs each longword at a0 with the key in d1 and advances a0 by one longword per pass until the count reaches zero, then returns.
- `frag_movem_andi_moveq_lea_clr_jsr_bra`: Short code fragment that saves registers on the stack, then masks d1 with 65535, then loads 0 into d3, then loads the address of a location in the code into a6, then clears the memory at a1, then calls another routine, then jumps to another point in the code.
- `frag_movem_movea_moveq_jsr_rts`: Short code fragment that saves registers on the stack, then copies a2 to a6, then loads 1 into d3, then calls another routine, then restores saved registers from the stack, then returns.
- `setup_stack_frame`: As library code, this routine disables task switching, reads fields from an input buffer into a stack structure, and then re-enables task switching.
- `frag_moveq_bra`: Short code fragment that loads 1 into d5, then jumps to another point in the code.
- `process_hunk_relocation_2`: Reads a hunk header from a global pointer, performs arithmetic operations on it, and prepares data for a checksum computation.
- `process_hunk_header`: Reads a hunk header from a global pointer, performs a series of arithmetic operations on it, and prepares data for a checksum computation.
- `process_hunk_relocation`: Reads relocation entries from a hunk structure, updates memory pointers based on entry types, and processes data stream operations for executable loading.
- `process_relocation_entry_2`: Reads a relocation entry from a hunk, updates memory pointers and table entries based on the entry type and offset values, then continues processing.
- `process_relocation`: Processes a relocation entry from a hunk, updates memory pointers and table entries based on the entry type and offset values, then continues.
- `process_relocation_entry`: Processes a relocation entry from a hunk, updates memory pointers and table entries based on the entry type and offset values, then continues to the next entry.
- `copy_and_relocate`: Copies data between memory locations while processing relocation information, and performs alignment adjustments on memory pointers before calling a library routine.
- `process_relocation_entry_3`: Reads a relocation entry from a hunk, updates memory pointers and table entries based on the entry type and offset values, then continues processing the next entry.
- `frag_movea_subq_bcs_adda_bra`: Short code fragment that copies a5 to a1, then subtracts 4 from a1, then subtracts 1 from d1, then branches if the carry flag is set, then copies the memory at a1 to a1, then adds a1 to a1 (2 times in a row), then jumps to another point in the code.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 4 to a1, then returns.
- `process_relocation_entry_4`: Reads a relocation entry from a hunk, updates memory pointers and table entries based on the entry type and offset values, then continues processing.
- `copy_and_allocate`: Copies a string from a source pointer to an internal buffer, then allocates memory based on a size value and stores the allocated pointer and size to output locations.
- `copy_string_and_set_flag`: Copies data from a source location to a destination buffer, continues copying until a null terminator is found, and sets a flag at a specific offset in the local data structure.
- `initialize_buffer`: Initializes a memory buffer with data from a source location, sets up pointers to the allocated memory block, and returns a value based on the operation's success or failure.
- `frag_move_lea_moveq_movea_jsr_subq_beq`: Short code fragment that copies d4 to d1, then loads the address of the memory at an offset from a5 into a0, then copies a0 to d2, then loads 2 into d3, then copies the memory at an offset from a4 to a6, then calls another routine, then subtracts 2 from d0, then branches if the last result was zero, then copies the memory at an offset from a5 to d7, then jumps to another point in the code.
- `process_buffer_data`: Allocates memory for a buffer, initializes it with data from a source location, and sets up pointers to the allocated memory block for further use.
- `allocate_and_initialize_buffer_2`: Allocates memory for a buffer, initializes it with data from a source location, and sets up pointers to the allocated memory block for further use.
- `allocate_and_initialize_buffer`: Allocates memory for a buffer, initializes it with data from a source location, and sets up pointers to the allocated memory block for further processing.
- `frag_move_lsr_addq_sub_bra`: Short code fragment that copies the memory at an offset from a5 to d0, then shifts d0 right by 8, then copies d0 to the memory at an offset from a5, then copies d0 to d6, then adds 8 to d6, then subtracts d7 from d5, then jumps to another point in the code.
- `frag_move_cmpi_bne_moveq_bra`: Short code fragment that copies the memory at an offset from a5 to d0, then compares d0 with 20560, then branches if the last result was not zero, then loads 250 into d7, then jumps to another point in the code.
- `validate_and_release_buffer_2`: Validates buffer parameters against expected values, frees allocated memory if validation passes, and returns appropriate status codes based on the validation result.
- `validate_and_release_buffer`: Validates buffer parameters against expected values, frees allocated memory if validation passes, and returns appropriate status codes based on the validation outcome.
- `free_buffer_and_return_value`: Frees a buffer through exec's FreeMem, then returns a value saved in its argument block.
- `initialize_parameters`: Sets up multiple offset values in a local structure based on input parameters and constants, then performs conditional updates to global variables depending on the input value.
- `update_parameters`: Calculates and stores several offset values based on structure fields and constants, then performs conditional updates to global variables before returning.
- `process_message_queue_2`: Handles incoming messages from a port, processes them based on their type, updates a global flag, and performs conditional operations before returning.
- `process_message_queue`: Processes a message queue by waiting for messages, retrieving them, and handling specific conditions until a termination criterion is met.
- `frag_movea_jsr_moveq_move_movem_unlk_rt`: Short code fragment that copies a2 to a0, then copies the memory at an offset from a4 to a6, then calls another routine, then loads 0 into d0, then copies the memory at a3 to d0, then restores saved registers from the stack, then removes the stack frame held in a5, then returns.

### Paragliding (1991)(Loriciel)(M4).adf:para

- `decompress_and_initialize_data_stream`: Allocates memory and opens a library, then processes a compressed data stream by reading values from a source, performing bitwise manipulations, and writing the resulting data into a destination buffer.
- `frag_movem_move_jsr_movea_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to d1, then copies a location in the code to d2, then copies 5120 to d3, then calls another routine, then restores saved registers from the stack, then copies a location in the code to a0, then returns.
- `decompress_data_block`: Decompresses a data block by reading compressed instructions, performing bit operations, and writing decompressed bytes to memory with conditional branching based on counter values.
- `frag_moveq_lsl_bne_bsr_roxl_dbra_rts`: Short code fragment that loads 0 into d2, then shifts d0 left by 1, then branches if the last result was not zero, then calls another routine, then rotates d2 left through the extend flag by 1, then loops back while counting down d1, then returns.
- `frag_move_cmpa_bne_bsr_roxl_rts`: Short code fragment that copies the memory at a0 (advancing a0) to d0, then compares a0 with a location in the code, then branches if the last result was not zero, then calls another routine, then copies 16 to the condition codes, then rotates d0 left through the extend flag by 1, then returns.

### Paragliding (1991)(Loriciel)(M4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
