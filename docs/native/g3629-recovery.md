# Game 3629 (filou (1993)(h. mueller)[h titan]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03629). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 23 of 23 named (0 author symbols), 23 of 23 described (6 by rule, 17 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 813 of 813 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 944 of 944 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 181777 |
| code | 3910 |
| data | 25682 |
| padding | 10327 |
| unused | 679424 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Filou (1993)(H. Mueller)[h Titan].adf:Filou | 2 | 934 | 22 |
| Filou (1993)(H. Mueller)[h Titan].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- 8 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- filou (1993)(h. mueller)[h titan]/Filou (1993)(H. Mueller)[h Titan].adf: blocks 212-212 (bytes 108544-109056) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.81)
- filou (1993)(h. mueller)[h titan]/Filou (1993)(H. Mueller)[h Titan].adf: blocks 215-215 (bytes 110080-110592) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- filou (1993)(h. mueller)[h titan]/Filou (1993)(H. Mueller)[h Titan].adf: blocks 217-217 (bytes 111104-111616) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.54)
- filou (1993)(h. mueller)[h titan]/Filou (1993)(H. Mueller)[h Titan].adf: blocks 883-884 (bytes 452096-453120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.64)
- filou (1993)(h. mueller)[h titan]/Filou (1993)(H. Mueller)[h Titan].adf: blocks 1154-1154 (bytes 590848-591360) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- filou (1993)(h. mueller)[h titan]/Filou (1993)(H. Mueller)[h Titan].adf: blocks 1157-1157 (bytes 592384-592896) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Filou (1993)(H. Mueller)[h Titan].adf:Filou

- `initialize_game_state`: Initializes global variables related to graphics and hardware registers, sets up display timing loops, configures interrupt servers, and prepares rendering parameters for the game's main loop.
- `frag_btst_bne_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `handle_input_event`: Reads a byte from a global variable, processes it through another routine three times, then writes it back to a table.
- `frag_move_cmp_beq_rts`: Short code fragment that copies the memory at an offset from a6 to d1, then compares d1 with the memory at an offset from a6, then branches if the last result was zero, then returns.
- `handle_input_state`: Reads joystick fire button state from a hardware register and updates a global variable to track input state, setting it to 1 when the button is pressed and -1 otherwise.
- `update_joystick_position`: Reads joystick movement data from hardware registers and updates global variables representing horizontal and vertical positions based on button states.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `update_sprite_data`: Updates sprite position and attribute data by copying values from a source table to multiple destination locations, with loop iterations handling different sprite components.
- `wait_for_bit_and_init`: Waits for a specific bit in a memory location to become set, then initializes several variables including a counter, a data pointer, and a word value.
- `setup_rendering_attributes`: Configures rendering attributes for graphical elements by calculating memory addresses and setting up color and positioning data in a structure.
- `setup_rendering_attributes_2`: Configures rendering attributes for graphical elements similar to routine R3 but with different memory addressing and parameter values.
- `copy_sprite_patterns`: Copies sprite pattern data from one memory location to another using a loop that processes multiple pattern entries and updates pointers accordingly.
- `manage_object_queue`: Manages an object queue by updating object properties, writing data to memory locations, and advancing the queue pointer with bounds checking.
- `process_audio_channels`: Processes audio channel data by decrementing counters, setting up DMA registers when counters reach zero, and managing audio playback state transitions.
- `frag_move_bsr_tst_bne_rts`: Short code fragment that copies 1178 to a fixed memory address, then copies 142 to a fixed memory address, then copies 31 to a fixed memory address, then copies 1 to a fixed memory address, then calls another routine (2 times in a row), then tests a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_move_bra`: Short code fragment that copies 1114 to a fixed memory address, then copies 142 to a fixed memory address, then copies 31 to a fixed memory address, then copies 1 to a fixed memory address, then jumps to another point in the code.
- `update_sprite_positions`: Updates sprite position values by comparing current and target values, adjusting position components based on comparison results, and managing update flags.
- `initialize_rendering_loop`: Sets up a nested loop structure that calls another routine 9 times for each of 8 iterations, using registers to track loop counters and incrementing values in a controlled manner.
- `configure_rendering_attributes`: Initializes rendering parameters by setting up memory addresses and loop counters, then calls another routine with incremented values while checking against specific limits before continuing the loop.
- `update_tile_status`: Updates tile status in a grid based on coordinates and neighboring tile values, setting specific tiles to value 6 or 7 depending on conditions, and manages a global pointer to the current tile being processed.
- `update_memory_entries`: Processes 89 memory entries starting at a global variable address, clearing entries with value 6 and setting entries with value 7 to 1, then increments the pointer and repeats until all entries are processed.
- `compare_sprite_data`: Compares two blocks of sprite data byte by byte, returning a success flag when all bytes match or a failure flag when any byte differs.

### Filou (1993)(H. Mueller)[h Titan].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
