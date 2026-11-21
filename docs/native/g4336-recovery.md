# Game 4336 (marble (1994)(h. mueller)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04336). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 43 of 43 named (0 author symbols), 43 of 43 described (8 by rule, 31 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1058 of 1058 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2070 of 2070 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 354499 |
| code | 8260 |
| data | 114522 |
| padding | 927 |
| unused | 422912 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Marble (1994)(H. Mueller).adf:Marble | 2 | 2060 | 42 |
| Marble (1994)(H. Mueller).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- 8 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- marble (1994)(h. mueller)/Marble (1994)(H. Mueller).adf: blocks 212-212 (bytes 108544-109056) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.81)
- marble (1994)(h. mueller)/Marble (1994)(H. Mueller).adf: blocks 215-215 (bytes 110080-110592) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- marble (1994)(h. mueller)/Marble (1994)(H. Mueller).adf: blocks 217-217 (bytes 111104-111616) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.54)
- marble (1994)(h. mueller)/Marble (1994)(H. Mueller).adf: blocks 883-884 (bytes 452096-453120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.64)
- marble (1994)(h. mueller)/Marble (1994)(H. Mueller).adf: blocks 1154-1154 (bytes 590848-591360) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- marble (1994)(h. mueller)/Marble (1994)(H. Mueller).adf: blocks 1157-1157 (bytes 592384-592896) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Marble (1994)(H. Mueller).adf:Marble

- `initialize_system`: Initializes system hardware registers including DMA and interrupt controllers, sets up blitter parameters, and configures custom chip registers for graphics operations.
- `frag_btst_bne_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `decrement_and_reset_counters`: Saves all general-purpose and address registers to the stack, calls another routine, then decrements a long word value at a specific memory location.
- `handle_keyboard_input`: Reads a byte from the keyboard serial register, stores it in a global variable, and calls another routine three times before clearing a bit in a hardware register.
- `frag_move_cmp_beq_rts`: Short code fragment that copies the memory at an offset from a6 to d1, then compares d1 with the memory at an offset from a6, then branches if the last result was zero, then returns.
- `process_number_string`: Processes numeric input from a string by converting it to a number through digit multiplication and addition, then performs comparisons against a lookup table of values while managing a counter for string positioning.
- `initialize_marble_data`: Reads character data from a buffer and maps each character to a specific value stored in a global variable, then performs blitter operations using calculated memory addresses and hardware register values.
- `check_collision_flags`: Sets bits in a register based on comparisons between values at specific offsets from a4 and constant values, then returns the resulting bitfield.
- `map_collision_to_direction`: Combines collision flags into a single value, then maps that value to a specific direction code using conditional jumps and returns the mapped result.
- `setup_blitter_parameters`: Calculates memory addresses for blitter operations, waits for a bit in a hardware register to clear, then writes multiple values to custom hardware registers including blitter control and address registers.
- `initialize_blitter_pattern`: Sets up blitter parameters for pattern drawing by writing specific values to custom hardware registers, waits for a bit in a hardware register to clear, then returns immediately.
- `validate_table_entry`: Calculates an address in memory using two data pointers and a counter, then compares the byte at that address with a constant value and returns 1 if they match or 0 otherwise.
- `compare_byte_value`: Reads a byte from a calculated memory address and compares it with the value one, branching to an offset if they are equal.
- `compute_memory_location`: Forms a memory address by adjusting a base pointer with a decremented counter multiplied by a fixed value, then uses this address to compare a byte against a constant and branches accordingly.
- `manage_blitter_operations`: Manages blitter operations by setting up memory addresses, waiting for hardware registers to clear, and performing multiple blitter operations with different parameters.
- `frag_move_or_and_rts`: Short code fragment that copies a location in the code to d5, then merges a location in the code into d5, then masks d5 with 3, then copies d5 to a fixed memory address, then returns.
- `setup_blitter_operation`: Configures blitter parameters for graphic operations using calculated memory addresses and hardware register values, then performs a blitter transfer setup.
- `configure_blitter_operation`: Sets up blitter parameters for drawing operations by calculating memory addresses, waits for a bit in a hardware register to clear, then writes values to custom hardware registers including blitter control and address registers.
- `setup_blitter_coordinates`: Calculates memory addresses for blitter operations using shift and multiply operations, waits for a bit in a hardware register to clear, then writes values to custom hardware registers including blitter control and address registers.
- `frag_move_bsr_cmpi_beq_rts`: Short code fragment that copies 2708 to a fixed memory address, then copies 1 to a fixed memory address, then calls another routine (2 times in a row), then compares a fixed memory address with 1, then branches if the last result was zero, then returns.
- `frag_move_bsr_cmpi_beq_rts_2`: Short code fragment that copies 2644 to a fixed memory address, then copies 1 to a fixed memory address, then calls another routine (2 times in a row), then compares a fixed memory address with 1, then branches if the last result was zero, then returns.
- `update_position_and_compare`: Decrements a counter and updates position values based on comparisons between two memory locations, then increments or decrements the position value depending on which comparison result is greater.
- `format_number_to_string`: Converts a number into a string representation by repeatedly subtracting values from it and storing characters in memory, or fills memory with zeros if the input is positive.
- `frag_move_bsr_lea_bra`: Short code fragment that copies a location in the code to d2, then calls another routine, then loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then jumps to another point in the code.
- `update_tile_data`: Copies tile data from source locations to destination structures, processing six data segments per tile and repeating this process for each of four tile operations.
- `copy_tile_data`: Copies tile data from a source table to a destination structure, processing six data segments per tile and repeating this process for each byte in the input buffer until a null terminator is encountered.
- `frag_movem_addq_bsr_rts`: Short code fragment that saves registers on the stack, then adds 1 to a fixed memory address, then calls another routine, then restores saved registers from the stack, then returns.
- `initialize_game_state`: Initializes game state by setting up memory structures, configuring hardware registers including CIAA_PRA for input handling, and clearing audio volume registers while managing a loop that compares input values against a threshold.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_counter_and_branch`: Increments a global byte variable and compares it with another global byte variable, branching to a specific location if the first is less than or equal to the second.
- `update_audio_registers`: Updates audio period registers for four sound channels based on table lookups and input data, then processes input bytes to modify hardware register values and update a global counter.
- `process_marble_data`: Reads input data from a structure, processes nibbles for table indexing, manipulates hardware registers, and updates a marble position value.
- `process_input_byte_2`: Reads an input byte from a structure, extracts nibbles for indexing into tables, performs bit manipulation on hardware registers, and updates global variables based on comparison results.
- `update_marble_position_2`: Reads input data from a structure, processes nibbles for table indexing, manipulates hardware registers, and updates a marble position value in a global variable.
- `update_marble_position`: Updates a marble's position based on table lookups and comparison checks, then stores the result in a global variable while clearing a status flag.
- `update_velocity_and_position`: Updates velocity and position values based on input parameters, performs division operations, and stores results in memory locations while handling boundary conditions.
- `calculate_movement_value`: Calculates a movement value based on input parameters, performs bit manipulation and arithmetic operations, then stores the result in a memory location.
- `process_input_byte`: Reads an input byte from a structure, extracts nibbles for indexing into tables, performs bit manipulation on hardware registers, and updates values in memory based on the input.
- `setup_and_execute_dma`: Sets up DMA channels by configuring hardware registers, performs memory transfers, and manages data movement between different memory locations using blitter operations.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `frag_movem_lea_move_addq_cmp_bne_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then copies the memory at a0 to d0, then adds 4 to d0, then copies the memory at a0 to d1, then compares d1 with d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.

### Marble (1994)(H. Mueller).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
