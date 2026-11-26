# Game 3473 (Dungeons, Amethysts, Alchemists 'n' Everythin'): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03473). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (910320 top-level bytes in 14 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 69 of 69 named (0 author symbols), 69 of 69 described (17 by rule, 49 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1239 of 1239 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 32768 bytes, 8 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3559 of 3559 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 494 |
| code | 11908 |
| data | 301721 |
| padding | 2277 |
| unused | 593920 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dungeons, Amethysts, Alchemists 'n' Everythin'.adf:FUCKTHISSHIT! | 1 | 99 | 3 |
| Type | 2 | 188 | 2 |
| LoadDAA | 1 | 290 | 8 |
| Wait | 2 | 392 | 12 |
| Disk-Validator | 1 | 386 | 1 |
| Dungeons, Amethysts, Alchemists 'n' Everythin'.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 175 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1910 | 25 |
| Dungeons, Amethysts, Alchemists 'n' Everythin'.adf:FUCKTHISSHIT!#unpacked | 1 | 175 | 8 |

## Cases handled

- Dungeons, Amethysts, Alchemists 'n' Everythin'/Dungeons, Amethysts, Alchemists 'n' Everythin'.adf:DAA.prg: high-entropy data file (7.95 bits/byte), classified as packed data
- runtime segment an address: 164 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 708 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Dungeons, Amethysts, Alchemists 'n' Everythin'/Dungeons, Amethysts, Alchemists 'n' Everythin'.adf:FUCKTHISSHIT!: crunched executable unpacked with ancient scan (TetraPack 2.2 (xfd), 4456 -> 32768 bytes), 5 of 5 windows confirmed in reference RAM, loaded at an address: recovered as u9
- Dungeons, Amethysts, Alchemists 'n' Everythin'/Dungeons, Amethysts, Alchemists 'n' Everythin'.adf:c/LoadDAA: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 32768 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Dungeons, Amethysts, Alchemists 'n' Everythin'.adf:FUCKTHISSHIT!

- `process_bit_shifts_4`: Processes bit-shifted data values by reading from memory, storing byte components to a target location, and performing logical right rotations on data.
- `process_shifted_values`: Processes bit-shifted data values by reading from memory, storing byte components to a target location, and performing logical right rotation operations on the data.
- `process_bit_shifts`: Processes bit shifts and recursive calls based on conditional checks, performing logical operations and looping through a counter.

### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.

### LoadDAA

- `initialize_game_subsystems`: Initializes core game subsystems by allocating memory for graphics and data structures, opening essential libraries, and setting up global pointers.
- `process_bit_shifts_5`: Processes conditional bit operations and table lookups using indexed access, performing recursive calls while decrementing counters.
- `load_level_data`: Reads and decodes 16-bit pixel values from a compressed data stream into a graphics buffer, then copies a block of data from one memory location to another using a nested loop structure.
- `update_pointer_data`: Updates pointer-based data by copying bytes from one location to another, with conditional checks and loop iterations.
- `iterate_with_conditions_2`: Executes conditional loops with bit operations, processes table lookups using indexed access, and performs recursive calls while decrementing counter values.
- `frag_lsl_bne_move_roxl_rts`: Short code fragment that shifts d3 left by 1, then branches if the last result was not zero, then copies the memory at a0 (advancing a0) to d3, then rotates d3 left through the extend flag by 1, then returns.
- `iterate_with_conditions`: Iterates through a loop with conditional branches, performing bit operations and recursive calls while decrementing counters.
- `frag_move_movea_clr_dbra_rts`: Short code fragment that copies 9999 to d0, then copies a location in the code to a0, then clears the memory at a0 (advancing a0), then loops back while counting down d0, then returns.

