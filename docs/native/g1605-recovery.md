# Game 1605 (rainbow islands (1987)(ocean)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01605). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908960 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 15 of 15 named (0 author symbols), 15 of 15 described (1 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 48 of 48 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 92672 bytes, 3 functions |
| Independent decoder cross-check (Ghidra against capstone) | 225 of 449 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 453 |
| code | 1412 |
| data | 719190 |
| padding | 1 |
| unused | 187904 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rainbow Islands (1987)(Ocean)[a].adf:bootblock | 1 | 19 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 36 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 3 |
| Rainbow Islands (1987)(Ocean)[a].adf bytes 45056-97792 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[a].adf bytes 100864-130048 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[a].adf bytes 265216-275968 (code never loaded by a reference run) | 1 | 365 | 3 |

## Cases handled

- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: custom-format (non-DOS) disk, 1391 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: bytes 45056-97792 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: bytes 100864-130048 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: bytes 265216-275968 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: 3 logged loads cover 92672 bytes of the image
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 22-86 (bytes 11264-44544) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.43)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 260-516 (bytes 133120-264704) outside the filesystem hold data, not code (opcode density 0.012, entropy 4.45)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 540-540 (bytes 276480-276992) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.59)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 542-542 (bytes 277504-278016) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.48)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 544-568 (bytes 278528-291328) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.26)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 570-744 (bytes 291840-381440) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.76)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 746-840 (bytes 381952-430592) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.17)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 842-942 (bytes 431104-482816) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.53)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 944-1054 (bytes 483328-540160) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.90)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 1056-1166 (bytes 540672-597504) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.02)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 1168-1290 (bytes 598016-660992) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.38)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 1292-1353 (bytes 661504-693248) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.86)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 1364-1364 (bytes 698368-698880) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.79)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 1366-1417 (bytes 699392-726016) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.33)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 1605-1619 (bytes 821760-829440) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.96)
- rainbow islands (1987)(ocean)[a]/Rainbow Islands (1987)(Ocean)[a].adf: blocks 1723-1736 (bytes 882176-889344) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.87)
- 2 never-loaded region(s), 81920 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rainbow Islands (1987)(Ocean)[a].adf:bootblock

- `clear_palette_then_copy_code`: Disables interrupts and DMA channels, clears the 16 palette color registers, traps once, copies 256 longwords from the boot block into low memory, and calls a routine through a pointer in the copied code.
- `copy_boot_longwords_and_call`: Copies 256 longwords from a fixed boot block source into the buffer addressed by the A5 register, then calls a routine through a table entry at offset 106 in that buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_forever_in_place`: A single-instruction branch to itself that never returns, entered at the head of its unit.
- `increment_counter_and_call_handler`: Increments a global counter, then calls a routine whose address is read from a pointer cell plus a 16-bit offset stored 38 bytes into the structure it points to.
- `dispatch_pending_interrupt_levels`: Reads the interrupt enable and request registers, masks the result to interrupt bits four to six, writes the masked bits to the interrupt request register, and calls up to three level handlers through function cells according to those bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_call`: Calls a subroutine using an address calculated by adding two values together.
- `store_word_triple`: Copies three consecutive 16-bit values from a source pointer into three consecutive 16-bit fields of a structure at fixed offsets, and returns minus one.
- `execute_callback_chain`: Walks a singly linked list starting from a pointer held in a structure, calling the routine at offset 16 in each node until a node's next pointer is zero.
- `call_structure_callback`: Calls the routine whose address is stored 20 bytes into the structure pointed to by the base register, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_once_then_increment_counter`: Calls a routine through a table entry when a 16-bit counter in the structure is zero, then increments the counter.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `execute_double_indirect_call`: Calls a subroutine by dereferencing a pointer located at a fixed offset from a base address.

### Rainbow Islands (1987)(Ocean)[a].adf bytes 265216-275968 (code never loaded by a reference run)

- `decrement_count_and_dispatch`: Calls a routine through a pointer stored in one structure, decrements a 16-bit count in a second structure with an overflow trap check, then calls another pointer-based routine and returns zero.
- `call_routine_per_character`: Walks a string until it finds a space or the count runs out, calling a routine looked up from a table at offset 8 of the base register for each character.
- `copy_name_component_padded`: Copies characters from a source string into a destination buffer up to a count, stopping at a zero, backslash or dot.
