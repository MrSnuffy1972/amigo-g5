# Game 2422 (caesar deluxe (1993)(impressions)[cr atx][fd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02422). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1809648 top-level bytes in 187 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 29 of 29 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 363 of 363 named (2 author symbols), 363 of 363 described (79 by rule, 152 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1563 of 1563 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8068 of 8068 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1227542 |
| code | 13986 |
| data | 495541 |
| padding | 57219 |
| unused | 15360 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:play | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:assign | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:echo | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:lab | 1 | 2 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:skip | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:CSR.EXE | 8 | 230 | 17 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:failat | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:Anthrox | 6 | 250 | 5 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:cd | 2 | 294 | 5 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:endif | 1 | 2 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:caesar.boot | 1 | 31 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:if | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:play | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:assign | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:echo | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:lab | 1 | 2 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:skip | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:CSR.EXE | 8 | 230 | 17 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:failat | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:Anthrox | 6 | 250 | 17 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:cd | 2 | 294 | 5 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:endif | 1 | 2 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:caesar.boot | 1 | 31 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:if | 2 | 117 | 2 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:bootblock | 1 | 18 | 1 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1670 | 47 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:Anthrox#unpacked | 2 | 1670 | 109 |
| Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:Anthrox#unpacked | 2 | 1670 | 109 |

## Cases handled

- caesar deluxe (1993)(impressions)[cr atx](disk 1 of 2)[fd]/Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf: file caeswa32.apx (header 27953 bytes, blocks hold 976): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 1314 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- caesar deluxe (1993)(impressions)[cr atx](disk 1 of 2)[fd]/Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:CSR.EXE: Imploder (xfd) stream found by ancient scan (113624 -> 261180 bytes) but only 0 of 114 sampled windows are in reference RAM: not used
- caesar deluxe (1993)(impressions)[cr atx](disk 1 of 2)[fd]/Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:Anthrox: crunched executable decrunched by XFDMaster (Imploder (xfd), 12208 -> 25784 bytes), 10 of 11 windows confirmed in reference RAM: recovered as a hunk executable
- caesar deluxe (1993)(impressions)[cr atx](disk 2 of 2)[fd]/Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:CSR.EXE: Imploder (xfd) stream found by ancient scan (113624 -> 261180 bytes) but only 0 of 114 sampled windows are in reference RAM: not used
- caesar deluxe (1993)(impressions)[cr atx](disk 2 of 2)[fd]/Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:Anthrox: crunched executable decrunched by XFDMaster (Imploder (xfd), 12208 -> 25784 bytes), 10 of 11 windows confirmed in reference RAM: recovered as a hunk executable
- caesar deluxe (1993)(impressions)[cr atx](disk 1 of 2)[fd]/Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:CSR.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- caesar deluxe (1993)(impressions)[cr atx](disk 2 of 2)[fd]/Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:CSR.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 packed track region(s), 29696 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:play

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_words_then_call`: Copies a fixed run of sixteen words from a source table offset by a constant into a destination pointer, then invokes a routine pointed to by a global slot.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:assign

- `find_max_and_prepare_header`: Walks a linked list of entries to find the maximum value in a field, then writes that value and a run of sixteen words into a header buffer.
- `conditionally_copy_words`: Copies a fixed number of words from a source table into a destination buffer pointed to by a parameter, then invokes a routine stored in a global slot.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:echo

- `find_max_and_copy_blocks`: Walks a linked list of entries to find the maximum value in a field, then writes that value and a small header into a buffer.
- `copy_block_and_call_3`: Copies a fixed-size block of data from a source address to a destination address when the destination is valid, then invokes another routine.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:skip

- `find_max_and_prepare_buffer_10`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block into a table and copies sixteen words from a source table into a destination pointer before invoking another routine.
- `copy_words_and_call`: Copies a fixed number of words from a source offset into a destination buffer when the destination is non-null, then invokes a routine through a stored pointer.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:CSR.EXE

- `run_setup_loop`: Performs a small sequence of setup operations, repeatedly invokes another routine and a function stored in a global variable table while decrementing a counter, then writes zero through a pointer.
- `decrement_to_negative_one`: Repeatedly subtracts one from a signed 16-bit variable until it becomes negative one, then exits.
- `loop_table_updates`: Performs three calls to a routine via a pointer in a4 while incrementing d6, then calls a routine via a pointer in a4 with d6 set to d5, and finally enters a loop that increments d5 until it reaches 4.
- `update_table_values`: Performs a series of checks and calls on d0, then calls a routine via a pointer in a4, and finally enters a loop that adds a value from d3 to a table at an offset determined by d0 and d2.
- `frag_beq_movea_bsr_bra`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `frag_cmpa_beq_move_subq_bne_rts`: Short code fragment that compares a2 with a6, then branches if the last result was zero, then copies the memory just below a2 (moving a2 down) to the memory at a1 (advancing a1) (4 times in a row), then subtracts 2 from d0, then branches if the last result was not zero, then copies the memory just below a1 (moving a1 down) to d0, then returns.
- `decode_variable_bitstream_2`: Reads a control byte from a table to determine a bit count and a number of repetitions, then decodes a variable-length bitstream from a source buffer in a0 into a destination buffer in a3 using bit-shifting and carry-propagation logic.
- `frag_moveq_add_bne_move_addx_dbra_addq`: Short code fragment that loads 2 into d7, then adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d0 and the extend flag to d0, then loops back while counting down d7, then adds 6 to d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_2`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_variable_bitstream_expansion`: Decodes a variable-length bitstream from a buffer in a0 by reading control bytes from a table to determine bit counts and using bit-shifting and carry-propagation to reconstruct values.
- `frag_add_bne_move_addx_bra`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `decode_variable_bitstream`: Reads a control byte from a table to determine a bit-count, then decodes a variable-length bitstream from a source buffer in a0 into a destination buffer in a3.
- `decode_and_copy_bitstream`: Reads a control byte from a table to determine a bit-count, then decodes a variable-length bitstream from a source buffer into a destination buffer using bit-shifting and carry operations.
- `frag_move_movem_bra`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:failat

- `find_max_and_prepare_buffer_11`: Walks a linked list of entries to find the largest value in a field, then, if a condition is met, writes that value and a small header into a buffer and sets up a callback.
- `copy_words_and_call_2`: Copies twelve words from a source table into a destination buffer when the buffer pointer is valid, then invokes the routine whose address is held in a register.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:Anthrox

- `initialize_aga_patch`: Invokes a couple of other routines, then runs a four-iteration loop that repeatedly calls another routine and dereferences a pointer stored in a global structure to invoke a function through it.
- `decrement_until_minus_one_3`: Repeatedly decrements a signed 16-bit local variable by one until it reaches negative one, then returns.
- `process_cached_data`: Executes a routine twice, then runs a two-iteration loop that repeatedly calls another routine and branches on the sign and zero status of its result to perform either an accumulating pointer update or a single call.
- `copy_byte_block`: Copies a block of bytes from one memory location to another using a length counter.
- `copy_memory_and_process_bits`: Copies a block of bytes from one memory location to another, decrementing both pointers as it goes.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:cd

- `find_max_and_prepare_buffer_12`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header of sixteen words into a destination buffer and invokes another routine.
- `copy_values_and_call`: Copies a fixed count of four-byte values from a source table, located at a fixed offset from a base pointer, into a destination buffer when that buffer is not null.
- `update_callback_state_and_structure`: Performs a bitwise XOR on a byte at a pointer, initializes several offsets within a structure, and executes a sequence of callback routines that conditionally update values at offset 600 and other locations within the structure based on table lookups.
- `execute_callback_sequence`: Calls a routine through a pointer in a5 multiple times with different parameters in a4 and d0, and updates a longword at offset 600 of a structure based on the results.
- `execute_sequence_of_calls`: Performs a series of calls through a pointer in a5 using various parameters from a2 and a1, updates a longword at offset 600 of a structure in a2, and ends with an indirect jump to the address in a6.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:endif

- `return_zero_3`: Returns a single zero value and performs no other operation.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:caesar.boot

- `ProgStart`: Opens the graphics and intuition libraries, sets a flag in the intuition display structure when it is still clear, and in that case frees a graphics raster and remakes the display, then closes both libraries and returns zero.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:if

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block of sixteen words into a destination buffer and invokes another routine pointed to by a value.
- `copy_buffer_and_call`: Copies a fixed-size block of words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:play

- `find_max_and_prepare_buffer_3`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block and copies sixteen words from a source table into a destination buffer before invoking a routine pointed to by the caller.
- `copy_buffer_and_call_2`: Copies a fixed block of 15 words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:assign

- `find_max_and_prepare_buffer_4`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header table filled with a repeating pattern and copies sixteen words from a source table into a destination buffer.
- `copy_block_and_call`: Copies a fixed-size block of words from a source table into a destination buffer when that buffer is not null, then invokes another routine.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:echo

- `find_max_and_prepare_buffer_5`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block and copies sixteen words from a source table into a destination.
- `copy_block_and_call_2`: Copies a fixed-size block of words from one memory location to another when the destination is valid, then invokes another routine.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:lab

- `return_zero`: Empty routine that returns zero without touching any data.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:skip

- `find_max_and_prepare_buffer_6`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header of sixteen words into a buffer and copies sixteen words from a source table into a destination pointer.
- `copy_words_then_call_2`: Copies fifteen consecutive 32-bit words from a source table pointed to by a register into the block of memory addressed by a stack parameter, then invokes the routine referenced by another register.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:CSR.EXE

- `run_startup_cleanup`: Performs a small cleanup sequence at program startup by invoking another routine several times and clearing a four-byte location pointed to by a stack value.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `execute_pointer_calls`: Calls a routine via a pointer in a4 three times with varying register values, then enters a loop that increments d5 and repeats the process until d5 reaches 4.
- `update_table_values_2`: Performs conditional calls to another routine and a routine via a pointer in a4, then enters a loop that adds a value from d3 to a table at a location determined by a3 and d2, decrementing d1 until it reaches zero.
- `frag_beq_movea_bsr_bra_2`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `frag_cmpa_beq_move_subq_bne_rts_2`: Short code fragment that compares a2 with a6, then branches if the last result was zero, then copies the memory just below a2 (moving a2 down) to the memory at a1 (advancing a1) (4 times in a row), then subtracts 2 from d0, then branches if the last result was not zero, then copies the memory just below a1 (moving a1 down) to d0, then returns.
- `decode_variable_bitstream_expansion_2`: Reads a control byte from a table to determine a bit count and a value, then decodes a variable-length bitstream from a source buffer in a0 into a destination buffer in a3 using bit-shifting and carry propagation.
- `update_bitstream_counters`: Increments a 16-bit value in d0 and a 8-bit value in d1 using the carry flag, then updates a 16-bit value in d0 by adding 6.
- `frag_moveq_bra_4`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_5`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_6`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_variable_length_bitstream`: Decodes a variable-length bitstream from a source buffer in a0 by reading control bytes from a table to determine bit counts, performing bitwise shifts and additions to reconstruct values, and updating a pointer in a0.
- `frag_add_bne_move_addx_bra_2`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `decode_variable_bitstream_expansion_3`: Reads a control byte from a table to determine a bit-count, then decodes a variable-length bitstream from a source buffer into a destination.
- `decode_variable_bitstream_data`: Reads a control byte from a table to determine a bit count and a number of repetitions, then decodes a variable-length bitstream from a source and writes the resulting values to a destination buffer.
- `frag_move_movem_bra_2`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:failat

