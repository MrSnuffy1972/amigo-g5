# Game 2907 (blade warrior (1989)(imageworks)[cr flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02907). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902016 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 22 of 22 named (0 author symbols), 22 of 22 described (3 by rule, 12 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 17 of 17 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 503 of 503 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 1240 |
| data | 1703 |
| padding | 1 |
| unused | 899072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Blade Warrior (1989)(ImageWorks)[cr FLT].adf:bootblock | 1 | 98 | 6 |
| logged disk load from Blade Warrior (1989)(ImageWorks)[cr FLT].adf boot-block load | 1 | 163 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 242 | 14 |

## Cases handled

- blade warrior (1989)(imageworks)[cr flt]/Blade Warrior (1989)(ImageWorks)[cr FLT].adf: the boot block reads 20480 bytes from disk offset 861184 to an address (found statically); recovered as a unit
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- blade warrior (1989)(imageworks)[cr flt]/Blade Warrior (1989)(ImageWorks)[cr FLT].adf: 1 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- `copy_and_jump` (bootblock): Copies data from one memory location to another using a loop that processes four bytes at a time, then jumps to a fixed address in memory after the copy operation completes. Kind: disk hardware check. Pass path: ADKCON register setup and track validation.
- `initialize_interrupt_and_dma_tables` (runtime segment an address): Configures custom hardware registers including DMA and interrupt control, sets up function pointers for interrupts, and writes a sequence of values into a table starting at a hardware register offset. Kind: disk hardware check. Pass path: The routine initializes DMA and interrupt tables, configures CIAB timer A for a specific timing, and then waits for a vertical blank interrupt (VBL) by polling CUSTOM_VPOSR. If the check passes (i.e., the expected VBL timing is observed), i.

## Functions


### Blade Warrior (1989)(ImageWorks)[cr FLT].adf:bootblock

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `setup_and_execute_io`: Configures an I/O request structure with specific command, data pointer, and status values before executing a system call to perform the I/O operation.
- `copy_and_jump`: Copies data from one memory location to another using a loop that processes four bytes at a time, then jumps to a fixed address in memory after the copy operation completes.
- `copy_and_clear_colors`: Copies data from one memory location to another using a loop that processes four bytes at a time, then clears color registers by writing a decrementing value starting from 65535.
- `setup_io_request`: Sets up an I/O request structure with specific values for command, data pointers, and status fields before calling the operating system's DoIO function.
- `configure_io_and_execute`: Configures an I/O request structure with predefined values for buffer addresses and command type, then calls DoIO and executes a function at a fixed address.

### logged disk load from Blade Warrior (1989)(ImageWorks)[cr FLT].adf boot-block load

- `rotate_and_store`: Loads a long word from memory, extracts and stores its low byte, writes the word to another location, sets the carry flag, and rotates the original value right through the carry flag.
- `rotate_and_extract_bits`: Processes a long word by rotating its bits and extracting individual bytes while maintaining carry flag state across multiple iterations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_loop_and_restore`: Processes a loop that decrements counters and clears flags, then calls other routines to handle state changes before restoring custom registers and returning from the interrupt.
- `frag_eori_bne_addq_lea_moveq_move_adda`: Short code fragment that exclusive-ors 1 into the memory at an offset from a5, then branches if the last result was not zero, then adds 2 to the memory at an offset from a5, then loads the address of the memory at an offset from a6 into a0, then loads 3 into d1, then copies a location in the code to d0, then copies d0 to the memory at a0, then adds 16 to a0, then loops back while counting down d1, then returns.
- `toggle_and_decrement_buffer`: Toggles a bit in a global variable and, if the result is zero and another condition is met, decrements a counter and fills a buffer with a constant value using a loop.
- `split_nibbles_into_table`: Calls a routine 32 times, then iterates over a table of 32 words, splitting each word into four-bit nibbles and storing them with their negated values into a destination structure.
- `frag_moveq_move_andi_cmp_bgt_lsr_dbra_r`: Short code fragment that loads 2 into d3, then copies d0 to d1, then masks d1 with 15, then compares d2 with d1, then branches if the comparison was greater, then copies d1 to d2, then shifts d0 right by 4, then loops back while counting down d3, then returns.
- `frag_subq_beq_rts`: Short code fragment that subtracts 1 from the memory at an offset from a5, then branches if the last result was zero, then returns.
- `process_table_bits`: Processes data in tables by iterating through multiple levels, performing bit operations on values and updating pointers based on specific conditions.
- `process_nested_loops`: Executes nested loops that manipulate values in tables and increment pointers based on specific conditions, performing bit operations and conditional updates.
- `decrement_table_values`: Decrements a 16-bit value in a global variable and, if it reaches zero, iterates through a table decrementing certain values based on bit pattern checks.
- `initialize_interrupt_and_dma_tables`: Configures custom hardware registers including DMA and interrupt control, sets up function pointers for interrupts, and writes a sequence of values into a table starting at a hardware register offset.
- `build_copper_style_word_table`: Fills a global table with four groups of word pairs holding descending or incrementing values together with the high and low halves of a fixed address, then writes 80 words of incrementing values alternated with zeros; it stores the table pointer and final counter into the structure addressed by a6.
- `restore_saved_custom_registers`: Writes the disable-everything mask to the DMA and interrupt enable registers relative to A6, then restores previously saved values into those registers and copies a pointer from a global structure into a data pointer slot.
- `lbl000352`: Writes the constant 255 to one variable and 32 to a global variable, which the listed facts associate with the interrupt request register.
- `frag_movem_move_lea_st_rte`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d0, then loads the address of a location in the code into a0, then sets the memory at a0 to all ones, then restores saved registers from the stack, then copies 8192 to a fixed memory address, then returns from the exception.
