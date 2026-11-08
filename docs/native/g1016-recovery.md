# Game 1016 (james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01016). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916592 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 39 of 39 named (0 author symbols), 39 of 39 described (1 by rule, 38 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 174 of 174 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 680 of 680 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11764 |
| code | 2298 |
| data | 66604 |
| padding | 854 |
| unused | 835072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf:bootblock | 1 | 29 | 3 |
| logged disk load from James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf boot-block load | 1 | 0 | 0 |
| logged disk load from James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf boot-block load | 1 | 97 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 554 | 34 |

## Cases handled

- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: file j1 (header 64832 bytes, blocks hold 11224): its blocks are judged as raw blocks outside the filesystem
- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: the boot block reads 5120 bytes from disk offset 749056 to an address (found statically); recovered as a unit
- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: the boot block reads 2560 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 174 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: 2 logged loads cover 0 bytes of the image
- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: blocks 748-757 (bytes 382976-388096) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.20)
- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: blocks 759-768 (bytes 388608-393728) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.73)
- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: blocks 770-779 (bytes 394240-399360) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.89)
- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: blocks 781-790 (bytes 399872-404992) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.52)
- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: blocks 792-801 (bytes 405504-410624) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.00)
- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: blocks 803-812 (bytes 411136-416256) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: blocks 814-823 (bytes 416768-421888) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.89)
- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: blocks 825-834 (bytes 422400-427520) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: blocks 836-845 (bytes 428032-433152) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.33)
- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: blocks 847-856 (bytes 433664-438784) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: blocks 858-879 (bytes 439296-450560) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.44)
- james pond - underwater agent (1990)(millennium)[cr pdx][t +2 afl]/James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf: blocks 883-883 (bytes 452096-452608) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.90)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf:bootblock

- `initialize_boot_sequence`: Calls a routine, sets three values in a structure pointed to by a1, enters a loop that calls a routine multiple times while updating structure offsets, and finally sets a global variable before triggering a trap.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `setup_io_and_call_routine`: Sets four values in a structure pointed to by a pointer in a global variable, calls a routine at an offset from that pointer, calls another routine, and then updates a global variable and a value at a specific memory location.

### logged disk load from James Pond - Underwater Agent (1990)(Millennium)[cr PDX][t +2 AFL].adf boot-block load

