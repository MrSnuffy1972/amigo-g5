# Game 1204 (jetstrike (demo) (1993)(rasputin)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01204). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 75 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 193 of 193 named (59 author symbols), 193 of 193 described (16 by rule, 174 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 163 of 163 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2720 of 2720 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 339280 |
| code | 7970 |
| data | 259972 |
| padding | 19530 |
| unused | 274432 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| ppmore | 2 | 209 | 8 |
| EndCLI | 2 | 117 | 2 |
| mathtrans.library | 17 | 830 | 97 |
| mathieeedoubbas.library | 3 | 632 | 60 |
| Jetstrike (demo) (1993)(Rasputin).adf:Jetstrike_Demo | 8 | 782 | 22 |
| Jetstrike (demo) (1993)(Rasputin).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |

## Cases handled

- jetstrike (demo) (1993)(rasputin)/Jetstrike (demo) (1993)(Rasputin).adf:Engine1.smp: high-entropy data file (7.64 bits/byte), classified as packed data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- jetstrike (demo) (1993)(rasputin)/Jetstrike (demo) (1993)(Rasputin).adf:c/ppmore: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_register_and_branch`: Moves the contents of d1 into d0 and then branches to another location.

### ppmore

- `process_amiga_hunk_relocations`: Allocates memory and parses a list of AmigaDOS hunk structures, performing relocations by adding offsets to memory locations based on specific hunk type codes.
- `increment_counter_if_nonzero`: Tests the value in d5 and returns immediately if it is zero, otherwise increments a counter in d6, sets d5 to one, and returns.
- `process_linked_list_nodes`: Walks a linked list by accessing a pointer from a structure in a5, performs recursive calls to itself, and updates values in a destination buffer based on offsets found in a3.
- `tiny_ori`: Small routine of 1 instruction that merges 220 into d0.
- `decompress_back_reference_stream`: Decodes a packed byte stream by reading control values that either copy literal bytes or repeat earlier output with a length and offset from the stream.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_cycle_counter`: Decrements a byte counter in d7 for each iteration of a loop controlled by d0, resetting it to a space character whenever it reaches zero, and returns.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### EndCLI

- `process_max_value_and_sync_buffer`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `transfer_register_to_d0`: Moves a 32-bit value from register d1 to register d0 and then branches to another routine.

### mathtrans.library

