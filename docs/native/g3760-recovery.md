# Game 3760 (galaxy '89 (1989)(kingsoft)[cr powerdrive]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03760). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 49 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 96 of 96 named (0 author symbols), 96 of 96 described (31 by rule, 63 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3696 of 3696 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 35672 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8185 of 8185 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 287662 |
| code | 38360 |
| data | 169556 |
| padding | 10342 |
| unused | 395264 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Galaxy '89 (1989)(Kingsoft)[cr Powerdrive].adf:type | 2 | 363 | 12 |
| Galaxy '89 (1989)(Kingsoft)[cr Powerdrive].adf:galaxy89 | 3 | 7664 | 81 |
| Galaxy '89 (1989)(Kingsoft)[cr Powerdrive].adf:bootblock | 1 | 157 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Galaxy '89 (1989)(Kingsoft)[cr Powerdrive].adf:hal1 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- galaxy '89 (1989)(kingsoft)[cr powerdrive]/Galaxy '89 (1989)(Kingsoft)[cr Powerdrive].adf: file QS (header 66268 bytes, blocks hold 63440): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- galaxy '89 (1989)(kingsoft)[cr powerdrive]/Galaxy '89 (1989)(Kingsoft)[cr Powerdrive].adf:hal1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- galaxy '89 (1989)(kingsoft)[cr powerdrive]/Galaxy '89 (1989)(Kingsoft)[cr Powerdrive].adf: blocks 11-16 (bytes 5632-8704) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.10)
- galaxy '89 (1989)(kingsoft)[cr powerdrive]/Galaxy '89 (1989)(Kingsoft)[cr Powerdrive].adf: blocks 22-142 (bytes 11264-73216) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.62)
- galaxy '89 (1989)(kingsoft)[cr powerdrive]/Galaxy '89 (1989)(Kingsoft)[cr Powerdrive].adf: blocks 152-153 (bytes 77824-78848) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.36)
- galaxy '89 (1989)(kingsoft)[cr powerdrive]/Galaxy '89 (1989)(Kingsoft)[cr Powerdrive].adf: blocks 886-886 (bytes 453632-454144) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.35)
- galaxy '89 (1989)(kingsoft)[cr powerdrive]/Galaxy '89 (1989)(Kingsoft)[cr Powerdrive].adf: blocks 995-1000 (bytes 509440-512512) outside the filesystem hold data, not code (opcode density 0.055, entropy 5.22)
- 1 never-loaded region(s), 35672 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_input_buffer` (galaxy89): Processes keyboard input by reading from a hardware register, converting backslash characters to null bytes, and storing the result in a buffer. Kind: manual/code-wheel/colour-card lookup. Pass path: The check passes if the input buffer matches the expected value (e.g., 'ENTER YOUR NAME, HERO !'), and the branch after the call at an address is taken when the check passes..

## Functions


### Galaxy '89 (1989)(Kingsoft)[cr Powerdrive].adf:type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `initialize_object_data`: Initializes object state variables including counters and flags, clears specific memory locations, and sets up data for subsequent processing steps.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_moveq_move_bra`: Short code fragment that loads 255 into d1, then copies d1 to the memory at an offset from a2, then jumps to another point in the code.
- `frag_moveq_move_bra_2`: Short code fragment that loads 255 into d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `update_object_counter`: Increments an object counter and performs conditional operations based on comparison results, updating global variables and calling other routines to handle rendering and stat updates.
- `process_object_action`: Processes object actions by reading from memory tables, performing comparisons, and calling other routines based on conditional branches.
- `process_game_objects`: Updates object counters and performs conditional operations based on comparison results, calling other routines to handle rendering and state management.
- `initialize_and_execute`: Sets a global variable to 1, performs several subroutine calls with fixed parameters, updates memory locations, and conditionally branches based on comparison results.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_move_movea_jsr_jmp`: Short code fragment that copies 232 to d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then jumps to the address in a6.

### Galaxy '89 (1989)(Kingsoft)[cr Powerdrive].adf:galaxy89

- `initialize_game_state`: Initializes global variables and hardware registers for the game's main loop, sets up system libraries and graphics resources, then opens and reads data files to populate memory buffers.
- `initialize_graphics`: Sets up graphics-related flags and registers, configures DMA and audio control registers, and manages a counter that decrements until a specific value is reached.
- `setup_graphics_flags`: Initializes multiple graphics-related flags in a global structure, sets a specific bit pattern, and manages a counter that decrements until zero.
- `frag_move_add_rts`: Short code fragment that copies 54067 to d2, then adds 29014 to d2, then returns.
- `validate_sprite_data`: Performs a series of comparisons on data from memory locations, checking for specific conditions before returning either 1 or -1.
- `frag_moveq_bsr_movea_cmp_bne_adda_bra_c`: Short code fragment that loads 16 into d0, then calls another routine, then copies a2 to a0, then calls another routine, then compares d2 with the memory at a0, then branches if the last result was not zero, then adds 2 to a0, then jumps to another point in the code, then compares the memory at a0 (advancing a0) with an address, then branches if the last result was not zero.
- `frag_move_add_and_lsl_or_rts`: Short code fragment that copies an address to d2, then adds an address to d2, then copies the memory at a0 to d0, then copies the memory at an offset from a0 to d1, then masks d0 with d2, then masks d1 with d2, then shifts d0 left by 1, then merges d1 into d0, then returns.
- `validate_and_process_data`: Clears a destination register, performs a series of comparisons and operations on memory data, and executes nested loops with conditional exits based on bit testing.
- `frag_bsr_move_clr`: Short code fragment that calls another routine, then copies a2 to the memory at an offset from a6 (2 times in a row), then clears the memory at an offset from a6 (2 times in a row), then copies 65535 to the memory at an offset from a6 (2 times in a row), then clears the memory at an offset from a6, then copies 6640 to the memory at an offset from a6, then copies 6656 to the memory at an offset from a6.
- `frag_btst_bne_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `frag_bclr_bra`: Short code fragment that clears bit 2 of the memory at a5, then jumps to another point in the code.
- `frag_bset_bra`: Short code fragment that sets bit 2 of the memory at a5, then jumps to another point in the code.
- `keyboard_input_check`: Shifts and XORs a byte value, stores the result in memory, performs bit testing on a hardware register, and returns different values based on the test outcome.
- `frag_move_btst_beq_bsr_add_bra`: Short code fragment that copies the memory at a3 to d0, then tests bit 4 of d0, then branches if the last result was zero, then calls another routine, then adds 1 to d5, then calls another routine, then jumps to another point in the code.
- `frag_bset_sub_bra`: Short code fragment that sets bit 1 of the memory at a5, then subtracts 1 from d5, then jumps to another point in the code.
- `frag_bclr_add_nop_bset_bra`: Short code fragment that clears bit 1 of the memory at a5, then adds 1 to d5, then clears bit 0 of the memory at a5, then pauses for one no-operation step (2 times in a row), then sets bit 0 of the memory at a5, then jumps to another point in the code.
- `delay_loop`: Pushes and pops a register onto the stack, initializes a counter, executes a decrement loop with conditional branching, and returns from the routine.
- `load_high_scores`: Initializes a global variable, opens a file named 'highscores' for reading, reads data into a buffer, closes the file, processes numeric strings in the buffer by subtracting ASCII '0', and updates a table with the processed values.
- `read_file_section`: Opens a file using a specified mode, reads data into a buffer, stores the read size, and writes the buffer address to another location while performing bit operations on a color register.
- `parse_file_data`: Opens a file, reads data into a buffer, searches for a specific identifier pattern, copies matching data sections to an output buffer, and performs bit operations on a color register when no match is found.
- `load_sprite_data`: Reads graphics data from a file into a global buffer, processes sprite information by extracting and combining bytes into words, then copies the processed data to output locations.
- `copy_buffer_data`: Opens a file, reads data into a global buffer, closes the file, then copies 512 bytes from a predefined memory location to an output buffer using a loop with decrementing counter.
- `process_graphics_data`: Opens a file, reads data into a global buffer, closes the file, then performs multiple word-sized memory operations by copying and combining data from a source location to several destination locations with bit operations.
- `initialize_dma_and_io`: Initializes DMA control and interrupt enable registers with specific values, sets up I/O operations by opening a file, writing data to it, and closing it, then configures system state and updates global function pointers.
- `keyboard_scan`: Clears registers, loads data from a hardware keyboard register, compares input against a lookup table, and returns a value based on matching results.
- `initialize_title_screen`: Calls initialization routines and sets up screen buffer clearing operations with various parameters for different text elements.
- `initialize_hall_of_fame`: Calls initialization routines and sets up screen buffer clearing operations with various parameters for hall of fame text entries.
- `initialize_game_data_2`: Initializes game data structures by copying values from one memory location to another, performing comparisons between longword values, and setting up pointers for further processing.
- `update_input_buffer`: Processes keyboard input by reading from a hardware register, converting backslash characters to null bytes, and storing the result in a buffer.
- `character_render`: Calculates memory addresses for character rendering, clears output buffers, processes character data through multiple loops, and updates memory locations with rendered characters.
- `clear_screen_buffers`: Clears large blocks of memory in two separate buffer areas, initializes a counter, and performs repeated clearing operations across multiple memory locations.
- `copy_render_data`: Loads source and destination addresses from global tables, copies data between memory locations using nested loops, and manages counters for row and column operations.
- `process_character_coordinates`: Processes character coordinate data by reading from memory tables, performing bit shifts and additions, then stores combined results in memory locations.
- `clear_render_rows`: Iterates through a table of addresses, clears specific memory locations in each row, and manages counters to control the number of rows processed.
- `update_score_data`: Processes character data in global variables, updates score-related values, and manages text rendering buffers.
- `update_sprite_state`: Updates sprite state values in a table based on conditions and counter checks, then performs arithmetic operations on table entries and clears specific flags.
- `frag_lea_cmpi_bne_move_rts`: Short code fragment that loads the address of a fixed memory address into a2, then compares an indexed entry from a2 with 1, then branches if the last result was not zero, then copies 1 to the memory at a0, then returns.
- `frag_lea_cmpi_beq_rts`: Short code fragment that loads the address of a fixed memory address into a2, then compares an indexed entry from a2 with 0, then branches if the last result was zero, then compares an indexed entry from a2 with 1, then branches if the last result was zero, then returns.
- `initialize_game_data`: Initializes multiple global variables with specific values based on table lookups and conditional comparisons, then sets up DMA control registers.
- `initialize_global_data`: Sets multiple global variables to specific values based on register contents and table lookups, then returns immediately.
- `manage_sprite_parameters`: Manages sprite parameters by checking conditions, updating counters, and performing arithmetic operations on values from memory tables.
- `update_and_clamp_value`: Updates a global value by adding two other values together, clamps the result to 16 bits using bitwise AND with 15, and adds another value before returning.
- `update_counter_and_copy_2`: Increments a counter in a table and performs conditional resets based on its value, then copies data from one table to another using an index derived from a global variable.
- `find_and_clear_slot`: Searches through two tables for a zero value, sets a global counter when found, clears multiple slots in different tables, and returns the counter value.
- `update_counter_and_copy`: Increments a counter in a table and performs conditional resets based on its value.
- `update_game_state`: Updates multiple global variables based on conditional checks and table lookups, then sets specific values in memory locations.
- `update_two_values`: Reads values from two tables using an index, increments one by 1 and decrements another by 17, then stores the results back to corresponding tables.
- `decrement_and_update`: Decrements a value in a table and branches to another routine when the value becomes zero.
- `setup_sprite_parameters`: Loads address values into registers and performs conditional operations based on a global variable before calling another routine with calculated parameters.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `update_game_objects`: Processes game object states and updates global variables based on conditional checks and table lookups, while managing DMA configuration and sprite parameters.
- `accumulate_and_store`: If a global variable is zero, sets it to one, adds values from two tables, and stores the sum in another table before returning.
- `initialize_sprite_data_2`: Initializes sprite-related data by setting up pointers and values based on input registers, then stores calculated address and bit mask value.
- `load_sprite_addresses`: Loads predefined memory addresses into register pairs and sets up initial values for sprite data management before branching to another routine.
- `frag_move_lea_moveq_dbra_rts`: Short code fragment that copies 32767 to the memory at an offset from a6 (2 times in a row), then loads the address of a fixed memory address into a0, then copies a0 to the stack, then loads the address of a fixed memory address into a1, then loads 18 into d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `frag_lea_move_clr_subq_bne_movea_adda_j`: Short code fragment that loads the address of a fixed memory address into a0, then copies an address to d0, then clears the memory at a0 (advancing a0), then subtracts 1 from d0, then branches if the last result was not zero, then copies an address to a0, then adds an address to a0, then jumps to the address in a0.
- `initialize_sprite_data`: Initializes sprite-related data by setting up pointers and values based on input registers, then stores calculated address and bit mask values into global tables.
- `initialize_data_structure`: Initializes a data structure by processing word values from a table and storing pointers in another table while incrementing offsets and decrementing counters.
- `combine_and_process_values`: Processes four input values through a series of bit shifts, rotates, and logical operations to produce a combined 32-bit result that is returned.
- `configure_blitter_operations`: Configures multiple blitter registers with calculated values including addresses and sizes, performs three blitter operations using a helper routine, then returns.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `frag_movea_move_bsr_rts`: Short code fragment that copies a fixed memory address to a0, then copies 38160 to d0, then calls another routine, then copies a fixed memory address to a0, then copies 38160 to d0, then calls another routine, then returns.
- `setup_blitter_dma`: Sets up blitter DMA parameters by calculating sizes and configuring registers, performs multiple blitter operations with different values, then returns.
- `initialize_and_copy_data`: Initializes a memory block with zero values, performs bitwise OR operations between two data blocks, and copies data from one location to another while clearing intermediate values.
- `initialize_starfield`: Initializes starfield coordinates and color values by calculating offsets from base positions and storing them in global variables.
- `update_coordinates`: Updates coordinate values in memory by subtracting fixed amounts, handling overflow conditions, and writing the updated coordinates to a buffer for rendering.
- `frag_move_rts`: Short code fragment that copies 29118 to a fixed memory address, then returns.
- `process_input_data`: Reads input bytes from a buffer and writes them to two memory locations with offset calculation, handling both positive and negative input values through bit manipulation and conditional branching.
- `format_and_display_value`: Converts a value into hexadecimal digits using a lookup table, stores the digits in reverse order to a buffer, and sets a color register before returning.
- `render_characters`: Calculates memory addresses for character rendering, clears output buffers, processes character data through multiple loops, and updates memory locations with rendered data.
- `frag_cmpi_bne_move_bsr_rte`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was not zero, then compares a fixed memory address with 191, then branches if the last result was not zero, then copies a7 to a fixed memory address, then calls another routine, then copies 2048 to a fixed memory address, then returns from the exception.
- `handle_dma_interrupts`: Checks interrupt status bits and decrements countdown values in global variables, then sets DMA control and interrupt enable registers based on which bits are set.
- `frag_cmp_beq_sub_move_and_rts`: Short code fragment that compares d7 with 0, then branches if the last result was zero, then subtracts 1 from d7, then copies a fixed memory address to d0, then masks d0 with 1, then branches if the last result was zero, then returns.
- `frag_cmp_beq_sub_move_and_bne_rts`: Short code fragment that compares d7 with 0, then branches if the last result was zero, then subtracts 1 from d7, then copies a fixed memory address to d0, then masks d0 with 1, then branches if the last result was not zero, then returns.
- `update_color_palette`: Updates color palette values by processing coordinate components and managing counters for red, green, and blue channels, then writes the final combined color value to a custom hardware register.
- `frag_movem_move_jmp`: Short code fragment that restores saved registers from the stack, then copies 16 to a fixed memory address, then jumps to another point in the code.
- `update_game_object_positions`: Updates positions of game objects by modifying coordinates stored in global variables, with boundary checks and adjustments based on object properties and table lookups.
- `update_joystick_input`: Reads joystick input state and updates a global counter based on button presses, then modifies sprite-related data using calculated values from the counter.
- `validate_and_update`: Validates input values against range conditions and updates multiple memory locations with specific values when the validation succeeds.
- `validate_joystick_input`: Checks joystick input state against predefined bounds and updates related counters and flags when input is within acceptable range.
- `frag_tst_beq_jsr_movea_moveq_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then calls another routine, then copies a fixed memory address to a1, then calls another routine, then loads 0 into d0, then copies a fixed memory address to a7, then returns.

### Galaxy '89 (1989)(Kingsoft)[cr Powerdrive].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `build_copper_gradient_list`: Writes a copper list into memory as two blocks of fifteen entries.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
