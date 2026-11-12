# Game 711 (blob (1993)(core design)[cr dcs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00711). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909088 top-level bytes in 55 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 29 of 29 units identical (bytes and relocations), 27 raw-word lines |
| Functions named and described | pass, 229 of 229 named (0 author symbols), 229 of 229 described (10 by rule, 184 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1000 of 1000 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3705 of 3705 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 376394 |
| code | 7962 |
| data | 112362 |
| padding | 6866 |
| unused | 405504 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Blob (1993)(Core Design)[cr DCS].adf:blob | 3 | 65 | 1 |
| Assign | 2 | 117 | 2 |
| Blob (1993)(Core Design)[cr DCS].adf:INTRO | 2 | 267 | 8 |
| explode.library | 1 | 521 | 29 |
| Blob (1993)(Core Design)[cr DCS].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 64 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
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
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1167 | 70 |
| Blob (1993)(Core Design)[cr DCS].adf:INTRO#unpacked | 6 | 1211 | 90 |

## Cases handled

- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 52 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 546 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- blob (1993)(core design)[cr dcs]/Blob (1993)(Core Design)[cr DCS].adf:blob: Imploder Library (xfd) stream found by ancient scan (27936 -> 81288 bytes) but only 0 of 27 sampled windows are in reference RAM: not used
- blob (1993)(core design)[cr dcs]/Blob (1993)(Core Design)[cr DCS].adf:INTRO: crunched executable decrunched by XFDMaster (PowerPacker 4.0 (xfd), 33072 -> 75288 bytes), 22 of 22 windows confirmed in reference RAM: recovered as a hunk executable
- self-modifying code at an address (39 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (62 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-lgpwide) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- blob (1993)(core design)[cr dcs]/Blob (1993)(Core Design)[cr DCS].adf:blob: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- blob (1993)(core design)[cr dcs]/Blob (1993)(Core Design)[cr DCS].adf:INTRO: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Blob (1993)(Core Design)[cr DCS].adf:INTRO
- Blob (1993)(Core Design)[cr DCS].adf:INTRO#unpacked

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

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_functions`: Clears d2 and d3, then calls a library function five times with d1 loaded with different values from a table, before jumping to a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_unidentified_library_function`: Makes one call through a library function table entry at a fixed negative offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_with_values_alt`: Repeatedly clears d2 and d3, calls a library routine at offset -222 of a6, and loads one of five different longwords into d1 before jumping to a hardware address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_main_loop`: Sets the coprocessor and DMA registers, calls another routine, updates a function pointer in a structure, sets interrupt registers, and enters several loops that call other routines and check the CIAA port and other conditions before restoring a function pointer and returning.
- `compare_masked_value`: Checks if the value at offset 4 of the structure in a5, when masked with a fixed address, equals a fixed address, and returns based on the result.
- `set_interrupt_request_2`: Saves status and registers, loads d0 with 15 and d1 with a byte from a global variable, calls another routine, sets bit 5 of the interrupt request register, and returns from interrupt.
- `test_bit_and_branch`: Writes 33792 to offset 96 of the structure in a5 and branches based on the state of bit 14 of offset 2 of the structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_library_call_and_load`: Loads a pointer from a structure in a5, calls another routine, and if the result is not zero, calls another routine, copies data from a table into a structure in a5, and then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_interrupt_state`: Loads a byte from a global variable into d1, calls another routine, restores registers, restores the status register, sets bit 5 of the interrupt request register, and returns from interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fetch_and_store_data`: Calls a routine via a pointer in a6, stores the result in a global variable at offset 36 of a5, and if the result is not zero, calls another routine, updates offset 44 and 46 of a5 using data from a pointer derived from the value at offset 36 of a5, and then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_interrupt_state_2`: Loads a byte from a global variable into a register, calls another routine, restores all registers from the stack, re-enables interrupts via the status register, sets bit 5 of the interrupt request register, and returns from an interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_conditional_data`: Calls a routine via a pointer in a6, stores the result in a global variable at offset 36 of a5, and if the result is not zero, calls another routine, copies a word and a long word from a table pointed to by the result into offsets 44 and 46 of a5, and if the word equals a specific constant, stores a long word in a global variable before calling another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_and_restore_interrupts`: Calls another routine, restores registers, restores the status register, sets bit 5 of the interrupt request register, and returns from interrupt.
- `update_hardware_register_loop`: Writes a 16-bit value to an offset of 96 from a pointer in a6, then loops while bit 14 of a byte at offset 2 of that same pointer is set, finally writing a different 16-bit value to the same offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_data_on_flag`: If a zero flag is not set, calls a routine via a pointer in a6, copies a word and a long word from a table pointed to by the value at offset 36 of a5 into offsets 44 and 46 of a5, and if the word equals a specific constant, stores a long word in a global variable before calling another routine; otherwise it calls another routine via a pointer in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_interrupt_request`: Restores the status register and sets bit 5 of the interrupt request register, then returns from interrupt.
- `update_hardware_register_loop_exec`: Writes a 16-bit value to an offset of 96 from a pointer in a5, then loops while bit 14 of a byte at offset 2 of that same pointer is set, finally writing a different 16-bit value to the same offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_header`: Copies a 16-bit value and a 32-bit value from a pointer in a0 to a structure in a5, then either sets a global variable and a flag if the first 16-bit value is 19993, or calls another routine and loops while bit 6 of a byte at offset 2 of the structure in a5 is set before writing 1024 to offset 150 of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_and_update_register`: Performs a conditional branch, checks bit 14 of a byte at offset 2 of a pointer in a5, and if that bit is not set, writes a 16-bit value to an offset of 96 from a5 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_header_data`: Copies a 32-bit value from a pointer in a0 to a structure in a5, then either sets a 32-bit value in a structure in a1 if the first 16-bit value is 19993, or calls another routine and loops while bit 6 of a byte at offset 2 of the structure in a5 is set before writing 1024 to offset 150 of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_bit_and_write`: Checks bit 14 of a byte at offset 2 of a pointer in a5; if the bit is not set, it writes a 16-bit value to an offset of 96 from a5 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_and_update_table_2`: Checks a value at a pointer in a0, potentially calls another routine, and if certain conditions are met, copies a 32-bit value to a structure in a1, calls another routine, and then loops to copy data from a source to a destination before performing a loop that writes 32-bit values to a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_tables_and_pointers`: Fills a table with a 16-bit value using a loop, then performs a second loop that writes a 32-bit value split into two 16-bit parts to a table using a pointer in a1, incrementing the value and the pointer in each iteration.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_with_context`: Pushes d7 to the stack, loads an address into d7, calls another routine, restores d7 from the stack, and branches to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_data_and_populate_table`: Copies a sequence of 16-bit words from a source to a destination using a loop, then populates a table with five 32-bit values by splitting each value into two 16-bit words at specific offsets within a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_and_branch`: Calls a function pointer from a table and then branches to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_table_values`: Runs a loop six times, where each iteration adds 7744 to a value in d0, stores the low word of d0 at offset 2 of a table, and stores the high word of d0 at offset 6 of the same table.
- `store_pointer_to_global`: Stores the value of a global variable into another global variable and returns.
- `process_input_buffer`: Manages a buffer by either decrementing a counter, updating a pointer with a new value from a table, or copying bytes from a table to a destination using nested loops and specific offsets.
- `load_buffer_parameters_one`: Loads two pointers into a0 and a1, sets d7 to 31 and d6 to 4, and calls another routine.
- `load_buffer_parameters_two`: Loads two different pointers into a0 and a1, sets d7 to 31 and d6 to 4, and calls another routine.
- `update_buffer_values`: Increments a byte at a global address and, if it matches a value in d6, enters a loop to compare and add or subtract values from a buffer using masked nibbles.
- `mask_low_nibbles`: Masks the lower 4 bits of d6 and d5 and stores the results in d3 and d2, then sets d4 to 1 and returns.
- `mask_middle_nibbles`: Masks bits 4 through 7 of d6 and d5 and stores the results in d3 and d2, then sets d4 to 16 and returns.
- `mask_high_nibbles`: Masks bits 8 through 11 of d6 and d5 and stores the results in d3 and d2, then sets d4 to 256 and returns.
- `branch_to_offset_1bc`: Performs a relative branch to another routine.
- `branch_to_hardware_register`: Performs a short branch to a location within the hardware register memory space.
- `branch_to_audio_management`: Performs a short branch to a location within another routine that calls manage_audio_dma.
- `manage_audio_dma`: Contains multiple subroutines that clear hardware registers, configure DMA channels and audio lengths from a table, and update audio channel status based on a value in d0.
- `process_audio_channels`: Checks bits 0 through 3 of a byte at offset 1692 of a structure in a2 to conditionally call another routine with specific hardware register addresses, then performs an indirect jump using a value from a table.
- `update_object_state_and_dispatch`: Fetches an object pointer from a table using an index in d1, populates a structure in a6 with data from a table in a3, and performs an indirect jump to a routine based on a value in a6.
- `update_display_parameters`: Updates a structure by fetching a word from a table based on a byte at offset 18 of the structure in a6, modifies a value at offset 46 of the structure in a6 using an offset from a table, writes a word to the DMA controller address register, and performs bitwise operations on bytes at offsets 27 and 29 of the structure in a6 based on bits 2 and 6 of a byte at offset 30.
- `initialize_audio_dma_and_interrupts`: Clears the CIAA interrupt request register and control register, sets the CIAA control register to 17, and configures the audio DMA channels and interrupt enable bit.
- `configure_dma_and_interrupt`: Tests a byte at a global address, then writes a value to the DMA control register, sets bit 3 of the interrupt request register, and stores a pointer in a global variable before returning from interrupt.
- `configure_audio_channels`: Checks a global variable to conditionally update the address and length registers for four audio channels using data from a table, clears a byte at a global location, sets a byte to 127, clears the CIAA control register, sets the CIAA interrupt request register to 127, and performs an exception return.
- `initialize_audio_dma_and_interrupts_2`: Configures the CIA interrupt control and timer registers, enables DMA and audio channels via the custom chip, and updates a global function pointer before branching to another routine.
- `configure_dma_alt`: Tests a byte at a global address, then writes a value to the DMA control register, sets bit 13 of the interrupt request register, and stores a pointer in a global variable before returning from interrupt.
- `configure_audio_channels_alt`: Checks a global variable to conditionally update the address and length registers for four audio channels using data from a table, clears a byte at a global location, sets a byte to 127, sets the CIAB control register to 0, sets the CIAB interrupt request register to 127, and performs an exception return.
- `update_audio_channels`: Enables DMA and configures four audio channels by loading channel addresses and lengths from a table into the custom chip registers.
- `jump_via_table`: Calls a subroutine, then checks a byte at offset 2 of a structure in a6 and jumps to an address calculated from a table using that byte.
- `copy_word_conditionally`: Copies a 16-bit word from offset 16 of a structure in a6 to offset 6 of a structure in a5 if d0 is not 7 or 10, otherwise it returns.
- `fetch_indexed_word`: Calculates an index by adding a value from a table to the result of a bitwise AND between d5 and a byte from a table, then stores a 16-bit word from that table into offset 6 of a structure in a5.
- `fetch_table_entry`: Calculates an index by adding a value from a table to the result of a bitwise AND between d5 and a byte from a table, then stores a 16-bit word from a larger table into a destination at offset 6 of a6 into a destination at offset 6 of a5.
- `copy_word_from_table`: Copies a 16-bit word from offset 16 of a table into offset 6 of another structure and returns.
- `decrement_structure_offset`: If a byte at offset 1682 of a structure in a2 is zero, it subtracts a value derived from a byte at offset 3 of a structure in a6 from offset 16 of a6, clamping the result to a minimum of 113, then copies the result to offset 6 of a structure in a5 and returns.
- `increment_structure_offset`: If a byte at offset 1682 of a structure in a2 is zero, it adds a value derived from a byte at offset 3 of a structure in a6 to offset 16 of a6, clamping the result to a maximum of 856, then copies the result to offset 6 of a structure in a5 and returns.
- `update_offset_from_table`: If a bit is set in a byte at offset 18 of a structure in a6 and a value derived from offset 1 of a6 is non-zero, it fetches a word from a table using an index and adds it to offset 24 of a structure in a6, then compares this to offset 16 of a6 and sets a byte at offset 22 of a6 based on the result.
- `clear_word_at_offset`: Clears a 16-bit word at offset 24 of a structure and returns.
- `transfer_byte_and_clear`: If a byte at offset 3 of a structure in a6 is non-zero, it copies that byte to offset 23 of the structure in a6 and clears the byte at offset 3; otherwise it branches to another routine.
- `update_counter_and_value`: If a 16-bit word at offset 24 of a structure in a6 is non-zero, it adds a byte from offset 23 to offset 16 of a6 and, if a specific condition involving a table and offset 46 is met, updates offset 16 and clears offset 24.
- `update_counter_and_word`: Subtracts d0 from a 16-bit value at offset 16 of a structure in a6, potentially updates a counter at offset 24, and if a bitwise AND between d5 and a byte at offset 31 of a6 is non-zero, copies a word from a table into offset 6 of a structure in a5.
- `store_word_to_offset`: Stores a 16-bit word from d2 into offset 6 of a structure in a5 and returns.
- `update_structure_bits`: Updates a byte at offset 26 of a structure in a6 by performing bitwise operations using bits from offset 3 of the same structure and values in d5 and d6.
- `calculate_scaled_offset_2`: Loads a byte from offset 27 of a structure in a6, uses it to index into a table via a4, and calculates a value based on bits in offset 30 of the structure and a value in d5.
- `calculate_subtraction`: Tests a byte at offset 27 of a structure in a6; if positive, it branches, otherwise it sets d2 to the result of subtracting d0 from a constant and branches.
- `move_byte_and_branch`: Moves a byte from register d0 to register d2 and then performs a relative branch.
- `calculate_scaled_value_2`: Loads a byte from a table at offset d0 of a4, multiplies it by a value derived from a byte at offset 1a of a6 and d5, and adds the result to a 16-bit word at offset 10 of a6.
- `subtract_and_advance_phase`: Subtracts d2 from d0 and stores the result in offset 6 of a structure in a5, then advances a phase byte in the structure by an amount taken from the high nibble of a neighbouring byte.
- `call_and_branch_3`: Calls another routine and then branches to another routine.
- `calculate_and_store_offset`: Updates a byte at offset 28 of a structure in a6 using bitwise logic, calculates a value by multiplying a table entry by a masked byte from offset 28, and stores the result in a byte at offset 9 of a structure in a5.
- `adjust_offset_and_flag`: Checks a byte at offset 3 of a structure in a6, and if non-zero, performs arithmetic involving a value at offset 8 and a bit-shifted version of that value to update a 32-bit long at offset 4 of a6.
- `set_and_return_flag`: Sets a 16-bit word at offset 8 of a structure in a6 to 1 and returns.
- `update_scaled_values`: Calculates a value by subtracting 1 from a byte at offset 3 of a structure in a6, doubling it, and storing it at offset 570 of a structure in a2, then clears offset 1684 and sets offset 1685 of the structure in a2.
- `clamp_and_store_byte`: Compares a byte at offset 3 of a structure in a6 with a global variable, stores the smaller of the two in offset 19 of the structure in a6 and offset 9 of a structure in a5, and returns.
- `update_scaled_byte`: Calculates a value by combining a byte from offset 3 of a6 with its own high nibble and a bitwise AND of d5, then stores the result at offset 1684 of a structure in a2 and sets offset 1685 to 255 if the result is less than 63.
- `execute_indirect_jump`: If a byte at offset 692 of the structure in a2 is zero, it clears that byte, sets it to a byte from offset 3 of the structure in a6, and returns; otherwise, it calls another routine, loads a byte from offset 2 of the structure in a6 into d7, doubles d7, uses the result to index a table to load a new value into d7, and jumps to the address at offset d7 of the structure in a2.
- `execute_jump_table`: Calculates an index using a byte at offset 3 of a structure in a6 and d6, then performs an indirect jump using a table at a relative offset based on that index.
- `update_global_from_ciaa_pra`: Masks bit zero of the byte at offset 3 of the structure in a6, shifts it left by one, and ORs the result into a global variable after clearing bit one of that variable.
- `update_bitmask_offset_1f`: Clears bits in the byte at offset 31 of a structure in a6 using d6, then sets bits using d5 and the byte at offset 3 of the structure in a6.
- `update_bitmask_offset_1e`: Clears bits in the byte at offset 30 of a structure in a6 using d6, then sets bits using d5 and the byte at offset 3 of the structure in a6.
- `adjust_counter_with_mask`: Subtracts a 16-bit word at offset 18 of a structure in a6 from a 16-bit word at offset 46 of the same structure, then updates offset 18 with a masked byte and adds that masked byte to the 16-bit word at offset 46.
- `decrement_and_store_byte`: Checks a byte in a global variable and a bitmask in d5; if conditions are met, it decrements a byte at offset 34 of the structure in a6 and stores a byte from offset 33 of the structure in a6 into a global variable via a pointer.
- `store_byte_and_branch`: Stores a byte from d0 into offset 22 of a structure in a6 and branches to another routine.
- `shift_and_store_byte`: Loads a 16-bit word from a location relative to the program counter, shifts it right by three bits, and stores the resulting byte at offset 33 of the structure in a6.
- `modify_structure_field`: Loads a byte from offset 3 of the structure in a6, shifts it left by four bits, and performs a bitwise OR with the byte at offset 30 of the structure in a6, which is itself masked with the value in d5.
- `configure_dma_and_interrupts`: Checks several conditions involving a byte at offset 3 of a6 and a byte at offset 1 of a6, then updates the DMA controller and calls another routine twice with different values from a6 stored in a5.
- `test_and_branch_mask`: If the byte at offset 692 of the structure in a2 is zero, it loads a byte from offset 3 of the structure in a6 into d0, masks it with d5, and branches to another routine; otherwise it returns immediately.
- `test_and_branch_mask_alt`: If the byte at offset 692 of the structure in a2 is zero, it loads a byte from offset 3 of the structure in a6 into d0, masks it with d5, and branches to another routine; otherwise it returns immediately.
- `clear_status_bytes`: If a byte from offset 3 of the structure in a6, masked with d5, matches a constant, it clears the byte at offset 13 of the structure in a6 and the byte at offset 9 of the structure in a5 before returning; otherwise it returns immediately.
- `check_byte_conditions`: If a byte from offset 3 of the structure in a6, masked with d5, matches a constant and the byte at offset 1 of the structure in a6 is non-zero, it branches to another routine; otherwise it returns immediately.
- `increment_and_store_byte`: If the byte at offset 692 of the structure in a2 is zero, it loads a byte from offset 3 of the structure in a6 into d0, masks it with d5, and if the byte at offset 699 of the structure in a2 is zero, it increments d0 and stores the result into offset 698 of the structure in a2 before returning; otherwise it returns immediately.
- `update_pointer_table`: Reads a byte from offset 31 of the structure in a6, uses it to index into a table to find a value, adds that value to offset 35 of the structure in a6, and if bit 7 of that byte is set, clears the byte and updates a pointer at offset 36 of the structure in a6 by incrementing it and potentially wrapping it based on a limit.
- `wait_for_vpos_change`: Polls the vertical position register in the custom chip, waiting until the value increases by four compared to the initial reading.
- `initialize_hardware_registers`: Calls several library routines, modifies the DMA controller, interrupt enable, and interrupt acknowledge registers, and sets several custom hardware register values to specific constants.
- `reset_custom_registers`: Sets several custom chip registers to a specific value, calls a routine via a pointer in a5, calls a library function to permit the library, calls another routine, closes the library, and updates a custom register with a value from the library structure.
- `initialize_graphics_library`: Checks various bits in the execution base to perform supervisor calls, then attempts to open the graphics library by checking the Denise chip ID and calling an open routine in a loop.

### Blob (1993)(Core Design)[cr DCS].adf:INTRO#unpacked

- `initialize_system_state`: Sets up a structure in a5, calls several routines to initialize hardware and tables, performs a jump to an address from a table, and eventually updates a pointer in a structure before returning.
- `wait_for_specific_bitmask`: Loops until a 32-bit word at offset 4 of the structure in a5, when masked with 65280, does not equal a fixed address, and then loops until that same masked value equals a fixed address.
- `update_audio_interrupt`: Loads a byte from an offset of 16 into d1, calls another routine, writes 32 to the interrupt request register, and returns.
- `toggle_structure_word`: Writes 33792 to offset 150 of the structure in a5, waits for bit 14 of offset 2 to be cleared, and then writes 1024 to offset 150 of the structure in a5.
- `fill_and_setup_table`: Fills a table with the value 38 using a loop, then populates another table with pointers and high-word components derived from a base address incremented by 7744 in each iteration.
- `copy_pointer_to_structure`: Loads the address of a location relative to the program counter into a0 and copies it to the address stored at offset 4 of the structure in a1.
- `decompress_data_stream`: Processes a data stream by reading bytes, performing bitwise operations, and using nested loops to copy blocks of data from one location to another based on the decoded values.
- `setup_buffer_comparison`: Loads two memory addresses into registers a0 and a1, sets d7 to 31 and d6 to 4, calls another routine, and returns.
- `setup_buffer_comparison_alt`: Loads memory addresses 1322 and 90 into registers a0 and a1, sets d7 to 31 and d6 to 4, calls another routine, and returns.
- `compare_and_adjust_buffer`: Iterates through a buffer, comparing words from two tables and adjusting values in a third table by adding or subtracting a constant based on the comparison results.
- `mask_lower_nibble`: Copies the contents of d6 and d5 to d3 and d2, then masks the lower four bits of d3 and d2 and returns.
- `mask_upper_nibble`: Copies the contents of d6 and d5 to d3 and d2, then masks bits four through seven of d3 and d2 and returns.
- `mask_byte_high`: Copies the contents of d6 and d5 to d3 and d2, then masks bits eight through fifteen of d3 and d2 and returns.
- `branch_to_address_2`: Performs a relative branch to a specific address.
- `jump_to_audio_update`: Performs a short branch to another routine.
- `branch_to_address`: Performs a relative branch to another routine.
- `check_byte_zero`: Pushes d0 to the stack, loads a byte from a constant offset, and returns d0 if that byte is not zero; otherwise it restores d0 from the stack and returns.
- `configure_audio_parameters`: Sets a byte at offset 1691 of a structure in a3 to 40, and if d1 is non-zero, clears that byte and performs conditional assignments to offsets 1676 and 1690 of the structure in a3 based on the value in d0.
- `setup_audio_tables`: If d0 is not 1, it loads a pointer from a relative address into a1, stores it at offset 1708 of a structure in a3, and then copies two words from a table into memory using a1 as a pointer.
- `initialize_audio_hardware`: If d0 is 2, it stores a routine address in a structure, populates a table with values from a0, sets a bit in a hardware register, clears several values in a structure, and performs two loops to initialize a table and audio volumes before clearing DMA channels and setting hardware registers.
- `update_audio_channel_parameters`: Stores a byte from d0 and a byte from d1 into offsets 1692 and 1694 of the structure in a2, increments a byte at offset 1682 of that structure, and performs conditional checks to either call another routine or branch to a different location.
- `call_audio_update_and_branch`: Calls update_audio_channels_2 and then branches to another routine.
- `update_audio_channels_2`: Checks bits 0 through 3 of a byte at offset 1692 of the structure in a2 and, if set, loads a hardware register address into a5 and calls another routine with a specific value in a6.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `execute_audio_callbacks`: Checks bits in a byte at offset 1692 of a structure in a2 and calls one of four subroutines using addresses from a table, then jumps to an address in a3.
- `update_object_properties`: Fetches an object pointer from a table using an index in d1, then populates a structure in a6 with data from a table in a3, including an offset calculation and various word and byte values.
- `execute_jump_table_2`: Clears a byte at offset 44 of a structure in a6, copies a longword from offset 4 to offset 10 and offset 36, copies a word from a table into offset 14, and then performs an indirect jump using a value from a table.
- `call_and_branch_alt`: Calls another routine and then performs a relative branch to a different location.
- `call_and_branch_2`: Calls another routine and then performs a relative branch.
- `process_table_and_dma`: Calculates a value using a byte from a structure and a table, stores it in a structure, and if a specific condition is met, updates the DMA control register and writes data to a pointer stored in a structure before calling another routine.
- `initialize_audio_dma_and_interrupts_3`: Configures the CIAA control and interrupt registers, enables the audio DMA controller, and sets the custom chip interrupt enable register.
- `configure_dma_and_interrupt_2`: Writes a value to the DMA control register, sets bit 3 of the interrupt request register, and stores a pointer to a table in a global variable before returning via an exception return.
- `configure_audio_channels_2`: Checks a byte at a specific memory location and, based on its bits, copies four sets of channel and length data from a table to the hardware audio registers, then clears a byte at a specific address and sets a bit in the CIAA interrupt register before returning via an exception.
- `initialize_audio_dma_and_interrupts_4`: Configures the audio DMA control registers and interrupt request registers, enables the audio DMA via the custom chip, and stores the address of another routine into a global variable before branching to a subsequent routine.
- `configure_dma_and_interrupt_bit_13`: Writes a value to the DMA control register, sets bit 13 of the interrupt request register, and stores a pointer to a table in a global variable before returning via an exception return.
- `configure_audio_channels_b`: Checks a byte at a specific memory location and, based on its bits, copies four sets of channel and length data from a table to the hardware audio registers, then clears a byte at a specific address and sets a bit in the CIAB interrupt register before returning via an exception.
- `update_audio_channels_and_volumes`: Enables DMA and configures the four audio channels by loading their respective addresses and lengths from a table.
- `update_volume_table`: Checks two bytes at offsets of a structure in a2, and if certain conditions are met, iterates through a table to update volume values at hardware register offsets based on values stored in a structure in a0.
- `dispatch_via_table`: Calls another routine, then uses a byte from offset 2 of a structure in a6 to index into a table and performs an indirect jump using a2 and the resulting value.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `copy_word_from_offset`: Copies a 16-bit word from offset 16 of a structure in a6 to offset 6 of a structure in a5 and returns.
- `select_offset_from_table`: Calculates a value by dividing a byte from a PC-relative location by 3, then uses a byte from offset 3 of a structure in a6 and an offset from 46 of a6 to index into a table and store a 16-bit word into offset 6 of a structure in a5.
- `fetch_table_value`: Calculates an index using a byte from a structure in a6, a byte in d5, and a word from a structure in a6, then copies a 16-bit word from a table at a2 using that index to offset 6 of a structure in a5.
- `copy_word_from_offset_alt`: Copies a 16-bit word from offset 16 of a structure in a6 to offset 6 of a structure in a5 and returns.
- `decrement_and_clamp_value`: If a byte at offset 1682 of a structure in a2 is zero, it stores d5 into offset 1687 of that structure, subtracts a masked byte from offset 3 of a structure in a6 from a 16-bit value at offset 16 of a6, and clamps the result to a minimum of 71 before storing it at offset 6 of a structure in a5.
- `increment_and_clamp_value`: If a byte at offset 1682 of a structure in a2 is zero, it stores d5 into offset 1687 of that structure, adds a masked byte from offset 3 of a structure in a6 to a 16-bit value at offset 16 of a6, and clamps the result to a maximum of 856 before storing it at offset 6 of a structure in a5.
- `update_value_from_table`: Calculates a value using a byte from offset 1 of a structure in a6 and a byte from offset 18 of a6, fetches a word from a table using a2, stores it in offset 24 of a structure in a6, and sets offset 22 of a structure in a6 to 1 if a comparison fails.
- `clear_word_at_offset_2`: Clears a 16-bit word at offset 24 of a structure in a6 and returns.
- `update_byte_value`: Checks a byte at offset 3 of a structure in a6; if it is non-zero, it copies that byte to offset 23 of the structure and clears the byte at offset 3.
- `update_structure_values`: Checks a word at offset 24 of a structure in a6; if non-zero, it performs arithmetic on offset 16 of the structure using a byte from offset 23 and updates offset 46 based on a value from a table in a2, potentially clearing offset 24 and updating offset 16.
- `decrement_and_store_word`: Subtracts a word from offset 16 of a structure in a6, performs several comparisons and updates to offset 46 and 24 of a structure in a6, and then either stores a word from a table into offset 6 of a structure in a5 or stores offset 16 of a structure in a6 into offset 6 of a structure in a5.
- `store_word_to_offset_2`: Stores a 16-bit word from d2 into offset 6 of a structure in a5 and returns.
- `update_structure_flags`: Updates a byte at offset 26 of a structure in a6 by performing bitwise operations with a byte at offset 3 of the same structure and values in d5 and d6, then returns.
- `calculate_scaled_offset_3`: Fetches a value from a table using an index derived from a byte at offset 27 of a structure in a6, scales it by a value at offset 26 of the structure and a value in d5, and adds the result to a 16-bit value at offset 16 of the structure.
- `calculate_signed_byte`: Checks a byte at offset 27 of a structure in a6; if it is negative, it sets d2 to the result of subtracting d0 from negative 255, otherwise it branches.
- `move_and_branch`: Copies a byte from d0 to d2 and then performs a relative branch.
- `calculate_scaled_offset`: Fetches a byte from a table using a4 and d0, multiplies it by a value derived from a byte at offset 1a of a structure in a6 and d5, and adds the result to a word from offset 10 of a structure in a6.
- `subtract_and_advance_phase_2`: Subtracts d2 from d0 and stores the result in offset 6 of a structure in a5, then advances a phase byte in the structure by an amount taken from the high nibble of a neighbouring byte.
- `call_clamping_routine`: Calls another routine and then performs a relative branch to another location.
- `branch_to_subroutine`: Calls a subroutine and then performs a relative branch.
- `update_clamped_offset`: Performs bitwise operations on bytes at offsets 3, 28, and 29 of a structure in a6 using d5 and d6, then calculates a scaled value using a table and bits from offset 29 and 30.
- `calculate_signed_byte_alt`: Checks a byte at offset 27 of a structure in a6; if it is negative, it sets d2 to the result of subtracting d0 from negative 255, otherwise it branches.
- `move_and_branch_alt`: Copies a byte from d0 to d2 and then performs a relative branch to a different location.
- `calculate_clamped_offset`: Fetches a byte from a table using a4 and d0, multiplies it by a value derived from a byte at offset 1c of a structure in a6 and d5, and adds the result to a byte from offset 13 of a structure in a6.
- `update_clamped_byte_and_phase`: Subtracts d2 from d0, clamps the result to a minimum of zero and a maximum defined by a global variable, stores the result in offset 9 of a structure in a5, and advances a phase byte in a structure in a6 using the high nibble of a byte at offset 28.
- `update_offset_and_pointer`: Reads a byte from offset 3 of the structure in a6 and, if non-zero, stores it at offset 32 of the structure, then calculates a value by shifting that byte left by 7 bits, compares it to a 16-bit word at offset 8, and if the value is smaller, subtracts it from the word at offset 8 and adds the doubled value to a 32-bit longword at offset 4 before returning.
- `set_word_to_one`: Stores the value 1 into a 16-bit word at offset 8 of a structure in a6 and returns.
- `clamp_and_store_byte_3`: Takes a byte from offset 3 of a structure in a6, shifts it, and adds it to offset 13 of a structure in a6, then clamps the result against a constant and stores the byte into offset 9 of a structure in a5.
- `decrement_byte_and_store`: Subtracts a masked byte from offset 3 of a structure in a6 from a byte at offset 19 of a6, clamps the result to zero if negative, and stores the byte in offset 9 of a structure in a5.
- `calculate_scaled_value`: Calculates a 16-bit value by taking a byte from offset 3 of a structure in a6, subtracting 1, doubling it, and storing it at offset 1434 of a structure in a2, then clears offset 1684 and sets offset 1685 of that structure to 255.
- `clamp_and_store_byte_2`: Compares a byte from offset 3 of a structure in a6 against a global variable, selects the smaller of the two, stores it in offset 19 of a structure in a6, and stores it in offset 9 of a structure in a5.
- `store_scaled_byte`: Calculates a value by scaling a byte from offset 3 of a structure in a6 and adding a masked version of that byte, then stores the result into offset 1684 of a structure in a2 if it is less than 63.
- `update_status_byte`: If a byte at offset 3 of a structure in a6 is not zero, it clears a byte at offset 692 of a2 and stores the byte from offset 3 into offset 693 of a2.
- `execute_jump_table_entry`: Fetches a byte from offset 2 of a structure in a6, uses it to index into a table of word offsets, adds the resulting offset to a6, and performs an indirect jump to the calculated address.
- `modify_offset_value`: Performs arithmetic on a 16-bit value at offset 46 of a structure in a6 by subtracting a byte from offset 18 and then adding a bitmasked byte from offset 3.
- `process_byte_and_store`: Tests a byte at offset 692 of a structure in a2; if non-zero, it performs a conditional branch, otherwise it masks a byte from offset 3 of a6 with d5, decrements a byte at offset 22 of a6, and stores a byte from offset 21 of a6 into a structure in a2.
- `store_byte_and_branch_2`: Stores a byte from register d0 into a global variable at offset 34 of the structure in a6 and branches to another routine.
- `shift_and_store_value`: Loads a 16-bit word from a table using the program counter, shifts it right by 3 bits, and stores the result in a byte at offset 33 of a structure in a6.
- `mask_and_shift_byte`: Takes a byte from offset 3 of a structure in a6, shifts it left by 4 bits, and performs an OR operation with a bitmasked byte at offset 30 of the same structure.
- `update_dma_configuration`: Checks several conditions involving a byte in a6, a byte at offset 1682 of a2, and a division result; if conditions are met, it updates the DMA controller, copies data from a structure in a6 to a pointer in a5, calls another routine, sets a bit in d5, updates the DMA controller again, calls another routine, and finally updates the pointer in a5 with data from a6.
- `mask_and_branch`: Tests a byte at offset 692 of a structure in a2; if non-zero, it branches, otherwise it masks a byte from offset 3 of a6 with d5 and branches.
- `check_byte_condition`: Tests a byte at offset 1682 of a2 and, if non-zero, performs a bitwise AND between a byte at offset 3 of a6 and d5 before branching to another routine.
- `clear_bytes_on_match`: If a byte at offset 3 of a structure in a6, when bitmasked with d5, matches a constant, it clears a byte at offset 19 of a6 and a byte at offset 9 of a5.
- `check_and_branch`: Masks a byte from offset 3 of a6 with d5, compares it to a byte from a table, and if they match and a byte at offset 1 of a6 is non-zero, it branches.
- `increment_register_on_condition`: If a byte at offset 692 of a2 is zero and a byte at offset 699 of a2 is zero, it increments a bitmasked byte from offset 3 of a6 and stores it in offset 698 of a2.
- `update_pointer_and_decrement`: Uses a byte from offset 31 of a6 to index into a table, adds the result to offset 53 of a6, and if a bit is set, updates a pointer at offset 36 of a6 and decrements the value at that pointer.
- `wait_for_vposr_change`: Polls the vertical position register until the value increases by 4.
- `initialize_hardware_registers_2`: Calls another routine, loads a pointer from a structure in a6, calls three library routines, calls another routine, and then performs multiple bitwise OR operations to update hardware registers and sets several custom register values including DMA and interrupt control.
- `disable_interrupts_and_dma`: Disables interrupts and DMA, loads values from a table into custom registers, calls a routine via a pointer, and then closes a library.
- `initialize_graphics_library_2`: Checks system flags to potentially call a supervisor routine, then attempts to open a specific version of the graphics library and stores the result in a global variable.
