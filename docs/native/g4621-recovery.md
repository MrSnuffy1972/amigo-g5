# Game 4621 (Overlander (Europe)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04621). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4808647 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 5 of 5 named (0 author symbols), 5 of 5 described (0 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3 of 3 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 36 of 36 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1996603 |
| code | 158 |
| data | 2802670 |
| unused | 9216 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Overlander (Europe).adf:bootblock | 1 | 14 | 2 |
| Overlander (Europe).ipf#adf:bootblock | 1 | 14 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |

## Cases handled

- Overlander (Europe)/Overlander (Europe).adf: custom-format (non-DOS) disk, 1749 non-empty blocks: matched against track-loaded segments
- Overlander (Europe)/Overlander (Europe).ipf decoded by disk-analyse to a 901120-byte disk image (tracks: T0.0: AmigaDOS; T0.1-79.1: Unformatted*)
- Overlander (Europe)/Overlander (Europe).ipf#adf: custom-format (non-DOS) disk, 1749 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Overlander (Europe)/Overlander (Europe).adf: blocks 11-1759 (bytes 5632-901120) hold ASCII text
- Overlander (Europe)/Overlander (Europe).ipf#adf: blocks 11-1759 (bytes 5632-901120) hold ASCII text

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Overlander (Europe).adf:bootblock

- `enable_dma_and_interrupts`: Writes a specific value to three hardware control registers related to DMA and interrupts, then prepares a subroutine call by placing an address on the stack and executes a trap instruction.
- `setup_stack_and_copy`: Initializes the supervisor stack pointer to a fixed address and copies 256 longwords from a program counter relative source to a destination address using a loop that decrements a word register.

### Overlander (Europe).ipf#adf:bootblock

- `enable_dma_and_interrupts_2`: Writes a specific value to three hardware control registers related to DMA and interrupts, then prepares a subroutine call by placing an address on the stack and executes a trap instruction.
- `setup_stack_and_copy_2`: Initializes the supervisor stack pointer to a fixed address and copies 256 longwords from a program counter relative source to a destination address using a loop that decrements a word register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_update_flag`: Reads a word value from a memory location, performs a bitwise AND with another word value at a different offset, and if the result is zero, stores the second value to yet another location before branching to a fixed address.
