# Game 1553 (llamatron (1991)(llamasoft)(pd)[shell - run llama]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01553). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901568 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 25 of 25 named (0 author symbols), 25 of 25 described (4 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 12 of 12 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 579766 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 607 of 607 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 1822 |
| data | 676464 |
| padding | 3122 |
| unused | 220160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| ppmore | 2 | 233 | 8 |
| echo | 2 | 117 | 2 |
| Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:LLAMA512 | 1 | 17 | 2 |
| arp.library | 3 | 88 | 7 |
| Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:LLAMA | 1 | 17 | 2 |
| Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 121 | 1 |
| Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:TRON1MEG (raw code file, load address unknown) | 1 | 2 | 1 |
| Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:TRON512 (raw code file, load address unknown) | 1 | 2 | 1 |

## Cases handled

- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- llamatron (1991)(llamasoft)(pd)[shell - run llama]/Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:TRON1MEG: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- llamatron (1991)(llamasoft)(pd)[shell - run llama]/Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:TRON512: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- llamatron (1991)(llamasoft)(pd)[shell - run llama]/Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:c/ppmore: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 579766 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### ppmore

- `load_executable_hunks`: Loads an AmigaDOS executable from the ppmore file.
- `wait_for_countdown_loop`: Runs a countdown loop on a 16-bit count until it reaches zero, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_backward_stream_step`: Part of a backwards bit-stream unpacker in ppmore.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `decode_bit_stream_values`: Reads bits from a packed longword stream, shifting by a count held in a data register and refilling from memory when the register empties.
- `subtract_word_then_trap`: Subtracts one register from the word at a pointer and then issues trap six.

### echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_table_then_call_pointer`: When an output pointer is supplied, copies sixteen longwords from a fixed table in the echo program into it, then calls a routine through a pointer.

### Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:LLAMA512

- `forward_to_unnamed_routine`: The decompiler gave no pseudo-code.
- `open_dos_and_read_file`: Opens the dos library through the exec library, then opens and reads from a file, with dos.library and tron512 referenced in its text.

### arp.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 39 into d0, then returns.
- `collect_library_entries_until_null`: Library code from arp.library.
- `initialize_list_header_node`: Library code from arp.library that initialises a list node.
- `setup_library_node_from_result`: Library code from arp.library.
- `poll_until_flag_then_count`: Library code from arp.library that repeats a library call at offset -462 until the zero flag is set, then increments a 16-bit counter in the structure.
- `clear_slot_then_call_library`: Library code from arp.library.
- `allocate_block_with_link_word`: Library code from arp.library.

### Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:LLAMA

- `open_file_and_initialize`: Opens the DOS library via the exec library and reads data from a specific file.
- `open_dos_and_read_file_2`: Opens the dos library, then opens and reads a file whose text is tron1meg.

### Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_then_call_library_vectors`: Calls exec.Wait and then a series of library vectors at offsets from -30 to -222.

### Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:TRON1MEG (raw code file, load address unknown)

- `execute_initial_instructions`: This routine consists of two instructions at the start of a raw code region.

### Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:TRON512 (raw code file, load address unknown)

- `return_from_subroutine`: The routine consists of a single instruction that returns control to a caller.
