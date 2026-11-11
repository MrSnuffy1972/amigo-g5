# Game 711 (blob (1993)(core design)[cr dcs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00711). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909392 top-level bytes in 52 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 26 of 26 units identical (bytes and relocations), 29 raw-word lines |
| Functions named and described | pass, 116 of 116 named (0 author symbols), 116 of 116 described (10 by rule, 72 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 981 of 981 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2258 of 2258 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 376912 |
| code | 7158 |
| data | 112999 |
| padding | 6819 |
| unused | 405504 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Blob (1993)(Core Design)[cr DCS].adf:blob | 3 | 65 | 1 |
| Assign | 2 | 117 | 2 |
| Blob (1993)(Core Design)[cr DCS].adf:INTRO | 2 | 267 | 8 |
| explode.library | 1 | 521 | 29 |
| Blob (1993)(Core Design)[cr DCS].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 64 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 963 | 49 |

## Cases handled

- 9 first-execution byte records contradict the logged opcode word (first an address): bad captures, the opcode word is used
- runtime segment an address: 22 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 52 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 546 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- blob (1993)(core design)[cr dcs]/Blob (1993)(Core Design)[cr DCS].adf:blob: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- blob (1993)(core design)[cr dcs]/Blob (1993)(Core Design)[cr DCS].adf:INTRO: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Blob (1993)(Core Design)[cr DCS].adf:INTRO

## Copy protection found

- none found

## Functions


### Blob (1993)(Core Design)[cr DCS].adf:blob

- `run_startup_library_sequence`: Checks a global state value and a flag, then either makes a sequence of exec and dos library calls through function pointers or tests a returned pointer on another path.

### Assign

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If the output pointer argument is non-null, copies sixteen longwords from a fixed offset in a base register into the memory it points to, then calls a function pointer held in a register.

### Blob (1993)(Core Design)[cr DCS].adf:INTRO

- `load_hunk_file_sections`: Parses AmigaDOS hunk records and allocates memory for each code, data and BSS section, copying section contents into it.
- `countdown_until_zero_loop`: Decrements a 16-bit register in a loop until it has counted past zero, then returns, so it only consumes time.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_backward_run_stream`: Decodes a packed stream into memory from the end toward the start, copying literal bytes when a flag is clear and copying runs of earlier bytes when it is set.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bit_buffer_loop`: Repeatedly shifts a multi-longword bit buffer by a count taken from the low bits of a value, refilling it from memory when it runs empty and carrying each outgoing bit into a flag.
- `trigger_software_interrupt`: Triggers a software interrupt via a trap instruction.

### explode.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `patch_branch_word_in_place`: Library code that writes a fixed 16-bit value into a location inside a structure addressed from the stack, then returns.
- `handle_message_request`: Reads a request structure from the caller, resolves a dos.library base via an indirect call and stores a handle, then extracts a short flag and a pointer from the request, and dispatches based on the flag value using table lookups, storing a result pointer and invoking further library routines before returning.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `restore_patched_jump_on_timeout`: Library code that decrements a countdown field in a structure, and when the count reaches zero with a flag set, checks for a patched jump and restores the saved words.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `restore_jump_and_release_library`: Checks whether a patched jump is still in place and, if so, restores the saved words and clears the processor cache.
- `set_structure_flag_bit_three`: Library code that sets bit three of a flag byte at a fixed offset in a structure addressed by a base register, and returns.
- `match_explode_library_entry`: Looks up an index through an unnamed routine, then checks the table entry at that index for a register-save prologue and for the name explode.library, packed into longwords.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d0.
- `verify_library_entry_name`: Looks up an index through an unnamed routine, then checks the table entry at that index for a register-save prologue and for the explode name spelled in four words.
- `check_library_name`: The routine receives a pointer in a0 and checks whether the four words pointed to by that pointer spell a library name string.
- `decrement_count_then_clear_cache`: Library code that decrements a 16-bit count field in a structure, calls one unnamed routine, and clears the processor cache when the library version is above a threshold.
- `build_table_and_expand_stream`: Library code that builds a decoding table from a header of bit lengths, then expands packed data by copying literal runs and back-references into the output.
- `copy_blocks_and_fix_pointers`: Library code that copies longword blocks from the input into the output, and for each indexed entry adds a base pointer value to a stored location.
- `copy_blocks_then_decode_fixups`: Library code in the same expansion family as the stream decoder: it copies longword blocks from the input, then decodes fixup entries by adding a base pointer value into stored locations.
- `decode_fixups_then_release_block`: Library code that copies longword blocks and decodes fixup entries against an indexed table, and releases a block through a library call once the input is finished.
- `enter_block_copy_loop`: A two-instruction entry in the explode library that calls one unnamed routine and then continues in the longword block copy loop shared with the expansion code.
- `decode_backward_bit_literals`: Library code that shifts bits out of a control byte with carry and copies bytes backwards from the input into the output while a counter remains.
- `decode_backward_length_codes`: Library code that reads bit-coded length codes from the input by shifting a control byte with carry, then copies a run of earlier output bytes.
- `decode_table_length_value`: Library code that reads a bit-coded value from the input, using bit counts taken from a table in the library's data, and then continues into the shared bit-decoding loop.
- `enter_bit_decode_loop_five`: Enters the shared bit-decoding loop with a starting count of five and a table index of three, then decodes bit-coded length and distance values from the input.
- `enter_bit_decode_loop_four`: Enters the shared bit-decoding loop with a starting count of four and a table index of two, then decodes bit-coded length and distance values from the input.
- `enter_bit_decode_loop_three`: Enters the shared bit-decoding loop with a starting count of three and a table index of one, then decodes bit-coded values from the input.
- `run_bit_decode_loop_zero`: Starts the shared bit-decoding loop with a counter of two and a table index of zero, then decodes bit-coded values from the input.
- `decode_bit_coded_distance`: Reads bit flags from the input and looks up a base value and an extra-bit count in the library's tables, then reads a byte from the input as an extra value.
- `read_table_sized_bit_field`: Reads a number of bits given by a library table entry from the input and shifts them into a value, then adds that value to a base count.
- `copy_back_reference_run`: Reads a length code from the input and a distance from a table-based bit field, then copies that many bytes from earlier in the output.
- `free_counted_memory_blocks`: Frees a counted list of memory blocks through the exec library's FreeMem function, working down from a block count stored in the structure, then clears the final link word.

### Blob (1993)(Core Design)[cr DCS].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.
- `call_library_with_values`: Repeatedly clears d2 and d3, calls a library routine at offset -222 of a6, and loads one of five specific longwords into d1 before jumping to a hardware address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_with_values_alt`: Repeatedly clears d2 and d3, calls a library routine at offset -222 of a6, and loads one of five different longwords into d1 before jumping to a hardware address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_interrupt_state`: Performs several operations including filling a memory area with 16-bit words, polling bitmasks, and updating status words while managing hardware registers for interrupts and DMA.
- `poll_bitmask_match`: Loops until a 32-bit value at an offset of 4 from a5, when masked with a constant, matches a specific value, then loops again until that same masked value does not match the constant before returning.
- `save_context_and_set_interrupt`: Saves the processor status register and all registers to the stack, calls another routine, restores the registers, writes 32 to a hardware register, and returns from an interrupt.
- `test_and_branch_bit`: Writes 33792 to an offset of 150 from a5, then checks bit 14 of a byte at offset 2 of a5 and branches if it is set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_library_call_and_load`: Loads a pointer from a structure in a5, calls another routine, and if the result is not zero, calls another routine, copies data from a table into a structure in a5, and then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_interrupt_and_return`: Loads d0 with 15 and d1 with a longword from a global variable, calls a routine, restores registers from the stack, restores the status register, writes 32 to the interrupt mask register, and returns from interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fetch_and_store_data`: Calls a routine via a pointer in a6, stores the result in a global variable at offset 36 of a5, and if the result is not zero, calls another routine, updates offset 44 and 46 of a5 using data from a pointer derived from the value at offset 36 of a5, and then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_interrupt_state`: Loads a byte from a memory location, calls another routine, restores registers from the stack, restores the status register, writes a word to a hardware register, and returns from interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_conditional_data`: Calls a routine via a pointer in a6, stores the result in a global variable at offset 36 of a5, and if the result is not zero, calls another routine, copies a word and a long word from a table pointed to by the result into offsets 44 and 46 of a5, and if the word equals a specific constant, stores a long word in a global variable before calling another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_interrupt_state_short`: Calls another routine, restores registers from the stack, restores the status register, writes a word to a hardware register, and returns from interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_data_on_flag`: If a zero flag is not set, calls a routine via a pointer in a6, copies a word and a long word from a table pointed to by the value at offset 36 of a5 into offsets 44 and 46 of a5, and if the word equals a specific constant, stores a long word in a global variable before calling another routine; otherwise it calls another routine via a pointer in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_interrupt_state_2`: Restores the processor status register from the stack, writes 32 to a hardware register, and returns from an interrupt.
- `update_control_structure`: Writes 32768 to an offset of 150 in a structure, then loops while bit 14 of a byte at offset 2 of that structure is set, finally writing 1024 to offset 150 of the structure before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_header`: Copies a 16-bit value and a 32-bit value from a pointer in a0 to a structure in a5, then either sets a global variable and a flag if the first 16-bit value is 19993, or calls another routine and loops while bit 6 of a byte at offset 2 of the structure in a5 is set before writing 1024 to offset 150 of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_bit_and_write`: Performs a conditional branch, checks bit 14 of a byte at offset 2 of a5, and if that bit is not set, writes a 16-bit word to offset 150 of a5 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_header_data`: Copies a 32-bit value from a pointer in a0 to a structure in a5, then either sets a 32-bit value in a structure in a1 if the first 16-bit value is 19993, or calls another routine and loops while bit 6 of a byte at offset 2 of the structure in a5 is set before writing 1024 to offset 150 of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_flag`: Loops while bit 6 of a byte at offset 2 of a structure in a5 is set, then writes the constant 1024 to offset 150 of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_and_update_table_2`: Checks a value at a pointer in a0, potentially calls another routine, and if certain conditions are met, copies a 32-bit value to a structure in a1, calls another routine, and then loops to copy data from a source to a destination before performing a loop that writes 32-bit values to a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_data_tables`: Fills a table starting at offset 2048 with the value 38 using a loop, then fills a second table starting at offset 2032 with 32-bit values derived from a source table using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_with_context`: Pushes d7 to the stack, loads an address into d7, calls another routine, restores d7 from the stack, and branches to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_and_update_table`: Enters a loop to copy words from a register to a destination pointed to by a1, then executes a loop that writes 32-bit values in increments of 7744 to a table starting at offset a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_and_branch`: Calls a function pointer from a table and then branches to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_table_writes`: Enters a loop that writes 32-bit values in increments of 7744 to a table starting at offset a fixed address.
- `copy_address_to_pointer`: Copies a longword from a global variable to another global variable and returns.
- `process_data_stream`: Checks a global variable and, if non-zero, decrements it; otherwise it enters a loop that processes bytes from a table, performing nested loops to copy data from one memory area to another, or calls a routine and updates a global counter.
- `initialize_counter_check_parameters`: Loads two memory addresses into registers, sets a loop limit of thirty-one in a register, sets a comparison value of four in another register, and calls another routine.
- `setup_buffer_comparison_parameters`: Loads two memory addresses into registers, sets a loop limit of thirty-one in one register and a comparison value of four in another, then calls increment_and_compare_buffer.
- `increment_and_compare_buffer`: Increments a byte in a global variable and, if it matches a specific value, enters a loop that compares words from two buffers, calls three different routines, and performs addition or subtraction on values pointed to by registers based on the comparison results.
- `return_empty_stub`: Returns at once without any work.
- `mask_and_transfer_registers`: Clears the lower four bits of two 16-bit values in registers and moves the results into two other registers before returning.
- `mask_register_values`: Copies d6 to d3 and d5 to d2, masks the upper nibble of both registers, sets d4 to 256, and returns.
- `branch_to_offset_two_hundred_ninety_two`: Performs a short branch to another routine.
- `branch_to_hardware_register`: Performs a short branch to the address stored in the second custom chip register.
- `branch_to_middle_of_routine`: Performs a short branch into the middle of another routine.
- `update_audio_and_dma_state`: Updates an audio structure by setting a byte at offset 1691, selecting a subroutine to store at offset 1676 based on a value in d0, and performing a loop to write values to DMA control registers.
- `dispatch_audio_updates`: Checks bits 0 through 3 of a byte at offset 1692 of a structure in a2 and, depending on which bits are set, calls another routine with specific hardware addresses in a5 and data in a6, then performs an indirect jump using a table.
- `update_object_properties_and_dispatch`: Updates a structure in a6 by fetching data from a table in a3 based on an index, then performs conditional logic to update offsets 8, 10, 14, 18, 36, and 40 of the structure.
- `process_and_configure_hardware`: Calls another routine, performs several table lookups to update offsets 16 and 46 of a structure in a6, writes a word to the DMA control register, and modifies a structure in a5 using values from a6.
- `reset_hardware_and_update_interrupt`: Resets specific bits in the CIAA and INTENA registers, updates a global variable with the address of another routine, and writes a new value to the hardware interrupt vector.
- `update_audio_registers_3`: Checks a longword at a global address, then writes a word from a table to the audio registers, sets bit 3 of a register, and writes a longword to a hardware register before returning from interrupt.
- `update_audio_registers`: Checks a global variable and, based on its bits, copies several longwords and words from a table to the audio registers, then sets a global variable and a value at a specific offset.
- `configure_audio_and_interrupt_handler`: Writes specific values to a table in memory, sets bit 7 of a byte at offset 1070 of a structure, and updates the hardware interrupt vector with a new routine address before enabling interrupts via the hardware register.
- `update_audio_hardware_registers`: Writes a 16-bit value from a table to the audio hardware register and sets bit 13 of the DMA control register, then returns via an exception return.
- `update_audio_registers_2`: Checks a bitmask in a global variable to selectively copy 32-bit and 16-bit values from a table into the audio hardware registers, then clears a byte and sets a word at a specific memory location before returning.
- `update_dma_control_registers`: Updates a structure in memory based on bitwise checks of a global variable and then enters a loop that writes values to the DMA control register through a pointer.
- `execute_jump_table`: Calls another routine, then reads a byte from offset 2 of a structure in a6 and uses it to index a table to perform an indirect jump via a2.
- `copy_word_and_branch`: Copies a word from offset 16 of a structure in a6 to offset 6 of a structure in a5, then branches to another routine if d0 is 7 or 10, otherwise it returns.
- `fetch_table_word`: Calculates an index by masking a byte from a6 with d5, adding a word from a6, and doubling the result, then uses this index to move a word from a table in a2 to offset 6 of a structure in a5.
- `fetch_table_value`: Calculates an index by masking a byte from a6 with d5, adding a word from a6, and doubling the result, then uses this index to move a word from a table at an offset of a2 into a location at offset six of a5.
- `copy_word_from_offset`: Moves a word from offset sixteen of a structure in a6 to offset six of a structure in a5 and returns.
- `update_structure_value`: Decrements a byte from offset 1 of a structure in a6, checks bit 4 of a byte at offset 18, and if conditions are met, fetches a word from a table using a2 and updates offset 24 of the structure in a6, finally setting offset 22 to 1 if a comparison fails.
- `process_complex_data`: Performs bitwise operations on bytes from a structure, calculates a value using a multiplication, and updates a byte at offset nine of a5 and a byte at offset twenty-nine of a6.
- `update_offset_and_pointer`: Moves a byte from offset 3 of a structure in a6 to offset 32 of the same structure, then performs arithmetic on a 16-bit value at offset 8 of the structure in a6 and updates a 32-bit value at offset 4 of the structure in a6.
- `set_memory_offset`: Sets a 16-bit value at offset eight of the structure in a6 to one.
- `update_status_bytes`: Calculates a value by subtracting one from a byte at offset 3 of a structure in a6, doubling it, and storing it at offset 5ba of a structure in a2, then clears a byte at offset 694 of a2 and sets a byte at offset 695 of a2 to 255.
- `clamp_byte_value`: Compares a byte from a structure at offset 3 in a6 with a constant, then stores the smaller of the two into offset 13 of the structure in a6 and offset 9 of the structure in a5.
- `calculate_and_store_byte`: Calculates a value based on a byte at offset 3 of a structure in a6 and a byte in d5, then stores the result at offset 1684 of a structure in a2 and sets the byte at offset 1685 of the same structure to 255 if the value is less than 63.
- `write_byte_to_table`: If a byte at offset 3 of a structure in a6 is not zero, it clears a byte at offset 692 of a table in a2 and stores the byte from offset 3 into offset 693 of that same table; otherwise it returns.
- `dispatch_jump_table`: Masks a byte from a structure in a6 with d6, shifts it right by 4, and uses the result to index into a table to perform an indirect jump via a2.
- `update_global_from_ciaa_pra`: Reads a byte from a structure in a6, masks bit zero, shifts it left by one, and ORs the result into the CIAA_PRA register after clearing bit one of that register.
- `update_masked_byte`: Performs bitwise AND operations between a byte from a structure in a6 and values in d5 and d6, then stores the result into offset 31 of the structure in a6.
- `adjust_structure_values`: Calculates a value by ANDing a byte from a structure in a6 with d5, then updates a 16-bit word at offset 46 of the structure in a6 by subtracting the byte at offset 18 and adding the calculated value.
- `update_pointer_and_value`: Increments a byte value using a table lookup and updates a pointer to a new location if a sign bit is set.
- `wait_for_vblank_change`: Polls the VHPOSR register in a loop until the byte value at offset 4 from the current register reading differs from the current reading.
- `initialize_hardware_state`: Performs several function calls via pointers in a structure, modifies global variables, and sets specific hardware registers including DMACON, INTENA, and INTENAR.
- `execute_indirect_calls`: Performs a series of indirect function calls using values retrieved from memory and global variables.
- `initialize_graphics_library`: Checks bits in a global variable to conditionally call functions, then enters a loop that calls a function repeatedly until it returns a non-zero value.
- `divide_and_mask`: Divides a 16-bit word in d0 by a 16-bit word in d1, masks bit zero of the quotient in d0, and performs a bitwise OR on d0 with zero.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `clear_and_copy_byte`: Clears register d0 three times, clears bit 1 of a byte at offset 2 of a structure in a4 using d2 as an index, writes the value 58320 to an offset of a1, and moves a byte from a5 to d1 while incrementing a5.