- `find_max_and_prepare_buffer_7`: Walks a linked list of entries to find the maximum value in a field, then writes a small header block containing that maximum and a run of sixteen words into a destination buffer.
- `copy_words_then_call_pointer`: Copies a fixed number of four-byte words from a table into a caller-provided buffer when that buffer is not null, then invokes a pointer stored in a register.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:Anthrox

- `initialize_patch_dispatch`: Performs a fixed sequence of setup calls, then runs a four-iteration loop that invokes another routine and calls through a pointer stored in a global variable.
- `decrement_until_minus_one_2`: Repeatedly subtracts one from a local word until it becomes negative one, then returns.
- `update_array_via_function_calls`: Calls a function pointer via a4 multiple times, using the returned values to increment elements in an array pointed to by a1.
- `accumulate_array_via_function_pointer`: Calls a function pointer via a4, then uses the returned value to add a value from a1 to an array at a3 using an index calculated from d0.
- `frag_beq_movea_bsr_bra_3`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_with_counter`: Compares a pointer in a2 with a pointer in a6 and, if they are not equal, copies four bytes from the source pointed to by a2 to the destination pointed to by a1 while decrementing a counter in d0.
- `expand_variable_length_bitstream`: Reads a control byte and a bit-length from a table to perform bitwise expansion of data from a source buffer into a destination buffer.
- `expand_bitstream_data_2_loop_control`: Iterates through a bitstream by reading a byte from a buffer in a0, adding it to d1 and d0 using extended addition, and updates d0 by six before branching to another routine.
- `frag_moveq_bra_7`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_8`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_9`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `expand_bitstream_data_2`: Reads a bit-length from a table and performs bitwise expansion of data from a source buffer into a destination buffer.
- `frag_add_bne_move_addx_bra_3`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `expand_bitstream_data`: Reads a bit-length from a table and performs bitwise expansion of data from a source buffer into a destination buffer.
- `decode_and_copy_bitstream_2`: Reads a control byte from a table to determine a bit-length and then performs a bitwise expansion of data from a source buffer into a destination buffer.
- `frag_move_movem_bra_3`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:cd

- `find_max_and_prepare_buffer_8`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes that value and a small header into a buffer.
- `copy_block_then_call`: Copies a fixed-size block of 32-bit words from one location to another when the destination is not null, then calls another routine.
- `update_state_via_callback_sequence`: Performs bitwise operations on a byte in a structure, updates several offsets within that structure, and executes a sequence of calls through a pointer in a5, conditionally branching based on values retrieved from a table and updating a longword at offset 600 of a structure.
- `execute_callback_sequence_2`: Calls a routine via a pointer in a5 using a value derived from a structure at offset 580 of a4, then performs several more calls using parameters from a2 and a1, and finally updates a longword at offset 608 of a structure in a2.
- `process_callback_data`: Performs a series of calls through a pointer in a5 using values from a structure in a2 and a1, updates a longword at offset 440 of a1, and ends by jumping to the address in a6.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:endif

- `return_zero_2`: An empty routine that immediately returns zero without touching any data.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:caesar.boot

- `ProgStart`: Opens the graphics and intuition libraries, sets a flag in the intuition display structure when it is still clear, and in that case frees a graphics raster and remakes the display before closing both libraries and returning zero.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:if

- `find_max_and_prepare_buffer_9`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header block into a buffer and copies sixteen words from a source table into it before invoking another routine.
- `copy_buffer_and_call_3`: Copies a fixed run of 15 words from a source table into a destination buffer pointed to by a parameter, but only when that parameter is not null.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:bootblock

- `open_libs_report_status`: Opens a library and sets a flag in it, then opens a second library and reports whether that second open succeeded.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_graphics_copper_sequence`: Opens the graphics library, saves interrupt and DMA state, waits on the raster beam, clears sprite control registers and fills a table of sprite values, then runs sixty passes that wait for raster lines while stepping a counter and two pointers through fixed tables, and finally restores the saved interrupt and library state.
- `swap_buffer_pair_store_offset`: Calls four helper routines in sequence, then stores a value 350 bytes past the second saved buffer address into a shared pointer and swaps the two saved buffer addresses.
- `process_counter_table_lookup`: Checks a global counter and, when it is nonzero, either decrements it recursively or indexes into a table using a scaled offset and copies several bytes from it.
- `swap_buffer_pair_back_offset`: Calls another routine twice, then swaps two adjacent global variables by XORing a flag bit and using a temporary value.
- `set_table_pointers`: Stores an incoming address into a global variable and writes a computed address into another global variable.
- `store_scaled_position_pair`: Stores the incoming value shifted right by three bits into one shared variable, then stores the value shifted right by six bits into a second shared variable.
- `combine_and_mask_coordinate_pair`: Adds each of two word pairs to its partner and masks both results to clear the low bit and the top four bits, keeping the values even and within a limited range.
- `init_blitter_registers`: Saves a caller-supplied value into a global variable, then writes several blitter control, address and size registers and waits for the blitter to become idle.
- `configure_blitter_registers`: Writes several blitter control, address and size registers, then waits for the blitter to become idle.
- `manage_flag_state_machine`: Checks a flag bit and, when set, queries a routine whose result controls a small state machine that counts down and eventually resets several fields.
- `configure_blitter_registers_2`: Writes several blitter control, address and size registers with a different height value, then waits for the blitter to become idle.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_blitter_idle`: Spins while the blitter busy flag in the DMA control status register is set, then returns.
- `reset_text_stream_state`: Points a text stream read position at a fixed start address, zeroes the column counter, sets the line counter to one, and clears a byte in shared memory.
- `advance_string_pointer_2`: Reads a character from a string and advances a pointer, skipping past leading zeros to the next non-zero entry.
- `advance_string_pointer_step`: Increments a counter and wraps it when it reaches a fixed limit, then advances a string pointer.
- `advance_string_pointer`: Initialises a counter and advances a pointer past a leading zero, then scans a table for the next non-zero entry, storing it along with a fixed length.
- `setup_blitter_aligned_copy`: Sets blitter source masks and modulo values, then chooses the control word and size depending on whether two address values are odd or even, offsetting the source and destination by fixed amounts in the odd case.
- `build_copper_list_from_pointer_2`: Writes two fixed copper instruction words at a destination pointer, then copies sixty-four word pairs from a source table into the list.
- `build_copper_list_from_pointer_3`: Writes two fixed copper instruction words at a destination pointer located further along, then copies sixty-four word pairs from a different source table into the list.
- `build_copper_list_from_pointer`: Writes two fixed copper instruction words at the destination pointer, then copies 64 word pairs from the source table into the list, advancing six words per source entry.
- `split_base_address_words`: Splits the low and high words of five base addresses, each formed by adding a fixed offset to a shared base, into paired global variables, then loads a value into a global variable.
- `encode_sprite_position_words`: Encodes a stored position into the words of two sprite control entries: the start position is halved, a fixed offset is added to the stop position, and a bit field derived from a second stored value is merged in.
- `store_address_and_offset`: Stores the incoming address into a global variable and then writes a computed address into another global variable.
- `call_and_toggle_flag`: Calls another routine and then flips the lowest bit of a global variable by XORing it with one.
- `store_arguments_run_helpers`: Saves the incoming address and two register values into shared variables, then calls three helpers in order.
- `test_memory_bit`: Polls a specific bit within a byte at an offset from a pointer until that bit is cleared.
- `setup_blitter_rect_copy`: Reads two corner coordinate pairs from shared storage, computes the blit width in words and the height in rows from their differences, sets a full-mask destination-only blit with a modulo, and starts it using an address supplied by a helper.
- `start_blitter_masked_rect_copy`: Computes the blit width and height from two stored corner coordinate pairs, sets a full-mask blit with a combined source and destination control word and modulo values, stores the destination address and starts the blit.
- `project_points_scaled_to_screen`: Resets a rectangle record to default bounds, then walks a table of points scaling each coordinate pair by a stored factor and adding fixed centre offsets before clamping the results to the bounds.
- `process_indexed_point_pairs`: Walks a list of index pairs, looks up two words for each index in a table, and passes them with the paired value to a helper, once per pair.
- `reset_default_clip_rect`: Stores four default bounds values, 160, 160, 128 and 128, into a shared rectangle record.
- `clamp_values_to_bounds`: Clamps a pair of coordinate values into the stored bounds of a rectangle, updating each edge only when the incoming value falls outside it.
- `setup_blitter_line_octant`: Initializes the blitter control and data pointers, then computes the source address, step counts, an octant selector from the coordinate signs, and a control word for drawing a line between two points, storing all of these values into the blitter registers.
- `find_max_byte_and_reset`: Scans a zero-terminated string from a base address to find its longest byte value, then uses that value to index a fixed table.
- `tick_counter_and_update_registers`: Increments a counter and resets a group of globals when a limit is reached, copying several values into a hardware register block.
- `clear_state_globals`: Clears five consecutive global variables to zero and sets the last one to a fixed value, then returns.
- `process_input_nibble_2_2`: Reads a nibble from a structure field and dispatches based on its value, scaling and clamping a counter up or down against stored limits.
- `setup_and_play_sound`: Copies a sound entry from a lookup table into a structure, setting its length, period, volume and pointer fields.
- `process_input_nibble_2`: Reads a nibble from a structure field and dispatches based on its value, scaling and clamping a counter up or down against stored limits.
- `copy_structure_word`: Copies a word from one structure field into another structure field at a fixed offset, using two passed base addresses.
- `search_table_and_set_flag`: Searches a table for a value within a limit, adjusting the search index and handling a sign flag.
- `clamp_and_lookup_value`: Clamps a structure field between zero and a stored maximum, adjusting it up or down and resetting the maximum when the limit is reached.
- `adjust_struct_value_from_table`: Reads a couple of fields from a structure, applies a sign and a scaling factor to a value selected from a table, then stores the adjusted result back into the structure and updates another of its fields.
- `process_input_nibble`: Reads a nibble from a joystick/mouse input byte and dispatches based on its value, adjusting structure fields by scaling, clamping, or storing them.
- `store_low_nibble_value`: Stores the low nibble of one structure field into another field of the same structure.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 1 of 2)[FD].adf:Anthrox#unpacked

- `initialize_graphics_and_copper_lists`: Checks attention flags to decide whether to enter supervisor mode, opens a library, and disables interrupts and DMA. It then initializes several copper lists and sprite control registers, populates a table with encoded position words, and enters a loop that updates a value in a global variable based on a counter and the status of a joystick button.
- `swap_buffer_pair_store_offset_2`: Calls four helpers, stores a value 350 bytes past the second saved buffer address into a shared pointer, then swaps the two saved buffer addresses.
- `dispatch_index_table_lookup`: Reads a stored index: when it is below ten it stores the index minus one and calls a helper; when it is above sixteen it looks up four bytes in a table at a position derived from the index and stores them in shared variables.
- `swap_buffer_pair_back_offset_2`: Calls two helpers, stores a value two bytes before the second saved buffer address into a shared pointer, then swaps the two saved buffer addresses.
- `set_table_pointers_2`: Stores a pointer to a table and a second pointer calculated from the first word of that table.
- `store_scaled_position_pair_2`: Calls a helper, stores the incoming position shifted right by three bits into one shared variable, calls another helper, then stores the position shifted right by six bits into a second shared variable.
- `combine_and_mask_coordinate_pair_2`: Adds each of two word pairs to its partner and masks both results to clear the low bit and the top four bits, keeping the values even and within a limited range.
- `frag_movea_suba_bra`: Short code fragment that copies a location in the code to a0, then subtracts 4096 from a0, then jumps to another point in the code.
- `configure_blitter_registers_3`: Writes values to several blitter control and size registers and waits for the blitter to become idle.
- `handle_interrupt_request`: Checks bit 5 of the interrupt request register, calls another routine, and updates various PC-relative memory locations based on the result.
- `frag_bsr_movem_move_rte`: Short code fragment that calls another routine, then restores saved registers from the stack, then copies 32 to a fixed memory address, then returns from the exception.
- `update_counter_and_call`: Calculates offsets based on a PC-relative word, calls another routine twice with different calculated addresses, and increments a PC-relative byte until it reaches 110.
- `frag_lea_bsr_move_rts`: Short code fragment that loads the address of a fixed memory address into a6, then calls another routine, then copies an address to the memory at an offset from a6, then copies an address to the memory at an offset from a6, then copies 0 to the memory at an offset from a6, then copies a0 to the memory at an offset from a6, then copies 151 to the memory at an offset from a6, then returns.
- `frag_subq_bne_bra`: Short code fragment that subtracts 1 from the memory at a0, then branches if the last result was not zero, then jumps to another point in the code.
- `wait_for_raster_line_2`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_blitter_idle_2`: Spins while the blitter busy flag in the DMA control status register is set, then returns.
- `reset_text_stream_state_2`: Resets a text stream: points the read position at a fixed start address, zeroes the column counter, sets the line counter to one and clears a byte in shared memory.
- `process_text_stream_character`: Reads the next character of a text stream: a zero byte skips padding, a one byte restarts the stream, a carriage return starts a new line, and any other character is passed to a helper and advances the column, starting a new line after 41 columns.
- `frag_lea_move_addq_cmpi_beq_bra`: Short code fragment that loads the address of a location in the code into a1, then copies a0 to the memory at a1, then loads the address of a location in the code into a5, then adds 1 to the memory at a5, then compares the memory at a5 with 41, then branches if the last result was zero, then jumps to another point in the code.
- `frag_lea_clr_addq_bra`: Short code fragment that loads the address of a location in the code into a5, then clears the memory at a5, then adds 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_lea_addq_tst_beq_move_rts`: Short code fragment that loads the address of a location in the code into a1, then adds 1 to a0, then tests the memory at a0, then branches if the last result was zero, then copies a0 to the memory at a1, then loads the address of a location in the code into a0, then copies 450 to the memory at a0, then returns.
- `frag_lea_adda_move_rts`: Short code fragment that loads the address of a location in the code into a0, then adds 5916 to a0, then loads the address of a location in the code into a1, then copies a0 to the memory at a1, then loads the address of a location in the code into a0, then copies 450 to the memory at a0, then returns.
- `frag_lea_move_addq_rts`: Short code fragment that loads the address of a location in the code into a5, then copies 0 to the memory at a5, then adds 1 to the memory at an offset from a5, then loads the address of a location in the code into a1, then copies a0 to the memory at a1, then returns.
- `setup_blitter_transfer`: Sets up blitter registers including source, destination, and control words using values from a1 and a3, then conditionally modifies the control register and destination address before returning.
- `frag_move_btst_bne_rts`: Short code fragment that copies an address to the memory at an offset from a6, then tests bit 0 of d1, then branches if the last result was not zero, then copies 32768 to the memory at an offset from a6, then copies 1994 to the memory at an offset from a6, then copies 2050 to the memory at an offset from a6, then returns.
- `frag_lea_movea_adda_bra`: Short code fragment that loads the address of a location in the code into a0, then copies a location in the code to a1, then adds 1280 to a1, then jumps to another point in the code.
- `frag_lea_movea_adda_bsr`: Short code fragment that loads the address of a location in the code into a0, then copies a location in the code to a1, then adds 1280 to a1, then calls another routine, then loads the address of a location in the code into a0, then copies a location in the code to a1, then adds 1792 to a1.
- `frag_move_addq_moveq_adda_dbra_rts`: Short code fragment that copies an address to the memory at a1, then copies an address to the memory at an offset from a1, then adds 4 to a1, then loads 63 into d7, then copies the memory at a0 to the memory at a1, then copies the memory at an offset from a0 to the memory at an offset from a1, then adds 2 to a1, then adds 12 to a0, then loops back while counting down d7, then returns.
- `set_copper_list_pointers`: Stores the high and low words of four list addresses, each a shared base address plus a fixed offset, into shared pointer variables, and sets a control constant.
- `encode_sprite_position_words_2`: Encodes a stored position into the words of two sprite control entries: the start position is halved, a fixed offset is added to the stop position, and a bit field derived from a second stored value is merged in.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `store_arguments_and_run_helpers`: Saves the incoming address and two register values into shared variables, then calls three helpers in order.
- `poll_memory_bit`: Polls a specific bit within a byte at an offset from a pointer until that bit is cleared.
- `setup_blitter_rect_copy_2`: Reads two corner coordinate pairs from shared storage, computes the blit width in words and the height in rows from their differences, sets a full-mask destination-only blit with a modulo from the width, and starts it using an address supplied by a helper.
- `compute_blit_dimensions`: Calculates blit width and height from coordinate pairs, sets up blitter registers with modulo values and control words, then starts the blit operation.
- `project_points_scaled_to_screen_2`: Walks a table of points, scales each coordinate pair by a stored factor shifted right by ten bits, adds fixed centre offsets of 160 and 128, stores the results and calls a helper for each point.
- `iterate_distance_calculations`: Loops through a table of indices, fetches two 16-bit values from a global structure using each index multiplied by 4, and calls another routine to perform calculations with those values and a value from the table.
- `reset_default_clip_rect_2`: Stores four default bounds values, 160, 160, 128 and 128, into a shared rectangle record.
- `expand_bounds_to_include_point`: Widens a shared bounding rectangle so that it includes the given point, lowering the minimum and raising the maximum for both coordinates.
- `setup_blitter_line_octant_2`: Sets up the blitter for a line between two points: computes the start address from the first point, the step counts from the coordinate differences, a line-octant code from their signs and comparisons, and the control word, then stores them for the blit.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `update_audio_parameters`: Calls another routine, copies a pointer to a global variable, loops to copy data from a table into a destination buffer, sets several audio volume registers to zero, and sets a bit in a hardware register.
- `check_and_initialize_audio`: Clears a 16-bit word in a global variable, increments a byte in a table, and if the byte is not less than a comparison value, clears another byte in a table and calls another routine if a byte at a specific location is non-zero.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `initialize_audio_channel_periods`: Sets the period registers for all four Amiga audio channels by calling a subroutine with specific data structures passed in a6.
- `initialize_audio_channels_3`: Calculates a value by fetching a byte from a table using an index, shifts it, and adds a constant, then clears a 16-bit word in a global variable and calls initialize_audio_channels four times with different hardware register addresses.
- `update_entity_attributes_and_dispatch`: Loads a pointer from a table into a6, extracts a type index from the bytes at offsets 2 and 0, and uses this index to fetch new attribute data from a table pointed to by a3.
- `update_structure_and_dispatch_command`: Calculates a new address by adding a value to a pointer in a structure, stores it at offsets 10 and 36, and updates a field from a table.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_3`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_hardware_and_dispatch_command`: Searches a table to find a specific entry, updates a structure with new values, modifies the DMA controller configuration, and then branches to a command dispatcher based on a masked byte from the structure.
- `initialize_audio_channels`: Sets up audio channel registers by writing values from a table to DMA control and audio registers, then performs a delay loop before returning.
- `run_channel_tick_effects`: Applies the per-tick effect of an audio channel from the effect nibble stored in its row: a slide or note change, a volume slide, a retrigger or a delay, then returns the new period.
- `frag_move_rts`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `process_table_index`: Divides a byte from a table by 3, swaps the result, and branches based on whether the value is 0 or 2, otherwise it performs a bit shift on a byte from a structure.
- `frag_moveq_move_andi_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `fetch_table_entry`: Calculates an offset into a table using a byte from a structure, fetches a word from that table, and calls another routine to store it at an offset from a5.
- `frag_move_rts_2`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `update_structure_value_subtraction`: If a global variable is zero, it sets it to 255, subtracts a masked byte from offset 3 of a structure from a word at offset 16, and potentially clamps the result to a minimum value.
- `update_structure_value_addition`: If a global variable is zero, it sets it to 255, adds a masked byte from offset 3 of a structure to a word at offset 16, and potentially clamps the result to a maximum value.
- `search_table_value`: Searches a table for a word that matches a value from a structure, then stores that word at offset 24 of the structure, clears a byte at offset 22, and optionally sets a byte at offset 22 to 1 if the value is less than a comparison value at offset 16.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a6, then returns.
- `frag_move_beq_clr`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then copies d0 to the memory at an offset from a6, then clears the memory at an offset from a6.
- `update_accumulator_value`: If a word at offset 24 of a structure is not zero and a byte at offset 22 is zero, it adds a byte from offset 23 to the word at offset 16, then clamps the value at offset 16 to the value at offset 24 and clears offset 24 if the limit is reached.
- `decrement_channel_period_and_lookup`: Subtracts the input value from the channel period and clamps it to the target when reached.
- `merge_nibbles_into_byte`: Merges the low nibble of a byte from offset 3 of a structure into the low nibble of a byte at offset 26 of a structure, and merges the high nibble of the same source byte into the high nibble of the destination byte.
- `calculate_scaled_offset`: Calculates a value by shifting and masking bytes from a structure, then stores the result at an offset from a5.
- `frag_tst_bpl_move_sub_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `frag_move_andi_mulu_lsr_tst_bmi_add_bra`: Short code fragment that copies an indexed entry from a4 to d2, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then multiplies d2 by d0 (unsigned), then shifts d2 right by 7, then copies the memory at an offset from a6 to d0, then tests the memory at an offset from a6, then branches if the last result was negative, then adds d2 to d0, then jumps to another point in the code.
- `subtract_and_advance_phase`: Subtracts d2 from d0 and stores the result in offset 6 of a structure in a5, then advances a phase byte in the structure by an amount taken from the high nibble of a neighbouring byte.
- `frag_bsr_bra_4`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_5`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_scaled_byte_value`: Merges nibbles from a byte at offset 3 into a byte at offset 28, then calculates a scaled value using a table and a byte from offset 29, and stores the result at offset 8 of a structure.
- `frag_tst_bpl_move_sub_bra_2`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_3`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `multiply_and_shift_byte`: Multiplies a byte from a structure by a masked byte from another location, shifts the result, and adds it to a byte from a structure if a bit is set.
- `clamp_and_store_value`: Clamps the signed difference between d2 and d0 to a range between zero and sixty-four, stores the result as a word at offset 8 of a structure in a5, and updates a byte at offset 29 of a structure in a6 by adding the high nibble of the byte at offset 28 shifted right by two.
- `update_accumulated_value`: Checks a byte from a structure, and if non-zero, updates a byte at offset 20 and performs arithmetic involving a bit-shifted value and a byte at offset 8.
- `frag_move_rts_3`: Short code fragment that copies 1 to the memory at an offset from a6, then returns.
- `increment_clamped_byte`: Extracts the high nibble of a byte from a structure, adds it to a byte at offset 19 with a maximum limit of 64, and stores the result in a word at offset 8 of a5.
- `decrement_and_clamp_byte`: Subtracts the low nibble of the byte at offset 3 of the structure in a6 from the byte at offset 19, clears that byte to zero when the result is negative, and stores the resulting byte as a word at offset 8 of the structure in a5.
- `frag_move_subq_lea_clr_st_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then subtracts 1 from d0, then loads the address of a location in the code into a4, then copies d0 to the memory at a4, then loads the address of a location in the code into a4, then clears the memory at a4, then loads the address of a location in the code into a4, then sets the memory at a4 to all ones, then returns.
- `clamp_byte_and_store`: Reads a byte from offset 3 of the structure in a6, clamps it to a maximum of sixty-four, stores the clamped value as a byte at offset 19 of that structure, and stores it as a word at offset 8 of a structure in a5.
- `calculate_index_and_store`: Calculates a value based on the high and low nibbles of a byte from a structure and stores the result in a global variable if it is less than 63.
- `frag_move_beq_lea_clr_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then loads the address of a location in the code into a4, then clears the memory at a4, then loads the address of a location in the code into a4, then copies d0 to the memory at a4, then returns.
- `dispatch_command_byte`: Reads a byte from offset 2 of a structure in a6, masks the lower nibble, and branches to different subroutines based on the resulting value.
- `frag_moveq_move_lea_movea_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then copies a6 to the stack, then loads the address of a location in the code into a6, then copies d0 to the memory at a6, then copies the top of the stack to a6, then returns.
- `dispatch_structure_operation_by_nibble_3`: Reads a byte from an offset of 3 in a structure pointed to by a6, extracts the high nibble, and branches to a specific handler routine based on that value.
- `frag_bset_rts`: Short code fragment that sets bit 1 of a fixed memory address, then returns.
- `mask_and_merge_byte`: Masks the low nibble of a byte at offset 31 from a pointer in a6 and then merges the low nibble of a byte at offset 3 from the same pointer into it.
- `clear_low_nibbles_of_byte`: Clears the low nibble of a byte at offset 30 from an address in a6 and then ORs in the low nibble of another byte from the same base.
- `store_masked_byte`: The routine reads a byte from an offset in a structure pointed to by a register, masks it with a constant, and stores the result into a different offset in the same structure.
- `update_structure_byte`: Checks a global byte and if it is non-zero, decrements a byte at offset 34 of the structure in a6 and stores a byte from offset 33 of the structure into another global variable; otherwise, it stores a bit-shifted version of a byte from offset 3 of the structure into a global variable.
- `frag_move_bra_4`: Short code fragment that copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_lsr_rts`: Short code fragment that copies a location in the code to d0, then shifts d0 right by 4, then copies d0 to the memory at an offset from a6, then returns.
- `mask_and_merge_nibble`: Masks the low nibble of a byte at offset 30 from a pointer in a6 and merges the high nibble from a byte at offset 3 of the same pointer, storing the result back at offset 30.
- `decode_and_copy_title_data`: Reads a nibble from a table entry and, if a global variable and the entry satisfy a modulo condition, copies two 32-bit words and one 16-bit word from the table through a pointer, performs two delay loops, writes a modified value to a global variable, and then copies two more 32-bit words from the table.
- `frag_lea_tst_bne_moveq_move_andi_bra`: Short code fragment that loads the address of a location in the code into a4, then tests the memory at a4, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_lea_tst_bne_moveq_move_andi_bra_2`: Short code fragment that loads the address of a location in the code into a4, then tests the memory at a4, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_moveq_move_andi_cmp_bne_clr_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a location in the code, then branches if the last result was not zero, then clears the memory at an offset from a6, then copies 0 to the memory at an offset from a5, then returns.
- `frag_moveq_move_andi_cmp_bne_beq_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a location in the code, then branches if the last result was not zero, then copies the memory at a6 to d0, then branches if the last result was zero, then copies d1 to the stack, then jumps to another point in the code.
- `increment_and_store_byte`: Checks a global byte and if it is non-zero, takes a byte from offset 3 of the structure in a6, masks it, increments it, and stores the result into a global variable; otherwise, it returns.
- `frag_lea_tst_bne_move_andi_lsl_or_beq`: Short code fragment that loads the address of a location in the code into a4, then tests the memory at a4, then branches if the last result was not zero, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then shifts d0 left by 4, then masks the memory at an offset from a6 with 15, then merges d0 into the memory at an offset from a6, then tests d0, then branches if the last result was zero.
- `update_animation_frame`: Reads a nibble from offset 31 of a structure, uses it as an index into a table to increment a byte at offset 35, and if the high bit of that byte is set, resets it to zero and updates a pointer at offset 36 by either incrementing it or wrapping it to a new address based on a comparison with a value derived from offsets 10 and 14.

