# Game 3974 (hydacorn (1995)(union systems)(pl)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03974). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901696 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 18 of 18 named (0 author symbols), 18 of 18 described (8 by rule, 9 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 248 of 248 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 295 of 295 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 974 |
| data | 839352 |
| padding | 442 |
| unused | 60928 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Hydacorn (1995)(Union Systems)(Pl).adf:bootblock | 1 | 148 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 116 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 30 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- hydacorn (1995)(union systems)(pl)/Hydacorn (1995)(Union Systems)(Pl).adf: custom-format (non-DOS) disk, 1639 non-empty blocks: matched against track-loaded segments
- reference run ref ends at a 68000 address error (odd PC an address after an address): the harness halted there, nothing after it ran (68000 HALTED: address error at odd PC an address after an address, vector 3 an address is not a handler in RAM (the 68000 double-faults and halts); run ended at frame 60)
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 103 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 23 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- hydacorn (1995)(union systems)(pl)/Hydacorn (1995)(Union Systems)(Pl).adf: blocks 11-12 (bytes 5632-6656) outside the filesystem hold data, not code (opcode density 0.021, entropy 3.16)
- hydacorn (1995)(union systems)(pl)/Hydacorn (1995)(Union Systems)(Pl).adf: blocks 33-43 (bytes 16896-22528) outside the filesystem hold data, not code (opcode density 0.012, entropy 5.86)
- hydacorn (1995)(union systems)(pl)/Hydacorn (1995)(Union Systems)(Pl).adf: blocks 201-202 (bytes 102912-103936) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.17)
- hydacorn (1995)(union systems)(pl)/Hydacorn (1995)(Union Systems)(Pl).adf: blocks 1045-1058 (bytes 535040-542208) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.14)
- hydacorn (1995)(union systems)(pl)/Hydacorn (1995)(Union Systems)(Pl).adf: blocks 1152-1174 (bytes 589824-601600) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.99)
- 10 packed track region(s), 812544 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Hydacorn (1995)(Union Systems)(Pl).adf:bootblock

- `enter_supervisor_mode`: Enters supervisor mode by reading a register value, testing a bit flag, and calling a supervisor routine when the flag is set before branching to another location.
- `initialize_dma_and_copy`: Initializes DMA control registers, sets up the stack pointer, and performs multiple memory copy operations using loop counters before jumping to a fixed address.
- `initialize_hardware`: Initializes hardware by calling several low-level routines and setting up a global variable with a specific value before jumping to a fixed address.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_lea_move_jmp`: Short code fragment that loads the address of a fixed memory address into a0, then copies 96 to the memory at an offset from a0, then jumps to the address in a0.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `configure_interrupts`: Configures interrupt settings by writing specific values to hardware registers and performing bit set/clear operations on I/O ports based on conditional tests.
- `frag_clr_move_bra`: Short code fragment that clears d1, then copies 136 to d1, then jumps to another point in the code.
- `update_port_bits`: Updates a byte value in a global variable at offset 256 using bitwise operations on data from registers, then performs a delay loop before returning.
- `process_data_blocks`: Processes data blocks by loading configuration values, performing bitwise operations on data, and looping through arrays with conditional branching based on computed results.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decode_and_copy_data`: Decodes data by performing arithmetic operations, copying memory blocks with loop counters, and handling conditional branches based on comparison results.
- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `fetch_and_mask_word`: Fetches a word value from a table using a calculated index and performs a bitwise AND operation with another register value before returning the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state`: Sets up initial values for game variables and configures hardware registers related to display and input.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
