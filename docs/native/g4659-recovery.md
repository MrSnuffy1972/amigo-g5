# Game 4659 (patience iii (1993)(h. mueller)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04659). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 34 of 34 named (0 author symbols), 34 of 34 described (3 by rule, 24 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1234 of 1234 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1809 of 1809 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 373626 |
| code | 6926 |
| data | 40021 |
| padding | 867 |
| unused | 479744 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Patience III (1993)(H. Mueller).adf:PatienceIII | 2 | 1798 | 32 |
| Patience III (1993)(H. Mueller).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- 5 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- patience iii (1993)(h. mueller)/Patience III (1993)(H. Mueller).adf: blocks 212-212 (bytes 108544-109056) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.81)
- patience iii (1993)(h. mueller)/Patience III (1993)(H. Mueller).adf: blocks 215-215 (bytes 110080-110592) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- patience iii (1993)(h. mueller)/Patience III (1993)(H. Mueller).adf: blocks 217-217 (bytes 111104-111616) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.54)
- patience iii (1993)(h. mueller)/Patience III (1993)(H. Mueller).adf: blocks 883-884 (bytes 452096-453120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.64)
- patience iii (1993)(h. mueller)/Patience III (1993)(H. Mueller).adf: blocks 1154-1154 (bytes 590848-591360) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- patience iii (1993)(h. mueller)/Patience III (1993)(H. Mueller).adf: blocks 1157-1157 (bytes 592384-592896) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Patience III (1993)(H. Mueller).adf:PatienceIII

- `initialize_graphics_system`: Sets up graphics configuration parameters including color masks and timing values, waits for a specific hardware register value, and copies data to multiple global variables.
- `frag_btst_bne_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `handle_input_event`: Reads a byte from a global variable, processes it through another routine three times, then writes it back to a table.
- `frag_move_cmp_beq_rts`: Short code fragment that copies the memory at an offset from a6 to d1, then compares d1 with the memory at an offset from a6, then branches if the last result was zero, then returns.
- `process_joystick_and_copy_data`: Calls another routine, copies sprite data from a table to buffers using a loop that processes 32 blocks, updates joystick position variables based on hardware register values, and manages a global flag based on disk status and other conditions.
- `update_joystick_positions`: Reads mouse and joystick movement data from hardware registers and updates global variables representing horizontal and vertical positions based on the input.
- `copy_input_data`: Copies 16 blocks of 4-word data from a source table to two destination areas, advancing pointers through each block and decrementing a loop counter.
- `setup_blitter_dma`: Configures blitter DMA registers with fixed values and waits for a bit to clear before returning.
- `configure_blitter_registers`: Sets up multiple blitter hardware registers with calculated source and destination addresses based on input parameters, then configures bitplane and control settings for graphics operations.
- `setup_blitter_graphics`: Calculates a source address offset using division and multiplication operations, then sets up blitter registers with specific values for graphics rendering operations.
- `process_byte_stream`: Processes bytes from memory using a subroutine, adjusts parameters after each byte, and performs bounds checking on the processed value.
- `handle_byte_stream`: Processes a stream of bytes from memory until it encounters a specific end marker, calling another routine when the marker is found.
- `scan_and_execute`: Scans through a memory area for a specific marker value and executes a routine if the marker is found, looping through multiple locations before calling another routine.
- `initialize_graphics_config`: Initializes graphics configuration parameters in a structure referenced through a6, including color masks, bitplane settings, and timing values, then processes a table until termination condition is met.
- `initialize_graphics`: Sets up graphics configuration parameters including color masks and timing values, waits for a specific hardware register value, and copies data to a memory area.
- `swap_table_elements`: Swaps pairs of bytes in a lookup table by reading values from a hardware register, performing multiple iterations of swapping operations.
- `process_byte_sequence`: Tests a global variable for positive value, then calls another routine multiple times with different string pointers when the test passes.
- `frag_tst_bmi_moveq_move_bpl_subq_andi_r`: Short code fragment that tests the memory at a0, then branches if the last result was negative, then loads 0 into d0, then copies the memory at a0 (advancing a0) to d0, then branches if the last result was not negative, then subtracts 2 from a0, then masks the memory at a0 with 63, then returns.
- `update_card_table`: Updates a card table by iterating through data, comparing values, and setting up pointers in a loop.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_counter_and_branch`: Increments a byte value at a fixed memory location and compares it with another byte value.
- `update_audio_registers`: Updates audio period registers based on global state and table lookups, then returns the modified value.
- `process_card_data`: Processes card information from a structure, updates coordinates and flags based on card values, then stores results in global variables and hardware registers.
- `process_input_value`: Processes a value from a structure, performs bit manipulation and comparisons, then stores a result from a table into another global variable.
- `copy_word_value`: Copies a 16-bit word value from a structure to another global variable, then performs bit manipulation and comparisons on data from tables before updating a game state word in memory.
- `process_card_value`: Processes a card value from a structure, updates a counter based on byte operations and comparisons, then stores a result from a table into another structure field.
- `process_value`: Processes a value from a structure, updates a counter based on byte operations and comparisons, then stores a result from a table into another global variable.
- `calculate_value`: Processes a byte value from a global structure, applies bit manipulation and multiplication operations, then stores the result in another global variable.
- `process_input_byte`: Processes a byte value from a structure, applies bit manipulation and multiplication operations, then stores the result in another global variable.
- `store_input_byte`: Extracts the lower nibble of a byte value from a structure and stores it in a global variable at offset 18.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `frag_movem_lea_move_addq_cmp_bne_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then copies the memory at a0 to d0, then adds 4 to d0, then copies the memory at a0 to d1, then compares d1 with d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.

### Patience III (1993)(H. Mueller).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
