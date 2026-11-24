# Game 4204 (lancaster (1989)(crl)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04204). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 57 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 440 of 440 named (0 author symbols), 440 of 440 described (230 by rule, 206 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1121 of 1121 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 20640 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12142 of 12142 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 670468 |
| code | 51284 |
| data | 162344 |
| padding | 16512 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Lancaster (1989)(CRL).adf:Run | 2 | 687 | 14 |
| Lancaster (1989)(CRL).adf:player | 1 | 713 | 15 |
| Lancaster (1989)(CRL).adf:Wait | 2 | 392 | 12 |
| Lancaster (1989)(CRL).adf:Lancaster | 8 | 10592 | 398 |
| Lancaster (1989)(CRL).adf:bootblock | 1 | 10 | 1 |
| Lancaster (1989)(CRL).adf:3D-ib-c (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- lancaster (1989)(crl)/Lancaster (1989)(CRL).adf:instr/Data2: high-entropy data file (7.75 bits/byte), classified as packed data
- lancaster (1989)(crl)/Lancaster (1989)(CRL).adf:3D-ib-c: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 5 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 4 immediate, 1 jump target
- 1 never-loaded region(s), 20640 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Lancaster (1989)(CRL).adf:Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `initialize_object_data`: Writes multiple values from registers and tables into a structure pointed to by a1, including bytes, integers, and bit operations on a global variable.
- `load_tile_data`: Reads a byte from a table using a calculated offset and stores it in a structure.
- `update_object_tile`: Reads a byte from a memory location using calculated offsets and stores it into another memory location.
- `update_object_data`: Writes structured data to memory locations based on calculated offsets, updates object state variables, and performs conditional operations on object properties before branching to another routine.
- `frag_moveq_add_bra`: Short code fragment that loads 1 into d1, then adds the memory at an offset from a1 to d1, then jumps to another point in the code.
- `frag_tst_bne_bra`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_move_and_tst_beq_ori_moveq_cmp_blt`: Short code fragment that copies 128 to d1, then masks d1 with the memory at an offset from a1, then tests d1, then branches if the last result was zero, then merges an address into the memory at an offset from a1, then loads 133 into d1, then compares d1 with the memory at an offset from a1, then branches if the comparison was less, then jumps to another point in the code.
- `update_graphics_fields`: Updates graphics-related fields in a structure, including clearing counters and setting up various offsets.
- `update_graphics_data`: Updates graphics-related fields in a structure pointed to by a1, including clearing counters and setting up various offsets.
- `initialize_graphics_data`: Initializes multiple graphics-related data fields in a structure pointed to by a1, including clearing counters and setting up various offsets.
- `check_and_modify_flag`: Tests a bit in a global variable, toggles a byte in another global variable, and branches conditionally based on the result of that test.
- `process_game_data`: Reads a value from a table using an index in a register, stores it at multiple offsets in a structure, and updates counters and flags based on comparisons with zero.

### Lancaster (1989)(CRL).adf:player

- `setup_input_handler`: Saves all general-purpose and address registers on the stack, then calls another routine to set up input handling.
- `frag_move_movem_andi_beq_jsr_jmp`: Short code fragment that copies the status register to the stack, then saves registers on the stack, then copies a fixed memory address to d0, then masks d0 with 32, then branches if the last result was zero, then calls another routine, then restores saved registers from the stack, then copies the top of the stack to the status register, then jumps to another point in the code.
- `open_dos_library`: Opens the DOS library using a specific library name string and stores the returned library pointer in a global variable.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_movem_bsr_rts`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `update_player_animation`: Reads a byte from a structure at offset 12, increments it, uses the result as an index to access a table, and copies data from the table to another location.
- `update_player_position`: Reads a byte from a structure at offset 12 of a pointer, increments that byte, uses it as an index to access a memory location, and stores the value at another memory location.
- `clear_audio_volumes_and_status`: Zeroes the volume registers of four audio channels and clears one global value and three RAM status bytes.
- `frag_bsr_clr_rts`: Short code fragment that calls another routine, then clears a fixed memory address (2 times in a row), then returns.
- `update_bit_pattern`: Updates a bit pattern stored in memory by shifting bits right and copying values between arrays.
- `calculate_zero_division_result`: Sets a global variable to zero, copies a value from another global variable, performs a division operation that will result in a divide-by-zero exception, and includes a loop that decrements a register until it reaches zero.
- `copy_animation_data`: Loads a byte from memory at offset 80 relative to a pointer, uses that value as an index into a table, then copies several bytes from the table entry to various offsets relative to the same pointer.
- `setup_audio_registers`: Configures audio hardware registers by reading data from memory tables and writing values to AUD0LCH and AUD0LEN registers.
- `copy_word_to_global`: Reads a word from a memory location calculated using a pointer and an offset, then stores it into a global variable.
- `update_dma_and_audio`: Writes several values to memory locations, modifies a global variable using bitwise operations, and interacts with audio and DMA hardware registers.

### Lancaster (1989)(CRL).adf:Wait

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, writes a header with that value, fills a buffer with sequential words, and sets up a stack frame for a subsequent call.
- `frag_move_bra_2`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `process_wait_data`: Updates wait-related data in a structure based on character table entries, initializes counters, and manages position information.
- `update_character_data`: Processes character data from a table using shift operations and stores results in structure offsets.
- `frag_move_lsl_tst_beq_bra`: Short code fragment that copies the memory at a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was zero, then copies the memory at an offset from a1 to the memory at an offset from a1, then jumps to another point in the code.
- `calculate_movement_data`: Performs bit shifting and arithmetic operations on values from a structure, calls library routines to update position data, and stores intermediate results back into the structure at various offsets.
- `initialize_wait_data`: Initializes wait-related data in a structure by loading values from a table, clearing counters, and setting up position information.
- `validate_ascii_range`: Checks if a character from a buffer falls within the ASCII range of 0-9, and if so, performs a series of operations including updating counters and calling a handler routine.
- `update_position_data`: Reads byte values from a table using indexed addressing, adjusts them by subtracting 48, stores results in structure fields, and performs conditional jumps based on comparisons with constants 24 and 60.
- `initialize_loop_counters`: Sets up loop counters and performs initialization logic for a processing loop.
- `frag_moveq_add_bra_2`: Short code fragment that loads 1 into d1, then adds the memory at an offset from a1 to d1, then jumps to another point in the code.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.

### Lancaster (1989)(CRL).adf:Lancaster

- `initialize_system`: Opens three system libraries and allocates memory blocks, then performs multiple data operations including file handling and memory management.
- `initialize_game_state_3`: Initializes multiple global variables and memory regions with specific values, clears large memory blocks, and sets up hardware registers for game operation.
- `open_system_libraries`: Opens three system libraries using the exec.OpenLibrary function and stores the returned handles in global variables.
- `allocate_memory_block`: Allocates a memory block of specific size using exec.AllocMem, copies initial data from a table, and sets up multiple pointers to different offsets within the allocated block.
- `frag_move_dbra_rts`: Short code fragment that copies 1000 to d7, then copies 2000 to d6, then loops back while counting down d6, then loops back while counting down d7, then returns.
- `frag_movea_bsr_rts`: Short code fragment that copies a fixed memory address to a6, then calls another routine (6 times in a row), then returns.
- `frag_movea_bsr_rts_2`: Short code fragment that copies a fixed memory address to a6, then calls another routine, then returns.
- `frag_lea_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a5, then calls another routine, then loads the address of a fixed memory address into a5, then calls another routine, then returns.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_subroutine_multiple_times`: Calls another routine multiple times with different address parameters and a fixed value.
- `frag_move_bsr_addi_rts`: Short code fragment that copies 800 to d1, then calls another routine, then adds 8000 to d0, then copies d0 to the memory at a5 (advancing a5), then adds 8000 to d0, then copies d0 to the memory at a5 (advancing a5), then adds 8000 to d0, then copies d0 to the memory at a5 (advancing a5), then returns.
- `frag_move_jsr_tst_ble_rts`: Short code fragment that copies 320 to d0, then calls another routine, then tests d0, then branches if the comparison was less or equal, then copies d0 to the memory at a5 (advancing a5), then returns.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `frag_lea_move_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies a1 to the memory at an offset from a0, then copies 200 to the memory at an offset from a1, then copies 320 to the memory at an offset from a1, then returns.
- `setup_memory_pointers`: Sets up memory pointers to specific offsets within allocated memory blocks by loading addresses into registers and storing them at calculated memory locations.
- `frag_movea_lea_jsr_rts`: Short code fragment that copies a fixed memory address to a6, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then calls another routine, then loads the address of a fixed memory address into a1, then calls another routine, then loads the address of a fixed memory address into a1, then calls another routine, then returns.
- `frag_move_lea_bsr_rts`: Short code fragment that copies 5521 to d1, then copies 32000 to d6, then loads the address of a fixed memory address into a5, then calls another routine, then returns.
- `frag_move_lea_bsr_rts_2`: Short code fragment that copies 5569 to d1, then copies 32000 to d6, then loads the address of a fixed memory address into a5, then calls another routine, then returns.
- `process_text_data`: Processes text data by calling a subroutine multiple times with different parameters.
- `frag_move_lea_bsr_rts_3`: Short code fragment that copies 5545 to d1, then copies 32000 to d6, then loads the address of a fixed memory address into a5, then calls another routine, then returns.
- `format_character_value`: Formats a character value by adding ASCII '0' to a word value from memory and storing the result in a global variable.
- `conditional_text_processing`: Checks a condition and if true, processes text data by calling subroutines with different parameters.
- `copy_memory_block`: Copies a large block of memory from one location to another using a loop that processes 8000 words.
- `update_display_digits`: Converts word values to ASCII digits and stores them in global variables, then calls another routine multiple times with different parameters before copying data.
- `handle_joystick_input_2`: Reads joystick data, updates position tracking variables, handles timing for input processing while setting display window registers, and copies data between memory locations.
- `update_digit_display`: Converts a word value to ASCII and stores it in a global variable, then calls another routine twice with different parameters before copying data between memory locations.
- `setup_interrupt_handler`: Enables and disables interrupts by writing to a hardware register, reads from a global variable, writes a character to another global variable, then calls another routine.
- `file_operation_sequence`: Opens a file using a global variable as the filename, reads data into a buffer, closes the file, then performs additional file operations based on a word value from another global variable.
- `frag_move_lea_bsr_rts_4`: Short code fragment that copies 5701 to d1, then copies 32000 to d6, then loads the address of a fixed memory address into a5, then calls another routine, then returns.
- `process_interrupt_handler`: Handles interrupt processing by reading joystick data, updating position tracking variables, and calling another routine to perform additional operations.
- `frag_move_lea_bsr_rts_5`: Short code fragment that copies 5821 to d1, then copies 32000 to d6, then loads the address of a fixed memory address into a5, then calls another routine, then returns.
- `frag_bsr_rts`: Short code fragment that calls another routine (3 times in a row), then returns.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `data_decompression_loop`: Reads data from a file into a buffer, processes the data by copying bytes while skipping specific values, and handles repeated byte sequences using a loop counter.
- `open_read_close_file`: Opens a file with specific parameters, reads data into a buffer, closes the file, then performs another file operation using the same file handle.
- `conditional_file_write`: Checks a global variable and returns immediately if it's zero, otherwise enables interrupts, opens a file, writes data to it, closes the file, then calls another routine.
- `file_write_operation`: Checks a global variable and returns immediately if it's zero, otherwise opens a file, writes data to it, and closes the file without any conditional branching.
- `multiple_data_operations`: Calls another routine multiple times with different memory addresses and word values, then performs additional operations including a final call to another routine with specific parameters.
- `frag_bsr_rts_2`: Short code fragment that calls another routine (3 times in a row), then returns.
- `frag_bsr_rts_3`: Short code fragment that calls another routine (3 times in a row), then returns.
- `call_open_2`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `allocate_and_read_buffer`: Allocates memory using a global variable as the size parameter, stores the allocated address in a structure, reads data from a file into the buffer, then reads additional data using the same buffer.
- `allocate_and_read_data`: Allocates memory using a global variable as the size parameter, stores the allocated address in a structure, then reads data from a file into the buffer using the allocated memory address.
- `frag_movea_move_jsr_tst_beq_rts`: Short code fragment that copies a fixed memory address to a6, then copies 52250 to d0, then copies 2 to d1, then calls another routine, then tests d0, then branches if the last result was zero, then copies d0 to a fixed memory address, then returns.
- `open_read_close`: Opens a file using a global variable and a file size constant, reads data into another global variable with a specific byte count, then closes the file handle stored in a global variable.
- `open_read_close_twice`: Opens a file twice with the same parameters, reads two separate data segments into the same buffer with different offsets and sizes, then closes the file handle twice, storing each file handle in a global variable.
- `manage_audio_dma`: Manages audio DMA operations by checking DMA status, performing multiple subroutine calls, and setting up audio registers with specific values before enabling interrupts.
- `read_dma_status`: Reads the DMA control register and checks bit 1, returning immediately if the bit is clear.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_audio_sequence`: Initializes audio playback by calling multiple subroutines, setting up audio registers with specific values, and configuring DMA control registers to enable audio interrupts.
- `frag_bsr_move_bra`: Short code fragment that calls another routine (3 times in a row), then copies 22 to d0, then calls another routine (3 times in a row), then jumps to another point in the code.
- `frag_bsr_move_bra_2`: Short code fragment that calls another routine (3 times in a row), then copies 14 to d0, then calls another routine (3 times in a row), then jumps to another point in the code.
- `frag_bsr_move_bra_3`: Short code fragment that calls another routine (3 times in a row), then copies 15 to d0, then calls another routine (3 times in a row), then jumps to another point in the code.
- `setup_audio_parameters_2`: Configures audio parameters by repeatedly calling subroutines to set up audio registers with fixed values, performing delay loops, and enabling DMA operations for audio playback.
- `frag_bsr_move_bra_4`: Short code fragment that calls another routine (3 times in a row), then copies 18 to d0, then calls another routine (3 times in a row), then jumps to another point in the code.
- `frag_bsr_move_bra_5`: Short code fragment that calls another routine (3 times in a row), then copies 19 to d0, then calls another routine (3 times in a row), then jumps to another point in the code.
- `frag_bsr_move_bra_6`: Short code fragment that calls another routine (3 times in a row), then copies 17 to d0, then calls another routine (3 times in a row), then jumps to another point in the code.
- `init_audio_dma`: Initializes audio DMA by calling multiple routines, checking a condition with a test instruction, and branching based on the result to either set a specific value or continue processing.
- `frag_bsr_bra`: Short code fragment that calls another routine (7 times in a row), then jumps to another point in the code.
- `frag_bsr_andi_addi_rts`: Short code fragment that calls another routine, then masks d0 with 1, then adds 20 to d0, then calls another routine (2 times in a row), then returns.
- `handle_audio_control`: Handles audio control operations by calling a subroutine, performing bitwise AND operation on a register, and branching based on the result of a comparison with zero.
- `check_dma_status`: Reads the DMA control register and waits while a specific bit is set, then calls another routine and updates memory locations with calculated values.
- `frag_subq_bsr_rts`: Short code fragment that subtracts 1 from d0, then calls another routine, then returns.
- `frag_clr_bsr_rts`: Short code fragment that clears d0, then calls another routine, then returns.
- `frag_move_bsr_rts`: Short code fragment that copies 10 to d0, then calls another routine, then returns.
- `frag_move_bsr_rts_2`: Short code fragment that copies 11 to d0, then calls another routine, then returns.
- `frag_move_clr_dbra_addi_rts`: Short code fragment that copies 127 to d7, then clears the memory at a1 (advancing a1), then loops back while counting down d7, then adds 64 to a fixed memory address, then returns.
- `frag_lea_move_dbra_addi_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 255 to d7, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d7, then adds 128 to a fixed memory address, then returns.
- `copy_and_accumulate`: Copies data from a table based on input value, advances pointers, and accumulates half of the copied value into a global variable while performing byte-by-byte copying.
- `initialize_audio_channels`: Configures audio hardware registers for two sound channels with specific parameters, sets up DMA and interrupt controls, and updates global variables to indicate channel activity.
- `configure_audio_channels`: Configures audio channels by opening files, reading data into buffers, closing files, and setting up audio registers with calculated values while handling DMA operations.
- `frag_movea_move_eor_dbra_rts`: Short code fragment that copies a fixed memory address to a0, then copies a fixed memory address to a1, then copies 2467 to d7, then copies the memory at a0 (advancing a0) to d0, then copies the memory at a1 to d1, then exclusive-ors d1 into d0, then copies the memory at a1 (advancing a1) to d1, then loops back while counting down d7, then returns.
- `frag_movea_jsr_move_rts`: Short code fragment that copies a fixed memory address to a6, then copies the memory at an offset from a6 to a0, then calls another routine (2 times in a row), then copies 4098 to a fixed memory address, then copies 9317 to a fixed memory address, then returns.
- `frag_move_movem_bsr_rte`: Short code fragment that copies the status register to the stack, then saves registers on the stack, then calls another routine (4 times in a row), then restores saved registers from the stack, then copies the top of the stack to the status register, then returns from the exception.
- `frag_move_andi_beq_cmpi_bne_clr_bra`: Short code fragment that copies a fixed memory address to d0, then masks d0 with 128, then branches if the last result was zero, then copies d0 to a fixed memory address, then copies a fixed memory address to d1, then compares d1 with 1, then branches if the last result was not zero, then clears a fixed memory address, then jumps to another point in the code.
- `frag_move_clr_rts`: Short code fragment that copies 1 to a fixed memory address, then copies 128 to a fixed memory address, then clears a fixed memory address, then returns.
- `frag_move_andi_beq_cmpi_bne_clr_bra_2`: Short code fragment that copies a fixed memory address to d0, then masks d0 with 256, then branches if the last result was zero, then copies d0 to a fixed memory address, then copies a fixed memory address to d1, then compares d1 with 1, then branches if the last result was not zero, then clears a fixed memory address, then jumps to another point in the code.
- `frag_move_clr_rts_2`: Short code fragment that copies 2 to a fixed memory address, then copies 256 to a fixed memory address, then clears a fixed memory address, then returns.
- `handle_interrupt_flag_a`: Checks a specific bit in a hardware register, writes the value to another register if set, tests a memory location, and clears a flag if conditions are met.
- `frag_move_clr_rts_3`: Short code fragment that copies 4 to a fixed memory address, then copies 512 to a fixed memory address, then clears a fixed memory address, then returns.
- `handle_interrupt_flag_b`: Checks another bit in a hardware register, writes the value to a register if set, tests a memory location, and clears a flag if conditions are met.
- `frag_move_clr_rts_4`: Short code fragment that copies 8 to a fixed memory address, then copies 1024 to a fixed memory address, then clears a fixed memory address, then returns.
- `setup_audio_parameters`: Calculates audio parameters based on input values, configures multiple audio hardware registers for a sound channel, sets up DMA and interrupt controls, and updates a global variable.
- `configure_audio_channel_0`: Initializes audio channel 0 with specific parameters including waveform data, volume, pitch, and interrupt settings, then updates a global flag to indicate channel activity.
- `configure_audio_channel_0_2`: Sets up audio channel 0 with fixed parameters for waveform data, volume, pitch, and interrupt controls, then updates a global flag to indicate channel activity.
- `configure_audio_channel_1`: Initializes audio channel 1 with specific parameters including waveform data, volume, pitch, and interrupt settings, then updates a global flag to indicate channel activity.
- `configure_audio_channel_1_2`: Sets up audio channel 1 with fixed parameters for waveform data, volume, pitch, and interrupt controls, then updates a global flag to indicate channel activity.
- `setup_audio_channel_2`: Configures audio channel 2 with loop count, sample data pointers, and volume settings while enabling DMA and interrupts.
- `setup_audio_channel_3`: Configures audio channel 3 with loop count, sample data pointers, and volume settings while enabling DMA and interrupts.
- `frag_bsr_rts_4`: Short code fragment that calls another routine (4 times in a row), then returns.
- `clear_dma_interrupt_1`: Checks DMA and interrupt status registers, clears a global variable if conditions are met, then sets interrupt enable and request registers.
- `clear_dma_interrupt_2`: Checks DMA and interrupt status registers, clears a global variable if conditions are met, then sets interrupt enable and request registers.
- `check_interrupts_and_clear`: Reads a hardware register, checks for specific bits, and clears a global variable if the bits are not set.
- `check_interrupt_flags`: Reads DMA and interrupt status registers, and if both conditions are met, clears a global variable and writes specific values to interrupt control registers.
- `frag_jmp`: Short code fragment that jumps to another point in the code.
- `frag_jmp_2`: Short code fragment that jumps to another point in the code.
- `frag_jmp_3`: Short code fragment that jumps to another point in the code.
- `frag_jmp_4`: Short code fragment that jumps to another point in the code.
- `frag_jmp_5`: Short code fragment that jumps to another point in the code.
- `frag_jmp_6`: Short code fragment that jumps to another point in the code.
- `frag_jmp_7`: Short code fragment that jumps to another point in the code.
- `frag_jmp_8`: Short code fragment that jumps to another point in the code.
- `clear_sprite_data`: Clears sprite data by writing zero to sixteen word locations starting at a specific address, then writes a value to a global variable and calls another routine.
- `frag_bsr_movem_move_rts`: Short code fragment that calls another routine, then copies several registers to or from memory (4 times in a row), then copies 6592 to a fixed memory address, then calls another routine, then returns.
- `frag_bsr_movem_move_rts_2`: Short code fragment that calls another routine (2 times in a row), then copies several registers to or from memory (4 times in a row), then copies 6496 to a fixed memory address, then calls another routine (2 times in a row), then returns.
- `frag_move_bsr_rts_3`: Short code fragment that copies 6656 to a fixed memory address, then calls another routine, then returns.
- `process_input_and_update`: Reads keyboard input from a serial register, updates global variables based on key presses, and copies data between memory locations.
- `update_joystick_position`: Reads mouse port movement data and updates position coordinates with boundary checks, then stores the results in global variables.
- `process_joystick_input`: Processes joystick input by reading a global variable and performing conditional calculations to determine which position in a table to update with a value, then calls another routine to handle the update.
- `check_collision_zones`: Performs boundary checks on position coordinates and sets collision flags based on specific coordinate ranges.
- `frag_bsr_tst_bne_rts_move`: Short code fragment that calls another routine, then tests a fixed memory address, then branches if the last result was not zero, then returns, then copies 65535 to a fixed memory address, then returns.
- `setup_display_buffers`: Copies data from source memory locations to display buffer locations, initializes several global variables with specific values, and updates a table of values in a loop before calling other routines.
- `configure_video_settings`: Copies memory contents between buffers, sets up various video registers with fixed values, reads from a global table to determine a value for a register, and calls other routines to complete the configuration.
- `initialize_game_objects`: Determines an object type based on input values, calculates position and size parameters for display registers, copies memory contents between buffers, and sets up video register values before calling other routines.
- `setup_background_pattern`: Calculates a memory address based on input values, initializes memory blocks with specific patterns, copies memory contents between buffers, and configures display registers with fixed values before calling other routines.
- `update_input_counter`: Reads a byte from a hardware register and updates a counter based on its value, with bounds checking and wrapping behavior for input handling.
- `swap_frame_buffers`: Exchanges the contents of two memory buffers by copying 4000 words between them using a loop counter.
- `swap_and_copy_blocks`: Exchanges two memory blocks pointed to by global variables, then copies 2000 32-bit words from one block to the other using a loop.
- `swap_and_copy_blocks_2`: Exchanges two memory blocks pointed to by global variables, then copies 2000 32-bit words from one block to the other.
- `frag_lea_move_dbra_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 259 to d7, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d7, then calls another routine (2 times in a row), then returns.
- `process_pattern_data`: Processes pattern data by calling another routine multiple times with varying parameters and advancing pointers through a loop.
- `loop_draw_pattern`: Iterates a fixed count to draw a pattern, incrementing a counter and calling another routine multiple times per iteration.
- `frag_move_bsr_dbra_rts`: Short code fragment that copies the memory at a3 (advancing a3) to d0, then calls another routine, then loops back while counting down d4, then returns.
- `frag_cmp_beq_addq_dbra_rts`: Short code fragment that compares d5 with a fixed memory address, then branches if the last result was zero, then adds 1 to a3, then adds 1 to a0, then loops back while counting down d4, then returns.
- `setup_copper_list_2`: Copies data from two source buffers to a copper list, then configures DMA registers to enable copper interrupts.
- `copy_sprite_data`: Copies 8 words from one table to another, then copies 16 words from a second table to the same location.
- `initialize_game_state`: Initializes multiple global variables with specific values, sets up data structures for rendering and sprite management, and configures DMA registers for copper interrupts.
- `rearrange_sprite_data`: Copies memory contents between buffers, reads values from a memory location, performs a series of swaps on values in that location, and calls other routines to complete the operation.
- `initialize_render_data`: Loads and processes input values to set up rendering parameters for graphical elements, including coordinates, dimensions, and color values, then stores these computed results into global variables.
- `adjust_and_store_values`: Adjusts an input value using bitwise and arithmetic operations, then stores calculated results into global variables.
- `update_copper_data`: Updates copper list data by processing 16 words from two buffers, applying bit-wise adjustments based on comparisons, and copying the modified data to a third buffer.
- `update_sprite_data_2`: Updates sprite data by processing 16 words of source data through a series of bitwise comparisons and increments, then copies the modified data to a destination table.
- `update_color_table`: Updates a color table by decrementing specific bits based on their values, then calls another routine for each entry in the table.
- `update_sprite_data_3`: Updates sprite position data by decrementing high, middle, and low bytes in sequence, then copies updated data to a buffer and performs additional processing on the sprite attributes.
- `decrement_sprite_data`: Reads sprite data from a table, decrements values in multiple stages based on bit conditions, and writes the modified data back to the same table.
- `update_sprite_states`: Processes sprite data by reading values from multiple global variables, checking conditions based on comparison operations, and updating specific memory locations with -1 when conditions are met.
- `process_input_loop`: Processes input bytes in a loop, reads from a pointer and compares with -1, calls another routine for each byte, decrements a counter until it reaches zero, then stores a value to a global variable and returns.
- `calculate_and_clear`: Performs arithmetic calculations on values from global variables, computes division results, compares with thresholds, and clears memory locations pointed to by multiple pointers in a loop.
- `initialize_game_parameters`: Sets up various game state variables including counters, coordinates, and timing values, then performs iterative updates to these values through multiple conditional branches and arithmetic operations.
- `frag_lea_movea_move_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies the memory at a0 (advancing a0) to a1, then copies 123 to the memory at a0, then copies 117 to the memory at a1, then returns.
- `frag_lea_movea_move_rts_2`: Short code fragment that loads the address of a fixed memory address into a0, then copies the memory at a0 (advancing a0) to a1, then copies 167 to the memory at a0, then copies 142 to the memory at a1, then returns.
- `copy_sprite_data_2`: Copies sprite-related data from one memory location to another, including coordinates and attributes, then performs additional data transfers and clears specific memory locations.
- `update_sprite_data_4`: Copies sprite position and attribute data from global memory locations to a display buffer, updating multiple sprite entries with coordinates and settings from predefined offsets.
- `generate_sprite_coordinates`: Calculates sprite position data by reading from global memory locations, performing bitwise operations and arithmetic calculations, then stores the results into multiple arrays at specific offsets.
- `update_game_objects_2`: Updates position and state data for multiple game objects stored in tables, processing each object through a loop that modifies coordinates based on random values, and finally calls a rendering routine.
- `store_calculated_values`: Calculates values based on global variables, performs division operations, stores results into arrays at specific offsets, and updates array entries with computed values.
- `fill_data_structure`: Fills a data structure with calculated values from another routine, stores specific byte values to multiple memory locations, and iterates through an array of 3 elements.
- `process_sprite_data_backward`: Processes sprite coordinate data in reverse direction by reading values from global variables, performing bitwise operations and arithmetic calculations, then storing results into multiple arrays with different memory offsets.
- `update_game_state_variables`: Updates game state counters and memory buffers through iterative operations, performs conditional modifications based on comparison results, and copies data between memory regions using loop structures.
- `copy_game_state_data`: Copies several 32-bit values from one memory region to another, then copies multiple 16-bit values from a global structure to a different memory region with some arithmetic and clearing operations.
- `update_game_state`: Updates game state variables including counters and memory buffers, then conditionally modifies screen and input parameters based on a comparison result.
- `frag_move_clr_bsr_rts`: Short code fragment that copies 64 to d0, then copies 200 to d1, then copies 6 to d5, then clears d7, then calls another routine, then returns.
- `frag_jmp_9`: Short code fragment that jumps to another point in the code.
- `frag_jmp_10`: Short code fragment that jumps to another point in the code.
- `frag_jmp_11`: Short code fragment that jumps to another point in the code.
- `frag_jmp_12`: Short code fragment that jumps to another point in the code.
- `frag_jmp_13`: Short code fragment that jumps to another point in the code.
- `frag_jmp_14`: Short code fragment that jumps to another point in the code.
- `frag_jmp_15`: Short code fragment that jumps to another point in the code.
- `frag_jmp_16`: Short code fragment that jumps to another point in the code.
- `frag_jmp_17`: Short code fragment that jumps to another point in the code.
- `frag_jmp_18`: Short code fragment that jumps to another point in the code.
- `frag_jmp_19`: Short code fragment that jumps to another point in the code.
- `frag_jmp_20`: Short code fragment that jumps to another point in the code.
- `frag_jmp_21`: Short code fragment that jumps to another point in the code.
- `frag_jmp_22`: Short code fragment that jumps to another point in the code.
- `frag_jmp_23`: Short code fragment that jumps to another point in the code.
- `frag_jmp_24`: Short code fragment that jumps to another point in the code.
- `frag_jmp_25`: Short code fragment that jumps to another point in the code.
- `frag_jmp_26`: Short code fragment that jumps to another point in the code.
- `frag_jmp_27`: Short code fragment that jumps to another point in the code.
- `frag_jmp_28`: Short code fragment that jumps to another point in the code.
- `frag_jmp_29`: Short code fragment that jumps to another point in the code.
- `frag_jmp_30`: Short code fragment that jumps to another point in the code.
- `frag_jmp_31`: Short code fragment that jumps to another point in the code.
- `frag_jmp_32`: Short code fragment that jumps to another point in the code.
- `frag_jmp_33`: Short code fragment that jumps to another point in the code.
- `initialize_game_state_2`: Initializes multiple global variables and memory regions with specific values, clears large memory blocks, and sets up hardware registers for display and input handling.
- `frag_move_rts`: Short code fragment that copies 16384 to a fixed memory address, then copies 11393 to a fixed memory address, then copies 62657 to a fixed memory address, then copies a fixed memory address to a fixed memory address, then copies 440 to a fixed memory address, then copies 49152 to a fixed memory address, then returns.
- `frag_move_bsr_rts_4`: Short code fragment that copies 1 to d5, then calls another routine, then copies 1 to d5, then calls another routine, then returns.
- `update_joystick_input`: Reads joystick data, updates position tracking variables, and handles timing for input processing while setting display window registers.
- `select_and_copy_data`: Chooses a data source based on a comparison value, copies memory contents to a destination, and calls another routine with the selected address.
- `frag_lea_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a1, then copies 15 to d7, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d7, then returns.
- `frag_lea_move_dbra_rts_2`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 15 to d7, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d7, then returns.
- `frag_lea_move_clr_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 15 to d7, then copies the memory at a0 to the memory at a1 (advancing a1), then clears the memory at a0 (advancing a0), then loops back while counting down d7, then returns.
- `frag_lea_move_dbra_rts_3`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 15 to d7, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d7, then returns.
- `frag_move_ext_asr_andi_clr_mulu_add_rts`: Short code fragment that copies a fixed memory address to d1, then sign-extends d1, then shifts d1 right by 3, then masks d1 with an address, then clears d0, then copies a fixed memory address to d0, then multiplies d0 by 40 (unsigned), then adds d1 to d0, then returns.
- `update_coordinates`: Updates coordinate values using helper routines, performs arithmetic operations, and stores results in memory locations while handling conditional checks for movement limits.
- `process_coordinate_changes`: Processes position coordinates from a source value, updates multiple coordinate variables, handles boundary conditions for movement limits, and performs arithmetic operations on coordinate values.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `calculate_and_store_coordinates`: Updates coordinate values using helper routines, performs arithmetic operations, and stores results in memory locations while handling conditional bit manipulation.
- `frag_addi_bsr_bra`: Short code fragment that adds 1 to a fixed memory address, then calls another routine, then jumps to another point in the code.
- `configure_blitter_registers`: Sets up multiple blitter registers with data pointers, configuration values, and size parameters before calling a routine to wait for blitter completion.
- `update_position_coordinates`: Processes position coordinates from a source value, updates multiple coordinate variables, and handles boundary conditions for movement limits.
- `generate_random_seed`: Calculates a pseudo-random number seed by multiplying a global variable by five, adding seventeen, and combining it with vertical beam position and another global variable.
- `frag_bsr_andi_cmpi_beq_subi_rts`: Short code fragment that calls another routine, then masks d0 with 3, then compares d0 with 3, then branches if the last result was zero, then subtracts 1 from d0, then returns.
- `handle_joystick_input`: Clears input state variables, reads joystick data from hardware registers, processes button states, and performs bitwise operations on input values before storing results.
- `frag_movea_movem_move_rts`: Short code fragment that copies a fixed memory address to a5, then copies several registers to or from memory, then copies a5 to a fixed memory address, then returns.
- `frag_movea_movem_move_rts_2`: Short code fragment that copies a fixed memory address to a5, then copies several registers to or from memory, then copies a5 to a fixed memory address, then returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `frag_move_lsl_add_rts`: Short code fragment that copies a fixed memory address to d0, then copies a fixed memory address to d1, then copies a fixed memory address to d0, then shifts d1 left by 8, then adds d0 to d1, then copies d1 to a fixed memory address, then returns.
- `frag_move_sub_rts`: Short code fragment that copies a fixed memory address to d0, then subtracts a fixed memory address from d0, then returns.
- `frag_move_lsl_add_sub_rts`: Short code fragment that copies a fixed memory address to d0, then copies a fixed memory address to d1, then copies a fixed memory address to d0, then shifts d1 left by 8, then adds d0 to d1, then subtracts a fixed memory address from d1, then copies d1 to a fixed memory address, then returns.
- `update_and_wait`: Updates several global variables with values from memory locations, clears a counter, and then waits for specific keyboard input before returning.
- `frag_movea_move_clr_dbra_rts`: Short code fragment that copies a fixed memory address to a0, then copies 7999 to d7, then clears the memory at a0 (advancing a0), then loops back while counting down d7, then returns.
- `frag_movea_bsr_rts_3`: Short code fragment that copies a fixed memory address to a1, then calls another routine, then returns.
- `copy_graphics_data`: Copies large blocks of graphic data between memory locations by iterating through multiple arrays and transferring 32-bit words using loop counters.
- `frag_movem_rts`: Short code fragment that copies several registers to or from memory (6 times in a row), then returns.
- `frag_movea_bsr_adda_move_rts`: Short code fragment that copies a fixed memory address to a0, then calls another routine, then adds 316 to a0, then copies a fixed memory address to a fixed memory address, then calls another routine, then returns.
- `transfer_pattern_data_2`: Transfers byte-sized pattern data from one memory location to another while incrementing pointers, handling multiple rows of data with specific bit manipulation operations.
- `frag_lea_clr_move_cmp_beq_addq_bra_bsr`: Short code fragment that loads the address of a fixed memory address into a2, then clears d1, then copies an indexed entry from a2 to d2, then compares d0 with d2, then branches if the last result was zero, then adds 1 to d1, then jumps to another point in the code, then copies d1 to d0, then calls another routine, then returns.
- `transfer_pattern_data`: Transfers byte-sized pattern data from one memory location to another while incrementing pointers, handling multiple rows of data with specific offset calculations.
- `frag_move_andi_lsr_rts`: Short code fragment that copies a fixed memory address to d0, then copies d0 to d1, then masks d0 with 255, then shifts d1 right by 8, then copies d0 to a fixed memory address, then copies d1 to a fixed memory address, then copies 160 to a fixed memory address, then copies 80 to a fixed memory address, then returns.
- `update_coordinates_2`: Updates coordinate values by reading from memory locations, performing arithmetic operations, applying boundary checks, and storing results back to memory while handling conditional logic.
- `configure_display_parameters`: Checks a keyboard status register and if it matches a specific value, updates display timing parameters and modifies values in memory arrays using arithmetic operations.
- `frag_move_clr_tst_bne_rts`: Short code fragment that copies a fixed memory address to d0, then clears a fixed memory address, then tests d0, then branches if the last result was not zero, then returns.
- `manage_counter_sequence`: Manages a counter sequence by iterating through memory locations, incrementing values with addition instructions, checking against maximum limits, and updating multiple memory locations with calculated values.
- `initialize_game_variables`: Sets up initial game state variables including memory pointers, coordinate values, and counter variables before beginning gameplay operations.
- `frag_subi_mulu_addi_move_bsr_clr_rts`: Short code fragment that subtracts 48 from d0, then multiplies d0 by 5 (unsigned), then adds 129 to d0, then copies d0 to a fixed memory address, then calls another routine, then adds 6 to a fixed memory address, then clears d0, then returns.
- `frag_bsr_cmpi_beq_rts`: Short code fragment that calls another routine, then compares a fixed memory address with 0, then branches if the last result was zero, then returns.
- `setup_copper_list`: Configures copper list registers by copying data from memory arrays, waits for a raster line, then sets up DMA control registers to enable display operations.
- `update_display_data`: Copies word-sized data from two different memory locations to a target address, then calculates and stores updated timing values based on a global variable.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_for_raster_line_2`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `frag_cmpi_bne_beq_rts`: Short code fragment that compares a fixed memory address with 205, then branches if the last result was not zero, then compares a fixed memory address with 205, then branches if the last result was zero, then compares a fixed memory address with 205, then branches if the last result was not zero, then compares a fixed memory address with 205, then branches if the last result was zero, then returns.
- `manage_game_state`: Manages game state variables including memory pointers, coordinate values, and counter variables before beginning gameplay operation, performing conditional checks and arithmetic operations on various counters.
- `increment_character_counters`: Increments character values in a sequence of memory locations, with conditional checks that reset counters when reaching specific thresholds and subtracting fixed values.
- `convert_number_to_digits`: Converts a word value into individual decimal digits and stores them in memory locations, then calls a character output routine four times to display each digit.
- `convert_number_to_digits_2`: Converts a word value into individual decimal digits and stores them in memory locations, then calls a character output routine three times to display each digit.
- `convert_number_to_digits_3`: Converts a word value into individual decimal digits and stores them in memory locations, then calls a character output routine five times to display each digit.
- `update_boundary_values`: Updates boundary-related values based on a global counter and performs conditional operations including random number generation and bit manipulation.
- `decrement_and_compare`: Decrements two global variables and compares them with corresponding values, calling other routines when the comparison fails.
- `clear_memory_blocks`: Clears multiple memory blocks by setting their contents to zero using loop counters and pointer arithmetic.
- `frag_jmp_34`: Short code fragment that jumps to another point in the code.
- `frag_jmp_35`: Short code fragment that jumps to another point in the code (2 times in a row).
- `frag_jmp_36`: Short code fragment that jumps to another point in the code.
- `frag_jmp_37`: Short code fragment that jumps to another point in the code (2 times in a row).
- `frag_jmp_38`: Short code fragment that jumps to another point in the code.
- `frag_jmp_39`: Short code fragment that jumps to another point in the code.
- `frag_jmp_40`: Short code fragment that jumps to another point in the code.
- `frag_jmp_41`: Short code fragment that jumps to another point in the code.
- `frag_jmp_42`: Short code fragment that jumps to another point in the code.
- `frag_jmp_43`: Short code fragment that jumps to another point in the code.
- `frag_jmp_44`: Short code fragment that jumps to another point in the code.
- `frag_jmp_45`: Short code fragment that jumps to another point in the code.
- `frag_jmp_46`: Short code fragment that jumps to another point in the code.
- `frag_jmp_47`: Short code fragment that jumps to another point in the code.
- `frag_jmp_48`: Short code fragment that jumps to another point in the code.
- `frag_jmp_49`: Short code fragment that jumps to another point in the code.
- `frag_jmp_50`: Short code fragment that jumps to another point in the code.
- `frag_jmp_51`: Short code fragment that jumps to another point in the code.
- `frag_jmp_52`: Short code fragment that jumps to another point in the code.
- `frag_jmp_53`: Short code fragment that jumps to another point in the code.
- `frag_jmp_54`: Short code fragment that jumps to another point in the code.
- `frag_jmp_55`: Short code fragment that jumps to another point in the code.
- `frag_move_rts_2`: Short code fragment that copies a fixed memory address to a fixed memory address, then returns.
- `process_boundary_conditions`: Processes boundary conditions by comparing values from memory locations, performing arithmetic operations on registers, and calling subroutines to handle different conditional branches based on comparison results.
- `compare_and_process`: Compares indexed values and processes data based on comparison results, looping through different conditions and calling other routines for further processing.
- `frag_tst_beq_movem_move_clr_cmp_bpl_lsl`: Short code fragment that tests d3, then branches if the last result was zero, then saves registers on the stack, then copies 1 to d0, then clears d1, then compares d3 with d2, then branches if the last result was not negative, then shifts d3 left by 1, then shifts d0 left by 1, then jumps to another point in the code.
- `frag_lsr_tst_beq_cmp_bmi_sub_add_bra`: Short code fragment that shifts d0 right by 1, then tests d0, then branches if the last result was zero, then shifts d3 right by 1, then compares d2 with d3, then branches if the last result was negative, then subtracts d3 from d2, then adds d0 to d1, then jumps to another point in the code.
- `frag_swap_move_movem_rts`: Short code fragment that swaps the halves of d2, then copies d1 to d2, then restores saved registers from the stack, then returns.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d2, then returns.
- `check_boundary_condition`: Checks if a boundary condition is met by comparing values and updates a counter, returning early if the condition fails.
- `adjust_position`: Adjusts a position value based on differences between two indexed values and stores a sign flag in a global variable.
- `adjust_and_or`: Adjusts position values based on indexed differences, performs bitwise OR operations with table data, and updates multiple memory locations with the results.
- `frag_or_addi_sub_beq_lsr_subq_move`: Short code fragment that merges d5 into the memory at a2 (advancing a2), then merges d5 into the memory at a3 (advancing a3), then merges d5 into the memory at a4 (advancing a4), then merges d5 into the memory at a5 (advancing a5), then adds 16 to d2, then subtracts d2 from d3, then branches if the last result was zero, then shifts d3 right by 4, then subtracts 1 from d3, then copies 65535 to d5.
- `copy_word_to_multiple_locations`: Copies a word value to multiple memory locations using a loop that increments pointers until a counter reaches zero.
- `or_word_to_multiple_locations`: Performs bitwise OR operations between a word value and contents of multiple memory locations, updating each location with the result.
- `frag_or_movem_rts`: Short code fragment that merges d5 into the memory at a2 (advancing a2), then merges d5 into the memory at a3 (advancing a3), then merges d5 into the memory at a4 (advancing a4), then merges d5 into the memory at a5 (advancing a5), then restores saved registers from the stack, then returns.
- `update_pattern_table`: Updates a pattern table by writing word values to multiple memory locations based on bitwise operations with a global variable, then increments a counter and shifts a bit mask in a loop.
- `save_registers_and_copy_data`: Saves register contents to memory, copies data between memory locations, and restores registers from saved values using multiple movem instructions.
- `update_sprite_data`: Updates sprite coordinate values and handles sprite rendering data based on a condition, then copies byte data from one memory location to another.
- `update_coordinates_and_call_handler`: Updates coordinate values based on conditions, calls a handler routine, and modifies additional values before returning.
- `frag_move_addi_bsr_rts`: Short code fragment that copies a fixed memory address to a fixed memory address (2 times in a row), then adds 40 to a fixed memory address, then adds 2 to a fixed memory address, then copies 1 to d7, then copies 9 to d6, then copies 10 to a fixed memory address, then calls another routine, then copies 12 to a fixed memory address, then returns.
- `frag_move_addi_clr_bsr_rts`: Short code fragment that copies a fixed memory address to a fixed memory address (2 times in a row), then adds 40 to a fixed memory address, then adds 27 to a fixed memory address, then clears d7, then copies 5 to d6, then calls another routine, then returns.
- `frag_move_addi_bsr_rts_2`: Short code fragment that copies a fixed memory address to a fixed memory address (2 times in a row), then adds 2 to a fixed memory address, then adds 39 to a fixed memory address, then copies 7 to d7, then copies 15 to d6, then calls another routine, then returns.
- `update_animation_state`: Updates animation state variables based on input data and performs conditional operations on counters.
- `frag_jmp_56`: Short code fragment that jumps to another point in the code.
- `frag_jmp_57`: Short code fragment that jumps to another point in the code.
- `frag_jmp_58`: Short code fragment that jumps to another point in the code.
- `update_projectile_data`: Updates projectile position and state information in global arrays based on calculated deltas and collision boundaries, then sets a global flag when conditions are met.
- `update_projectile_state`: Updates projectile position and state information based on calculated deltas and collision boundaries, then sets a global flag when conditions are met.
- `validate_collision_data`: Validates collision detection data by checking multiple conditions against stored values.
- `validate_collision_data_2`: Checks collision boundaries against screen coordinates and updates position offsets based on calculated deltas, then sets a global flag when certain conditions are met.
- `generate_random_coordinates`: Generates random coordinate values for graphical elements by calling a random number generator function.
- `update_graphics_elements`: Updates graphical element coordinates and states in multiple tables based on current values and a global offset, with conditional logic for boundary checks and state transitions.
- `update_game_state_2`: Updates game state variables including input handling, position tracking, and rendering parameters based on keyboard input and loop conditions, then restores saved register values before returning.
- `initialize_render_loop`: Initializes rendering loop parameters and calls another routine in a counted loop.
- `frag_lea_move_rts_2`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies a fixed memory address to d0, then copies an indexed entry from a0 to a fixed memory address, then copies an indexed entry from a1 to a fixed memory address, then returns.
- `populate_render_buffer`: Populates render buffer entries with calculated position and size data.
- `scale_coordinates`: Processes coordinate scaling for graphical elements by reading input values, clamping them within specific ranges, performing multiplication and division operations, and storing the results back to global variables.
- `initialize_collision`: Initializes collision detection parameters and sets up global variables for object interaction.
- `update_collision_data`: Processes collision detection parameters by updating counters, calculating coordinates, and checking boundaries before invoking another routine.
- `initialize_render_params_2`: Initializes rendering parameters by copying data from one global structure to another, setting specific word values in multiple global variables.
- `initialize_render_params`: Initializes rendering parameters by copying data from one global structure to another, setting specific word values in multiple global variables, and performing a calculation involving a global word variable before restoring its original value.
- `update_collision_masks`: Processes collision mask data by performing bitwise AND and OR operations on memory locations, updating the masks based on conditions.
- `update_lancaster_position`: Updates position coordinates for a game object based on collision data and movement parameters, adjusting boundaries and updating related global variables.
- `initialize_level_data`: Reads level coordinates from a data structure, calculates position values, and initializes multiple global variables with tile mapping and positioning data.
- `copy_memory_block_2`: Copies data between memory locations by moving 8000 words from one table to another through a loop that processes 2000 iterations.
- `update_projectile_data_2`: Updates projectile position and state information based on calculated deltas and collision boundaries, then sets a global flag when conditions are met.
- `copy_and_swap_registers`: Copies data between multiple memory locations and swaps register contents, moving 8 words from one table to another, then 16 words from a different table, and finally exchanges register contents between three groups of registers.
- `copy_and_clear_words`: Copies 8 words from one memory location to another, then clears 15 words at a subsequent memory location.
- `update_position_vector`: Calculates a normalized direction vector from two position coordinates and updates movement parameters in global variables.
- `increment_score_digit`: Increments a score digit at offset 26BA, resets it to '0' if it exceeds '9', and increments the tens digit at offset 26B9 when the units digit overflows.
- `increment_and_bound_check`: Increments values in a sequence of memory locations with boundary checks, updating each location based on comparison results and applying adjustments when boundaries are exceeded.
- `calculate_distance`: Calculates a distance value using coordinate differences, applies bit shifts and negations, compares values, exchanges registers when needed, and computes a final result with bit operations.
- `set_coordinates_and_parameters`: Sets coordinate values and various parameters based on input data, performs boundary checks, and calls another routine to process the results.
- `adjust_coordinates_with_bounds`: Adjusts coordinate values with boundary checks, clamps values to specific ranges, and updates memory locations with the adjusted coordinates.
- `initialize_interrupts`: Sets up interrupt handling by writing specific values to a hardware register, then performs a series of memory operations including copying data between locations and updating counters, before branching to a common exit point.
- `fill_memory_with_ones`: Fills a memory area with 32-bit values of all ones using a loop that decrements a counter.
- `update_score_display`: Updates score display digits by copying character data from various sources into display buffers, handles digit rollover for tens place, and manages carry propagation for multi-digit scores.
- `frag_movea_adda_bsr_rts`: Short code fragment that copies a fixed memory address to a0, then adds 965 to a0, then copies a fixed memory address to a3, then calls another routine, then returns.
- `frag_movea_adda_bsr_rts_2`: Short code fragment that copies a0 to a3, then copies a fixed memory address to a0, then adds 1445 to a0, then calls another routine, then returns.
- `frag_movea_adda_bsr_rts_3`: Short code fragment that copies a0 to a3, then copies a fixed memory address to a0, then adds 1445 to a0, then calls another routine, then returns.
- `frag_lea_movea_adda_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a3, then copies a fixed memory address to a0, then adds 1445 to a0, then calls another routine, then returns.
- `frag_lea_movea_adda_bsr_rts_2`: Short code fragment that loads the address of a fixed memory address into a3, then copies a fixed memory address to a0, then adds 4485 to a0, then calls another routine, then returns.
- `call_subroutine_multiple_times_2`: Calls another routine multiple times with different parameters, incrementing an address pointer between each call.
- `frag_clr_move_bsr_addq_rts`: Short code fragment that clears d5, then copies 3 to d4, then calls another routine, then adds 1 to a0, then copies 9 to d4, then calls another routine, then adds 1 to a0, then copies 1 to d4, then calls another routine, then returns.
- `frag_move_bsr_dbra_rts_2`: Short code fragment that copies the memory at a3 (advancing a3) to d0, then calls another routine, then loops back while counting down d4, then returns.
- `add_decimal_digits`: Performs decimal addition on digit values from two memory locations, handles carry propagation, and updates the second location with the result.
- `subtract_decimal_digits`: Performs decimal subtraction on digit values from two memory locations, handles borrowing, and updates the second location with the result.
- `frag_move_dbra_rts_jmp`: Short code fragment that copies 400 to d7, then copies 2000 to d6, then loops back while counting down d6, then loops back while counting down d7, then returns, then jumps to another point in the code.
- `frag_jmp_59`: Short code fragment that jumps to another point in the code.
- `frag_jmp_60`: Short code fragment that jumps to another point in the code.
- `frag_jmp_61`: Short code fragment that jumps to another point in the code.
- `frag_jmp_62`: Short code fragment that jumps to another point in the code.
- `frag_jmp_63`: Short code fragment that jumps to another point in the code.
- `frag_jmp_64`: Short code fragment that jumps to another point in the code.
- `frag_jmp_65`: Short code fragment that jumps to another point in the code.
- `frag_jmp_66`: Short code fragment that jumps to another point in the code.
- `frag_jmp_67`: Short code fragment that jumps to another point in the code.
- `frag_jmp_68`: Short code fragment that jumps to another point in the code.
- `frag_jmp_69`: Short code fragment that jumps to another point in the code.
- `frag_jmp_70`: Short code fragment that jumps to another point in the code.
- `frag_jmp_71`: Short code fragment that jumps to another point in the code.
- `frag_jmp_72`: Short code fragment that jumps to another point in the code.
- `frag_jmp_73`: Short code fragment that jumps to another point in the code.
- `frag_jmp_74`: Short code fragment that jumps to another point in the code.
- `frag_jmp_75`: Short code fragment that jumps to another point in the code.
- `frag_jmp_76`: Short code fragment that jumps to another point in the code.
- `frag_jmp_77`: Short code fragment that jumps to another point in the code.
- `frag_jmp_78`: Short code fragment that jumps to another point in the code.
- `frag_jmp_79`: Short code fragment that jumps to another point in the code.
- `frag_jmp_80`: Short code fragment that jumps to another point in the code.
- `frag_jmp_81`: Short code fragment that jumps to another point in the code.
- `frag_jmp_82`: Short code fragment that jumps to another point in the code.
- `frag_jmp_83`: Short code fragment that jumps to another point in the code.
- `frag_jmp_84`: Short code fragment that jumps to another point in the code.
- `frag_jmp_85`: Short code fragment that jumps to another point in the code.
- `frag_jmp_86`: Short code fragment that jumps to another point in the code.
- `frag_jmp_87`: Short code fragment that jumps to another point in the code.
- `frag_jmp_88`: Short code fragment that jumps to another point in the code.
- `frag_jmp_89`: Short code fragment that jumps to another point in the code.
- `frag_jmp_90`: Short code fragment that jumps to another point in the code.
- `frag_jmp_91`: Short code fragment that jumps to another point in the code.
- `frag_jmp_92`: Short code fragment that jumps to another point in the code.
- `initialize_game_state_4`: Checks if a global counter equals two and sets another global variable to ten, then clears multiple global variables and calls several routines before returning.
- `frag_bsr_move_rts`: Short code fragment that calls another routine (4 times in a row), then copies 4369 to a fixed memory address, then returns.
- `decrement_and_process`: Decrements a counter in a global variable and processes object state data, updating position and velocity values while managing a loop of up to 16 iterations based on a table lookup.
- `process_object_states`: Checks two global counters and processes object state data when both are non-zero, updating position and velocity values while managing a loop of up to seven iterations.
- `update_object_states`: Processes object states stored in a global table, updating counters and performing arithmetic operations based on state values.
- `update_object_state`: Processes object state data from a global table, updating counters and performing arithmetic operations based on state values, then stores the results back into the table.
- `process_input_value`: Processes a value from a global variable through bitwise operations and conditional branches, returning different variants of that value based on comparison results with constants.
- `set_render_parameters`: Sets specific longword values into two memory locations based on the comparison of a global variable with constant values, effectively selecting rendering parameters.
- `process_object_data`: Processes object data entries, updates coordinates and states based on conditions, and performs calculations for rendering or physics.
- `update_enemy_data`: Processes enemy entries from a global table, updating their positions and states based on conditions, and handles enemy lifecycle management including creation and removal.
- `update_game_objects`: Processes game object data from multiple tables using a loop counter, updates coordinates and state flags, and calls another routine to handle object rendering or movement.
- `handle_input_events`: Reads keyboard input from a hardware register and modifies values in memory based on specific key presses, updating position coordinates for game elements.
- `update_keyboard_state`: Reads the keyboard serial register and inverts a global variable's value when the register equals 127.
- `frag_move_addi_subi_rts`: Short code fragment that copies a fixed memory address to d3, then adds 4 to d3, then copies d3 to a fixed memory address, then copies a fixed memory address to d3, then subtracts 4 from d3, then copies d3 to a fixed memory address, then copies a fixed memory address to d3, then adds 4 to d3, then copies d3 to a fixed memory address, then returns.
- `read_keyboard_state`: Repeatedly reads the keyboard serial register and compares its value against fixed constants, but does not store or process the results.
- `frag_move_cmpi_bne_subi_addi_andi_rts`: Short code fragment that copies a fixed memory address to d0, then compares a fixed memory address with 99, then branches if the last result was not zero, then subtracts 4 from d0, then compares a fixed memory address with 97, then branches if the last result was not zero, then adds 4 to d0, then masks d0 with 12, then copies d0 to a fixed memory address, then returns.
- `perform_bit_shift_operations`: Executes a series of bit manipulation operations on two long integers, including negation, shifting, and logical operations, but returns an undefined value.
- `copy_position_data`: Copies word-sized values from one memory location to another in a sequence of moves, then calls another routine to process the copied data.
- `frag_move_add_bmi`: Short code fragment that copies a fixed memory address to d0, then adds a fixed memory address to d0, then branches if the last result was negative.
- `frag_bsr_rts_clr`: Short code fragment that calls another routine (5 times in a row), then returns, then clears a fixed memory address, then returns.
- `compute_vector_products`: Computes vector products from global variables and stores intermediate results in global memory locations.
- `process_vertex_data`: Processes vertex coordinates from a data table, calculates accumulated sums of absolute values and adds a constant, then stores the results in a global buffer.
- `sort_collision_data`: Sorts collision data using a variant of insertion sort, processing pairs of values from two tables based on a comparison key.
- `iterate_and_update`: Processes a counted loop of data entries, updating pointers and calling another routine for each iteration, then returns a modified value.
- `process_enemy_data`: Processes enemy position and state data using global variables and tables, updating counters and handling collision detection logic for specific enemy types.
- `frag_jmp_93`: Short code fragment that jumps to another point in the code.
- `frag_jmp_94`: Short code fragment that jumps to another point in the code.
- `frag_jmp_95`: Short code fragment that jumps to another point in the code.
- `frag_jmp_96`: Short code fragment that jumps to another point in the code.
- `frag_jmp_97`: Short code fragment that jumps to another point in the code.
- `frag_jmp_98`: Short code fragment that jumps to another point in the code.
- `frag_jmp_99`: Short code fragment that jumps to another point in the code.
- `frag_jmp_100`: Short code fragment that jumps to another point in the code.
- `frag_jmp_101`: Short code fragment that jumps to another point in the code.
- `frag_jmp_102`: Short code fragment that jumps to another point in the code.
- `frag_jmp_103`: Short code fragment that jumps to another point in the code.
- `frag_jmp_104`: Short code fragment that jumps to another point in the code.
- `frag_jmp_105`: Short code fragment that jumps to another point in the code.
- `frag_jmp_106`: Short code fragment that jumps to another point in the code.
- `frag_jmp_107`: Short code fragment that jumps to another point in the code.

### Lancaster (1989)(CRL).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