- `triggers_processor_exception`: Executes an illegal instruction to trigger a processor exception.
- `store_pointer_to_global`: As library code, it stores the value in a0 into a global variable at offset 376.
- `unlink_list_node`: As library code, it checks a 16-bit value at offset 32 of a structure in a6; if zero, it unlinks a node by updating pointers in a linked list and calls a routine via a pointer from a global variable, otherwise it sets bit 3 of a byte at offset 30 of the structure.
- `set_library_status_bit`: As library code, it sets bit 3 of a byte at offset 14 of a structure pointed to by a6 and returns.
- `initialize_library_function`: As library code, it checks bit 3 of a byte at offset 14 of a structure in a6; if not set, it increments a word at offset 32, and if that word is one, it calls another routine using a pointer from a global variable.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `decrement_counter_and_call`: Library code that decrements a 16-bit counter at offset 32 of the structure in a6, calls another routine if the counter reaches zero and bit 3 of the flag byte at offset 14 is set, and calls another routine with a global value when the counter is zero.
- `mask_and_branch`: Library code that masks the lower seven bits of d0, calls another routine, and sets bit 1 of the condition code register.
- `calculate_bitfield_difference`: As library code, it checks if d0 is zero or negative, then performs a series of shifts and arithmetic operations involving d0, d1, d4, d5, and d6 to update a value.
- `adjust_bitfield_value`: As library code, it sets the bit selected by register d1 in d0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `process_and_mask_byte`: As library code, it moves d0 to d7, checks if d7 is non-zero, calls a routine via a pointer from a global variable, sets bit 1 and clears bit 3 of d0, and branches to another routine.
- `call_jump_table_entries`: Calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_and_branch`: Clears the long register d1 and branches to another location in the library code.
- `execute_complex_conditional_calls`: Library code that retrieves a base address from a pointer in a global variable, performs several conditional calls to routines at specific offsets from that base using values in d0 and d1, and calls two other routines before returning.
- `execute_conditional_calls`: Library code that moves d6 to d4, calls a routine at an offset from a5 using d5 and d1, and if the result is not negative, calls another routine at a different offset from a5 twice using d5 and d1 before branching to another location.
- `execute_conditional_pointer_calls`: Library code that moves a longword from d5 to d2, then checks d6 and d4 to conditionally call a routine at an offset from a5 twice, calls another routine using d7, and potentially calls a final routine if the result is negative.
- `execute_pointer_calls`: Library code that moves d7 to d1, calls a routine using a pointer at an offset from a5, and if d4 is not zero, performs two calls through pointers at offsets minus 48 and minus 42 from a5 after loading a constant into d1.
- `call_and_store_result`: As library code, it moves d2 to d0, calls a routine using a pointer at an offset from a5, moves the result back to d2, and branches to another routine.
- `conditional_load_constant`: As library code, it moves d2 to d0, checks if d4 is non-zero, otherwise it loads a 32-bit constant into d2 and branches to another routine.
- `mask_return_value`: The library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `process_bit_manipulation`: Library code that saves registers and a byte from d7 to the stack, performs bitwise operations and comparisons, and if certain conditions are met, calls a routine using a pointer from the stack, performs arithmetic on d7, and branches based on the result.
- `shift_and_setup_registers`: Library code that performs a logical shift right on d7, clears d6, sets d5 to a constant, loads a pointer into a0, and sets d1 and d2 before branching to another routine.
- `execute_loop_with_branching`: Library code that executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `push_and_branch`: Library code that pushes the word negative 65538 onto the stack and then branches to another location.
- `push_and_branch_2`: Library code that pushes the word negative 65535 onto the stack and then branches to another location.
- `push_and_branch_3`: Library code that pushes the word 1 onto the stack and then branches to another location.
- `check_byte_and_return`: Library code compares a byte in d7 to 184, otherwise it branches to another location, increments a7 by 2, tests d7, and returns.
- `test_and_modify_byte`: Library code that tests a byte in d7 and, if it is between 56 and 168, saves it to d2, doubles it, and sets the carry flag if the result is between 138 and 168, otherwise it returns.
- `normalize_and_scale_value`: Calculates a bitwise scale factor based on the input in d2, performs a bitwise AND on d7 using that scale, and then executes a loop that performs arithmetic operations on d4, d5, d6, and d7 using values from a table in memory.
- `calculate_scaled_offset`: As library code, it performs arithmetic on d2 and d4 to determine a value in d7, then branches to another routine.
- `process_accumulated_values_2`: Performs arithmetic on d4, d5, d6, and d7, then enters a loop that iterates 23 times, updating d6, d4, and d7 using values from a table pointed to by a0.
- `process_accumulated_values`: Library code that performs arithmetic on d4, d5, d3, and d6, iterates through a list pointed to by a0 using a loop, and conditionally calls a routine via a pointer in memory.
- `load_constant_and_branch`: Library code loads the constant an address into d7 and branches to another location.
- `call_and_restore_stack`: Library code that moves d5 to d6, calls another routine, moves the result to d7, tests d7, and restores registers from the stack before returning.
- `call_and_restore_stack_2`: Library code that pushes d5 onto the stack, calls another routine, loads a value from an offset of the stack into d6, restores d6 from the stack, and branches to another location.
- `compare_and_return`: Library code that moves d6 to d4, and if d4 is not negative, compares d6 to a constant and branches based on the result, otherwise it clears d6 and returns.
- `shift_and_jump`: Library code arithmetic shifts d4 right by 8, increments d4 by 1, and if d4 is zero it jumps to a location via an indirect jump; otherwise it jumps to a location via an indirect jump.
- `return_without_effect`: Library code calls a routine and then, if the carry flag is not set, calls it again and then calls a third routine.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_conditional_callback_chain`: Library code tests a byte in d7, and if it is non-zero and the carry flag is not set, calls a routine, then conditionally calls two more routines and a final routine using values from the stack.
- `rotate_and_branch`: Library code loads the constant an address into d7, rotates d7 right by 1 bit, and branches to another location.
- `load_constant_and_branch_2`: Library code loads the constant an address into d7 and branches to another location.
- `test_and_branch_d6`: Pops a word from the stack into d6, tests if it is positive, and if so, clears d7 and branches to another routine; otherwise it clears d7 and branches to a different location.
- `set_interrupt_mask_and_restore_registers`: Sets the interrupt mask to a specific value by modifying the status register and restores the previously saved registers from the stack before returning.
- `set_status_and_return`: Loads a specific 32-bit constant into a register, adjusts the stack pointer by an offset of 30, and returns to the caller.
- `process_normalized_fixed_point_2`: Library code that saves registers and d7 to the stack, performs bitwise operations on d7, calls a routine with a constant in d6, and if the overflow flag is not set, performs arithmetic on d5 and d7, calls another routine, and branches to another location.
- `process_normalized_fixed_point`: Library code that manipulates d2, calls another routine three times with varying parameters, performs arithmetic using a value from the stack, calls a routine via a pointer, and enters a loop that branches based on the sign of a result.
- `process_table_and_accumulate`: Library code that increments d2, moves d5 and d6 to d3 and d4, performs arithmetic on d3 and d4, and then enters a loop that subtracts values from a table pointed to by a0 and updates d5, d6, and d7.
- `subtract_table_from_accumulator`: The routine walks through a table pointed to by a pointer and repeatedly adds the current table entry to an accumulator, then subtracts subsequent entries while a counter is decremented.
- `test_and_set_status`: Library code that tests d7, and if it is not zero and not positive, masks d7 and calls another routine, then sets bit 1 of the condition code register before returning.
- `set_d7_and_branch`: Sets d7 to negative one and branches to another routine.
- `invoke_multiple_callbacks`: Library code that saves registers, pushes d7 to the stack, calls three different routines via pointers, and then performs a loop involving bit-shifting and arithmetic based on the value of d7.
- `complex_loop_with_branching`: Library code executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `test_and_call_routine`: Tests the byte in d7, branches if it is positive, otherwise masks d7 and calls a routine via a branch to stack.
- `execute_library_chain`: Library code that calls a routine to load an address and branch, saves registers d3 through d5 to the stack, calls another routine, restores the registers, and then jumps to a third routine.
- `transform_d7_register`: Adds the value of a register to itself, conditionally performs a series of bitwise manipulations including a sign bit flip, an arithmetic shift, a subtraction, a word swap, and a 7-bit left rotation on the result, then returns.
- `modify_d7_bits`: Library code that swaps the high and low words of d7, performs a rotate left through carry, performs an exclusive OR on a byte of d7, adds d7 to itself, and performs further arithmetic and rotations based on the overflow flag.
- `test_and_modify_d7`: Library code that branches based on a condition, compares d7 to two different constants, adds a value to d7, performs a rotate left through carry, and tests d7 before branching.
- `mask_and_test_d7`: Performs a bitwise AND on d7 with a mask, then branches to another routine if the result is zero, otherwise it clears d7 and branches to another location.
- `test_and_shift_d7`: Library code that compares d7 to a constant, and if not equal, performs two logical shifts on d7 and branches based on the result.
- `shift_and_set_cc`: Shifts d7 left by eight bits, sets d7 to negative one, performs a rotate through carry on d7, sets bit 1 of the condition code register, and branches to another routine.
- `clear_d7_and_branch`: Sets d7 to zero and branches to another routine.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `load_address_and_branch_6`: Library code that loads a longword from a specific memory location into a0 and branches to another routine.
- `load_callback_address_5`: Loads the address of a global variable into register a0 and branches to another location.
- `load_address_and_branch_7`: Loads a long pointer from a memory location into a0 and branches to the start of the routine.
- `load_zero_pointer`: Library code that loads a null pointer into a0 and returns immediately via a branch.
- `load_address_and_branch_10`: Library code that loads a 32-bit address into a0 and then branches to the next instruction.
- `load_callback_pointer`: Loads a pointer from a location in memory into register a0 and branches to another routine.
- `load_callback_pointer_2`: Loads a pointer from a global variable into a0 and branches to it.
- `store_register_via_pointer`: Library code that loads a pointer into a0, saves several registers to the stack, calls another routine, stores the resulting 32-bit value from d7 into the memory address pointed to by a0, and returns.
- `load_callback_address_3`: Loads the address of a routine from a location at offset 52 from a global variable into a0 and then branches to the next instruction.
- `load_null_pointer_and_branch`: Loads a null pointer into a0 and branches to the start of the routine.
- `load_callback_address_4`: Loads the address of a global variable into a register and then performs an unconditional branch to the next instruction.
- `load_constant_pointer`: Library code that loads a constant value into a0 and returns immediately via a branch.
- `load_callback_address`: Loads a pointer from a global variable into address register a0 and then branches to that address.
- `load_address_and_branch_8`: Loads the address of a global variable into a register and then branches to a fixed location.
- `load_address_and_branch_9`: As library code, it loads a long value into a0 and branches to another location.
- `load_zero_address`: As library code, it loads a zero value into a0 and branches to another routine.
- `load_callback_address_2`: Loads a pointer from a location at offset 20 into address register a0 and then branches to the next instruction.
- `load_offset_address_a`: As library code, it loads a specific offset into a0 and branches to another routine.
- `dispatch_indirect_call`: Loads the address of a global variable into a register and branches to another routine that performs an indirect call using an offset from that address.
- `load_address_and_branch`: Loads a longword from a global variable into a0 and branches to a relative offset.
- `load_address_and_branch_2`: Loads the address of a hardware register into a6 and branches to another routine.
- `load_pointer_and_branch`: Library code loads a long value from a global variable into a0 and branches to another location.
- `load_pointer_and_branch_2`: Library code loads a long value from a different global variable into a0 and branches to another location.
- `load_address_and_branch_3`: Loads a longword from a global variable into a6 and branches to another routine.
- `load_address_and_branch_4`: Loads a longword from a global variable into address register zero and branches to another routine.
- `load_address_and_branch_5`: Loads a longword into a register from a specific memory location and performs a short branch to another routine.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### mathieeedoubbas.library

