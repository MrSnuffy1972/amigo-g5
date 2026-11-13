# Game 1091 (red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01091). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (917208 top-level bytes in 68 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 48 of 48 units identical (bytes and relocations), 43 raw-word lines |
| Functions named and described | pass, 391 of 391 named (0 author symbols), 391 of 391 described (47 by rule, 277 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2657 of 2668 (99.59%) |
| Recovered without load address (counted in proof 1, flagged) | 386879 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10410 of 10494 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 24763 |
| code | 24312 |
| data | 816416 |
| padding | 9733 |
| unused | 41984 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:install | 10 | 250 | 17 |
| Run | 2 | 263 | 8 |
| More | 2 | 263 | 8 |
| Avail | 11 | 376 | 12 |
| AddBuffers | 2 | 117 | 2 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:sim | 8 | 250 | 17 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:ps | 9 | 250 | 5 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:baron | 9 | 250 | 5 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:nem | 2 | 85 | 3 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:amigasnd.drv | 3 | 1820 | 112 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 149 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2429 | 57 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 117 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 44 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 45 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 66 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 157 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 68 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 30 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 103 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 406 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 30 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 65 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 119 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 49 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 42 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 65 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 33 | 2 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:baron#unpacked | 5 | 2381 | 55 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:credits.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:volume.001 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- extended reference run ignored: it does not re-execute 2022 of the 2717 PCs the canonical run logged (165208 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 159 64 KB pages each with >= 29491 distinct PCs executed, 2605019 of its 2775484 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 159 64 KB pages each with >= 29491 distinct PCs executed, 2605019 of its 2775484 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 159 64 KB pages each with >= 29491 distinct PCs executed, 2605019 of its 2775484 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 159 64 KB pages each with >= 29491 distinct PCs executed, 2605019 of its 2775484 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- runtime segment an address (f100): 141 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 775 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- RAM snapshot an address: 35 executed PCs whose RAM was overwritten before any snapshot match its bytes 6864-7152: recovered as a unit run at an address
- RAM snapshot an address: 37 executed PCs whose RAM was overwritten before any snapshot match its bytes 8176-8384: recovered as a unit run at an address
- RAM snapshot an address: 31 executed PCs whose RAM was overwritten before any snapshot match its bytes 3968-4112: recovered as a unit run at an address
- RAM snapshot an address: 7 executed PCs whose RAM was overwritten before any snapshot match its bytes 4416-4512: recovered as a unit run at an address
- RAM snapshot an address: 4 executed PCs whose RAM was overwritten before any snapshot match its bytes 4112-4192: recovered as a unit run at an address
- RAM snapshot an address: 4 executed PCs whose RAM was overwritten before any snapshot match its bytes 4752-4928: recovered as a unit run at an address
- 790 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 25 first-run image unit(s) in 1 version group(s); 2620 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:install: Imploder (xfd) stream found by ancient scan (12376 -> 20036 bytes) but only 1 of 10 sampled windows are in reference RAM: not used
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:c/Run: PowerPacker 4.0 (xfd) stream found by ancient scan (2356 -> 2568 bytes) but only 0 of 2 sampled windows are in reference RAM: not used
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:c/More: PowerPacker 4.0 (xfd) stream found by ancient scan (6960 -> 11284 bytes) but only 0 of 6 sampled windows are in reference RAM: not used
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:sim: Imploder (xfd) stream found by ancient scan (173172 -> 308692 bytes) but only 1 of 147 sampled windows are in reference RAM: not used
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:ps: Imploder (xfd) stream found by ancient scan (154520 -> 312268 bytes) but only 0 of 149 sampled windows are in reference RAM: not used
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:baron: crunched executable decrunched by XFDMaster (Imploder (xfd), 6564 -> 8980 bytes), 3 of 4 windows confirmed in reference RAM: recovered as a hunk executable
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:nem: ByteKiller 1.3 (xfd) stream found by ancient scan (1712 -> 2028 bytes) but only 1 of 1 sampled windows are in reference RAM: not used
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:credits.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:volume.001: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:roster.dat: raw file found loaded verbatim in runtime segment an address (f100) at an address; classified through the recovered segment
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:c/More: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:sim: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:ps: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)[a2]/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:baron: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 386879 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_hardware_and_graphics` (runtime segment an address (f100)): Waits for a bit in a hardware register, sets several hardware registers including DMA and interrupt control, and performs multiple loops to update a table based on bitwise checks of a structure field. Kind: disk hardware/timing check. Pass path: the routine completes and proceeds to exec.OpenLibrary to load the main game/application.

## Functions


### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:install

- `initialize_game_state`: Executes a sequence of function calls and checks a global pointer to swap a memory value if a specific constant is found.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `initialize_and_loop`: Calls a routine twice, increments a pointer in a1 by a constant, calls a routine again, sets a pointer in a6 and a2, and enters a loop that calls another routine, increments d5, and checks if d5 equals six, eventually calling a function from a global variable if a condition is met.
- `update_table_values`: Doubles d0 and branches if negative; otherwise, it calls another routine, then potentially calls a function at a4, and enters a loop that calls another routine, adds d0 to d2, and adds the result from a1 to a table at a3 using an offset in d2, repeating until d1 reaches zero.
- `call_subroutine_conditional`: If a byte is zero, it returns; otherwise, it moves the value in a3 to a1, calls another routine, and branches back to the start.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes`: If a pointer in a6 equals a pointer in a2, it returns; otherwise, it copies four bytes from the source at a2 to the destination at a1 using a loop that decrements d0 and repeats until d0 is zero, then loads a value from a1 into d0 and returns.
- `decompress_bitstream_data_2`: Reads a bitstream from a source buffer in a0 to reconstruct values using a series of bit-shifting and conditional byte-fetching operations, then copies the resulting data to a destination buffer in a3.
- `expand_bitstream_value_2`: Doubles the byte in d1 and, if the result is zero, fetches a new byte from the buffer in a0 into d1, then performs two additions with carry into d1 and d0 before looping up to 2 times using d7, adding 6 to d0, and branching to another location.
- `set_registers_and_branch`: Sets d0 to 5, d7 to 3, and branches to another location.
- `set_registers_and_branch_2`: Sets d0 to 4, d7 to 2, and branches to another location.
- `set_registers_and_branch_3`: Sets d0 to 3, d7 to 1, and branches to another location.
- `expand_bitstream_value`: Doubles the value in d1 and, if the result is zero, fetches a new byte from the buffer in a0 to continue the expansion via addition with carry.
- `update_bitstream_values`: Doubles the byte in d1, branches to an earlier instruction if the result is non-zero, otherwise it loads a byte from the address in a0 into d1, adds d1 to d1 with carry, adds d2 to d2 with carry, and branches to another routine.
- `decompress_bitstream_data`: Reads a bitstream from a source buffer to reconstruct values, then copies a block of data from a source address to a destination address using a table-based lookup.
- `decompress_and_copy_data`: Reads a bitstream from a source buffer to reconstruct values, then copies a block of data from a source address to a destination address using a loop.
- `push_registers_to_stack`: Pushes bytes from d0 and d1, and a word from d2, onto the stack at the location pointed to by a4, then branches to another routine.

### Run

