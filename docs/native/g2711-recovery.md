# Game 2711 (Apprentice): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02711). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906208 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 16 of 16 named (0 author symbols), 16 of 16 described (8 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 623 of 623 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 47104 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 796 of 797 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 13111 |
| code | 3028 |
| data | 882901 |
| unused | 7168 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Apprentice.adf:bootblock | 1 | 6 | 1 |
| logged disk load from Apprentice.adf boot-block load | 1 | 28 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 763 | 10 |
| Apprentice.adf bytes 804352-851456 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Apprentice/Apprentice.adf: custom-format (non-DOS) disk, 1744 non-empty blocks: matched against track-loaded segments
- Apprentice/Apprentice.adf: the boot block reads 11264 bytes from disk offset 0 to an address (found statically); recovered as a unit
- runtime segment an address: 623 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Apprentice/Apprentice.adf: bytes 804352-851456 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- Apprentice/Apprentice.adf: 2 logged loads cover 58368 bytes of the image
- Apprentice/Apprentice.adf: blocks 22-1569 (bytes 11264-803840) hold a BK: ByteKiller stream (792576 -> 6022 bytes)
- Apprentice/Apprentice.adf: blocks 1666-1671 (bytes 852992-856064) outside the filesystem hold data, not code (opcode density 0.039, entropy 4.85)
- Apprentice/Apprentice.adf: blocks 1676-1687 (bytes 858112-864256) outside the filesystem hold data, not code (opcode density 0.003, entropy 1.52)
- Apprentice/Apprentice.adf: blocks 1690-1755 (bytes 865280-899072) outside the filesystem hold data, not code (opcode density 0.012, entropy 6.75)
- 1 never-loaded region(s), 47104 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Apprentice.adf:bootblock

- `frag_move_jsr_jmp`: Short code fragment that copies 2 to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then copies 0 to the memory at an offset from a1, then copies 11264 to the memory at an offset from a1, then calls another routine, then jumps to another point in the code.

### logged disk load from Apprentice.adf boot-block load

- `copy_until_match`: Reads bytes from a memory location pointed by a0 and copies them to another location pointed by a2, stopping when a matching byte is found.
- `frag_move_movea_jmp`: Short code fragment that copies d0 to the memory just below a2 (moving a2 down), then copies a location in the code to a2, then copies 33280 to the memory at an offset from a4, then copies 49152 to the memory at an offset from a4, then jumps to the address in a2.
- `frag_move_subq_bne_bra`: Short code fragment that copies the memory just below a0 (moving a0 down) to d2, then copies the memory just below a0 (moving a0 down) to d1, then copies d1 to the memory just below a2 (moving a2 down), then subtracts 1 from d2, then branches if the last result was not zero, then copies d1 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_moveq_move_bra`: Short code fragment that loads 0 into d1, then copies the memory just below a0 (moving a0 down) to d2, then jumps to another point in the code.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_display_dma_2`: Configures DMA channels and interrupt settings for display rendering, then updates screen-related global variables with calculated values derived from table lookups.
- `frag_movem_lea_bsr_jmp`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a6, then calls another routine (3 times in a row), then restores saved registers from the stack, then jumps to another point in the code.
- `frag_lea_cmpa_beq_move_addq_bra`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then compares a0 with an address, then branches if the last result was zero, then copies the memory at a0 to the memory at a1, then adds 1 to a0, then adds 1 to a1, then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_and_execute`: Initializes several hardware registers and memory locations based on bits in a global variable, then enters an infinite loop that continuously checks and updates system state while performing various memory writes to specific addresses.
- `update_render_data`: Updates rendering parameters in a global structure including memory addresses, offsets, and status flags based on data fetched from a table, with repeated iterations until a specific bit in the status byte is set.
- `update_input_state`: Reads the joystick/mouse fire button status and updates global variables based on its state, including handling input direction changes and modifying scroll positions.
- `initialize_globals`: Sets a global variable to zero, calculates a memory address using multiplication and addition, and clears another global variable.
- `process_character_data`: Processes character data from a buffer, updates memory addresses based on comparisons, and stores calculated values into a structure at multiple offsets.
- `update_display_dma`: Updates display DMA configuration and transfers data through custom hardware registers, managing multiple blitting operations with varying parameters and waiting for completion between transfers.
