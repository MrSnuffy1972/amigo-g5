# Game 3358 (denaris (1989)(rainbow arts)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03358). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901776 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 7 of 7 named (0 author symbols), 7 of 7 described (1 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 8 of 8 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 213 of 213 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 704 |
| data | 895440 |
| unused | 5632 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Denaris (1989)(Rainbow Arts).adf:bootblock | 1 | 16 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 197 | 5 |

## Cases handled

- denaris (1989)(rainbow arts)/Denaris (1989)(Rainbow Arts).adf: custom-format (non-DOS) disk, 1747 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- denaris (1989)(rainbow arts)/Denaris (1989)(Rainbow Arts).adf: blocks 1529-1529 (bytes 782848-783360) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.12)
- denaris (1989)(rainbow arts)/Denaris (1989)(Rainbow Arts).adf: blocks 1531-1540 (bytes 783872-788992) outside the filesystem hold data, not code (opcode density 0.010, entropy 2.65)
- denaris (1989)(rainbow arts)/Denaris (1989)(Rainbow Arts).adf: blocks 1542-1759 (bytes 789504-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.37)
- 2 packed track region(s), 777216 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Denaris (1989)(Rainbow Arts).adf:bootblock

- `jump_to_next_routine`: Prepares for execution of the next routine by placing a return address on the stack, loading a value from memory location 128, and invoking a trap instruction.
- `initialize_hardware`: Sets up system registers including DMA control and color settings, then copies data from a source buffer to a destination address using a loop that processes 256 32-bit words.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_data_2`: Processes sprite position and state data, updates movement flags, and prepares rendering information for display.
- `update_sprite_data`: Processes sprite position and state data, updates movement flags, and prepares rendering information for display.
- `update_movement_state`: Calculates movement differences between current and previous values, updates state flags based on direction, and executes a fragment routine multiple times while decrementing a counter.
- `frag_move_dbra_rts`: Short code fragment that copies d7 to the stack, then copies 20480 to d7, then loops back while counting down d7, then copies the top of the stack to d7, then returns.
- `compute_string_hash`: Processes a null-terminated string by converting characters to lowercase, accumulating their values with bit shifting operations, and stores the final hash result in a global variable.