- `residentMI`: As library code, it executes an illegal instruction.
- `libInit2`: As library code, it stores the contents of a0 into a global variable at offset 258.
- `MIExpunge`: Checks a byte at offset 32 of a structure in a6; if it is non-zero, it sets bit 3 of a byte at offset 14 of that structure and returns; otherwise, it unlinks a node from a list by updating pointers, calculates a new address using values at offsets 16 and 18, calls another routine, and returns a value from a global variable.
- `Exp_InUse`: As library code, it sets bit 3 of a byte at offset 14 of a structure pointed to by a6.
- `MIOpen`: Tests bit 3 of a byte at offset 14 of a structure in a6; if set, it branches to another routine; otherwise, it increments a word at offset 32 of the structure, moves a6 to d0, and returns.
- `openFail`: As library code, it returns negative one in d0.
- `MIClose`: Library code that decrements a short counter stored at offset 32 of the structure in a6, and when the counter reaches zero and bit 3 of the byte at offset 14 is set, call the library expunge routine and return its result, otherwise return zero.
- `DPFix`: This library routine performs bitwise manipulation on two 32-bit values pointed to by a register, including a loop that shifts bits through a register based on a calculated count, and then tail calls another routine or returns.
- `dfi4`: Tests bit 1 of d0; if set, it branches to another routine, otherwise it clears d1 and branches to another location.
- `dfi6`: Clears d1 and branches to another routine.
- `dfix1`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `DPFlt`: As library code, it converts a signed single-precision value in d1 into a double-precision representation stored through the pointer in d0, computing an exponent and mantissa by shifting the input, assembling two 32-bit words while preserving the sign, and writing them via the pointer.
- `DPAbs`: As library code, it copies the high word of a double from the source pointer in d1 to the destination pointer in d0 and writes the low word with its sign bit cleared, producing an absolute value of a double-precision number.
- `dcmp_1`: Small routine of 2 instructions that subtracts 1 from d0, then returns.
- `dcmp_2`: Small routine of 2 instructions that adds 1 to d0, then returns.
- `dtst_1`: As library code, it decrements the value in d0 by one and returns.
- `dtst_2`: As library code, it increments the value in d0 by one and returns.
- `cc_1`: As library code, it calls a function whose address is passed in an address register and then returns.
- `cc_2`: As library code, it calls a function whose address is passed in an address register and then returns the contents of a stack slot.
- `cc_3`: As library code, it calls a function pointer held in register a1 and returns the value left on the stack at offset 18.
- `MIIEEEDPFix`: This library code calls another routine and then returns.
- `MIIEEEDPFlt`: Library code that calls a floating point routine and returns the value previously on the stack.
- `MIIEEEDPCmp`: Library code that jumps to another routine.
- `MIIEEEDPTst`: Loads the address of a routine from a relative offset into a pointer and calls it via a tail branch.
- `MIIEEEDPAbs`: Loads an address from a relative offset into a6 and calls another routine via a tail branch.
- `MIIEEEDPNeg`: Loads an address from a relative offset into a register and calls another routine to negate a double precision floating point value.
- `MIIEEEDPAdd`: Loads an address from a relative offset into a register and performs a tail branch to another routine.
- `MIIEEEDPSub`: Loads an address from a relative offset into a register and calls another routine via a tail branch.
- `MIIEEEDPMul`: Loads an address from a constant offset relative to the program counter into a register and calls another routine at a different relative offset.
- `MIIEEEDPDiv`: Loads an address from a table using the program counter and calls the division routine DPDiv via a tail branch.
- `MIIEEEDPCeil`: Library code that calls a routine to negate a value, calls another routine to perform a floor operation, and then calls a third routine to negate the result.
- `MIIEEEDPFloor`: Library code that saves registers, sets d4 and d5 to specific constants, calls a fix routine and a comparison routine, and if the comparison does not result in a non-zero value, adjusts the stack pointer and moves d6 and d7 into d0 and d1 before branching to exit.
- `test_d6_d7`: Library code that moves d6 and d7 to d0 and d1, calls another routine, and if the result is not negative, restores registers and calls a second routine.
- `lt_zero`: This library routine stores the value from register d5 into register d2, sets register d3 to zero, restores registers d0 and d1 from the stack, and then calls another routine.
- `fptrap`: As library code, it reads a word from a pointer, masks it, and uses the result to perform a jump into a table.
- `fpoptab`: Jump stub: passes control straight on to badtrap.
- `jump_to_badtrap`: Jump stub: passes control straight on to badtrap.
- `dfsub`: Library code that saves several registers to the stack, loads a pointer from d1 into a0 and a pointer from d2 into a1, loads a byte from the memory at a1 into d3, performs a bitwise change on d3, and branches to another routine.
- `dfadd`: Performs addition of two double-precision floating-point numbers stored in memory pointed to by two registers, writing the result back to the first memory location.
- `dfa11`: Subtracts two double-precision floating-point numbers stored in memory and writes the result back to the same location.
- `dfa14`: Library code that sets d0 to a constant, clears d7, performs a bitwise NOT on d1 and d2, adds one to d2, and performs an addition with extend from d7 into d1 before branching to another location.
- `dfa13`: Executes a loop based on d0 that performs bit shifts and rotations on d3 and d4, then performs additions using the carry flag and branches to another routine based on the result.
- `dfa5`: Library code that performs bitwise negation and addition on d1 and d2 using d7 as an extend, compares the result to an address, and either adds an address to d5 and branches to another routine or performs bit shifts and further additions before branching to another routine.
- `dfa8`: Library code that compares d1 and d4, and if d1 is less than d4, subtracts d4 from d5, shifts d2 left by one, and performs a rotate left through extend on d1 before branching to another location.
- `dfa7`: Library code that tests d5, modifies d1 using bitwise operations with d5 and d0, stores d1 and d2 into a structure pointed to by a0, and then branches to another routine.
- `dfa9`: Library code that clears the longword at a0 and the longword at offset 4 of a0, then branches to another location.
- `dfauo`: This library routine compares register d5 to a constant, and if it is less than that constant, it performs a bitwise OR between d0 and a constant, stores the result at the pointer in a0, stores a second constant at offset 4 of that pointer, and then tail branches to another routine.
- `dfmul`: Performs multiplication of two double-precision floating-point numbers stored in memory pointed to by two registers, handling sign bits, exponent addition, and mantissa multiplication through a series of shifts and word-sized multiplications.
- `dfm4`: Clears the long value in a0 and branches to another routine.
- `dfmuo`: This library routine compares register d0 to a constant, and if it is less than that constant, it performs a bitwise OR between d4 and a constant, stores the result at the pointer in a0, stores a constant of all ones at offset 4 of that pointer, and then tail branches to another routine.
- `dfdiv`: Performs double-precision floating-point division using a software-based iterative algorithm.
- `dfd10`: Executes a loop to perform long division on 64-bit values using bit-shifting and subtraction, then stores the resulting quotient and remainder into memory pointed to by a register.
- `dfd4`: Performs arithmetic operations on values in d0 and d5 to compute a result, stores the resulting 32-bit words into a table pointed to by a0, and calls another routine.
- `dfsetz`: Sets registers d5 and d6 to zero and branches to another routine.
- `dfdbyz`: Loads the constant 7 into d3 and branches to another routine.
- `dfduo`: This library routine compares register d7 to a constant, and if it is less than that constant, it performs a bitwise OR between d0 and a constant, stores the result in register d5, sets the condition codes using a constant in d3, and then tail branches to another routine.
- `dfneg`: Moves a pointer from d1 to a0, reads a word from that address, clears its lower 11 bits, and if the result is zero, performs an XOR on the byte at that address before branching to another routine; otherwise it branches to another routine.
- `dfcmp`: Saves registers to the stack, compares two long values from memory pointed to by d1 and d2 by checking their upper bits and then their lower bits, updates the condition codes, and branches to another routine.
- `dftst`: As library code, it loads a pointer from d1, reads a 16-bit word from that location into d0 and d3, and if the word at the pointer is not zero when masked with 32752, it returns; otherwise it restores registers and returns.
- `badtrap`: Small routine of 1 instruction that copies 224 to d1.

