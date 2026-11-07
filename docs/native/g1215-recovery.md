# Game 1215 (LegendsOfValour v1.0 0837): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01215). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (7051690 top-level bytes in 216 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 19 of 19 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 306 of 306 named (0 author symbols), 306 of 306 described (7 by rule, 285 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1024 of 1024 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 467313 bytes, 5 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2343 of 2359 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2317222 |
| code | 7886 |
| data | 4726538 |
| padding | 44 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 970 | 66 |
| AMIGA2 | 2 | 233 | 8 |
| charact | 2 | 233 | 8 |
| textfile | 1 | 0 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 266 | 211 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |
| ACO_SAT.PCX (raw code file, load address unknown) | 1 | 0 | 0 |
| GHO_BEA.PCX (raw code file, load address unknown) | 1 | 0 | 0 |
| GOB_CYC.PCX (raw code file, load address unknown) | 1 | 0 | 0 |
| JESTGIRL.PCX (raw code file, load address unknown) | 1 | 0 | 0 |
| MAPU0009.DAT (raw code file, load address unknown) | 1 | 0 | 0 |
| MONK_FE.PCX (raw code file, load address unknown) | 1 | 0 | 0 |
| VAMPWOLF.PCX (raw code file, load address unknown) | 1 | 0 | 0 |
| effects (raw code file, load address unknown) | 1 | 207 | 2 |
| fpanels.pcx (raw code file, load address unknown) | 1 | 0 | 0 |
| music (raw code file, load address unknown) | 1 | 440 | 3 |
| ospanels.pcx (raw code file, load address unknown) | 1 | 0 | 0 |
| text.dat (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- LegendsOfValour_v1.0_0837/LegendsOfValour_v1.0_0837.hdf: hard-disk image (AmigaDOS OFS, 7968 blocks): its filesystem parses and all 211 files in it are byte-identical to the extracted tree LegendsOfValour_v1.0_0837_hdf, which is recovered file by file; no block outside the filesystem holds data
- 4 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 189 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- LegendsOfValour_v1.0_0837/LegendsOfValour_v1.0_0837_hdf/LegendsOfValour/data/ACO_SAT.PCX: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- LegendsOfValour_v1.0_0837/LegendsOfValour_v1.0_0837_hdf/LegendsOfValour/data/GHO_BEA.PCX: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- LegendsOfValour_v1.0_0837/LegendsOfValour_v1.0_0837_hdf/LegendsOfValour/data/GOB_CYC.PCX: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- LegendsOfValour_v1.0_0837/LegendsOfValour_v1.0_0837_hdf/LegendsOfValour/data/JESTGIRL.PCX: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- LegendsOfValour_v1.0_0837/LegendsOfValour_v1.0_0837_hdf/LegendsOfValour/data/MAPU0009.DAT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- LegendsOfValour_v1.0_0837/LegendsOfValour_v1.0_0837_hdf/LegendsOfValour/data/MONK_FE.PCX: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- LegendsOfValour_v1.0_0837/LegendsOfValour_v1.0_0837_hdf/LegendsOfValour/data/VAMPWOLF.PCX: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- LegendsOfValour_v1.0_0837/LegendsOfValour_v1.0_0837_hdf/LegendsOfValour/data/effects: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- LegendsOfValour_v1.0_0837/LegendsOfValour_v1.0_0837_hdf/LegendsOfValour/data/fpanels.pcx: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- LegendsOfValour_v1.0_0837/LegendsOfValour_v1.0_0837_hdf/LegendsOfValour/data/music: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- LegendsOfValour_v1.0_0837/LegendsOfValour_v1.0_0837_hdf/LegendsOfValour/data/ospanels.pcx: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- LegendsOfValour_v1.0_0837/LegendsOfValour_v1.0_0837_hdf/LegendsOfValour/data/text.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- LegendsOfValour_v1.0_0837/LegendsOfValour_v1.0_0837_hdf/LegendsOfValour/data/AMIGA2: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- LegendsOfValour_v1.0_0837/LegendsOfValour_v1.0_0837_hdf/LegendsOfValour/data/charact: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 12 never-loaded region(s), 467313 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `execute_indirect_calls`: This routine stores the current routine address and a register value into global variables before performing three indirect calls through pointers located at specific offsets from a register and a global variable.
- `update_jump_table`: Stores a value derived from a global variable into two different offsets within a jump table and then executes an indirect call.
- `execute_indirect_jump`: Performs an indirect jump using a value from a jump table.
- `set_register_and_return_offset`: If a register is zero, it copies the value of another register into it, then returns that register's value decremented by eight.
- `mask_global_variable`: Returns a global variable after applying a bitwise AND mask.
- `read_interrupt_request_register`: Reads the value of the interrupt request enable register.
- `store_interrupt_request_to_global`: Stores a value from the interrupt request register into a global variable.
- `return_zero_constant`: The routine immediately returns zero without performing any operations or modifying data.
- `set_flag_in_struct`: The routine receives a pointer in a register and sets bit 1 in the byte at offset 14 of the pointed-to structure.
- `return_without_effect_2`: The routine consists of a single return instruction and performs no operations.
- `store_register_to_global`: The routine copies the value from D0 into a global variable.
- `load_global_and_jump`: Loads a global variable into a register and then performs an indirect jump.
- `copy_byte_with_offset`: Copies a byte from an offset of a pointer into another memory location determined by a different pointer and an offset.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_global_constant`: The routine compares a global variable to a specific constant value.
- `initialize_pointer_chain`: The routine writes a sentinel value into a pointer and stores the address of the next entry into three fields in two different structures.
- `copy_nested_pointers`: Copies 32-bit values from pointers referenced at offsets four and eight of a source structure into fields of a destination structure.
- `negate_nonzero_flag`: The routine receives a value in a register and returns zero if the value is non-zero, and returns a negative one if the value is zero.
- `manipulate_bit_rotated_value`: This routine performs bitwise rotations on a value and compares the result against a global variable before updating memory or the stack.
- `return_empty`: An empty routine that performs no operations before returning.
- `manage_interrupt_state`: This routine modifies a global variable and interacts with the interrupt enable register.
- `calculate_table_offset`: Reads values from a global table and performs arithmetic operations including division, multiplication, and addition.
- `process_status_byte`: This routine checks bits in a structure, performs conditional assignments to a byte in another structure, and executes indirect calls based on those conditions.
- `negate_short_toggle_bit`: Negates a short value at a specific offset in a structure and toggles the high bit of a byte at an offset in another structure.
- `set_flag_from_bit`: Reads a bit from a global variable using an index derived from a byte in another memory location.
- `return_empty_routine`: This routine is empty and performs no operations before returning to its caller.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `clear_bit_and_increment`: This routine tests a bit in a global variable using a mask derived from a structure, clears that bit, and increments a value in a structure.
- `return_without_effect`: The routine consists of two instructions that perform no computation and do nothing beyond returning to its caller, as shown by the decompiled body being a bare return that calls only an empty routine.
- `do_nothing_stub`: The routine executes five instructions but performs no observable work, ending without modifying memory, registers, or calling another routine.
- `return_without_effect_3`: Routine consisting only of a return instruction with no operations, so it does nothing when entered.
- `clear_offset_and_save_register`: Stores zero to a word-sized location at an offset from a pointer in a register, then saves the value of another register into a global variable before returning.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `execute_conditional_call`: This routine compares a value against a constant and performs an indirect call through a pointer stored in a structure.
- `assign_function_pointer`: This routine stores the address of itself into a global variable.
- `copy_string_to_buffer`: Calculates the length of a null-terminated string and copies it into a buffer retrieved via a pointer in a global variable.
- `initialize_expansion_structures`: Uses memory allocation and queueing functions to build descriptor structures, sets flags, and stores global pointers into RAM.
- `call_library_function`: Calls a function from a library and then calls a second function using an offset from the result of the first call.
- `wait_for_message`: This routine interacts with the operating system to find a task, wait for a port, and retrieve or return messages.
- `process_multiple_calls`: This routine consists of nine instructions and calls several other routines including one that waits for a message.
- `execute_callback_sequence`: A five instruction routine that calls several other routines including one that waits for a message.
- `process_nested_results`: A routine consisting of 96 instructions that calls another routine twice and compares the results while interacting with the operating system to wait for a message.
- `wait_for_os_message_2`: This routine is 66 instructions long, is not reached by any known caller (its code was recovered from a gap that decodes as code), and makes one Amiga OS library call at library offset -198 that is named wait_for_os_message, plus a call to nop_routine, which performs no operations.
- `wait_for_os_message`: Calls a routine to retrieve a message from the operating system.
- `execute_nested_calls_2`: A two instruction routine that calls another routine.
- `execute_nested_calls`: This routine consists of thirteen instructions and calls several routines, including one that waits for a message.
- `parse_path_and_wait`: A fifteen instruction routine that calls several routines including one to parse a file path and one to wait for a message.
- `process_message_callback`: A fourteen instruction routine that calls a routine to wait for a message and another to validate and execute a callback.
- `invoke_message_wait_3`: A seven instruction routine that calls another routine to wait for a message.
- `invoke_message_wait_2`: This routine consists of seven instructions and calls two routines, one of which waits for a message.
- `invoke_message_wait`: Calls a routine to retrieve a message from a port.
- `return_immediately`: This routine performs no operations and immediately returns to its caller.
- `return_from_subroutine_2`: Executes a single instruction to return control to the calling routine.
- `return_void`: The routine is empty and performs no operations before returning.
- `rotate_string_and_store_length`: The routine rotates a null-terminated byte string one position to the right by shifting each byte forward and writes the original first byte into the last position, then stores the length of the string minus one as a byte just before the string in memory.
- `process_nested_results_2`: Parses a file path and compares nested results using a boolean check.
- `validate_and_execute_callback`: This routine checks if a register is non-zero and compares a value in a register against a field in a structure.
- `parse_file_path`: This routine manipulates string data to construct or modify a file path by searching for directory separators and colons.
- `compare_nested_results`: Calls another routine twice and returns its result based on a boolean check.
- `compare_two_strings`: The routine compares the bytes pointed to by two pointers until a zero byte is found, returning early if the bytes differ, otherwise it falls through after the terminating zero.
- `empty_routine`: This routine contains no executable instructions and performs no operations.
- `nop_routine`: This routine is empty and performs no operations.
- `patch_memory_constants`: Copies a string to a buffer and patches specific memory locations with new constants.
- `update_memory_patterns`: The routine checks two specific locations within a table for a constant value and, if matched, overwrites them and subsequent adjacent memory locations with new constant values.
- `patch_data_constants`: Checks two specific memory locations for a constant value and, if matched, overwrites them and subsequent adjacent memory offsets with new constant values.

### AMIGA2

- `load_hunk_data`: Parses a custom data stream containing AmigaDOS-style hunk type codes to allocate and relocate memory segments.
- `decrement_until_zero`: Decrements a word value in a register until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decompress_bit_stream`: Performs bitwise manipulations and memory shifts to expand data from a stream.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: Performs complex bitwise manipulations and shifts on data from a table and registers, while modifying a color register.
- `execute_trap_instruction`: Triggers a software interrupt via a trap instruction.

### charact

- `parse_hunk_data_stream`: Parses a data stream containing hunk type codes to allocate memory and relocate data.
- `decrement_loop_counter`: Decrements a value in a register within a loop until it reaches zero.
- `return_immediately_2`: This routine is empty and returns immediately to the caller.
- `decompress_bit_stream_2`: Processes a bit stream by performing complex shifts and byte-wise memory copies.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `extract_bitstream_data`: Processes a bitstream by extracting bits based on a variable length and updating a value through bitwise shifts and rotations.
- `decompress_bit_stream_3`: Performs bitwise manipulations and shifts to expand data from a stream while modifying a color register.
- `execute_trap_instruction_2`: Executes a software trap instruction.

### textfile

- `manipulate_bitfield_data`: This routine performs bitwise operations on memory locations using bit shifts and masks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `empty_routine_2`: This routine is empty and contains no executable instructions.
- `empty_routine_3`: This routine is empty and contains no valid instructions.
- `undefined_instruction_trap`: This routine contains no valid instructions and terminates control flow immediately upon entry.
- `trap_invalid_instruction`: The decompiled body is only a bad-instruction marker: the disassembler could not decode the single instruction at this entry point, so control flow is truncated there.
- `empty_routine_4`: This routine contains no executable instructions.
- `execute_empty_call`: This routine performs no operations and immediately returns to the caller after calling another empty routine.
- `nop_padding_byte`: This routine is empty and consists of a single byte of data that does not form a valid instruction.
- `empty_routine_5`: The routine contains no executable instructions.
- `nop_padding_sequence`: This routine is empty and consists of a single instruction that does not perform any operation.
- `mark_empty_routine`: The routine contains no instructions and performs no operation.
- `null_routine`: The routine is empty and contains no executable instructions.
- `null_routine_2`: The routine is empty and contains no valid instructions.
- `nop_padding_instruction`: This routine is empty and consists of a single non-operational byte used for alignment or as a gap between executable code blocks.
- `nop_instruction`: This routine consists of a single instruction that performs no operation.
- `mark_empty_stub`: The routine contains no executable instructions; it is an empty stub with no code to describe.
- `execute_empty_call_2`: This routine performs no operations and immediately returns to the caller after calling an empty routine.
- `execute_null_call`: This routine performs no operations and returns immediately to the caller.
- `return_immediately_4`: This is an empty routine that performs no operations and returns to its caller.
- `trap_on_bad_instruction`: The decompiler could not decode the bytes at this entry point and emitted only a truncation marker, so the routine's real behaviour is not recoverable from this output.
- `empty_data_word`: The routine contains no executable instructions; it is a single data word (0) that the disassembler flags as bad instruction data.
- `terminate_execution`: The routine contains no valid instructions and terminates the current execution flow.
- `call_empty_subroutine`: This routine calls another routine that performs no operations and immediately returns.
- `nop_padding_instruction_2`: The routine contains a single instruction that results in an immediate termination of the instruction stream during analysis.
- `return_immediately_5`: This routine performs no operations and returns immediately to its caller.
- `trap_on_undecodable_bytes`: The routine is a single instruction, and the decompiler could not decode it: its bytes are data or an invalid opcode, so the decompiler truncated the function there.
- `flag_unused_data`: This routine contains no executable instructions; the bytes at this location are unreferenced data that only appears to be code because nothing calls it.
- `terminate_execution_2`: The routine contains no valid instructions and causes the processor to stop.
- `nop_instruction_2`: The routine consists of a single instruction that performs no operation.
- `empty_routine_6`: This routine contains no valid instructions and performs no operations.
- `mark_empty_stub_2`: The routine contains no executable instructions; it is a completely empty stub that performs no operations.
- `empty_routine_13`: This routine is empty, containing no instructions and performing no operations.
- `mark_empty_routine_2`: The routine contains no executable instructions; it is empty.
- `nop_placeholder`: This routine contains a single instruction that does not perform any operation.
- `null_operation_routine`: The routine contains no valid executable instructions.
- `empty_routine_7`: This routine contains no valid instructions.
- `mark_empty_gap`: This routine contains no executable instructions; it is a gap that only decodes as code because of misaligned disassembly.
- `empty_routine_14`: The routine contains no executable instructions; it is an empty routine with no code to describe.
- `nop_padding_byte_2`: This routine consists of a single byte of data that does not form a valid instruction.
- `execute_invalid_instruction`: The routine contains a single instruction that is not recognized as valid code, causing the execution flow to terminate immediately.
- `copy_byte_via_pointer`: Reads a byte from a global variable and stores it back to another global variable through a pointer.
- `empty_routine_8`: The routine contains no executable instructions and performs no operations.
- `terminate_execution_3`: The routine contains no valid instructions and stops processing.
- `empty_routine_15`: The routine contains no executable instructions; the referenced bytes are treated as data rather than code, so the routine performs no operation.
- `decode_invalid_instruction`: A lone undecodable instruction sitting in a gap that the tool decodes as code, so execution never legitimately reaches it.
- `mark_empty_routine_3`: The routine contains no executable instructions; it is effectively empty.
- `mark_empty_routine_4`: The routine contains no executable instructions; it is empty and performs no operations on any data.
- `mark_empty_routine_5`: The routine contains no executable instructions; it is a single empty routine with no data manipulation, no library calls, and no hardware register access.
- `empty_data_segment`: The routine contains no executable instructions, only a single data word that the disassembler could not decode as valid 68000 code.
- `is_empty_routine`: The routine contains no executable instructions; it is an empty routine with no code to execute.
- `empty_bad_data_block`: The routine contains no valid instructions; the bytes at its location are not executable code but arbitrary data.
- `nop_instruction_placeholder`: This routine consists of a single instruction that does not alter the processor state or data flow.
- `empty_routine_16`: The routine is empty; it contains no instructions and performs no operations on any data.
- `empty_routine_17`: The routine does nothing; it contains no code that executes.
- `encode_noncode_data`: The code at this location is not executable instructions but data, so it does not perform any operation on any data.
- `execute_null_call_2`: This routine performs no operations and immediately returns to the caller after calling a routine that contains no valid instructions.
- `decode_single_instruction`: The routine contains a single instruction whose meaning cannot be reliably determined from the surrounding bytes.
- `empty_data_block`: The routine contains no executable instructions; it holds only non-code data.
- `non_code_data`: The routine contains no executable instructions; it holds only non-code data.
- `nop_padding_byte_3`: The routine consists of a single byte of data that does not correspond to a valid instruction.
- `flag_invalid_code_block`: The routine contains no valid instructions; the bytes at this location are data rather than executable code, so there is nothing for the processor to run here.
- `nop_instruction_3`: The routine contains a single instruction that does not perform any operation.
- `empty_routine_9`: This routine contains no valid executable instructions.
- `contains_baddata_word`: The routine contains a single bad-instruction data word with no decodable code, so it performs no operations and has no body.
- `nop_padding_sequence_7`: This routine is empty and consists of a single instruction that does not perform any functional operation.
- `mark_empty_region`: The routine contains no executable instructions; the disassembler could not identify any valid code at this location, so there is nothing for the routine to do or any data to touch.
- `nop_instruction_4`: The routine contains a single instruction that does not perform a functional operation.
- `nop_padding_gap`: This routine is empty and consists of a single instruction that serves as padding within a code gap.
- `halt_baddata`: The routine contains no executable instructions; it holds only data that the disassembler could not decode as valid 68000 code.
- `mark_routine_empty`: The routine contains no executable instructions; it is empty.
- `misaligned_data_gap`: This routine is not real code; it is a gap of unreferenced bytes that the disassembler misinterprets as instructions.
- `mark_empty_routine_6`: The routine contains no executable instructions; it is an empty routine that does nothing.
- `halt_baddata_2`: This routine contains no executable instructions; the entry point holds only non-code data that Ghidra flags as bad instruction data.
- `copy_byte_in_place`: Reads a byte from a global variable and stores it back into the same global variable through a pointer.
- `abort_execution`: The routine contains no valid instructions and terminates the current process.
- `halt_baddata_3`: The routine contains no executable instructions; it holds only data that the disassembler could not decode as valid 68000 code, so it performs no operation.
- `mark_empty_stub_3`: The routine contains no executable instructions; it is an empty stub that performs no operations.
- `nop_padding_sequence_2`: This routine is empty and consists of a single instruction that serves as padding within the executable segment.
- `halt_baddata_4`: The routine contains no executable instructions; it holds only data that the disassembler flagged as invalid, so it performs no operations on any data.
- `terminate_execution_4`: This routine contains no valid instructions and causes the processor to stop.
- `halt_baddata_5`: The routine contains no executable instructions; it holds only data that disassembly tools cannot decode as valid 68000 code.
- `empty_routine_18`: This routine contains no executable instructions; it is an empty routine.
- `undefined_instruction_trap_2`: The routine contains no valid instructions and triggers a processor exception due to invalid opcode data.
- `execute_invalid_instruction_2`: The routine contains no valid instructions and terminates execution immediately upon entry.
- `decode_gap_instruction`: The routine is a single instruction that decodes from stray data in an unreached gap, so it does not correspond to any real behaviour.
- `mark_empty_routine_7`: The routine contains no executable instructions; it is an empty routine.
- `mark_empty_routine_8`: The routine contains no executable instructions; it is a single empty routine with no code, no library calls, no hardware register access, and no data manipulation.
- `nop_padding_gap_7`: This routine is empty and consists of a single instruction that serves as a gap between valid code segments.
- `halt_on_bad_instruction`: The routine consists of a single instruction that the decompiler could not decode as valid 68000 code, so control flow is truncated at that point.
- `execute_single_instruction_2`: The routine contains a single instruction and no further code, so it performs exactly one operation and then ends.
- `is_empty_routine_2`: The routine contains no executable instructions; it is an empty routine with no data operations or calls.
- `halt_baddata_6`: The routine contains no executable instructions, only bad instruction data that terminates control flow.
- `empty_routine_10`: The routine contains no valid instructions.
- `nop_padding_sequence_3`: This routine is empty and consists of a single non-instructional byte used for data alignment or padding within the executable segment.
- `nop_instruction_5`: This routine consists of a single instruction that does not perform any operation.
- `store_unused_data`: The routine contains no executable instructions; it holds only data that the disassembler cannot decode as valid 68000 code.
- `misaligned_gap_bytes`: This routine is not real code; it is a gap of unreferenced bytes that the disassembler misinterprets as instructions.
- `mark_empty_segment`: The routine contains no executable instructions; it is a data-only or empty segment with no operations performed on any data.
- `nop_placeholder_2`: The routine contains a single instruction that does not perform any meaningful operation or data manipulation.
- `empty_byte_gap`: This routine contains no executable instructions; it is a one-byte gap that decodes as code but performs no operation.
- `null_routine_3`: This routine is empty and contains no valid executable instructions.
- `nop_padding_gap_8`: This routine is empty and consists of a single byte of non-executable data acting as a gap between valid instructions.
- `decode_bad_data`: The routine contains a single instruction that is not valid 68000 code, so it cannot be decoded as a meaningful operation.
- `nop_padding_slot`: This routine is empty and consists of a single instruction that does not execute any functional logic.
- `store_bad_data`: The routine contains no executable instructions; it holds only data bytes that the disassembler could not decode as valid 68000 code.
- `decode_bad_instruction`: Decodes a single instruction that is invalid, then stops execution because the processor encountered bad instruction data.
- `flag_bad_data`: A single instruction that is not valid 68000 machine code, so it cannot be decoded as an executable routine.
- `null_operation_routine_2`: This routine contains no valid instructions and performs no actions.
- `nop_padding_sequence_4`: This routine is empty and consists of non-executable data or padding within the binary segment.
- `halt_baddata_7`: The routine contains only unreachable instruction data with no executable code, so it performs no operation.
- `terminate_execution_5`: The routine contains no valid instructions and results in immediate execution failure.
- `nop_instruction_placeholder_2`: The routine contains a single instruction that does not perform any operation or change the state of the processor.
- `abort_execution_2`: The routine contains no valid instructions and terminates execution immediately.
- `nop_padding_sequence_5`: This routine is empty and consists of a single non-instruction byte that acts as padding between executable blocks.
- `empty_routine_19`: The routine contains no executable instructions; it is an empty routine that performs no operations.
- `nop_padding_gap_9`: This routine is an empty gap in the executable containing no valid instructions.
- `halt_baddata_8`: The routine contains no executable instructions; the disassembly at this location holds only non-code data, so the routine performs no operation.
- `nop_padding_sequence_6`: This routine is empty and consists of a single instruction that serves as a gap in the executable code.
- `store_empty_routine`: This routine contains no executable instructions; it is a completely empty routine with no code and no data to describe.
- `halt_baddata_9`: The routine contains no executable instructions; it holds only data that the disassembler could not decode as valid code.
- `dump_inactive_bytes`: A routine containing only non-executable bytes that the disassembler cannot interpret as valid instructions, so no real operations are performed.
- `halt_execution`: This routine contains no valid instructions and effectively terminates the current execution flow.
- `halt_invalid_instruction`: The routine consists of a single instruction that decodes as an invalid or unimplemented opcode, so it performs no defined operation.
- `store_byte_from_arg`: Reads a byte from the data referenced by its single argument and stores that byte into a global variable.
- `jump_via_byte_pointer`: Fetches a byte from the current program counter and branches to the address stored in that byte.
- `mark_empty_stub_4`: The routine contains no executable instructions; it is an empty stub that performs no operation.
- `nop_padding_gap_10`: This routine is empty as it consists of a single instruction gap within the executable segment.
- `halt_baddata_10`: The routine contains no valid instructions, only data that does not decode as executable code.
- `misdecode_data_byte`: This is not real code; it is a single data byte that Ghidra misinterprets as an instruction.
- `halt_baddata_11`: The routine contains only a single instruction that is invalid or non-executable data, so it performs no operation and returns immediately.
- `nop_padding_gap_2`: This routine is an empty execution gap consisting of a single non-functional instruction used for memory alignment.
- `empty_routine_20`: The routine contains no executable instructions; it is effectively empty.
- `hit_bad_instruction`: The routine begins by executing an instruction that the disassembler cannot decode as valid 68000 code, so the decompiler flags the following bytes as bad instruction data and stops control flow.
- `flag_bad_instruction`: A single instruction that is not valid 68000 machine code, so the disassembler flags it as bad instruction data and control flow is truncated here.
- `mark_empty_routine_9`: The routine contains no executable instructions; it is an empty routine with no behaviour.
- `halt_on_bad_data`: The routine consists of a single instruction that the disassembler could not decode, so Ghidra emitted a bad-data warning and truncated the function there.
- `halt_baddata_12`: The routine contains a single instruction that is invalid or non-executable data, so the disassembler cannot decode it into a recognizable operation.
- `mark_empty_routine_10`: This routine contains no executable instructions; it is an empty routine with no code to execute.
- `flag_noncode_region`: The routine contains no executable instructions; the region at this address holds non-code data rather than code.
- `empty_routine_21`: The routine contains no executable instructions; it is empty and performs no operations.
- `nop_padding_block`: This routine is empty and consists of a single non-instruction byte acting as padding within the executable segment.
- `undefined_instruction_trap_3`: This routine contains no valid instructions and triggers an execution error.
- `nop_instruction_6`: The routine contains a single instruction that does not perform any operation or modify any state.
- `misaligned_data_gap_2`: This routine is not real code; it is a gap of unreferenced data that the disassembler misinterprets as instructions.
- `halt_baddata_13`: The routine contains no executable instructions; it holds only data bytes that the disassembler could not decode as valid 68000 code.
- `noop_routine`: This routine is empty and performs no operations.
- `do_nothing`: This routine is empty; it contains no instructions and performs no operations on any data.
- `halt_baddata_14`: The routine contains no valid instructions, only data that the disassembler could not decode.
- `decode_gap_data`: The routine consists of a single instruction that is not part of any reachable code path; it is unreferenced gap data that only decodes as an instruction by coincidence.
- `recover_bad_data`: The instruction data at this location does not decode as valid 68000 code, so no operation can be recovered.
- `empty_routine_22`: This routine is empty and contains no instructions, so its purpose cannot be inferred from the code.
- `decode_bad_data_2`: This routine is a gap of unrecoverable instruction data that Ghidra fails to decode as valid 68000 code.
- `halt_baddata_15`: The routine contains no executable instructions; it holds only data that the disassembler could not decode.
- `mark_empty_routine_11`: The routine contains no executable instructions; it is a single empty entry point that performs no operations on any data.
- `decode_gap_data_2`: This routine is a gap of unreferenced data that Ghidra misinterprets as code, so it does not correspond to a real routine.
- `halt_baddata_16`: The routine contains no valid instructions; the bytes at this location are not executable code.
- `store_next_table_byte`: Reads a byte from a table and writes the byte at the following index into a global variable.
- `nop_padding_gap_3`: This routine is empty and consists of a single non-instructional byte used as padding within the executable segment.
- `nop_placeholder_3`: The routine is empty and contains no valid executable instructions.
- `copy_byte_through_pointer`: Reads a single byte from a memory location pointed to by a register and stores it through another pointer.
- `nop_padding_block_2`: This routine is empty and consists of non-executable data or padding within the code segment.
- `halt_baddata_block`: The routine contains no executable instructions; it holds only bad instruction data that terminates control flow.
- `halt_baddata_17`: The routine contains no executable instructions; it holds only data that disassembly tools misidentified as code, so it performs no operations.
- `empty_routine_23`: The routine contains no executable instructions; it is a single empty entry point that performs no operations.
- `nop_padding_byte_4`: This routine consists of a single instruction that serves as an empty gap in the executable.
- `halt_baddata_18`: This routine contains no executable instructions; it holds only invalid data that the disassembler flags as a bad instruction.
- `copy_byte_between_globals`: Reads a byte from a global variable and writes it to another global variable.
- `nop_padding_byte_5`: This routine consists of a single non-executable byte used as padding between functional code blocks.
- `empty_routine_11`: The routine contains no valid instructions and performs no operations.
- `nop_padding_filler`: This routine is empty and consists of a single non-operational instruction used for memory alignment or padding.
- `halt_execution_2`: The routine contains no valid instructions and terminates immediately upon entry.
- `nop_padding_byte_6`: This routine consists of a single instruction that acts as a padding byte within the executable segment.
- `trigger_illegal_instruction`: It executes a single illegal instruction, which triggers the illegal-instruction handler.
- `nop_padding_gap_4`: This routine is empty and consists of a single non-functional byte used as alignment padding between executable sections.
- `terminate_execution_6`: The routine contains no valid instructions and terminates the current execution flow immediately.
- `empty_data_segment_2`: The routine contains no executable instructions, only a data segment that Ghira flags as bad instruction data, so it performs no operations.
- `decode_invalid_byte`: The routine consists of a single instruction that is not valid 68000 code; the surrounding bytes are data rather than executable instructions.
- `misidentified_data_block`: This routine is not real code; it is a block of data that the decompiler misread as instructions, so it performs no operations.
- `mark_empty_routine_12`: The routine contains no executable instructions; it is an empty routine.
- `nop_padding_gap_5`: This routine is empty and consists of non-executable data used as padding between functional code blocks.
- `trap_undecodable_instruction`: The routine consists of a single instruction that the disassembler could not decode; Ghidra marks it as bad instruction data and truncates the function there, so no operation, library call, hardware register access, or caller-visible effect can be attributed to it.
- `nop_padding_byte_7`: This routine consists of a single instruction that serves as a data gap or padding within the executable segment.
- `mark_empty_routine_13`: The routine contains no executable instructions; it is empty.
- `empty_data_region`: The routine contains no executable instructions; it holds only data bytes that a disassembler misinterprets as code, so there is no behaviour to describe.
- `nop_padding_gap_6`: This routine is empty and consists of a single instruction that serves as a gap between functional code blocks.
- `halt_baddata_19`: The routine contains no executable instructions; it holds only bad instruction data and immediately terminates execution.
- `halt_baddata_20`: The routine contains no valid instructions; the code segment here holds only non-executable data rather than any code.
- `misaligned_data_bytes`: This routine is not real code; the bytes at this location are data that was misinterpreted as instructions.
- `halt_baddata_21`: The routine consists of a single instruction that appears to be data rather than executable code, so no meaningful operation can be recovered from it.
- `trap_invalid_instruction_stub`: The routine consists of a single instruction that Ghidra could not decode, so the decompiler stopped the control flow at that point.
- `nop_padding_gap_11`: This routine is empty and consists of a single non-instruction byte used as padding between code segments.
- `empty_stub_routine`: The routine contains no executable instructions; it is an empty stub.
- `call_decrement_routine`: Calls another routine and returns its result.
- `call_external_routine`: Calls a routine located at a fixed address in memory and returns its result.
- `call_decrement_routine_2`: Executes another routine and returns its result.
- `execute_indirect_call`: Calls a routine through a pointer stored in memory.
- `decrement_offset_word`: Decrements a 16-bit value located at a fixed positive offset from a base pointer register.
- `execute_indirect_call_2`: Calls a routine through a pointer stored in memory.
- `decrement_memory_word`: Calls another routine to decrement a 16-bit value at a fixed offset from a base pointer and returns a zero value.
- `return_zero_constant_2`: This routine is empty and returns a zero value to the caller.
- `increment_word_at_offset`: The routine increments a 16-bit value located at a fixed positive offset from a base pointer register in place and then returns.
- `decrement_word_at_offset`: The routine decrements a 16-bit value located at a fixed positive offset from a base pointer register and then returns a zero value.
- `store_single_data_byte`: The routine contains no executable instructions, only a single piece of data.
- `null_operation_padding`: The routine is empty and consists of non-executable data acting as a gap between code segments.
- `flag_bad_data_region`: The routine contains no valid instructions; the bytes at this location are data rather than executable code, so the decompiler cannot decode any operation here.
- `mark_empty_routine_14`: The routine contains no executable instructions; it is a completely empty routine with no code, no data references, and no library calls.
- `empty_routine_12`: This routine performs no operations and immediately returns to the caller.
- `return_from_subroutine_4`: Performs a single return instruction to exit the routine.
- `empty_return_routine`: The routine performs no operations other than returning to the caller.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a5, then returns.
- `execute_single_instruction`: This routine consists of a single instruction.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `shift_memory_block`: This routine moves a block of data by copying values from a source range to a destination range using a nested loop structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_no_operation`: This routine is empty and performs no actions before returning.
- `empty_routine_24`: The routine contains no instructions and performs no operations; it returns immediately without touching any data or registers.
- `empty_routine_25`: The routine contains no instructions and performs no operations; it returns immediately without reading or writing any data.
- `return_immediately_3`: This is an empty routine that performs no operations and returns to its caller immediately.
- `return_from_subroutine_3`: This routine is empty and performs no operations before returning to the caller.
- `return_no_op`: The routine consists of a single instruction that returns immediately without performing any work, so it has no side effects.

### effects (raw code file, load address unknown)

- `initialize_global_values`: Sets several global variables and memory locations to specific constant values.
- `wait_counter_decrement`: Waits by decrementing a local counter from a fixed value until it reaches negative one, then returns.

### music (raw code file, load address unknown)

- `initialize_audio_hardware`: Configures the audio hardware and initiates the decoding of a table stream.
- `configure_audio_hardware`: Interacts with audio, DMA, and CIA registers and decodes a table stream.
- `decode_table_stream`: Walks through a byte table using a base pointer and an index, incrementing the index and skipping over entries whose high bit is set, then stores a decoded byte and the next index into a per-channel structure.
