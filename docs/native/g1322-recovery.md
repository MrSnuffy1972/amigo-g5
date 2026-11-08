# Game 1322 (warhead (1989)(activision)[cr pna]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01322). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906800 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 21 of 21 named (0 author symbols), 21 of 21 described (1 by rule, 20 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 234 of 234 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 523 of 523 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2798 |
| code | 2356 |
| data | 8204 |
| padding | 2 |
| unused | 893440 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Warhead (1989)(Activision)[cr PNA].adf:bootblock | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 500 | 20 |

## Cases handled

- runtime segment an address: 234 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- warhead (1989)(activision)[cr pna]/Warhead (1989)(Activision)[cr PNA].adf: blocks 11-21 (bytes 5632-11264) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Warhead (1989)(Activision)[cr PNA].adf:bootblock

- `allocate_and_run_io_block`: Saves the result of a system library call into a field of the structure pointed to by a1, then subtracts a constant from a value and aligns it down to a multiple of 256, storing the aligned result at offset 36.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `negate_table_entry`: Reads a 32-bit value from a table indexed by a register offset, negates it in place, and returns.
- `poll_input_and_dispatch`: Reads the joystick, mouse, and CIAA port to detect button presses and movement, then branches to the corresponding input handler based on the button state.
- `loop_global_variable_check`: Writes the value 255 to a global variable, then repeatedly reads that variable and returns while it is zero, otherwise writing 255 to it again.
- `calculate_offset_difference`: Reads a global variable and subtracts another global variable from it, then adds ten to the result when the difference is negative, leaving the value in d0.
- `increment_pointer_with_wrap`: Reads a byte from the address held in a global pointer and returns it in d0, then advances that pointer by one, resetting it to a fixed base address when it reaches the limit at offset 42.
- `copy_and_iterate_string`: Reads a global byte to compute an index, doubles it, and stores two 16-bit values from a table into global variables.
- `extract_and_or_nibble`: Reads a byte from a global variable to compute a base address in a table, then loops five times extracting a nibble from a source table and OR-ing it into eight words of a structure.
- `and_values_into_memory`: Bitwise-ANDs the value in d4 into the words at offsets 2380, 1576, 792 and the address in a1, then branches away; the following block does the same with the value in d3 before branching away.
- `mask_and_shift_structure`: Bitwise-ANDs the values in d4 and d1 into several fixed offsets of the structure pointed to by a1, then loops over the structure extracting bit fields from d1 and d2 and ORing the results into the same offsets, incrementing a global variable and returning when the counters run out.
- `jump_indirect_from_global`: Saves the processor registers on the stack, loads a pointer from a global variable into a0, and jumps to the routine whose address that pointer holds.
- `store_constant_and_return`: Stores the constant 5504 into a global variable and then returns immediately.
- `configure_audio_channel`: Selects one of four audio channels based on a bit in the VBLANK flag register, then configures that channel's data, length, period and volume registers and enables its DMA. When the channel's state holds a special marker it instead loads a sound pointer from a global variable and calls the sound handler before returning.
- `configure_audio_and_interrupts`: Writes values into the audio and interrupt hardware registers, enables interrupts, then loads a pointer from a global variable and performs an indirect call before restoring the registers and returning; several similar blocks follow, each configuring audio registers, calling the routine at the global pointer, and returning.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `or_byte_zero_d0`: Contains a single instruction that performs a bitwise OR of zero into d0, which leaves the register unchanged.
- `or_constants_into_d0`: Performs a bitwise OR of zero into d0, then ORs the byte values 40 and 64 into d0 and into the byte at offset 64 of the structure pointed to by a0.
- `decrement_global_variable`: Initializes a global variable to two thousand, then repeatedly decrements it by one and returns when it reaches zero.
- `configure_audio_channel_zero_2`: Tests a global byte at offset 25 and, when it is zero, loads the audio channel zero data register from a0, copies a value from a0 into the channel length register, stores a value from d0 into a global variable, and returns.
- `configure_audio_channel_zero`: Tests a global byte at offset 25; if nonzero it writes to the audio channel zero registers, enabling interrupts and DMA, then calls another routine to compute a period value and stores it.
- `configure_audio_channel_one`: Tests a global byte at offset 26; if nonzero it writes to the audio channel one registers, enabling interrupts and DMA, then calls another routine to compute a period value and stores it.
