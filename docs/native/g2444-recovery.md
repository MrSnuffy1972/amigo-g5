# Game 2444 (links - the challenge of golf v1.53 (1990)(access)(disk 1 of 2)[cr combat 18]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02444). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 30 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 15 of 15 named (0 author symbols), 15 of 15 described (3 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 83 of 83 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 636 of 636 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 501208 |
| code | 1626 |
| data | 385307 |
| padding | 8371 |
| unused | 4608 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Links - The Challenge of Golf v1.53 (1990)(Access)(Disk 1 of 2)[cr Combat 18].adf:GOLF | 2 | 267 | 8 |
| run | 2 | 117 | 2 |
| Type | 2 | 117 | 2 |
| Links - The Challenge of Golf v1.53 (1990)(Access)(Disk 1 of 2)[cr Combat 18].adf:Type | 2 | 117 | 2 |
| Links - The Challenge of Golf v1.53 (1990)(Access)(Disk 1 of 2)[cr Combat 18].adf:bootblock | 1 | 18 | 1 |

## Cases handled

- links - the challenge of golf v1.53 (1990)(access)(disk 1 of 2)[cr combat 18]/Links - The Challenge of Golf v1.53 (1990)(Access)(Disk 1 of 2)[cr Combat 18].adf:INTROSND.LZ: high-entropy data file (7.85 bits/byte), classified as packed data
- links - the challenge of golf v1.53 (1990)(access)(disk 1 of 2)[cr combat 18]/Links - The Challenge of Golf v1.53 (1990)(Access)(Disk 1 of 2)[cr Combat 18].adf:SOUNDS.LZ: high-entropy data file (7.74 bits/byte), classified as packed data
- links - the challenge of golf v1.53 (1990)(access)(disk 1 of 2)[cr combat 18]/Links - The Challenge of Golf v1.53 (1990)(Access)(Disk 1 of 2)[cr Combat 18].adf:TOPVIEW.LZ: high-entropy data file (7.76 bits/byte), classified as packed data
- links - the challenge of golf v1.53 (1990)(access)(disk 1 of 2)[cr combat 18]/Links - The Challenge of Golf v1.53 (1990)(Access)(Disk 1 of 2)[cr Combat 18].adf:GOLF1.LZ: high-entropy data file (7.72 bits/byte), classified as packed data
- links - the challenge of golf v1.53 (1990)(access)(disk 1 of 2)[cr combat 18]/Links - The Challenge of Golf v1.53 (1990)(Access)(Disk 1 of 2)[cr Combat 18].adf:SCORECARD.LZ: high-entropy data file (7.90 bits/byte), classified as packed data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Links - The Challenge of Golf v1.53 (1990)(Access)(Disk 1 of 2)[cr Combat 18].adf:GOLF

- `load_executable_hunks_with_relocation`: Program entry that allocates and frees memory through exec and parses AmigaDOS executable hunk records, comparing them against hunk type codes for loading and relocation.
- `run_countdown_with_library_calls`: Calls two Amiga library functions and runs a countdown loop on an input counter until it wraps.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_backward_run_length_bytes`: Decodes a packed stream into bytes written backwards from a moving buffer pointer, handling run lengths with special cases for the values three and seven.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_multiword_with_extend_flag`: Performs a multi-word bit shift loop over a count held in an input register, using the extend flag to carry bits between words from memory.
- `stub_trap_then_bad_data`: Issues a trap and then truncates at bad instruction data, so only the trap call is recovered.

### run

- `find_max_and_prepare_buffer_2`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_and_call_handler`: Copies sixteen longwords from a table into a stack-supplied buffer when that pointer is non-null, then calls through a stored code pointer.

### Type

- `find_max_and_prepare_buffer`: Walks a linked list of entries to find the largest value stored in a field.
- `copy_stack_data`: Copies sixteen bytes of data from a memory location offset from a register to the current stack pointer.

### Links - The Challenge of Golf v1.53 (1990)(Access)(Disk 1 of 2)[cr Combat 18].adf:Type

- `initialize_relocation_table`: Calculates the maximum size required for a relocation table by traversing a linked list of structures.
- `copy_memory_block`: Copies sixteen bytes of data from a source offset to a destination address using a loop.

### Links - The Challenge of Golf v1.53 (1990)(Access)(Disk 1 of 2)[cr Combat 18].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