### Jetstrike (demo) (1993)(Rasputin).adf:Jetstrike_Demo

- `initialize_resource_and_decompress_data`: Initializes a structure by setting a pointer to a hardware register and calling several routines to validate system state, then performs a decompression routine that uses a bit reader to decode a bitstream into a target buffer while updating multiple pointers in a table.
- `initialize_subsystem_and_cleanup_memory`: Initializes a table by zeroing 50 entries and sets several pointers in a structure.
- `copy_strings_and_initialize_subsystem`: Copies two null-terminated strings from a structure into local stack buffers, checks for a specific four-byte magic identifier in a data block to copy a subsequent sequence of bytes, calls an indirect routine, calls two routines via the exec library, and then iterates through a table of twenty-two entries to clear them while calling a memory allocation routine.
- `validate_and_initialize_subsystem`: Performs a series of conditional checks by calling other routines with specific values in d0, and if successful, initializes a table by zeroing out entries in a loop and updating several pointers in a structure via a6.
- `branch_and_set_d7`: Calls another routine, sets d7 to 255, and branches to another routine.
- `branch_on_word_and_call`: The routine tests a word through a pointer against a constant and calls one of two indirect routines through a global variable depending on the result, then returns a concatenated value from two inputs.
- `call_indirect_via_global`: The routine calls an indirect routine through a global variable.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_copy_mem`: Library stub: loads its arguments and the exec library base, calls CopyMem and returns its result to the caller.
- `walk_byte_table_and_update_pointers`: The routine walks a byte table until a zero byte is found, and for each byte it either advances a pointer by a fixed amount or computes a new pointer from the byte value and adds an offset from a base pointer, storing the result back into the pointer.
- `clear_word_and_call_indirect`: The routine clears a word through a pointer and calls an indirect routine through a pointer in memory.
- `process_descriptor_and_chain`: The routine processes a descriptor pointed to by an argument, writes fields into a table using an index derived from a short value, updates a count and a flag byte, chains pointers in a loop based on a count, calls another routine, and adjusts two counters through pointers.
- `process_memory_table_entry`: Iterates through a table of pointers to calculate an offset, then uses the resulting address to perform memory operations and calls other routines to manage or free memory based on the data found.
- `zero_d1_and_branch`: Sets d1 to zero and branches to another routine.
- `call_and_branch`: Calls another routine and then branches to another routine.
- `check_table_entry_and_prepare`: Examines a table entry pointed to by a global variable, conditionally calls an indirect routine through a global variable, and prepares output registers with values derived from the table and other routines.
- `reset_counters_and_call`: The routine calls another routine, clears two words through a pointer, and returns an input value.
- `signal_task_on_switcher`: The routine reads a global variable, calls another routine through a table and, if the result is non-zero, calls a second routine through the same table.
- `calculate_pointer_offsets`: Adds the value in d0 to d1, sets a0 and a1 to the results, and performs several memory operations including an XOR between a value from a pointer in a0 and d5.
- `decompress_bitstream`: Decodes a packed bitstream using a bit reader refill and bit read helper, decodes control bits to choose copy or literal handling, and writes bytes into a destination buffer pointed to by a register while tracking a count and a repeat length.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Jetstrike (demo) (1993)(Rasputin).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_color_register`: Writes a value to the sixteenth color register of the hardware.
