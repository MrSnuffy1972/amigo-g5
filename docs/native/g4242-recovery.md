# Game 4242 (leonardo (1989)(starbyte)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04242). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903728 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 35 of 35 named (0 author symbols), 35 of 35 described (19 by rule, 16 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 241 of 241 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 22528 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 556 of 556 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3428 |
| code | 4198 |
| data | 831590 |
| unused | 64512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Leonardo (1989)(Starbyte).adf:bootblock | 1 | 129 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 427 | 21 |
| Leonardo (1989)(Starbyte).adf bytes 382976-405504 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: custom-format (non-DOS) disk, 1632 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 241 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: bytes 382976-405504 look like code no run loaded: recovered position-independently (load address unknown)
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: 1 logged loads cover 22528 bytes of the image
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: 9 track-loaded blocks found in recovered runtime segments and classified through them
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: blocks 2-65 (bytes 1024-33792) outside the filesystem hold data, not code (opcode density 0.057, entropy 5.84)
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: blocks 70-91 (bytes 35840-47104) outside the filesystem hold data, not code (opcode density 0.028, entropy 4.79)
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: blocks 96-154 (bytes 49152-79360) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.58)
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: blocks 506-676 (bytes 259072-346624) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.10)
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: blocks 682-715 (bytes 349184-366592) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.83)
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: blocks 726-726 (bytes 371712-372224) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.39)
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: blocks 796-817 (bytes 407552-418816) outside the filesystem hold data, not code (opcode density 0.029, entropy 4.76)
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: blocks 822-850 (bytes 420864-435712) outside the filesystem hold data, not code (opcode density 0.058, entropy 4.65)
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: blocks 852-858 (bytes 436224-439808) outside the filesystem hold data, not code (opcode density 0.051, entropy 3.87)
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: blocks 861-887 (bytes 440832-454656) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.11)
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: blocks 890-890 (bytes 455680-456192) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.53)
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: blocks 902-1006 (bytes 461824-515584) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.71)
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: blocks 1012-1354 (bytes 518144-693760) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.87)
- leonardo (1989)(starbyte)/Leonardo (1989)(Starbyte).adf: blocks 1364-1726 (bytes 698368-884224) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.48)
- 1 never-loaded region(s), 22528 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 169472 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_dma_controller` (bootblock): Sets a bit in a hardware register, calls two routines for initialization, performs division and conditional operations, and manages a loop with bit testing and clearing. Kind: disk hardware check. Pass path: The routine modifies CIAB_PRB to toggle bit 2 (0x4), and continues execution only if certain conditions on track numbers and loop counters are met. A successful pass would proceed through the loop without triggering early termination or err.
- `update_dma_counter` (bootblock): Updates a counter in a global variable by incrementing it through a loop, with conditional exits based on the counter's value, and performs bit manipulation on another global variable. Kind: disk hardware check. Pass path: The check passes when the CIAB_PRB bit 2 (0x4) is set correctly, allowing execution to continue through the loop and eventually return normally. If the bit is not set, u0__sub_017C() is called, likely triggering a trap or abort mechanism..
- `frag_clr_eori_btst_beq_bsr_bra` (bootblock): Short code fragment that clears d3, then exclusive-ors 4 into a fixed memory address, then tests bit 2 of a fixed memory address, then branches if the last result was zero, then calls another routine, then jumps to another point in the code. Kind: disk hardware check. Pass path: branch where (bVar1 & 4) == 0, i.e. the else path of the if statement, or the call to u0__sub_017C() is skipped.
- `initialize_dma_channels` (bootblock): Sets up multiple hardware registers related to disk and DMA control, including enabling specific DMA channels and configuring data transfer parameters. Kind: disk track sync check. Pass path: The routine initializes DMA channels and waits for a specific interrupt (INTREQR & 2), which suggests it's checking for a valid disk sync pattern. If the check passes, the code continues execution normally, but if it fails, it would likely .
- `frag_move_nop_dbra_rts` (bootblock): Short code fragment that copies 125 to a fixed memory address, then pauses for one no-operation step (2 times in a row), then copies 117 to a fixed memory address, then copies 45056 to d6, then loops back while counting down d6, then returns. Kind: disk hardware check. Pass path: CIAB_PRB writes and delay loop proceed normally, indicating successful hardware verification.
- `fall_through_entry` (bootblock): Entry point part-way through straight-line code: runs one instruction (bclr.b) and then falls through into the code that follows, with no return of its own. Kind: disk hardware check. Pass path: branch taken when CIAB_PRB check passes (likely the 'beq.b an address' path in sub_007E).
- `frag_bclr_bset_move_dbra_bsr_rts` (bootblock): Short code fragment that clears bit 0 of a fixed memory address, then sets bit 0 of a fixed memory address, then copies 7096 to d6, then loops back while counting down d6, then calls another routine, then returns. Kind: disk hardware check. Pass path: bra.b an address.
- `frag_btst_beq_bset_bsr_bra` (bootblock): Short code fragment that tests bit 4 of a fixed memory address, then branches if the last result was zero, then sets bit 1 of a fixed memory address, then calls another routine, then jumps to another point in the code. Kind: disk hardware check. Pass path: the loop continues until bit 4 of CIAA_PRA is clear, then returns normally.

## Functions


### Leonardo (1989)(Starbyte).adf:bootblock

- `frag_nop_lea_move_dbra_jmp`: Short code fragment that pauses for one no-operation step (3 times in a row), then loads the address of a location in the code into a0, then copies 1023 to d0, then loads the address of a fixed memory address into a1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then jumps to another point in the code.
- `initialize_system`: Disables interrupts by setting a specific bit in a hardware register, enters supervisor mode, and performs two sequential initialization operations with distinct parameters before ending.
- `initialize_dma_controller`: Sets a bit in a hardware register, calls two routines for initialization, performs division and conditional operations, and manages a loop with bit testing and clearing.
- `update_dma_counter`: Updates a counter in a global variable by incrementing it through a loop, with conditional exits based on the counter's value, and performs bit manipulation on another global variable.
- `frag_clr_eori_btst_beq_bsr_bra`: Short code fragment that clears d3, then exclusive-ors 4 into a fixed memory address, then tests bit 2 of a fixed memory address, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `initialize_dma_channels`: Sets up multiple hardware registers related to disk and DMA control, including enabling specific DMA channels and configuring data transfer parameters.
- `frag_move_nop_dbra_rts`: Short code fragment that copies 125 to a fixed memory address, then pauses for one no-operation step (2 times in a row), then copies 117 to a fixed memory address, then copies 45056 to d6, then loops back while counting down d6, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (bclr.b) and then falls through into the code that follows, with no return of its own.
- `frag_bclr_bset_move_dbra_bsr_rts`: Short code fragment that clears bit 0 of a fixed memory address, then sets bit 0 of a fixed memory address, then copies 7096 to d6, then loops back while counting down d6, then calls another routine, then returns.
- `frag_btst_bne_rts`: Short code fragment that tests bit 5 of a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_btst_beq_bset_bsr_bra`: Short code fragment that tests bit 4 of a fixed memory address, then branches if the last result was zero, then sets bit 1 of a fixed memory address, then calls another routine, then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_data_block`: Processes a data block by comparing specific values, extracting and transforming byte data using bit manipulation operations, then stores the result to an output buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_audio_counter`: Increments a global counter, clears it when reaching six, calls another routine, and manages audio channel updates for three channels.
- `frag_lea_moveq_movea_clr_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads 14 into d0, then copies the memory at a0 (advancing a0) to a1, then clears the memory at a1, then loops back while counting down d0, then returns.
- `frag_lea_st_clr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then sets a fixed memory address to all ones, then clears the memory at an offset from a0 (4 times in a row), then clears a fixed memory address (3 times in a row), then returns.
- `frag_lea_sf_clr_move_rts`: Short code fragment that loads the address of a fixed memory address into a0, then clears a fixed memory address, then clears the memory at an offset from a0 (4 times in a row), then copies 15 to the memory at an offset from a0, then returns.
- `find_maximum_value`: Iterates through a data structure to find the maximum byte value, increments it by one, and multiplies the result by 1024 before returning.
- `update_position_bounds`: Updates position values based on bounds checking and conditional arithmetic operations, then stores the result in a global structure.
- `frag_clr_move_lsr_bra`: Short code fragment that clears d0, then copies the memory at an offset from a6 to d0, then shifts d0 right by 4, then jumps to another point in the code.
- `frag_clr_move_andi_bra`: Short code fragment that clears d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `update_position_values`: Reads position-related values from memory, performs conditional arithmetic operations based on bit shifts and comparisons, then updates corresponding memory locations.
- `update_audio_dma`: Configures audio DMA channels by setting bit 15 of the DMA control register and performs a delay loop of 250 iterations before returning.
- `update_object_state`: Updates object state using lookup tables based on type and position, then writes computed values to a structure and modifies global DMA control registers.
- `decode_pattern_data_2`: Processes compressed pattern data by decoding bits and copying bytes to output buffers, handling various control codes and bit manipulation.
- `clear_and_compare`: Clears a byte at a memory location, decrements a counter, and compares addresses with conditional branches based on the comparison results.
- `frag_cmpa_bcs_adda_bra`: Short code fragment that compares a1 with a0, then branches if the carry flag is set, then adds a4 to a1, then jumps to another point in the code.
- `decode_pattern_data`: Processes compressed pattern data by decoding bits and copying bytes to output buffers, handling various control codes and bit manipulation operations during decompression.
- `clear_and_branch`: Clears a byte at a memory location, decrements a counter, and performs address comparisons with conditional branches based on the comparison results.
- `frag_cmpa_bcs_adda_bra_2`: Short code fragment that compares a1 with a0, then branches if the carry flag is set, then adds a4 to a1, then jumps to another point in the code.
- `frag_btst_beq_and_addq_move_dbra_bra`: Short code fragment that tests bit 6 of d0, then branches if the last result was zero, then masks d0 with 63, then adds 6 to d0, then copies the memory just below a1 (moving a1 down) to the memory just below a2 (moving a2 down), then loops back while counting down d0, then jumps to another point in the code.
- `frag_and_addq_moveq_move_dbra_bra`: Short code fragment that masks d0 with 63, then adds 6 to d0, then loads 0 into d3, then copies d3 to the memory just below a2 (moving a2 down), then loops back while counting down d0, then jumps to another point in the code.
- `frag_move_moveq_addq_dbra_bra`: Short code fragment that copies the memory just below a1 (moving a1 down) to d3, then loads 0 into d0, then copies the memory just below a1 (moving a1 down) to d0, then adds 6 to d0, then copies d3 to the memory just below a2 (moving a2 down), then loops back while counting down d0, then jumps to another point in the code.
- `update_interrupt_handler`: Configures interrupt settings, copies data from one memory location to another while decrementing a counter, then restores interrupt configuration before returning.
