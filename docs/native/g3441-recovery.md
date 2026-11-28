# Game 3441 (dragon ninja (1988)(imagine)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03441). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908688 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 36 of 36 named (0 author symbols), 36 of 36 described (8 by rule, 24 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 431 of 431 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1201 of 1201 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 661 |
| code | 5030 |
| data | 897363 |
| padding | 2 |
| unused | 5632 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dragon Ninja (1988)(Imagine)[cr QTX].adf:bootblock | 1 | 143 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 252 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 794 | 23 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 3 |

## Cases handled

- dragon ninja (1988)(imagine)[cr qtx]/Dragon Ninja (1988)(Imagine)[cr QTX].adf: custom-format (non-DOS) disk, 1747 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 154 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 269 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- dragon ninja (1988)(imagine)[cr qtx]/Dragon Ninja (1988)(Imagine)[cr QTX].adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.020, entropy 2.61)
- dragon ninja (1988)(imagine)[cr qtx]/Dragon Ninja (1988)(Imagine)[cr QTX].adf: blocks 22-1759 (bytes 11264-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.99)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Dragon Ninja (1988)(Imagine)[cr QTX].adf:bootblock

- `initialize_boot_data`: Initializes a block of memory at a fixed address with zero values, then populates it with pattern data from a table, setting up control structures for subsequent boot operations.
- `copy_pattern_data`: Copies 32 bytes of pattern data from a source buffer to multiple destination locations based on indexed offsets, handling special cases when certain bytes equal 255.
- `increment_counters`: Increments multiple counters stored at specific memory locations with varying step sizes, then branches to a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_and_dma`: Configures audio playback parameters and manages DMA channel settings, including updating sound registers and setting up coprocessor pointers for video display.
- `delay_loop`: Executes a nested loop structure that performs a total of 16 iterations, clearing a global variable and then decrementing a counter in each loop.
- `process_audio_data`: Processes audio command data by parsing characters, calculating offsets, and setting up sound registers for playback.
- `copy_palette_block_and_advance_script`: Copies eight longwords from a source pointer to the palette area, advances the palette pointer, examines a command byte, optionally writes the next byte to hardware, updates pointers and a flag, then branches.
- `write_timer_byte_and_finish_interrupt`: Writes the byte through a0 to timer hardware, advances a script pointer, restores a0 and a1, clears an interrupt bit, decrements a count, conditionally clears control, and returns from exception.
- `process_linked_list`: Processes a linked list by traversing nodes, copying data from one location to another, updating pointers, and performing memory operations on specific addresses.
- `frag_clr_move_rts`: Short code fragment that clears d0, then copies a fixed memory address to d0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_joystick_input_2`: Reads joystick input values from hardware, compares them with stored previous values, and updates global variables when changes occur.
- `read_joystick_input`: Reads joystick input from hardware registers, checks bit states, and combines multiple input conditions into a single return value representing button states.
- `update_position_value`: Updates a position value by reading from memory locations, performing arithmetic operations with signed byte values, and storing the result back to a global variable.
- `setup_audio_channel_0`: Configures audio channel 0 by setting DMA control registers, initializing sound parameters, and enabling audio playback with specific volume and period settings.
- `setup_audio_channel_1`: Configures audio channel 1 by setting DMA control registers, initializing sound parameters, and enabling audio playback with specific volume and period settings.
- `setup_audio_channel_2`: Configures audio channel 2 by setting DMA control registers, initializing sound parameters, and enabling audio playback with specific volume and period settings.
- `setup_audio_channel_3`: Configures audio channel 3 by setting DMA control registers, initializing sound parameters, and enabling audio playback with specific volume and period settings.
- `frag_movem_move_bra`: Short code fragment that saves registers on the stack, then copies a2 to a fixed memory address, then copies d2 to d5, then copies d1 to d2, then copies d3 to d1, then copies d5 to d3, then jumps to another point in the code.
- `setup_blit_dma`: Configures bit-block transfer (BLT) hardware registers for graphics operations using calculated dimensions and offsets, then waits for the operation to complete.
- `decode_dimensions_and_packed_size`: Reads and increments two dimensions from a0, skips four bytes, stores the data pointer in a3, computes twice their product in d5, and packs shifted width with height in d4.
- `blit_dma_setup`: Sets up bit-block transfer (BLT) operations by configuring various hardware registers including size, pointers, and control words, then waits for DMA completion before returning.
- `frag_move_btst_bne_rts`: Short code fragment that copies 33792 to a fixed memory address, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then returns.
- `update_dma_parameters`: Adjusts bit-block transfer hardware parameters based on a global variable's value, then configures and triggers multiple DMA operations with calculated offsets and dimensions.
- `configure_blit_dma_2`: Sets up bit-block transfer hardware registers with calculated dimensions and offsets, then waits for the operation to complete multiple times.
- `configure_blit_dma`: Sets up bit-block transfer hardware registers with calculated dimensions and offsets, then waits for the operation to complete multiple times.
- `copy_pattern_data_2`: Copies word-sized data from a source buffer to multiple destinations with fixed offsets, then advances pointers and repeats based on counters.
- `configure_blitter_copy_and_wait`: Computes and writes the blitter source pointer, loads the destination pointer, programs modulos, controls, masks and size, then branches to code that waits for blitter completion.
- `frag_move_lea_moveq_dbra_rts`: Short code fragment that copies a1 to a fixed memory address (2 times in a row), then loads the address of a fixed memory address into a0, then loads 15 into d1, then copies the memory at a1 (advancing a1) to d0, then copies d0 to the memory at a0 (advancing a0), then loops back while counting down d1, then returns.
- `frag_lea_moveq_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads 15 into d1, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d1, then returns.
- `frag_movea_move_clr_subq_bne_rts`: Short code fragment that copies a fixed memory address to a0, then copies 67200 to d0, then clears the memory at a0 (advancing a0), then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `setup_blitter`: Configures blitter registers for a graphics operation including source address, size, and control settings, then waits for completion.
- `handle_interrupt_input`: Reads interrupt status from a hardware register, checks a specific bit condition, and updates global variables accordingly before clearing the interrupt request.
- `handle_interrupts`: Reads interrupt status from a hardware register, checks specific bit conditions, and clears corresponding bits in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `bcd_adjust`: Performs binary-coded decimal adjustment on byte values using the add with carry instruction.
- `frag_clr_move_rts_2`: Short code fragment that clears a fixed memory address (2 times in a row), then copies 0 to a fixed memory address (4 times in a row), then copies 15 to a fixed memory address, then returns.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