- `parse_and_relocate_hunks`: Parses a data structure containing hunk type codes to perform memory allocation and relocation of data segments.
- `decrement_and_loop_d1`: Decrements a 16-bit value in d1 in a loop until it reaches zero, otherwise it returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `extract_bits_from_long`: Loads a long from a0, performs several calls to other routines to process bit shifts and additions, and stores bytes into a destination pointed to by a3.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `extract_bits_loop`: Performs a loop that uses bit shifts and rotations to extract bits from a long value in a0 and stores them into a destination pointed to by a3.
- `execute_trap`: Moves a word from a location offset by 3085 bytes from a2 into d4, sets bit 3 of d0, calls a trap, and performs a logical OR of d0 into d1.

### More

- `parse_amiga_hunk_relocations`: Parses a data structure containing AmigaDOS-style hunk types, performing memory allocation and relocation adjustments by iterating through segments, handling code and data hunks, and applying address offsets to relocation tables.
- `process_loop_and_swap`: Executes a loop that decrements a 16-bit value in d1, then conditionally calls two routines and writes a value to a global pointer if a specific condition is met.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `extract_bits_loop_2`: Processes a long value from a0 by performing bit shifts and rotations, calling several routines, and storing bytes into a destination pointed to by a3 using a loop.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `subtract_d4_from_d0`: Subtracts the value in d4 from d0.
- `extract_variable_length_bits`: Extracts a specified number of bits from a long value pointed to by a0 and stores the resulting bits into a destination buffer.
- `trap_with_data`: Moves a word from an offset of a2 into d4, sets bit 5 and 4 of d0, calls a trap, and performs a bitwise OR on d1.

### Avail

- `parse_command_string`: Parses a command string by splitting it into tokens, handling quoted substrings and converting specific character sequences into control characters, then stores the resulting pointers in a structure.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `sum_memory_region_sizes`: Iterates through a linked list of memory regions and sums the size of any region whose attribute flags match a provided mask.
- `print_memory_availability_table`: Checks a command string or argument to determine if the output should be filtered by chip, fast, or total memory types, then calculates and prints the available, in-use, and maximum block sizes for the selected memory type or a combination of types.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### AddBuffers

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Copies the contents of d1 to d0 and then performs a relative branch.

### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:sim

- `initialize_system_memory`: Performs a series of function calls, decrements a counter in a loop, and checks a specific memory pattern to update a global variable.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.
- `accumulate_array_values`: Performs a loop four times, calling a routine via a pointer in a4 and adding the returned value to an array at a specific memory location.
- `accumulate_array_elements`: Iterates through a range of values defined by a count in d0, calls a subroutine to fetch a value, and adds that value to an array located at the address in a3 using an index calculated from d0.
- `call_subroutine_conditional_2`: If a byte is zero, it returns; otherwise, it moves the value in a3 to a1, calls another routine, and branches back to the start.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_2`: If a pointer in a6 equals a pointer in a2, it returns; otherwise, it copies four bytes from the source at a2 to the destination at a1 using a loop that decrements d0 and repeats until d0 is zero, then loads a value from a1 into d0 and returns.
- `decode_bitstream_variable_integers`: Decodes a sequence of variable-length integers from a bitstream buffer in a0 by repeatedly shifting a value left and fetching new bytes from the buffer when the value reaches zero, then stores the resulting values into a destination buffer pointed to by a3.
- `expand_bitstream_value_3`: Doubles the byte in d1 and, if the result is zero, fetches a new byte from the buffer in a0 into d1, then performs two additions with carry into d1 and d0 before looping up to 2 times using d7, adding 6 to d0, and branching to another location.
- `set_registers_and_branch_4`: Sets d0 to 5, d7 to 3, and branches to another location.
- `set_registers_and_branch_5`: Sets d0 to 4, d7 to 2, and branches to another location.
- `set_registers_and_branch_6`: Sets d0 to 3, d7 to 1, and branches to another location.
- `decode_variable_length_integers_3`: Decodes a sequence of variable-length integers from a buffer in a0 by checking for carry bits and fetching additional bytes from the buffer when necessary.
- `process_variable_integers`: Multiplies the byte in d1 by two and branches back if non-zero, otherwise it fetches a byte from the pointer in a0 into d1, performs an addition of d1 to d1 with carry, performs an addition of d2 to d2 with carry, and branches to another routine.
- `decode_variable_length_integers_2`: Decodes a sequence of variable-length integers from a memory buffer using bit-shifting and carry propagation, then uses these values to perform memory copies or data manipulations involving a table and a destination pointer.
- `decode_variable_length_integers`: Reads a sequence of bytes to decode variable-length integers using a bit-shifting loop, then uses the resulting values to perform a memory copy from a source buffer to a destination buffer.
- `push_registers_to_stack_2`: Pushes bytes from d0 and d1, and a word from d2, onto the stack at the location pointed to by a4, then branches to another routine.

### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:ps

- `free_blocks_and_restore_patch`: Calls two unnamed routines, loops to free four memory blocks via a library call, and restores a saved word in a table if a specific masked pattern is matched.
- `decrement_until_minus_one_3`: Decrements a 16-bit value in d7 until it reaches negative one and then returns.
- `accumulate_array_values_five`: Performs a loop five times, calling a routine via a pointer in a4 and adding the returned value to an array at a specific memory location.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_and_decrement_counter`: Compares a pointer in a2 with a pointer in a6 and, if they are not equal, copies four bytes from the source at a2 to the destination at a1 while decrementing a counter in d0.

### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:baron

- `free_blocks_and_restore_patch_2`: Calls two unnamed routines, loops to free five memory blocks, and restores a saved word in a table if a specific masked pattern is matched.
- `decrement_word_register_in_loop`: Decrements a 16-bit value in a register until it reaches negative one and then returns.
- `expand_and_process_data`: Calls two routines to load and expand data, then iterates through a loop to add values to a buffer at a specific offset, potentially calling another routine and a function from a global structure.
- `load_a1_and_d0`: Loads the address of a label into a1 and sets d0 to 2.
- `expand_compressed_data`: Decompresses a data stream using a variable-length encoding scheme involving bit-shifting and repeated byte expansions.

### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:nem

- `depack_packed_stream_backward`: Decodes a packed stream by calling a bit-reader refill and a multi-bit read routine, copying literal and back-reference runs backwards into memory.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:amigasnd.drv

