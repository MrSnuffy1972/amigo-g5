# Game 3033 (carnage (1992)(zeppelin)[cr ministry]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03033). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905376 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 46 of 46 named (0 author symbols), 46 of 46 described (18 by rule, 25 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 663 of 663 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 270336 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1046 of 1046 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 24161 |
| code | 6400 |
| data | 828735 |
| unused | 46080 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Carnage (1992)(Zeppelin)[cr Ministry].adf:bootblock | 1 | 126 | 8 |
| logged disk load from Carnage (1992)(Zeppelin)[cr Ministry].adf boot-block load | 1 | 116 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 144 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 660 | 24 |
| Carnage (1992)(Zeppelin)[cr Ministry].adf blocks 610-1137 (bytes 312320-582656) (code never loaded by a reference run, load address unknown) | 1 | 0 | 0 |

## Cases handled

- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: custom-format (non-DOS) disk, 1668 non-empty blocks: matched against track-loaded segments
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: the boot block reads 28160 bytes from disk offset 675840 to an address (found statically); recovered as a unit
- runtime segment an address: 135 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 498 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 address
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: 1 logged loads cover 28160 bytes of the image
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: 5 track-loaded blocks found in recovered runtime segments and classified through them
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: blocks 2-383 (bytes 1024-196608) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.44)
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: blocks 386-386 (bytes 197632-198144) outside the filesystem hold data, not code (opcode density 0.027, entropy 5.24)
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: blocks 390-584 (bytes 199680-299520) outside the filesystem hold data, not code (opcode density 0.012, entropy 5.34)
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: blocks 586-597 (bytes 300032-306176) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.86)
- Carnage (1992)(Zeppelin)[cr Ministry].adf blocks 610-1137 (bytes 312320-582656) (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 0 of 270336 bytes decode)
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: blocks 1162-1319 (bytes 594944-675840) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.34)
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: blocks 1375-1548 (bytes 704000-793088) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.15)
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: blocks 1557-1559 (bytes 797184-798720) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.10)
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: blocks 1572-1572 (bytes 804864-805376) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.81)
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: blocks 1574-1574 (bytes 805888-806400) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.81)
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: blocks 1579-1697 (bytes 808448-869376) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.25)
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: blocks 1699-1728 (bytes 869888-885248) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.13)
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: blocks 1748-1748 (bytes 894976-895488) outside the filesystem hold data, not code (opcode density 0.008, entropy 1.69)
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: blocks 1751-1751 (bytes 896512-897024) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.43)
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: blocks 1754-1754 (bytes 898048-898560) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.55)
- carnage (1992)(zeppelin)[cr ministry]/Carnage (1992)(Zeppelin)[cr Ministry].adf: blocks 1759-1759 (bytes 900608-901120) hold ASCII text
- 1 never-loaded region(s), 270336 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_device` (bootblock): Clears a 112-byte buffer, calls exec.AddPort, opens a trackdisk device, and sets up I/O request structures with specific command codes and addresses before calling exec.DoIO. Kind: trackdisk.device I/O check. Pass path: The routine successfully opens and performs DoIO with trackdisk.device, indicating a valid disk check or track access validation. If the check passes, it continues execution without entering the infinite loop (which is triggered on failure).
- `execute_io_request` (bootblock): Sets up an I/O request structure with command code 2, address values, and calls exec.DoIO, then executes a function through a jump table and restores register state. Kind: trackdisk.device I/O check. Pass path: The call to execute_io_request (via exec.DoIO) succeeds and loads data from a specific track, indicating a track-based verification check..
- `initialize_player_names` (runtime segment an address): Clears a block of memory locations, initializes specific global variables with fixed values, and sets up character data for player names based on a global variable's value using conditional assignments. Kind: copy protection. Pass path: The routine initializes player names and sets up data based on _DAT_0001cce0 value (1, 2, or 3). A successful check would proceed through the conditional branches where each player's name is initialized, indicating a valid game state. The c.

## Functions


### Carnage (1992)(Zeppelin)[cr Ministry].adf:bootblock

- `copy_boot_data`: Copies boot data from a source location to a target address using a loop that continues until all bytes are transferred, then jumps to the target address.
- `setup_coprocessor`: Initializes coprocessor registers with specific values, sets up DMA control registers, waits for raster lines, and configures memory addresses before jumping to different code segments.
- `initialize_device`: Clears a 112-byte buffer, calls exec.AddPort, opens a trackdisk device, and sets up I/O request structures with specific command codes and addresses before calling exec.DoIO.
- `frag_move_bra`: Short code fragment that copies 3840 to a fixed memory address, then jumps to another point in the code.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `frag_movea_lea_clr_cmpa_blt_jmp`: Short code fragment that copies 69632 to a0, then loads the address of a location in the code into a1, then clears the memory at a0 (advancing a0), then compares a0 with a1, then branches if the comparison was less, then jumps to the address in a5.
- `execute_io_request`: Sets up an I/O request structure with command code 2, address values, and calls exec.DoIO, then executes a function through a jump table and restores register state.
- `execute_trap_zero`: Invokes trap zero and returns the combined 64-bit value of the D0 and D1 registers.

### logged disk load from Carnage (1992)(Zeppelin)[cr Ministry].adf boot-block load

- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `fetch_and_mask`: Reads a 16-bit value from a table using an index derived from a register, applies a bitwise AND with another register's value, and returns the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bsr_rts`: Short code fragment that calls another routine, then returns.
- `frag_bsr_movea_jsr_rts`: Short code fragment that calls another routine, then copies a fixed memory address to a1, then calls the routine whose address is in a1, then calls another routine, then returns.
- `frag_move_movea_jsr_rts`: Short code fragment that copies 32767 to the memory at an offset from a5 (2 times in a row), then copies a fixed memory address to a fixed memory address (2 times in a row), then copies a fixed memory address to the memory at an offset from a5 (2 times in a row), then copies a fixed memory address to a6, then calls another routine, then returns.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `update_joystick_status_2`: Reads joystick status from CIA registers, performs bitwise AND operation on the values, stores the result in a global variable, and calls another routine twice if a specific bit is set.
- `update_joystick_status`: Updates joystick status by writing zero to a CIA register, performing bitwise AND operations on another CIA register, and calling multiple routines based on a control value.
- `frag_lea_bra`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then jumps to another point in the code.
- `frag_lea_bra_2`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then jumps to another point in the code.
- `frag_lea_bra_3`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then jumps to another point in the code.
- `process_joystick_input`: Reads joystick status from CIA registers based on a control value and sets byte values in memory according to specific bit conditions and shift operations.
- `read_joystick_input`: Reads joystick status from CIA registers, processes bit flags to set byte values in memory based on specific bit conditions and shift operations.
- `frag_lea_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 24 to d7, then copies 0 to the memory at a0 (advancing a0), then loops back while counting down d7, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_subq_bne_rts`: Short code fragment that copies an address to d2, then subtracts 1 from d2, then branches if the last result was not zero, then returns.
- `frag_lea_move_dbra_rts_2`: Short code fragment that loads the address of a fixed memory address into a1, then copies 12799 to d1, then copies 0 to the memory at a1 (advancing a1), then loops back while counting down d1, then returns.
- `initialize_display_buffers`: Clears a block of memory locations, initializes various global variables with specific values, and sets up display buffer configurations by writing fixed values to multiple memory addresses.
- `update_counter_value`: Increments or decrements a counter value based on the state of two global variables, ensuring the counter stays within bounds of 1 to 4.
- `frag_tst_bne_rts_move`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then tests a fixed memory address, then branches if the last result was not zero, then returns, then copies 0 to a fixed memory address, then returns, then copies 1 to a fixed memory address, then returns.
- `update_input_config`: Updates input configuration by writing byte values to a global variable through multiple loops, and performs several rendering and setup operations for keyboard input elements.
- `frag_sub_move_cmp_beq_btst_ror_not_rts`: Short code fragment that subtracts d6 from d6, then copies a fixed memory address to d6, then compares d6 with a fixed memory address, then branches if the last result was zero, then copies d6 to a fixed memory address, then tests bit 0 of d6, then branches if the last result was zero, then rotates d6 right by 1, then inverts d6, then returns.
- `render_text_glyphs`: Renders text glyphs by calculating memory addresses and calling character rendering routines with specific width and height parameters based on control values.
- `render_menu_text`: Renders menu text by calling character overlay routines with specific width, height, and memory address parameters for multiple text strings.
- `render_player_menus`: Initializes and updates player menu text rendering by setting up character positions, calling various rendering routines, updating counters, and managing display states based on conditional checks and loop iterations.
- `render_joystick_label`: Renders joystick label text by calculating memory addresses based on control values and calling character rendering routines with specific positioning parameters.
- `update_counter`: Updates a global counter by adding a value loaded from another global variable, with conditional checks and assignments based on the counter's current value.
- `frag_lea_move_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a2, then copies 5120 to d4, then copies 5 to d5, then calls another routine, then returns.
- `initialize_player_names`: Clears a block of memory locations, initializes specific global variables with fixed values, and sets up character data for player names based on a global variable's value using conditional assignments.
- `frag_lea_move_bsr_rts_2`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a2, then copies 5120 to d4, then copies 4 to d5, then calls another routine, then returns.
- `initialize_character_data`: Initializes character data by copying values from a lookup table at an offset into multiple memory locations with specific byte offsets, using a base address calculation.
- `copy_character_data`: Copies character data from a lookup table at an offset into multiple memory locations with specific byte offsets, using a base address calculation and pointer arithmetic.
- `copy_character_bits`: Copies character data bits from one memory location to another based on bit testing conditions, handling multiple rows and columns with specific offset calculations.
- `adjust_raster_positions`: Adjusts raster position values in two memory areas by comparing current values with target values, incrementing or decrementing based on comparisons, and calling a raster wait routine.
- `decrement_raster_values`: Decrements raster position values in two memory areas by subtracting specific amounts from each bit field, then calls a raster wait routine multiple times.
- `copy_character_glyphs`: Copies character glyph data from a lookup table at an offset into multiple memory locations with specific byte offsets, using address calculations based on character values.
- `overlay_character_glyphs`: Overlays character glyph data from a lookup table at an offset into multiple memory locations with specific byte offsets, performing bitwise OR operations to combine data.
- `wait_for_raster_line_2`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_while_byte_flag_set`: Loops while bit 6 of the byte at offset 2 from a5 is set, then returns.
