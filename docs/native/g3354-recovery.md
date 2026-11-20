# Game 3354 (Demon Blue): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03354). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (917856 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 186 raw-word lines |
| Functions named and described | pass, 66 of 66 named (0 author symbols), 66 of 66 described (14 by rule, 51 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1976 of 1976 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3713 of 3713 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11695 |
| code | 28542 |
| data | 862122 |
| padding | 1161 |
| unused | 14336 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Demon Blue.adf:bootblock | 1 | 42 | 3 |
| logged disk load from Demon Blue.adf boot-block load | 1 | 155 | 2 |
| logged disk load from Demon Blue.adf boot-block load | 1 | 434 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3082 | 45 |

## Cases handled

- Demon Blue/Demon Blue.adf: custom-format (non-DOS) disk, 1730 non-empty blocks: matched against track-loaded segments
- Demon Blue/Demon Blue.adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- Demon Blue/Demon Blue.adf: the boot block reads 4096 bytes from disk offset 1536 to an address (found statically); recovered as a unit
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1959 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Demon Blue/Demon Blue.adf: 2 logged loads cover 9728 bytes of the image
- Demon Blue/Demon Blue.adf: 30 track-loaded blocks found in recovered runtime segments and classified through them
- Demon Blue/Demon Blue.adf: blocks 22-71 (bytes 11264-36864) outside the filesystem hold data, not code (opcode density 0.047, entropy 6.60)
- Demon Blue/Demon Blue.adf: blocks 82-82 (bytes 41984-42496) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.69)
- Demon Blue/Demon Blue.adf: blocks 103-114 (bytes 52736-58880) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.10)
- Demon Blue/Demon Blue.adf: blocks 126-145 (bytes 64512-74752) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.23)
- Demon Blue/Demon Blue.adf: blocks 148-160 (bytes 75776-82432) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.60)
- Demon Blue/Demon Blue.adf: blocks 162-409 (bytes 82944-209920) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.34)
- Demon Blue/Demon Blue.adf: blocks 412-662 (bytes 210944-339456) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.69)
- Demon Blue/Demon Blue.adf: blocks 664-665 (bytes 339968-340992) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.18)
- Demon Blue/Demon Blue.adf: blocks 667-1293 (bytes 341504-662528) outside the filesystem hold data, not code (opcode density 0.011, entropy 7.17)
- Demon Blue/Demon Blue.adf: blocks 1295-1320 (bytes 663040-676352) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.15)
- Demon Blue/Demon Blue.adf: blocks 1324-1361 (bytes 677888-697344) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.23)
- Demon Blue/Demon Blue.adf: blocks 1363-1390 (bytes 697856-712192) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.75)
- Demon Blue/Demon Blue.adf: blocks 1392-1429 (bytes 712704-732160) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.33)
- Demon Blue/Demon Blue.adf: blocks 1431-1457 (bytes 732672-746496) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.22)
- 1 packed track region(s), 153600 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Demon Blue.adf:bootblock

