# Game 3500 (E.T.'s Rugby League): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03500). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 36 of 36 named (0 author symbols), 36 of 36 described (6 by rule, 28 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 34 of 34 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 10240 bytes, 15 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1555 of 1561 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 8154 |
| code | 6266 |
| data | 825252 |
| padding | 8 |
| unused | 61440 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| E.T.'s Rugby League.adf:bootblock | 1 | 35 | 1 |
| logged disk load from E.T.'s Rugby League.adf boot-block load | 1 | 107 | 2 |
| logged disk load from E.T.'s Rugby League.adf boot-block load | 1 | 435 | 18 |
| E.T.'s Rugby League.adf bytes 245248-255488 (code never loaded by a reference run) | 1 | 984 | 15 |

## Cases handled

- E.T.'s Rugby League/E.T.'s Rugby League.adf: custom-format (non-DOS) disk, 1636 non-empty blocks: matched against track-loaded segments
- E.T.'s Rugby League/E.T.'s Rugby League.adf: the boot block reads 4096 bytes from disk offset 856064 to an address (found statically); recovered as a unit
- E.T.'s Rugby League/E.T.'s Rugby League.adf: the boot block reads 9216 bytes from disk offset 236032 to an address (found statically); recovered as a unit
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- E.T.'s Rugby League/E.T.'s Rugby League.adf: bytes 245248-255488 look like code no run loaded: recovered position-independently (load address unknown)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: 3 logged loads cover 23552 bytes of the image
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 2-460 (bytes 1024-236032) outside the filesystem hold data, not code (opcode density 0.011, entropy 7.19)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 500-913 (bytes 256000-467968) outside the filesystem hold data, not code (opcode density 0.032, entropy 4.54)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 926-1093 (bytes 474112-560128) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.00)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1111-1211 (bytes 568832-620544) outside the filesystem hold data, not code (opcode density 0.050, entropy 5.68)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1214-1219 (bytes 621568-624640) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.03)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1221-1226 (bytes 625152-628224) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.45)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1229-1234 (bytes 629248-632320) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.47)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1237-1242 (bytes 633344-636416) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.79)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1245-1250 (bytes 637440-640512) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.75)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1253-1258 (bytes 641536-644608) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.51)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1261-1266 (bytes 645632-648704) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.29)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1269-1274 (bytes 649728-652800) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.71)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1277-1282 (bytes 653824-656896) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.71)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1285-1290 (bytes 657920-660992) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.63)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1293-1299 (bytes 662016-665600) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.49)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1301-1311 (bytes 666112-671744) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.66)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1313-1457 (bytes 672256-746496) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.02)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1688-1706 (bytes 864256-873984) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.76)
- E.T.'s Rugby League/E.T.'s Rugby League.adf: blocks 1708-1759 (bytes 874496-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.42)
- 2 packed track region(s), 79872 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### E.T.'s Rugby League.adf:bootblock

- `initialize_boot_block`: Sets up hardware registers for boot block initialization, allocates memory, performs I/O operations, and calls a function pointer through the allocated memory.

### logged disk load from E.T.'s Rugby League.adf boot-block load

- `shift_and_store_bits`: Performs a bit shift operation on a value while decrementing a counter, stores the shifted result in memory, and returns the updated value.
- `bit_shift_loop`: Performs a bit shift operation on a value while decrementing a counter, and calls another routine when the shifted value becomes zero, continuing until the counter reaches zero.

