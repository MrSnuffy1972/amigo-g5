# Game 901 (rampage (1989)(activision)[cr band][t +1 band]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00901). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903888 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 26 of 26 named (0 author symbols), 26 of 26 described (7 by rule, 19 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 42 of 42 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 5632 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 736 of 736 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 306 |
| code | 2304 |
| data | 9186 |
| padding | 188 |
| unused | 891904 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rampage (1989)(Activision)[cr Band][t +1 Band].adf:bootblock | 1 | 21 | 2 |
| logged disk load from Rampage (1989)(Activision)[cr Band][t +1 Band].adf boot-block load | 1 | 106 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 609 | 21 |
| Rampage (1989)(Activision)[cr Band][t +1 Band].adf bytes 5632-11264 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- rampage (1989)(activision)[cr band][t +1 band]/Rampage (1989)(Activision)[cr Band][t +1 Band].adf: file            (header 5124 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- rampage (1989)(activision)[cr band][t +1 band]/Rampage (1989)(Activision)[cr Band][t +1 Band].adf: the boot block reads 6656 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- runtime segment an address: 42 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rampage (1989)(activision)[cr band][t +1 band]/Rampage (1989)(Activision)[cr Band][t +1 Band].adf: bytes 5632-11264 look like code no run loaded: recovered position-independently (load address unknown)
- rampage (1989)(activision)[cr band][t +1 band]/Rampage (1989)(Activision)[cr Band][t +1 Band].adf: 2 logged loads cover 5632 bytes of the image
- 1 never-loaded region(s), 5632 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rampage (1989)(Activision)[cr Band][t +1 Band].adf:bootblock

- `copy_words_and_jump`: Calls another routine, sets a2 to a stack address, copies 128 long words from a0 to a2 using a dbra loop, and jumps to a Kickstart vector.
- `init_struct_and_clear_colors`: Initializes a structure pointed to by a1 with a couple of words and long values, calls another routine through a vector in a global variable, then calls a Kickstart vector and clears four color registers in the custom chip.

### logged disk load from Rampage (1989)(Activision)[cr Band][t +1 Band].adf boot-block load

- `shift_and_copy_data`: Pops a long word from a0, stores its high byte into a word, sets a condition code bit, rotates the value right, and returns, with a following section that shifts and copies data in a loop.
- `loop_copy_and_dispatch`: Loops decrementing a counter while shifting a value, calling a routine when a bit becomes clear, and otherwise dispatches based on a counter value to copy bytes from a source into a destination until a range is reached.
- `frag_movem_lea_move_adda_dbra_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 31 to d0, then copies the memory at a0 (advancing a0) to the memory at an offset from a1, then adds 4 to a1, then loops back while counting down d0, then restores saved registers from the stack, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_and_clear_flags`: Waits in a loop until a word at offset -31368 of a1 is nonzero and then has bit 7 clear, clears that word, zeroes eleven consecutive words starting at offset -31426, and sets bit 7 of six more words before returning.
- `frag_move_jsr_lsl_lea_rts`: Short code fragment that copies d1 to the stack, then calls wait_and_clear_flags, then copies d0 to d1, then shifts d1 left by 1, then loads the address of the memory at an offset from a1 into a5, then copies an indexed entry from a5 to d0, then copies the top of the stack to d1, then returns.
- `update_state_machine`: Updates a state value based on comparisons with stored values and a flag, then performs conditional operations including clearing and setting specific memory locations depending on the computed state.
- `frag_cmpi_beq_bne_subq_jmp`: Short code fragment that compares d4 with 68, then branches if the last result was zero, then compares d4 with 79, then branches if the last result was not zero, then compares d2 with 0, then branches if the last result was zero, then subtracts 1 from d2, then jumps to another point in the code.
- `frag_cmpi_bne_beq_addq_jmp`: Short code fragment that compares d4 with 78, then branches if the last result was not zero, then compares d2 with 3, then branches if the last result was zero, then adds 1 to d2, then jumps to another point in the code.
- `process_rendering_conditions_2`: Compares word values with a register, updates multiple data fields in memory, and performs conditional operations including loop iterations and memory writes.
- `initialize_collision_flags_2`: Clears a word register and stores a byte value of 1 at a specific offset in a structure, then writes two word values to offsets within the structure.
- `initialize_collision_flags`: Clears a word register and stores a byte value of 1 at a specific offset in a structure, then writes two word values to offsets within the same structure.
- `frag_move_rts`: Short code fragment that copies 0 to an indexed entry from a1, then returns.
- `wait_and_clear_flags_2`: Performs bitwise operations on word registers, reads from memory locations with indexed addressing, and conditionally calls other routines based on register values.
- `update_sprite_coordinates_2`: Copies data from one location to another using indexed addressing, performs several operations on word registers including bit manipulation, and calls a routine with specific parameters.
- `update_sprite_coordinates`: Copies data from one location to another using indexed addressing, performs several operations on word registers including bit manipulation and shifting, and calls multiple routines to update sprite rendering parameters.
- `frag_movem_move_movea_clr_jsr_rts`: Short code fragment that saves registers on the stack, then copies 5352 to d6, then copies 3 to an indexed entry from a1, then copies 28276 to an indexed entry from a1, then copies a fixed memory address to a5, then clears d7, then calls another routine, then copies 1 to the memory at an offset from a1, then restores saved registers from the stack, then returns.
- `update_vsync_data`: Waits for a specific vertical sync condition, then updates multiple data fields including position and state information in a structured memory layout.
- `process_rendering_conditions`: Compares values from a structure with a register, updates word registers based on comparison results, and conditionally calls rendering routines while managing stack data.
- `render_sprite_frames`: Processes sprite rendering by calling a graphics routine with different parameters to draw various sprite elements at specific coordinates.
- `update_object_values`: Calculates and stores scaled values based on input magnitude, updates object position and status fields, then performs conditional arithmetic operations on object coordinates before storing the final result.
- `set_scaled_value`: Selects and stores a scaled value based on the magnitude of an input byte into a memory location with an offset.
- `frag_eor_jsr_move`: Short code fragment that exclusive-ors d5 into d5, then calls another routine, then copies 1 to d5.
- `update_sprite_position`: Updates sprite position and state information by performing arithmetic operations, conditional checks, and calling graphics routines to manage sprite movement and rendering.
- `read_inputs_and_set_audio`: When bit 5 of the input port register is set, increments a counter, stores several processed values of the input and joystick data into a structure, and sets audio volume registers based on button state, then returns via a jump.
