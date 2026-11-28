# Game 4717 (Pipeline (Softworx)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04717). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903984 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 68 raw-word lines |
| Functions named and described | pass, 109 of 109 named (0 author symbols), 109 of 109 described (29 by rule, 68 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 426 of 426 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 355556 bytes, 80 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3180 of 3251 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1184 |
| code | 12170 |
| data | 320169 |
| padding | 2141 |
| unused | 568320 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| echo | 2 | 117 | 2 |
| Pipeline (Softworx).adf:ORACLE | 2 | 83 | 3 |
| Pipeline (Softworx).adf:pipeline | 1 | 62 | 6 |
| Pipeline (Softworx).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 431 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| Pipeline (Softworx).adf:ORACLE#unpacked | 1 | 416 | 24 |
| Pipeline (Softworx).adf:bem (raw code file, load address unknown) | 1 | 2532 | 56 |

## Cases handled

- runtime segment an address: 304 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Pipeline (Softworx)/Pipeline (Softworx).adf:ORACLE: crunched executable unpacked with ancient scan (ByteKiller 1.3 (xfd), 24856 -> 76928 bytes), 16 of 16 windows confirmed in reference RAM, loaded at an address: recovered as u7
- Pipeline (Softworx)/Pipeline (Softworx).adf:bem: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 4 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 branch displacement, 1 immediate+address, 1 jump target, 1 address
- Pipeline (Softworx)/Pipeline (Softworx).adf:ORACLE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 355556 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_graphics_system` (bem (raw code file, load address unknown)): Initializes graphics hardware registers, configures display modes, sets up interrupt handling, and manages color palette values while processing input conditions. Kind: disk hardware check. Pass path: The routine executes normally and returns a value in sVar2, with no branching to sabotage or error paths. The presence of ADKCON and CIAB_PRB in the evidence suggests hardware-level checks are involved..

## Functions


### echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.

### Pipeline (Softworx).adf:ORACLE

- `intro_bytekiller_depack`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Pipeline (Softworx).adf:pipeline

- `initialize_system_resources`: Initializes system resources by allocating memory, opening libraries, setting up hardware registers, and copying initialization data to specific memory locations.
- `initialize_dma_transfer`: Sets up a direct memory access transfer by loading a source address and copying data from a table to a destination buffer.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs three instructions (ori.b, ori.b, ori.b) and then falls through into the code that follows, with no return of its own.
- `frag_bhi_blt`: Short code fragment that branches if the comparison was higher (unsigned), then branches if the comparison was less.

### Pipeline (Softworx).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_memory_block`: Copies data from one memory location to another in a loop, updating values based on a counter and incrementing pointers with each iteration.
- `update_structure_fields_2`: Reads a global variable and conditionally modifies fields in a table, then updates the global variable with a new value before returning.
- `update_structure_fields`: Decrements and increments specific fields in a data structure through a loop, then checks a condition and sets a global variable when the condition is met.
- `copy_and_modify_data`: Copies data between memory locations with modifications, using two nested loops to process multiple entries in a structure and updating fields based on index values.
- `update_structure_values`: Updates specific fields in a data structure by copying values from adjacent entries and performing arithmetic operations on them in a loop.
- `initialize_graphics`: Sets up graphics hardware registers with specific values, configures DMA and interrupt settings, and calls multiple routines to initialize audio and display systems.
- `copy_byte_data`: Copies bytes from one memory location to multiple offsets in another location using an index table, processing data in a loop until reaching the end of the source data.
- `handle_audio_state`: Checks specific bits in a hardware register, modifies global counters based on conditions, updates audio channel states by calling other routines, and performs arithmetic operations on data in a loop before returning.
- `initialize_audio_volumes`: Initializes audio volume registers to zero for four sound channels and clears multiple global variables while processing a data buffer with looped comparisons and calculations.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `frag_bsr_jmp`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_audio_channels`: Updates audio channel states based on a global counter and handles audio register modifications for four channels.
- `handle_input_events`: Processes input events based on different conditions, updates values in memory locations, and modifies hardware registers including CIAA_PRA in response to input data.
- `compare_a6_against_global_variable`: Compares the address in a6 against three different global variable addresses in succession and returns immediately upon finding a match.
- `update_dma_and_render_object`: Loads object data from memory tables, performs arithmetic operations to update rendering parameters, and writes updated values to DMA control registers while managing conditional rendering states through multiple branches.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_payload_to_low_memory_and_trap`: Raises the interrupt mask, copies longwords from a source to low memory while d7 remains nonnegative, stores a vector, and invokes trap zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_then_jump_low`: Clears d2 and d3, loads a6 with a library base, calls its negative vector, then jumps to a low-memory address.

### Pipeline (Softworx).adf:ORACLE#unpacked

- `initialize_audio_channels`: Sets up audio channel parameters by loading data from memory locations and performing arithmetic operations on word-sized values before storing results back to memory.
- `process_input_state`: Checks if a global variable equals -1, and if so, calls another routine; otherwise it updates values in a structure by incrementing and decrementing word-sized data at specific offsets before looping through 120 iterations.
- `update_timer_values`: Updates timer-related values in a structure by decrementing and incrementing specific word offsets, then performs a loop that iterates 120 times while modifying data at various memory locations.
- `initialize_dma_channels`: Sets up DMA channel configurations by copying data from memory locations, performing arithmetic operations on long values, and calling several library routines to manage system resources including interrupts and hardware registers.
- `copy_pixel_data`: Copies byte-sized pixel data from one memory location to multiple offsets in another memory area using an index table, then increments the destination pointer and repeats this process 8 times.
- `configure_audio_registers`: Initializes audio register values by loading a base address, reading byte values from memory, comparing them with a counter, and updating audio volume registers in a loop of 128 iterations.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address (4 times in a row), then copies 15 to a fixed memory address, then returns.
- `frag_bsr_jmp_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_audio_pointers`: Increments a global counter and branches to another routine when the counter reaches a specific value.
- `dispatch_rendering_action`: Reads a byte value from memory, masks it with 15, tests if the result is zero, and branches to different routines based on comparison against constants 1, 2, 12, 14, and 15.
- `select_rendering_mode`: Compares a global variable's value against constants 1 through 5 and branches to different code sections based on the comparison results, otherwise it returns immediately.
- `frag_clr_move_lsr_bra`: Short code fragment that clears d0, then copies the memory at an offset from a6 to d0, then shifts d0 right by 4, then jumps to another point in the code.
- `frag_clr_move_andi_bra`: Short code fragment that clears d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `frag_lsl_clr_move_lea_cmp_beq_addq_bra`: Short code fragment that shifts d0 left by 1, then clears d1, then copies the memory at an offset from a6 to d1, then loads the address of a fixed memory address into a0, then copies an indexed entry from a0 to d2, then compares d1 with the memory at a0, then branches if the last result was zero, then adds 2 to a0, then jumps to another point in the code.
- `frag_move_rts`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `frag_bsr_clr_move_andi_add_cmpi_bmi_rts`: Short code fragment that calls another routine, then clears d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then adds d0 to the memory at a4, then compares the memory at a4 with 856, then branches if the last result was negative, then copies 856 to the memory at a4, then copies the memory at a4 to the memory at an offset from a5, then returns.
- `frag_bsr_clr_move_andi_sub_cmpi_bpl_rts`: Short code fragment that calls another routine, then clears d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then subtracts d0 from the memory at a4, then compares the memory at a4 with 113, then branches if the last result was not negative, then copies 113 to the memory at a4, then copies the memory at a4 to the memory at an offset from a5, then returns.
- `frag_move_rts_2`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `frag_move_andi_lsl_or_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then masks d0 with 1, then shifts d0 left by 1, then masks a fixed memory address with 253, then merges d0 into a fixed memory address, then returns.
- `frag_clr_move_andi_rts`: Short code fragment that clears d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then copies d0 to a fixed memory address, then returns.
- `load_rendering_pointers`: Checks the value of a register against specific constants and loads different memory addresses into another register depending on the comparison result, with a fallback to load a default address if none match.
- `initialize_audio_channels_2`: Clears a global variable, loads data from multiple memory locations into registers, and performs bit operations on a DMA control register to enable audio channels.
- `load_rendering_pointers_2`: Loads rendering data into a structure based on a lookup table indexed by the lower 4 bits of a byte, then updates several fields in the structure and performs conditional operations on DMA registers.

### Pipeline (Softworx).adf:bem (raw code file, load address unknown)

- `frag_bra`: Short code fragment that jumps to another point in the code (2 times in a row).
- `read_keyboard_input`: Reads a byte from a keyboard serial register, performs bitwise operations on it, and compares the result with specific values to control program flow.
- `frag_cmp_bne_cmpi_beq_addq_bra`: Short code fragment that compares d0 with 74, then branches if the last result was not zero, then compares a fixed memory address with 72, then branches if the last result was zero, then adds 2 to a fixed memory address, then jumps to another point in the code.
- `update_io_registers`: Sets or clears bits in hardware registers based on input byte values, updates global variables, and performs bit manipulation operations on I/O control registers.
- `check_input_condition`: Tests a global variable's sign and compares it with data at a specific offset, using bit tests to control execution flow and determine conditional branches.
- `transform_memory_values`: Iterates through a memory block, masking each word with a specific bitmask and multiplying the result by two.
- `copy_transform_data`: Copies word-sized data from one memory location to multiple offsets in another location, then performs a large block copy of longwords between two memory regions.
- `setup_graphics_registers`: Saves the current status register, modifies system registers to disable certain interrupts, configures graphics-related hardware registers with specific values, and sets up data structures in memory for rendering operations.
- `update_entry_pointer`: Searches through a table of longword entries to find a specific value, updates memory locations with new data when the value is not found, and handles memory allocation for new entries.
- `frag_lea_moveq_move_ori_dbra_rts`: Short code fragment that loads the address of a location in the code into a0, then loads 3 into d0, then copies the status register to the stack, then merges 1792 into the status register, then copies 3212 to the memory at a0 (advancing a0), then copies 128 to the memory at a0 (advancing a0), then loops back while counting down d0, then copies the top of the stack to the status register, then returns.
- `process_graphics_entry`: Processes graphics data by checking conditions based on hardware register values, performs arithmetic operations on data pointers, and updates graphics buffers with computed results.
- `compute_and_store_coordinates`: Performs arithmetic computations on two 16-bit values using addition with carry, swaps byte order of intermediate results, and stores the final computed values to specific memory locations.
- `process_pixel_data`: Processes pixel data using bitwise operations on word-sized values, applying rotation and masking based on input parameters.
- `fill_memory_with_register`: Fills a block of memory with the contents of a register using a pointer.
- `process_data_chunk`: Processes a data chunk by applying bitwise operations and stores results at specific offsets in a buffer, iterating through 8 iterations.
- `apply_bitwise_transform`: Applies a bitwise transformation to data using XOR and AND operations, storing the results at specific memory locations with byte-level precision.
- `decode_incremental`: Decodes incremental data by performing addition operations on bytes from a buffer, updating carry flags and storing results back to the buffer.
- `decode_decremental`: Decodes decremental data by performing subtraction operations on bytes from a buffer, handling borrow flags and storing results back to the buffer.
- `copy_transform_data_2`: Copies transformed data from a source table to a destination buffer through a bitwise operation, repeating the process eight times while incrementing the destination pointer after each iteration.
- `process_bcd_values`: Decrements a global counter, and if it becomes negative, increments it by 5, then processes byte values using BCD subtraction and bitwise operations, performs comparisons with constants, and calls other routines based on the results.
- `extract_nibbles_and_transform`: Extracts four-bit nibbles from a global word value, applies a bitwise transformation routine to each, and stores results at specific memory locations.
- `transform_and_store_data`: Transforms data using division and bitwise operations, then applies a bitwise transformation to store results at specific memory locations.
- `transform_and_store_data_2`: Transforms data using division and bitwise operations, then applies a bitwise transformation to store results at specific memory locations.
- `transform_data_blocks`: Processes data blocks using bitwise operations with a global mask, storing transformed values at specific memory offsets.
- `transform_data_block`: Processes a block of data by applying bitwise transformations using XOR and AND operations, storing results at specific memory locations with byte-level precision.
- `decode_stream`: Reads bytes from a buffer until encountering a null terminator, calling another routine for each non-null byte, and advances the buffer pointer through each iteration.
- `process_coordinate_data`: Reads a byte from a memory location pointed to by a register, increments a counter in another location, and performs division and comparison operations on the byte value.
- `generate_pattern_sequence`: Generates a sequence of pattern values by applying bit manipulation and conditional addition operations, storing results in two separate buffers.
- `modify_pattern_values`: Modifies pattern values using bitwise operations and conditional additions, storing updated values in two separate buffers while handling overflow conditions.
- `copy_pattern_buffer`: Copies data from one memory buffer to another using a loop that iterates a specific number of times, with each iteration moving a word from the source to the destination buffer.
- `loop_copy_and_blend`: Processes pixel data by copying bytes from a source buffer through a table lookup, blending with destination pixels, and repeating four times.
- `copy_and_blend_pixels`: Copies pixel data through table lookups, blends source pixels with destination pixels using bitwise operations, and repeats this process four times with different parameters.
- `update_data_structure`: Copies data between memory locations in a loop, updates global counters, and calls another routine when the counters reach specific values, performing operations on data at different offsets in memory.
- `copy_pipeline_data`: Copies structured data from a source table to a destination buffer, processing 26 records with fixed offsets and sizes.
- `frag_ori`: Short code fragment that merges 180 into d0, then merges 204 into d0, then merges 228 into d0, then merges 252 into d0.
- `frag_ori_2`: Short code fragment that merges 20 into d0, then merges 44 into d0, then merges 68 into d0, then merges 92 into d0.
- `select_and_copy_data`: Uses a byte value to index into a table of pointers, loads word values from memory locations, and calls another routine with these values when a global flag is set, performing similar operations twice.
- `copy_structure_data`: Copies structure data from one location to another by moving multiple data elements including long words and words with specific offsets.
- `frag_tst_beq_lea_bsr_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then loads the address of a fixed memory address into a0, then calls another routine, then loads the address of a fixed memory address into a0, then calls another routine, then returns.
- `initialize_graphics_registers`: Sets up multiple graphics hardware registers with specific values, configures display modes, and manages color palette settings while processing coordinate data and performing arithmetic operations on memory locations.
- `frag_ori_3`: Short code fragment that merges 116 into d0, then merges 100 into d0, then merges 84 into d0, then merges 68 into d0.
- `frag_ori_4`: Short code fragment that merges 52 into d0, then merges 36 into d0, then merges 20 into d0, then merges 4 into d0.
- `validate_character_context`: Validates character context by comparing input values against boundary conditions in a lookup table, returning zero or negative one based on comparison results.
- `frag_movem_mulu_add_lea_move_addi`: Short code fragment that saves registers on the stack, then multiplies d1 by 9 (unsigned), then adds d1 to d0, then loads the address of a fixed memory address into a2, then copies d2 to an indexed entry from a2, then restores saved registers from the stack, then multiplies d0 by 25 (unsigned), then adds 48 to d0, then multiplies d1 by 25 (unsigned), then adds 22 to d1.
- `blend_pixel_data`: Processes pixel data by blending source values with destination pixels using bitwise operations and shift amounts derived from input parameters, writing results to specific memory offsets.
- `update_pipeline_state`: Updates pipeline state variables based on input byte values, modifying counters and triggering data processing routines.
- `update_pipeline_counters`: Compares byte values with stored constants, decrements counters when they reach zero, updates pipeline state variables, and calls other routines to process data blocks.
- `frag_ori_5`: Short code fragment that merges 148 into d0, then merges 132 into d0, then merges 76 into d0, then merges 236 into d0, then merges 156 into d0, then merges 16 into d0, then merges 218 into d0.
- `update_coordinates`: Decrements a counter at offset 6 of a structure, and if it reaches zero, copies the value at offset 0 to offset 6, then performs arithmetic operations on values at offsets 2 and 4, updates a counter at offset 8, and calls another routine when that counter reaches zero.
- `setup_graphics_system`: Initializes graphics hardware registers, configures display modes, sets up interrupt handling, and manages color palette values while processing input conditions.
- `manage_data_buffer`: Processes a global word value by masking it, performs conditional arithmetic operations based on its value, and calls other routines to handle data transformation and display.
- `process_coordinate_data_2`: Multiplies input values by 25, adds constants, and uses these results as parameters for a loop that processes graphics data through multiple routine calls.
- `process_input_byte`: Reads a byte from memory, clears its high bit, and performs conditional arithmetic operations based on comparisons with specific values, updating global counters and calling other routines.
- `update_pipeline_state_2`: Processes input data to update pipeline coordinates and state variables based on conditional logic, then calls another routine to handle graphics updates.
- `validate_and_compare`: Performs a loop that repeatedly adds and compares values, returning immediately when the comparison fails, otherwise continuing through the loop until completion.
- `compare_and_fetch_value`: Compares an input byte with data from a table and fetches a specific value if the comparison matches, otherwise returns zero.