### Wait

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, writes a header with that value, fills a buffer with sequential words, and sets up a stack frame for a subsequent call.
- `frag_move_bra_2`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `initialize_character_data`: Initializes multiple fields in a character data structure, including counters and flags, then performs conditional operations based on data from tables.
- `update_character_data`: Reads character data from a table using indexed offsets, updates multiple fields in a structure, and performs conditional operations based on character values.
- `frag_move_lsl_tst_beq_bra`: Short code fragment that copies the memory at a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was zero, then copies the memory at an offset from a1 to the memory at an offset from a1, then jumps to another point in the code.
- `update_position_components_2`: Calculates position values by performing arithmetic operations on memory data, stores intermediate results to structure offsets, and handles conditional execution paths.
- `validate_and_store_digit`: Reads a byte value from a table using an index, checks if it's a decimal digit, and stores the converted value in a global variable.
- `validate_digit_range`: Checks if a character from a buffer falls within the decimal digit range 0-9, and sets a flag based on the result.
- `update_position_components`: Reads byte values from a table using an index, adjusts them by subtracting 48, stores the results in global variables, and performs conditional jumps based on comparison with constants 24 and 60.
- `process_game_loop`: Manages game loop state by updating counters and performing conditional jumps, with calls to hardware or system routines for input/output operations.
- `frag_moveq_add_bra`: Short code fragment that loads 1 into d1, then adds the memory at an offset from a1 to d1, then jumps to another point in the code.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.

### Disk-Validator

- `initialize_disk_validator`: Sets up disk validation parameters including buffer pointers and counters, initializes data structures, and prepares for disk access operations.

### Dungeons, Amethysts, Alchemists 'n' Everythin'.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_and_restore_registers`: Saves all general purpose and address registers to the stack, sets up a program counter relative address as a base pointer, stores it to a fixed memory location, then executes a system trap before restoring all registers.
- `initialize_hardware`: Sets up custom hardware registers including display and DMA control, saves current processor state, and configures interrupt handling before entering a main loop.
- `process_input_buffer`: Reads input data from a buffer and updates state variables based on the read values, then writes processed data to memory locations.
- `initialize_rendering_data`: Sets up rendering-related pointers and initializes loop counters for processing graphical elements in a grid pattern.
- `frag_subi_beq_asl_lea_adda_movea_bsr_mo`: Short code fragment that subtracts 32 from d0, then branches if the last result was zero, then shifts d0 left by 3, then loads the address of the memory at an offset from a6 into a1, then adds d0 to a1, then copies the memory at an offset from a6 to a0, then calls another routine, then copies a0 to the memory at an offset from a6, then returns.
- `frag_move_addq_rts`: Short code fragment that copies the memory at a1 (advancing a1) to the memory at an offset from a0 (8 times in a row), then adds 1 to a0, then returns.
- `toggle_keyboard_clock`: Reads the keyboard serial register, sets bit 6 of the keyboard control register, waits for a delay loop, then clears bit 6 of the keyboard control register.
- `populate_address_table`: Copies memory addresses into a table structure, storing both high and low bytes of each address in sequential locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_moveq_movea_jsr_jmp`: Short code fragment that loads 0 into d2, then loads 0 into d3, then copies 5120 to a6, then calls another routine, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system`: Initializes system memory blocks and global variables, sets up library handles, and configures structure offsets with specific values.
- `initialize_message_buffer`: Initializes a message buffer by copying characters from a source location until a null terminator is found, and sets specific byte values at predetermined offsets in the buffer.
- `initialize_and_process`: Initializes byte values at offsets 2 and 3 of a structure, clears a global variable, and loads a pointer from a table to prepare for further processing.
- `clear_memory_blocks`: Clears multiple memory blocks by writing zero values to specific offsets within a data structure, using nested loops for iteration.
- `update_global_variable`: Updates a global variable based on conditions involving table lookups and comparisons, then performs conditional operations including calls to other routines and writes to memory locations.
- `load_and_decode_data`: Decodes compressed data from a file and copies it into memory locations, then clears memory blocks in a structured pattern.
- `copy_and_compare_bytes`: Copies bytes from one memory location to another while comparing values, performs conditional operations based on byte comparisons, and manages memory pointers during the copy process.
- `process_bit_shifts_3`: Performs bit shifting operations on data values, processes conditional branches based on shift results, and recursively calls itself while decrementing a counter.
- `frag_lsl_bne_move_roxl_rts_2`: Short code fragment that shifts d3 left by 1, then branches if the last result was not zero, then copies the memory at a0 (advancing a0) to d3, then rotates d3 left through the extend flag by 1, then returns.
- `process_bit_shifts_2`: Performs bit shifting operations on data values, processes conditional branches based on shift results, and recursively calls itself while decrementing counters.
- `handle_message_queue`: Processes messages from a queue by checking message types and handling specific conditions, with cleanup operations for different message types.
- `frag_moveq_rts_movea_bra`: Short code fragment that loads 0 into d0, then returns, then copies a fixed memory address to a7, then jumps to another point in the code.
- `frag_move_movea_clr_dbra_rts_2`: Short code fragment that copies 9999 to d0, then copies a location in the code to a0, then clears the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `validate_character_class`: Checks if a character matches specific values and sets a flag based on the comparison results, returning immediately otherwise.
- `check_ascii_range`: Validates whether a value falls within specific ASCII ranges and sets a flag accordingly before returning.
- `convert_to_lowercase`: Converts uppercase ASCII characters to lowercase by adding 32 to their values, copies processed characters to a destination buffer, and handles string termination.
- `compare_string_sequences`: Compares two string sequences character by character until a mismatch is found or end of string is reached, returning 1 for match or 0 for mismatch.
- `parse_string_sequences`: Processes string sequences by comparing characters and handling special control conditions, with multiple code paths for different matching scenarios.
- `compare_strings`: Compares character sequences from two strings, returns match status, and processes special control characters while advancing string pointers.
- `frag_lea_bsr_moveq`: Short code fragment that loads the address of a location in the code into a0, then calls another routine, then loads the address of a location in the code into a0, then loads 10 into d1, then loads 0 into d7.
- `handle_input_characters`: Reads input characters from a buffer and processes them according to specific conditions, updating memory locations and handling various character types including control characters.
- `frag_movem_bsr_move_rts`: Short code fragment that saves registers on the stack, then calls another routine, then copies 10 to d0, then calls another routine, then restores saved registers from the stack, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `parse_text_tokens`: Analyzes text tokens by checking for specific character patterns and handling conditional logic based on token length and content.
- `update_game_data`: Updates game state data structures by copying values from lookup tables to memory locations, handles array indexing with offset calculations, and manages counter increments.

