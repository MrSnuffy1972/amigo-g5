# Game 1784 (captive (1990)(mindscape)[cr cls][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01784). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 71 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 15 raw-word lines |
| Functions named and described | pass, 757 of 757 named (0 author symbols), 757 of 757 described (13 by rule, 743 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4 of 4 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 40000 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 17693 of 156104 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1000 |
| code | 3008 |
| data | 883892 |
| padding | 9188 |
| unused | 4096 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Captive (1990)(Mindscape)[cr CLS][a2].adf:fed_MapGen#unpacked | 1 | 7208 | 145 |
| Captive (1990)(Mindscape)[cr CLS][a2].adf:Off | 2 | 52 | 2 |
| Captive (1990)(Mindscape)[cr CLS][a2].adf:CLASSIC!! | 1 | 89 | 3 |
| Captive (1990)(Mindscape)[cr CLS][a2].adf:type | 2 | 117 | 2 |
| Captive (1990)(Mindscape)[cr CLS][a2].adf:Fed_Demo#unpacked | 1 | 394 | 30 |
| Captive (1990)(Mindscape)[cr CLS][a2].adf:federation#unpacked | 1 | 11668 | 472 |
| Captive (1990)(Mindscape)[cr CLS][a2].adf:fed_bootfile | 1 | 611 | 31 |
| Captive (1990)(Mindscape)[cr CLS][a2].adf:fed_MapView#unpacked | 1 | 648 | 57 |
| Captive (1990)(Mindscape)[cr CLS][a2].adf:case.pic#unpacked | 1 | 62 | 4 |
| Captive (1990)(Mindscape)[cr CLS][a2].adf:mouse | 2 | 3 | 1 |
| Captive (1990)(Mindscape)[cr CLS][a2].adf:A_CLASSIC_CRACK!!! | 1 | 110 | 9 |
| Captive (1990)(Mindscape)[cr CLS][a2].adf:bootblock | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |
| Captive (1990)(Mindscape)[cr CLS][a2].adf:FedAnni-3.pi#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:fed_MapGen: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (11768 -> 29340 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:fed4.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (11582 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:fed2-b.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (26109 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:FedAnni-1.pi: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (22521 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:FedAnni-2.pi: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (23355 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:fed10.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (8247 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:fed7-E.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (20812 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:fed6-F.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (14549 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:Fed_Demo: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (107960 -> 192084 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:federation: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (62737 -> 95576 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:FedAnni-3.pi: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (18569 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:Fed7-C.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (22414 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:fed6-D.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (28223 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:Fed9.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (8296 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:Fed7-A.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (22998 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:Fed6-B.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (28062 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:Fed5.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (23048 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:Fed3-a.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (24669 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:fed_MapView: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (6742 -> 10236 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:Fed2-A.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (26681 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:Fed_Sounds: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (49186 -> 52204 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:case.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (49942 -> 360580 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:fed7-d.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (19028 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:fed6-E.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (26175 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:Fed7-B.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (25991 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:fed6-C.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (29603 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:Fed8.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (28269 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:Fed6-A.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (28092 -> 40000 bytes)
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:Fed3-B.pic: RNC1: Rob Northen RNC1 Compressor (old)-packed, unpacked with ancient (27914 -> 40000 bytes)
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 8 64 KB pages each with >= 29491 distinct PCs executed, 131065 of its 135694 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 8 64 KB pages each with >= 29491 distinct PCs executed, 131065 of its 135694 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 8 64 KB pages each with >= 29491 distinct PCs executed, 131065 of its 135694 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 8 64 KB pages each with >= 29491 distinct PCs executed, 131065 of its 135694 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x25k slid through memory (an address: 8 64 KB pages each with >= 29491 distinct PCs executed, 131065 of its 135694 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op ABAD) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- captive (1990)(mindscape)[cr cls][a2]/Captive (1990)(Mindscape)[cr CLS][a2].adf:FedAnni-3.pi#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 40000 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Captive (1990)(Mindscape)[cr CLS][a2].adf:A_CLASSIC_CRACK!!!

## Copy protection found

- `mask_ciab_prb_register` (federation#unpacked): Reads a global variable, masks it with 231 and ORs 152, then writes the result back to the same variable. Kind: register masking/bit manipulation. Pass path: the instruction immediately following the call (the routine modifies a hardware register/status bit to mask or validate a specific state).

## Functions


### Captive (1990)(Mindscape)[cr CLS][a2].adf:fed_MapGen#unpacked

- `initialize_map_generation_buffers`: Sets up a series of memory pointers by calculating offsets from a base address using values stored in a table.
- `fill_map_with_pattern`: Reads a byte count from a structure, clamps it to a maximum of six, and loads a word from a global table.
- `generate_map_entries`: The routine loops over a count stored in a structure, uses a scaling routine to find matching nibble values in two tables, stores derived differences into the structure, then reads a scaled value and writes constants and derived bytes into the structure.
- `initialize_map_grid`: The routine clears and then conditionally fills a grid of words based on comparisons of bytes from an input structure, using nested loops over rows and columns, and updates the base pointer each outer iteration.
- `fetch_byte_from_table_2`: Adds a word from d5 to d2, loads a byte from a table at the address in a0 plus the new d2 value into d3, and returns.
- `compute_scaled_value_2`: The routine calls another routine, takes its word result, shifts it left by sixteen bits minus the low six bits of the result and ORs it with the result shifted right by the low six bits, then returns the combined word.
- `call_and_return`: The routine calls another routine and then returns.
- `transform_data_grid`: The routine iterates through a two-dimensional grid, calling another routine to determine a value, and then modifies bytes from a source table based on that value before storing them in a destination table.
- `expand_map_tile_data`: Iterates through a nested loop to process a two-dimensional grid of data.
- `process_status_bytes`: Performs nested loops to read bytes from a source, conditionally modifies them via bitwise shifts, and writes them to a destination buffer.
- `update_status_byte`: The routine clears a byte in a structure and conditionally sets its high bit if a specific input byte has its high bit set.
- `decrement_and_return`: Decrements a register using a loop instruction and returns.
- `compute_scaled_value`: The routine reads a word from a structure, scales it, stores the updated word back, and returns a combined value derived from the intermediate result with a constant xor.
- `configure_map_generation_parameters`: Clamps a value from a structure and calculates a derived value to store in a global location.
- `propagate_map_cells`: The routine iterates over a two-dimensional area using two counters, checks neighboring cells for zero values, counts qualifying neighbors, and conditionally writes constants to the current cells.
- `loop_until_nonzero`: The routine repeatedly calls another routine twice inside nested loops, exiting the inner loop when a short read through a pointer is negative and the outer loop when that short is zero.
- `write_two_word_flags`: The routine conditionally calls another routine based on two global bytes and a global word, then stores constant words to three locations derived from input pointers and a register, and stores two words from registers to global offsets.
- `update_map_generation_parameters`: Updates map generation state by accumulating values from a structure and performing conditional loops based on bitmask checks.
- `set_dual_values`: The routine stores the value zeroxffff at one memory location and the value zerox80 at another memory location.
- `process_map_generation_loop`: The routine calls two other routines, then loops eight times calling another routine and conditionally writes constant words to two memory locations derived from pointers and a register, finally returning the value from the first call.
- `empty_routine_2`: The routine contains eight instructions but performs no observable operation on registers, memory, or hardware; it is effectively empty.
- `set_register_constants`: The routine performs bitwise OR operations to set specific bits in registers d2, d6, and d4.
- `fill_map_with_typed_values`: The routine reads a scaled limit from a global variable, loops that many times, and for each iteration scans a set of entries using another routine and a word from a structure.
- `process_map_data`: The routine scales a value from a table, then enters nested loops to perform bitwise checks on a word and conditionally increment a global variable or write a byte to a memory location.
- `process_nested_loop_data`: Reads a word from a global structure and enters a loop that conditionally calls several routines and modifies a global variable and a byte in a table based on a character comparison.
- `loop_process_map_entries`: Loops over a table pointed to by a global pointer, invoking other routines for each entry and updating local variables.
- `conditional_return_value`: The routine calls another routine and returns its result unless a word flag is set, in which case it returns a constant; if the flag is clear and the flag matches specific values it calls another routine before returning.
- `empty_routine_3`: The routine consists of eight instructions and does not perform any observable operation on data or registers; it ends without side effects.
- `modify_word_based_on_mask`: Tests a word from a global variable and branches based on the result.
- `branch_on_masked_value`: The routine masks a word with a constant and branches to different locations based on the result, or increments a global word.
- `store_byte_and_word`: The routine writes a constant word to a memory location pointed to by a pointer and then stores a byte from a data register into the same location, overwriting the low byte.
- `handle_recursive_map_data`: Performs conditional recursive calls, writes pixel data, and updates memory based on a value in a global structure.
- `call_subroutine`: The routine calls a subroutine using a relative branch.
- `mask_and_rotate_values`: Copies d0 to d1 and d2, masks d1 with 63, rotates d2 right by 6 bits, masks d2 with 31, and returns.
- `initialize_map_generation_state`: Clears two words in a global structure, writes two constant words to a global structure, derives a short flag from a global byte and stores it, then calls another routine.
- `update_memory_value`: The routine performs a multiplication and addition on a value from a memory location, stores the result back into that location, and returns a calculated value.
- `initialize_and_copy_memory`: Fills a block of memory with a constant value and then copies a sequence of words from one memory location to another.
- `iterate_map_cells_with_checks`: The routine reads a word from a global variable, derives a loop count, and repeatedly performs a series of checks on memory locations and a global word while calling other routines, updating an index each iteration.
- `initialize_table_and_process_entries`: Clears a global word, initializes a global word and a table of words to a constant, then iterates over entries writing words from inputs and a global variable and calling another routine, returning early if a condition is met.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `test_map_cell_value_2`: The routine checks two input words against limits, calls another routine, then returns a constant or a masked value based on several word reads from global and pointer-derived locations, and may write a constant word to a pointer-derived location.
- `validate_and_mark_map_cell`: Checks a map cell value against several limits and conditions via a table pointer and a global variable.
- `compare_and_call_routines`: The routine compares a register with a global word and conditionally calls two other routines.
- `return_immediately_3`: The routine compares a register with a global word, conditionally calls two other routines, and returns without modifying memory or registers.
- `check_bounds_and_test`: The routine checks three inputs against limits and a global word, conditionally calls another routine and another routine, returning the result of the second call or zero.
- `store_registers_and_branch`: Copies a word from a global variable into a register and stores three registers into memory at fixed offsets from a pointer.
- `update_map_metadata_and_structure`: Updates various global state variables and structure members based on bitfield extractions and conditional checks.
- `store_word_to_global`: The routine stores a word from a register into a global variable at a fixed offset and then returns.
- `compare_stack_byte`: Calculates an offset using d1 and d2, loads a byte from the stack at that offset into d3, compares it with a byte from the stack, and returns.
- `calculate_map_dimensions_and_iterate`: Extracts width and height parameters from a bitfield to initialize a grid structure, then executes nested loops to process map data.
- `branch_to_address_3`: Adds 4 to a word from the stack in d5 and branches to a new location.
- `move_word_to_register`: The routine moves a word from a memory location to a data register.
- `process_map_data_2`: Performs complex operations including scanning tables, scaling values, and writing words into a table using computed indices and nested loops.
- `scan_and_mark_table_entries`: The routine loops a fixed number of times, scanning entries via another routine and checking specific conditions.
- `nested_loop_mask_words`: Contains nested loops that call another routine and conditionally OR a mask into a word at an address derived from a pointer and an offset.
- `tiny_moveq_clr`: Small routine of 2 instructions that loads 254 into d5, then clears d6.
- `replace_matching_short`: The routine loops a fixed number of times over a short array pointed to by a pointer, comparing each element to a value in a register and replacing it with another register value when they match, then advances the pointer each iteration.
- `replace_matching_short_2`: The routine loops a fixed number of times over a short array pointed to by a pointer, comparing each element to a value in a register and replacing it if it matches.
- `modify_and_return_register_values`: Shifts the word in register d5 left by six bits, adds the word in register d1, adds the word in register d2, doubles the resulting value in d5, and returns.
- `load_stack_words`: Loads three consecutive words from the stack into d1, d2, and d3 and returns.
- `scan_table_and_mark_entries_2`: The routine stores a constant byte into a global flag, loops a fixed number of times calling another routine with an offset base address and a decreasing counter, then returns the input word if the flag is negative or otherwise returns the result of another routine call.
- `scan_table_and_mark_entries`: The routine stores a pointer into a global variable, scans a short table for a condition involving another routine and a register, writes constants to memory via a pointer, marks entries as -1, and repeats the marking over a fixed number of entries.
- `load_bytes_to_registers`: Clears three registers and loads two bytes from an offset of a pointer into two registers.
- `test_map_cell`: The routine clears a word, calls another routine, and returns if the result in a register is zero.
- `recursively_call_on_failure`: Calls another routine and checks a boolean flag; if the flag is false, it calls that same routine four more times and returns a value of all ones.
- `loop_test_map_cell`: The routine loops a fixed number of times, calling another routine on each iteration and incrementing a counter.
- `test_map_cell_value`: The routine computes an index from two inputs and a base pointer, reads a word from a table, and returns a flag based on whether the value is zero or below a threshold, otherwise it returns a default flag.
- `compute_scaled_value_wrapper`: The routine calls another routine and combines the returned word with input registers using shifts and masks to form a 32-bit result.
- `store_word_into_map`: The routine computes an index from two inputs and a base pointer and writes a word from a register into the table at that location.
- `process_byte_flags_and_update_table`: The routine loops over data, reads a byte from a global buffer, conditionally calls other routines based on the byte value, writes flag bytes to a global buffer and updates a byte in a global buffer.
- `fill_table_with_flags`: The routine loops over a short array and, based on conditions involving a global variable and a byte value, writes flag bytes and word values into a table pointed to by a pointer.
- `call_three_routines_sequentially`: The routine calls three other routines in sequence and then returns.
- `recursive_self_calling_routine`: The routine calls another routine three times in succession and then returns.
- `call_four_routines_sequentially`: The routine calls four other routines in sequence and then returns.
- `conditional_early_return`: The routine checks two input words against limits and, if both are within range, calls another routine and conditionally returns early based on a word read from memory.
- `load_byte_from_stack`: Calculates an offset using d1 and d2, clears d3, loads a byte from the stack at that offset into d3, and returns.
- `subtract_stack_values`: Calculates an offset using d3 and d5, subtracts a word from the stack at that offset from d1, subtracts another word from the stack at that offset plus 8 from d2, and returns.
- `add_stack_values`: Calculates an offset using d3 and d5, adds a word from the stack at that offset to d1, adds another word from the stack at that offset plus 8 to d2, and returns.
- `swap_table_entries`: The routine saves a word from a global variable, loops a fixed number of times swapping pairs of words in a table using an index derived from another routine, then restores the saved word.
- `initialize_table_and_scan_bytes`: The routine clears a block of words in a global variable and then scans a byte array using nested loops, calling another routine to obtain values and conditionally writing bytes to a table and updating counters.
- `scan_table_and_copy_byte`: The routine scans a table of words for entries matching bit masks, and either calls other routines and writes a constant byte to a buffer or reads a byte and calls another routine with a combined value.
- `guarded_byte_copy_loop`: The routine calls two other routines and then iterates over a small range, reading a byte from a buffer and conditionally copying a value from another routine into the buffer.
- `guarded_stub_call`: The routine checks two input words against limits and calls an empty stub routine only when both limits are satisfied.
- `iterate_cells_and_clear_flags`: The routine computes a scaled value from a global word, stores it in a global variable, then loops over a table of shorts and conditionally calls other routines and clears words at two addresses derived from pointers and an offset.
- `fetch_and_modify_byte`: The routine calls another routine, reads a byte from a table using an offset, and modifies a register based on the value compared against constants.
- `return_from_subroutine`: Adds two words from the stack, indexed by d0, to d1 and d2, and returns.
- `read_byte_scaled`: The routine calls an empty stub routine and returns a value derived from a byte read from a table offset by an index, scaled by multiplication and xor.
- `double_stub_return`: The routine calls two other routines and then returns a constant.
- `call_two_stubs_and_return_max`: Calls two empty routines and returns a value consisting of all bits set.
- `conditionally_loop_with_global_check`: The routine checks a global word for non-zero, then enters a loop that calls another routine and tests a global word against a constant, returning early if the word exceeds the constant or a counter expires.
- `loop_check_global_flag`: The routine initializes a counter and a flag and enters a loop that calls another routine, checks a global word for a specific value, decrements the counter, and returns when the counter expires or the condition is met.
- `generate_recursive_map_structure`: Performs a deeply nested recursive traversal that conditionally writes data to a buffer and updates global state.
- `generate_map_recursive_structure`: A recursive routine that traverses a data structure by checking byte values against a set of constants and conditionally calling itself.
- `conditional_recursive_calls`: The routine checks an input word against several constants and conditionally calls other routines in a nested manner before returning.
- `check_map_cell_and_recurse`: The routine checks two input values against fixed limits and, if both are within range, calls another routine and then tests a short value read through a pointer formed from a base pointer and an offset.
- `loop_clear_table_entries`: The routine obtains a word value from another routine and repeatedly tests a word in a table addressed via a base register and an offset derived from the value.
- `loop_test_table_entries`: The routine obtains a word value from another routine and loops a fixed number of times.
- `clear_global_and_loop_calls`: The routine clears a global word and then loops a fixed number of times calling another routine each iteration, with an additional call when a counter reaches zero.
- `search_table_entry`: Obtains a word from a table based on a clamped value and repeatedly tests a word in another table until a condition is met.
- `generate_map_cell`: Stores an input word into a global variable and reads a byte from a table using that word as an index.
- `generate_map_tile_data_2`: Iterates through a circular index to find a valid tile type based on bitmask checks and global table constraints.
- `rotate_and_mask_table_value`: The routine reads a value from a global variable via an index derived from a register, conditionally writes a byte to a location, rotates a word from a register until a mask matches, stores the result back, and returns an input value.
- `map_generation_state_machine`: The routine saves input values into a global structure, repeatedly calls another routine to drive a small state machine, and conditionally updates fields in the same global structure based on flags read from a buffer and on results returned by the called routine.
- `call_four_routines_sequentially_2`: The routine calls another routine four times in a row and then returns.
- `call_test_map_cell`: The routine calls another routine once and then returns.
- `compute_masked_lookup_index`: The routine reads two words from a register, clamps one, multiplies them, forms a masked index, and returns a value obtained by combining bits from a table lookup with the masked index.
- `swap_linked_list_elements`: Resets several values in a structure and swaps two adjacent elements in a linked list if a condition is met.
- `populate_map_data_table`: Iterates through a loop based on a counter to write bytes and words into a memory table.
- `process_map_tiles`: Calculates a loop count based on global values and iterates through a buffer, setting a high bit on specific characters and clearing certain global word values.
- `scan_map_buffer`: Iterates through a buffer for a fixed number of steps, checking if characters match specific constants and updating global word values and a byte flag.
- `update_register_and_memory`: The routine stores a word from a register into a memory location and then loads a new constant value into that register.
- `generate_map_tile_data`: Iterates through a circular index to find a valid tile type based on bitmask checks and global table constraints.
- `traverse_linked_list`: The routine follows a chain of short offsets starting from a base pointer plus four, repeatedly adding the offset found at plus two until the offset becomes negative or zero.
- `check_allowed_value`: The routine examines a short input value and returns early unless the value is one of 1, 2, 4, 5 or 9.
- `test_multiple_bits`: Tests specific bits within two different registers.
- `determine_value_from_table`: Checks a value at a memory offset and either retrieves a word from a table or calculates a new value based on a mathematical formula before storing it in a global variable.
- `chain_test_map_cells`: Initializes a boolean flag and repeatedly calls another routine while checking several output registers against constant values to return a combined result.
- `test_map_cell_value_3`: Calls another routine and returns its result unless a zero result combined with a specific byte value in memory causes it to return one.
- `generate_map_data`: Loops through a series of iterations to test map cell values and write pixel data to memory based on the results.
- `write_pixel_data`: Writes a byte and a constant to one memory location, then writes a four-byte sequence to a location calculated from a pointer and an offset, finally incrementing that offset.
- `reset_map_generation_state`: Sets a global flag to one and reads a global word.
- `initialize_map_generation`: Stores a word into a global variable, loads another word into a register, calls a subroutine, and then performs an unconditional branch.
- `normalize_and_hash`: The routine reduces an input word by repeatedly subtracting and shifting until it is small, then repeatedly multiplies and adds a constant to a word from a global structure in a loop controlled by a mask and a bound, without storing the result.
- `replace_characters_in_string`: Iterates through a string to replace specific characters with different values based on a counter and a limit.
- `loop_and_store_counter`: Executes a loop that updates a value in a structure based on another memory location and stores a final counter value into a global variable.
- `loop_with_constant_assignment`: If a specific memory location is non-zero, it enters a loop that repeatedly writes a constant value to a structure and calls another routine.
- `loop_with_limited_iterations`: If a specific memory location is non-zero, it runs a loop that writes a constant to a structure and terminates after a fixed number of successful calls to another routine.
- `save_registers_and_branch`: Saves multiple data and address registers onto the stack and performs a short branch to another routine.
- `update_linked_list_entry_2`: Checks a character value and conditionally writes two bytes to a memory location after calling another routine.
- `update_tile_values`: Calculates a loop limit from a global value and iterates through a buffer, replacing specific word values with a constant and incrementing a global counter.
- `modify_map_data`: Loops through a buffer based on a global limit, checking for specific character or word matches to update a byte in the buffer and increment a global counter.
- `replace_characters_with_offset`: Iterates through a string to replace specific characters with different values and increments a global counter.
- `replace_map_characters`: Iterates through a buffer based on a global value, checking for a specific character and replacing it with another while incrementing a global counter.
- `execute_infinite_recursion`: This routine performs an infinite recursive call to itself.
- `self_recursive_call`: The routine contains thirteen instructions and immediately calls another routine with the same entry point, creating a self-recursive call.
- `clear_word_at_offset`: Clears a word in a memory location using an offset from a register.
- `insert_into_linked_list`: Stores several bytes into a memory area and then searches a list to either insert a new entry or update an existing one while shifting subsequent data.
- `calculate_buffer_parameters`: Calculates a value based on a multiplication and a shift, clamps it between two bounds, and stores it in a global structure along with a bit-manipulated version of another value.
- `process_table_buffer`: Iterates through a table of words, calling another routine for each non-zero entry, and performs memory modifications and bitwise operations if a specific flag is set.
- `store_bytes_and_update_limits`: Copies two bytes from a buffer into two global words, then calls another routine twice.
- `single_call_stub`: The routine calls one other routine and then returns.
- `swap_map_characters`: Iterates through a buffer to find specific character pairs and swaps their values while updating two global word counters.
- `update_linked_list_entry`: Modifies a byte in a memory location based on a condition, clears a word, and then attempts to insert a new entry into a linked list.
- `insert_list_item`: Clears a byte in a memory location and then attempts to insert a new entry into a linked list.
- `check_and_call_subroutine`: Tests a global word and, if it is zero, calls another routine before moving a constant into a register.

### Captive (1990)(Mindscape)[cr CLS][a2].adf:Off

- `open_libraries_and_run_task`: Program entry of fifty instructions that opens libraries by name, adds an interrupt server, waits until two screen dimension fields reach fixed values, and then removes its task through Exec routines.
- `move_and_or_register`: Moves a byte from a pointer into a memory location and performs a bitwise OR operation on a register.

### Captive (1990)(Mindscape)[cr CLS][a2].adf:CLASSIC!!

- `decompress_bitstream_data`: Expands a compressed bitstream into a destination buffer by reading bits and shifting them into words.
- `manipulate_word_and_return`: Combines a byte and a word into a new value and returns a bitwise modified version of a loaded word.
- `perform_bit_shift_loop`: Repeatedly shifts a value right until it reaches zero while decrementing a counter.

### Captive (1990)(Mindscape)[cr CLS][a2].adf:type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of one register into another and performs a relative branch.

### Captive (1990)(Mindscape)[cr CLS][a2].adf:Fed_Demo#unpacked

- `initialize_demo_sequence`: Performs a series of table copies, populates overlapping global areas with specific patterns, and configures interrupt registers.
- `wait_for_demo_flag`: The routine calls another routine and then spins until a short value in a global area reaches a threshold, then calls an indirect routine via a stack value and stores a constant into a global variable.
- `copy_word_table_to_global`: The routine copies thirty-two words from a source pointer to consecutive locations in a global variable area, advancing both pointers each iteration.
- `copy_long_table_from_global`: The routine copies ten long words from a global variable area to a destination pointer, advancing both pointers each iteration.
- `initialize_demo_tables`: The routine writes a repeated pattern of short zeros, long ones and long constants into three overlapping global areas for 145 iterations, updating a running sum and a decrementing counter each loop.
- `scan_and_clear_demo_buffers`: The routine scans paired short and word entries in global areas for a terminating condition, then clears a range of shorts and masks the high word of corresponding uints, finally writing a constant to a global variable.
- `write_interrupt_registers`: Stores the address of the current routine into a global variable, then writes two constants to a global variable that corresponds to the INTENA and INTREQ hardware registers.
- `check_interrupt_request`: Reads a value from a hardware register and tests its fifth bit.
- `set_interrupt_request_and_return`: The routine calls another routine, writes a constant to a global variable, and returns a value formed by concatenating a stack value with an input register.
- `load_addresses_to_registers`: Toggles a bit in a word at a PC-relative address and loads two addresses from the stack into registers before branching.
- `configure_coprocessor_registers`: The routine loads two addresses into registers, writes them to coprocessor registers, clears a hardware register, and performs conditional branches based on a value from a relative offset and a global variable.
- `call_multiple_subroutines`: An address-taken routine that calls three different subroutines in sequence.
- `expand_bitplanes_and_copy_data`: This routine performs bitplane expansion by reading from a source buffer and writing to a destination buffer, followed by several data movement operations including copying words from a global area and block-copying memory segments.
- `modify_registers_and_branch`: Sets a register to a constant, subtracts a value from another register, calls another routine, and branches to another location.
- `decrement_and_branch_subroutine`: Clears a register, subtracts a constant from another register, calls a subroutine, and then performs a relative branch.
- `execute_sequential_subroutines`: Sets a global variable and calls five subroutines in sequence.
- `expand_bitplane_to_bytes`: The routine computes a base pointer from an input value and a global base, then for each of forty bytes it expands four bitplanes by reading bits from a source area, forming a word index, looking up a byte in a global table, and writing the reconstructed bits back to the destination area.
- `duplicate_global_words`: The routine copies sixteen words from a global area onto itself with an offset and then copies a block of words from one global area to another global area using a loop.
- `update_sorted_list`: Iterates through a list of words, updating values and modifying a global variable based on a comparison with a threshold.
- `move_and_loop`: The routine moves a word into a register and then executes a loop using a decrement and branch instruction.
- `iterate_and_decrement_table_word`: Loads a global value and three relative pointers into registers, then iterates through a table of words, decrementing each entry by one until a zero value is encountered.
- `process_table_values`: Performs arithmetic on values in a table, applying a decrementing loop that modifies entries based on a comparison.
- `update_score_table`: The routine updates an integer table by adding a value from a register, then repeatedly writes a register value into an array and decrements a counter in a short table, adjusting a running difference and applying a negation when the difference is negative.
- `populate_structured_table_2`: Compares elements in a list against a range and populates a table with structured data, or writes specific byte sequences to a string if the comparison fails.
- `populate_structured_table`: Compares elements in a list and populates a table with structured data including specific constant values and calculated offsets.
- `process_string_and_table`: The routine performs a string comparison loop and then populates a table with structured data using a loop.
- `load_address_pointers`: Loads two addresses from memory into registers and branches to another location.
- `load_address_pointers_swapped`: Loads two addresses from memory into registers in reverse order and branches to another location.
- `interpolate_color_values`: Performs nibble-based arithmetic scaling on pairs of words to generate interpolated values for a destination table.
- `interpolate_palette`: The routine processes pairs of words from input pointers, mixing nibbles and performing arithmetic scaling with two divisor words, writing the resulting word to an output pointer in a nested loop.

### Captive (1990)(Mindscape)[cr CLS][a2].adf:federation#unpacked

- `store_registers_and_branch_2`: Loads a pointer from a relative address, stores the contents of registers into memory at that location, and performs a short relative branch.
- `populate_data_table`: Copies values from a source into a global structure, performs a loop to write several multi-byte values into a table, and then performs a block copy of data from a pointer into the global structure.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `initialize_hardware_registers_2`: The routine writes a constant to a global variable and then calls three other routines to configure hardware and copy data.
- `increment_global_variable`: The routine increments a global variable by a fixed amount through a loop.
- `initialize_hardware_and_copy`: The routine writes several constants to global variables that alias custom hardware registers, copies a block of words from one global area to another using a small loop, and writes a constant to two global variables.
- `initialize_hardware_registers`: Writes constants to the BLTADAT, DMACON, and INTENA hardware registers and several global variables, clears audio channels, and sets the interrupt request register.
- `clear_audio_channels`: Clears a memory location and writes a fixed pattern to four consecutive audio hardware registers, then returns the input value unchanged.
- `set_global_constant`: The routine stores a constant value into a global variable.
- `configure_custom_hardware_registers`: The routine writes three constants to a global variable that aliases ADKCON, DMACON and INTENA, then writes another constant to a global variable that aliases INTREQ, and returns.
- `set_global_seven`: Stores the value seven into a global variable.
- `set_global_high_value`: Stores a specific word value into a global variable.
- `set_global_max`: Stores a value of all ones into a global variable.
- `write_interrupt_request`: Writes a word of all ones to the interrupt request register.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `check_interrupt_status`: The routine saves registers, reads a hardware register, and performs conditional branches based on specific bits.
- `call_stub_and_set_byte`: Calls another routine and then stores the value sixteen into a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_global_constant_2`: Sets a global variable to a constant value and returns.
- `clear_spr2pth_registers`: The routine clears five consecutive long words at the SPR2PTH hardware register block, increments three global variables, and then calls another routine.
- `test_joystick_bit`: Saves registers to the stack, reads a word from a hardware register, tests a specific bit, and branches based on the result.
- `check_joystick_status`: Tests a bit in the CIAA peripheral register to determine if a branch should be taken.
- `mask_byte_and_spin_on_variable`: The routine masks a bit in a byte at an offset from a pointer and calls another routine.
- `branch_on_word_range`: Compares a word in a register against two specific constants and performs conditional branches based on the comparison results.
- `clear_memory_block`: The routine initializes a counter and a pointer and repeatedly stores zero to consecutive 32-bit locations pointed to by the pointer, advancing the pointer each iteration, until the counter reaches a terminal value, then returns.
- `update_table_entry`: Checks a value against specific constants and updates a byte in a table using a value derived from a bitwise operation.
- `call_table_update`: The routine saves registers to the stack, calls another routine with a specific constant, and then restores the registers before branching.
- `update_table_entry_and_flag`: The routine conditionally updates several fields in a table pointed to by a pointer, using an index derived from the table and input values, and then stores a transformed byte to a location computed from another pointer and an input word, before returning.
- `fetch_table_byte`: Masks the lower seven bits of d0, uses the result as an offset into a table located via the program counter, and returns the byte found at that location in a0.
- `update_global_from_table`: Clears a global variable and, if a flag is non-zero, decrements it and stores a byte from a table into another global variable.
- `return_address`: The routine returns the address of another routine.
- `clear_global_value`: The routine stores zero into a global variable.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `clear_joytest_register`: Stores zero into the joystick test register.
- `mask_ciab_prb_register`: Reads a global variable, masks it with 231 and ORs 152, then writes the result back to the same variable.
- `update_word_bits`: Updates a word or two bytes at an offset from a base pointer by applying bitwise AND and OR operations using provided values.
- `load_offset_and_branch_2`: Loads a word into a register, retrieves a pointer from a base address plus an offset, adds a constant to that pointer, and performs a short branch.
- `prepare_blitter_jump_offset`: Loads a pointer from a stack offset, adds a register value to it, and branches to another location.
- `configure_blitter_parameters`: Sets various hardware registers for blitter operations and executes a loop to perform multiple transfers.
- `sync_bitmask_to_data`: Repeatedly calls another routine and, based on the returned value, updates a destination buffer by setting or clearing specific bits from a source buffer.
- `increment_pointer_offset`: Loads a value from a table into a register and then adds a constant offset to that register.
- `wait_and_set_custom_registers`: Sets several custom chip registers and a global variable, then loops waiting for a bit in a hardware register while updating two pointers.
- `execute_blitter_operation`: This routine invokes a procedure to populate hardware blitter registers with data from a structure and waits for the blitter to complete its task.
- `configure_blt_registers`: Sets several values in a structure used for hardware blitter operations and waits for a specific bit in a status register to clear.
- `shift_left_by_three`: Returns the input value shifted left by three bits.
- `apply_bitmask_to_buffer`: Initializes a small table of words based on a bitmask and then performs a nested loop to apply these values to a destination buffer using bitwise logic.
- `initialize_blt_parameters`: Writes several values to a memory area and waits for a bit in a hardware register to change.
- `initialize_data_structures`: Sets several memory locations to zero or specific values and enters a loop that waits for a bit in a memory location to change.
- `copy_and_mask_buffers`: Copies blocks of bytes from one buffer to another, masks them with a byte, and repeats the operation for multiple rows using two nested loops.
- `set_blt_parameters`: Sets several custom chip registers and a global variable, then performs multiple loops waiting for a specific bit in a hardware register to clear.
- `blit_and_mask_buffer`: Configures the hardware blitter to perform area fills or transfers and executes a software-based bitmasking routine to combine multiple data buffers.
- `mask_and_store_bytes`: Builds a mask from registers and memory, combines it with bytes read from a buffer, stores masked results back into the buffer, and returns a masked value.
- `update_hardware_colors`: Calculates values based on several global variables and stores the results into a pointer-based structure and hardware color registers.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `update_mouse_cursor_position`: Reads joystick or mouse input to increment or decrement coordinate values, clamps these values within a defined boundary, and calculates a smoothed position for a cursor or pointer.
- `clamp_arithmetic_result`: Performs arithmetic by adding a sign-extended byte to a word and then clamps the result to zero if it is negative or below a specific threshold.
- `update_table_values`: Calculates new values based on arithmetic operations and bit shifts, then stores them into a table and updates counters in a global structure.
- `update_and_branch`: Moves a word to a memory location, branches based on a condition, clears a word, and performs an unconditional branch.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `transfer_audio_data`: Iterates through a loop to copy words from a source to an audio hardware register, updating a control value and incrementing a byte in a table.
- `enable_dma_transfer`: Writes a word to the DMA control register and performs a short relative branch.
- `transfer_data_with_dma`: Copies three words from a source to a destination, enables a DMA channel, and iterates through a loop using a decrementing counter.
- `decompress_and_reformat_bitstream`: This routine checks a flag to determine whether to clear a buffer or perform a decompression routine.
- `copy_memory_block_2`: Copies a block of data from a source pointer to a destination pointer using nested loops.
- `process_buffered_data`: Decrements a counter in a global variable and either copies a block of data or applies bitwise logic to a buffer based on a table lookup.
- `clear_data_register_bytes`: performs four consecutive logical OR operations with zero on a data register.
- `clear_and_branch`: Clears the data register and performs a short relative branch.
- `transform_data_buffer`: Doubles values in a buffer through a loop, performs bitwise OR operations on specific offsets, and applies a bit-shifting transformation to a segment of memory.
- `process_state_structure`: Clears several memory locations, updates status flags, executes a loop that decrements a value, and performs several indirect calls and bitfield synchronizations.
- `execute_infinite_loop`: Checks a value at an offset from a pointer and enters a loop that compares two global variables before entering an infinite loop.
- `initialize_federation_entity_state`: Resets specific status flags and memory buffers within a data structure before executing a series of state-update routines.
- `copy_words_from_source_to_dest`: Copies a fixed number of words from a source pointer to a destination pointer using a loop that writes several spaced words per iteration and advances both pointers.
- `wait_for_specific_return_values`: The routine repeatedly calls another routine and exits when a zero flag is set or when the returned short equals 84 or 212, then clears a word in a structure pointed to by a6.
- `update_sprite_animation_state`: Checks a state byte to determine if a sprite animation should advance or reset.
- `update_entity_buffer_state`: The routine checks a global value and, if non-zero, enters a loop that updates a global pointer and modifies values in a data structure based on an offset and a counter.
- `update_frame_counter_and_process_status`: Increments a global counter and performs conditional updates to status bytes based on a table lookup.
- `render_sprite_mask_layer`: Initializes a memory buffer with a specific pattern and performs bitwise OR operations to apply sprite pixel data to multiple memory offsets based on a lookup table.
- `update_sprite_buffer_state`: Updates sprite metadata and manages memory offsets for sprite data structures.
- `blit_indexed_sprite_data_2`: Iterates through a data stream to fetch indices and performs block transfers of pixel data into multiple destination buffers.
- `parse_control_sequence_stream`: Iterates through a character stream to decode control sequences and update a state structure.
- `parse_input_stream_and_update_state`: Iterates through a character stream to decode control sequences and update a state structure.
- `calculate_scaled_index_value`: Takes an input value and a table index to retrieve a base value from a global table.
- `test_and_set_status_bits`: Iterates through a loop to test values in a table and updates a status byte by setting or clearing its least significant bit.
- `conditionally_set_flag_byte`: Tests two words from a pointer and either sets the least significant bit of a byte pointed to by a pointer or clears it, then returns.
- `reset_byte_and_parse_stream`: Sets a byte in a structure to 255, calls another routine to parse an input stream, and then sets the overlapping word to zero.
- `process_sprite_metadata_table`: Iterates through a table of sprite data to update a counter based on specific byte values.
- `parse_control_sequence_string`: Iterates through a string to interpret control characters and update internal counters, offsets, and pointers.
- `parse_control_string`: Iterates through a null-terminated string to interpret control characters and escape sequences.
- `check_flag_and_process_sprite`: The routine checks a flag byte and returns early if it is zero, then calls another routine twice and reads a short value from a pointer.
- `decode_tile_plane_wrapper`: The routine checks a flag in a global variable and returns if it is set.
- `skip_if_byte_nonzero`: Tests a byte in a register and branches to another routine if the byte is zero, otherwise branches to a different location.
- `merge_bitplane_data`: Performs bitwise logical operations to combine data from multiple source locations into a destination buffer.
- `call_subroutine_3`: Performs a subroutine call to a specific address.
- `update_global_value`: Adds a value to a word stored at an offset from a global pointer.
- `process_string_counter_table`: Iterates through a table of values to update a series of character-based counters in a destination buffer based on numeric values.
- `update_sprite_graphics_table`: Calculates sprite dimensions and offsets to populate a data structure used for rendering.
- `calculate_and_offset_pointer`: Calculates an offset by multiplying a value derived from subtracting a register from fifteen by twenty, adds this to a pointer in a1, or otherwise modifies a pointer in a2 by adding five hundred twelve and increments a register in d3 before returning.
- `compare_and_call_routine`: Compares a byte in a register to a constant and, if not equal, branches to another location; otherwise, it loads a pointer into a register and calls another routine.
- `call_unnamed_routine`: Performs a call to another routine.
- `load_byte_and_return`: The routine loads a byte from an offset of the argument pointer and returns without further effect after evaluating a range and equality test on that byte.
- `test_byte_range_and_return`: Loads a byte from an offset of the argument pointer and returns without further effect after evaluating a range and equality test on that byte.
- `load_offset_and_branch`: Loads a word into a register and branches to another location.
- `move_address_to_register`: Moves a value from a memory location at an offset of a base pointer into a register.
- `load_word_and_branch`: Loads a word into a data register and performs a short branch to another location.
- `prepare_offset_table`: Loads a pointer from a table, calculates an offset using a constant and a register, calls another routine, and then adjusts a pointer by a fixed amount.
- `loop_offset_pointer`: Calls another routine, decrements a counter in a loop, and increments a pointer by a fixed amount.
- `copy_words_to_offset_table`: Copies a fixed number of words from a source pointer to a destination pointer, writing the first word and several spaced words per iteration and advancing both pointers.
- `mask_buffer_bytes`: Performs a bitwise AND operation on bytes at a destination pointer and several fixed offsets using a calculated bitmask, repeating this process for a set number of iterations.
- `mask_bytes_with_constant`: Repeatedly ANDs a byte from a register with bytes at the destination pointer and at several spaced offsets, advancing the destination pointer by a constant each iteration.
- `perform_bitmask_blitting`: Performs a complex bitwise operation and data transfer involving multiple loops to combine source data with a destination buffer using bitwise AND and OR operations.
- `check_and_update_status`: Tests two words at offsets from a base register and, if both meet specific conditions, calls another routine with a constant and stores two bytes into a structure.
- `update_structure_and_loop`: The routine decrements a value at a pointer with a floor of four, clears several memory locations, and then executes a loop five times that calls another routine with incrementing offsets and pointers.
- `decrement_byte_at_pointer`: Checks a byte at a pointer and, if it is not zero, decrements the value at that same pointer by one.
- `calculate_sum_and_test`: Performs arithmetic operations on a register to calculate a sum and then tests if the result is non-zero to determine a branch.
- `apply_bitmask_to_buffer_2`: Initializes a small table of words based on a bitmask and then performs a nested loop to apply these values to a destination buffer using bitwise operations.
- `process_conditional_bitmask_buffer`: Checks specific flags and memory values to decide whether to execute a jump table or perform a bitwise masking and merging loop.
- `check_and_clear_word`: Tests a word at an offset from a register and, if it is zero, clears a different word at an offset and branches to another routine.
- `test_and_branch_word`: Tests a word at an offset from a register and branches to another routine if the value is zero.
- `update_state_structure`: The routine modifies several fields within a state structure, including clearing a word and conditionally setting a byte to a specific value based on other structure members.
- `parse_control_sequence_stream_2`: Iterates through a character stream to decode control sequences and update a state structure.
- `dispatch_data_structure`: Iterates through a list of structures, checking a bitmask against a value in each entry to decide whether to call another routine with parameters from the structure.
- `scan_strings_for_terminators`: Iterates over a series of null-terminated strings in a global variable, advancing through each string and counting down a limit, stopping when a terminator condition is met.
- `skip_strings_by_count`: The routine advances a pointer through up to a given number of null-terminated strings, moving the pointer past each terminator.
- `dispatch_data_by_index`: Uses a value to select between several different execution paths, which involve calling various routines and copying data from one location to another.
- `copy_byte_and_branch`: Loads a pointer, extracts a byte from an offset of an argument, calls another routine, and then performs an unconditional branch.
- `copy_two_bytes`: The routine copies two consecutive bytes from a source pointed to by one register to a destination pointed to by another.
- `copy_three_bytes_offset`: The routine copies three consecutive bytes from an offset within a word-sized argument to a destination pointer.
- `copy_and_transform_five_bytes`: The routine copies five bytes from a source to a destination, replacing the fourth byte with a zero character if it is one, and modifying the fifth and sixth bytes based on whether the fifth byte is a zero character.
- `compare_and_load_short`: Compares a word at an offset from a register to a constant and, if the comparison is not equal, loads a constant into a register.
- `update_string_counters_2`: Iterates through a table of values to update a series of character-based counters in a destination buffer based on the numeric values found.
- `write_byte_with_conditional_override`: The routine writes a byte to a location before a pointer and conditionally overwrites it based on the value in a register.
- `update_string_counters`: Iterates through a table of values to update a series of character-based counters in a destination buffer based on the numeric values found in a source table.
- `increment_counter_on_match`: Checks if the byte at offset 2 of the structure in a1 is 1 and if the byte at offset 3 is 1, incrementing d0 by 2 for each match found before returning.
- `transform_and_branch`: Calls another routine, moves a value between registers, calls a second routine, loads a pointer, and branches to another location.
- `load_four_words`: The routine loads four consecutive words from offsets relative to a register into four different registers.
- `compute_indexed_value_alt`: The routine computes a composite return value using an index and values from a global table and registers, with a different arithmetic path than the similar routine.
- `store_computed_fields_2`: The routine calls another routine, computes a value from registers, stores it into a structure pointed to by a register, then conditionally updates fields in another structure using values from registers and an input pointer.
- `update_byte_fields`: The routine calls another routine, computes a scaled value from a structure field and an optional divisor, stores a byte into the structure, then conditionally updates a byte in another structure using a byte from an input pointer and a scaled register value.
- `store_computed_fields`: The routine stores computed 32-bit values into a structure pointed to by a register, using data from an input pointer and registers.
- `set_word_field_from_scaled_value`: Clears a byte in a global variable and computes a scaled value from a global variable shifted right, optionally dividing by a register value, then stores part of the result into a word field of a structure pointed to by a parameter while preserving other bits.
- `load_three_words`: Loads three consecutive words from an offset relative to a base pointer into three data registers.
- `return_input_value`: Returns the value it receives in a data register unchanged.
- `copy_bitfield_to_structs`: Copies bits from five source bitfields into five destination bitfields within a set of structures using a nested loop with bit-level indexing.
- `initialize_structure_and_call`: The routine clears a byte in a structure, copies bytes from a short table into a series of four-byte slots with markers, then calls another routine ten times with a decreasing short argument and finally stores a constant into a word field.
- `transfer_bit_reversed_data`: The routine performs a loop that calculates a destination address and populates it with values derived from a source table and a register.
- `store_pointer_sequence`: The routine reads a sequence of words from a global variable and stores the resulting incrementing pointers into a destination table.
- `clear_memory_with_loop`: The routine pushes two values onto the stack, clears a memory location pointed to by a register, and executes a loop based on a constant value before popping the stack.
- `copy_table_to_structure`: The routine copies 32 words from a global variable into a structure pointed to by a register, writes constants to fields in the structure, calls another routine four times, and clears a word in the structure.
- `fill_byte_pattern`: The routine writes a byte to an input pointer, fills the next seven bytes with a constant, then decrements two following bytes by a constant.
- `bit_reversal_transfer`: The routine iterates through a 16-bit value, performs a bit-reversal operation on segments of the data, and stores the results into a destination buffer.
- `copy_words_to_hardware`: The routine copies sixteen words from a global variable to a fixed hardware address by repeated stores.
- `load_byte_from_structure`: Performs bitwise operations on d2 and d1 to calculate an index, then loads a byte from a structure at a specific offset in a6 into d3.
- `test_map_cell_value_4`: The routine computes an index from a register and checks two table entries offset by that index against limits, calling another routine if both limits are satisfied.
- `test_map_cell_value_4_2`: The routine computes an index from a register and checks two table entries offset by that index against limits, calling another routine if both are within range.
- `write_byte_to_indexed_offset`: Calculates an offset using a register value and writes a single byte to a memory location relative to a base address stored in another register.
- `process_indexed_status_list`: The routine iterates through a list of structures, updates a counter, and writes status values to an indexed offset based on specific field conditions.
- `reset_map_state_and_parameters`: Clears specific flags and dimensions within a map structure and resets various configuration variables.
- `clear_and_copy_bytes`: The routine clears a word in a global area, calls another routine, and if a condition holds copies and masks a byte from an output pointer into global bytes and stores a pointer derived from the output pointer.
- `set_bit_conditionally`: Calls another routine and, if the result is non-zero, sets a specific bit in a structure before branching to another location.
- `check_two_word_values`: The routine tests two 16-bit values taken from an address in a register.
- `initialize_map_generation_parameters`: Resets map-related global variables and initializes a grid structure by clearing memory and setting specific dimensions.
- `clear_map_flag`: The routine checks if a word at a specific offset from a base register is non-zero, and if so, clears it to zero before calling itself recursively.
- `setup_and_branch`: The routine saves registers to the stack and performs a branch to another location.
- `call_external_function`: The routine calls a function at a fixed memory address and returns the concatenated results in two registers.
- `initialize_state_and_clear_memory`: Performs a series of recursive calls, clears a block of memory with zeros, and sets three specific values in a structure.
- `process_complex_sequence`: The routine performs multiple recursive calls and executes a loop that writes specific values to a stack-based buffer before performing arithmetic operations on registers.
- `decode_stream_with_loop`: The routine loops while a flag is set, repeatedly calling a helper and another routine, then examines a short value and a word value to decide whether to compute and return a packed result derived from those values; otherwise it returns the word value.
- `perform_recursive_calls`: Executes three consecutive calls to itself before returning a concatenated value from two data registers.
- `copy_from_table_to_global_2`: The routine copies three bytes selected from a source table into a global variable array using an index derived from a structure field.
- `reset_map_generation_parameters`: Clears several state variables and configuration flags within a data structure.
- `load_and_call_subroutines`: Loads two words from a structure into registers, calls two subroutines, and then performs a relative branch.
- `update_structure_fields_5`: Checks if a word in a structure is zero, then conditionally performs arithmetic, updates a global variable, and modifies bytes within the structure.
- `update_buffer_state`: Sets a byte in a structure to a constant value and performs conditional checks on a word and a short within a structure to update various offsets.
- `call_and_check_d2`: Calls another routine and then checks if d2 is equal to 65535, 65533, or 65532 before returning.
- `set_bit_in_buffer`: Compares a word to a constant and, if they match, sets the third bit of a byte located at an offset from a pointer.
- `load_address_to_register`: The routine loads a long word from a memory location into a register.
- `mask_byte_if_match`: The routine tests a byte pointed to by a1 with a mask and, if the masked value equals a constant, clears a specific bit in that byte through the pointer.
- `index_byte_to_word`: Loads a byte from a structure field, uses it as an index into a byte array in the same structure, and if the indexed byte is non-zero it multiplies the index by four before returning it as a word.
- `increment_structure_field`: Calls another routine and, if the zero flag is not set, increments a word at a specific offset within a structure.
- `update_entity_state_and_counters`: Updates various fields within a structure based on bitwise tests and register comparisons, including incrementing counters and modifying a byte in a table using bitwise masking and shifting.
- `compare_and_call_subroutines`: Compares a word in a register against several constants, increments a global variable if a match is found, and then calls two subroutines while using the stack to preserve a value.
- `execute_subroutines_and_branch`: Loads constants into registers, calls two different subroutines, and then performs a relative branch.
- `test_bit_and_branch_2`: Tests a bit in a memory location and branches if set, otherwise compares a word in a register against several constants before returning.
- `test_hardware_and_branch`: Tests a bit in a hardware register and conditionally increments a global variable, then performs two subroutine calls and moves a value from the stack to a register.
- `call_multiple_subroutines_2`: Loads constants into registers and calls three different subroutines in sequence before branching to another location.
- `fetch_byte_from_table`: The routine returns a byte read from an address computed from an unreferenced register and an index derived from another unreferenced register and a global variable.
- `call_other_routine`: The routine calls another routine and then returns.
- `update_structure_fields`: Stores two bytes into a structure, calls another routine, and then stores two more values into a structure at different offsets.
- `loop_indefinitely_after_check`: The routine checks a short value from an unreferenced base and conditionally executes a bounded loop that compares two global variables, then enters an infinite loop.
- `iterate_four_times_calling_routines`: The routine initializes a short counter and a pointer offset, then loops four times.
- `check_hunk_size_range`: The routine computes a difference of two shorts from an input pointer and returns early if the difference is within a specific range, otherwise it falls through to return.
- `check_word_and_call`: Loads a word from a structure, checks if it is equal to two, and if not, loads a new address into a register and calls another routine.
- `update_clamped_field`: Checks multiple flags in a structure and conditionally modifies a byte field by subtracting a value and clamping the result to zero.
- `adjust_and_clamp_int_field`: The routine reads an integer field from a structure pointed to by a pointer, adds a scaled value from another pointer, writes the result back, and clears the field to zero if the result is negative.
- `conditionally_update_pointer`: The routine tests a short and a uint from an input pointer and, if conditions hold, adds an adjusted input value to a field in the pointed structure.
- `call_and_return_concat`: The routine calls another routine and returns a 64-bit value formed by concatenating two input words.
- `call_and_return_2`: The routine calls another routine and then returns without further work.
- `process_conditional_table_updates`: Tests a value against a constant and, if matched, performs a series of conditional subroutine calls and updates various structure members.
- `set_byte_to_constant`: The routine stores a constant byte into a location calculated from an offset of a register.
- `loop_and_call_subroutines`: Tests a word in a structure and, if it matches a specific value, enters a loop that calls a subroutine multiple times with varying register values.
- `increment_and_wrap_counter`: Increments a word at a specific offset in a structure and wraps it to zero if it exceeds seven.
- `clear_and_call_subroutines`: Tests a word in a structure and, if zero, clears a word in a global area and calls two subroutines before performing a relative branch.
- `update_structure_and_branch`: Tests a word in a structure and, if non-zero, clears a word in a global area, calls two subroutines, increments a global word, calls a third subroutine, and performs a relative branch.
- `test_and_call_routine`: Calls a routine and, if the result is non-zero or if a word at a specific offset is non-zero, branches to another location; otherwise, it calls a second routine.
- `toggle_flag_and_store`: Tests a bit in a memory location and, if not set, sets the bit and stores a byte into a structure.
- `test_and_branch_on_values_2`: Tests a specific bit in a global variable and branches if it is set, then checks if a word at an offset from a register is zero before calling another routine.
- `test_conditions_and_call`: The routine calls a subroutine, branches if a condition is met, tests a word at an offset from a register, and calls another subroutine if the word is non-zero.
- `test_and_reset_bit`: Tests a specific bit in a global variable and branches if it is set.
- `test_and_branch_on_values`: The routine tests two words at specific offsets from a register and branches accordingly before returning.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `clear_flag_and_store_bytes`: The routine checks a byte from an unreferenced base for a flag and equality with another byte, then clears the flag in the first byte and stores constant bytes to two other locations.
- `clear_flag_on_match`: Checks if a bit is set in a byte, compares that byte to another value, and if they match, clears the bit and stores two bytes into a structure.
- `clear_flag_on_match_2`: Tests a word at an offset from a register and, if it is zero, stores two bytes from a structure into other offsets within the same structure before jumping to a calculated address.
- `loop_through_table_entries`: Moves two bytes into a structure, compares a word to a value at a pointer, and then enters a loop that iterates seven times to modify a pointer.
- `calculate_table_index_2`: Stores a word from one location into a structure, calls another routine, and then modifies the structure with a masked value and a constant before calling another routine.
- `mask_input_word`: The routine returns the input word masked with a constant, keeping only the lowest two bits.
- `loop_until_null_or_nonzero`: Repeatedly calls another routine and exits early if a specific field is non-zero, otherwise it loops while a byte from a global variable indexed by a shifted word is non-zero.
- `calculate_and_store_difference`: The routine computes the difference between a value in a structure and a returned value, clamps the result to zero if negative, and stores it back.
- `save_registers_and_branch_2`: Saves several data and address registers to the stack, calls another routine, loads an address into a register, and performs a short branch.
- `update_structure_fields_4`: The routine calls another routine, computes a value from registers, stores it into a structure pointed to by a register, then conditionally updates a byte field in the structure based on a short field and a byte comparison, masks and writes a byte field using a computed word, calls another routine with concatenated values, and returns a concatenated result.
- `update_structure_fields_3`: Calculates a value based on input registers, updates a byte field in a structure using a clamped difference, and modifies a word at a calculated offset in a table.
- `conditional_buffer_clear`: The routine tests a short value and a byte value at fixed offsets from a base register together with a word value, and if the condition holds it calls another routine.
- `sync_bitfield_structures`: Synchronizes multiple bitfields within a set of structures by iterating through source bitfields and applying bitwise transformations to corresponding destination offsets.
- `update_data_from_table`: Calculates a pointer to a table based on a value in a structure, reads three consecutive bytes, and stores them into three different locations within that structure.
- `restore_structure_values`: Copies values from three specific memory locations back into a structure and clears a field at a fixed offset.
- `apply_sprite_mask_to_planes`: The routine sets a pointer to a location within a structure and then enters a loop that calls another routine multiple times while incrementing a counter.
- `clear_memory_block_2`: Fills a block of memory with zeros using a loop that runs for a fixed number of iterations.
- `clamp_integer_field`: Subtracts a scaled value from an integer field in a structure and clamps the result to zero if it is negative.
- `update_pointer_from_offset`: Calculates a new address based on a pointer and an offset stored in a structure, then stores this address into another location.
- `loop_and_clear_value`: Executes a loop that increments a value by eight until it reaches a threshold, then clears a word at a specific offset in a structure.
- `clear_nested_structure_fields`: Iterates through a nested loop to clear multiple four-byte words in a structure pointed to by a value stored in another structure.
- `compare_byte_limit`: Compares a byte at an offset in a structure to a constant and branches if it is less than or equal to that constant.
- `compare_multiple_bytes`: Compares two different bytes at offsets within a structure to a constant and branches if they are less than or equal to it.
- `compare_and_branch_byte`: Compares a byte at an offset within a structure to a constant value and branches if the value is less than or equal to that constant.
- `conditionally_store_byte_to_fields`: The routine conditionally stores an input byte to a byte field in a structure based on a byte field value, returns early if another byte field is greater than a threshold, and otherwise stores the input byte to three byte fields in the structure.
- `branch_to_handler`: Calls a routine and then performs an unconditional branch to another location.
- `render_federation_text_block`: Reads two short values and a two-bit field from a structure pointed to by A6, then uses them to index into a 64-byte-per-row table located at an offset from A6.
- `blit_sprite_pixel_data`: Fetches sprite metadata to determine source and destination memory addresses, then performs a software blit using nested loops.
- `update_sprite_animation_state_3`: Updates sprite animation frame indices and metadata by evaluating bitmasks and state transitions.
- `process_sprite_animation_sequence`: Updates sprite animation frame indices and metadata by recursively evaluating a bitmask and a sequence of state transitions.
- `call_subroutine_and_return`: Calls another routine using a word-sized branch instruction.
- `execute_recursive_logic`: Checks a bit in a structure and, if set, calls itself recursively before performing further conditional checks and calls.
- `execute_recursive_calls_2`: The routine performs two consecutive calls to itself before returning.
- `update_sprite_animation_state_2`: Calculates and updates sprite animation frame indices and metadata based on a counter and a table of animation data.
- `call_other_routines`: The routine calls another routine three times in sequence and then returns.
- `test_bit_and_modify`: Tests a bit in a register and, if set, modifies a byte and conditionally calls another routine.
- `call_two_subroutines`: This routine calls two other routines in sequence and then returns.
- `process_descriptor_table`: Iterates over a short list using a counter, and for each positive entry writes three global variables with decreasing values and calls another routine with parameters derived from the list and inputs.
- `clear_memory_blocks`: If specific conditions are met regarding a register and a table value, this routine performs nested loops to clear multiple blocks of memory.
- `update_sprite_animation_frames`: Validates an animation index and decrements it before calculating a memory offset into a sprite data table.
- `execute_chained_subroutines`: This routine performs two subroutine calls and then executes a relative branch.
- `search_table_for_match`: Iterates through a table of words to find a match against a value stored in a structure.
- `update_sprite_graphics`: Validates an animation index and uses it to fetch sprite data from a table.
- `validate_animation_index`: Checks if a word in a register is between zero and ten.
- `update_animation_frame`: Compares a word value against a constant and conditionally decrements a counter, adds it to a register, and branches to other routines or performs a bit test on a memory location.
- `check_flag_and_process`: The routine calls another routine, tests a flag byte in a structure pointed to by a register and conditionally calls another routine with registers, then calls another routine unconditionally and returns a register value.
- `test_bit_and_branch`: Tests a bit in a memory location and branches if the bit is zero.
- `execute_conditional_subroutines`: Performs two subroutine calls and then conditionally executes a third subroutine and another call if a specific bit in a byte at a structure offset is set.
- `execute_recursive_call`: The routine performs a recursive call to itself and then returns.
- `blit_indexed_sprite_data_3`: Fetches sprite metadata from a table and performs a software blit of pixel data to a destination.
- `load_word_and_branch_2`: The routine loads a word from a memory location calculated using a register and an offset into a register, then branches to another location.
- `load_alternate_word_and_branch`: The routine performs one of two operations: it either loads a word from a calculated address into a register or loads a word from a different offset into a register, before branching to another location.
- `check_string_termination`: Tests if a value at a memory location calculated from a pointer and an offset is non-negative.
- `execute_sprite_rendering`: Calls several routines to check flags, render animated frames, and perform indexed sprite data blits.
- `dispatch_animation_type`: Compares a word in a register against several small constants.
- `render_animated_sprite_frame`: Calculates sprite source offsets based on animation state and frame metadata, then performs a software blit of pixel data to a destination buffer.
- `call_next_routine`: Performs a subroutine call to another routine.
- `add_bytes_to_struct`: The routine adds two input bytes to two other bytes and stores the results into two bytes of a structure pointed to by a register.
- `compute_and_store_index`: Checks if two values are within specific limits and, if so, stores a calculated value into a structure.
- `calculate_table_index`: Checks if two bytes at specific offsets within a structure are within defined limits, and if so, stores a computed word into a fixed offset of that structure.
- `call_subroutine_4`: Performs a branch to a subroutine.
- `compute_table_index`: The routine checks two input words against limits and, if both are within range, stores a computed word into a fixed offset of a structure pointed to by a register.
- `move_word_from_offset`: The routine moves a word from an offset of a structure pointed to by a register into a register.
- `modify_registers_by_d6`: Checks the value of d6 masked by 3; depending on whether d6 is 1, 2, or other values, it performs exchanges or negations on d2 and d3 before returning.
- `copy_memory_blocks_2`: Copies data from a source table to a destination buffer using nested loops to handle multiple blocks of a specific size.
- `branch_to_subroutine`: Calls a subroutine and then performs a conditional branch based on the status of the processor flags.
- `load_address_and_branch_3`: Loads a memory address into a register and then performs a short branch to another location.
- `calculate_offset_with_bounds`: Checks if a value is within specific ranges and, if so, returns the sum of that value and another word; otherwise it returns negative one.
- `scale_value_and_return`: The routine reads a word from a pointer, scales it with a constant and an offset, writes the scaled word back through the pointer, and returns a combined value derived from the original and scaled words.
- `scale_word_in_place`: The routine reads a word from a pointer, multiplies it by a constant and adds an offset, then writes the result back to the same location.
- `return_scaled_field`: The routine reads a word from a fixed offset of a structure pointed to by a register, multiplies it by a constant, and returns the product.
- `branch_to_address`: Performs a short branch to a specific address.
- `accumulate_value_to_memory`: Adds a value stored in a data register to a word located at an offset within a structure pointed to by a base register.
- `branch_to_address_2`: Performs a short relative branch to a specific memory address.
- `fill_memory_and_set_constants`: Fills a memory range with ones using a loop and sets three global values to specific constants.
- `branch_to_offset_3`: This routine performs a single short relative branch to a fixed address.
- `branch_to_data_processing`: Performs a short relative branch into the middle of another routine.
- `branch_to_offset`: Performs a short relative branch to a specific instruction offset within another routine.
- `branch_to_middle_of_routine_2`: Performs a short relative branch into the middle of another routine.
- `branch_to_offset_2`: Performs a short relative branch to a specific location within another routine.
- `branch_to_target`: Performs a short relative branch to another routine.
- `branch_to_status_update`: Performs a short relative branch to a location within another routine.
- `store_computed_fields_3`: Computes values from registers and stores them into a structure pointed to by a register.
- `update_byte_fields_2`: Computes a scaled value from a structure field and stores it into a byte within that structure.
- `check_byte_equality`: Loads a byte from a memory location pointed to by an argument and compares it against a value.
- `branch_to_two_routines`: Performs two consecutive short branches to different locations in the code.
- `branch_multiple_targets`: Performs a series of short branches to different addresses.
- `decrement_and_tally_values`: Iterates through a table of numeric values to decrement a target value while incrementing corresponding character counters in a destination buffer.
- `perform_no_operation`: This routine is empty and performs no operations before returning to its caller.
- `execute_double_branch`: Performs two consecutive short relative branches to different memory addresses.
- `branch_to_middle_of_routine`: Performs a short branch into the middle of another routine.
- `decompress_and_reformat_bitstream_2`: Checks if a data buffer requires updating by comparing two memory locations.
- `test_and_branch`: Tests a word at a specific offset from a register and branches if it is not zero.
- `process_scaled_data`: Decrements a value in a structure, then enters a nested loop that performs bitwise operations on data at calculated offsets based on multiple retrieved values.
- `fetch_data_from_offset_table`: Calculates an address by adding a scaled word to a pointer, loads a word from that address, and then branches to another location.
- `blit_indexed_sprite_data`: Fetches sprite metadata from a table and performs a block transfer of pixel data.
- `update_structure_fields_by_bitmask`: The routine tests bits in a word and, depending on the result, writes constant or derived values to specific offsets of a structure pointed to by a register.
- `blit_sprite_with_transparency`: Performs a software sprite blit by iterating through pixel data and applying a bitwise transparency mask.
- `clear_struct_members`: The routine iterates through a list of pointers, setting specific offsets within each structure to zero or a constant value.
- `clear_memory_block_3`: The routine iterates through a list of pointers and sets the value at each pointer to zero.
- `process_indexed_values`: Iterates through a list of indices to check specific byte values and calls another routine if certain conditions are met.
- `initialize_and_branch`: Clears a word at a location derived from a register, clears a long value at a global offset, and performs a branch to another routine.
- `update_state_and_copy_data`: Updates a state variable based on existing values and performs a multi-layered block memory copy from a source table to a destination buffer.
- `copy_block_data_and_set_char`: Performs a nested loop to copy data from a source buffer to a destination buffer in blocks of specific sizes.
- `format_federation_string_2`: Copies a string into a destination buffer and performs arithmetic transformations on specific bytes to populate a structured data block.
- `format_federation_string_3`: Copies a source string into a destination buffer and performs arithmetic transformations on specific bytes to populate a structured data block.
- `format_identifier_string`: Iterates a fixed number of times to copy data from a source into a destination buffer while conditionally inserting incrementing characters based on a bitmask.
- `execute_string_formatting`: Performs two consecutive calls to another routine using different source and destination pointers retrieved from a table.
- `format_federation_string`: Copies a string from a source into a destination buffer and performs arithmetic transformations on specific bytes to populate a structured data block.
- `format_string_buffer`: Calculates several character values based on two bytes from a structure and writes them into a destination buffer at specific offsets.
- `execute_recursive_calls`: Calls itself twice before returning.
- `increment_and_update_table`: Increments two counters in a global structure and updates a table entry with a value derived from a bit-shifted sum and a byte from another location.
- `decrement_global_short`: Decrements a two-byte value stored at an offset within a structure if it is not zero.
- `update_pointer_and_conditional_offset`: Loads a pointer from a global variable into a register and stores a word from a register into a global variable.
- `parse_command_string`: Iterates through a null-terminated string to interpret specific character codes as commands.
- `apply_masked_word_updates`: Builds a mask from a global variable and a parameter, inverts it, then iterates five times over a block of memory pointed to by a parameter.
- `execute_indirect_call`: Calls a routine through a pointer stored at an offset within a global structure.
- `check_zero_flag_and_call`: Calls another routine and, if the zero flag is set, calls a second routine and returns zero.
- `parse_encoded_byte`: Evaluates a byte from a pointer against several numeric ranges and performs conditional branches or memory updates based on the result.
- `search_linked_short_list`: The routine walks a linked list of short entries starting from a pointer in a register.
- `manage_dynamic_string_buffer`: This routine performs memory management for a dynamic string or data buffer, handling resizing and character insertion.
- `move_word_to_register_2`: Moves a word from an offset of a pointer in a register into a register.
- `shift_buffer_and_patch_list`: The routine computes a length from two shorts in a pointer, updates the second short with a short from a register, then copies a block of ints from a source derived from that short into a destination pointer.
- `search_linked_list`: Loads an address into a register, calls another routine, and branches based on the result.
- `save_and_restore_word`: The routine saves a word from a fixed offset of a structure, calls another routine, then restores the saved word to the same offset.
- `mask_byte_if_allowed`: The routine reads a byte through a pointer, masks it, checks it against a set of allowed values and a range, and if the check passes it clears the least significant bit of the byte through the same pointer.
- `update_table_entry_conditional`: The routine calls another routine and then conditionally writes a word and two bytes to a table entry and updates a byte in another structure based on a byte value derived from an input word.
- `initialize_data_structures_2`: The routine fills several memory ranges with specific constant values and clears various offsets within a structure.
- `execute_nested_loops`: Sets up two registers for loop counters and performs a nested loop that calls another routine.
- `nullify_on_delimiter`: The routine examines the byte following a pointer in a register.
- `check_short_table_match`: The routine compares a short from a global variable with four consecutive shorts stored in a global structure.
- `process_input_and_state`: Enters a loop that calls other routines and checks specific conditions within a global structure.
- `clamp_and_store_byte_values`: Clamps three input values to a maximum of two hundred and fifty-five and stores them into global byte variables.
- `reset_and_update_state`: Sets a long value to all ones, clears a byte at a calculated offset, calls two other routines, and sets a byte to one.
- `reset_global_state`: Sets several bytes and words in a global structure to specific values, including copying a long value and two bytes from other global locations.
- `update_structure_values`: Stores two bytes from a structure into another location within a structure and updates a long value using a global variable.
- `repeat_call_until_threshold_store`: The routine repeatedly calls another routine while a word output is greater than a threshold, stores that word into a fixed offset of a structure, and stores zero to the next word if the output is zero.
- `pass_through_registers`: Copies the contents of two data registers to the return value.
- `update_structure_state_bytes`: Copies two bytes from a structure into two consecutive global variables and then executes a subroutine before branching to another location.
- `manage_structure_state`: Copies bytes from a structure to a temporary global area, performs several subroutine calls while swapping values between a structure field and the temporary area, and executes a loop that terminates based on a value in a register.
- `swap_values_if_equal`: The routine compares two input values with an unrelated value and stores one input into a memory location pointed to by a pointer when a match occurs.
- `update_state_and_index_structure`: Updates a structure by decrementing a counter and modifying specific bitfields within a byte.
- `update_and_index_structure`: Copies two consecutive bytes from a pointer into a structure and, if both bytes are within specific bounds, calculates a linear index based on their values and stores it at a fixed offset within that structure.
- `test_byte_and_branch`: The routine tests a bit in a byte at an offset from a register and branches if set, otherwise compares a byte to a value and branches if it is higher.
- `copy_d0_d1_to_stack`: Moves the contents of two data registers onto the stack.
- `manage_entity_state_and_counters`: Increments or decrements a byte value within a structure and toggles specific bits in a status byte.
- `call_and_branch_on_sign`: The routine loads an address into a register, calls another routine, and then branches based on the sign of the result.
- `call_and_branch_on_sign_2`: The routine loads a different address into a register, calls another routine, and then branches based on the sign of the result.
- `load_address_and_branch`: The routine loads an address into a register and branches to another routine.
- `load_address_and_branch_2`: The routine loads a different address into a register and branches to another routine.
- `store_bytes_and_search_table`: The routine stores two input bytes into a structure pointed to by a register and then searches a table pointed to by a pointer for a matching short value for three iterations, returning early on a match.
- `tiny_moveq`: Small routine of 1 instruction that loads 255 into d3.
- `store_calculated_offset`: Calculates a value based on two inputs and stores the result into a global variable offset.
- `initialize_data_block`: Clears several fields in a memory block with zeros and sets specific fields to maximum values before loading a value from a table.
- `execute_conditional_subroutine`: The routine loads a word from a table, saves registers to the stack, calls another routine, and then performs arithmetic on a pointer before returning.
- `copy_from_table_to_global`: The routine copies three bytes selected from a source table into a global variable array using an index derived from a structure field, iterating with a fixed count.
- `check_attribute_threshold`: Iterates through a table and calls another routine if a byte retrieved via a masked index is less than three.
- `check_attribute_range`: Iterates through a table and calls another routine if a byte retrieved via a masked index is between three and five.
- `check_attribute_range_2`: Iterates through a table and calls another routine if a byte retrieved via a masked index is greater than five.
- `update_attribute_buffer`: Calculates and updates various attributes and flags within a data structure based on values retrieved from a table.
- `calculate_offset_coordinates`: Calculates new coordinate values by adding elements from two arrays, performing bitwise rotations on a value from a structure, and applying conditional offsets based on flags.
- `rotate_and_add_values`: Performs a bitwise rotation on a value in a structure if a specific flag is set, then returns the sum of a value from a structure and a value from another structure.
- `compare_byte_with_pair`: Iterates through a table and calls another routine if a byte at a specific offset matches one of two values stored in a global structure.
- `compare_byte_with_other_pair`: Iterates through a table and calls another routine if a byte at a specific offset matches one of two different values stored in a global structure.
- `calculate_jump_address`: Loads a byte from a global variable, adds it to a base address, and performs a branch to the resulting location.
- `update_sprite_animation_and_position`: Updates sprite animation frames and increments frame counters based on a comparison between a target value and a stored state.
- `modify_register_and_load_data`: Adds a value to a register, loads a pointer from a global variable, retrieves a word from a table using the modified register, and performs arithmetic on the result.
- `calculate_sprite_attribute_buffer`: Calculates sprite dimensions and offsets based on input parameters to populate a data structure.
- `update_linked_list_data`: The routine searches a linked list for a matching value, and if found, increments a value at an offset, shifts a word right, and updates two other values in a structure.
- `load_pointers_to_registers`: Loads two memory addresses into two data registers from offsets relative to a base register.
- `search_linked_list_2`: Traverses a linked list to find a matching short value.
- `modify_byte_via_pointer`: Clears a specific bit in a byte located at an address derived from a pointer and a global offset.
- `copy_interleaved_buffer_data`: Copies data from a source pointer to a destination pointer using a nested loop structure that performs interleaved memory access.
- `calculate_offset_pointer`: Loads a word from a table relative to the program counter and adds it to a pointer in a register.
- `add_offset_to_pointer`: Loads a word from a location relative to the program counter into a register and adds it to a pointer in another register.
- `merge_bitmask_data`: Performs a nested loop to bitwise merge bytes from five different source offsets into a destination buffer using a calculated mask.
- `call_subroutine_2`: The routine calls another routine.
- `copy_sprite_blocks`: The routine copies five bytes from a source pointer to a destination pointer for each of three blocks, using a global variable and an input.
- `conditional_call_by_byte_flag`: The routine tests a byte pointed to by a register for a flag and calls another routine only when the flag is clear.
- `calculate_scaled_address`: Loads a byte from a structure at offset 390 in a6 into d6, multiplies d6 by 270, and adds the result to a pointer at offset 382 in a6.
- `clear_and_merge_federation_blocks`: The routine clears a set of federation data blocks in a global structure and then merges five bytes from each block into a destination buffer.
- `clear_and_merge_federation_blocks_twice`: The routine invokes another routine twice and then returns.
- `update_bitmask_and_blend_graphics`: Calculates a bitmask based on a variable and applies it to several memory locations.
- `copy_block_repeated`: Repeatedly copies four 8 KiB blocks from a source pointer to a destination pointer and advances the destination pointer by 40 bytes each iteration until a counter reaches -1.
- `decrement_table_value_and_recurse`: Decrements a value in a table at an offset determined by a scaled index and calls itself recursively multiple times if the initial value is non-zero.
- `update_object_attributes_from_table`: Checks specific coordinate and value ranges to select and store data from a lookup table into a structure.
- `shift_left_value`: Returns a value shifted left by two bits.
- `update_object_state_and_clear_flags`: Updates an object's attribute byte based on a value from a table and performs conditional execution of other routines.
- `reset_pointer_if_nonempty`: Tests whether the second byte of a structure pointed to by a register is zero, conditionally calls another routine and returns a sentinel value, otherwise writes zero to the structure and returns.
- `test_and_execute_sequence`: Tests a byte at an offset from a register, and if non-zero, performs a sequence of calls and register saves before clearing a long value and branching.
- `copy_and_verify_bytes`: Copies three consecutive bytes from a source to a global structure and calls another routine to check if the third byte is zero.
- `copy_table_entry_if_tagged`: If a byte at an offset from a table pointer equals 'c', it computes a destination pointer from a byte at another offset and copies 16 words from the table pointer to the destination.
- `copy_memory_if_char_matches`: If a byte at a specific offset matches a character, the routine copies a block of data from a calculated source to a destination pointed to by a register.
- `shift_and_load_pointer`: Shifts a word in a register left by two, loads a long value from a location relative to a base register, and then calculates a new pointer by adding an offset to that value.
- `tiny_adda`: Small routine of 1 instruction that adds d0 to a0.
- `swap_and_update_value`: Swaps a long value between a pointer and a memory location, then performs arithmetic on specific bytes within that memory location based on a comparison.
- `sync_object_attributes_from_table`: Copies specific data bytes from a lookup table into a structure based on an index and updates related fields via word copies and decrements.
- `fill_memory_and_set_values`: Calls another routine, fills a memory range with ones, and then sets three specific global values to specific constants.
- `initialize_object_memory_and_attributes`: Fills a memory range with specific patterns, sets several global constants, and populates a table with pointers.
- `fill_memory_and_set_values_2`: The routine fills a memory range with ones, sets specific global values to constants, and then populates a table with pointers to a global variable.
- `update_structure_fields_2`: Sets two global variables to specific constants, copies bytes between offsets of a structure, and performs an indirect call through a pointer stored in a structure.
- `copy_word_to_offset`: Copies a word from a memory location to a specific offset within a structure.
- `copy_word_to_offset_alt`: Copies a word from a memory location to a different offset within a structure.
- `clear_memory_word`: Sets a word at a memory location calculated from a base register and an offset to zero.
- `empty_routine_9`: The routine contains five instructions and performs no observable operation on registers, memory, or global data.
- `iterate_data_table`: Loops through a table of data, compares values against a target, and conditionally updates a global variable or calls another routine based on character checks.
- `calculate_sprite_offset_and_bounds`: Calculates a sprite's position and bounding box by applying offsets and bitwise modifiers to values retrieved from a data table.
- `modify_register_bits`: Applies a bitwise AND mask to a word, performs a single-bit rotate right on a byte, and then performs a bitwise OR with another word.
- `try_insert_federation_entry`: The routine walks a small circular index stored in a global variable and attempts to insert data into a table pointed to by an argument.
- `concatenate_registers`: Combines two 32-bit registers into a single 64-bit return value.
- `process_string_data`: Evaluates characters at specific offsets in a pointer, modifies a byte at an offset, and conditionally clears a bit or updates a global word based on various character comparisons and structure values.
- `update_entity_state_and_attributes`: Decrements a counter in an entity structure and modifies its state flags based on specific character identifiers.
- `update_entity_state_flags`: Decrements a counter within an entity structure and modifies state flags based on specific character identifiers.
- `compute_table_index_2`: Checks two input bytes against specific limits and, if both are within range, stores a computed word into a fixed offset of a structure.
- `mask_bytes_and_call_helper`: The routine masks a byte from a register with 15, ORs it with an unrelated byte, and calls another routine with the combined value.
- `validate_and_iterate_data_structures`: Compares bytes from a pointer against specific values in a global structure to determine a branching path.
- `update_resource_values`: Calculates a new value based on a multiplication and bit-shifting operation, then updates a global counter and a structure field.
- `update_structure_via_table`: Stores a value derived from a table and a bit-shifted register into a structure offset, then copies a word from another structure into a fixed offset.
- `multiply_bytes_from_pointer`: Loads bytes from offsets 6 and 7 of the structure in a0 into d0 and d1, then multiplies d1 by d0 and returns.
- `check_extraout_d1w`: Calls another routine and then checks a short value in a register against negative one using multiplication and shift operations.
- `scan_for_byte`: Computes a pointer from a global variable and a register offset, builds a length from a byte in that area, then scans forward byte by byte until a matching byte is found or the length expires.
- `set_flags_from_table`: Reads a word from a table indexed by a byte, derives a flag byte from its low nibble and bits, stores it in a global variable, sets another global byte, and calls another routine before returning the two input registers concatenated.
- `validate_structure_conditions`: Evaluates multiple conditional flags and bitwise states within a structure and returns a status value based on the results.
- `set_bit_in_field`: Iterates through a sequence of values to find the first bit that is not set in a specific field within a structure.
- `update_complex_state_values`: Performs conditional arithmetic and table lookups to update multiple fields in a structure, including a value derived from a multiplication and a subtraction.
- `toggle_bit_and_branch`: Loads a word from a memory location using an offset from a base register, performs a bitwise exclusive OR operation on it, and branches to another location.
- `modify_structure_field`: Loads a word from a structure, performs bitwise operations, calls another routine, increments a byte in a structure, and branches to another routine.
- `check_unaff_d2w_value`: The routine receives an unsigned word in an unspecified data register, checks if it is less than two and returns early if so, then falls through to a second return.
- `compare_and_branch`: Compares a word at a specific offset against a value, branches if less than or equal, then compares another word against a value and branches if higher than, before loading a byte from a relative address into a register.
- `set_flag_and_branch`: Calls another routine, sets a bit in a register, and branches to another routine.
- `process_input_and_update_state`: The routine checks a value in a structure, potentially decrements a byte, and then updates a variable using a table lookup and a call to another routine.
- `forward_input_pair`: Calls another routine and returns the two input registers concatenated unchanged.
- `update_calculated_values`: Updates several memory locations using arithmetic operations involving a global variable, a table, and values derived from multiplication and bit shifts.
- `branch_on_result`: Calls another routine and then branches if the result is negative.
- `compute_complex_value`: Loads two words from a table using an offset derived from d0, performs a series of arithmetic operations including multiplication, division, and bit shifts involving d2, d3, d4, d5, and d7, and returns a value in d0.
- `compare_and_check_flag`: Compares a byte from a table with a value derived from a global variable and a register, then checks a specific memory flag before potentially returning a specific value.
- `parse_character_value`: Evaluates a byte from memory against several numeric ranges and returns a calculated value or sets a memory flag based on the input.
- `transform_hardware_register_values`: Reads a word from a hardware register, performs a multiplication and an addition, and writes the result back to that register.
- `traverse_linked_list_2`: Iterates through a list of structures, incrementing a byte in each entry or resetting it and recursing if a condition is met.
- `process_recursive_state_updates`: Performs recursive state updates by checking bit flags in a structure and searching a global table for matching values.
- `update_counter_value`: Multiplies a short at a fixed offset of a structure by 1509, adds 41, and stores the result back to the same offset.
- `move_and_decrement_pointers`: Moves bytes from four different memory locations into locations pointed to by four different registers while simultaneously decrementing those registers.
- `update_status_byte_2`: Checks various conditions involving memory contents, a table, and a register to conditionally modify a byte in a structure using bitwise operations.
- `update_entity_state`: Modifies a byte value through a pointer and updates a byte at a fixed offset in a structure based on specific conditions.
- `clamp_and_store_byte_values_2`: Clamps three input values to a maximum of two hundred and fifty-five and stores them into global byte variables.
- `calculate_offset_value`: Computes a value based on a word from a memory location, a flag, and a bit-shifted value from a hardware register.
- `copy_bytes_and_set_flag`: Copies two bytes from offsets in a structure to other offsets, calls two other routines, and writes the value 4 to a global variable.
- `update_bit_flags_loop`: If a specific bit is set in an input structure, it updates a global variable using a linear congruential generator and iterates through a list of structures, clearing two bits in a byte and setting a third bit based on the sign of the calculated value.
- `search_table_and_recursive_call`: Checks a flag in a structure, computes offsets, searches a global table for a matching byte, and recursively calls itself while looping over a count.
- `process_attribute_lookup_table`: Processes a byte from a pointer using a global table to obtain an offset and a flag.
- `self_recursive_call_2`: The routine immediately calls another routine with the same entry point and then returns.
- `update_sprite_attributes`: Calculates new position and attribute data for two sprite objects based on a lookup table and coordinate offsets.
- `store_computed_display_value`: The routine calls another routine and stores the returned byte into a byte at an offset from a pointer.
- `concatenate_two_routine_results`: Calls another routine twice and returns a 32-bit value formed by concatenating the two results.
- `calculate_bit_mask_value`: Performs a mathematical transformation on a value in a global variable and uses bitwise operations to extract and return a value based on a table lookup and bit testing.
- `copy_memory_blocks`: Copies blocks of data from a source pointer to a destination, using a fixed stride and multiple word-sized writes per iteration.
- `search_and_loop`: Searches for a byte matching a specific bit pattern in a table and then enters a loop that calls another routine until a register is zero.
- `mask_and_merge_words`: The routine loops over a small number of iterations, loading five words from a base pointer at fixed offsets and then merging them with a destination buffer using a bitwise OR of the destination with the inverse of the OR of the five source words.
- `merge_and_clear_data_blocks`: Iterates through a series of data structures to perform bitwise OR operations between specific word-sized fields and then either clears or overwrites them based on a control value.
- `scale_byte_value`: The routine reads a byte from an address derived from a register, multiplies it by seven, shifts the result right by three and stores the resulting byte back to the same address.
- `build_bit_masks`: For each set bit it sets corresponding bits in the two output areas using a rotating bit position.

### Captive (1990)(Mindscape)[cr CLS][a2].adf:fed_bootfile

- `initialize_system_state`: Performs a memory clear and executes a sequence of function calls through pointers in a global structure, writing specific values to a memory offset during the process.
- `initialize_bitplane_copper_list`: Clears a large memory block and populates it with a sequence of copper instructions and bitplane configuration data.
- `conditional_table_dispatch`: The routine calls a function via an address in A6, and if the result is non-negative it calls another function via an address in A6 and returns that result, otherwise it returns zero.
- `fill_jump_tables`: The routine fills two tables with successive addresses of another routine, advancing the address by six bytes each iteration for the first table and keeping the address constant for the second table.
- `call_system_initialization`: This two-instruction routine performs a PC-relative load effective address to prepare a jump or call and then invokes another routine.
- `call_initialization_routine`: This routine consists of two instructions and calls another routine that clears a memory block, copies a data structure, and enters a loop that modifies a global variable and toggles a value.
- `save_registers_to_ram`: The routine stores the contents of all data and address registers, as well as the current stack pointer, into a series of RAM locations and returns.
- `save_context_and_initialize_environment`: Saves the current processor state, including registers and the stack pointer, to a memory block before clearing a data table and copying a specific set of values into a new location.
- `initialize_system_state_and_loop`: Clears a memory block, copies a data structure to a new location, and enters a loop that modifies a global variable and toggles a value between specific constants.
- `initialize_stack_and_setup_loop`: Sets up a stack frame by storing parameters and pointers, clears a memory block, and copies a data table to a new location.
- `initialize_hardware_and_loop`: Clears a block of memory, copies data from a global variable to a new location, and enters a loop that modifies a global variable and toggles a state value.
- `load_constant_and_branch`: Loads a word into a data register and performs a short relative branch to another location.
- `loop_input_and_state`: Repeatedly updates values in a structure based on a loop, checks a bit in a global variable, and monitors a hardware register.
- `loop_word_mask_and_call`: The routine repeatedly reads a word from a global variable via a pointer, masks and shifts it, and loops while a condition on the high byte holds.
- `execute_complex_sequence`: Performs multiple operations including masking a word from a pointer, copying slices from a table, and executing a sequence of calls that includes a loop of seven double calls and a loop of four calls repeated five times.
- `repeat_calls_three_times`: Initializes a short counter to three and loops, calling another routine each iteration and decrementing the counter, then returns.
- `loop_table_copy`: Loops a number of times determined by an unreferenced short register, each iteration calling another routine that copies eight bytes from a global table indexed by the low byte of D1 to a buffer with a stride of 28.
- `rotate_and_branch`: The routine loads a small constant into a register, performs a longword rotation, and branches to another location.
- `swap_and_branch`: The routine loads a small constant into a register, swaps the contents of a register, and branches to another location.
- `repeat_table_copy_eight`: Loops eight times calling another routine that copies eight bytes from a global table indexed by the low byte of D1 to a buffer with a stride of 28.
- `conditional_rotated_table_copy`: Loops four times rotating an unreferenced word each iteration and conditionally calls another routine that copies eight bytes from a global table indexed by the low byte of D1 to a buffer with a stride of 28.
- `copy_table_slice`: The routine copies eight bytes from a global variable table, indexed by the low byte of D1, to a buffer pointed to by A0 with a stride of 28.
- `rotate_global_word_and_call_subroutines`: Rotates the low byte of a word stored at a memory location, updates that location with the new value, and then calls two subroutines using a pointer retrieved from a global structure.
- `fill_buffer_with_rotated_byte`: The routine fills a buffer with a rotating byte value, writing 27 bytes per row for three rows, rotating the byte each row.
- `write_nibble_pair_pattern`: The routine writes a rotating byte split into high and low nibbles to two locations in a buffer with a stride of 28 for 122 iterations.
- `repeat_seven_double_calls`: The routine initializes a short counter to seven and loops eight times, calling another routine twice per iteration and decrementing the counter until it reaches minus one.
- `decode_and_concatenate_memory`: Performs complex data manipulation involving decoding bitstreams, building 32-bit values by concatenating input with memory, and copying memory blocks based on conditional checks.
- `decode_and_copy_bytes`: The routine decodes a bitstream from a source buffer using a control table in a global variable and copies the decoded bytes to a destination buffer.
- `decode_bitstream_with_table`: The routine repeatedly shifts a byte value left and fetches bits from a byte pointer, using carry to propagate bits, for a fixed count of three iterations.
- `decode_bitstream_conditional`: The routine branches on a word flag.
- `concat_input_with_memory`: The routine builds a 32-bit value by concatenating the low 31 bits of an input register with successive bytes read from a pointer, iterating three times and advancing the pointer each iteration, then returns the constructed value.

### Captive (1990)(Mindscape)[cr CLS][a2].adf:fed_MapView#unpacked

- `initialize_jump_table`: Sets two values in a structure, conditionally calls a function pointer and recurses, and then populates a table with calculated addresses.
- `update_map_view_state`: Updates a view structure by checking a status field and performing conditional calculations on offsets and counters.
- `update_view_parameters_2`: The routine performs multiple conditional checks on values within a structure and updates various offsets and calculated values based on those checks.
- `store_structure_data_block`: Copies several words from one part of a structure to another part of the same structure using values returned from other routines.
- `store_returned_word`: Calls another routine and stores the returned word into a location within a structure.
- `update_view_parameters`: Updates multiple values within a large data structure based on conditional checks of existing offsets and performs arithmetic calculations involving division and multiplication.
- `update_map_view_structure`: Copies specific word values from one section of a data structure to another section.
- `write_constant_to_offset`: The routine writes a constant word to a fixed offset from a pointer.
- `calculate_map_view_offset`: Compares a value in a data structure against a constant and, if greater, calculates a new offset by multiplying a word from a data structure by eleven and adding ten.
- `divide_and_branch`: Writes a value to a global location, divides a register by eleven, and branches to another routine.
- `set_bits_in_buffer`: The routine checks if two input values are within specific bounds, then performs a loop to set specific bits in a buffer based on a calculated offset and bit position.
- `update_global_state`: The routine writes a longword and two bytes to specific offsets within a global structure and conditionally writes another byte based on a value at a third offset.
- `clear_on_bracket`: The routine checks the byte after a pointer for a left bracket character and, if found, writes zero to the longword at the pointer.
- `initialize_state_and_loop`: The routine writes several constants into fields of a global structure, repeatedly calls another routine until it returns zero three times, then calls another routine multiple times and writes additional constants into the structure.
- `mark_map_cells`: Iterates over a table of pointers in a global structure, writes a marker byte to entries that are zero, and stops after a fixed count.
- `fill_output_bytes`: Calls another routine repeatedly to obtain values and writes derived bytes to two output buffers in a loop.
- `scale_short_array`: Iterates 50 times over a short array located via a base pointer and applies a linear transform to each element, storing the result back in place.
- `transform_indexed_pairs`: Loops 100 times over a short pair array, reads an index byte from each entry, fetches a value from a global table using that index, and writes a transformed pair back using a linear formula.
- `generate_unique_values`: Computes a value from an input word using a global table, checks it against a list in a global structure, and writes unique values to an output buffer.
- `clear_table_and_fill_from_lookup`: The routine clears a block of memory at a fixed offset from a base pointer and then repeatedly builds a 32-bit value from two bytes taken from a global table using indices derived from input registers, checking the value against the cleared block and writing it through a pointer until a counter expires.
- `transform_d0_to_d1`: Multiplies d0 by 1509, adds 41, and stores the result in d1 after performing a 2-bit rotate right.
- `calculate_offset_index_2`: The routine iterates through a table of words until a value meets a condition, then performs bitwise operations on a global variable to calculate a value.
- `load_and_branch`: Loads the value thirty-two into a register and branches to another location.
- `zero_d0_and_branch`: Clears register d0 and performs a relative branch to another location.
- `calculate_offset_index`: The routine iterates through a table of words until a value meets a condition, then performs bitwise operations on a global variable to calculate and return a modified offset.
- `decrement_and_branch`: Loads a word from a structure, decrements it, and branches to another routine if the result is zero.
- `update_global_structure_2`: The routine loops a fixed number of times, subtracting values from two longwords stored in a structure and then writing the results back to that structure.
- `update_global_structure`: Writes two longwords and a word to a structure, sets a bit in a word based on another value, and updates several bytes including a pointer to a global variable.
- `decrement_and_branch_2`: Loads a word from a structure, decrements it, and branches to another location if the result is zero, repeating this process with a second decrement.
- `compare_map_status_value`: Compares a word at a specific memory location with a constant value.
- `update_map_state_and_coordinates`: Manages a state machine by checking a status word and updating it to different values based on specific conditions.
- `modify_word_conditionally_2`: The routine checks if a word at a specific offset in a global structure equals two and, if so, updates it to four.
- `update_global_state_2`: The routine checks if a word at a specific offset in a global structure equals two and, if so, updates it to six and calls another routine.
- `modify_word_conditionally`: Compares two words from a structure, and if they are not equal, subtracts one from a byte, increments a word, and clears the least significant bit of that word.
- `decrement_and_mask_word`: The routine subtracts two from a word at a specific offset relative to a pointer and then clears the least significant bit of that word.
- `write_word_to_offset`: The routine writes the word value two to a specific offset relative to a pointer.
- `update_structure_values_2`: Loads two words from a structure, calls another routine, and then stores the results along with a longword and a word into a different location in the structure.
- `update_structure_data`: Loads two words from a structure, calls another routine, and then stores the results into a different location in the structure.
- `rotate_coordinate_pair`: Loads two 16-bit values from a memory location, calls another routine, and then stores the results back to a different offset within the same structure using a bitwise rotation or swap logic.
- `subtract_structure_values`: Decrements a byte in a structure, then subtracts two longwords from two other longwords within a structure.
- `update_conditional_state`: Compares a word in a global structure to a constant and either writes a new word and two bytes or writes a word and a byte to that structure.
- `reset_structure_values`: Writes a word, a pointer to a global variable, and several bytes to a structure, conditionally updating one byte based on a value in the structure.
- `count_table_elements`: Compares a word from a global variable against a constant and branches if it is higher.
- `update_limit_value`: Compares a register with a value in a global variable and, if the register is higher, stores the register value into that global variable.
- `update_map_viewport_coordinates`: Calculates and updates viewport position and dimensions by decrementing counters and applying scaling factors from a coordinate table.
- `decrement_list_values`: Subtracts two longwords from two different locations in a structure and performs a loop using a counter register.
- `update_map_view_window`: Calculates and updates the viewport coordinates and dimensions by fetching values from a coordinate table and applying scaling factors.
- `decrement_word_and_branch`: Subtracts thirty-two from a word at a memory location relative to a base pointer and performs a short branch.
- `set_multiple_words`: The routine writes three specific word values to three different offsets relative to a pointer.
- `update_map_view_coordinates`: Calculates and updates view window offsets and dimensions based on a structure containing coordinate data and scaling factors.
- `transform_and_loop_2`: Swaps the high and low words of two registers, adds a constant to each, and calls another routine within a loop.
- `process_coordinate_differences`: Subtracts d0 from d2 and d1 from d3, setting bits 0 and 1 of d5 if the results are negative, then performs a division of the absolute values if d2 is greater than d3 and d2 is not zero, before branching to another routine.
- `clear_registers_and_branch`: Clears three registers and branches to another routine.
- `set_registers_and_branch`: Moves a word from one register to another, sets two registers to a constant value, and branches to another routine.
- `calculate_division_result`: Performs a division of the value in d2 by the value in d3, masks the result to 16 bits, and conditionally negates the quotient or a constant based on bits 0 and 1 of d5 before calling another routine.
- `transform_and_loop`: Calls another routine, moves two words into registers, swaps the high and low words of those registers, adds a constant to each, and branches to another location.
- `loop_other_routine`: Executes a loop that calls another routine while decrementing a counter.

### Captive (1990)(Mindscape)[cr CLS][a2].adf:case.pic#unpacked

- `initialize_globals_and_loop`: Initializes several global variables, copies blocks of words between global areas, and enters a loop that repeatedly calls other routines while a global integer remains non-negative.
- `wait_for_custom_bit`: The routine spins on a global variable until its least significant bit is cleared and then spins again until the bit is set, touching a custom hardware register.
- `fill_word_table_from_input`: The routine writes a sequence of word values derived from an input integer into a table located at a fixed offset, advancing the destination pointer each iteration and updating the input value by a constant amount.
- `copy_color_words`: The routine copies a fixed number of word entries from one area of a global variable to another area, touching the COLOR00 hardware register, and then returns.

### Captive (1990)(Mindscape)[cr CLS][a2].adf:mouse

- `wait_for_cia_bit`: The routine spins on a global variable until a specific bit is cleared, while checking a CIAA register.

### Captive (1990)(Mindscape)[cr CLS][a2].adf:A_CLASSIC_CRACK!!!

- `initialize_system_and_copy`: An entry point routine that touches the INTENA register and calls routines to copy a memory block and decode a color stream.
- `copy_memory_block`: Copies a block of four-byte words from a source pointer to a destination pointer for a fixed number of iterations.
- `decode_and_apply_color_stream`: Decodes a bitstream into a sequence of values and writes them to a table or memory area.
- `load_registers_and_branch`: Loads two registers with small constant values and then performs a relative branch.
- `store_and_transform_word`: The routine stores a word into a memory location using a combination of a register and a high byte from another register, then returns a bitwise shifted version of the loaded word.
- `process_encoded_stream`: Performs a loop that calls another routine and moves bytes from a source to a destination based on a counter.
- `process_input_recursively`: Performs a recursive loop that shifts a value right until it reaches zero, calling itself during the process.
- `branch_and_setup_counter`: Compares a register against small constants to determine a branch path, then loads a register with a constant and another with a value from a register before branching.
- `expand_encoded_stream`: Processes a stream of bytes, expanding specific sequences into repeated byte patterns and storing the results into a destination buffer.

### Captive (1990)(Mindscape)[cr CLS][a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
