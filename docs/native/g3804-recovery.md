# Game 3804 (giddy! ii (1994)(p. ruston)[t +2 gin]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03804). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911216 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 46 of 46 named (0 author symbols), 46 of 46 described (18 by rule, 26 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 514 of 514 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1083 of 1084 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5766 |
| code | 4406 |
| data | 378567 |
| padding | 749 |
| unused | 521728 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Giddy! II (1994)(P. Ruston)[t +2 GIN].adf:Game | 1 | 476 | 19 |
| Giddy! II (1994)(P. Ruston)[t +2 GIN].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 590 | 26 |

## Cases handled

- runtime segment an address: 290 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_hardware_2` (runtime segment an address): Opens the graphics library by name and closes it again, and writes to the audio, DMA and interrupt control registers, including the copper location registers. Kind: disk hardware check. Pass path: ADKCON register read/write operations continue normally, no trap or error condition triggered.

## Functions


### Giddy! II (1994)(P. Ruston)[t +2 GIN].adf:Game

- `setup_graphics`: Allocates memory for graphics resources, opens system libraries, and initializes display hardware registers.
- `frag_tst_beq_move_movea_jsr_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then copies a location in the code to d1, then copies a location in the code to a6, then calls another routine, then copies a fixed memory address to a6, then calls another routine, then copies a location in the code to a1, then calls another routine, then returns.
- `frag_move_movea_jsr_tst_rts`: Short code fragment that copies d0 to d7, then copies a location in the code to a6, then calls another routine, then tests d0, then returns.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `write_text_via_dos`: Returns immediately when a control word is zero.
- `check_version_and_call`: Checks if a version number at offset 20 in a structure is less than or equal to 37, and if so, calls a routine through a function pointer obtained from a global variable.
- `initialize_interrupts`: Sets up interrupt vectors and enables DMA and interrupts, then performs a series of operations including copying data and calling routines before returning.
- `set_trap_vector`: Sets a trap vector in the interrupt table to point to a specific handler address and then executes a trap instruction.
- `initialize_hardware`: Sets up custom hardware registers including display and interrupt control, configures DMA and bitplane settings, and initializes global variables related to system state and coprocessor pointers.
- `initialize_game`: Initializes game hardware registers, sets up interrupt handlers, and configures sound and display control registers before starting the main game loop.
- `frag_move_lea_movea_jmp`: Short code fragment that copies a1 to the stack, then copies 20153 to a fixed memory address, then loads the address of a location in the code into a1, then copies a1 to a fixed memory address, then copies 20085 to a fixed memory address, then copies the top of the stack to a1, then copies 96 to a fixed memory address, then jumps to another point in the code.
- `frag_movem_clr_move_mulu_movea_lea_bra`: Short code fragment that saves registers on the stack, then clears d2, then copies d7 to d2, then copies d2 to d0, then multiplies d0 by 512 (unsigned), then copies d6 to d1, then copies a6 to a0, then loads the address of a location in the code into a1, then copies 7 to the memory at a1, then jumps to another point in the code.
- `process_coordinates`: Processes coordinate values by performing multiplication, division, and subtraction operations to calculate offsets, then copies data from a source buffer to a destination buffer using these calculated values.
- `frag_move_cmpi_bne_eori_not_rts`: Short code fragment that copies the memory at an offset from a2 to d0, then compares d0 with 95, then branches if the last result was not zero, then exclusive-ors 12904 into a fixed memory address, then compares d0 with 93, then branches if the last result was not zero, then exclusive-ors 51571 into a fixed memory address, then inverts d0, then returns.
- `frag_movea_clr_cmpi_beq_subq_move_add_b`: Short code fragment that copies a location in the code to a1, then clears d6, then clears d7, then compares d4 with 0, then branches if the last result was zero, then subtracts 1 from d4, then copies the memory at a1 (advancing a1) to d7, then adds d7 to d6, then jumps to another point in the code, then returns.
- `copy_byte_block`: Copies bytes from one memory location to another using a counter loaded from a source pointer, decrementing the counter and branching back until it reaches zero.
- `copy_byte_block_2`: Copies bytes from a source buffer to a destination buffer using a counter loaded from a source pointer, decrementing the counter and branching back until the counter reaches zero.
- `decode_compressed_data`: Decompresses compressed data from a source buffer to a destination buffer using a custom decoding algorithm that processes bits and bytes in a loop, with conditional branches based on bit values.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Giddy! II (1994)(P. Ruston)[t +2 GIN].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_movea_moveq_btst_beq_lea_jsr`: Short code fragment that copies 16384 to a fixed memory address, then copies a fixed memory address to a6, then loads 0 into d0, then tests bit 0 of the memory at an offset from a6, then branches if the last result was zero, then loads the address of a location in the code into a5, then calls another routine, then jumps to another point in the code.
- `initialize_hardware_2`: Opens the graphics library by name and closes it again, and writes to the audio, DMA and interrupt control registers, including the copper location registers.
- `frag_movem_lea_move_andi_btst_beq_rte`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a6, then copies the memory at an offset from a6 to d0, then masks d0 with 63613, then copies d0 to the memory at an offset from a6, then tests bit 5 of d0, then branches if the last result was zero, then restores saved registers from the stack, then returns from the exception.
- `run_next_nonempty_table_handler`: Steps a stored index backwards through a table in eight-byte steps until it finds a non-empty entry, calls that entry as a routine, and advances the index by one entry when the result is non-zero.
- `adjust_word_nibbles_toward_target`: Walks two arrays of 16-bit words together and increments each word of the first array in steps of 1, 16 or 256 until each nibble matches the second array.
- `decrement_word_nibbles_toward_target`: Walks two arrays of 16-bit words together and decrements each word of the first array in steps of 1, 16 or 256 until each nibble matches the second array.
- `update_display_sequence`: Sets up display-related variables and hardware registers when the sequence state is not 2, then advances the frame counter and refreshes the display.
- `update_mouse_movement`: Reads mouse movement data from a hardware register, calculates a difference value, clamps it to a range, and updates a global variable with the clamped result.
- `frag_cmpi_beq_move_bne_lea`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was zero, then copies 1 to a fixed memory address, then compares a fixed memory address with 0, then branches if the last result was not zero, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a2.
- `copy_text_glyphs_to_buffer`: Copies the glyph columns for each character of a null-terminated text string into a buffer, moving to a new line after a fixed number of characters.
- `advance_frame_counter`: Increments a frame counter and when it reaches a threshold, copies data from one table to another, updates step counters, and resets the frame counter.
- `refresh_table_slots`: Calls a nibble adjustment routine, copies data from a source table to a destination table, and increments a step counter after each operation.
- `update_game_state`: Updates game state variables based on input conditions and counter values, performing conditional logic to advance through different game states and reset various game parameters.
- `frag_btst_beq_clr_bra`: Short code fragment that tests bit 6 of a fixed memory address, then branches if the last result was zero, then clears d0, then jumps to another point in the code.
- `reset_game_state`: Resets various game state variables and performs conditional logic based on a counter value and another global variable.
- `frag_clr_addq_cmpi_blt_bsr_move_ble_bra`: Short code fragment that clears d0, then adds 1 to a fixed memory address, then compares a fixed memory address with 4, then branches if the comparison was less, then calls another routine, then copies 0 to a fixed memory address, then compares a fixed memory address with 15, then branches if the comparison was less or equal, then copies 0 to a fixed memory address, then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_character_data`: Copies character data from a source buffer to a destination table while advancing a counter, and resets the counter and updates the destination pointer when it reaches 81.
- `populate_character_data`: Uses a lookup table to copy character data from one memory area to another, with offsets calculated based on input values, and advances a pointer through the destination buffer.
- `frag_move_rts`: Short code fragment that copies 1 to a fixed memory address, then returns.
- `frag_addq_cmpi_blt_lea_move_jsr_addi_rt`: Short code fragment that adds 1 to a fixed memory address, then compares a fixed memory address with 4, then branches if the comparison was less, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 4 to d5, then calls another routine, then adds 1 to a fixed memory address, then copies 0 to a fixed memory address, then returns.
- `frag_lea_move_jsr_addi_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 4 to d5, then calls another routine, then adds 1 to a fixed memory address, then returns.
- `open_resource_library`: Opens system libraries and resources using library calls, accesses hardware registers for input devices, and handles resource management operations.
- `allocate_memory_block`: Allocates a memory block using a library call and returns immediately without performing any other operations.
- `initialize_audio_system`: Initializes audio system registers by setting multiple volume controls to zero, clearing several global variables, and writing specific values to hardware control registers related to sound output and disk status.
- `frag_sf_lea_clr_move_rts`: Short code fragment that clears a fixed memory address, then loads the address of a fixed memory address into a0, then clears the memory at an offset from a0 (4 times in a row), then copies 15 to a fixed memory address, then returns.
