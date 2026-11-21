# Game 4172 (klappersnake (19xx)(mounty - dahrn)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04172). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901408 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 12 of 12 named (0 author symbols), 12 of 12 described (2 by rule, 9 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 17 of 17 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 216 of 216 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 788 |
| data | 817163 |
| padding | 1 |
| unused | 83456 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Klappersnake (19xx)(Mounty - Dahrn).adf:bootblock | 1 | 139 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 54 | 5 |

## Cases handled

- klappersnake (19xx)(mounty - dahrn)/Klappersnake (19xx)(Mounty - Dahrn).adf: custom-format (non-DOS) disk, 1595 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- klappersnake (19xx)(mounty - dahrn)/Klappersnake (19xx)(Mounty - Dahrn).adf: blocks 2-760 (bytes 1024-389632) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.53)
- klappersnake (19xx)(mounty - dahrn)/Klappersnake (19xx)(Mounty - Dahrn).adf: blocks 769-779 (bytes 393728-399360) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.34)
- klappersnake (19xx)(mounty - dahrn)/Klappersnake (19xx)(Mounty - Dahrn).adf: blocks 792-801 (bytes 405504-410624) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.32)
- klappersnake (19xx)(mounty - dahrn)/Klappersnake (19xx)(Mounty - Dahrn).adf: blocks 814-1564 (bytes 416768-801280) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.12)
- klappersnake (19xx)(mounty - dahrn)/Klappersnake (19xx)(Mounty - Dahrn).adf: blocks 1584-1586 (bytes 811008-812544) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.41)
- klappersnake (19xx)(mounty - dahrn)/Klappersnake (19xx)(Mounty - Dahrn).adf: blocks 1604-1604 (bytes 821248-821760) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- klappersnake (19xx)(mounty - dahrn)/Klappersnake (19xx)(Mounty - Dahrn).adf: blocks 1606-1630 (bytes 822272-835072) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.96)
- klappersnake (19xx)(mounty - dahrn)/Klappersnake (19xx)(Mounty - Dahrn).adf: blocks 1650-1653 (bytes 844800-846848) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.40)
- klappersnake (19xx)(mounty - dahrn)/Klappersnake (19xx)(Mounty - Dahrn).adf: blocks 1672-1674 (bytes 856064-857600) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.25)
- klappersnake (19xx)(mounty - dahrn)/Klappersnake (19xx)(Mounty - Dahrn).adf: blocks 1694-1718 (bytes 867328-880128) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.19)
- klappersnake (19xx)(mounty - dahrn)/Klappersnake (19xx)(Mounty - Dahrn).adf: blocks 1738-1740 (bytes 889856-891392) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.32)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_graphics_system` (bootblock): Initializes graphics hardware registers, allocates memory for display structures, opens device drivers, and sets up system resources for video output. Kind: trackdisk.device I/O check. Pass path: The routine calls exec_OpenDevice and exec_DoIO with a jumptable-based indirect call, suggesting it's performing a hardware or disk-specific check (likely sync, track, or sector validation) to verify the presence of a legitimate boot medium.

## Functions


### Klappersnake (19xx)(Mounty - Dahrn).adf:bootblock

- `copy_graphics_init`: Allocates memory for graphics initialization data and copies a routine's code into the allocated space, then returns zero.
- `initialize_graphics_system_2`: Allocates memory for graphics structures, sets up display registers including bitplane pointers and modulos, configures copper lists with specific hardware values, and initializes system resources for video rendering.
- `initialize_graphics_system`: Initializes graphics hardware registers, allocates memory for display structures, opens device drivers, and sets up system resources for video output.
- `frag_move_subq_bne_rts`: Short code fragment that copies d0 to the memory at a0 (advancing a0), then subtracts 1 from d1, then branches if the last result was not zero, then returns.
- `copy_character_data`: Copies character bitmap data from a source table to multiple destination locations, advancing the destination pointer by 40 bytes after each copy operation.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_buffer`: Processes input buffer entries by reading bytes and updating a global variable based on specific conditions, then performs bit manipulation on another global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `shift_queue_element`: Shifts elements in a queue structure by one position towards the beginning, decrementing a counter and returning the first element.
- `shift_queue_element_2`: Shifts elements in a second queue structure by one position towards the beginning, decrementing a counter and returning the first element.
- `shift_queue_element_3`: Shifts elements in a third queue structure by one position towards the beginning, decrementing a counter and returning the first element.
- `shift_queue_element_4`: Shifts elements in a fourth queue structure by one position towards the beginning, decrementing a counter and returning the first element.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
