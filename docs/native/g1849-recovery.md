# Game 1849 (f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01849). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903260 top-level bytes in 27 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 23 of 23 units identical (bytes and relocations), 50 raw-word lines |
| Functions named and described | pass, 88 of 88 named (0 author symbols), 88 of 88 described (17 by rule, 48 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 272 of 272 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 146678 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1942 of 1942 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 164701 |
| code | 5568 |
| data | 682632 |
| padding | 4791 |
| unused | 45568 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:library | 3 | 17 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:frontend | 3 | 11 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:type | 2 | 363 | 12 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:GOLLUM! | 3 | 26 | 2 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:assign | 4 | 18 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:f15 | 3 | 16 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:end | 3 | 9 | 1 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:select | 3 | 18 | 2 |
| explode.library | 1 | 402 | 25 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:game | 3 | 19 | 2 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:Crystal | 1 | 166 | 6 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:wait | 2 | 395 | 13 |
| F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:bootblock | 1 | 138 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 167 | 7 |
| logged disk load from F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf | 1 | 120 | 3 |
| logged disk load from F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf | 1 | 16 | 2 |
| logged disk load from F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf | 1 | 28 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| LB.WLD (raw code file, load address unknown) | 1 | 0 | 0 |
| PG.WLD (raw code file, load address unknown) | 1 | 0 | 0 |
| CE.WLD (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- extended reference run ignored: it does not re-execute 214 of the 235 PCs the canonical run logged (131341 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 8 64 KB pages each with >= 29491 distinct PCs executed, 131198 of its 131341 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- 46 first-execution byte records contradict the logged opcode word (first an address): bad captures, the opcode word is used
- prefix run f150 ignored: it logged 131190 PCs the canonical run never executed (not a prefix of it)
- prefix run f75 ignored: it logged 131182 PCs the canonical run never executed (not a prefix of it)
- runtime segment an address (play-rs-m1500): 134 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf: 46 executed PCs whose RAM was overwritten before any snapshot match its bytes 511040-511504: recovered as a unit run at an address
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf: 16 executed PCs whose RAM was overwritten before any snapshot match its bytes 518672-518768: recovered as a unit run at an address
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf: 14 executed PCs whose RAM was overwritten before any snapshot match its bytes 512416-512624: recovered as a unit run at an address
- 6 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 3 first-run image unit(s) in 1 version group(s); 40 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:library: Imploder Library (xfd) stream found by ancient scan (11976 -> 49684 bytes) but only 0 of 16 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:frontend: Imploder Library (xfd) stream found by ancient scan (105648 -> 233072 bytes) but only 0 of 87 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:assign: Imploder Library (xfd) stream found by ancient scan (1912 -> 2960 bytes) but only 0 of 2 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:f15: Imploder Library (xfd) stream found by ancient scan (44668 -> 58984 bytes) but only 0 of 29 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:end: Imploder Library (xfd) stream found by ancient scan (49104 -> 55432 bytes) but only 0 of 23 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:select: Imploder Library (xfd) stream found by ancient scan (121388 -> 165612 bytes) but only 0 of 70 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:game: Imploder Library (xfd) stream found by ancient scan (135324 -> 267092 bytes) but only 0 of 91 sampled windows are in reference RAM: not used
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:F15II(scenario)/LB.WLD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:F15II(scenario)/PG.WLD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:F15II(scenario)/CE.WLD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 3 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 immediate, 1 operand
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf: 3 logged loads cover 0 bytes of the image
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:frontend: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:f15: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- f-15 strike eagle ii (1991)(microprose)(m3)[cr csl][t +4 metal force]/F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:select: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 146678 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_game_state` (runtime segment an address (play-rs-m1500)): Initializes hardware registers and clears a memory block from a fixed address to a fixed address, then sets up interrupt and DMA control registers before entering a loop that waits for vertical blanking and processes keyboard input. Kind: custom coprocessor loop with input checking. Pass path: The routine waits for specific key inputs ('P', 'Q', 'R', 'S') to toggle flags. A successful check would involve the correct sequence or state being met, leading to a branch that continues execution after '@' is received..

## Functions


### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:library

- `invoke_object_method_loop`: Loops calling an object's method through a table at offset -552 of a global structure until the returned value is nonzero, then invokes a method at offset -30 of that returned object, and finally masks a byte in a global variable at offset 676 with 142 and stores a global byte at offset a fixed address through a pointer.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:frontend

- `save_registers_call_library`: Saves registers d0-d7 and a0-a7 on the stack, loads the value 3 into d0, computes an address from a table, and calls a routine in the library found in a6.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_d1_to_d0`: Copies the value from register d1 into register d0 and then jumps to another routine.
- `update_entity_state_and_process_events`: Increments an index in a structure, clears several fields in a global structure, and performs a series of conditional calls to update entity properties, status flags, and event data based on values retrieved from tables.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_moveq_move_bra`: Short code fragment that loads 255 into d1, then copies d1 to the memory at an offset from a2, then jumps to another point in the code.
- `frag_moveq_move_bra_2`: Short code fragment that loads 255 into d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `fetch_byte_and_call`: Calculates an index using a value from offset 220 in a1 and an offset of 224 in a1, fetches a byte from a0 at that index into d2, and calls another routine before branching to another routine.
- `fetch_and_compare_byte`: Calculates an index using values from a1, fetches a byte from a0 at that index, calls a routine with a pointer from a2, and compares the result to the fetched byte before branching.
- `process_indexed_data_and_call`: Increments a long value at offset 224 of a1, then checks a byte from a table using an index derived from offset 220 of a1.
- `update_status_and_call`: Stores 1 in offset 616 of a2, calls a routine using a pointer from a2, updates offset 208 of a1, and performs several conditional calls to other routines using pointers from a2 before branching.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_move_movea_jsr_jmp`: Short code fragment that copies 232 to d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then jumps to the address in a6.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:GOLLUM!

- `open_dos_library_then_run`: Program entry that opens the dos library by name, makes three library calls, then one further call, and returns zero.
- `wait_for_button_release`: Waits in a loop while bit 6 of the joystick fire button register is set and returns as soon as that bit becomes clear.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:assign

- `poll_library_until_ready`: Calls the routine at the function pointer stored at offset -552 of a global structure in a6 until it returns a nonzero result, then invokes the routine at the function pointer at offset -30 of that returned value, then stores a byte through a1 and returns.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:f15

- `poll_library_until_ready_2`: Calls a function pointer from a global table until it returns nonzero, then invokes a routine at offset -30 of that result and stores the value in d5 into a buffer at offset an address of a5, handling negative results by writing bytes and setting a flag bit in a RAM location.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:end

- `init_explode_library`: Saves the caller's registers on the stack, loads value 3 into a register, gets the address of a table, fetches the base pointer of an Amiga OS library from a global variable, and calls a routine inside the explode library, then returns.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:select

- `push_registers_and_call_library`: Pushes all registers on the stack, loads the value one into a data register, computes a call target, copies a global base address into a register, and calls a library entry point, then tests the result and returns when it is zero.
- `branch_on_status_flags`: Evaluates a chain of conditional branches based on the status flags, jumping to one of several target locations, and finally calls the routine at the following address via a subroutine call, then returns.

### explode.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `init_explode_library_base`: Initializes fields of the new library base structure, reads a global variable to obtain a related structure, and stores zero into two of its function vectors.
- `increment_structure_counter_2`: Increments the 32-bit value stored at offset 42 of the structure in a6 by one, then returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_library_and_increment`: Calls another routine to obtain a pointer, multiplies the result by four, and if the value is nonzero and the pointed-to structure contains the expected library identification strings, increments a global counter and calls another routine, otherwise it returns the original pointer.
- `increment_counter_and_call`: Increments a 32-bit global variable in the explode library data by one, calls another routine, then restores the saved registers and returns the original first argument unchanged.
- `find_task_set_priority_around`: Reads a global variable, calls the exec FindTask routine to obtain the current task, then calls the priority-setting exec routine on that task, invokes another routine, applies the priority again, and returns its first argument unchanged.
- `init_packed_field_and_accumulate`: Initializes a 48-byte field in the structure pointed to by a4 from a packed input buffer, builds a lookup table and a set of cumulative values, then repeatedly calls a routine to accumulate a signed sum into a field while a value is negative.
- `accumulate_signed_loop`: Repeatedly runs a loop body while the incoming value in d0 is negative, accumulating a signed sum into a field through the pointer in a3 and storing the running total back.
- `branch_and_call_routine`: Branches to a nearby routine when the condition code is equal, then copies the address in a3 into a1 and calls the routine at offset 334.
- `load_table_base_and_two`: Loads the address of the 48-byte field at offset 48 of the structure in a4 into a1, then loads the 2 into d0, and falls through into the following routine.
- `copy_run_and_loop`: Copies four bytes from the source buffer in a2 to the destination in a1, advancing both pointers, then decrements a counter at offset 58 of the structure in a4 and a loop counter in d0.
- `copy_run_and_decode_code`: Copies a run of bytes from the source buffer in a0 to the destination in a2, decrementing the run counter, then reads and decodes the next run-length code byte from the buffer in a0.
- `add_run_length_code_to_counter_2`: Decodes a run-length code byte from the buffer in a0, adds its value to the 16-bit counter in d2, then reads a second byte and multiplies the counter by that value, and returns.
- `setup_run_length_five`: Loads the run-length code value five into d0 and the multiplier three into d7, then branches into the run-length decoder routine to process the next buffer byte.
- `setup_run_length_four`: Loads the run-length code value four into d0 and the multiplier two into d7, then branches into the run-length decoder routine to process the next buffer byte.
- `setup_run_length_three`: Loads the run-length code value three into d0 and the multiplier one into d7, then branches into the run-length decoder routine to process the next buffer byte.
- `add_run_length_code_to_counter`: Reads a run-length code byte from the buffer in a0 and adds its value to the 16-bit counter in d2, then reads a second byte and multiplies that counter by two for each bit set, and returns.
- `read_run_length_code`: Reads a run-length code byte from the buffer in a0 into d1, then uses that code to increment the 16-bit value in d2 by the code's value before returning.
- `copy_run_length_bytes`: Reads a run-length code from the buffer in a0, then copies the specified number of bytes from the output buffer back to itself through the pointer in a3, decrementing the byte counter in d0 until the run is complete.
- `decode_run_length_stream`: Decodes a run-length encoded byte stream from the buffer in a0, expanding each run into the destination buffer in a2, then copies the remaining bytes and stores the updated counters and pointers back into the structure in a4.
- `store_registers_and_branch_back`: Stores a byte from a data register at offset three of a structure, stores another byte at offset two, stores a word at the base of the structure, restores several registers from the stack, and branches back into the library.
- `loop_and_update_global`: Increments a word in a structure, runs a loop that calls a routine and another routine while decrementing a counter, then adds a value from the structure to a global variable and returns.
- `add_register_to_global`: Adds the 32-bit value from d0 to the global 32-bit variable and returns.
- `delay_countdown_loop`: Copies the 16-bit value from d6 into d7, decrements it by one, and if it was not originally zero branches into a loop that repeatedly decrements d7 until it reaches minus one, then returns.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:game

- `push_registers_and_call_library_again`: Pushes all registers on the stack, loads the value three into a data register, computes a call target, copies a global base address into a register, and calls a library entry point, then tests the result and returns when it is zero.
- `compare_structure_byte`: Compares the byte at offset 6 of the structure in a6 with zero and branches based on the result, then compares it with 108 and branches based on the result.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:Crystal

- `clear_memory_then_setup_display`: Sets several global values, clears a block of longwords from a fixed address up to a fixed end address in a loop, calls six routines, then waits for a key press and handles the pressed keys before returning.
- `copy_bytes_into_low_memory`: Copies 74 bytes from a block of code into a fixed low-memory area starting at address 256, using a counter initialized to 73 and decrementing until it underflows, then returns.
- `patch_branch_return_when_flagged`: When a global flag is set, overwrites instruction words at offsets 37396 and 37430 with 24580 and 20085, then sets 24580 at offsets 36742, 36656 and 36696, and finally stores 255 at offset 676 of a structure.
- `update_keyboard_state_byte`: Reads a global keyboard-state byte, clears it and clears bit 6 of the CIA control register, then computes an inverted, right-rotated value and stores it back into the global byte when that value is zero.
- `interleave_word_pairs_into_buffer`: Writes 48 pairs of 16-bit words into a destination buffer, each pair holding a value from a register and the next word read from a source pointer, looping until the counter reaches negative one.
- `draw_text_glyphs_to_bitplane`: Copies each eight-row glyph of a zero-terminated string from a font table into a bitplane buffer, advancing the destination by 40 bytes per row and the source by one byte per glyph, until the end of the string is reached.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:wait

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value stored in a field, then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_d1_to_d0_and_branch`: Copies the 32-bit value from d1 into d0, then branches to the target address without returning on its own.
- `initialize_entity`: Initializes entity data by clearing specific fields in a structure, updating a value through pointer arithmetic, and setting up conditional branches based on table lookups.
- `process_entity_event_table`: Fetches an index from a structure, uses it to look up a table entry, and retrieves a byte value from that table.
- `check_table_and_branch`: Loads a long value from a1 into d1, shifts it left by 2, and tests a long value at offset 8 of a0 plus the shifted d1.
- `calculate_timer_decrement`: Shifts a value from offset 216 of a1, calls a routine, stores the result in offset 212 of a1, and then subtracts 60 from a value in a2 to update offset 208 of a1.
- `parse_numeric_input_string`: Fetches a value from a table using an index in a1, clears two fields in a structure, and calls another routine.
- `validate_and_call_subroutine`: Checks if a byte at a calculated offset in a table falls within the range of 49 to 56 inclusive; if it does, it calls another routine with a specific parameter and returns, otherwise it performs bitwise operations on two calculated indices and returns.
- `process_input_buffer_values`: Checks a value at an offset in a structure against a constant, fetches a byte from a table using an index from the structure, and performs arithmetic to update several offsets within that structure.
- `process_buffer_values`: Fetches a byte from a0 using an index derived from a1, performs arithmetic to update offset 212 of a1, and enters a loop that calls a routine multiple times while updating various offsets in a1.
- `execute_loop_logic`: Stores 60 and 1 into offsets 228 and 232 of a structure in a1, then enters a loop that calls a routine multiple times based on values in a2 and a1, eventually adding a value from offset 232 of a1 to d1 and branching.
- `frag_moveq_add_bra`: Short code fragment that loads 1 into d1, then adds the memory at an offset from a1 to d1, then jumps to another point in the code.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.

### F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `set_boot_library_request`: It clears a field and sets a second field in a structure, then makes one library call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.
- `initialize_game_state`: Initializes hardware registers and clears a memory block from a fixed address to a fixed address, then sets up interrupt and DMA control registers before entering a loop that waits for vertical blanking and processes keyboard input.
- `copy_74_bytes_low_memory`: Copies 74 bytes from a block of code into a fixed low-memory area and returns.
- `patch_branch_and_return_opcodes`: When a global flag is set, overwrites two code locations with a short forward-branch instruction and a return instruction.
- `update_keyboard_state_byte_2`: Reads a global keyboard-state byte, clears related bits, then computes an inverted, shifted value and stores it back when the shifted value is zero.
- `interleave_word_pairs_into_buffer_2`: Writes 48 pairs of 16-bit words into a destination buffer, each pair holding a value taken from a register and the next word read from the source pointer.
- `draw_text_glyphs_to_bitplane_2`: Draws each character of a zero-terminated string into a 320-pixel-wide bitplane by copying its eight-row glyph from a font table.

### logged disk load from F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf

- `setup_video_dma`: Configures interrupt and DMA controller registers, clears memory from a fixed address to a fixed address, and initializes display hardware before entering a vertical blanking loop that handles input processing.
- `copy_74_bytes_low_memory_2`: Copies 74 bytes from a block of code into a fixed low-memory area and returns.
- `copy_font_glyphs`: Copies font glyph data from a source buffer to a destination buffer using indexed addressing with multiplication and bit manipulation operations, repeating until all glyphs are processed.

### logged disk load from F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf

- `transfer_glyph_data`: Transfers glyph data between memory locations using indexed addressing with multiplication and bit manipulation operations, repeating until all glyphs are processed.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### logged disk load from F-15 Strike Eagle II (1991)(MicroProse)(M3)[cr CSL][t +4 Metal Force].adf

- `update_keyboard_state_byte_3`: Reads a global keyboard-state byte, clears related bits, then computes an inverted, shifted value and stores it back when the shifted value is zero.
- `interleave_words_into_buffer`: Writes 48 pairs of 16-bit words into a destination buffer, where each pair consists of a value from d1 and a word from the source pointer in a0, starting with the 32-bit value in d0.
- `frag_move_tst_beq_lea_movea_ori`: Short code fragment that copies 320 to d2, then tests the memory at a1, then branches if the last result was zero, then loads the address of a location in the code into a0, then copies an address to a2, then merges 8 into d0, then merges 201 into d0, then merges 46 into d0, then merges 232 into d0, then merges 234 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_movem_rts`: Short code fragment that copies an address to a fixed memory address, then copies several registers to or from memory, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_cmpi_beq_bsr_bra_move_movea`: Short code fragment that compares a fixed memory address with 80, then compares a fixed memory address with 64, then branches if the last result was zero, then calls another routine, then jumps to another point in the code, then copies 57388 to a fixed memory address, then copies a fixed memory address to a6, then copies the memory at an offset from a6 to a6.
- `frag_bne`: Short code fragment that branches if the last result was not zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (movea.l) and then falls through into the code that follows, with no return of its own.
