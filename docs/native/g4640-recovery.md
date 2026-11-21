# Game 4640 (paperboy (1989)(elite)[cr band][t +2 band]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04640). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903024 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 42 of 42 named (0 author symbols), 42 of 42 described (33 by rule, 9 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 308 of 308 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 616 of 616 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 1760 |
| data | 1230 |
| padding | 962 |
| unused | 899072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Paperboy (1989)(Elite)[cr Band][t +2 Band].adf:bootblock | 1 | 8 | 1 |
| logged disk load from Paperboy (1989)(Elite)[cr Band][t +2 Band].adf boot-block load | 1 | 99 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 509 | 39 |

## Cases handled

- paperboy (1989)(elite)[cr band][t +2 band]/Paperboy (1989)(Elite)[cr Band][t +2 Band].adf: the boot block reads 29184 bytes from disk offset 421376 to an address (found statically); recovered as a unit
- runtime segment an address: 308 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate+address
- paperboy (1989)(elite)[cr band][t +2 band]/Paperboy (1989)(Elite)[cr Band][t +2 Band].adf: 1 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_audio` (runtime segment an address): Sets up audio registers with predefined values, configures DMA and interrupt control registers, initializes a global variable, and performs a loop that updates memory locations until a condition is met. Kind: disk hardware check. Pass path: The routine initializes audio registers and sets up ADKCON, which suggests a disk drive synchronization or hardware verification check. The call to u2__sub_0180() and the loop that waits for a bit in the status register (likely related to d.

## Functions


### Paperboy (1989)(Elite)[cr Band][t +2 Band].adf:bootblock

- `frag_move_movea_jsr_jmp`: Short code fragment that copies 2 to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then copies 29184 to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then copies a fixed memory address to a6, then calls another routine (2 times in a row), then jumps to another point in the code.

### logged disk load from Paperboy (1989)(Elite)[cr Band][t +2 Band].adf boot-block load

- `shift_and_branch_2`: Performs a bitwise right shift operation on a value, stores the result in a global variable, and recursively calls itself while handling conditional branches.
- `shift_and_branch`: Performs bitwise right shift operations on a value, decrements a counter, and recursively calls itself while handling conditional branches based on comparison results.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_movea_move_rts`: Short code fragment that copies an address to a5, then copies 15 to the memory at an offset from a5, then returns.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `initialize_audio_2`: Configures audio hardware registers by setting specific values, enables DMA channels and interrupts through control registers, and initializes a global variable.
- `initialize_audio`: Sets up audio registers with predefined values, configures DMA and interrupt control registers, initializes a global variable, and performs a loop that updates memory locations until a condition is met.
- `frag_move_rts`: Short code fragment that copies 32 to the memory at an offset from a0, then copies 15 to the memory at an offset from a0, then returns.
- `handle_interrupts`: Reads interrupt status registers and clears pending interrupts, then increments a counter and calls another routine when a specific bit is set.
- `frag_add_jmp`: Short code fragment that adds d0 to d0 (2 times in a row), then jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `update_game_objects`: Updates game object data structures by calculating memory addresses, setting object properties, clearing status flags, and managing object positioning based on input parameters.
- `process_input_buffer_2`: Processes input commands from a buffer, updates global variables based on command types, handles conditional logic, and manages status flags for input processing.
- `frag_bra_8`: Short code fragment that jumps to another point in the code.
- `frag_bra_9`: Short code fragment that jumps to another point in the code.
- `frag_bra_10`: Short code fragment that jumps to another point in the code.
- `frag_bra_11`: Short code fragment that jumps to another point in the code.
- `frag_bra_12`: Short code fragment that jumps to another point in the code.
- `frag_bra_13`: Short code fragment that jumps to another point in the code.
- `frag_bra_14`: Short code fragment that jumps to another point in the code.
- `frag_bra_15`: Short code fragment that jumps to another point in the code.
- `frag_bra_16`: Short code fragment that jumps to another point in the code.
- `frag_bra_17`: Short code fragment that jumps to another point in the code.
- `frag_bra_18`: Short code fragment that jumps to another point in the code.
- `frag_bra_19`: Short code fragment that jumps to another point in the code.
- `frag_bra_20`: Short code fragment that jumps to another point in the code.
- `frag_bra_21`: Short code fragment that jumps to another point in the code.
- `frag_bra_22`: Short code fragment that jumps to another point in the code.
- `frag_bra_23`: Short code fragment that jumps to another point in the code.
- `frag_bra_24`: Short code fragment that jumps to another point in the code.
- `frag_bra_25`: Short code fragment that jumps to another point in the code.
- `frag_bra_26`: Short code fragment that jumps to another point in the code.
- `frag_bra_27`: Short code fragment that jumps to another point in the code.
- `frag_bra_28`: Short code fragment that jumps to another point in the code.
- `process_input_command`: Reads commands from a buffer and updates global variables based on command types, including handling conditional logic and updating status flags.
- `frag_st_clr_move_bra`: Short code fragment that sets the memory at an offset from a5 to all ones, then clears d1, then copies the memory at a0 (advancing a0) to d1, then copies d1 to the memory at an offset from a5, then copies a0 to the memory at an offset from a5, then jumps to another point in the code.
- `process_input_buffer`: Reads commands from a buffer and updates global variables based on command types, including handling conditional logic and updating status flags.