### Dungeons, Amethysts, Alchemists 'n' Everythin'.adf:FUCKTHISSHIT!#unpacked

- `save_and_restore_registers_2`: Saves all general purpose and address registers to the stack, sets up a program counter relative address as a base pointer, stores it to a fixed memory location, then executes a system trap before restoring all registers.
- `initialize_hardware_2`: Sets up custom hardware registers including display and DMA control, saves current processor state, configures interrupt handling, and manages keyboard input processing with multiple initialization steps.
- `initialize_buffer_sequence`: Initializes a buffer sequence by loading data from a table and setting up pointers, then performs a loop of memory operations using shift and mask calculations.
- `populate_grid_data`: Populates grid data by iterating through rows and columns, calling another routine for each cell, and updating memory addresses in a loop.
- `frag_subi_beq_asl_lea_adda_movea_bsr_mo_2`: Short code fragment that subtracts 32 from d0, then branches if the last result was zero, then shifts d0 left by 3, then loads the address of the memory at an offset from a6 into a1, then adds d0 to a1, then copies the memory at an offset from a6 to a0, then calls another routine, then copies a0 to the memory at an offset from a6, then returns.
- `frag_move_addq_rts_2`: Short code fragment that copies the memory at a1 (advancing a1) to the memory at an offset from a0 (8 times in a row), then adds 1 to a0, then returns.
- `poll_keyboard_input`: Reads a byte from the keyboard serial register and writes it to the keyboard control register with bit 6 set, then performs a delay loop before clearing bit 6 of the control register.
- `store_address_pairs`: Stores address pairs in memory by loading addresses from specific offsets, extracting low and high words, and writing them to designated memory locations.