- `allocate_and_execute_io`: Allocates memory using the exec library, sets up an I/O request with specific parameters, executes it, and then calls another function with the result.
- `frag_movem_move_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies 2 to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then copies 5632 to the memory at an offset from a1 (2 times in a row), then copies a fixed memory address to a6, then calls another routine (2 times in a row), then restores saved registers from the stack, then returns.
- `execute_io_request`: Sets up and executes a hardware I/O request using the exec library, then calls a function with the result and returns a combined value.

### logged disk load from Demon Blue.adf boot-block load

- `shift_and_store`: Performs bit shifting on a value from memory, stores the result in another memory location, and returns with the shifted value in a register.
- `bit_shift_and_copy`: Performs bit shifting operations on input values, recursively calls itself, and accumulates results in a register before returning.

### logged disk load from Demon Blue.adf boot-block load

- `initialize_dma_and_interrupts`: Initializes DMA and interrupt settings by configuring hardware registers including display synchronization, then sets up global variables for animation and input handling before returning.
- `wait_and_setup_dma`: Waits for a DMA channel to become available, configures multiple hardware registers including bitblt parameters, and ensures the DMA transfer is properly set up before returning.
- `update_animation_counter`: Updates an animation frame counter and handles buffer position tracking when the counter reaches four, clearing the buffer position and copying data from a table to two locations before incrementing the buffer position and counter.
- `update_keyboard_state`: Reads keyboard input from hardware registers, modifies a global variable with bit manipulation, and updates CIA control registers based on input conditions.
- `render_text_characters`: Waits for display synchronization, processes character data from memory, and copies character graphics to video memory using a loop with multiple memory operations.
- `update_joystick_input`: Reads joystick data from hardware registers, calculates movement values, stores them in multiple memory locations, and updates global variables with computed results.
- `configure_dma_registers`: Checks conditions on global variables and writes fixed values to DMA control registers, then copies data from one memory location to another using a loop with a counter.
- `divide_and_store_value`: Divides a value by ten and stores the result in a global variable, incrementing a counter after each operation.
- `update_digit_display`: Performs arithmetic operations on digit values, updates display buffers, and modifies character pointers for rendering.
- `frag_move_moveq_bsr_addi_dbra_rts`: Short code fragment that copies 0 to a fixed memory address, then loads 0 into d6, then calls another routine, then adds 800 to a fixed memory address, then loops back while counting down d6, then copies 0 to a fixed memory address, then returns.
- `copy_character_data`: Copies character data from one memory location to another using a loop with fixed offsets and a counter.
- `copy_structure_data`: Copies specific byte values from one memory region to another across multiple iterations, updating pointers and decrementing a counter until completion.
- `initialize_character_data`: Multiplies an input value by 800, stores the result in a global variable, calls another routine, and writes zero to a memory location calculated from the input.
- `calculate_and_copy_rgb_values`: Performs division and subtraction operations on a value to calculate three components, then copies these values to memory locations in a loop.
- `frag_movem_lea_moveq_move_dbra_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then loads 7 into d7, then copies an address to the memory at a0 (advancing a0), then loops back while counting down d7, then restores saved registers from the stack, then returns.
- `write_dma_control`: Writes fixed values to multiple DMA control registers and updates a global variable with data from another memory location before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_data`: Processes sprite position and rendering data, updating collision detection and visual properties based on calculated values.
- `setup_blitter`: Configures multiple blitter control registers including BLTCON0, BLTCON1, and memory stride values.
- `update_sprite_data_3`: Loads sprite data from multiple tables based on index values and stores it into a fixed memory location, updating several global variables with values from the loaded data.
- `frag_addi_cmpi_beq_rts_clr_jsr`: Short code fragment that adds 1 to a fixed memory address, then compares a fixed memory address with 16, then branches if the last result was zero, then returns, then clears a fixed memory address, then calls another routine, then returns.
- `copy_pattern_data`: Copies data between multiple memory locations using byte and long word moves, then performs a loop that transfers 800 words from one memory area to another with specific offsets.
- `frag_moveq_movea_move_addi_cmp_bne_rts`: Short code fragment that loads 0 into d0, then copies a fixed memory address to a0, then copies 0 to the memory at a0 (advancing a0), then adds 1 to d0, then compares d0 with 8000, then branches if the last result was not zero, then returns.
- `load_collision_handler`: Reads a value from a global variable, uses it as an index into a table to load a handler address, then jumps to that address.
- `frag_subi_ori`: Short code fragment that subtracts 1650 from the memory at a6 (advancing a6), then merges 110 into d0, then merges 24 into d0, then merges 92 into d0.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (andi.b, btst.l) and then falls through into the code that follows, with no return of its own.
- `process_collision`: Processes collision detection for an object, updating position and velocity based on boundary checks and table lookups.
- `copy_palette_data`: Copies 4000 words from a source table to a destination buffer with specific offsets, using a loop that increments both pointers and a counter until reaching 4000 iterations.
- `frag_lea_moveq_move_addq_cmp_bne_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads 0 into d1, then copies 0 to the memory at a0 (advancing a0), then adds 1 to d1, then compares d1 with 50, then branches if the last result was not zero, then returns.
- `clear_and_set_registers`: Clears several byte-sized hardware registers and sets specific byte values to other hardware registers, effectively initializing or resetting certain display or sound control registers.
- `frag_moveq_lea_move_addq_cmp_bmi_rts`: Short code fragment that loads 0 into d0, then loads the address of a fixed memory address into a0, then copies 0 to the memory at a0 (advancing a0), then adds 1 to d0, then compares d0 with 1400, then branches if the last result was negative, then returns.
- `update_color_palette`: Updates color palette entries in hardware registers based on data from a lookup table, with conditional logic that handles different modes of operation and modifies counters and offsets during the process.
- `frag_move_rts`: Short code fragment that copies 1 to a fixed memory address, then returns.
- `update_screen_pixels`: Processes pixel data from a table to update screen memory locations by applying bitwise operations with mask and set values from another table.
- `copy_sprite_data_4`: Copies sprite attribute data from a table to multiple memory locations based on index and offset calculations, updating data in chunks of 64.
- `copy_sprite_data_3`: Copies sprite attribute data from a table to multiple memory locations based on index and offset calculations, updating data in chunks of 64 entries before moving to the next block.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_sprite_data`: Reads sprite data from a table using an index from a global variable, then copies specific word-sized values from that table to multiple offsets in another structure.
- `decrement_timer`: Decrements a global timer variable and updates corresponding status flags based on its value, with specific handling for values below and above ten.
- `sort_and_swap_elements`: Compares 32-bit values at memory locations calculated from two offsets, swaps the values when the first is greater than or equal to the second, and also swaps corresponding elements in multiple memory regions.
- `move_and_swap_elements`: Moves data between memory locations based on calculated offsets, then swaps 32-bit values in a sequence of memory regions before continuing the loop.
- `scale_position`: Processes position components from memory, scales them using multiplication factors, accumulates the results into a total value, and performs conditional adjustments based on comparison with zero.
- `frag_move_cmp_bpl_jmp`: Short code fragment that copies a fixed memory address to d0, then compares d1 with d0, then branches if the last result was not negative, then copies 1 to a fixed memory address, then jumps to another point in the code.
- `frag_move_rts_2`: Short code fragment that copies d1 to a fixed memory address, then returns.
- `update_byte_sequence`: Reads 6 bytes from a table at offset 55022, adds the constant 161 to each byte, and writes the results to a global buffer starting at offset 7382.
- `copy_byte_block`: Copies 19 bytes from one memory region starting at offset 7405 to another region starting at offset 7382, incrementing both pointers after each byte transfer.
- `copy_sprite_data_2`: Copies sprite pattern data from two source tables into two destination areas, using indices fetched from global variables and a lookup table.
- `update_sprite_data_2`: Loads sprite data from multiple tables based on index values and stores it into a fixed memory location, updating several global variables with specific values.
- `reset_sprite_counter`: Increments a counter and compares it with another value, clearing memory at a fixed address when they match, then sets a global variable to 99.
- `process_audio_buffer`: Processes audio data by copying values from one buffer to another, clears memory locations, and updates DMA control registers while setting several global variables.
- `update_game_state`: Increments a global counter and checks if it exceeds a threshold, clearing the counter and branching based on another global variable's value.
- `update_audio_channels`: Updates audio channel registers based on input values and performs conditional calculations on audio period data, writing results to AUD0LCH through AUD3LCH.
- `update_sprite_data_4`: Processes input byte values to update sprite position and attribute data, including handling special cases for specific input values and updating DMA control registers.
- `process_input_value`: Processes an input value by extracting its lower nibble and performing different operations based on the value, including storing data and updating memory locations.
- `store_input_value`: Reads a 16-bit value from a global variable, stores it into another global variable, and then returns.
- `validate_and_set_threshold`: Compares a word value against entries in a table, updates a memory location with a threshold value based on the comparison result, and sets a flag when the values match or exceed the threshold.
- `compute_adjustment_value`: Processes input byte values through bit manipulation and arithmetic operations to compute a final adjustment value stored in a memory location, with conditional logic based on comparison results.
- `calculate_adjustment`: Processes input byte values through bit manipulation and arithmetic operations to compute a final adjustment value stored in a memory location.
- `process_input_byte`: Processes an input byte value by extracting its lower nibble and performing different operations based on the value, including storing data, modifying global variables, and manipulating hardware registers.
- `store_nibble_data`: Extracts the lower nibble from a byte value and stores it at a specific memory offset, effectively masking and preserving only the least significant four bits of the original data.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (bchg.b, btst.l) and then falls through into the code that follows, with no return of its own.
