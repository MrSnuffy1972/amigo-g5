# Game 1342 (venus the flytrap (1990)(gremlin)(disk 1 of 2)[cr miracle]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01342). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908832 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 36 of 36 named (0 author symbols), 36 of 36 described (1 by rule, 32 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 231 of 231 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 964 of 964 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 123439 |
| code | 3210 |
| data | 333443 |
| padding | 2276 |
| unused | 446464 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Venus the Flytrap (1990)(Gremlin)(Disk 1 of 2)[cr Miracle].adf:Type | 2 | 117 | 2 |
| Venus the Flytrap (1990)(Gremlin)(Disk 1 of 2)[cr Miracle].adf:Venus | 2 | 83 | 3 |
| Venus the Flytrap (1990)(Gremlin)(Disk 1 of 2)[cr Miracle].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 739 | 28 |

## Cases handled

- venus the flytrap (1990)(gremlin)(disk 1 of 2)[cr miracle]/Venus the Flytrap (1990)(Gremlin)(Disk 1 of 2)[cr Miracle].adf:00: high-entropy data file (7.87 bits/byte), classified as packed data
- venus the flytrap (1990)(gremlin)(disk 1 of 2)[cr miracle]/Venus the Flytrap (1990)(Gremlin)(Disk 1 of 2)[cr Miracle].adf:01: high-entropy data file (7.64 bits/byte), classified as packed data
- venus the flytrap (1990)(gremlin)(disk 1 of 2)[cr miracle]/Venus the Flytrap (1990)(Gremlin)(Disk 1 of 2)[cr Miracle].adf:02: high-entropy data file (7.62 bits/byte), classified as packed data
- runtime segment an address: 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 138 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- venus the flytrap (1990)(gremlin)(disk 1 of 2)[cr miracle]/Venus the Flytrap (1990)(Gremlin)(Disk 1 of 2)[cr Miracle].adf:Venus: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_complex_data` (runtime segment an address): Disables interrupts, modifies custom hardware registers, calls several routines to perform arithmetic and bitwise operations, updates a table with ASCII characters based on a calculated value, compares two strings, and restores the processor state before returning. Kind: disk hardware/track check. Pass path: the branch where sVar5 == -1 is reached after the string comparison loop, leading to u4__sub_1D80() and the return statement.

## Functions


### Venus the Flytrap (1990)(Gremlin)(Disk 1 of 2)[cr Miracle].adf:Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_register_and_branch`: Moves a longword from register d1 to register d0 and then branches to another routine.

### Venus the Flytrap (1990)(Gremlin)(Disk 1 of 2)[cr Miracle].adf:Venus

- `unpack_compressed_data`: Decompresses a bitstream into a destination memory buffer using a variable-length coding scheme.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Venus the Flytrap (1990)(Gremlin)(Disk 1 of 2)[cr Miracle].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `push_stack_pointer`: Pushes a longword from the stack onto the stack and moves a longword into a memory location at offset 32.
- `initialize_and_copy_memory`: Clears sixteen longwords starting at a hardware register address, then copies one thousand two hundred eighty longwords from a source table to a destination table starting at a fixed address, and finally jumps to the address stored at the destination.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_interrupt_status`: Saves registers and the status register, reads a longword from a hardware register, and if bit 5 is not set, writes a byte to a specific memory location before calling another routine.
- `restore_interrupt_context`: Restores the status register and all data and address registers from the stack, writes 112 to a hardware register, and executes an interrupt return.
- `initialize_graphics_and_copy_data`: Clears hardware registers, copies a block of memory, and performs multiple calls to subroutines that manipulate data structures.
- `call_subroutine`: Calls a subroutine at a specific longword address.
- `execute_no_operation`: Performs three no-operation instructions and proceeds into the subsequent code.
- `decrement_nibbles_in_table`: Calls another routine twice, then loops through a table of words to decrement the high nibble, middle nibble, and low nibble of each word if they are non-zero, and writes the results back to the table.
- `update_global_status_2`: If bit 5 of a hardware register is set, it writes a byte to a global variable, calls another routine, and then performs two loops that copy bytes from a table to three different memory locations based on values in global variables, finally updating those global variables and returning.
- `update_global_status`: Checks bit 5 of a hardware register, and if set, writes a byte to a global variable; otherwise it returns, and then restores the status register and registers before performing an interrupt return with 112 written to a hardware register.
- `clear_and_check_byte`: Clears a byte in a global variable and, if it is zero, returns; otherwise it returns.
- `store_address_to_memory`: Stores a long value into a memory location at offset 32.
- `update_hardware_and_status`: Sets a bit in a global variable, updates that variable and another with specific constant values, and stores a function pointer into a global variable.
- `decompress_bitstream_data`: Searches memory for a specific magic signature and then performs a bit-shifting decompression algorithm to reconstruct bytes from a packed bitstream into a destination buffer.
- `read_bit_stream_bits`: Pulls a count of bits from a packed stream into a shift register, reloading a longword from the input whenever the register empties.
- `configure_interrupt_hardware`: Sets specific byte values in the CIAA_PRA register and other memory locations, updates a global function pointer, and modifies the INTENA and custom chip interrupt registers.
- `save_register_context`: Stores a series of registers into a table in memory and performs conditional logic based on bits in a global variable.
- `process_complex_data`: Disables interrupts, modifies custom hardware registers, calls several routines to perform arithmetic and bitwise operations, updates a table with ASCII characters based on a calculated value, compares two strings, and restores the processor state before returning.
- `call_and_branch`: Calls another routine and then branches to another routine.
- `assign_routine_result`: Calls another routine and stores its return value into a global variable.
- `interleave_bit_data`: Performs bitwise interleaving of two data sources by masking bits and shifting them to create a single stream.
- `process_bitmask_loop`: Loads two long values from a table, performs bitwise AND, OR, and addition operations on them, and then enters a loop that executes 255 times to XOR a value from a table with d0 before comparing the result to a calculated value.
- `set_bit_and_branch`: Sets bit 1 of a byte at the address in a6, checks bit 4 of a byte at the address in a6, and branches to another routine if the bit is set; otherwise it returns.
- `toggle_status_bit`: Clears and then sets bit 0 of a byte at a pointer before branching to another routine.
- `decrement_and_loop`: Clears bit 1 of a byte at a pointer, adds a word to a memory location, and enters a loop that calls another routine while decrementing a value and potentially setting bit 1 of a byte.
- `write_and_wait_for_bit`: Stores a word into two separate memory locations and then waits in a loop until a specific bit in a third memory location is cleared.
- `clear_status_bits`: Performs bitwise OR and AND operations on a byte at a pointer and then clears bits 3 and 7 of that same byte before branching to another routine.
- `modify_status_bits`: Performs bitwise OR and AND operations on a byte at a pointer and then branches to another routine with a word value in d0.
- `load_word_from_table`: Loads a word from a table using an offset in d0 and an address in a0 into d0.
- `calculate_arithmetic_result`: Performs a series of arithmetic operations involving multiplication, bitwise AND, and addition on registers, then stores the result in a register before returning.
