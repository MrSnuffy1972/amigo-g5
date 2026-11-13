# Game 419 (Red Baron): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00419). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2719448 top-level bytes in 105 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 50 of 50 units identical (bytes and relocations), 43 raw-word lines |
| Functions named and described | pass, 391 of 391 named (0 author symbols), 391 of 391 described (47 by rule, 277 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2657 of 2668 (99.59%) |
| Recovered without load address (counted in proof 1, flagged) | 1800694 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10410 of 10494 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 106393 |
| code | 24312 |
| data | 2347081 |
| padding | 16894 |
| unused | 224768 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:install | 10 | 250 | 17 |
| Run | 2 | 263 | 8 |
| More | 2 | 263 | 8 |
| Avail | 11 | 376 | 12 |
| AddBuffers | 2 | 117 | 2 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:sim | 8 | 250 | 17 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:ps | 9 | 250 | 5 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:baron | 9 | 250 | 5 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:nem | 2 | 85 | 3 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:amigasnd.drv | 3 | 1820 | 112 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:bootblock | 1 | 18 | 1 |
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
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:baron#unpacked | 5 | 2381 | 55 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:credits.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:volume.001 (raw code file, load address unknown) | 1 | 0 | 0 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 2 of 3).adf:volume.002 (raw code file, load address unknown) | 1 | 0 | 0 |
| Red Baron (1990)(Dynamix)[cr Nemesis](Disk 3 of 3).adf:volume.003 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- red baron (1990)(dynamix)[cr nemesis](disk 3 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 3 of 3).adf:volume.003: high-entropy data file (7.79 bits/byte), classified as packed data
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
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:install: Imploder (xfd) stream found by ancient scan (12376 -> 20036 bytes) but only 1 of 10 sampled windows are in reference RAM: not used
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:c/Run: PowerPacker 4.0 (xfd) stream found by ancient scan (2356 -> 2568 bytes) but only 0 of 2 sampled windows are in reference RAM: not used
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:c/More: PowerPacker 4.0 (xfd) stream found by ancient scan (6960 -> 11284 bytes) but only 0 of 6 sampled windows are in reference RAM: not used
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:sim: Imploder (xfd) stream found by ancient scan (173172 -> 308692 bytes) but only 1 of 147 sampled windows are in reference RAM: not used
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:ps: Imploder (xfd) stream found by ancient scan (154520 -> 312268 bytes) but only 0 of 149 sampled windows are in reference RAM: not used
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:baron: crunched executable decrunched by XFDMaster (Imploder (xfd), 6564 -> 8980 bytes), 3 of 4 windows confirmed in reference RAM: recovered as a hunk executable
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:nem: ByteKiller 1.3 (xfd) stream found by ancient scan (1712 -> 2028 bytes) but only 1 of 1 sampled windows are in reference RAM: not used
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:credits.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:volume.001: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- red baron (1990)(dynamix)[cr nemesis](disk 2 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 2 of 3).adf:volume.002: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- red baron (1990)(dynamix)[cr nemesis](disk 3 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 3 of 3).adf:volume.003: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:roster.dat: raw file found loaded verbatim in runtime segment an address (f100) at an address; classified through the recovered segment
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf: blocks 915-915 (bytes 468480-468992) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.35)
- red baron (1990)(dynamix)[cr nemesis](disk 2 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- red baron (1990)(dynamix)[cr nemesis](disk 3 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:c/More: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:sim: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:ps: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red baron (1990)(dynamix)[cr nemesis](disk 1 of 3)/Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:baron: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 4 never-loaded region(s), 1800694 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_hardware_and_coprocessor` (runtime segment an address (f100)): Waits for a bit in a hardware register, sets several hardware registers including CIA and custom registers, and performs a series of loops to update values in a table based on bitwise checks. Kind: disk hardware/timing check. Pass path: the routine continues to exec.OpenLibrary and proceeds to normal execution after the timing loops and CIA/Custom chip register manipulations complete successfully..

## Functions


### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:install

- `initialize_game_state`: Executes a sequence of function calls and checks a global pointer to swap a memory value if a specific constant is found.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `initialize_cache_and_loop_counter`: Clears the instruction cache, increments a counter, and executes a loop that calls a routine up to six times while performing pointer arithmetic and conditional calls to a function pointer or a table-based routine.
- `update_buffer_with_offset`: Doubles d0, and if positive, branches away; otherwise, calls another routine, copies d0 to d1, and if d0 is not zero, calls another routine, copies d0 to d6, calls a routine via a4, copies a1 to d3, and if d0 is not zero, adds d0 to d2 and adds d3 to a memory location at a3 plus d2, decrementing d1 and looping if d1 is not zero.
- `set_destination_pointer`: If a condition is met, copies a pointer from a3 to a1, calls another routine, and then branches away.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_bytes_from_source`: Compares a pointer in a6 to a2, and if not equal, copies a byte from a decreasing source pointer in a2 to an increasing destination pointer in a1, repeating this or a certain number of times via d0, then loads a byte from a1 into d0 before returning.
- `decompress_bit_stream_chunk`: Expands a bit-encoded or compressed-length data stream by reading a control byte, then uses a loop to shift and append bits or copy a certain number of bytes from a source pointer to a destination pointer, while updating a counter or size or offset.
- `process_bit_stream_chunk`: Sets d7 to 2 and checks if d1 is even; if not, it decrements a pointer in a0, loads a byte, and uses a loop that decrements d7 to perform arithmetic on d1 and d0 before jumping to a target address.
- `initialize_stream_parameters`: Sets d0 to 5, d7 to 3, and jumps to a target address.
- `initialize_stream_parameters_2`: Sets d0 to 4, d7 to 2, and jumps to a target address.
- `initialize_stream_parameters_3`: Sets d0 to 3, d7 to 1, and jumps to a target address.
- `expand_bit_encoded_stream`: Calculates a target copy size by processing a bit-encoded or compressed-length data stream, updating a pointer in a0 and a size or offset, and potentially calls decompress_and_expand_data_stream or modify_d1_and_branch.
- `modify_d1_and_branch`: Multiplies d1 by two and branches if the result is non-zero, otherwise it decrements a pointer in a0, adds the new d1 value to d1 with carry, adds d2 to d2 with carry, and jumps to another routine.
- `decompress_and_expand_data_stream`: Reads a bit-count-prefixed or control-byte-prefixed stream to calculate a target copy size, then expands bit-encoded or compressed-length data to copy a certain number of bytes from a source pointer to a destination buffer, potentially updating a global counter or pointer.
- `expand_and_copy_data_stream`: Reads a control byte or a bit-count-prefixed byte to determine a certain number of bytes to copy, then copies bytes from a source pointer to a destination pointer, potentially expanding the data or adjusting the destination pointer, and finally restores registers and branches to another routine.
- `push_registers_to_stack`: Pushes d0, d1, and d2 onto the stack, restores several registers, and branches to another routine.

### Run

- `process_amiga_hunk_relocations`: Parses a data structure containing hunk type codes to perform memory allocation and relocation.
- `process_pointer_and_call`: Performs arithmetic on a pointer in a1, calls a routine via a pointer in a3, and if a condition is met, calls another routine and stores a shifted value from d0 into a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `expand_and_copy_data_blocks`: Reads a count from a pointer in a0, calculates a destination address using bit shifts, and enters a loop that copies variable-sized blocks of data from a source table to a destination buffer in a3.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `extract_bits_and_call`: Performs bitwise shifts and rotations using d4 and d5, and calls several other routines based on comparisons and stack operations.
- `execute_trap_instruction`: Moves a word from a table at an offset of a2 into d4, performs a bitwise OR on d0, calls a trap, and performs a bitwise OR on d1.

### More

- `apply_relocation_hunks`: Parses a data structure containing hunk type codes to perform memory relocations and data expansions.
- `process_loop_and_swap`: Decrements a 16-bit counter in a loop, and if the counter is not equal to 978, performs memory arithmetic, calls a routine, and potentially updates a global variable with a shifted value.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `process_bit_stream`: Extracts a byte from a pointer in a0, then enters a loop that performs bitwise shifts and calls other routines to process data into a table at a3.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `move_word_to_register_2`: Moves a 16-bit word from a memory location pointed to by register d4 into register d0.
- `extract_bitstream_to_memory`: Performs bitwise shifts and rotations on a bitstream to extract variable-length bit fields, then writes the resulting values into a destination buffer in a3 using a loop controlled by a count in d0.
- `trap_with_data`: Moves a word from a pointer in a2 to d4, performs a bitwise OR on d0 with 20, calls a trap, and performs a bitwise OR on d1 with d0.

### Avail

- `parse_command_arguments`: Parses a command string by splitting it into tokens based on spaces or quotes, handling escape characters for newlines and escape symbols, and stores the resulting pointers in a structure.
- `process_message_reply`: Updates a message structure by storing a value at offset 12, calls three different routines based on flags in the structure, and returns a value from the structure.
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
- `move_and_branch`: Copies the contents of register d1 to d0 and then performs a short branch to a location further ahead in the code.

### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:sim

- `initialize_system_memory`: Performs a series of function calls, decrements a counter in a loop, and checks a specific memory pattern to update a global variable.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.
- `execute_array_accumulation_loop`: Clears d5, increments d6, calls a routine, sets a pointer to a large offset, increments d6, calls a routine, and then loops a certain number of times to call another routine and add a value from a pointer-indexed table into a destination array, finally checking a global variable and calling another routine if a condition is met.
- `accumulate_array_elements`: Calculates a value, calls a routine, and then loops to add a value from a pointer-indexed table into a destination array, decrementing a counter until it reaches zero.
- `set_destination_pointer_2`: If a condition is met, copies a pointer from a3 to a1, calls another routine, and then branches away.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_bytes_from_source_2`: Compares a pointer in a6 to a2, and if not equal, copies a byte from a decreasing source pointer in a2 to an increasing destination pointer in a1, repeating this or a certain number of times via d0, then loads a byte from a1 into d0 before returning.
- `expand_bitstream_buffer_chunk_2`: Expands a bit-stream or compressed-style data by doubling or shifting values and writes the resulting bytes or words into a destination buffer.
- `expand_bitstream_buffer_chunk`: Sets d7 to 2 and checks if d1 is even; if not, it decrements a pointer in a0, loads a byte, and uses a loop that decrements d7 to perform arithmetic on d1 and d0 before jumping to a target address.
- `initialize_buffer_expansion`: Sets d0 to 5, d7 to 3, and jumps to a target address.
- `initialize_buffer_expansion_2`: Sets d0 to 4, d7 to 2, and jumps to a target address.
- `initialize_buffer_expansion_3`: Sets d0 to 3, d7 to 1, and jumps to a target address.
- `expand_bitstream_to_buffer_2`: Expands a bit-stream or compressed-style data by doubling or expanding values and writes the resulting bytes or words into a destination buffer.
- `update_d1_and_jump`: Multiplies d1 by two and branches if the result is non-zero, otherwise it decrements a pointer in a0, adds the new d1 value to d1 with carry, adds d2 to d2 with carry, and jumps to another routine.
- `expand_bitstream_to_buffer`: Reads a bit-stream or compressed-style data, expanding or doubling values, and writes the resulting bytes or words into a destination buffer.
- `expand_and_copy_buffer_data`: Reads a value from a table, then enters a loop that expands bit-stream or compressed-style data by doubling or shifting values, and subsequently copies or moves bytes between memory buffers using pointers in a2, a3, and a5, finally restoring registers and returning to another routine.
- `push_registers_to_stack_2`: Pushes d0, d1, and d2 onto the stack, restores several registers, and branches to another routine.

### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:ps

- `free_blocks_and_restore_patch`: Calls two unnamed routines, loops to free five memory blocks, and restores a saved word in a table if a specific masked pattern is matched.
- `decrement_until_minus_one_3`: Decrements a local variable in d7 until it reaches negative one and then returns.
- `execute_array_accumulation_loop_variant`: Clears d5, increments d6, calls a routine, sets a pointer to a different large offset, increments d6, calls a routine, and then loops a certain number of times to call another routine and add a value from a pointer-indexed table into a destination array, finally checking a global variable and calling another routine if a condition is met.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_from_source`: Checks if a pointer in a2 equals a pointer in a6, otherwise it copies four bytes from the source pointed to by a2 to the destination pointed to by a1 while decrementing the source pointer, then decrements a counter in d0 and loops if not zero, otherwise it loads a value from the destination pointer into d0 and returns.

### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:baron

- `free_memory_and_patch_table`: Calls a routine twice, then enters a loop to call another routine and execute a function from a pointer found in a global variable until a counter reaches four.
- `loop_through_a_memory_address`: Loads a 32-bit address from a location relative to the program counter, shifts that address left by two bits, and then iterates a loop a number of times specified by a 16-bit value in d7, adding 4 to the resulting address in each iteration before returning.
- `expand_bitstream_data_2`: Performs several calls to a routine via a pointer in a4, updates a value in a table using a calculated index, and calls another routine if a comparison with a value from a global variable succeeds.
- `load_a1_and_d0`: Loads the address of a label into a1 and sets d0 to 2.
- `expand_bitstream_data`: Decodes a variable-length bitstream by reading bytes from a source buffer and performing bitwise shifts and additions to reconstruct values.

### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:nem

- `depack_packed_stream_backward`: Decodes a packed stream by calling a bit-reader refill and a multi-bit read routine, copying literal and back-reference runs backwards into memory.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:amigasnd.drv

- `execute_indirect_call_2`: Performs an indirect function call using a pointer derived from a global variable and a register value.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `compare_and_set_status`: Compares bytes from two tables at offsets 520 and 524 of a4 using d0 and d1 with an index in d2; if they match and a byte at offset 440 of a4 indexed by the value at offset 520 is not zero, it writes 1 to offset 528 of a4 at the index in d2.
- `write_status_bytes`: Writes the byte 1 to an offset of a1 determined by d2, writes the byte 0 to another offset of a1 determined by d2, and branches to another routine.
- `pop_and_return`: Increments d2 and compares it to 4; if not equal, it pops five values from the stack into a1, a0, d3, d2, and returns.
- `swap_and_call_routine`: Pushes d2, a0, and a1 onto the stack, swaps the contents of d1, and if the low byte of d1 is zero, calls another routine; otherwise, it swaps d1 again and branches to another routine.
- `find_audio_parameter_match`: Swaps d1, compares a byte at offset 520 of a4 with d0 and a byte at offset 524 of a4 with d1, and if both match it writes 0 to offset 528 of a4 and calls two routines; otherwise it branches back to the start of the routine.
- `increment_and_branch_parameter`: Increments d2, compares it to 4, and branches if not equal; otherwise it calls another routine, compares d2 to 255, and calls another routine before popping registers and returning.
- `update_audio_parameter_byte`: Checks if a byte in a register is equal to seven, then swaps the register and performs bitwise operations to derive a value which is stored at an offset of 392 plus a register index into a structure pointed to by a4.
- `compare_and_call_swapped_byte`: Compares d1 to 64 and, if equal, swaps d1, calls another routine, and then swaps d1 again before branching to another routine; otherwise it branches to another routine.
- `write_swapped_byte`: Compares d1 to 1, and if not equal it branches to another routine; otherwise it loads a pointer from offset 264 of a4, swaps d1, writes the byte to the pointer at offset d0, swaps d1 again, and branches.
- `swap_and_call_data`: Compares d1 to 75, and if equal it swaps d1, calls another routine, swaps d1 again, and branches.
- `search_audio_data`: Compares a byte in d0 against a sequence of bytes in a global structure and updates a local variable if a match is found and the corresponding byte at offset 524 is not 255.
- `store_byte_at_offset`: Stores a byte into a memory location calculated from a base address, an offset, and a word value.
- `update_audio_register`: Calculates a 16-bit value based on d1 and d0, stores it at an offset in a4 determined by d0 and d1, and then iterates through a four-byte sequence to check if a byte at offset 520 of a4 matches d0, calling another routine if a match is found.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `get_and_set_byte`: Returns a byte from a memory location and, if the input byte is not negative one, updates that location with the input byte.
- `get_and_set_word`: Returns a word from a memory location and, if the input word is not negative thirty-two thousand seven hundred sixty-eight, updates that location with the input word.
- `update_byte_via_pointer`: Reads a byte from a memory location determined by adding an offset to a value in a register and conditionally overwrites it with a value from a register if that register does not contain a specific negative value.
- `update_parameter_loop`: Iterates through a table at offset 504 of a4 to find a byte matching d0, then checks if a corresponding byte at offset 520 of a4 matches d1; if a match is found, it calculates a value using a table at offset 540 of a4 and updates a value in a structure.
- `update_audio_parameter`: Compares a word from a table at offset 532 of a4 with d5, and if d5 is larger, performs a series of swaps and shifts before writing a byte to a structure at a0.
- `update_audio_channel`: Saves registers to the stack, compares a byte at offset 520 of a4 with d0, and if they match it increments two counters and checks if the second counter reaches 4; if it does, it calculates a value from a table at offset 456 of a4, compares it to d1, and if d1 is greater than the value and not equal to it, it subtracts the value from d1 and calls another routine.
- `call_routine_twice`: Subtracts d1 from d4, moves the result to d1, calls a routine, calls another routine, and returns.
- `process_audio_channel_data`: Pushes five registers onto the stack, checks if a byte at offset 520 of a4 is minus one, and if so, increments two counters; if the counters reach specific values, it checks a byte at offset 456 of a4, subtracts a value from it, and calls another routine.
- `write_byte_and_call`: Subtracts d3 from d4, stores the result in a table at offset a0 using d0 as an index, moves d4 to d1, and calls another routine.
- `pop_and_return_loop`: Increments d0 and compares it to 10; if not equal, it pops five values from the stack into a0, d4, d3, d1, and d0, then returns.
- `modify_audio_channel`: Searches for a specific byte value in a structure, updates it, and increments a value at a calculated memory offset.
- `compare_and_clear_byte`: Compares a byte at offset 456 of a structure in a4 indexed by d0 with d1, and if they are equal, clears that byte to zero and branches to another routine; otherwise it branches to another routine.
- `subtract_table_byte`: Subtracts a byte from a table at offset a0 indexed by d0 using the value in d1, stores the result back in the table, and branches to another location.
- `reset_and_update_table`: Subtracts a byte at offset a0 indexed by d0 from d1, clears that byte, and then iterates through tables at offsets 520 and 524 of a4 to find matching values, potentially updating a value at offset 540 of a4.
- `track_maximum_value`: Iterates through a table at offset 532 of a4 to find the maximum word value, then writes 0 or 255 to a structure at a0 based on the results.
- `execute_empty_routine`: This routine consists of ten instructions but performs no observable operation, as it does not modify any registers, memory, or hardware state before returning.
- `update_status_table`: Stores a byte at a calculated offset and, if that byte is zero, iterates through a table to update specific flags based on a matching index.
- `reset_and_search_audio_parameters`: Clears several bytes and words in a structure pointed to by a4, then searches a table for a range matching a value in d1.
- `increment_pointer_and_branch`: Adds 20 to the address in a1 and then performs a short branch to an earlier location.
- `update_audio_parameters`: Calculates several addresses based on a1 and a4, stores four pointers and a 16-bit word into tables at offsets 568, 584, 600, and 616 of a4, updates two bytes in a4 using d1, and calls another routine.
- `call_audio_reset`: Calls a routine and then restores six registers from the stack before returning.
- `clear_audio_channel_data`: Calls another routine and then resets several memory locations and offsets within a global structure to zero or 255 based on an index in d2.
- `fetch_audio_parameters`: Loads a byte from a table at offset 524 of a4 indexed by d2, then uses d2 to fetch four pointers from tables at offsets 568, 584, 600, and 616 of a4, and calls two other routines based on specific conditions.
- `return_stack_value`: Returns a 24-bit value formed by combining a constant with a 4-byte field taken from a 32-bit value located on the stack at a fixed offset.
- `update_audio_voice_counters`: Pushes several registers to the stack, checks if a byte at offset 524 of a4 is not 255, calls another routine, increments a value at offset 532 of a4 using a shifted index, checks if a byte at offset 560 of a4 is not 0, increments a value at offset 540 of a4 using a shifted index, calls another routine, calls another routine, and increments d2 until it reaches 4 before popping registers and returning.
- `update_audio_envelope_state_2`: Updates an audio envelope by modifying a state byte and a volume counter based on values from a table, and calls modify_audio_data if a specific condition is met.
- `update_audio_envelope_state`: Updates an audio envelope by modifying a state byte and a volume counter based on values from a table.
- `add_audio_value`: Loads a byte from offset 68 of a4 using an index from offset 10 of a1, stores it in a structure at offset a0, adds a byte from offset 100 of a4 to that structure, and if the result is less than or equal to a byte at offset 11 of a1, it updates a status byte at offset a6.
- `decrement_byte_conditionally_2`: Compares a byte at offset d2 of a6 with 1, and if it is not 1, branches to another routine; if it is 1, it compares a byte at offset d2 of a0 with 0, and if it is not 0, decrements the byte at offset d2 of a0.
- `subtract_audio_value`: Loads a byte from offset 68 of a4 using an index from offset 12 of a1, stores it in a structure at offset a0, subtracts a byte from offset 100 of a4 from that structure, and if the result is less than or equal to a byte at offset 13 of a1, it updates a status byte at offset a6.
- `copy_byte_conditionally`: Compares a byte at offset a6 indexed by d2 with 2, and if they are equal, it copies a byte from offset d of a1 to offset d2 of a2 and branches to another location.
- `decrement_byte_conditionally`: Compares a byte at offset a6 indexed by d2 to 3, then compares a byte at offset a0 indexed by d2 to 0; if the second comparison is true, it decrements the byte at offset a0 indexed by d2.
- `modify_audio_data`: Subtracts a value from a byte in a structure using an offset from another table and calls itself recursively if a condition is met.
- `calculate_volume_scaling_2`: Loads a byte from a table at offset 520 of a4 indexed by d2, uses that byte to index into a table at offset 392 of a4, and then loads a byte from a table at offset 564 of a4 indexed by d2, clearing d4 if the second byte is zero.
- `branch_on_non_zero`: Compares d4 with 0 and branches to another location if it is not zero; otherwise it branches to a different location.
- `calculate_scaled_product`: Multiplies d3 and d4, divides the result by 63, and if a byte at offset 556 of a4 indexed by d2 is not zero, it uses the result to perform further calculations.
- `calculate_scaling_factor`: Multiplies two 8-bit values from d3 and d4, divides the result by 63, then multiplies this by a value from a structure at offset 662 of a4 shifted left by 2, and finally stores the result in a byte at offset 692 of the structure in a4.
- `calculate_volume_scaling`: Calculates a value based on the product of two inputs and stores the result in a global variable.
- `search_audio_channel_index`: Pushes several registers to the stack, compares a byte at offset 520 of a4 with d1, and if they match it branches to another routine; otherwise it increments d2 and compares it to 4 before branching.
- `update_audio_parameters_2`: Updates several entries in a table using an index in d2, stores a pointer to a structure in a table at an offset calculated from d2, and calls another routine.
- `configure_audio_parameters`: Writes a value to offset 18 of a structure in a3, updates two entries in a table using a pointer from a4 and an index in d2, then fetches data from a table to populate several fields in a structure pointed to by a pointer from a4, before calling another routine and potentially writing a byte to a table at offset 524 of a4.
- `reset_audio_hardware_state`: Calls a routine to reset the audio hardware and returns a value from a memory location via a pointer.
- `search_table_loop`: Pushes d2 and a1 to the stack, then enters a loop that compares a value in a0 to bytes in a table at offset 288 of a4, incrementing d2 until it reaches 16.
- `shift_and_pop_registers`: Shifts the value in d2 right by 2, calls another routine, and then pops two words from the stack into a1 and d2 before returning.
- `search_table_for_match`: Pushes d2, d1, and a1 onto the stack, then enters a loop that compares a0 with values in a table at offset 608 of a4 using d2 as an index, incrementing d2 until a match is found or d2 reaches 16, otherwise it sets d0 to 1 and branches.
- `clear_and_return_byte_2`: Sets a word at a fixed offset to one, retrieves a byte from a table using an index, clears that byte in the table, and returns the byte shifted.
- `clear_and_return_byte`: Sets a word at a fixed offset to one, retrieves a byte from a table using an index, clears that byte in the table, and returns the byte shifted left by sixteen bits.
- `fetch_swapped_data`: Swaps the value in d2, compares it to 192, and if it matches, loads a word from a table at an offset calculated from d2 into d0 before jumping to another routine.
- `fetch_swapped_byte`: Compares d2 with 192, and if not equal it branches to another routine; otherwise it swaps d2, loads a byte from offset 376 of a4 into d0, and jumps to another routine.
- `load_table_byte_swapped`: Compares d2 with 176, and if not equal, branches to another routine; otherwise it swaps d1, compares the lower word of d1 with 7, and if not equal, branches to another routine, finally loading a byte from a table at offset 296 of a4 indexed by the lower 8 bits of d0.
- `fetch_table_byte_2`: Compares d2 with 64, and if not equal, it loads a byte from a table at offset 472 of a4 using d0 masked to 8 bits as an index and branches to another location.
- `fetch_table_byte_conditionally`: Compares d2 to 1; if equal, it loads a byte from a table at offset 456 of a4 using d0 masked to 8 bits as an index.
- `fetch_table_byte`: Compares d2 to 75 and d1 to 255, then if both conditions are met, loads a byte from a table at offset 456 of a4 using d0 masked to 8 bits as an index into d0.
- `find_matching_byte`: Compares a byte in d0 with a byte at an offset of 520 plus a value in d3 from a table in a4, and if they match, increments d3 through a loop until it reaches 4 and returns the result.
- `return_constant_value`: Returns a constant 16-bit value.
- `update_buffer_pointers`: Stores a longword from a0 into offset 132 of a4, adds the value of a4 to the longwords at offsets 140, 172, 204, and 236 of a4, and then calls another routine.
- `get_concatenated_data`: Concatenates two 16-bit values from a table into a single 32-bit result.
- `allocate_audio_memory_blocks`: Pushes several registers to the stack, calls another routine, calls a routine to allocate memory, and then stores the returned pointer and several subsequent pointers incremented by 224 into a table at offsets 5296, 5306, 5310, 5314, 5318, 5322, 5326, 5330, and 5334 of a4; finally it checks if the first stored pointer is not 0 and branches to another routine if it is not.
- `open_audio_device`: Sets up parameters in a structure, calls a routine to open a device, and if the result is 0 it jumps to another location; otherwise it sets d1 to -1 and jumps to another location.
- `initialize_audio_interrupt_handlers`: Initializes four structures with specific values and function pointers, calls another routine four times to store return values in a table, writes to the INTENA register, and calls another routine.
- `release_audio_resources`: Writes 780 to the INTENA register, calls four routines via pointers, calls another routine, and then conditionally calls two more routines via pointers based on values in a4.
- `reset_audio_hardware`: Clears two bytes in a table at offsets 5358 and 5354 of a4, resets a hardware register at offset 8 of a calculated address, calls another routine, and sets a byte at offset 5301 of a4 to 1.
- `update_audio_registers`: Loads a byte and a word from a table into specific hardware registers and clears a third register.
- `calculate_scaled_index_2`: Pushes six registers onto the stack, checks a value at an offset in a4, and if non-zero, performs several arithmetic operations including division and multiplication to load a word from a table at offset 4852 of a4 into d3 before branching.
- `calculate_scaled_index`: Calculates a value by adding a constant to a modified d1, uses d2 to fetch a byte from a table in a4, performs division, and uses the result to add a value from another table in a4 to d1 before branching.
- `update_scaled_value`: Calculates an index by adding d6 to d1 shifted left by 4, loads a longword from a table in a3 at that index into d3, and if a value at offset 8 of a1 is non-negative, subtracts a scaled difference between two longwords from the table in a3 from d3.
- `calculate_scaled_offset`: Subtracts a value in d5 from a value in d6, performs a multiplication using a value from a table in a1, adds the result to d3, and performs bitwise shifts and comparisons.
- `pop_arguments_from_stack`: Loads six values from the stack into registers and moves a seventh value from the stack into a register before returning.
- `write_audio_register`: Writes a 16-bit value to a hardware register.
- `write_audio_register_alt`: Calculates an offset into the audio hardware registers by shifting a 16-bit value left by four bits and stores a 16-bit value from a register into that calculated location, then returns.
- `decrement_and_update_audio`: Decrements a byte in a table at a4 plus an offset, and if it reaches a specific value, updates three hardware audio registers with values derived from a table.
- `update_audio_sample_buffer`: Copies a large block of data from a source buffer to a destination buffer within a structure, then updates a volume or pitch value and sets a flag in a global table based on calculated offsets.
- `update_audio_hardware`: Sets a byte at an offset of 656 plus d2 in a4, calculates a value in d0, writes it to an offset of 632 plus d2 in a4, and then stores a 16-bit word from a table at a6 plus d2 into the AUD0LCH register at an offset determined by d2.
- `store_table_word`: Stores a 16-bit word from a table at a6 plus d2 into a memory location at offset 4 of a0.
- `update_sample_buffer_and_envelope`: Copies a large block of data from a source buffer to a destination buffer based on a pointer and an offset, then updates an audio envelope and volume parameters in a structure.
- `update_audio_hardware_silent`: Sets a byte at an offset of 656 plus d2 in a4, calculates a value in d0, writes it to an offset of 632 plus d2 in a4, and then stores a 16-bit word from a table at a6 plus d2 into the AUD0LCH register at an offset determined by d2, but sets a byte at offset 5358 plus d2 in a4 to zero.
- `store_word_from_table`: Reads a 16-bit word from a table using an offset in a register and stores it into a memory location at a 4-byte offset from a pointer in a register, then returns.
- `check_interrupt_status`: Saves registers and d2 to the stack, loads two addresses relative to the program counter into a0 and a1, compares the byte at a0 with 255, and if it is 255 it sets d2 to 0 and calls another routine; otherwise it returns.
- `set_interrupt_byte`: Compares a byte at address a1 with 0, and if it is not 0, writes the value 255 to the address in a0 and branches to another routine; otherwise it branches to another routine.
- `trigger_interrupt_if_nonzero`: Compares a byte at the address in a0 with 0, and if it is non-zero, calls another routine and branches to another location.
- `trigger_interrupt_request`: Calls another routine, sets bit 7 of the interrupt request register, and returns using values popped from the stack.
- `set_audio_flag_one_2`: Saves registers and d2 to the stack, loads two addresses relative to the program counter into a0 and a1, compares the byte at a0 with 255, and if it is 255 it sets d2 to 1 and calls another routine; otherwise it returns.
- `check_and_set_audio_flag`: Compares a byte at the address in a1 to zero; if it is not zero, it writes the value 255 to the address in a0 and branches to another routine, otherwise it returns.
- `set_audio_flag_one`: Sets register d2 to 1, checks if the byte at the address in a0 is zero, and if it is not zero, calls another routine; otherwise it returns.
- `set_audio_interrupt_flag_high`: Calls another routine, writes 256 to the interrupt request register, and returns using values popped from the stack into registers d2, a4, a1, and a0.
- `set_audio_flag_two_2`: Saves registers and d2 to the stack, loads two addresses relative to the program counter into a0 and a1, compares the byte at a0 with 255, and if it is 255 it sets d2 to 2 and calls another routine; otherwise it returns.
- `check_and_set_audio_flag_2`: Compares a byte at the address in a1 to zero; if it is not zero, it writes the value two hundred fifty-five to the address in a0 and branches to another routine, otherwise it returns.
- `set_audio_flag_two`: Sets register d2 to 2, checks if the byte at the address in a0 is zero, and if it is not zero, calls another routine; otherwise it returns.
- `set_audio_interrupt_request`: Calls another routine, writes 512 to the interrupt request register, and returns using values popped from the stack into registers d2, a4, a1, and a0.
- `set_audio_flag_three_2`: Saves registers and d2 to the stack, loads two addresses relative to the program counter into a0 and a1, compares the byte at a0 with 255, and if it is 255 it sets d2 to 3 and calls another routine; otherwise it returns.
- `check_audio_flag_and_set_byte`: Compares a byte at the address in a1 to zero; if it is not zero, it writes the value two hundred fifty-five to the address in a0 and branches to another routine, otherwise it branches to another routine.
- `set_audio_flag_three`: Sets register d2 to 3, checks if the byte at the address in a0 is zero, and if it is not zero, calls another routine; otherwise it returns.
- `reset_audio_interrupt_flag`: Calls another routine, writes 1024 to the interrupt request register, and returns using values popped from the stack into registers d2, a4, a1, and a0.
- `configure_display_and_interrupt_registers`: Writes specific constant values to the interrupt enable and DMA control registers, then writes specific constant values to the display enable and copper control registers before returning.
- `process_entity_list_and_modify_bytes`: Iterates through a table of pointers, then traverses a linked list of structures where each entry contains a pointer to a data block.
- `decrement_and_loop`: Decrements d0 and compares it to 512, looping if they are not equal, then restores registers and returns.

### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_coprocessor`: Waits for a bit in a hardware register, sets several hardware registers including CIA and custom registers, and performs a series of loops to update values in a table based on bitwise checks.
- `copy_data_table`: Calculates a destination address based on a byte from a table, then copies a block of seven bytes from a source table to that destination in a loop.
- `write_header_values`: Stores three 16-bit values into a table starting at a global address, where the values are derived from the high words of a constant and specific constants.
- `clear_memory_block`: Clears a block of memory consisting of 4096 words starting at a global address using a loop.
- `update_graphics_parameters`: Copies three 16-bit values from a structure to another structure, calls a library routine, and then sets three 16-bit values in a structure with the bitwise OR of their original values and 32768.
- `pop_multiple_longs`: Decrements the value in a0 by 20 and then pops six 32-bit words from the stack into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_dos_message`: Waits for a message on the current task's port, retrieves it, and if the message contains a pointer to a lock, duplicates that lock and retrieves the current directory.
- `execute_callback_chain`: Calls several functions via pointers stored in a structure and calls free_linked_list if a specific field is non-zero.
- `initialize_graphics_and_intuition`: Attempts to open the graphics and intuition libraries, and if successful, copies a string from a table into a local buffer and calls a routine to manage display contexts.
- `process_input_stream`: Calls another routine, calls another routine, and if the result is not 10 or 27 it decrements a 16-bit counter in a global variable and, if not below zero, reads a byte from a table at a global pointer, increments that pointer, and calls another routine; otherwise it calls another routine and returns.
- `process_linked_list_nodes`: Calls a routine via a pointer from a global variable, stores the result in a global variable, and then iterates through a linked list to copy pointers into a global array.
- `update_linked_list_data`: Iterates through a linked list, copying a longword from a table into a field at offset 4 of the current list element, then calls a routine via a pointer from a global table and updates a global pointer.
- `jump_to_address_3`: Performs a long jump to a specific address.
- `jump_to_address_4`: Performs a long jump to a specific address.
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
- `call_subroutine`: Calls another routine and then returns.
- `fall_through_entry_5`: Entry point part-way through straight-line code: runs one instruction (ori.l) and then falls through into the code that follows, with no return of its own.
- `process_task_message_2`: Performs bitwise operations on d0 and branches to another routine.
- `process_task_message`: Retrieves a message from a task, stores a pointer from the message into a global variable, and updates a task's function pointer with the address of this routine.
- `update_global_params`: Stores a value into a global variable, clears another global variable, and calls two other routines using pointers from a second parameter.
- `manage_device_resources`: Performs several library calls to manage ports and devices, and modifies a byte in a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `negate_register`: Returns the two's complement of the value in d0.
- `compute_fixed_point_product_2`: Takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value by multiplying high and low 16-bit parts and combining cross terms.
- `normalize_signed_division`: Handles the sign logic for a division routine by negating d0 or d1 as needed before calling another routine.
- `normalize_fixed_point_divide_2`: Divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method and returns a 64-bit result with the quotient in the high 32 bits and the remainder in the low 32 bits.
- `prepare_data_offset_2`: Sets up a stack frame, saves registers, loads a pointer from offset 484 of a4 into a0, loads a byte from offset -1 of that pointer into d7, compares d7 to 79, and branches to another routine if d7 is less than or equal to 79, otherwise it sets d7 to 79, sets d0 to 79, and branches to another routine.
- `copy_and_call_task_function`: Copies bytes from a source pointer to a destination pointer using a loop, clears a byte at an offset of minus 81 from a pointer in a5, and calls a routine using a pointer from a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_task_and_call_function`: Calls three routines using pointers from a6, updates a global variable with a pointer from a task, and calls another routine using a pointer calculated from a value in a global variable.
- `update_global_params_2`: Stores a value into a global variable, clears another global variable, and calls two other routines using pointers from a second parameter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_string_with_limit_2`: Calculates the length of two null-terminated strings, then copies the second string to the first destination, limiting the number of bytes copied to a specified maximum length and ensuring the result is null-terminated.
- `copy_string_to_buffer_2`: The routine finds the terminating zero byte of the destination buffer and copies bytes from the source pointer into that location until a zero byte is copied.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `free_linked_list_memories`: Iterates through a linked list, calls a routine to free the memory at each node, and stores two zeroed values into a structure at offsets 928 and 932.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_linked_list_node`: Calculates a 4-byte aligned offset, updates a linked list structure by modifying pointers and size values within a structure, traverses a linked list until a null pointer is reached, and calls another routine and a function pointer stored in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `allocate_memory_from_pool`: Searches a linked list of memory blocks to find a suitable allocation size, performing a split if the block is significantly larger than requested.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_custom_stack_pointer`: Adjusts the stack pointer by four bytes, writes a specific memory address to a global variable, and saves several registers to the stack before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `clear_memory_and_set_signal`: Clears a range of memory in a loop, sets a signal, and calls a routine using a pointer retrieved from memory.
- `close_library_and_cleanup`: Loads values from a structure, calls two library functions, and then performs several conditional calls to other routines and a library function to close a library.
- `restore_registers_and_return`: Restores several registers from the stack and returns.
- `clear_and_call`: Clears the byte at the address in a0, calls a routine at an offset from a6, and returns if d0 is zero.
- `load_address_and_call`: Loads an address into a1, clears d0, calls a routine at an offset from a6, and returns if d0 is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_and_test_pointer`: Loads a pointer into a register, tests its value, and then performs several operations on the d0 register.
- `conditional_branch_and_return`: Branches if a condition is met, otherwise returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_to_global_structure`: Compares the stack pointer to a global value, otherwise it calls two routines and then writes a value from a global variable into a structure at an offset of 184 relative to a pointer in a0.
- `open_library_and_check_memory`: Checks a global variable, calls a routine, opens a library, and calls a function via a pointer from the library structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_available_memory`: Calls a library function to check for available memory and performs arithmetic on the result.
- `find_task_and_call`: Compares a value at a global address to the stack pointer, saves registers, and calls a library function using a pointer from the stack.
- `initialize_hardware_state`: Copies a longword from a pointer in a register to a global variable, sets that global variable's offset 184 to negative one, calls another routine, and then performs a series of byte-wise zeroing operations on register d0 before calling another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_string_data`: Calculates the length of a string, then copies characters from one memory location to another until a null terminator is reached.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_record_data_3_2`: Increments a 16-bit value at offset 7240 of a4 and decrements a 16-bit value at offset -4846 of a4; if the latter is not below zero, it copies a byte from a pointer at offset -4852 of a4 to a new location, increments that pointer, and calls another routine.
- `clear_and_call_format`: Clears a 16-bit word at offset 7240 of a4, pushes two pointers from the stack onto the stack, calls another routine, restores the two pointers from the stack to a2 and a3, and returns the 16-bit value from offset 7240 of a4 in d0.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `execute_nested_routines`: Saves registers, calls another routine, modifies the stack, and then calls a second routine based on a bit test of a value in a register.
- `copy_string_to_buffer_3`: Calculates the length of a string, then iterates through the string to copy characters into a buffer at a global location while decrementing a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `iterate_and_call_function`: Loops through a null-terminated string and calls a routine for each character in the string.
- `initialize_system_state`: Checks a word at offset 7542 of a structure, performs a series of data copies and memory operations including a call to another routine, and updates several status words at offsets 7244, 7250, and 7256.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `update_structure_fields`: Calls a routine with a value from a global variable, stores the result in a structure at offsets 4 and 12, and clears words at offsets 8 and 10.
- `clear_flags_and_counters`: Ands the byte-sized mask 243 into the word at offset 18 of the structure in a3, then clears the words at offsets 10 and 8 of that structure to zero, restores a3 from the stack and returns.
- `update_structure_fields_2`: A routine consisting of 180 instructions that calls another routine and modifies a structure at offsets 4, 8, and 12.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `iterate_and_call_routine`: Decrements a value from a global variable in a loop, checks a bit in a table, and calls another routine if the bit is not set.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_counter`: Calculates a value based on a register, adds it to a global variable, and stores a pointer and a masked value into a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_or_operations`: Saves registers to the stack, loads a pointer from a structure, and performs five bitwise OR operations on register d0.
- `branch_to_offset`: Performs a short branch to an offset of 8.
- `clear_structure_pointers`: Decrements a pointer in a0 and stores the resulting address into two global variables at offsets 1e10 and 1e0c.
- `allocate_and_link_structure`: Allocates memory, populates a new structure with a pointer from a global variable and a zeroed value, and links this structure into two global pointers.
- `manage_memory_list`: Checks a value against a field in a structure to determine if it can be updated or if it must call other routines to allocate and link new memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_and_call_list`: Traverses a linked list starting from a global variable and calls another routine when the list is exhausted.
- `execute_conditional_library_call`: Checks a global variable, calls another routine if it is non-zero, clears a word at offset -32736, and calls a library function with three registers as arguments.
- `return_modified_long`: Compares a long value in d5 with negative one, and if they are not equal, returns the value from d5; otherwise it returns the value from d5 with bit 0 set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_function`: Saves registers to the stack, loads a value from an offset of the stack pointer into d7, checks a long value at a global address, and if not zero, calls a routine at an offset from a pointer in a4, then calls another routine at an offset from a6.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `lookup_table_index`: Saves d7 to the stack, loads a word from the stack into d7, and if d7 is between zero and a value at a global offset, calculates an index by multiplying d7 by 6, checks a word at that index in a table, and returns the resulting address; otherwise it returns the original value of d7.
- `set_exec_signal`: Saves registers to the stack, loads the value 49152 into d1, and calls a routine at an offset from a pointer at a fixed address.
- `check_bit_and_clear`: Performs a bitwise AND on d7 with 12288, and if the result is not zero, performs several bitwise OR operations on d0 before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_d0_on_condition`: Saves a6 to the stack, checks if a long value at a global address is non-zero, and if so, performs several bitwise OR operations on d0 before restoring a6 and returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_d0_on_test`: Saves a6 to the stack, tests a word at a global address, and if not zero, performs several bitwise OR operations on d0 before restoring a6 and returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_multiple_subroutines`: Calls four different subroutines sequentially and then performs a branch to a fixed address.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_library_conditionally`: Saves a6 to the stack, loads a pointer from a fixed address into a6, checks four different global long values, and if any are non-zero, performs several bitwise OR operations on d0 before calling a routine at an offset from a6 with a pointer from a table.
- `call_library_function_2`: Moves d0 into a6 and calls a routine at a specific offset from a6.
- `call_close_library`: Loads a pointer from a fixed address into a6, calls a routine at an offset from a6, and then performs a bitwise OR operation on d0.
- `configure_hardware_registers`: Writes values to the DMA and interrupt enable registers and calls a routine using a pointer from a global variable.
- `store_d0_to_globals`: Stores the contents of register d0 into five different global longword locations and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_clear_d0`: Checks a word in a global variable and, if non-zero, performs nine operations that set register d0 to zero before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_global_pointer`: Loads a pointer from a global variable into register a3, and if the pointer is not null, performs several operations before clearing the global variable and returning.
- `clear_multiple_globals`: Checks three different global locations and, if they are non-zero, performs several operations to set register d0 to zero, finally clearing a longword at a specific global address.

### Red Baron (1990)(Dynamix)[cr Nemesis](Disk 1 of 3).adf:baron#unpacked

- `initialize_io_and_load_file`: Initializes system libraries, waits for a message port, and parses a command line string to open a file.
- `cleanup_resources`: Performs a series of conditional calls to routines to close a file, unlock a file, free memory, reply to a message, and close a library.
- `initialize_graphics_and_intuition_2`: Opens the graphics and intuition libraries, then attempts to allocate memory and initialize a series of buffers and structures.
- `parse_input_stream`: Reads bytes from a data stream by either fetching from a table or calling another routine, then loops until it encounters a newline or an escape character.
- `process_linked_list_nodes_2`: Calls a routine via a pointer from a global variable, stores the result in a global variable, and then iterates through a linked list to copy pointers into a global array.
- `update_list_via_table`: Iterates through a linked list, copying a longword from a table into a field at offset 4 of the current list element, calls a routine via a pointer from a global table, and updates a global pointer.
- `jump_to_address`: Performs a jump to an address stored in a memory location.
- `jump_to_address_2`: Performs a jump to an address stored in a memory location.
- `call_indirect_function_2`: Loads a function pointer from a stack-based structure and calls it using arguments passed via registers a0 through d3.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `resolve_entry_pointer`: Calls a routine to resolve a base address, checks bit 3 of a byte at offset 1 of the resulting pointer, and then either calls another routine with a pointer from offset 2 of that pointer or returns -1.
- `unload_segment_and_free_memory`: Waits for a message port, retrieves a message, and if certain conditions are met, calls a routine to dispose of a message port structure, unloads a segment, and frees memory.
- `copy_string_to_buffer_4`: Calculates the length of a null-terminated string, then iterates through the string to copy bytes into a destination buffer at offset 52 of a global structure, calling another routine if a counter at offset 58 reaches zero.
- `parse_command_line_and_initialize_io`: Parses a string into a table of pointers, handling quoted strings and escape sequences like newlines or carriage returns.
- `allocate_memory_block`: Checks a bit in a structure and, if conditions are met, calls another routine to obtain a pointer, then updates the structure's offset 4 and offset 12 with the result, or performs bitwise operations and updates offsets 8, 10, and 16 if the result is zero.
- `manage_record_data_2`: Processes a record structure by validating flags, adjusting a count at offset 16, and managing a linked list of data buffers.
- `fall_through_entry_6`: Entry point part-way through straight-line code: runs one instruction (movep.w) and then falls through into the code that follows, with no return of its own.
- `decode_variable_length_byte_2`: Parses a variable-length encoded stream by reading bytes from a buffer pointer and updating a structure's offset and flags.
- `iterate_and_call_handler`: Iterates through a list based on a value from a structure, checks a bit in a table entry, and calls another routine if the bit is not set.
- `copy_string_with_limit_3`: Calculates the length of a source string and a destination string, then copies bytes from the source to the destination up to a specified maximum length, ensuring the destination string is null-terminated.
- `append_string_to_buffer`: Locates the null terminator of a destination buffer and copies bytes from a source pointer to that location until a null byte is encountered, returning the original destination pointer.
- `insert_into_ordered_list_2`: Aligns a requested size to a 4-byte boundary and inserts a new memory block into a linked list of free memory nodes.
- `fetch_table_entry_address`: Calls a routine to check a signal, then if successful, calls another routine to retrieve an address from a table using a pointer at offset 2 of the retrieved address, and returns the result or negative one if a signal check or a global variable check fails.
- `free_memory_list`: Iterates through a linked list of memory blocks and calls a library routine to free each one, then clears a global pointer and sets a global variable to zero.
- `insert_memory_node`: Retrieves a pointer from a global variable, calls another routine to find a block, and then either updates a global pointer with the result or calls another routine to allocate a new block and updates the list.
- `resolve_table_entry_pointer`: Calls a routine to find a base address, then calls another routine using a pointer from that address to retrieve a specific entry.
- `allocate_and_link_node_2`: Allocates a memory block of a specified size plus twelve bytes, then initializes it and inserts it into a linked list.
- `manage_free_memory_list`: Aligns a requested size to a 4-byte boundary and searches a linked list of free memory nodes for a suitable block.
- `update_pointer_list`: Checks if a register is non-zero, calls another routine, and then updates a global variable by subtracting two from the register's value.
- `traverse_and_process_string_list`: Checks if a pointer at offset 12 of a structure is null; if not, it pushes the address of the next element and the values at offsets 4 and 8 of each element onto the stack and calls another routine.
- `create_message_port_from_path`: Locks a file path, calculates the total length of a list of strings, allocates a memory block and an entry, and then populates the memory block with the string data separated by spaces and a terminator.
- `duplicate_locked_file_path`: Locks a file path provided in a register, parses the string to identify a directory component, and then attempts to duplicate the lock on a directory structure by traversing task-related memory.
- `process_linked_records_2`: Checks bits 1 and 2 of a byte at offset 25 of a structure pointed to by a value derived from a register, calculates a size by subtracting a value at offset 12 from a value at offset 4, and if the size is non-zero, calls another routine with the size and two other values from the structure before calling a third routine with a value from the stack.
- `write_to_file`: Calls another routine to check a signal, then calls a routine to write data, and if an error occurs, calls another routine to retrieve the error and updates a global variable.
- `seek_in_file`: Calls another routine to check a signal, then calls a routine to seek within a file, and based on the result, performs additional seeks or updates a global variable with an error code.
- `read_from_file`: Calls another routine to check a signal, then calls a routine to read data, and if an error occurs, calls another routine to retrieve the error and updates a global variable.
- `dispose_msg_port_structure`: Checks if a field in the provided structure is non-zero to call a removal routine, then writes specific reset values to offsets 8 and 20 of the structure.
- `call_conditional_handler_2`: Checks if a longword at offset 524 of a structure pointed to by a4 is non-zero, then calls a routine via a pointer at offset 1016 of that same structure.
- `multiply_fixed_point_values`: Multiplies two 32-bit fixed-point values provided in registers and returns the 32-bit product in the first register.
- `calculate_absolute_division`: Calculates the absolute value of a division between two 32-bit integers by normalizing the divisor and calling another routine.
- `divide_32bit_fixed_point`: Performs a 32-bit division of a dividend in d0 by a divisor in d1 using a normalization process to align the divisor.
- `prepare_log_data`: Loads a value from a table at an offset from a pointer, compares it to a constant, and branches based on the result.
- `write_log_and_open_library`: Copies a block of bytes, then attempts to find a task to write a message to its file descriptor, otherwise it attempts to open a library and store the result in a global variable.
- `open_library_and_store_result_2`: Calls a routine via a pointer from a global variable, stores the result in a global variable at an offset from a pointer in a6, and then branches to another routine.
- `call_function_with_arguments`: Stores a pointer in a global location, pushes several arguments onto the stack, and calls another routine, returning zero if the call succeeds or negative one otherwise.
- `function_epilogue_4`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `create_message_node_2`: Allocates a memory block and a signal, then initializes it as a message port using parameters from a table.
- `get_table_entry_address`: Checks if a value is within a specific range and if a corresponding entry in a table is non-zero, then calculates and returns an address based on that value multiplied by six.
- `check_signal_and_handler`: Calls a routine to set a signal, checks if a specific bit is set in the return value, and if so, calls a routine stored at an offset in a global structure.
- `call_handler_wrapper`: Calls another routine and then returns.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_input_device`: Checks if a global word is zero, then finds a task, adds a port, opens a device, and performs I/O operations using library calls.
- `close_device_and_remove_port`: If a global word is non-zero, it clears that word and calls three different routines via a library base to perform I/O operations and remove a port.