- `extract_bitstream_word`: Moves a longword from a pointer in a0 to d0, stores a byte from d0 into a location pointed to by a5, and returns a value derived from d0 after a logical shift right.
- `process_bitstream_data`: Performs a loop that shifts a value in d0 and calls a routine when d0 reaches zero, and contains logic to compare and copy bytes from a source to a destination based on a constant value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_pointer_to_memory`: Stores the value of a pointer in a1 into a memory location at offset 10e.
- `copy_color_data`: Copies 32 words from a source pointed to by a1 to both a destination pointed to by a0 and a second destination at offset ce in a loop.
- `run_unrecovered_tiny_leaf`: Clears a 16-bit value at offset 14c, then checks if it is zero; if it is not zero, it clears it again and returns, otherwise it enters an infinite loop.
- `process_interrupt_and_register`: Reads a byte from a memory location, sets bit 6 of another memory location, runs a loop 50 times, clears bit 6 of that location, and writes 8 to a hardware register.
- `set_stack_pointer`: Stores the value of a0 into a global variable, loads a value from a memory location into a0, and branches to another location.
- `set_stack_and_copy_register`: Sets the stack pointer to a0, loads a value from a memory location into a0, copies a hardware register value to a memory location, and branches to another routine.
- `process_conditional_logic`: Checks bit 4 of a value from a hardware register and bit 5 of another, then performs a series of bitwise operations on joystick and status data before calling various routines and returning via an interrupt return.
- `update_counter_with_overflow`: Multiplies a 32-bit value in a memory location by two and increments it if certain bits are set, otherwise it returns.
- `store_byte_to_memory`: Stores a byte into a memory location at offset 14e and returns.
- `update_audio_parameters`: Checks a global variable to determine which audio channel to configure, then loads parameters from a table, calls a routine to update them, and writes the results to the corresponding audio hardware registers and DMA control registers.
- `run_decrement_loop`: Runs a loop 512 times and then returns.
- `set_audio_channel_zero_sample`: Loads a pointer into the audio channel zero location register, sets the audio channel zero length register to sixteen, clears a 16-bit value at offset 544 of a structure, and returns.
- `set_audio_channel_one_sample`: Loads a pointer from a table into the audio channel one location register, sets the audio channel one length register to sixteen, clears a 16-bit value at a global memory location, and returns.
- `set_audio_channel_two_sample`: Sets the audio channel two location and length registers using a pointer to a table and then clears a 16-bit value at a global memory location before returning.
- `set_audio_channel_three_sample`: Sets the audio channel three sample pointer to a table via a pointer and sets its length to sixteen, then clears a 16-bit value in a global variable before returning.
- `disable_dma_and_set_volume`: Writes 1 to a hardware register and clears a 16-bit value in another hardware register before returning.
- `set_audio_volume_two`: Sets the first audio volume register to two and clears the second audio DMA control register, then returns.
- `set_audio_volume_four`: Sets the first audio volume register to four and clears the second audio DMA control register, then returns.
- `set_audio_volume_eight`: Sets the first audio volume register to eight and clears the second audio DMA control register, then returns.
- `test_audio_status_and_branch`: Performs eight logical OR operations on d0 with zero, tests a 16-bit value at offset 10132, and branches to another routine if the value is zero; otherwise it returns.
- `clear_all_audio_channels`: Clears four 16-bit values at offsets 872, 876, 880, and 884 of a structure, then calls six different routines in sequence.
- `process_audio_counters`: Checks a 16-bit value at offset 10132 and returns if zero; otherwise, it checks several 16-bit global variables, calls various audio routines, and decrements those variables if they are non-zero.
- `check_audio_status`: Tests a 16-bit value at offset 10100 and returns if it is zero, otherwise it returns immediately.
- `update_audio_parameters_2`: Loads a 16-bit value from a global variable, masks it, and branches based on the result; it may call another routine or write sixteen to a global variable at offset 870.
- `update_audio_configuration`: Compares a 16-bit value in d0 to two, and if not equal, writes sixteen to a global variable at offset 870 before branching.
- `select_audio_configuration`: Compares d0 to 3 and branches to another routine if not equal; then checks a 16-bit value at offset 362 against several constants, branching to another routine if any match, otherwise it writes 8 to offset 870 and branches to another routine.
- `set_audio_parameter_sixteen`: Writes 17 to a 16-bit value at offset 870 of a structure and branches to another routine.
- `set_audio_parameter_two`: Writes 2 to a 16-bit value at offset 870 of a structure and branches to another routine.
- `update_audio_channel_three_volume`: Checks a 16-bit value at offset 884 of a structure and a 16-bit value at offset 918 of a structure; if both conditions are met, it calls another routine, performs arithmetic on a value from offset 920, writes the result to the audio channel three volume register, and increments the value at offset 920.
- `update_on_value_change`: When a stored value is nonzero and differs from the incoming value, it stores the new value, calls two unnamed routines and sets a global to one.
- `store_data_to_stack`: Stores the value in d1 to the end of the stack pointed to by a2, then performs several memory writes and bitwise operations using d2, d3, d4, d5, and d6.
- `update_sprite_data_loop`: Checks a global variable and enters a loop that processes data from a table, updates sprite attributes and positions, and performs complex pointer arithmetic and table lookups before returning.
- `calculate_and_store_table_entry`: If two global variables meet specific conditions, it calculates a value based on a byte from a table, performs arithmetic, and stores a 16-bit result into a table at an offset derived from that calculation, then updates a pointer in a hardware register.
- `calculate_audio_data`: Performs complex pointer arithmetic and table lookups to write a 16-bit value to a memory location, while touching sprite-related hardware registers.