- `execute_indirect_call`: Performs an indirect function call using a pointer derived from a global variable and a register value.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `compare_and_set_flags`: Compares two bytes from offsets 520 and 524 of a structure in a4 with d0 and d1, and if they match, checks a byte at offset 440 of a4; if that byte is not zero, it sets a byte at offset 528 of a4 to 1.
- `write_bytes_to_structure`: Writes the byte value 1 to an offset in a structure and the byte value 0 to another offset in a structure, then branches.
- `pop_and_compare_d2`: Increments d2 and compares it to four; if not equal, it returns, otherwise it pops four values from the stack into a1, a0, d3, and d2.
- `push_registers_and_call_subroutine`: Pushes d2, a0, and a1 onto the stack, swaps the high and low bytes of d1, and if the low byte is not zero, swaps d1 again and calls another routine; otherwise it swaps d1 again and branches to another routine.
- `clear_audio_channel_data_3`: Swaps d1, compares bytes at offsets 520 and 524 of a4 with d0 and d1, and if both match, writes 0 to offset 528 of a4, calls two other routines, and branches to another routine; otherwise it branches to another routine.
- `increment_and_branch_audio_index`: Increments a 16-bit value in d2, compares it to 4, and branches to another routine if not equal; otherwise it calls another routine, compares d2 to 255, and calls another routine before popping registers and returning.
- `update_audio_parameter_table`: Checks if a byte in d1 is equal to 7; if so, it extracts a value from the upper word of d1, performs a bitwise shift and increment, and stores the resulting byte into a table at an offset of 392 plus the value in d0 relative to a pointer in a4.
- `call_subroutine_on_byte_match`: Compares d1 to 64 and, if not equal, branches to another routine; otherwise it swaps d1, calls another routine, swaps d1 again, and branches to another routine.
- `store_swapped_audio_byte`: Compares d1 to 1, and if not equal, branches to another routine; otherwise it loads a pointer from offset 1a8 of a4, swaps d1, stores the byte into the table at that pointer using d0 as an index, swaps d1 again, and branches to another routine.
- `swap_and_call_audio_data`: Compares d1 to 75, and if equal, swaps d1, calls another routine, swaps d1 again, and branches to another routine; otherwise it branches to another routine.
- `search_audio_data`: Compares a byte in d0 against a sequence of bytes in a global structure and updates a local variable if a match is found and the corresponding byte at offset 524 is not 255.
- `store_byte_at_offset`: Stores a byte into a memory location calculated from a base address, an offset, and a word value.
- `update_audio_register`: Calculates a 16-bit value based on d1 and d0, stores it at an offset in a4 determined by d0 and d1, and then iterates through a four-byte sequence to check if a byte at offset 520 of a4 matches d0, calling another routine if a match is found.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `get_and_set_byte`: Returns a byte from a memory location and, if the input byte is not negative one, updates that location with the input byte.
- `get_and_set_word`: Returns a word from a memory location and, if the input word is not negative thirty-two thousand seven hundred sixty-eight, updates that location with the input word.
- `update_byte_via_pointer`: Reads a byte from a memory location determined by adding an offset to a value in a register and conditionally overwrites it with a value from a register if that register does not contain a specific negative value.
- `update_audio_index_2`: Iterates through a structure to find a matching byte, performs bitwise operations on a value, and updates a word in a structure based on a comparison with d5.
- `update_audio_index`: Compares a word from a structure against d5, performs bit manipulation on d5 and d2, and if certain conditions are met, writes a byte to a structure and calls another routine.
- `update_audio_channel`: Pushes several registers to the stack, searches for a byte in a table at offset 520 of a4, and if found, updates a value at offset 456 of a4 and calls another routine.
- `call_routine_twice`: Subtracts d1 from d4, moves the result to d1, calls a routine, calls another routine, and returns.
- `process_audio_channels`: Pushes several registers onto the stack, checks if a byte at offset five hundred twenty of a structure in a4 is negative, and if so, increments two counters and checks if the first counter reaches four; if the second counter is not zero, it iterates through a structure at offset four hundred fifty-six of a4, comparing bytes to a value and potentially updating them and calling another routine.
- `modify_channel_byte`: Subtracts d3 from d4, stores the result in a byte at offset d0 of a structure in a0, moves d4 to d1, and calls another routine.
- `pop_and_compare_d0`: Increments d0 and compares it to ten; if not equal, it returns, otherwise it pops five values from the stack into a0, d4, d3, d1, and d0.
- `modify_audio_channel`: Searches for a specific byte value in a structure, updates it, and increments a value at a calculated memory offset.
- `clear_byte_if_equal`: Compares a byte at offset 456 plus a value in a4 and d0 with d1; if they are equal, it stores zero at that location and branches to another routine, otherwise it branches to another location.
- `decrement_byte_at_offset`: If a carry is set, it branches to another location; otherwise, it subtracts a byte in d1 from a byte at offset d0 of a structure in a0, stores the result back at that offset, and branches to another location.
- `process_audio_loop_2`: Subtracts a byte at offset d0 of a structure in a0 from d1, clears that byte, and then iterates through a structure to update values based on comparisons with d0.
- `process_audio_loop`: Iterates through a structure to find a maximum word value, performs bit manipulation, and writes bytes to a structure while calling another routine.
- `execute_empty_routine`: This routine consists of ten instructions but performs no observable operation, as it does not modify any registers, memory, or hardware state before returning.
- `update_status_table`: Stores a byte at a calculated offset and, if that byte is zero, iterates through a table to update specific flags based on a matching index.
- `reset_audio_channel_parameters`: Clears several bytes and words in a table at offsets 520 through 656 relative to a4 based on a value in d2, then searches a linked list of audio structures to find an entry where a value in d1 falls between two stored bounds; if found, it updates several fields in the table and calls update_audio_parameters.
- `increment_pointer_and_branch`: Adds fourteen to a1 and then performs a relative branch.
- `update_audio_parameters`: Stores several values into a table at offsets in a4 based on d2, updates two bytes at offsets in a4 using a value derived from d1, and calls another routine.
- `call_audio_reset`: Calls a routine and then restores six registers from the stack before returning.
- `clear_audio_channel_data`: Calls another routine and then resets several memory locations and offsets within a global structure to zero or 255 based on an index in d2.
- `fetch_audio_parameters`: Loads several pointers from a table at offset 524 plus a scaled index in a4, then calls two different routines and branches to another location based on the results.
- `return_stack_value`: Returns a 24-bit value formed by combining a constant with a 4-byte field taken from a 32-bit value located on the stack at a fixed offset.
- `update_audio_voice_counters`: Pushes several registers to the stack, checks a byte at offset 524 of a4, calls another routine if not 255, increments values in a table at offset 532 and 540 based on a scaled d2, calls another routine, calls another routine, and increments d2 in a loop until it reaches 4 before returning.
- `update_audio_envelope_state_2`: Updates an audio envelope state by managing a counter and a volume offset value within a structure.
- `update_audio_envelope_state`: Processes an audio envelope by shifting a value in a structure, decrementing a counter, or loading new sample data and volume offsets from tables based on the current state.
- `add_audio_value`: Loads a byte from a table at offset 68 of a4 into a structure in a0, adds another byte from a table at offset 100 of a4 to it, and if the result is less than or equal to a byte from a table at offset 11 of a1, updates the structure and sets a byte in a6 to 1.
- `decrement_audio_byte_conditionally`: Compares a byte at offset d2 of a structure in a6 with 1 and branches to another routine if they are not equal; otherwise, it compares a byte at offset d2 of a structure in a0 with 0, and if not 0, subtracts 1 from that byte before branching to another routine.
- `subtract_audio_value`: Loads a byte from a table at offset 68 of a4 into a structure in a0, subtracts a byte from a table at offset 100 of a4 from it, and if the result is greater than or equal to a byte from a table at offset 13 of a1, updates the structure and sets a byte in a6 to 2.
- `copy_audio_byte`: Compares a byte at offset d2 of a structure in a6 with two, and if they are equal, copies a byte from offset d in a1 to offset d2 of a structure in a2 before branching to another location.
- `decrement_channel_byte`: Compares a byte at offset d2 of a structure in a6 to 3, then compares a byte at offset d2 of a structure in a0 to 0; if both conditions are met, it decrements the byte at offset d2 of a structure in a0.
- `modify_audio_data`: Subtracts a value from a byte in a structure using an offset from another table and calls itself recursively if a condition is met.
- `calculate_volume_scaling_4`: Loads bytes from a table at offset 520 plus d2 in a4, a table at offset 392 plus the first byte in a4, and a table at offset 564 plus d2 in a4; if the second byte is zero, it clears d4 and branches to another location, otherwise it branches to another location.
- `check_volume_byte`: Compares the value in d4 with zero and branches to another location if it is not zero; otherwise it branches to another location.
- `calculate_volume_scaling_3`: Multiplies d3 and d4, divides the result by 63, and if a byte at offset d2 of a structure in a0 is not zero, it branches to another routine; otherwise it clears d4.
- `calculate_volume_scaling_2`: Multiplies two 8-bit values, divides the result by 63, and then calls another routine to calculate a value based on that result and a word from a structure in a4 shifted left by 2.
- `calculate_volume_scaling`: Calculates a value based on the product of two inputs and stores the result in a global variable.
- `search_audio_channel_match`: Pushes several registers to the stack, compares a byte at offset 520 of a4 with d1, and if they match, branches to another routine; otherwise it increments d2 and compares it to 4, branching to another routine if not equal.
- `update_audio_parameters_2`: Updates several entries in a structure in a4 using a scaled index in d2, including writing a byte derived from d0, setting two specific bytes at offsets 524 and 564, and populating three longword entries at offsets 136, 264, and 600.
- `configure_audio_parameters`: Sets a value at offset 18 of a structure in a3, updates two entries in a structure in a4 using a scaled index in d2, and populates a second structure with bytes from a table before calling another routine.
- `reset_audio_hardware_state`: Calls a routine to reset the audio hardware and returns a value from a memory location via a pointer.
- `search_audio_data_2`: Pushes d2 and a1 onto the stack, then enters a loop that compares a long value in a0 with bytes at an offset from a structure in a1, incrementing the offset until it reaches 16.
- `clear_audio_channel_data_2`: Shifts the value in d2 right by 2, calls another routine, and then pops two words from the stack into a1 and d2 before returning.
- `search_table_for_match`: Pushes d2, d1, and a1 onto the stack, loads a pointer from a4 plus 640, and enters a loop that compares a value in a0 to a table at that pointer, incrementing d2 by 4 until a match is found or d2 reaches 16.
- `clear_and_return_byte_2`: Sets a word at a fixed offset to one, retrieves a byte from a table using an index, clears that byte in the table, and returns the byte shifted.
- `clear_and_return_byte`: Sets a word at a fixed offset to one, retrieves a byte from a table using an index, clears that byte in the table, and returns the byte shifted left by sixteen bits.
- `fetch_table_value_2`: Swaps the high and low bytes of d2, compares the result to a constant, and if not equal, loads a word from a table at offset 280 of a4 using a scaled index in d2 before jumping to another routine.
- `fetch_table_byte_4`: Compares d2 with a constant, and if equal, swaps d2 and loads a byte from a table at offset 376 of a4 into d0 before jumping to another location.
- `fetch_table_byte_3`: Compares d2 with 176; if not equal, it branches to another location, otherwise it swaps d1, compares the new d2 with 7, and if not equal it branches to another location, otherwise it loads a byte from a table at offset 392 plus a masked d0 into d0 and branches to another location.
- `fetch_table_value`: Compares d2 with sixty-four and branches to another location if they are not equal; otherwise it loads a pointer from offset four hundred forty in a4, masks d0 with two hundred fifty-five, and loads a byte from that pointer using d0 as an offset before branching to another location.
- `fetch_table_byte_2`: Compares d2 to 1; if equal, it masks d0 with 255, loads a byte from a table at offset 424 of a4 using the masked d0 as an index, and branches to another routine.
- `fetch_table_byte`: Compares d2 to 75, and if equal, compares d1 to 255; if d1 is not 255, it loads a pointer from a4 plus 456, masks d0 with 255, and moves a byte from that table into d0.
- `compare_and_increment_counter`: Compares a byte at an offset of a structure in a4 with d0, and if they match, performs a series of swaps and additions on a 32-bit value in d3 before returning a 16-bit result in d0.
- `return_constant_value`: Returns a constant 16-bit value.
- `update_buffer_pointers`: Stores a longword from a0 into offset 132 of a4, adds the value of a4 to the longwords at offsets 140, 172, 204, and 236 of a4, and then calls another routine.
- `get_concatenated_data`: Concatenates two 16-bit values from a table into a single 32-bit result.
- `allocate_audio_memory_blocks`: Pushes several registers to the stack, calls another routine, calls another routine to allocate memory, and then stores the returned pointer into eight consecutive longword locations in a4 at offsets 5296, 5306, 5310, 5314, 5318, 5322, 5326, and 5330, before checking if the first stored pointer is zero and branching accordingly.
- `open_audio_device`: Sets up a structure with a byte, a pointer, and a longword, then calls another routine using a pointer from a global variable; if the return value is 0, it continues, otherwise it sets d1 to -1 and jumps to another routine.
- `initialize_audio_interrupt_handlers`: Initializes four structures with specific byte values and function pointers, calls a routine for each structure to store returned values, writes a value to the INTENA register, and calls another routine.
- `release_audio_resources`: Writes a value to the INTENA register, calls four routines via pointers, and then conditionally calls two more routines via pointers based on values in a4.
- `reset_audio_hardware`: Clears two bytes in a table at offsets 5358 and 5354 of a4, resets a hardware register at offset 8 of a calculated address, calls another routine, and sets a byte at offset 5301 of a4 to 1.
- `update_audio_registers`: Loads a byte and a word from a table into specific hardware registers and clears a third register.
- `calculate_scaled_index_2`: Pushes six registers onto the stack, checks a value at an offset in a structure in a6 using d2 as an index, and if non-zero, performs several arithmetic operations involving division and multiplication with values from tables in a4 to load a new value into d3 before branching to another location.
- `calculate_scaled_index`: Calculates an index by adding a value from a table to a modified d1, then uses this index to fetch values from several other tables in a4 and updates d1 and d6.
- `adjust_scaled_value`: Calculates an index by adding d6 to d1 shifted left by 4, loads a longword from a table in a3 at that index, and if a word at offset 8 of a1 is non-negative, subtracts a scaled difference between two longwords from the original value.
- `calculate_scaled_offset`: Subtracts a value from a memory location to get a difference, performs a multiplication using a value from a1, and adds the result to d3 before performing a conditional bit shift and comparison.
- `pop_arguments_from_stack`: Loads six values from the stack into registers and moves a seventh value from the stack into a register before returning.
- `write_audio_register`: Writes a 16-bit value to a hardware register.
- `write_audio_register_alt`: Writes a 16-bit value to the audio hardware register at an offset determined by shifting a value in d2 left by 4 and adding it to the register base.
- `decrement_and_update_audio`: Decrements a byte in a table and, if it reaches a specific value, updates hardware registers with values from a table.
- `update_audio_sample_buffer`: Copies a large block of data from a source buffer to a destination buffer within a structure, then updates volume and playback control bytes in the audio hardware structure based on calculated values.
- `update_audio_hardware`: Sets a byte in a structure in a4, calculates a value based on d3 and a2, and writes a 32-bit value from a table in a4 to an offset of the AUD0LCH register.
- `store_table_word`: Stores a 16-bit word from a table into a memory location at offset 4 of the structure in a0.
- `update_audio_sample_buffer_2`: Copies a large block of sample data from a source table to a destination buffer within a structure, then updates hardware parameters and sample length values based on calculated offsets and bounds checks.
- `set_audio_hardware_parameter`: Sets a byte in a structure in a4, calculates a value based on d3 and a2, and writes a 32-bit value from a table in a4 to an offset of the AUD0LCH register with a different byte value in a structure in a4.
- `store_table_word_2`: Reads a 16-bit word from a table using an offset in a register and stores it into a memory location at an offset of 4 from a pointer in a register, then returns.
- `reset_audio_channel`: Saves registers to the stack, compares a byte at offset 0 of a structure in a0 with 255, and if equal, sets d2 to 0 and calls another routine.
- `set_audio_flag`: Compares a byte at the address in a1 with 0; if it is 0, it branches to another location, otherwise it stores 255 at the address in a0 and branches to another location.
- `trigger_audio_interrupt`: Sets d2 to zero, compares a byte at the address in a0 with zero, and if it is not zero, calls another routine and branches to another location.
- `trigger_interrupt_request`: Calls another routine and sets bit 7 of the interrupt request register, then returns.
- `reset_audio_channel_alt`: Saves registers to the stack, compares a byte at offset 0 of a structure in a0 with 255, and if equal, sets d2 to 1 and calls another routine.
- `check_and_set_audio_byte`: Compares a byte at the address in a1 to zero; if it is not zero, it writes 255 to the address in a0 and branches to another routine, otherwise it returns.
- `check_audio_byte_and_call`: Loads the value 1 into d2, checks if the byte at the address in a0 is zero, and if it is not zero, calls another routine.
- `set_audio_interrupt_flag_high`: Calls another routine and then writes 256 to the interrupt request register.
- `reset_audio_channel_v3`: Saves registers to the stack, compares a byte at offset 0 of a structure in a0 with 255, and if equal, sets d2 to 2 and calls another routine.
- `check_and_reset_audio_byte`: Compares a byte at the address in a1 to zero; if it is not zero, it writes two hundred fifty-five to the address in a0 and otherwise returns.
- `check_audio_byte_and_call_two`: Loads the value 2 into d2, checks if the byte at the address in a0 is zero, and if it is not zero, calls another routine.
- `set_audio_interrupt_request`: Calls another routine, writes five hundred twelve to the interrupt request register, and returns using values popped from the stack into registers d2, a4, a1, and a0.
- `reset_audio_channel_v4`: Saves registers to the stack, compares a byte at offset 0 of a structure in a0 with 255, and if equal, sets d2 to 3 and calls another routine.
- `check_and_set_audio_byte_2`: Compares a byte in a memory location to zero; if it is not zero, it writes the value two hundred fifty-five to a byte at a pointer in a0 and branches to another routine, otherwise it branches to another routine.
- `check_audio_byte_and_call_three`: Loads the value 3 into d2, checks if the byte at the address in a0 is zero, and if it is not zero, calls another routine.
- `reset_audio_interrupt_flag`: Calls another routine and then writes the constant 1024 to the interrupt request register.
- `write_hardware_constants`: Writes specific constant values to two hardware registers at offsets a fixed address and a fixed address.
- `process_entity_data_table`: Iterates through a table of pointers, then traverses a nested list of structures until a terminator is reached.
- `decrement_and_loop`: Decrements d0 and compares it to 512, looping if not equal, then restores registers and returns.

### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_graphics`: Waits for a bit in a hardware register, sets several hardware registers including DMA and interrupt control, and performs multiple loops to update a table based on bitwise checks of a structure field.
- `copy_data_blocks`: Calculates a destination address based on a byte from a table, then copies 8 bytes from a source table to that destination in a loop.
- `initialize_data_table`: Writes a series of 16-bit values to a table starting at a specific address, then returns.
- `clear_memory_block`: Clears a block of 4000 words starting at a specific memory address using a loop, then returns.
- `set_graphics_parameters`: Stores three 16-bit values from a structure into a table, calls a library routine using a pointer from a global variable, and then updates three values in a structure with the results of bitwise OR operations.
- `pop_long_values`: Decrements a byte through a pointer and then pops six 32-bit words from the stack into register d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_dos_message`: Waits for a message on the current task's port, retrieves it, and if the message contains a pointer to a lock, duplicates that lock and retrieves the current directory.
- `execute_callback_chain`: Calls several functions via pointers stored in a structure and calls free_linked_list if a specific field is non-zero.
- `initialize_graphics_and_intuition`: Attempts to open the graphics and intuition libraries, and if successful, copies a string from a table into a local buffer and calls a routine to manage display contexts.
- `process_input_stream`: Calls another routine, calls another routine, and if the result is not 10 or 27 it decrements a 16-bit counter in a global variable and, if not below zero, reads a byte from a table at a global pointer, increments that pointer, and calls another routine; otherwise it calls another routine and returns.
- `process_linked_list_nodes`: Calls a routine via a pointer from a global variable, stores the result in a global variable, and then iterates through a linked list to copy pointers into a global array.
- `update_linked_list_data`: Iterates through a linked list, copying a longword from a table into a field at offset 4 of the current list element, then calls a routine via a pointer from a global table and updates a global pointer.
- `jump_to_address_c02374`: Performs a long jump to the address held in a register.
- `jump_to_address_c023d6`: Performs a long jump to the address held in a register.
- `call_indirect_function`: Loads a function pointer from an offset relative to a value stored on the stack and executes it.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `validate_pointer_and_execute`: Calls a routine to retrieve a pointer, checks if bit 3 of the byte at offset 1 of that pointer is set, and if so calls a second routine with a value of 2; it then calls a third routine using a pointer from offset 2 of the first pointer, and finally checks if a value at offset 436 of a structure in a4 is non-zero before returning a result.
- `cleanup_display_context_2`: Calls two routines via pointers in a structure, then conditionally calls a third routine if specific fields are zero, and finally performs a series of memory operations and calls involving a structure and a global pointer.
- `process_record_data`: Calculates the length of a null-terminated string and then iterates through its characters, decrementing a 16-bit counter in a structure and either writing each character to a table at offset 52 or calling another routine if the counter reaches zero.
- `parse_command_string_2`: Parses a string into a table of pointers, handling quoted substrings and expanding escape sequences like newline or carriage return.
- `manage_list_node`: Checks a flag in a structure and, if certain conditions are met, calls another routine to allocate memory, updates two pointers in the structure with the result, and if the result is zero, sets a value in a global variable and returns -1; otherwise it clears two fields and returns 0.
- `manage_record_data`: Performs multiple operations involving seeking, reading, validating, and allocating handles for records within a table by manipulating structure flags, updating pointers, and calling various subroutines to manage memory and data integrity.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs one instruction (movep.w) and then falls through into the code that follows, with no return of its own.
- `decode_variable_length_byte`: Parses a variable-length encoded stream by reading bytes from a buffer pointer, handling escape sequences and length prefixes.
- `iterate_table_with_function`: Iterates through a table of four values, passing a pointer from a structure and each value from the table to another routine.
- `copy_string_with_limit`: Copies a null-terminated string from a source pointer to a destination pointer, truncating the copy if it exceeds a specified maximum length and ensuring null-termination.
- `copy_string_to_buffer`: The routine finds the terminating zero byte of the destination buffer and copies bytes from the source pointer into that location until a zero byte is copied.
- `insert_into_ordered_list`: Inserts a block of memory into a linked list at a position determined by a size value, potentially splitting an existing node.
- `execute_io_with_pointer`: Saves registers, calls a routine, and if the result is non-zero, calls another routine using a pointer from the first result, then checks a 16-bit value at offset 436 of a structure in a4 to determine the final return value.
- `free_linked_list`: Iterates through a linked list and calls a function for each node before clearing the list pointers in a structure.
- `insert_memory_block`: Calls another routine to free or manipulate a block at a pointer from a global variable, then calls another routine to allocate a new block of a specified size, and if successful, writes the size into the new block and returns a pointer to it.
- `validate_nested_pointer_and_check_flag`: Calls a routine to retrieve a pointer, then calls a second routine using a value from that pointer; if the first call fails or if a specific 16-bit value at offset 436 of a structure is zero, it returns negative four, otherwise it returns the result of the second routine.
- `allocate_and_link_node`: Allocates a block of memory of a specified size and inserts it into a linked list structure.
- `manage_ordered_list`: Manages an ordered list by searching for a position based on a size, potentially splitting nodes, allocating new memory, and calling routines for fixed-point math and list insertion.
- `update_memory_pointer`: If a pointer in d0 is not zero, it calls another routine and stores the resulting pointer, decremented by 2, into a global variable at offset 940 of a4.
- `traverse_linked_list_3`: Iterates through a linked list starting at offset 12 of a structure, pushing the address of the next element and the contents of offsets 4 and 8 of each element onto the stack, then calls another routine.
- `format_path_string`: Calculates the total length of a list of strings, populates a structure with metadata including a count and flags, and constructs a single space-separated string by copying the input strings into a new memory block.
- `resolve_directory_path`: Calls a routine to get the current directory, then iterates through a provided string to find a directory separator or null terminator.
- `process_linked_records`: Saves registers, checks bits 1 and 2 of a byte at offset 25 of a structure, calculates a size by subtracting a value at offset 12 from a value at offset 4, and if the size is non-zero, calls another routine with the size and two other values before calling a third routine using a value from the stack.
- `execute_io_callback`: Checks a global variable, calls another routine, and if the result is -1, calls a second routine from a table and updates a global variable at offset 436 of a4.
- `process_io_with_logic`: Calls another routine and uses a value from a table to determine a result, which is then modified based on the value of a third parameter before returning.
- `execute_io_operation`: Checks if a value at offset 460 of a structure in a4 is non-zero, calls another routine, clears a 16-bit field at offset 436 of the structure, and then calls a routine via a pointer at offset 1012 of the structure, potentially updating offset 436 and offset 1012 with a new value.
- `cleanup_display_context`: It conditionally invokes a callback when a structure field is nonzero, then writes reset values at two fixed offsets.
- `call_conditional_handler`: Checks if a longword at offset 524 of a structure pointed to by a4 is non-zero, then calls a routine via a pointer at offset 1016 of that same structure.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `normalize_fixed_point_divide_3`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `prepare_data_offset`: Sets up a stack frame, saves registers, loads a pointer from offset 484 of a4 into a0, loads a byte from offset -1 of that pointer into d7, compares d7 to 79, and branches to another routine if d7 is less than or equal to 79, otherwise it sets d7 to 79, sets d0 to 79, and branches to another routine.
- `copy_and_write_data`: Copies a block of bytes from one location to another using a loop, clears a byte at an offset from a pointer, calls a routine to find a task, and if a specific field is non-zero, calls a routine to write data to a file before returning -1.
- `open_library_and_store_result`: Calls a routine via a pointer from a global variable, stores the result in a global variable at an offset from a pointer in a6, and then branches to another routine.
- `invoke_parameterized_function`: Stores a pointer derived from a register into a structure offset, pushes several arguments onto the stack, and calls another routine.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `create_message_node`: It obtains a global variable and calls another routine through a table.
- `get_table_entry`: Sets a value at offset 436 of a structure to zero, then checks if a value from the stack is within a specific range and points to a non-zero entry in a table to return a pointer, otherwise it sets an offset in the structure to 9 and returns zero.
- `check_and_set_signal`: Calls a library routine to check a status, and if a specific bit is set and a pointer in a global variable is non-zero, calls the routine pointed to by that global variable and clears it.
- `call_check_routine`: Calls another routine and then returns.
- `fall_through_entry_5`: Entry point part-way through straight-line code: runs one instruction (ori.l) and then falls through into the code that follows, with no return of its own.
- `process_task_message_2`: Performs bitwise operations on register d0 and branches to another routine.
- `process_task_message`: Retrieves a message from a task, stores a pointer from the message into a global variable, and updates a task's function pointer with the address of this routine.
- `update_global_params`: Stores a value into a global variable, clears another global variable, and calls two other routines using pointers from a second parameter.
- `perform_io_operations`: Uses several library calls to manage devices, tasks, and ports, and modifies a byte in a structure by setting its lowest bit.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `negate_register`: Returns the two's complement of the value in register d0 and returns.
- `multiply_fixed_point_values`: Takes two 32-bit fixed-point values in registers and returns their 32-bit product by multiplying the high and low 16-bit components and summing the cross products.
- `perform_signed_division`: Performs a 32-bit division of two signed values by calling another routine and adjusting the sign of the result.
- `divide_32bit_fixed_point`: Performs a 32-bit division of a dividend in d0 by a divisor in d1 using a normalization process to align the divisor.
- `prepare_data_offset_2`: Sets up a stack frame, saves registers, loads a pointer from offset 484 of a4 into a0, loads a byte from offset -1 of that pointer into d7, compares d7 to 79, and branches to another routine if d7 is less than or equal to 79, otherwise it sets d7 to 79, sets d0 to 79, and branches to another routine.
- `copy_bytes_and_call`: Copies bytes from a source in a0 to a destination in a1 using a loop controlled by d0, clears a byte at a specific offset in a structure in a5, and calls a routine using a pointer from a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_task_and_call_function`: Calls three routines using pointers from a6, stores a pointer to itself in a structure in a4, and calls a routine using a pointer calculated from a value in a global variable and the stack pointer.
- `update_global_params_2`: Stores a value into a global variable, clears another global variable, and calls two other routines using pointers from a second parameter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_string_with_limit_2`: Calculates the length of a null-terminated string from a source pointer and compares it against a provided maximum length.
- `copy_string_to_buffer_2`: The routine finds the terminating zero byte of the destination buffer and copies bytes from the source pointer into that location until a zero byte is copied.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `free_linked_list_and_clear_offsets`: Iterates through a linked list by calling a routine to free each node, then stores zero in two specific offsets of a structure in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_memory_and_call_routine`: Performs arithmetic on values in a structure, updates a pointer in a structure, iterates through a linked list, and calls a routine using a pointer from a structure in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_and_reallocate_memory_list`: Iterates through a linked list of memory blocks to find a suitable chunk, performing a split if the block is larger than required or freeing all remaining blocks in the list if the search fails.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_custom_stack_pointer`: Adjusts the stack pointer by four bytes, writes a specific memory address to a global variable, and saves several registers to the stack before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `execute_conditional_logic_2`: Clears a range of memory in a loop, sets a signal, and calls a routine via a pointer retrieved from a memory location.
- `close_library_and_cleanup`: Stores values from a structure into stack offsets, calls a routine via a pointer, calls a library function to close a library, and then calls another library function to close a second library.
- `restore_registers_and_clear`: Restores several registers from the stack and then returns.
- `execute_conditional_branch`: Performs an OR operation on a byte at a pointer, calls a routine via a pointer in a6, and returns if a register is zero.
- `execute_conditional_branch_with_offset`: Loads an address into a1, clears d0, calls a routine via a pointer in a6, and returns if d0 is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_and_test_pointer`: Loads a pointer from a memory address into a register, tests its value, and then performs several byte-sized operations based on the result.
- `conditional_return`: Returns immediately if a condition is not met, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_to_global_structure`: Compares the stack pointer to a global value, otherwise it calls two routines and then writes a value from a global variable into a structure at an offset of 184 relative to a pointer in a0.
- `open_library_and_check_memory`: Calls a routine to determine a value, opens a library, calls a function via a pointer from that library, and then calls a routine to check available memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `request_available_memory`: Calls a library function to check available memory and returns a value derived from the result.
- `find_task_and_call`: Compares the stack pointer to a global variable, saves several registers to the stack, and calls a routine using a pointer from the stack.
- `initialize_system_state_2`: Copies a longword from a pointer in a0 to a global variable, sets that global variable's offset 184 to negative one, calls another routine, and then performs a series of byte-wise zeroing operations on d0 before calling another routine with a pointer from a stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_string_data`: Calculates the length of a null-terminated string and copies its contents to a new location using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_record_data_3_2`: Increments a 16-bit value at offset 7240 of a4 and decrements a 16-bit value at offset -4846 of a4; if the latter is not below zero, it copies a byte from a pointer at offset -4852 of a4 to a new location, increments that pointer, and calls another routine.
- `clear_and_call_format`: Clears a 16-bit word at offset 7240 of a4, pushes two pointers from the stack onto the stack, calls another routine, restores the two pointers from the stack to a2 and a3, and returns the 16-bit value from offset 7240 of a4 in d0.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `execute_conditional_logic`: Saves registers to the stack, calls a routine, and if a bit in a byte at an offset of a register is not set, calls another routine before returning.
- `copy_string_to_buffer_3`: Iterates through a null-terminated string, decrements a counter in a global variable, and copies each byte to a destination buffer until the counter reaches zero or the end of the string is reached.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_callback_loop`: Loops through a null-terminated string and calls a routine pointed to by a register for every byte in the string.
- `initialize_system_state`: Checks a value at offset 7542 of a structure, performs a series of data copies and memory operations involving a jump table, calls a library routine, and updates several word-sized fields in a structure based on the result.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `update_structure_from_call`: Calls a routine using a pointer from a structure, stores the result in two offsets of a structure, and clears two other word-sized fields in that structure.
- `clear_flags_and_counters`: Ands the byte-sized mask 243 into the word at offset 18 of the structure in a3, then clears the words at offsets 10 and 8 of that structure to zero.
- `update_structure_fields`: Calls a routine using a pointer from a structure, stores the result in two offsets of a structure, and clears two other word-sized fields in the structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `iterate_and_call_routine`: Decrements a value from a structure and enters a loop that checks a bit in a word at an offset, calls another routine if the bit is not set, and performs a final call after the loop.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_counter`: Calculates a value based on a parameter, updates a counter in a structure, stores a pointer in a structure, and sets two fields in a structure pointed to by a parameter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_multiple_bytes`: Restores registers from the stack, loads a pointer from a structure, and performs five consecutive byte-sized OR operations with zero.
- `branch_to_offset`: Performs a short branch to an offset of 8.
- `nullify_structure_pointers`: Sets the words at offsets 7440 and 7444 of a structure to zero and restores registers from the stack.
- `allocate_and_link_structure`: Allocates memory, populates a new structure with a pointer, a value from a structure, and zero, then links this new structure into two different locations in a structure.
- `manage_memory_pool`: Checks if a requested size fits in a memory pool, otherwise calls two routines to allocate and link a new structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_and_call_list`: Traverses a linked list starting from an offset in a structure, calls another routine, and performs bitwise operations on a value returned from a call.
- `execute_conditional_library_call`: Checks a value in a structure, calls another routine if the value is non-zero, clears a word in a structure, and calls a library routine using a pointer from a structure.
- `return_modified_d5`: Compares a long value in d5 with negative one, and if they are not equal, returns the value from d5; otherwise it returns the value from d5 with bit 0 set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_function`: Saves registers, loads a value from a stack offset, checks a long value at an offset from a4, calls a routine at an offset from the program counter, loads a value from an offset from a4 into a6, and calls another routine at an offset from a6.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_table_offset`: Saves d7 to the stack, loads a word from the stack into d7, clears a value at an offset from a4, and if d7 is positive and less than a value at an offset from a4, calculates a pointer to a table entry by multiplying d7 by 6 and adding it to a base address; otherwise it returns the original d7 value.
- `set_signal_bit`: Saves registers, loads the value 49152 into d1, and calls a routine at an offset from a pointer in a6.
- `check_bit_and_clear_d0`: Performs a bitwise AND on d7 with 12288, and if the result is not zero, performs eight OR operations on d0 with zero before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_d0_on_condition`: Saves a6 to the stack, checks a long value at a specific address, and if it is not zero, performs twelve OR operations on d0 with zero before restoring a6 and returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_d0_on_test`: Saves a6 to the stack, tests a word at a specific address, and if it is not zero, performs six OR operations on d0 with zero before restoring a6 and returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_multiple_subroutines`: Calls four different subroutines in sequence and then performs a branch to a fixed address.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_library_conditionally`: Saves a6 to the stack, loads a pointer from an offset from a4 into a6, and performs a series of checks on four different long values at specific addresses; if any are non-zero, it performs several OR operations on d0 before calling a routine at an offset from a6 using a string pointer from a table.
- `call_library_function_2`: Loads a pointer from an offset of a4 into a6, calls a routine at an offset from a6, and performs an OR operation on d0 with zero.
- `call_close_library`: Loads a pointer from an offset from a4 into a6, calls a routine at an offset from a6, and performs an OR operation on d0 with zero.
- `configure_hardware_registers`: Writes two 16-bit values to the custom chip registers, loads a 32-bit value from a global variable into another custom chip register, calls a routine via a pointer in a6, and performs an or operation on d0.
- `store_d0_to_globals`: Copies the 32-bit value from d0 into five different global longword locations and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_clear_d0`: Pushes a6 to the stack, checks a word at a global address, and if non-zero, performs several or operations on d0 before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_global_pointer`: Saves registers to the stack, loads a pointer from a global variable into a3, and if a3 is non-zero, performs several or operations on d0 and clears the global variable before returning.
- `clear_multiple_globals`: Checks three different global locations; if the first is non-zero, it performs several or operations on d0, then checks the second and third locations, and finally clears a longword at a specific global address if certain conditions are met.

### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3)[a2].adf:baron#unpacked

- `initialize_file_io_environment`: Clears a memory block, sets a signal on the current task, and opens a library.
- `execute_callback_chain_2`: Calls a function pointer from a structure, followed by two other subroutines, and then performs cleanup by either freeing a block of memory or closing a file, unlocking it, and replying to a message depending on a flag.
- `initialize_graphics_and_intuition_2`: Attempts to open the graphics and intuition libraries, and if successful, copies strings from a table into local buffers to manage display contexts.
- `process_input_stream_2`: Calls two routines and checks if the result is 10 or 27; if not, it decrements a 16-bit counter in a global variable and, if the counter is not below zero, reads a byte from a table at a global pointer and increments that pointer, otherwise it calls another routine to read a byte.
- `process_linked_list_nodes_2`: Calls a routine via a pointer from a global variable, stores the result in a global variable, and then iterates through a linked list to copy pointers into a global array.
- `update_list_via_table`: Iterates through a linked list, copying a longword from a table into a field at offset 4 of the current list element, calls a routine via a pointer from a global table, and updates a global pointer.
- `execute_indirect_jump`: Performs an indirect jump to an address stored at an offset from the current program counter.
- `execute_indirect_jump_2`: Performs an indirect jump to an address stored at an offset from the current program counter.
- `call_indirect_function_2`: Loads several values from the stack into registers and calls a routine using a function pointer retrieved from an offset relative to a value stored on the stack.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `validate_pointer_and_execute_2`: Calls another routine to retrieve a pointer, checks if bit 3 of the byte at offset 1 of that pointer is set, and if so calls a second routine with a value of 2; it then calls a third routine using a pointer from offset 2 of the first pointer, and finally checks if a value at offset 436 of a structure in a4 is non-zero before returning a result.
- `dispose_message_port_and_segment`: Waits for a message port and loops until a specific message is received, then replies to the message and conditionally calls another routine.
- `process_record_data_2`: Iterates through a null-terminated string, decrements a 16-bit counter in a global variable, and either writes bytes to a table at offset 52 of a4 or calls another routine when the counter is too low.
- `parse_command_string_and_update_io`: Parses a string into a table of pointers, handling quoted substrings and expanding escape sequences, then updates a global structure by opening files, performing input/output operations, and setting specific bitmasks and integer values in a control structure.
- `manage_list_node_2`: Checks a flag and a 16-bit value in a structure; if conditions are met, it calls manage_free_memory_list to allocate memory, updates two pointers in the structure with the result, and if the result is zero, sets a global variable to 12 and returns -1; otherwise it clears two 16-bit fields and returns 0.
- `manage_record_data_2`: Processes a data structure by checking flags and bitmasks, updating a 16-bit value at offset 16, and manipulating a pointer at offset 4.
- `fall_through_entry_6`: Entry point part-way through straight-line code: runs one instruction (movep.w) and then falls through into the code that follows, with no return of its own.
- `process_encoded_stream_data`: Parses a byte stream from a buffer pointer within a structure, handling escape sequences for nested data or length prefixes.
- `iterate_table_with_conditional_call`: Iterates through a table of values starting at offset 496 of a structure, checking if a byte at the current index is non-zero and has bit 4 cleared, then calls another routine using a pointer found at offset 2 of the table entry.
- `copy_string_with_limit_3`: Calculates the length of a source string and a destination string, then copies bytes from the source to the destination up to a specified maximum length.
- `append_string_to_buffer`: Locates the null terminator of a destination buffer and copies bytes from a source pointer into that location until a null byte is encountered.
- `insert_into_ordered_list_2`: Aligns a requested size to a 4-byte boundary and attempts to insert a new memory block into a linked list of free memory nodes.
- `execute_io_with_pointer_2`: Calls a routine, and if the result is non-zero, calls another routine using a pointer from the first result, then checks a 16-bit value at offset 436 of a structure in a4 to determine the final return value.
- `free_linked_list_2`: Iterates through a linked list by following pointers, calls a memory freeing routine for each node using the size stored at an offset of 8 from the node pointer, and finally clears the head pointer and a secondary pointer in a structure.
- `reallocate_memory_block`: Checks a global pointer to see if it is non-zero, and if so, calls another routine to free the memory block at that pointer before clearing the global pointer.
- `validate_nested_pointer_and_check_flag_2`: Calls another routine to retrieve a pointer, then calls a second routine using a value from that pointer; if the first call fails or if a 16-bit value at offset 436 of a structure in a4 is zero, it returns negative one, otherwise it returns the result of the second routine.
- `allocate_and_link_node_2`: Allocates a memory block with a size increased by 12, initializes the first three words of the block with a size value, a pointer to a global variable, and zero, then inserts this block into a linked list by updating several global pointers.
- `manage_free_memory_list`: Aligns a requested size to a 4-byte boundary and searches a linked list of free memory nodes to find a suitable block.
- `free_and_update_memory_pointer`: Checks if a passed pointer is non-zero; if so, it calls another routine to free the memory at that location and then stores the pointer decremented by 2 into a global variable at offset 940 of a4.
- `traverse_linked_list_and_push_elements`: Checks if a pointer at offset 12 of a structure is null; if not, it iterates through a linked list, pushing the addresses found at offsets 4 and 8 of each element onto the stack before calling another routine.
- `format_path_string_2`: Calculates the total length required for a list of strings, allocates a memory block and an entry, and populates a structure with metadata including a count and flags.
- `resolve_directory_path_2`: Iterates through a string to find a directory separator or null terminator, then attempts to navigate the file system by locking and changing the current directory through a series of pointer dereferences.
- `process_linked_records_2`: Checks bits 1 and 2 of a byte at offset 25 of a structure, calculates a size by subtracting a value at offset 12 from a value at offset 4, and if the size is non-zero, calls another routine with the size and two other values before calling a third routine using a value from the stack.
- `execute_io_callback_2`: Checks a global variable and calls a routine if it is non-zero; then performs an I/O operation and, if that operation returns negative one, calls a second routine from a table to update a global variable and sets a value at offset 1012 of the structure in a6.
- `process_io_with_logic_2`: Calls another routine to check a signal handler, then performs a seek operation using parameters from the stack.
- `execute_io_operation_2`: Checks if a value at offset 460 of a structure in a4 is non-zero, calls a routine, clears a 16-bit field at offset 436 of the structure, and then calls a routine via a pointer at offset 1016 of the structure; if the primary call returns -1, it calls another routine via a pointer at offset 1016, updates offset 436 with that result, and sets the value at offset 1012 to 5.
- `dispose_msgport_structure`: Checks if a field at offset 16 is non-zero to call a removal routine, then writes a byte of 255 to offset 8 and a longword of negative one to offset 20 of the structure.
- `call_conditional_handler_2`: Checks if a longword at offset 524 of a structure pointed to by a4 is non-zero, then calls a routine via a pointer at offset 1016 of that same structure.
- `multiply_fixed_point_values_2`: Takes two 32-bit fixed-point values in registers and returns their 32-bit product by multiplying the high and low 16-bit components and summing the cross-products.
- `divide_32bit_fixed_point_3`: Normalizes the sign of two 32-bit integers and calls a division routine to return a 64-bit result.
- `divide_32bit_fixed_point_2`: Performs a 32-bit division of a dividend in d0 by a divisor in d1 using a normalization process to align the divisor, returning a 64-bit result containing the quotient in the high 32 bits and the remainder in the low 32 bits.
- `check_string_byte`: Loads a byte from an offset of a pointer retrieved from a structure and compares it to a constant, branching if it is less than or equal to that constant.
- `copy_and_write_string`: Copies a string from one location to another using a loop, then attempts to write a character and a string to a file via a task's file handle.
- `open_library_and_store_result_2`: Calls a routine via a pointer from a global variable, stores the result in a global variable at an offset from a pointer in a6, and then branches to another routine.
- `call_indirect_function_3`: Pushes several arguments onto the stack and calls a routine via a function pointer, then returns zero if the result is one, otherwise it returns negative one.
- `function_epilogue_4`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `create_message_node_2`: Allocates a signal and a memory block to initialize a new message port structure using values from a table.
- `get_table_entry_2`: Sets a word at offset 436 of a structure to zero, then checks if a value from the stack is non-negative and less than a value stored at offset 0 of that structure; if so, it verifies that a table entry at an offset calculated by multiplying the value by 6 is non-zero and returns a pointer to that entry, otherwise it sets a word at offset 1012 of the structure to 9 and returns zero.
- `check_and_execute_signal_handler`: Checks if a specific bit is set in the return value of a library call and, if so, executes a routine stored in a global variable and clears that variable.
- `call_subroutine_and_return`: Calls another routine and then returns.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_input_device`: Checks if a global word is zero, then finds a task, adds a port, opens a device, and performs I/O operations using library calls.
- `close_device_and_remove_port`: If a global word is non-zero, it clears that word and calls three different routines via a library base to perform I/O operations and remove a port.