### logged disk load from E.T.'s Rugby League.adf boot-block load

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `handle_interrupt_request`: Saves multiple registers to the stack, reads bytes from two hardware addresses, performs bitwise operations on the data, writes results to other hardware addresses, calls a routine through a function pointer, sets an interrupt request value, and then restores registers before returning.
- `frag_ori_moveq_sub_bne_andi_rts`: Short code fragment that merges 64 into a fixed memory address, then loads 32 into d0, then subtracts 1 from d0, then branches if the last result was not zero, then masks a fixed memory address with 191, then returns.
- `update_game_state`: Updates game state variables based on conditional checks, modifies hardware registers related to interrupts and coprocessors, and calls several routines with different parameters.
- `setup_video_dma`: Initializes video display and DMA settings, configures interrupt masks, sets up memory pointers for graphics data, and performs multiple hardware register writes to control the Amiga's custom chips.
- `frag_lea_moveq_clr_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a2, then loads 7 into d0, then clears the memory at a2 (advancing a2), then clears the memory at a1 (advancing a1), then clears the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `frag_move_rts`: Short code fragment that copies 32 to a fixed memory address, then returns.
- `copy_word_arrays`: Copies word-sized data between four memory arrays using a loop that decrements a counter and advances pointers through each array.
- `update_animation_data`: Updates animation data by copying 160 bytes from four source structures to their respective destination structures, then copies 48 words from a table to a global structure.
- `copy_struct_data`: Copies 160 bytes of data from four source structures to their respective destination structures, then copies 48 words from a table to the end of each structure.
- `copy_sprite_data`: Copies sprite attribute data from a table into a destination buffer, advancing through the table and buffer with calculated offsets.
- `frag_move_rts_bra`: Short code fragment that copies a0 to a fixed memory address, then returns, then jumps to another point in the code.
- `initialize_graphics`: Sets up graphics hardware registers including display modes, bitplane configurations, and copper list pointers while waiting for a specific vertical retrace position.
- `setup_copper_list_entry`: Writes a series of words to memory starting at address in a0, including values from register d7 and its swapped half, followed by a final 32-bit value.
- `build_copper_list`: Constructs a copper list by writing pairs of words to memory, including timing values and data for display processing with a loop that iterates 16 times.
- `setup_copper_list_entry_2`: Writes a series of words to memory starting at address in a0, including values from register d7 and its swapped half, followed by a final 32-bit value.
- `build_simple_copper_list`: Creates a simplified copper list structure by writing alternating word values to memory, using a loop that iterates 16 times and sets up display timing parameters.
- `infinite_loop`: Contains an infinite loop that never exits, effectively making this routine do nothing except consume processor cycles.

### E.T.'s Rugby League.adf bytes 245248-255488 (code never loaded by a reference run)

- `initialize_display_parameters`: Initializes various display-related parameters by writing constants to memory locations, sets up global variables, and calls multiple routines for display setup.
- `initialize_render_states`: Sets up multiple render state values by writing specific constants to memory locations, then calls a rendering function with different parameters based on a condition.
- `handle_game_state`: Checks game state conditions and calls rendering functions based on specific state values, performing different actions depending on whether certain flags are set.
- `process_string_data`: Processes character data from a table by examining specific bytes and modifying them based on comparison results, with multiple conditional branches that modify the same memory locations.
- `decrement_and_render`: Decrements a global counter, calls a rendering function, sets a global variable to 255, and then calls another rendering function before returning.
- `calculate_display_parameters`: Calculates various display-related parameters by performing bit operations on input values, storing results in memory locations and calling a function at the end.
- `copy_table_entries`: Copies byte values from a table to specific memory locations using calculated offsets based on a global variable, performing two separate byte copies.
- `copy_table_data`: Copies data from a table to specific memory locations using calculated offsets based on a global variable, performing word and byte copies from different table positions.
- `frag_move_rts_2`: Short code fragment that copies 87 to a fixed memory address (6 times in a row), then returns.
- `set_color_registers`: Sets specific color register values to a fixed byte value and clears a global word variable, then returns immediately.
- `clear_memory_block`: Clears a block of memory by writing zeros to a sequence of bytes starting at a fixed address, repeating a specific number of times.
- `unpack_and_process_data`: Unpacks data from memory locations into global variables, performs conditional lookups based on unpacked values, and calls rendering functions with different parameters.
- `update_display_colors`: Updates color registers by copying values from memory locations, calling rendering functions conditionally based on flag states, and setting up color data for display output.
- `pack_coordinates`: Combines scaled X and Y coordinates with a byte value into a packed 32-bit word and stores it at the memory location pointed to by a0.
- `unpack_coordinates`: Extracts scaled X and Y coordinates, a byte value, and a lookup index from a packed 32-bit word, then performs a conditional lookup based on the index to set a value at offset 24 of the structure in a1.
