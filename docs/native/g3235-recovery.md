# Game 3235 (crazy challenge (19xx)(ckb)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03235). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 43 of 43 named (0 author symbols), 43 of 43 described (10 by rule, 31 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1190 of 1190 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2901 of 2901 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 443495 |
| code | 13678 |
| data | 45219 |
| padding | 904 |
| unused | 397824 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Crazy Challenge (19xx)(CKB).adf:Crazychallenge | 4 | 2891 | 42 |
| Crazy Challenge (19xx)(CKB).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Crazy Challenge (19xx)(CKB).adf:Crazychallenge

- `initialize_graphics_tables`: Initializes multiple graphics parameter tables by copying coordinate data and setting up blitter configurations with predefined values, then configures blitter parameters for rendering text glyphs and performs multiple blitter operations to display them.
- `frag_movea_bsr_move_rts`: Short code fragment that copies an address to a1, then copies an address to a0, then calls another routine, then copies 2 to a fixed memory address, then returns.
- `render_game_text`: Configures blitter parameters for rendering text glyphs using predefined memory addresses and performs multiple blitter operations to display game instructions, then returns.
- `frag_bsr_move_rts`: Short code fragment that calls another routine, then copies a0 to a fixed memory address, then copies a1 to a fixed memory address, then copies 30 to a fixed memory address, then copies 34 to a fixed memory address, then copies an address to a fixed memory address, then copies 2544 to a fixed memory address, then copies 0 to a fixed memory address, then copies 25861 to a fixed memory address, then returns.
- `configure_blitter_params`: Sets up multiple blitter registers including source and destination pointers, control words, and sizes using predefined values from data segments, then returns.
- `initialize_hardware`: Waits for a joystick button to be released, then configures multiple Amiga hardware registers including interrupt enable, DMA control, copper list pointers, and sets up display registers for video output.
- `setup_blitter_parameters`: Initializes various blitter control registers with specific values, clears multiple global variables, and configures the blitter for drawing operations by setting source and destination pointers, moduli, and size parameters.
- `frag_move_bsr_rts`: Short code fragment that copies 852 to a fixed memory address, then copies 397 to a fixed memory address, then copies 677 to a fixed memory address, then calls another routine, then returns.
- `frag_move_bsr_rts_2`: Short code fragment that copies 2147 to a fixed memory address, then copies 1841 to a fixed memory address, then copies 689 to a fixed memory address, then calls another routine, then returns.
- `update_game_state`: Updates game state variables based on joystick input and timing, manages difficulty progression through multiple counters, and configures blitter parameters for graphics rendering.
- `update_game_state_2`: Updates game state variables based on joystick input and timing, then configures blitter parameters for graphics rendering.
- `update_game_state_3`: Updates game state variables based on joystick input and timing, then configures blitter registers for graphics rendering.
- `update_game_state_4`: Updates game state variables based on joystick input and timing, modifies scroll offsets and display parameters, and manages game progression flags.
- `update_game_state_5`: Updates game state variables based on counters and performs division operations on byte values from global variables, then calls another routine with calculated data pointers.
- `configure_blitter_operations`: Saves register contents to stack, waits for blitter completion, sets up multiple blitter source and destination pointers with calculated addresses, and configures blitter control registers for different drawing operations.
- `format_number_to_string`: Converts three numeric values into character representations and stores them in a string buffer for display.
- `frag_movem_movea_adda_move_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to a0, then adds 541 to a0, then copies 32 to a fixed memory address, then copies a0 to a fixed memory address, then copies 256 to a fixed memory address, then copies 0 to a fixed memory address, then copies 2822 to a fixed memory address, then restores saved registers from the stack, then returns.
- `adjust_coordinates`: Processes coordinate data by comparing values in two arrays, adjusting elements based on comparison results, and updating memory locations with modified coordinate values through bit manipulation and arithmetic operations.
- `initialize_blitter_and_counters`: Sets up blitter parameters including addresses, sizes, and control registers while initializing several global counters to zero.
- `frag_bsr_move_rts_2`: Short code fragment that calls another routine, then copies a0 to a fixed memory address, then copies a1 to a fixed memory address, then copies 12 to a fixed memory address, then copies 40 to a fixed memory address, then copies an address to a fixed memory address, then copies 2544 to a fixed memory address, then copies 0 to a fixed memory address, then copies 7170 to a fixed memory address, then returns.
- `convert_number_to_string`: Converts a 16-bit number into a five-character string representation by extracting digits from thousands, hundreds, tens, and units places and storing them in consecutive memory locations.
- `configure_blitter_operations_2`: Configures blitter registers with specific values for graphics rendering, including modulo settings, control registers, and transfer sizes.
- `configure_blitter`: Sets up multiple blitter register fields with specific values, including modulo settings, control registers, and transfer size, then returns.
- `calculate_table_offsets`: Calculates memory offsets by multiplying an input value with a constant, adds the result to base addresses, and stores the calculated values in a table structure for later use in graphics operations.
- `write_vertex_data`: Writes calculated word values to specific offsets in a table structure based on a lookup index, using multiple base addresses and a fixed multiplication factor.
- `setup_graphics_transfer`: Sets up blitter parameters for graphics data transfers based on input values, configures memory pointers and blitter control registers.
- `update_and_swap_values`: Updates global counters and swaps 32-bit values in global variables while storing results at specific table offsets.
- `process_graphics_data`: Processes byte data from a buffer by dividing values to calculate indices, uses these indices to access memory locations, and configures blitter registers for drawing operations based on the processed data.
- `iterate_graphics_processing`: Processes multiple bytes of graphics data in a loop, calculates memory addresses using division and multiplication operations, configures blitter registers for drawing operations, and advances pointers through memory regions.
- `render_text_glyphs`: Processes character codes from a string to configure blitter parameters for rendering text glyphs, updating display memory pointers and blitter control registers accordingly.
- `swap_and_store_values`: Swaps two 32-bit values in global variables and stores the swapped results at specific offsets in a table.
- `swap_and_store_values_2`: Swaps two 32-bit values in global variables and stores the swapped results at specific offsets in another table.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `frag_bsr_btst_bne_beq_rts`: Short code fragment that calls another routine, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then calls another routine, then tests bit 6 of a fixed memory address, then branches if the last result was zero, then returns.
- `wait_for_button_release`: Waits for a button to be released by polling a hardware register until the button bit is clear, then returns.
- `initialize_music_system`: Walks through a byte table to build a pointer table for music samples, clears several global variables, and resets various audio counters.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_channels`: Processes audio channel data by updating sample pointers and lengths for four sound channels based on calculated indices and loop counters, then returns.
- `update_object_state`: Processes object state data from a structure, updates position and velocity fields based on lookup table values, and writes results to another structure.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `process_input_data`: Reads input data from a structure, processes it based on control values, updates position and counter fields, and writes results to another structure.

### Crazy Challenge (19xx)(CKB).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