### Caesar Deluxe (1993)(Impressions)[cr ATX](Disk 2 of 2)[FD].adf:Anthrox#unpacked

- `initialize_graphics_and_copper_lists_2`: Checks the attention flags to decide whether to enter supervisor mode, opens a library, and disables interrupts and DMA. It then initializes several copper lists and sprite control registers, populates a table with encoded position words, and enters a loop that updates a value in a global variable based on a counter and the status of a joystick button.
- `swap_buffer_pair_store_offset_3`: Calls four helpers, stores a value 350 bytes past the second saved buffer address into a shared pointer, then swaps the two saved buffer addresses.
- `update_table_pointers`: If a value from a global variable is between 1 and 9, it decrements a global variable and calls another routine; otherwise, if the value is greater than 16, it extracts four bytes from a table and stores them as four longwords in another location.
- `swap_buffer_pair_back_offset_3`: Calls two helpers, stores a value two bytes before the second saved buffer address into a shared pointer, then swaps the two saved buffer addresses.
- `pass_through_helper_call`: Calls one helper routine and returns, with no other work of its own.
- `store_scaled_position_pair_3`: Calls a helper, stores the incoming position shifted right by three bits into one shared variable, calls another helper, then stores the position shifted right by six bits into a second shared variable.
- `combine_and_mask_coordinate_pair_3`: Adds two word values from a structure at offsets 8 and 12 to the structure's base address, masks the results to clear the lowest bit, and returns the modified values.
- `frag_movea_suba_bra_2`: Short code fragment that copies a location in the code to a0, then subtracts 4096 from a0, then jumps to another point in the code.
- `configure_blitter_registers_4`: Writes values to several blitter control and size registers and waits for the blitter to become idle.
- `handle_interrupt_logic`: Checks bit 5 of the custom interrupt request register; if set, it calls another routine, updates a global variable, and performs conditional logic involving several other global variables and branches.
- `frag_bsr_movem_move_rte_2`: Short code fragment that calls another routine, then restores saved registers from the stack, then copies 32 to a fixed memory address, then returns from the exception.
- `increment_and_check_counter`: Calculates two memory addresses based on a global variable and a constant, calls another routine twice at those addresses, increments a global variable, and if it reaches 110, sets another global variable to -2.
- `setup_blitter_parameters`: Calls another routine, sets a global variable to all ones, sets another global variable to sixteen million, clears the blitter mode register, sets the blitter depth to the value in a0, sets the blitter size to one hundred fifty-one, and returns.
- `frag_subq_bne_bra_2`: Short code fragment that subtracts 1 from the memory at a0, then branches if the last result was not zero, then jumps to another point in the code.
- `wait_for_raster_line_3`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_blitter_idle_3`: Spins while the blitter busy flag in the DMA control status register is set, then returns.
- `reset_text_stream_state_3`: Resets a text stream: points the read position at a fixed start address, zeroes the column counter, sets the line counter to one and clears a byte in shared memory.
- `process_text_stream`: Reads a byte from a global pointer, and depending on its value, either increments the pointer, resets certain counters, or calculates a destination address for a blitter transfer and calls another routine.
- `frag_lea_move_addq_cmpi_beq_bra_2`: Short code fragment that loads the address of a location in the code into a1, then copies a0 to the memory at a1, then loads the address of a location in the code into a5, then adds 1 to the memory at a5, then compares the memory at a5 with 41, then branches if the last result was zero, then jumps to another point in the code.
- `frag_lea_clr_addq_bra_2`: Short code fragment that loads the address of a location in the code into a5, then clears the memory at a5, then adds 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_lea_addq_tst_beq_move_rts_2`: Short code fragment that loads the address of a location in the code into a1, then adds 1 to a0, then tests the memory at a0, then branches if the last result was zero, then copies a0 to the memory at a1, then loads the address of a location in the code into a0, then copies 450 to the memory at a0, then returns.
- `frag_lea_adda_move_rts_2`: Short code fragment that loads the address of a location in the code into a0, then adds 5916 to a0, then loads the address of a location in the code into a1, then copies a0 to the memory at a1, then loads the address of a location in the code into a0, then copies 450 to the memory at a0, then returns.
- `frag_lea_move_addq_rts_2`: Short code fragment that loads the address of a location in the code into a5, then copies 0 to the memory at a5, then adds 1 to the memory at an offset from a5, then loads the address of a location in the code into a1, then copies a0 to the memory at a1, then returns.
- `setup_blitter_transfer_2`: Configures several blitter registers including source, destination, and control words using values from a1 and a3, then conditionally modifies the control register and destination address based on bits in the registers.
- `frag_move_btst_bne_rts_2`: Short code fragment that copies an address to the memory at an offset from a6, then tests bit 0 of d1, then branches if the last result was not zero, then copies 32768 to the memory at an offset from a6, then copies 1994 to the memory at an offset from a6, then copies 2050 to the memory at an offset from a6, then returns.
- `frag_lea_movea_adda_bra_2`: Short code fragment that loads the address of a location in the code into a0, then copies a location in the code to a1, then adds 1280 to a1, then jumps to another point in the code.
- `frag_lea_movea_adda_bsr_2`: Short code fragment that loads the address of a location in the code into a0, then copies a location in the code to a1, then adds 1280 to a1, then calls another routine, then loads the address of a location in the code into a0, then copies a location in the code to a1, then adds 1792 to a1.
- `copy_structured_data_block`: Initializes two memory locations with constant values, then loops 64 times to copy pairs of words from a source buffer in a0 to a destination buffer in a1, including an offset of 64 words for the second word of each pair.
- `set_copper_list_pointers_2`: Stores the high and low words of four list addresses, each a shared base address plus a fixed offset, into shared pointer variables, and sets a control constant.
- `encode_sprite_control_words`: Takes a starting position and an ending position from global variables, halves them, and writes them to two separate memory structures at specific offsets.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `store_arguments_and_run_helpers_2`: Saves the incoming address and two register values into shared variables, then calls three helper routines in sequence.
- `poll_memory_bit_2`: Polls a specific bit within a byte at an offset from a pointer until that bit is cleared.
- `compute_blit_parameters`: Reads coordinate pairs from a table, calculates blit dimensions and modulo, then configures custom blitter registers with these values.
- `compute_blit_dimensions_2`: Calculates blitter width and height from coordinate pairs, sets up blitter registers with modulo values and control words, then starts the blit operation.
- `scale_and_store_coordinates`: It iterates through a list of values, scales each value by a factor derived from a global variable using a bit shift, and stores the results into a global structure.
- `iterate_distance_calculations_2`: Loops through a table of indices, fetches two 16-bit values from a global structure using each index multiplied by 4, and calls setup_blitter_line_octant_2 to perform calculations with those values and a third value from the table.
- `reset_default_clip_rect_3`: Stores four default bounds values, 160, 160, 128 and 128, into a shared rectangle record.
- `update_coordinate_bounds`: Compares two 16-bit values against four stored values in a global structure and updates them if the input values are outside the current bounds.
- `calculate_distance_metrics`: Performs arithmetic calculations using two input values to update several global variables and a value in a memory table based on the difference between the inputs.
- `frag_bra_8`: Short code fragment that jumps to another point in the code.
- `frag_bra_9`: Short code fragment that jumps to another point in the code.
- `frag_bra_10`: Short code fragment that jumps to another point in the code.
- `manage_audio_state`: Increments a byte in a table, clears a 16-bit word in a global variable, and if the byte is not less than a comparison value, clears another; it also sets several audio registers and enables DMA.
- `increment_audio_index`: Increments a byte in a table, clears a 16-bit word in a global variable, and if the byte is not less than a comparison value, clears another; otherwise it returns.
- `frag_bsr_bra_6`: Short code fragment that calls another routine, then jumps to another point in the code.
- `initialize_audio_channel_periods_2`: Sets the period registers for all four Amiga audio channels by calling a subroutine with specific data structures passed in a6.
- `initialize_audio_channels_2`: Calculates a period value from a table and a global variable, clears a 16-bit word in a global variable, and calls another routine four times to set up audio channel registers.
- `parse_audio_command_and_update_structure`: Loads a pointer from a table using a value derived from bytes at offsets 2 and 0 of a structure in a6, then populates several fields in that structure and a second structure in a5 using data from a table pointed to by a3.
- `update_entity_attributes_and_dispatch_2`: Calculates a new address by adding a value to a pointer, stores it in two locations within a structure, and updates a field using a value from a table.
- `frag_bsr_bra_7`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_8`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_hardware_and_dispatch_command_2`: Searches a table to find a value based on a masked byte from a structure, updates the DMA controller and other hardware registers, and then branches to a subroutine based on the lower nibble of a byte in the structure.
- `configure_audio_channels_and_delay`: Performs two delay loops, enables DMA by setting a bit in the DMA control register, and populates the four audio channel address and length registers using values from a table.
- `dispatch_structure_operation_by_nibble`: Extracts a nibble from a byte at offset 2 of a structure in a6 and branches to different handlers based on its value, or copies a word from offset 16 of a6 to offset 6 of a5 if the nibble is 7 or 10.
- `frag_move_rts_4`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `process_table_index_2`: Divides a byte from a table by 3, swaps the result, and branches based on whether the value is 0 or 2, otherwise it performs a bit shift on a byte from a structure.
- `frag_moveq_move_andi_bra_2`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra_5`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `search_table_for_value`: Iterates through a table to find a word that is less than or equal to a value in a register, then calls another routine to copy a value to an offset from a6.
- `frag_move_rts_5`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `decrement_audio_period`: Decrements a 16-bit value in a structure by a nibble extracted from a byte at offset 3 of a6, then clamps the result to a minimum value and stores it at offset 6 of a5.
- `increment_audio_period`: Increments a 16-bit value in a structure by a nibble extracted from a byte at offset 3 of a6, then clamps the result to a maximum value and stores it at offset 6 of a5.
- `update_audio_note`: Searches a table for a value matching a 12-bit field in a6, then updates offset 24 of a6 with a value from the table and sets offset 22 of a6 based on a comparison.
- `frag_clr_rts_2`: Short code fragment that clears the memory at an offset from a6, then returns.
- `frag_move_beq_clr_2`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then copies d0 to the memory at an offset from a6, then clears the memory at an offset from a6.
- `adjust_audio_period_clamped`: Adds or subtracts a byte from a 16-bit value in a structure and clamps the result to a target value stored in another offset of the structure.
- `decrement_channel_period_and_lookup_2`: Subtracts the input value from the channel period and clamps it to the target when reached.
- `merge_nibbles_into_byte_2`: Merges nibbles from a byte at offset 3 of a6 into a byte at offset 26 of a6.
- `calculate_scaled_offset_and_update_index`: Extracts an index and a mode from a structure in a6, then calculates a value by either fetching from a table, performing a bitwise shift, or setting a constant, finally adding this to a value at offset 16 and updating a byte at offset 27 of the structure.
- `frag_tst_bpl_move_sub_bra_3`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_6`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `scale_and_add_period`: Multiplies a byte from a table by a nibble from a6, shifts the result, and adds it to a 16-bit value in a6 if a sign bit is not set.
- `subtract_and_advance_phase_2`: Subtracts d2 from d0 and stores the result in offset 6 of a structure in a5, then advances a phase byte in the structure by an amount taken from the high nibble of a neighbouring byte.
- `frag_bsr_bra_9`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_10`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_scaled_volume`: Updates a byte at offset 28 of a6 by merging nibbles, then calculates a scaled value and stores it at offset 8 of a5.
- `frag_tst_bpl_move_sub_bra_4`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_7`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `multiply_and_shift_byte_2`: Multiplies a byte from a structure by a masked byte from another location, shifts the result, and adds it to a byte from a structure if a bit is set.
- `clamp_and_store_value_2`: Clamps the signed difference between d2 and d0 to a range between zero and sixty-four, stores the result as a word at offset 8 of a structure in a5, and updates a byte at offset 29 of a structure in a6 by adding the high nibble of the byte at offset 28 shifted right by two.
- `update_accumulated_value_2`: Checks a byte from a structure, and if non-zero, updates a byte at offset 20 and performs arithmetic involving a bit-shifted value and a byte at offset 8.
- `frag_move_rts_6`: Short code fragment that copies 1 to the memory at an offset from a6, then returns.
- `increment_clamped_byte_2`: Extracts the high nibble of a byte from a structure, adds it to a byte at offset 19 with a maximum limit of 64, and stores the result in a word at offset 8 of a5.
- `decrement_and_clamp_byte_2`: Subtracts the low nibble of the byte at offset 3 of the structure in a6 from the byte at offset 19, clears that byte to zero when the result is negative, and stores the resulting byte as a word at offset 8 of the structure in a5.
- `decrement_and_store_bytes`: Copies a byte from offset 3 of the structure in a6 to d0, subtracts 1 from d0, then stores d0, zero, and 255 into three consecutive memory locations relative to the program counter.
- `clamp_byte_and_store_2`: Reads a byte from offset 3 of the structure in a6, clamps it to a maximum of sixty-four, stores the clamped value as a byte at offset 19 of that structure, and stores it as a word at offset 8 of a structure in a5.
- `calculate_and_store_index`: Extracts a byte from offset 3 of the structure in a6, calculates a value based on its nibbles, and if the result is less than 63, stores that value and 255 into two consecutive memory locations relative to the program counter; otherwise it branches to another routine.
- `frag_move_beq_lea_clr_rts_2`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then loads the address of a location in the code into a4, then clears the memory at a4, then loads the address of a location in the code into a4, then copies d0 to the memory at a4, then returns.
- `dispatch_structure_operation_by_nibble_2`: Extracts a nibble from a byte at offset 2 of a structure in a6 and branches to various handler routines based on the value.
- `frag_moveq_move_lea_movea_rts_2`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then copies a6 to the stack, then loads the address of a location in the code into a6, then copies d0 to the memory at a6, then copies the top of the stack to a6, then returns.
- `dispatch_structure_command_nibble`: Extracts a nibble from a byte at offset 3 of a structure in a6 and branches to a specific handler based on its value.
- `frag_bset_rts_2`: Short code fragment that sets bit 1 of a fixed memory address, then returns.
- `mask_and_merge_byte_2`: Masks the low nibble of a byte at offset 31 from a pointer in a6 and then merges the low nibble of a byte at offset 3 from the same pointer into it.
- `clear_low_nibbles_of_byte_2`: Clears the low nibble of a byte at offset 30 from an address in a6 and then ORs in the low nibble of another byte from the same base.
- `store_masked_byte_2`: The routine reads a byte from an offset in a structure pointed to by a register, masks it with a constant, and stores the result into a different offset in the same structure.
- `process_effect_nibble`: Checks a global byte and, if zero, extracts a nibble from offset 3 of the structure in a6; if that nibble is non-zero, it decrements the byte at offset 34 of the structure in a6 and, if the result is zero, stores a byte from offset 33 of the structure into a global variable.
- `frag_move_bra_8`: Short code fragment that copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_lsr_rts_2`: Short code fragment that copies a location in the code to d0, then shifts d0 right by 4, then copies d0 to the memory at an offset from a6, then returns.
- `mask_and_merge_nibble_2`: Masks the low nibble of a byte at offset 30 from a pointer in a6 and merges the high nibble from a byte at offset 3 of the same pointer, storing the result back at offset 30.
- `decode_and_copy_title_data_2`: Reads a nibble from a table entry and, if a global variable and the entry satisfy a modulo condition, copies two 32-bit words and one 16-bit word from the table through a pointer, performs two delay loops, writes a modified value to a global variable, and then copies two more 32-bit words from the table.
- `frag_lea_tst_bne_moveq_move_andi_bra_3`: Short code fragment that loads the address of a location in the code into a4, then tests the memory at a4, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `extract_nibble_and_branch`: Tests a global byte and, if zero, extracts the low nibble from offset 3 of the structure in a6 before branching to another routine.
- `frag_moveq_move_andi_cmp_bne_clr_rts_2`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a location in the code, then branches if the last result was not zero, then clears the memory at an offset from a6, then copies 0 to the memory at an offset from a5, then returns.
- `compare_nibble_and_branch`: Extracts the low nibble from offset 3 of the structure in a6, compares it to a global byte, and if they are equal and the word at offset 0 of the structure in a6 is non-zero, pushes d1 to the stack and branches to another routine.
- `increment_and_store_byte_2`: Checks a global byte and if it is non-zero, takes a byte from offset 3 of the structure in a6, masks it, increments it, and stores the result into a global variable; otherwise, it returns.
- `update_nibble_field`: Checks a global byte and, if zero, extracts the low nibble from offset 3 of the structure in a6, shifts it left by 4, and performs an OR operation with the byte at offset 31 of the structure in a6.
- `clear_sprite_entry_2`: Reads a nibble from offset 31 of the structure in a6, uses it as an index into a table to add a value to offset 35 of the structure in a6; if bit 7 of that byte is set, it clears the byte, calculates a new pointer using values at offsets 10 and 14 of the structure in a6, updates the pointer at offset 36, and toggles the sign of the byte at the new pointer location.
