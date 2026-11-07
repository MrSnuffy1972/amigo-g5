# Game 280 (Lotus2 v1.9 0497): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00280). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1018201 top-level bytes in 99 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 127 of 127 units identical (bytes and relocations), 69 raw-word lines |
| Functions named and described | pass, 1118 of 1118 named (0 author symbols), 1118 of 1118 described (9 by rule, 1105 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 7206 of 7211 (99.93%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 26512 of 26528 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 126869 |
| code | 87864 |
| data | 802698 |
| padding | 2 |
| unused | 768 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Lotus2.Slave | 1 | 1131 | 66 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 12572 | 657 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 630 | 30 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 60 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12128 | 356 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 7 |

## Cases handled

- Lotus2_v1.9_0497/Lotus2/Disk.1: raw disk image (931380 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- 122 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 353 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f150): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5964 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Lotus2_v1.9_0497/Lotus2/Disk.1: bytes 675746-675840 beside a logged load are part of a file the game's own loader reads (unpack_own loader replay)
- Lotus2_v1.9_0497/Lotus2/Disk.1: 122 logged loads cover 550682 bytes of the image
- Lotus2_v1.9_0497/Lotus2/Disk.1: chunks 22-42 (bytes 5632-11008) hold ASCII text
- Lotus2_v1.9_0497/Lotus2/Disk.1: chunks 266-361 (bytes 68096-92672) hold 4 packed streams unpacked by the game's own file loaders sub_B374 (an address, unit u4, file selected by d0) and sub_B3D8 (an address, unit u4, file selected by d0), replayed with WHDLoad DiskLoad served from the disk image and proven against its reference-run call, 100% of the bytes
- Lotus2_v1.9_0497/Lotus2/Disk.1: chunks 383-559 (bytes 98048-143360) hold 8 packed streams unpacked by the game's own file loader sub_B374 (an address, unit u4, file selected by d0), replayed with WHDLoad DiskLoad served from the disk image and proven against its reference-run call, 100% of the bytes
- Lotus2_v1.9_0497/Lotus2/Disk.1: chunks 996-1011 (bytes 254976-259072) hold 1 packed streams unpacked by the game's own file loader sub_B374 (an address, unit u4, file selected by d0), replayed with WHDLoad DiskLoad served from the disk image and proven against its reference-run call, 100% of the bytes
- Lotus2_v1.9_0497/Lotus2/Disk.1: chunks 1577-1783 (bytes 403712-456704) hold 2 packed streams unpacked by the game's own file loader sub_B374 (an address, unit u4, file selected by d0), replayed with WHDLoad DiskLoad served from the disk image and proven against its reference-run call, 100% of the bytes
- Lotus2_v1.9_0497/Lotus2/Disk.1: chunks 1788-1794 (bytes 457728-459520) hold 1 packed streams unpacked by the game's own file loader sub_B374 (an address, unit u4, file selected by d0), replayed with WHDLoad DiskLoad served from the disk image and proven against its reference-run call, 100% of the bytes
- Lotus2_v1.9_0497/Lotus2/Disk.1: chunks 2449-2528 (bytes 626944-647424) hold 2 packed streams unpacked by the game's own file loader sub_B374 (an address, unit u4, file selected by d0), replayed with WHDLoad DiskLoad served from the disk image and proven against its reference-run call, 100% of the bytes
- Lotus2_v1.9_0497/Lotus2/Disk.1: chunks 2640-2663 (bytes 675840-681984) hold 1 packed streams unpacked by the game's own file loader sub_B374 (an address, unit u4, file selected by d0), replayed with WHDLoad DiskLoad served from the disk image and proven against its reference-run call, 100% of the bytes
- Lotus2_v1.9_0497/Lotus2/Disk.1: chunks 2788-3638 (bytes 713728-931380) hold 64 packed streams unpacked by the game's own file loader sub_B374 (an address, unit u4, file selected by d0), replayed with WHDLoad DiskLoad served from the disk image and proven against its reference-run call, 100% of the bytes

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Lotus2.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `install_jump_table_entries_4`: The routine uses a pointer passed in A0 to call several routines from a table and to read a short value that selects a configuration path.
- `call_routine_and_store_word`: Calls another routine, stores a constant word through a pointer derived from an unassigned register, and returns.
- `xor_decrypt_buffer`: The routine conditionally runs a byte-wise XOR mixing loop over a buffer pointed to by A0 using data from a global variable and a length derived from D0, then updates a global buffer backwards via copy_words_from_pointer.
- `copy_words_from_pointer`: Reads a count from an argument, copies that many words from a pointer obtained from the argument into a global buffer backwards, updating a global variable with the count, and returns.
- `check_flag_and_dispatch`: Calls another routine unconditionally and returns zero unless a global variable is non-zero, in which case it calls another routine and returns its result.
- `check_flag_and_return`: The routine calls another routine unconditionally and returns zero unless a global variable is non-zero, in which case it calls another routine and returns its result.
- `copy_config_blocks_and_reverse`: If a global variable is non-zero the routine copies two blocks of word data from a table into a stack buffer via an indirect call, then reverses the order of two other word ranges in the table and writes a constant to a stack slot before calling another routine through the global variable and returning a value built from the stack.
- `swap_and_reverse_tables`: The routine copies two 16-bit tables from a fixed area into a temporary stack buffer, then copies them back in reverse order, and finally stores a 32-bit value and calls another routine via a global variable, returning a 64-bit value derived from the buffer.
- `load_config_words_and_dispatch`: The routine copies a range of words from a table backwards into a stack area, then copies another range backwards into the stack, stores a constant, saves an input value, calls a routine through a global variable and optionally updates the global variable and calls another routine before returning a combined value from the stack and the table.
- `copy_config_block`: The routine copies a block of words from a table in the data segment into a small stack buffer, then copies another block of words from a second table into the same buffer area, overwriting the first copy.
- `empty_routine`: The routine contains four instructions and performs no observable work; it ends immediately without accessing memory, registers, or calling another routine.
- `conditional_copy_and_dispatch`: If a global variable is zero, copies a fixed number of bytes from a global area to a memory area derived from an unassigned register and calls another routine; otherwise it dispatches through a function pointer, calls another routine, and conditionally calls a third routine, then returns the concatenation of two input registers.
- `transform_ascii_digits_2`: Iterates over a fixed number of lines and columns in a table referenced via a global variable, replaces each non-space character with a transformed digit and inserts line feeds, writing the results back into the table.
- `transform_ascii_digits`: The routine iterates over a fixed number of lines and columns in a table, replaces each non-space character with a transformed digit and inserts line breaks, writing the results back into the same table.
- `call_four_routines`: Calls four other routines in sequence and then returns.
- `call_through_pointer`: The routine calls a function located at an offset from the supplied parameter and then returns.
- `xor_decrypt_bytes`: The routine XORs a fixed number of bytes in a table with a decrementing short value starting at a constant, writing the results back into the same table.
- `copy_global_bytes_and_dispatch`: If a global variable is zero the routine copies 23 bytes from that global variable to a fixed address and then calls another routine.
- `call_indirect_and_conditional`: Calls a function through a pointer from a global variable, calls another routine, and conditionally calls another routine based on a global variable, returning the initial stack value.
- `self_recursive_stub`: The routine contains a self-recursive call repeated four times before returning, with no other observable effect.
- `call_indirect_via_pointer`: The routine receives a pointer argument and calls a function located at an offset from that pointer.
- `xor_decrement_table`: The routine XORs a block of bytes starting at an offset from a register with a decrementing short counter beginning at 263, writing the result back to the same bytes.
- `wait_for_flag_then_store_word`: The routine spins until a global variable’s high bit clears, then stores a constant word through a pointer derived from an unassigned register.
- `wait_for_flag_and_decrement`: The routine initializes a counter to 200 and repeatedly calls another routine to obtain a word, decrementing the counter each iteration.
- `wait_for_flag_then_store_word_2`: The routine spins until a global variable’s high bit clears, then stores a constant word through a different pointer derived from an unassigned register.
- `wait_for_flag_and_decrement_alt`: The routine initializes a counter to 200 and repeatedly calls another routine to obtain a word, decrementing the counter each iteration.
- `tiny_subi_rts`: Small routine of 2 instructions that subtracts 14 from (a7), then returns.
- `wait_for_masked_word`: The routine repeatedly calls another routine to obtain a word, compares it with the previous value and loops while the new value is greater or equal.
- `return_masked_word_from_pointer`: The routine contains an infinite loop that compares the same memory location, then returns a masked word read from an offset of an unassigned register.
- `decode_message_header_2`: The routine copies fields from an input structure into a global variable, conditionally copies a block of data using a shift-limited loop and then invokes another routine to compute parity and store results.
- `looped_shift_with_table_lookup`: The routine loads a short from a fixed offset of an unassigned register and repeatedly shifts an input value, using a table pointer when the shifted value becomes zero, decrementing the counter until it reaches minus one.
- `decode_message_header`: The routine copies fields from an input structure into a global variable, conditionally copies a block of data using a shift-limited loop and another block when a flag is set, accumulates a value derived from a short field, and then enters an infinite loop that repeatedly calls other routines via table lookups while checking a global flag and a counter, returning a combined value when a pointer reaches a limit.
- `looped_shift_with_table_lookup_2`: The routine prepares an input value with a bitwise OR, then repeatedly shifts it, using a table pointer when the value becomes zero, decrementing a short counter until it reaches minus one.
- `looped_shift_with_table_lookup_3`: The routine repeatedly shifts an input value, using a table pointer when the value becomes zero, decrementing a short counter derived from an input register until it reaches minus one.
- `copy_bytes_with_offset_minus_one`: The routine reads a short counter from a global variable, calls another routine, and then copies bytes from one buffer to another using masked indices that wrap, with the loop count decreased by one before copying.
- `copy_bytes_with_counter_2`: The routine reads a short counter from a global variable, calls another routine, and then copies bytes from one buffer to another using masked indices that wrap for the number of iterations given by the counter.
- `copy_bytes_with_offset_plus_one`: The routine reads a short counter from a global variable, calls another routine, and then copies bytes from one buffer to another using masked indices that wrap, with the loop count increased by one before copying.
- `copy_bytes_with_offset_plus_two`: The routine reads a short counter from a global variable, calls another routine, and then copies bytes from one buffer to another using masked indices that wrap, with the loop count increased by two before copying.
- `copy_bytes_with_offset_plus_three`: The routine reads a short counter from a global variable, calls another routine, and then copies bytes from one buffer to another using masked indices that wrap, with the loop count increased by three before copying.
- `copy_bytes_with_offset_plus_four`: The routine reads a short counter from a global variable, calls another routine, and then copies bytes from one buffer to another using masked indices that wrap, with the loop count increased by four before copying.
- `copy_bytes_with_variable_offset`: The routine calls another routine twice, reads a short counter from a global variable, and then copies bytes from one buffer to another using masked indices that wrap, with the loop count increased by a value derived from a register plus five before copying.
- `copy_bytes_with_offset`: The routine reads a short counter from a global variable, adds an offset derived from a register, then copies bytes from one buffer to another using a masked index that wraps at 16383.
- `shift_copy_with_counter`: The routine reads a short counter from a global variable, adds a constant and a register value, then repeatedly copies a byte from one offset in a buffer to another offset, decrementing both offsets with a mask and decrementing the counter until it reaches -1.
- `shift_copy_loop`: The routine reads a short counter from a global variable, adds a constant and a register value, then repeatedly decrements two masked indices and copies a byte from one offset in a buffer to another offset in the same buffer until the counter reaches a sentinel value.
- `rotate_buffer_with_counter`: The routine reads a short counter from a global variable, adds a constant and a register value, then repeatedly copies a byte from one masked index to another masked index inside the same buffer while decrementing the counter.
- `copy_bytes_with_counter`: The routine reads a short counter from a global variable and then copies bytes from one buffer to another using a masked index that wraps at 16384.
- `fill_bytes_with_constant_2`: The routine reads a short counter from a global variable, adjusts it with a register value, then writes a constant byte from a register into a buffer using a masked index that wraps at 16383.
- `fill_bytes_with_constant`: The routine loads a short counter from a global variable, adds an offset derived from a register, then writes a constant byte from a register into a buffer using a masked index that wraps at 16384, repeating until the counter reaches -1.
- `fill_bytes_limited_2`: The routine initializes a short counter to zero and then writes a constant byte from a register into a buffer using a masked index that wraps at 16383.
- `fill_bytes_limited`: The routine sets a short counter from a register plus one and writes a constant byte from a register into a buffer using a masked index that wraps at 16384, repeating until the counter reaches -1.
- `fill_bytes_limited_3`: The routine decrements a short counter on each iteration and writes a constant byte from a register into a buffer using a masked index that wraps at 16383.
- `expand_bytes_to_bits_and_scatter`: The routine processes sixteen bytes from a pointer, building six bit-accumulation words from carry results, then scatters those words to memory locations derived from an unassigned register and a global variable, and returns an input value ORed with a constant.
- `expand_bytes_to_bits_and_scatter_2`: The routine processes sixteen bytes from a pointer, building five bit-accumulation words from carry results, then scatters those words to memory locations derived from an unassigned register and a global variable, and returns an input value unchanged.
- `expand_bytes_to_bits_and_scatter_3`: The routine processes sixteen bytes from a pointer, building four bit-accumulation words from carry results, then scatters those words to memory locations derived from an unassigned register and a global variable, and returns an input value unchanged.
- `compute_parity_and_store`: Iterates over a byte buffer derived from an unassigned pointer, builds three short accumulators from carry bits of the bytes and their multiples, then stores the three results through pointers derived from a global variable and an unassigned register and returns an input value.
- `call_indirect_via_global`: The routine calls a function pointer taken from an offset of a global variable and then returns a constant.
- `install_jump_table_entries`: The routine stores a constant word through a pointer derived from a stack offset and conditionally writes a word and a code pointer into three different stack locations based on three global variables.
- `indirect_jump_stub`: The routine calls another routine and then performs an indirect jump through an unrecovered jump table, with no other observable work.
- `store_word_via_pointer`: The routine stores a constant word through a pointer derived from an unassigned register and returns.
- `call_indirect_table_entry`: Calls a function pointer taken from a global variable and returns zero.
- `install_jump_table_entries_3`: Copies a global variable to a stack location, conditionally writes a word and a code pointer into three stack offsets depending on three global flags, then performs an indirect call via a global variable.
- `install_jump_table_entries_2`: The routine stores a constant word through a pointer derived from a global variable and conditionally writes a word and a code pointer into three different memory locations based on three global variables.
- `empty_routine_2`: The routine contains three instructions and performs no observable work; it is empty.
- `patch_code_table`: The routine walks a table of words in memory, checks a pattern of values, and when matched writes a constant word, patches a code pointer to another routine, and stores a computed value.
- `infinite_loop_wait`: Contains an infinite loop and touches the VHPOSR hardware register.

### logged disk load from Disk.1

- `initialize_game_memory`: The routine copies blocks of a global variable to itself, writes constant words to fields of three structures, initializes animation entries from a table, clears a memory region, and patches code words in another routine.
- `initialize_hardware_registers_and_globals`: The routine calls another routine, writes a constant to a hardware register, sets a global variable to one, stores two constants through a pointer derived from an uninitialized register, conditionally calls other routines based on a flag in memory, calls several other routines unconditionally, and stores the address of a global variable into a memory-mapped location.
- `initialize_hardware_and_globals`: The routine writes constants to a global variable and to memory locations derived from a register, conditionally calls another routine twice, calls several other routines in sequence, and stores the address of a global variable into a custom hardware register.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `wait_for_hardware_flag`: The routine spins until a bit in a word accessed through an uninitialized register is set, then spins until that bit is cleared, and finally returns a constant.
- `iterate_table_and_update_pointers`: The routine iterates over a word table until a zero terminator, builds a value from an input and the table entry, stores a base pointer, conditionally calls another routine with arguments or without, updates a pointer from a register-based address, advances the base pointer and finally stores a constant word to a register-based offset.
- `loop_update_pointers_and_flag`: The routine receives a parameter and stack arguments, repeatedly calls another routine, updates a pointer with a value from a register-based area, accumulates the parameter, and sets a word flag to one when a word from the stack is zero.
- `set_memory_flag`: The routine stores the constant one into a word located at an offset from an uninitialized register and then returns.
- `iterate_table_and_dispatch_2`: The routine initializes a counter and a pointer in a structure, then loops while a table entry is non-zero, copying a pointer into a global table, calling another routine, updating the pointer and incrementing the counter.
- `empty_routine_13`: The routine contains no instructions and returns immediately.
- `call_dispatch_once`: The routine calls another routine once and returns.
- `call_dispatch_twice`: The routine calls another routine twice in succession and returns.
- `copy_words_from_table`: The routine copies 411 words from a source table to a destination table using a loop that decrements a counter and advances both pointers.
- `initialize_input_routines`: The routine writes a word from a register into a global variable, calls another routine to obtain a value, then iterates over a 32-byte table.
- `call_other_routine_7`: The routine consists of a single call to another routine and then returns.
- `call_subroutine_a00`: The routine calls another routine at a fixed address and then returns.
- `initialize_input_state_2`: The routine programs custom chip registers and a global variable, then repeatedly clears and sets word fields of a structure pointed to by a base address, calling another routine between writes and waiting on status words until they change.
- `call_single_helper`: The routine calls another routine and then returns.
- `call_once_subroutine`: A wrapper routine that invokes another routine exactly once and then returns.
- `store_byte_to_offset`: The routine stores a byte from a data register to a byte at a fixed offset from a register.
- `fetch_and_clear_byte`: The routine reads a byte from a fixed offset of a register, clears that byte to zero, and returns the original byte.
- `initialize_flags_and_update_global`: The routine writes constant values to several offsets from a register-based pointer, calls another routine, conditionally calls another routine based on two flags, and updates a global variable with a byte derived from a flag.
- `update_input_flag`: The routine calls another routine, reads a word from a register-based offset to decide a byte value, combines it with a global variable using bitwise operations and stores the result to the global variable.
- `set_car_display_parameters`: The routine reads two pairs of 32-bit values from a structure pointed to by a register, computes their absolute differences, scales two 16-bit values from the same structure by a constant and clamps the results, and writes the scaled values and derived offsets into a second structure pointed to by another register.
- `scale_value_and_branch_3`: The routine computes a scaled word from a register value, clamps it to a minimum and conditionally calls another routine based on the scaled value or an alternative register comparison.
- `scale_value_and_branch_2_2`: The routine computes a scaled word from a register value, clamps it to a minimum of 1000, and conditionally calls another routine when the value is above a threshold.
- `scale_value_and_branch_2`: The routine computes a scaled word from a register value, clamps it to a minimum of 1000, and conditionally calls another routine when the value is below 3000.
- `update_opponent_state_and_timers`: The routine updates a block of words in a global structure by copying from a source area indexed by a field, then checks several short fields in the structure and conditionally calls another routine.
- `check_opponent_collision_and_state`: The routine inspects several short fields in a global structure, decrements two timer fields, and conditionally calls another routine and a routine that updates opponent state timers.
- `update_opponent_state_timers_2`: The routine decrements two short timer fields in a global structure and, when either reaches a specific value, calls another routine.
- `update_opponent_state_timers`: The routine decrements two short timer fields in a global structure and, when either reaches a specific value, calls another routine.
- `check_opponent_collision`: The routine reads flags and position/timing fields from a global structure and, when conditions on those fields are met, updates state fields and calls another routine.
- `update_opponent_state_3`: The routine checks several flags in a global structure and, when conditions on position and timing fields are met, updates state fields and triggers another routine.
- `check_and_update_opponent_state`: The routine checks several flags in a global structure and, when conditions on position and timing fields are met, updates state fields and invokes another routine.
- `update_opponent_state_2`: The routine reads and conditionally updates several fields of a global structure, invoking another routine when certain counters increase, and updates a table entry based on a flag and a table lookup.
- `update_opponent_state`: The routine reads and conditionally updates several fields of a global structure, invoking another routine when certain counters increase, and clears a flag.
- `update_counter_and_store_shifted`: The routine conditionally inverts a word, adds two words with clamping, stores the result to a register-based area, and writes a shifted version of the result to a different offset depending on a flag.
- `store_word_to_offset`: The routine stores a word from a data register to a word at a fixed offset from a register.
- `update_counter_and_store_shifted_alt`: The routine conditionally inverts a word, adds two words with different clamping limits, stores the result to a register-based area, and writes a shifted version of the result to an offset chosen by a flag.
- `store_word_to_table`: The routine stores a 16-bit value from a data register into a global variable via a pointer.
- `compare_and_update_table_entries`: The routine checks a flag, loads a word from a table indexed by an input word, compares four word pairs and updates a table entry, a flag word and an index word when a condition is met.
- `write_three_words_and_return`: The routine writes three words to locations derived from a pointer, an index and constants, then returns a value built from an index and a stack value.
- `process_table_entries`: The routine walks four entries of a table pointed to by an unreferenced register, updates word fields in each entry, builds a value from a counter and writes words to a custom hardware base and to an unreferenced register base.
- `iterate_and_copy_table_pointers_2`: The routine loops over a structure pointed to by registers, increments words, builds a value from a counter, writes words to a register-base and copies fields from a table pointed to by a global variable into a structure.
- `build_and_iterate_table_value`: The routine loads a pointer from a table indexed by a field of a structure, copies several words from the pointed structure into another structure, builds a 32-bit value by concatenating words from the structure and a counter, and loops over the structure fields incrementing a counter and updating the value.
- `iterate_and_copy_table_pointers_3`: The routine walks a table pointed to by a register, increments a word field in the structure, builds a 32-bit value from a counter and table entries, and writes words to a register-based area.
- `iterate_and_copy_table_pointers`: The routine loops over a structure pointed to by registers, increments words, builds a value from a counter, writes words to a register-based area, and copies words from a table pointer obtained via an index into fields of the structure.
- `handle_input_byte`: The routine obtains a byte from another routine, stores a constant to a global variable, and then branches based on the byte value.
- `store_byte_to_global`: The routine stores a byte from a data register to a byte at a fixed offset from an unreferenced register and then calls another routine.
- `conditional_call_and_return_stack`: The routine checks a word at a fixed offset from an unreferenced register and conditionally calls another routine that stores a byte to a global location and then calls another routine.
- `set_word_and_global`: The routine stores the constant 1 to a word at a fixed offset from a register and stores 1 to a global variable, then returns.
- `handle_byte_input_state`: The routine reads a byte from a register-based area, sets a word to maximum when the byte equals 127, otherwise updates a byte buffer and a counter, or initializes state bytes and a word field.
- `set_word_to_max`: The routine stores the maximum word value to a word field of a global structure via an address in a register.
- `initialize_state_bytes`: The routine clears a byte field of a global structure, copies an input byte to another byte field of the same structure, and sets a word field of the structure to one.
- `empty_routine_14`: The routine contains no instructions and returns immediately.
- `decrement_counter_and_recurse`: The routine checks a word at a fixed offset from an unreferenced register and, if zero, reads a byte from a global variable and conditionally clears it and calls another routine; otherwise it increments an integer at a fixed offset, decrements a word at a fixed offset and calls another routine.
- `increment_counters_if_flag`: The routine checks a bit in a byte at an offset from an unreferenced register and, if set, increments a word field of a structure via an unreferenced register, decrements another word field of the same structure, and calls another routine.
- `wait_for_bit_then_update_state`: The routine spins until a bit in a word at an offset from an unreferenced register is set, writes a constant word to a word field of that structure, then conditionally updates fields of a structure via an unreferenced register based on a byte from the structure and a flag, and returns a concatenated value from two inputs.
- `conditionally_update_fields`: The routine tests an input byte and, if it matches a specific value, writes a constant word to a field of a structure via one register and increments a word field of another structure via another register, then returns the incoming stack value.
- `configure_car_state_flags_2`: The routine is seven instructions long and is entered because its address is taken.
- `store_word_and_recurse_3`: The routine stores the incoming word from a data register into a word field of a structure referenced by an unreferenced register and then calls another routine that stores a constant to a word field, increments a word at a fixed offset and conditionally recurses.
- `call_other_routine_twice_2`: The routine is seven instructions long and is entered because its address is taken.
- `store_word_and_recurse_4`: The routine stores a word from a data register into a word field of a structure pointed to by an unreferenced register, then calls another routine and returns the value that was passed on the stack.
- `configure_car_state_flags`: The routine writes constant words to several structure fields via two registers, calls another routine twice, and sets two words in the same structure to specific constant values before calling another routine.
- `call_other_routine_twice`: The routine calls another routine twice in succession and then returns.
- `clear_word_then_call`: The routine clears a word field of a structure via an unreferenced register and then calls another routine.
- `set_word_to_one`: The routine stores the value one to a word field of a global structure via an address in a register.
- `increment_counter_and_recurse_2`: The routine stores a constant to a word field of a structure, increments a word at a fixed offset from an unreferenced register and conditionally calls another routine.
- `call_store_masked_word`: The routine calls a routine that masks the low byte of an input word and stores the result combined with a constant to a word field of a structure.
- `wait_for_flag_and_return_byte`: The routine loops until a specific bit in a word field of a structure is set, then returns the low byte of that word field.
- `store_masked_word_2`: The routine masks the low byte of an input word and stores the result combined with a constant to a word field of a structure.
- `update_car_state_and_timers_2`: The routine calls another routine and returns the concatenation of two input longs.
- `recursive_self_call_8_2`: The routine sets a short local variable to a constant and then repeatedly calls itself while decrementing the variable until it reaches -1, followed by a series of additional self calls.
- `call_another_routine_4`: The routine calls another routine and then returns.
- `call_other_routine_11`: The routine calls another routine and then returns.
- `conditional_table_dispatch`: The routine returns zero unless a table entry indexed by an input short is non-zero, in which case it calls another routine and returns its result.
- `conditional_call_and_return_2`: The routine loads a short from a table indexed by an unspecified value, tests it, and if non-zero calls another routine and returns its result; otherwise it returns zero.
- `empty_routine_15`: The routine contains no instructions and performs no operations.
- `recursive_self_call_8`: The routine calls itself once and then returns.
- `blend_tiles_with_mask_4`: The routine stores two pointers from a register into a global variable, computes a base pointer from a table entry and parameters, and then for each mask entry blends four 16-bit tile planes into a destination buffer using a mask table and a tile table referenced through a global variable.
- `blend_tiles_with_mask_3`: The routine walks a mask table and a tile table referenced through a global variable and blends four 16-bit tile planes into a destination buffer using a mask value per word.
- `blend_tiles_with_mask_2_2`: The routine processes a mask table and a tile table referenced through a global variable to blend four 16-bit tile planes into a destination buffer.
- `blend_tiles_with_mask`: The routine iterates over a nested loop using values from registers and a global variable to compute pointers, then for each entry blends two tables of words using a mask and its complement, copying or combining words at fixed offsets, and returns a value from the stack.
- `blend_tiles_with_mask_2`: The routine iterates over a nested loop using values from registers and a global variable to compute pointers, then for each entry blends two blocks of words using a mask word and writes the result to a destination block.
- `store_word_and_set_flag`: The routine calls a routine to obtain a value, stores the result into a global variable, calls the same routine again, writes a constant word to a field of a structure via an unreferenced register, and returns a concatenated value from two inputs.
- `conditional_decode_string`: The routine checks a word field of a structure via an unreferenced register and, if non-zero, calls a routine that stores a word from a register into a global variable and builds a 32-bit word.
- `clear_flags_and_wait`: The routine writes a constant word to a word field of a structure via an unreferenced register, clears a word field of a structure via another unreferenced register, and calls a routine that loops while a short value at an offset from an unreferenced register is negative or greater than zero.
- `initialize_input_state`: The routine writes constants to custom chip registers and a global variable, clears and sets word fields of a structure pointed to by a base address, calls another routine repeatedly, and loops waiting for a word field to become non-zero before continuing with further writes and calls.
- `reset_game_state_2_2_2`: The routine repeatedly clears and initializes a set of word fields of a structure pointed to by a register, increments a counter, and configures timing and mode words before waiting on two flag words.
- `reset_car_state`: The routine clears and initializes a set of word fields of a structure pointed to by a register, copies a few words from one part of the structure to another, and then calls several other routines.
- `update_car_state_and_dispatch`: The routine updates several word fields of a structure pointed to by a register, manipulates a counter in a register, clears and sets flags in the structure, and conditionally zeroes a memory table at a fixed address before invoking other routines and a helper.
- `initialize_car_state_flags_3`: The routine writes constant values to fields of a structure pointed to by a register and to fields of another structure pointed to by a different register, and calls another routine several times while polling short flag fields in the first structure.
- `configure_car_state_flags_3`: The routine writes constants to fields of a structure pointed to by a register and to fields of another structure pointed to by a different register, repeatedly calls another routine and polls short flag fields in the first structure.
- `initialize_game_state_2_2`: The routine writes constants to fields of a global structure, repeatedly calls another routine and polls short flag fields in that structure.
- `reset_game_state_2_2`: The routine clears several word fields of a structure pointed to by a register, writes constant words to fields of that structure, stores a value derived from an input word to a global variable and then repeatedly stores a 32-bit value split into two 16-bit halves.
- `initialize_game_state_2`: The routine writes constants to fields of a global structure, repeatedly calls another routine and polls short flag fields in that structure, clearing and setting several word fields in a loop.
- `reset_game_state_2`: The routine clears several word fields of a structure pointed to by a register, writes constant words to fields of that structure, stores a pointer to a global variable into another structure, and repeatedly calls other routines.
- `zero_table_and_process_bytes`: The routine zeroes a fixed number of entries in a table pointed to by a field of a structure, then iterates over a small fixed number of entries using two pointers derived from inputs, calling a routine with a byte from a global variable and other values, and references the text attempting to link up... press escape to quit.
- `tiny_ori`: Small routine of 1 instruction that merges 120 into d6.
- `initialize_state`: The routine writes constants to several global variables and clears fields in a structure pointed to by an unreferenced register.
- `wait_then_return_constant_2`: The routine calls another routine, stores a word derived from an input word to a fixed offset from an unreferenced register and then loops calling another routine while a short value at a fixed offset is non-zero, returning a constant.
- `wait_then_return_constant`: The routine calls a routine that loops while a short value at an offset from an unreferenced register is negative or greater than zero, and then returns a constant value.
- `wait_for_flag`: The routine loops while a short value at an offset from an unreferenced register is negative or greater than zero, decrementing a register until it reaches minus one, then returns the short value at that offset.
- `clear_track_state_and_init`: The routine clears a small range of words in a global structure, sets a flag word indexed by a short field, and then writes several constant words and bytes into the structure.
- `initialize_track_data_1_2`: The routine writes constants to fields of a global structure, computes a table index from a short field in the structure and copies several 16-bit entries from three tables into three destination buffers.
- `initialize_track_data_1_3`: The routine writes constants to fields of a global structure, computes a table index from a short field in the structure and copies several 16-bit entries from three different tables into three destination buffers.
- `initialize_track_data_1_4`: The routine writes constants to fields of a global structure, computes a table index from a short field in the structure and copies several 16-bit words from three tables into three destination buffers.
- `initialize_track_data_1_5`: The routine writes constants to fields of a global structure, computes a table index from a short field in the structure and copies several 16-bit entries from three tables into three destination buffers.
- `initialize_track_data_1_6`: The routine writes constants to fields of a global structure, computes a table index from a short field in the structure and copies several 16-bit entries from three tables into three destination buffers.
- `initialize_track_data_1_7`: The routine writes constants to fields of a global structure, computes a table index from a short field in the structure and copies several 16-bit entries from three different tables into three destination buffers in the structure.
- `initialize_track_data_1`: The routine writes constants to fields of a global structure, computes a table index from a short field in the structure and copies several words from tables in the structure to other fields of the structure.
- `initialize_track_data_2`: The routine writes constants to fields of a global structure, computes a table index from a short field in the structure and copies several blocks of word data from tables in the executable to fields of the structure, with a conditional copy from another routine’s data when a flag is set.
- `initialize_track_data_3`: The routine writes constants to fields of a global structure, computes a table index from a short field in the structure and copies several blocks of word and byte data from tables in the structure to other fields of the structure.
- `initialize_track_data`: The routine writes constants to fields of a global structure, computes a table index from a short field in the structure and copies several blocks of words and bytes from tables indexed by that field to fixed offsets in the same structure, using a different index mapping for one block.
- `copy_track_data_blocks_2`: The routine copies several blocks of words from tables indexed by a short field in a global structure to fixed offsets in the same structure, then copies a fixed block of words from a source area into a destination area in a register argument, and updates pointer fields in the structure with addresses derived from a global variable and the result of another routine.
- `copy_track_data_blocks`: The routine copies several blocks of words from tables indexed by a short field in a global structure to fixed offsets in the same structure, using forward and reverse loops, and conditionally copies additional blocks from a global table when a short flag is non-zero.
- `return_minus_one`: The routine returns the constant value minus one as a 32-bit integer.
- `check_and_copy_block`: The routine checks a byte in a buffer and, if it differs from a constant, copies a block from the buffer to a structure, then compares two bytes.
- `compare_and_swap_block`: The routine checks a byte in a buffer and, if it differs from a constant, copies a block from the buffer to a structure, then compares two blocks byte by byte and swaps bytes within a block for each iteration.
- `derive_byte_value`: The routine repeatedly reads a short from an unreferenced register, mixes it with a running value using shifts and masks, and loops until a condition on the mixed value is met, then stores the low seven bits of the mixed value into a byte in a structure.
- `fill_byte_array`: The routine reads a byte from a structure and writes that byte repeatedly into a consecutive block of memory within the same structure.
- `copy_bytes_to_global`: The routine stores constants and values derived from global variables to fixed offsets, copies eight bytes from two source areas in a global variable to two destination areas in a global variable and then calls another routine.
- `copy_byte_blocks`: The routine conditionally copies a byte from one field to another in a structure, then copies two small blocks of bytes from one area of the structure to two other areas.
- `copy_bytes_to_global_2`: The routine stores constants and values derived from global variables to fixed offsets, copies nine bytes from two source areas in a global variable to two destination areas in a global variable and then calls another routine.
- `update_counters_and_copy`: The routine decrements two bytes in a structure, updates a short with a maximum, then copies two small blocks of bytes from one area of the structure to two other areas.
- `store_byte_to_global_2`: The routine stores constants and a byte from a global variable to fixed offsets in a global variable and then calls another routine.
- `copy_single_byte`: The routine copies a single byte from one field to another within the same structure.
- `copy_buffers_and_format_digits`: The routine copies four 10-byte blocks from offsets of a global structure to other offsets of the same structure.
- `format_number_digits`: The routine splits an input integer into decimal digits and stores the digits into a byte array within a structure, writing the lower and upper parts separately.
- `sort_player_names`: The routine initializes four name slots with default strings, builds a small flag array from two short fields in a global structure, clears a name slot when the flag is zero, then performs a bubble sort over the four slots by comparing selected bytes and swapping the 24-byte entries.
- `set_pointer_and_wait_flag`: The routine stores a pointer into a field of a global structure and stores a byte plus two into a short field.
- `initialize_state_and_return`: The routine writes constants to a global variable and several RAM locations, stores a constant and a pointer through an address derived from an unused register, and returns a constant.
- `process_input_byte_and_update_flags_2`: Derives a byte from a byte at an offset from a pointer and stores it through a pointer derived from a register.
- `process_input_byte_and_update_flags`: The routine derives a byte from a byte at an offset from A0, stores it through a pointer derived from A3, sets a flag byte at an offset from A0 and a byte at another offset from A0, and conditionally updates a flag byte in a global variable based on the derived byte value, then stores the constant eight to a word at an offset from A6.
- `clear_bit_in_buffer`: The routine clears bit 6 of a byte located at an offset from a pointer in A0 and stores the constant 8 into a word at an offset from A6.
- `set_flag_bit_one`: The routine ORs an input byte with one and stores the result to a byte in a global structure, and stores the constant eight to a word in another structure.
- `set_flag_bit_two`: The routine ORs an input byte with two and stores the result to a byte in a global structure, and stores the constant eight to a word in another structure.
- `set_flag_bit_eight`: The routine ORs an input byte with eight and stores the result to a byte in a global structure, and stores the constant eight to a word in another structure.
- `set_flag_bit_four`: The routine ORs an input byte with four and stores the result to a byte in a global structure, and stores the constant eight to a word in another structure.
- `set_flag_bit_sixteen`: The routine ORs an input byte with sixteen and stores the result to a byte in a global structure, and stores the constant eight to a word in another structure.
- `clear_flag_bit_one`: The routine ANDs an input byte with zero-fe and stores the result to a byte in a global structure, and stores the constant eight to a word in another structure.
- `clear_flag_bit_two`: The routine ANDs an input byte with zero-fd and stores the result to a byte in a global structure, and stores the constant eight to a word in another structure.
- `mask_byte_and_store_word`: The routine masks the low byte of D1 with 247 and stores the result through a pointer derived from A3, then stores the word 8 through a pointer derived from A6, and returns the value taken from the stack.
- `mask_byte_and_store_word_2`: The routine masks the low byte of D1 with 251 and stores the result through a pointer derived from A3, then stores the word 8 through a pointer derived from A6, and returns the value taken from the stack.
- `mask_byte_and_store_word_3`: The routine masks the low byte of D1 with 239 and stores the result through a pointer derived from A3, then stores the word 8 through a pointer derived from A6, and returns the value taken from the stack.
- `clear_flag_and_copy_byte`: The routine reads a byte from a global variable, writes 255 to that byte location, and copies the original byte into a nearby 16-bit field of the same structure.
- `swap_two_longwords`: The routine swaps the contents of two longword locations that are four bytes apart within the same A3-based structure.
- `mask_word_and_store_2`: The routine masks a word from an offset in A6 with the value 3 and stores the result at another offset in A6.
- `process_input_event`: The routine checks a bit in the interrupt request register and, if set, stores the bit to a global variable and calls another routine twice.
- `update_counter_and_dispatch_2`: The routine reads a short value from a global variable, adds it to another short field in the same structure, clamps the result to zero or eight, clears related flags, and then calls another routine.
- `update_counter_and_dispatch`: The routine updates a short counter in a global variable by adding an input value and clamps it to a maximum, clears a related flag, then calls another routine and conditionally calls it again based on the value of a different short in the same global area.
- `call_three_times_and_return_2`: The routine calls another routine three times in sequence and then returns the value that was on the stack at entry.
- `call_three_times_and_return_3`: The routine calls another routine three times in sequence and then returns the value that was on the stack at entry.
- `call_three_times_and_return`: The routine calls another routine three times in sequence and then returns the value that was on the stack at entry.
- `update_two_digit_display_3`: The routine updates a two-digit value stored in a global variable and a second byte, clears a 10 by 4 table of 32-bit entries, and then calls another routine 25 times with the two digit bytes and other arguments.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 14 into d0, then merges 39 into (a4)+, then merges 39 into -(a7).
- `update_digit_display_and_clear_buffer`: The routine checks a short counter against a limit in a global variable and, if the condition holds and a flag bit is clear, updates a short counter in a global variable with wrap-around logic, writes two ASCII digit characters to global variables, clears a block of words in a buffer derived from a global pointer, and calls another routine to render digits.
- `update_counter_and_render_digits`: The routine reads and conditionally updates a short counter in a global variable, writes two ASCII digit characters to global variables, clears a block of memory pointed to by a global variable, and calls another routine repeatedly with the digit bytes and other registers to process the data.
- `copy_track_palette_data`: The routine selects an index from a global variable and a lookup table, then copies fixed blocks of 32-bit words from four source tables into a destination buffer pointed to by a global pointer, using two different layouts depending on a flag byte in a global variable.
- `copy_palette_blocks_2`: The routine copies a fixed number of 32-bit words from four source tables, each indexed by the low word of the input register, into a destination buffer whose base is taken from a global variable.
- `copy_palette_blocks`: The routine copies a fixed number of 32-bit words from four source tables, each indexed by the low word of the input register, into a destination table pointed to by a global variable.
- `check_flag_and_call`: The routine tests a word field of a structure for a constant value and, if it matches, calls another routine twice.
- `adjust_gear_and_handle_shift`: The routine adjusts a gear-related word in a global structure using a flag byte, applies wrap-around for a small range, writes the result back, and then calls another routine repeatedly in several cases.
- `adjust_index_and_recurse`: The routine reads a short field of a structure and, if it is negative, computes a new index from another short field using flag bits in a byte field, writes the new index back, and then recurses.
- `copy_word_and_recurse_2`: Calls another routine twice and copies a word from an offset of a structure to another offset of the same structure before returning.
- `copy_word_and_recurse`: The routine calls another routine twice, copies a word from an offset of a structure to another offset of the same structure, and returns.
- `reset_state_and_recurse`: The routine reads a short from an offset of a structure pointed to by a register, checks if the value is zero, four or eight and calls another routine in those cases, then writes a constant word to the same offset, calls another routine and returns.
- `set_word_and_recurse_2`: Calls another routine, stores a constant word at an offset of a structure, calls another routine again, and returns.
- `set_word_and_recurse`: The routine calls another routine, stores a constant word at an offset of a structure, calls another routine again, and returns.
- `conditional_table_dispatch_2`: Checks a short field of a structure and either recurses four times or dispatches via a table indexed by that short.
- `call_other_routine_6`: The routine calls another routine and then returns.
- `call_four_routines_3`: The routine calls four other routines in sequence and returns.
- `self_recursive_loop`: The routine calls itself four times in a row and then returns.
- `recursive_self_call_6`: The routine calls itself four times in a row and then returns.
- `call_empty_routine_chain`: Calls another routine three times in sequence and returns.
- `store_byte_and_call_3`: Calls another routine three times in sequence and returns.
- `set_table_byte_via_index_2`: Calls another routine three times in sequence and returns.
- `update_display_buffer`: The routine initializes a display buffer area in a global variable when a flag is clear, then reads a byte from a table indexed by a short from the global variable.
- `clear_two_bytes_and_decrement_counter`: If the input word is non-zero, the routine stores the byte 32 at two consecutive byte locations derived from a pointer and the input word, then decrements a short counter stored at an offset from a global variable.
- `clear_name_buffer_and_type_char`: The routine clears a name buffer and a counter when a flag is zero and a length check passes, then writes a character from a table into the buffer at an index tracked by a counter, incrementing or decrementing the counter depending on the character value, and also updates a second buffer location in parallel.
- `clear_two_bytes_and_decrement_counter_2`: If the input word is non-zero, the routine stores the byte 32 at two consecutive byte locations derived from a pointer and the input word, then decrements a different short counter stored at an offset from a global variable.
- `reset_and_fill_name_buffer`: The routine resets a name buffer and a related counter when a flag is clear and a length limit is not exceeded, writing spaces into four words of the buffer.
- `clear_two_bytes_and_decrement_counter_3`: If the input word is non-zero, the routine stores the byte 32 at two consecutive byte locations derived from a pointer and the input word, then decrements another short counter stored at an offset from a global variable.
- `store_byte_and_call_2`: The routine stores a constant byte into a byte field of a structure at an offset computed from a short field, then calls another routine.
- `store_masked_byte_3`: The routine stores a byte derived from a masked short in a global variable at an offset computed from a pointer and a short counter, then calls another routine.
- `set_byte_via_indexed_offset`: The routine stores a constant byte into a location computed from a global variable and an indexed offset taken from another global variable, then calls another routine.
- `set_flag_from_input_byte`: The routine reads a byte from a global variable and uses it as an index into a table, then stores a byte derived from a global variable into the indexed location.
- `set_table_byte_via_index`: The routine writes a constant byte to a byte in a table indexed by a byte from another global variable, then calls another routine.
- `set_flag_from_input_byte_2`: The routine reads a bit from a global variable and stores a derived byte into a byte in a table indexed by a byte from another global variable.
- `copy_table_entries_from_indexed_source`: The routine uses a global variable and a pointer to copy bytes from a table indexed by an input byte into a destination area, iterating over a fixed number of blocks and entries.
- `iterate_mask_application`: The routine loops over a fixed number of entries using pointers into a global variable and a global variable area, calling another routine with values loaded from those areas and incrementing counters.
- `iterate_mask_application_2`: The routine computes a pointer into a global variable using an input short, then loops a fixed number of times loading bytes from that area and calling another routine with those values and incrementing counters.
- `copy_and_iterate_masks_2`: The routine checks a short in a global variable and, if it differs from a constant, copies a fixed number of bytes from a global variable into a buffer and then iterates over a table, calling another routine with values derived from the buffers.
- `copy_and_iterate_masks`: The routine checks a short in a global variable and if it differs from a constant copies a fixed number of bytes from a global variable into a buffer in the global variable, then loops over a fixed number of entries using pointers into the global variable and a buffer area, calling another routine with values loaded from those areas.
- `mask_and_or_bytes_from_table`: The routine reads bytes from a table indexed by an input word and combines them with bytes in a destination buffer using bitwise AND and OR operations, iterating over a fixed number of entries.
- `apply_hunk_type_masks`: The routine iterates over a small fixed number of entries using two pointers derived from the inputs.
- `check_flags_and_call`: The routine compares two short values from a global variable and tests a flag byte in the same variable.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `decrement_counter_from_global`: The routine decrements a short value stored at an offset from a global variable by using the value from another offset in the same global variable.
- `copy_words_from_indexed_table`: The routine copies a fixed number of words from a table indexed by a short value in a global variable into a global variable area, advancing both source and destination pointers each iteration.
- `fill_word_pairs_loop`: The routine writes a constant derived from an input word to a global variable and then repeatedly stores a 32-bit value split into two 16-bit halves into consecutive memory locations, incrementing the value and the destination.
- `store_incrementing_word_pairs`: The routine stores a 32-bit value split into two 16-bit halves into consecutive memory locations, incrementing the value and the destination pointer each iteration until a counter expires.
- `expand_palette_entries`: The routine reads words from a table in a global variable and writes transformed words to a fixed memory area, using a multiplier that increases each outer iteration.
- `configure_dispatch_and_branch`: The routine stores a value into a global variable, calls another routine, and if a short field of a structure is zero it writes a constant to a global variable, calls another routine, and conditionally calls another routine again.
- `configure_dispatch_with_self`: The routine stores the address of itself into a global variable, calls another routine, and if a short field of a structure is zero it writes a constant to a global variable, calls another routine, and conditionally calls another routine again.
- `check_input_state_and_reset`: The routine copies a value from a global variable into another global variable, calls another routine, and then checks a short value at an offset from an uninitialized pointer.
- `decode_input_buffer_2`: The routine stores a word from a register into a global variable, calls another routine to obtain a value, then iterates over a 32-byte table, combining each byte with a running value and writing the result back to memory.
- `initialize_game_state`: Calls several helper routines to derive a byte value, fill a byte array, apply masks, set a flag and update a linear congruential state.
- `set_flag_and_call`: The routine ORs 60 into a byte at offset 190 from a pointer in A0 and then calls another routine.
- `call_and_return`: The routine calls another routine and returns.
- `dispatch_input_store`: Calls another routine once and returns.
- `call_other_routine_10`: The routine unconditionally calls another routine and then returns.
- `conditional_input_dispatch`: Calls another routine and then conditionally calls another routine if two short fields of a structure are zero.
- `check_two_flags_and_call_2`: The routine calls another routine unconditionally, then checks two short fields in a structure pointed to by a register and calls another routine if both fields are zero.
- `conditional_memory_copy`: Calls another routine and then conditionally copies a large block of memory from a fixed location into a structure pointed to by a field, after checking two short fields of the structure.
- `check_flag_and_call_2`: The routine calls another routine and then checks a short flag in a global variable.
- `check_flag_and_call_twice`: The routine calls another routine and then checks a short flag in a global variable.
- `check_flag_and_call_once`: The routine calls another routine and then checks a short flag in a global variable.
- `copy_block_via_pointer`: The routine calls another routine, then copies a large block of data from a source pointer obtained from a global variable to a fixed destination using a loop.
- `call_two_routines_3`: The routine calls two other routines in sequence and returns.
- `nested_write_loop`: The routine uses a global variable to compute a base address and then runs nested loops that write constant values and results from another routine into multiple memory locations derived from several pointers.
- `copy_block_table`: The routine copies words from a fixed table to a destination buffer derived from a global variable, using nested loops that clear and fill multiple offsets.
- `mask_and_merge_words`: The routine iterates over a fixed source table and a fixed destination table, masking destination words with the inverse of a combined source mask and then ORing source words into the destination.
- `rotate_bits_right`: The routine takes a 16-bit value from D0, extracts its bits and builds a result by shifting and inserting the bit test, effectively rotating the value right.
- `branch_on_flag_and_call`: The routine calls another routine and then tests a short flag in a global variable.
- `call_shuffle_blocks`: The routine consists of a single call to another routine and then returns.
- `shuffle_word_blocks`: The routine copies a block of words from a pointer obtained via a global variable to two other locations and then performs repeated XOR-and-AND mixing across four word streams in multiple passes.
- `reverse_copy_and_mix`: The routine reverses copies a block of words between two buffers derived from a global variable, then copies forward, and finally mixes four word streams with AND, OR and NOT operations, returning a constant.
- `copy_and_mask_tables`: The routine copies a block of words from one region of a structure to another region, first backwards then forwards, then iterates over a set of words applying a bitwise OR and AND-NOT mask between two tables and two destination buffers, using a third table as a source mask.
- `initialize_graphics_tables`: The routine writes a series of computed values into a set of tables located via a base pointer, initializing palette-related and geometry data and clearing several work areas.
- `mask_buffers_with_prng_2`: Clears several buffers in a global structure, copies a word from one offset to another, loops calling another routine while incrementing a counter, then copies a block of words from one area to another using a computed length.
- `mask_buffers_with_prng`: The routine stores an input pointer into a structure field indexed by a short, computes a loop count from structure shorts, then iterates updating two byte buffers bit by bit using a pseudo-random value derived from a table and input shorts.
- `mask_bytes_with_prng`: The routine uses an input short as an index into a table to obtain a mask byte and then updates two byte buffers bit by bit using a pseudo-random sequence derived from an input value, stopping after a fixed count.
- `clear_and_set_bit_masks`: The routine clears a bit in a byte pointed to by one argument and then uses a pseudo-random sequence to read bytes from a third pointer, using the low nibble to conditionally clear or set a bit in a byte pointed to by a second argument, and repeats until a counter reaches a limit.
- `process_bit_masks`: The routine repeatedly clears a bit in a byte pointed to by one argument and then uses a pseudo-random sequence to read bytes from a third pointer, testing bits and setting or clearing bits in two output byte arrays.
- `copy_indexed_word`: Reads a short from a global variable offset, optionally adds another short and one, then stores a word from a table indexed by that short into a global variable offset.
- `call_other_routine_8`: The routine calls another routine and then returns.
- `copy_block_with_overlap`: Copies a block of words from a source area derived from a global variable offset to a destination area using a loop that moves data with overlapping offsets.
- `process_input_loop_3`: Clears a buffer in a global structure, copies a word from one offset to another, then loops reading a byte from a table indexed by a counter and, under conditions, copies a word from a pointer into a destination area and updates fields in the global structure.
- `process_input_loop_2`: The routine loops processing an input word, updating fields in a global structure, copying words from a table into a destination area, and incrementing a counter until a stop condition is met.
- `process_input_loop`: The routine loops processing an input word, updating fields in a global structure, copying words from a table into a destination area, and invoking other routines under specific input conditions while checking two short flags.
- `copy_table_entries_2`: The routine iterates over a table in a global variable, stopping when a short entry is zero, and copies corresponding 32-bit values into a region addressed via a global variable.
- `empty_routine_16`: The routine contains no instructions and returns immediately.
- `scan_table_and_store_count`: The routine calls another routine and, when two short flags in a global variable are clear, scans a table for a specific short value and stores a computed count into a field of the global variable.
- `call_other_routine_9`: The routine contains no instructions that access memory or registers and performs no data operations.
- `swap_and_sort_track_nodes`: The routine swaps two word fields of a global structure and conditionally swaps them again, computes four index values from those fields and a mode flag, writes the count to a global word, then calls another routine ten times to obtain two values and conditionally swaps two dword entries in a table pointed to by the first returned pointer using the second returned value and a masked value.
- `initialize_track_node_data`: The routine clears several fields of a global structure and then walks a per-frame counter, copying values from two decrementing source pointers into fields of the structure and invoking another routine to fill node data.
- `fill_track_node_data_2`: The routine walks a per-frame counter and, for each step, copies values from two decrementing source pointers into fields of a global struct.
- `fill_track_node_data_3`: The routine walks a per-frame counter and, for each step, copies values from two decrementing source pointers into fields of a global struct.
- `fill_track_node_data`: The routine walks a per-frame counter and, for each step, copies values from two decrementing source pointers into fields of a global structure and into a node table.
- `empty_stub_placeholder`: The routine is empty and returns immediately without performing any operations.
- `zero_memory_range`: Writes zero to a fixed range of words starting at address 6144, iterating a fixed number of times.
- `copy_struct_array`: Copies a series of structures from a source area based on a global variable and a global variable offset to a destination area derived from a global variable offset, clearing a field in each.
- `update_checksum_flags_2`: Calls another routine twice and returns.
- `loop_store_results`: The routine clears a word field in a global variable and repeatedly calls another routine four times per iteration, storing each returned word into successive offsets of a pointer.
- `validate_and_store_flag`: The routine checks a byte derived from the input word against a set of allowed values and a global variable.
- `set_flag_word`: Stores the constant one into a word at a global variable offset.
- `update_checksum_flags`: Iterates over a fixed number of entries, accumulates bytes into two running sums, tests a byte for specific values to set a flag, masks and sets a word field, and advances the pointer.
- `compress_column_bounds_2`: The routine processes a table of 1024 entries, each with a 16-byte record, and for each entry it scans a column of bytes, skipping marker bytes, to find the minimum and maximum byte values.
- `compress_column_bounds`: The routine scans a column of bytes pointed to by a register, skipping certain marker bytes, and tracks the minimum and maximum byte values seen in the column.
- `shift_byte_column_2`: The routine processes a column of bytes pointed to by a register, copying a byte from an input register into a byte field and updating two running byte values based on comparisons with neighboring bytes.
- `shift_byte_column`: The routine iterates over a byte column in memory, moving each byte down by one position and updating a running byte value based on comparisons with the previous byte.
- `scatter_load_words`: Loads a series of words from a source area indexed by a short from a global variable offset and writes them to multiple global variable locations.
- `reset_state_fields`: The routine writes a series of word and long values to fixed offsets of a structure pointed to by a register, setting them to all-ones, then calls another routine twice.
- `reset_track_state`: The routine clears a large set of short and long fields in a structure pointed to by a register, writes a byte-derived value into a word field and conditionally writes it elsewhere, then sets a few control words and calls another routine four times with a control word cycled through 2, 1 and 0, and finally stores two pointers into the structure.
- `return_concatenated_register_pair`: The routine calls another routine and then returns the concatenation of the two 32-bit values that were in its input registers.
- `read_joystick_state_2`: The routine reads joystick words from a structure, derives button bits using shifts and masks, combines them with a global variable, stores the result into global variable bytes, and updates edge detection bytes for two joystick sets.
- `read_joystick_state`: Reads a word from a structure pointed to by a register, derives button bits from CIAA_PRA and a global variable, stores the result into a global variable offset, and updates edge detection bytes in global variable offsets.
- `update_car_state_and_timers`: The routine updates several short counters and eight 32-bit position fields by subtracting a constant, clears custom hardware interrupt request bits for two indices when a flag is set, copies two word fields between structures, and manipulates a set of state flags and timers based on multiple short fields in a global variable, calling other routines throughout the process.
- `conditional_state_updates_and_flag_xor`: The routine calls process_state_machine_step and store_three_words_from_registers, conditionally calls another routine based on a word field of a global variable, decrements a word field in a global variable, and updates several word fields in a global variable with conditional assignments and a xor operation.
- `conditional_flag_set_and_state_updates`: The routine calls process_state_machine_step, accumulate_flags and process_table_entries_loop, conditionally sets a bit in a word field of a global variable, decrements a word field in a global variable, and updates several word fields in a global variable with conditional assignments and a xor operation.
- `process_state_machine_step`: The routine calls another routine, conditionally calls four other routines based on a word field of a global variable, decrements a word field of the global variable when it is positive, and updates several word fields of the global variable based on comparisons and flag tests.
- `update_state_flags_and_values`: The routine updates several word fields in a structure pointed to by a register, conditionally sets flags based on two word fields, toggles a word flag, XORs two word fields and stores the result, then calls another routine.
- `empty_stub_routine_3`: The routine contains no instructions that access memory or registers and simply calls another routine three times before returning.
- `call_increment_table_bytes_twice`: The routine calls another routine twice in succession and then returns.
- `increment_table_bytes`: The routine computes an index from an input value and a global variable, then iterates over eight entries stepping backwards by sixteen bytes.
- `accumulate_flags`: The routine conditionally clears two word fields in a global variable and, if corresponding counters are non-zero, decrements them and adds a table value indexed by a short from a global variable to two accumulator fields.
- `compare_and_update_state_flags`: The routine compares two integer fields of a structure pointed to by a register and stores a word constant into a field, conditionally sets a flag and calls another routine, then repeats the same logic for a second pair of fields.
- `update_flags_and_state`: The routine stores the constant 2 into a word field of a structure, conditionally sets a flag and calls another routine, then compares two long values from a global variable and updates a word field and another flag with a conditional call.
- `set_flag_and_call_2`: The routine stores the constant 2 into a word field of a structure and, if a flag in a global variable is non-zero and another word field is zero, calls another routine and sets the word field to 1.
- `copy_state_on_condition`: The routine checks a short field in a global variable and two other condition fields.
- `set_flag_based_on_conditions`: The routine tests several word fields of a global variable and, depending on the results, stores one, two or zero into a word field of the global variable.
- `set_flag_two`: The routine stores the constant value two into a 16-bit field of a global variable and then returns.
- `clear_flag`: The routine stores zero into a 16-bit field of a global variable and then returns.
- `combine_and_store_values`: The routine builds two 32-bit values by combining fields from a global variable with masked parts of other fields, stores them, and then computes and stores several 16-bit derived values from those fields.
- `update_position_fields`: The routine extracts nibbles from a byte in a global variable, adjusts two 32-bit position fields using short offsets with sign handling, and then recomputes and stores derived 32-bit values based on those positions and other fields.
- `copy_blocks_to_buffer`: The routine copies multiple 32-bit and 16-bit fields from one set of offsets in a global variable to another set, then copies a series of 32-bit words from a source area to a destination area using a loop that steps through both areas.
- `increment_counter_with_wrap`: The routine increments a 16-bit counter in a global variable unless it equals a sentinel value, and resets the counter to a constant when it reaches a specific value.
- `chain_subroutine_calls_conditional`: The routine calls another routine six times unconditionally, checks a word at a fixed offset of a global variable and calls another routine if the word is non-zero, then calls another routine once more.
- `swap_two_words_and_call_helpers`: The routine copies a word between two fixed offsets of a structure pointed to by a register, then swaps the contents of those two offsets using a temporary word.
- `guarded_chain_subroutine_calls`: The routine checks a word at a fixed offset of a global variable and, if non-zero, calls another routine five times, checks a second word at another fixed offset of the same global variable and calls another routine if non-zero, then calls another routine once more.
- `compute_direction_byte`: The routine computes a signed difference between an averaged byte pair from a global variable and a shifted word from a structure, stores a direction byte with a constant bit set into the structure, and may call a routine that writes a constant byte.
- `write_constant_byte`: The routine stores a constant byte value into an offset of a structure pointed to by a register and then returns.
- `conditionally_swap_words_and_call_helpers`: The routine checks a short field of a structure pointed to by a register and, if it is non-zero, copies a word between two fixed offsets of the structure, swaps the contents of those offsets using a temporary word, and calls another routine multiple times, including conditionally when another short field is non-zero.
- `advance_index_and_store`: The routine increments a short counter in a global variable, selects a pointer based on a short value, reads a short from a table indexed by another short and stores it into a global variable field.
- `select_and_advance_index_2`: The routine reads a short from a table indexed by a short in a global variable, stores it into a global variable field, and conditionally updates several global variable fields and a pointer in another structure.
- `select_and_advance_index`: The routine reads a short from a table indexed by another short in a global variable, stores it, and if a comparison with another field holds it updates a pointer in another structure, advances and masks the index, and cycles a separate short counter.
- `process_car_record_5_2_2_2`: The routine updates fields in a car record pointed to by a register, increments counters, checks flags and conditionally calls another routine.
- `process_car_record_5_2_2`: The routine processes a car record pointed to by a register, computes a value from a byte field and a short field and stores it, then conditionally calls another routine.
- `process_car_record_5_2`: The routine processes a car record pointed to by a register, computes a value from a byte field and a short field and stores it, then conditionally updates several short fields in the record and in a global table, and may call another routine.
- `scale_and_sign_adjust`: The routine stores a value from a register into a short field, sets a word flag, swaps halves of a long value, calls another routine, scales a word field by a constant and adjusts a short field based on the sign of a long field.
- `process_car_record_5`: The routine processes a car record pointed to by a register, computes a value from a byte field and a short field and stores it, then conditionally calls another routine.
- `process_car_record_4`: The routine reads a byte from a table indexed by a computed offset from a pointer and a word field, then uses the byte value to index two tables and compute a value that is stored in a field of a structure pointed to by a register.
- `process_car_record_2`: The routine processes a car record pointed to by a register, computes a value from a byte field and a short field and stores it, then conditionally updates several short fields in the record based on an input code and a few flags, invoking another routine repeatedly.
- `process_car_record`: The routine processes a car record pointed to by a register, computes a value from a byte field and a short field and stores it, then conditionally scales a short field and updates several short and word fields of the record based on an input code.
- `set_byte_and_scale_field`: The routine stores the constant 1 into a byte at an offset from a pointer using an index, calls another routine, scales a word field by a constant and stores the result back, then sets a word field to 1.
- `increment_field_and_set_flag`: The routine adds 500 to a short field of a structure, calls another routine, then stores 1 into a word field of the same structure.
- `adjust_fields_and_call`: The routine adds 100 to a short field and conditionally adds 3 to another short field, clears a word field, conditionally writes a long constant to a field, then calls another routine and sets a word field to 1.
- `scale_and_sign_adjust_3`: Loads a word from a structure, clears a word field and conditionally clears another flag and updates a counter, then either returns the loaded word or calls another routine and sets two word fields before returning.
- `process_car_record_2_2`: The routine computes a signed offset from a byte in a buffer and two words from two structures, then sets or clears a flag word in a car record and conditionally returns.
- `clear_flag_and_adjust_counter_2`: Sets two word fields of a structure, then conditionally clears a flag, updates a counter by adding a routine address, and optionally calls another routine.
- `process_car_record_3`: The routine clears a field in a car record and then branches on a short value in D1.
- `scale_field_by_table`: The routine calls another routine, scales a word field by a value loaded from a table indexed via a global variable and a field, stores the result back, then sets a word field to 1.
- `compare_and_update_table_entries_2`: Checks a word field for a range, clears flags and updates a counter conditionally, then scales a word field by a register value, computes a short offset and conditionally negates it, stores results into the structure and sets a flag.
- `update_linked_checksum_words_2`: Loads a word from a structure and if zero calls another routine, computes two signed short values with conditional negation and stores them into the structure, sets a flag and returns; otherwise clears a flag, updates a counter and optionally calls another routine before returning the loaded word.
- `update_linked_checksum_words`: The routine calls another routine, conditionally negates input words based on a global variable, stores the words into a structure if a field is zero, and sets a word field to one.
- `scale_and_sign_adjust_2`: The routine calls another routine, scales a word field by a register value, computes a short offset from the word field and negates it when a long field is positive, then sets a word flag.
- `clear_flag_and_adjust_counter`: The routine clears a word field of a structure pointed to by a register and, if a flag is set, clears the flag, adds the contents of a word field to a pointer and optionally calls another routine.
- `search_table_and_update_entry`: The routine iterates over a table pointed to by a register, computes a difference and a scaled value, clamps the result, conditionally sets a flag and calls another routine, then updates a short field in the table entry.
- `conditional_scale_and_offset`: The routine computes a difference between a shifted register value and a short field, clamps it, conditionally sets a flag, calls another routine, scales a word field and computes a signed short offset, or updates a pointer field and clears a flag.
- `normalize_and_clear_field_2`: The routine computes a difference between a shifted register value and a short field, clamps it to a range and conditionally negates a short field, then writes a computed short to another field and clears a word field.
- `normalize_and_clear_field`: The routine makes a short field positive if negative, copies a shifted word from a structure into another field, and clears a word field of a structure.
- `conditional_scale_and_set`: The routine checks a word flag and, if clear, sets it, calls another routine, scales a word field by a constant and computes a short offset that is negated when a register value is positive.
- `clear_struct_field`: Stores zero to a word field of a structure pointed to by a register.
- `search_table_and_update`: The routine loops over a table pointed to by a register, compares values with a field, and when a condition is met calls another routine, scales a word field, computes a signed short offset and returns, otherwise clears a word field.
- `search_and_scale`: The routine loops over a table pointed to by a register, comparing values and, when a condition is met, calls another routine, scales a word field by a constant and sets a short field with a sign-adjusted offset, then returns.
- `update_structure_fields`: The routine computes a signed difference from a register and a field in a structure pointed to by a4, clears a two-byte field at an offset when the difference is out of range, otherwise sets a flag, calls another routine, scales a word from the structure using a constant and stores the result back, derives a signed short from the scaled word and stores it, and invokes a helper to clear a two-byte field.
- `clear_two_byte_field`: The routine stores a zero word into a fixed offset of a structure pointed to by a4.
- `call_three_routines_2`: The routine calls another routine three times in succession and then returns.
- `call_other_routine_three_times`: The routine calls another routine three times in succession and then returns.
- `adjust_pointer_and_fields`: The routine compares a short from a register with a field in a structure pointed to by a4 and checks a range condition on a pointer difference, then calls another routine, updates the pointer by adding a constant offset, scales a word from the structure and stores it back, derives a signed short from the scaled word and stores it, and invokes a helper to compare and update table entries.
- `decrement_counter_with_guard`: Checks a global variable and two fields of a structure and, if conditions hold, decrements a word field, clamps it to zero and decrements another word field.
- `accumulate_scaled_square`: Computes a scaled square of a word field from a structure and adds the scaled value to another integer field of the same structure.
- `copy_and_test_byte_2`: The routine indexes tables using a value derived from a structure, tests a high byte for a specific value, updates several fields of a structure including a 32-bit accumulator with overflow clamping, and copies bytes from tables into structure fields.
- `copy_and_test_byte`: Clears a word field of a structure, copies a shifted word from a table addressed by a pointer and an index, and conditionally copies a signed byte from the same table into another word field.
- `update_car_steering_and_audio_2`: The routine updates steering-related fields in a car structure pointed to by a4 using values from a global table pointed to by a3 and several internal counters.
- `update_car_steering_and_audio`: The routine updates steering-related fields in a car structure pointed to by a4 using values from a global table pointed to by a3 and several input registers.
- `adjust_car_audio_and_steering_2`: The routine updates fields in a car structure pointed to by a4 using values from a global table pointed to by a3 and several input registers.
- `adjust_car_audio_and_steering`: The routine updates fields in a car structure pointed to by a4 using values from a global table pointed to by a3 and several input registers.
- `update_car_audio_pitch_2`: The routine computes a pitch value from a car structure and a global table, applies several clamps and adjustments, then stores the resulting pitch into a global variable.
- `update_car_audio_pitch`: The routine updates fields in a car structure pointed to by a4 using values from a global table pointed to by a3 and several input registers.
- `adjust_speed_and_gear_2`: The routine updates a car structure pointed to by a4, clamping a short field at 206 to a maximum and decaying two short fields at 30 and 28 toward zero, and clamping a short at 30 to a range.
- `adjust_speed_and_gear`: The routine clamps a signed index, decays two short fields toward zero, computes a speed value from a table using a base value and writes it to a global variable, then selects a new gear index based on flags and speed thresholds, recomputes speed for the chosen gear and updates the global speed and gear state if the result is in range.
- `update_car_speed_and_gear`: The routine decrements a counter in a state structure and adjusts a related field, then computes a speed value from a table using a base value derived from state fields and a map byte, clamps the result, and stores it in a global variable.
- `adjust_gear_and_speed`: The routine decrements a counter in a state structure and, when a condition on another field holds, adjusts a speed accumulator using a byte from a table indexed by a value derived from a register and a global base, then clamps the accumulator and computes a scaled speed value from a table entry.
- `scale_value_and_store_results`: The routine conditionally adjusts an input word, reads a word from a table indexed by the adjusted value, computes a scaled quotient, stores the quotient and derived values into fields of two structures.
- `update_state_from_inputs_2`: The routine conditionally reads a table entry using an input index, computes a scaled value from two inputs, stores the scaled result and de.
- `update_state_from_inputs`: The routine conditionally reads a table entry using an input index, computes a scaled value from two inputs, stores the scaled result and derived values into a structure pointed to by a pointer, and then stores several input registers and a global variable into fixed offsets of that structure.
- `store_struct_fields_from_registers`: The routine stores four words from registers and a global variable into fixed offsets of a structure pointed to by a pointer.
- `adjust_scaled_value_and_store`: The routine reads a word from a global variable and compares it with a word in a structure.
- `scale_and_store_values`: If an input word is large and another input differs, it scales a value using a table entry, stores the scaled result and derived values into fields of two structures.
- `update_car_visual_and_physics_1`: The routine updates per-car visual and physics state stored in a structure pointed to by a4, using global variables pointed to by a3 and a6.
- `update_car_visual_and_physics_2`: The routine computes a distance-based value from a car structure and updates a per-car visual and physics state, including custom hardware registers, a flag field, and a table entry.
- `update_car_visual_and_physics`: The routine updates per-car visual and physics state stored in a structure pointed to by a4.
- `clamp_and_store_offset`: Subtracts a constant from an input word, clamps the result against another register value, and stores the result as a word in a structure.
- `store_three_words_from_registers`: The routine calls another routine and then stores three word values from registers into three distinct offsets of a structure pointed to by a register.
- `shift_and_multiply`: Returns the input word shifted right by fourteen bits multiplied by two.
- `update_linear_congruential_state`: Updates four consecutive integer fields of a global structure with a linear congruential style recurrence using constant additions.
- `update_three_linked_checksum_words`: The routine reads a global variable, adds a constant to it, writes it back, then uses the updated value to update two more global variables with further constant additions, and finally updates a fourth global variable using the last computed value and another constant.
- `update_track_state_buffers`: The routine repeatedly calls another routine and then copies fields from a set of source offsets to a set of destination offsets within a structure pointed to by a register, updates a small state machine and a rotating counter, and writes a few control words to a second structure pointed to by another register.
- `swap_track_state_buffers_2_2_2_2`: The routine repeatedly calls another routine and then copies fields from a set of source offsets to a set of destination offsets within a structure pointed to by an argument.
- `swap_track_state_buffers_2_2_2`: The routine repeatedly calls another routine and then copies fields from a set of source offsets to a set of destination offsets within a structure pointed to by a register, updates a small counter and swaps two words, and writes a constant to a word in another structure.
- `swap_track_state_buffers_2_2`: The routine repeatedly calls another routine and then copies fields from a set of source offsets to a set of destination offsets within a structure pointed to by a register, updates a small circular accumulator table using an input word and a rotating index, and swaps two pointer values.
- `swap_track_state_buffers_2`: The routine repeatedly calls another routine and then manipulates fields of a structure pointed to by a register.
- `swap_track_state_buffers`: The routine repeatedly calls another routine and then manipulates fields of a structure pointed to by a register.
- `sync_state_fields_3`: The routine compares several fields from one structure to a shadow copy in another structure and updates the shadow copy when a difference is found, invoking another routine after each update.
- `update_car_display_state`: The routine compares several fields from a source structure with cached copies in a destination structure and updates the cached copies when they differ, invoking another routine after each update.
- `invoke_two_subroutines`: The routine calls two other routines in sequence and then returns without performing further operations.
- `copy_word_blocks`: The routine copies 16 words from a source pointer to a destination pointer using a fixed stride.
- `copy_and_compare_byte_block`: The routine calls another routine and then copies eight bytes from a fixed offset in a structure to a destination pointer, comparing each byte with the destination and replacing it when they differ.
- `invoke_three_subroutines`: The routine calls three other routines in sequence and then returns without performing further operations.
- `copy_byte_table`: The routine computes a source address from an input word and a global base and copies 32 bytes from that source to a destination pointer at fixed offsets.
- `scale_and_clamp_index`: The routine derives a short index from the high word of an input integer, adds a constant, and clamps the result between zero and 45.
- `copy_hunk_structures`: The routine selects a source table based on a flag in an input word and copies three fields from each of three structures to a destination area.
- `copy_long_blocks`: The routine builds a source pointer from an input word and a global base and copies 16 longwords from that source to a fixed destination area.
- `copy_long_structures`: The routine builds a source pointer from an input word and a global base and copies selected fields from ten structures to a destination area.
- `store_normalized_result`: The routine calls another routine and then stores a short value derived from an input short plus a constant and a word value into two fixed offsets of a structure.
- `store_generated_word`: The routine calls another routine, receives a word result, and stores that word into a fixed offset of a structure.
- `write_table_entry`: The routine stores a 16-bit value derived from a register plus a constant into a table entry at an offset from a pointer, and stores a 16-bit value from a register into the following table entry.
- `store_generated_value`: The routine calls another routine and stores the returned 16-bit value into a 16-bit field located at a fixed offset from a pointer in A3.
- `normalize_table_entries`: The routine walks a table of short entries using an index derived from two inputs and a global pointer, normalizes an index field in each entry, computes a value from the entry and a lookup table and writes the result back.
- `find_minimum_score`: The routine iterates over a table of shorts, normalizes an index field, computes a score from the entry and a table lookup, and keeps the smallest score seen.
- `compute_and_track_score`: The routine iterates over a table of shorts, normalizes an index field, computes a score from the entry and a table lookup, and tracks the minimum intermediate value.
- `update_table_entries_2`: The routine iterates over a table of shorts, normalizes an index field, computes a value from the entry and a table lookup, and writes the result back into the entry.
- `generate_output_buffer_2`: The routine builds a small lookup table from a table of bytes derived from a global variable and then iterates over a table of shorts, computing an index from a base value and an entry offset, clamping the result and deriving a byte from the lookup table.
- `generate_output_buffer`: The routine loops over a table of shorts, derives a byte value from a lookup table and writes it to an output buffer when it changes, and updates a set of control words in another structure each iteration.
- `select_palette_and_dispatch_4`: The routine selects one of three sets of global pointers based on a short flag in a structure and then, depending on another short flag, writes pointer words into two of those globals and performs an indirect call through a table indexed by a clamped value derived from a short field.
- `select_palette_and_dispatch_3_2`: The routine reads a flag from a structure pointed to by a register and, if clear, computes an index from a short field, clamps it, writes to two global variables using a table lookup and an indirect call through a table.
- `select_palette_and_dispatch_3`: The routine reads a flag from a structure pointed to by a register and, if clear, computes an index from a short field, clamps it, writes to global variables and dispatches through an indirect jump; otherwise it computes another index and dispatches similarly.
- `select_palette_and_dispatch_2_3`: The routine selects four pointers from a global variable based on a short field in a structure pointed to by a register, then reads a short field and a flag from that structure.
- `select_palette_and_dispatch_2_2`: The routine reads a flag from a structure pointed to by A3 and, if clear, computes an index from a short field in that structure, clamps it, stores a palette pointer and size in RAM and dispatches via an indirect jump table to another routine.
- `select_palette_and_dispatch_2`: The routine reads a flag from a structure pointed to by A3 and, if clear, computes an index from a short field in that structure, clamps it, stores a derived pointer and its high word into two global variables, then calls another routine via an indirect jump table.
- `dispatch_sprite_palette`: The routine computes an index from a short field and two input shorts, writes a derived pointer into a structure, and then dispatches to one of several routines via an indirect jump table.
- `write_sprite_palette_entries`: The routine writes a sequence of bytes derived from a base byte in D1 and a series of 16-bit words taken from a table pointed to by A4 into a buffer pointed to by A0.
- `unpack_sprite_palette_entries`: The routine copies a byte from a source table to the output buffer and then writes three 16-bit words from a palette table indexed by the next byte from the source table shifted left by three, repeating the copy-and-lookup for a series of source bytes.
- `store_generated_track_values`: The routine calls another routine, captures its return value and an extra output word, and stores both results into two fields of a structure pointed to by A3.
- `store_generated_track_values_2`: The routine calls another routine, captures its return value and an extra output word, and stores both results into two different fields of a structure pointed to by A3.
- `generate_track_vertices_2`: The routine uses an input value to set up a count and a base pointer, then iterates over a table of signed byte pairs to accumulate sums and compute a scaled offset from a table of base values and multipliers.
- `generate_track_vertices_2_2`: The routine uses an input value to set up a count and a base pointer, then iterates over a table of signed byte pairs to accumulate sums and store results into an output buffer.
- `compute_and_store_pair_2`: The routine computes a difference between a combined value from an input register and an auxiliary register and a pointer, stores zero and a table word through a pointer, and returns a concatenated pair of values.
- `compute_and_store_pair`: The routine computes a difference between a combined value from an input register and an auxiliary register and a pointer, stores zero and two copies of a word from a global variable into the pointed structure, stores a zero word, and returns a combined value derived from the difference.
- `fill_struct_conditional_2_2_3_2`: The routine walks a list of structures pointed to by an argument and, for each entry, either skips ahead, writes a fixed set of words to the output buffer, or expands a gap by repeatedly writing a pattern derived from a global table and a running accumulator.
- `fill_struct_conditional_2_2_3`: The routine walks a list of structures pointed to by an argument and, for each entry, either skips ahead, writes a fixed set of words to the output buffer using values from registers and a table, or generates a run of entries by repeatedly updating a 16-bit accumulator and writing the accumulator and a decrementing offset.
- `fill_struct_conditional_2_2_2`: The routine walks a list of structures pointed to by an argument and, for each entry, either skips ahead, writes a fixed set of words to the structure using values from registers and a table, or fills the structure conditionally by iterating while decrementing a counter and updating a running value with arithmetic derived from a table entry.
- `fill_struct_conditional_2_2`: The routine writes several words to a structure pointed to by an argument using values from registers and a table, loops while decrementing.
- `fill_struct_conditional_2`: The routine writes several words to a structure pointed to by an argument using values from registers and a table, loops while decrementing a counter, and returns early when a counter becomes negative.
- `clamp_and_copy_offsets_2`: The routine is empty except for a single call to another routine and an immediate return.
- `call_self_recursively`: The routine is empty and ends with a return.
- `clamp_and_copy_offsets`: The routine copies a word from one offset of a structure to another, computes a delta using values from a global variable and the structure, clamps several intermediate short values to non-negative ranges and to a limit, then stores the clamped results into multiple fields of the structure.
- `fill_sprite_descriptor_table_2`: The routine writes a series of sprite descriptor entries into a structure pointed to by an argument, computing offsets from a global base and several global fields.
- `write_all_ones_word_2_2`: The routine writes all ones to a 32-bit location pointed to by an argument and conditionally fills a structure with computed offsets derived.
- `write_all_ones_word_2`: The routine writes all ones to a 32-bit location pointed to by an argument and conditionally fills a structure with computed offsets derived from a global variable and registers.
- `write_all_ones_word`: The routine writes the value all ones to a 32-bit location pointed to by an argument and then returns.
- `fill_collision_box_2_2`: The routine writes a sentinel value to a global variable and, if a flag in another structure is set, computes a size and offset from two short fields and stores them into a table pointed to by a pointer argument, repeating the pattern for several flag-controlled blocks.
- `fill_collision_box_2`: The routine writes a sentinel value to a global variable and, if a flag in another structure is set, computes a size and offset from two sho.
- `fill_collision_box`: The routine writes a sentinel value to a global variable and, if a flag in another structure is set, computes a size and offset from two short fields and a register value and stores a series of repeated size and accumulated offset values into the global variable as a small table of shorts and ints.
- `clear_global_variable`: The routine writes the value a fixed address to a global variable via an address taken by the caller.
- `initialize_data_tables`: The routine copies data from a global variable into several buffers, repeatedly calling another routine and storing its results into those buffers, then updates pointers in a global structure to the end of each filled region.
- `update_three_linked_checksum_words_2`: The routine calls another routine and then loops calling another routine until a masked low nibble of the result is not less than a register value, then returns.
- `update_three_linked_checksum_words_2_2`: The routine calls another routine and then loops calling another routine until a masked low nibble of the result is not less than a register.
- `decode_and_scatter_data_3_2`: The routine builds a small header string in a buffer, fills it with spaces, then copies and XORs bytes from a table indexed by input bytes i.
- `call_update_and_decompress`: The routine calls another routine and then calls a decompression routine that writes a header, builds a filler area and expands a compressed block by indexing a table with bytes.
- `self_recursive_call_5`: The routine consists of a single call to another routine and then returns.
- `scale_and_pack_result_2`: The routine is empty except for two calls to other routines and an immediate return.
- `scale_and_pack_result_3`: The routine is empty except for three calls to other routines and an immediate return.
- `scale_and_pack_result_4`: The routine is empty except for four calls to other routines and an immediate return.
- `call_five_subroutines`: The routine calls another routine five times in sequence and then returns.
- `copy_words_with_loop`: The routine copies a long word from one offset of a global variable to another offset, then loops a fixed number of times calling another routine and storing a result through a pointer.
- `loop_until_mask_condition`: The routine calls another routine, then repeatedly obtains a value from another routine and loops while a masked low nibble comparison with an input short remains greater than -1.
- `loop_until_mask_condition_2`: The routine calls another routine, then repeatedly obtains a value from another routine and loops while a masked low nibble comparison with.
- `empty_routine_17`: The routine contains no instructions that perform work and returns immediately.
- `call_three_subroutines`: The routine calls another routine three times in sequence and then returns.
- `call_five_subroutines_2`: The routine calls another routine five times in sequence and then returns.
- `call_seven_subroutines`: The routine calls another routine seven times in sequence and then returns.
- `scale_and_pack_result`: The routine performs a modular subtraction and multiplication using an input value and values from a global variable, computes a division and remainder, conditionally adjusts a packed result, and returns a 64-bit value composed of scaled and packed components.
- `fill_struct_conditional`: The routine computes several intermediate values from inputs and a global variable, tests a derived short value against a divisor, and if the test succeeds writes multiple words and dwords into a structure pointed to by an argument using the computed values.
- `populate_struct_fields_2_2`: The routine computes several intermediate values from inputs and a global variable, then writes a series of words and dwords into a structur.
- `repeated_self_calls`: The routine immediately calls another routine six times in a row and then returns.
- `populate_struct_fields_2`: The routine computes several intermediate values from inputs and a global variable, then writes a series of words and dwords into a structure pointed to by a pointer, using conditional constants based on an input word.
- `populate_struct_fields`: The routine computes several intermediate values from inputs and a global variable, then writes a series of words and dwords into a structure pointed to by an argument, filling fields with combinations of the inputs and the computed values.
- `empty_stub_routine_83de`: The routine contains twelve 68000 instructions and performs no operations, modifies no registers, accesses no memory, and makes no library calls.
- `empty_stub_routine_2`: The routine contains three 68000 instructions that perform no operations, modify no registers, access no memory, and make no library calls.
- `perform_no_operation`: A routine that executes three instructions which do not modify any data, perform no memory accesses, and contain no library calls.
- `empty_stub_routine`: The routine contains three 68000 instructions that perform no operations, modify no registers, access no memory, and make no library calls.
- `empty_routine_18`: The routine consists of six 68000 instructions that do not modify any data, perform no memory accesses, and contain no library calls.
- `empty_routine_27`: The routine contains twelve instructions and performs no observable work; it is effectively empty.
- `empty_routine_25`: The routine consists of three instructions.
- `empty_routine_23`: The routine contains three instructions and performs no observable work; it is effectively empty.
- `stub_return_only`: Routine consisting only of a return, with no calls, memory access or hardware register use; it has 3 instructions in the listing and performs no work.
- `stub_return_only_2`: Routine consisting only of a return, with no calls, memory access or hardware register use; it has 6 instructions in the listing and performs no work.
- `validate_state_and_branch_3`: The routine checks several short fields in two structures for zero and signed ranges, then either returns a constant short or calls another routine and adjusts the returned short by adding nine when an extra output is non-zero.
- `validate_state_and_branch_2`: The routine checks a short field in a structure for zero and then validates several short fields from two structures against a set of signed comparisons.
- `validate_state_and_branch`: The routine checks a short field in a structure for zero and then validates several short fields from two structures against a set of signed bounds around a loaded short value.
- `validate_state_bounds_2`: The routine loads a short from an offset in a structure and tests it against several positive and negative bounds together with a zero test.
- `validate_state_bounds`: The routine loads a short from an offset in a structure and tests it against several positive and negative bounds together with a zero test.
- `check_and_branch_on_state`: The routine loads a short from an offset in a structure and tests it against several positive and negative bounds together with a zero test of another short field.
- `return_constant_four`: The routine returns the constant value four.
- `return_constant_three`: The routine returns the constant value three.
- `check_condition_and_branch`: The routine receives a short value in D1, adds a constant to it and checks that the result is positive and that a global word is zero.
- `call_other_and_adjust`: Calls another routine and returns its short result, adding 9 to the result when the extra output in D1 is non-zero.
- `copy_digit_tables_2`: The routine reads a short value from an argument structure, scales it to a 0-99 range and selects a pair of digit tables based on a pointer comparison.
- `call_decompress_multiple_times`: The routine calls another routine four times in sequence and then calls a decompression routine that copies a header, builds a filler area and expands a compressed block by indexing a table with bytes.
- `decode_and_decompress_hunk_2_2`: The routine copies a small header into a buffer, builds a short filler area, then expands a compressed block by indexing a table with bytes.
- `decode_and_decompress_hunk_2`: The routine copies a small header into a buffer, builds a short filler area, then expands a compressed block by indexing a table with bytes from a stream and writing the resulting bytes to several offsets in a destination area.
- `decode_and_decompress_hunk`: The routine decodes a block of data by copying bytes from a table indexed by a byte stream and then performs an in-place bit-level transposition on several word fields of the decoded block.
- `populate_track_object_entries_2`: The routine obtains an index from another routine and, if the index is non-negative, reads a byte from a table and a short from a global variable.
- `populate_track_object_entries`: The routine obtains an index from another routine and, if the index is non-negative, reads a byte from a table and a short from a global variable.
- `tiny_moveq_move_rts`: Small routine of 3 instructions that loads 0 into d0, then copies d0 to (a4)+, then returns.
- `process_table_entries_loop`: Walks a table pointed to by a pointer derived from a global variable, reads and updates fields of each entry with computed values, and conditionally calls two other routines for each entry.
- `swap_and_write_table_entries_3`: The routine walks a table pointed to by a pointer and repeatedly reads and writes fields of each entry.
- `swap_and_write_table_entries_2`: The routine walks a table pointed to by a pointer, repeatedly reading and writing fields of each entry.
- `swap_and_write_table_entries`: The routine repeatedly reads and writes fields of a table pointed to by a pointer, swapping two 16-bit values and two 32-bit values under certain comparisons and updating a 16-bit accumulator derived from a global table entry.
- `initialize_track_state_buffer`: The routine clears a word in a structure, copies a 32-bit value from a global variable into another field, then copies eight 32-bit words from a calculated source area into a destination buffer using a decrementing loop.
- `merge_track_state_flags_3_2_2_3`: The routine walks a pointer through a table of words, updating a 32-bit accumulator from a table of shorts and a global pointer, writing a byte to the table and a short to a global variable.
- `merge_track_state_flags_3_2_2`: The routine walks a pointer through a table of words, updating a 32-bit accumulator from a table of shorts and a global pointer, writing a byte derived from the accumulator into the table and handling a 64512 marker and a 64768 terminator.
- `merge_track_state_flags_3_2_2_2`: The routine walks a pointer through a table of words, updating a 32-bit accumulator from a table of shorts and a global pointer, writing a byte to the table and setting several global variables.
- `merge_track_state_flags_4`: The routine walks a pointer through a table of words, updating a 32-bit accumulator from a table of shorts and a global pointer, writing a byte to the table and storing a short to a global variable.
- `merge_track_state_flags_3_2`: The routine repeatedly calls another routine with two arguments and a global pointer, then merges words from a global variable with values i.
- `merge_track_state_flags_3`: The routine repeatedly calls another routine with two arguments and a global pointer, then merges words from a global variable with values i.
- `merge_track_state_flags_2`: The routine repeatedly calls another routine with two arguments and a global pointer, then merges words from a global variable with values in registers using bitwise operations and stores the result back to a global variable.
- `process_track_object_state_2`: The routine updates a global variable and a set of short fields in a structure pointed to by an unreferenced register, using values derived from a counter and a pointer that is moved backwards each iteration.
- `decrement_counter_load_word_2`: The routine repeatedly decrements a short counter in a global variable, loads a word from a pointer that is moved backwards each iteration, and processes the loaded words using another routine.
- `merge_track_state_flags`: The routine repeatedly calls another routine with two arguments and a global pointer, then merges words from a global variable with values derived from a register, and updates several short fields in a structure pointed to by an unreferenced register.
- `process_state_table_entry`: The routine repeatedly calls another routine with two arguments and a global pointer, then merges words from a global variable with values returned by that routine to form a 64-bit value.
- `process_race_state_table_2`: The routine drives a short counter in a global variable by repeatedly invoking another routine and copying words from a table pointed to by a register.
- `process_race_state_table`: The routine drives a short counter in a global variable by repeatedly invoking another routine and copying words from a table pointed to by an argument into fields of a global variable.
- `update_track_object_state_3`: The routine manipulates fields in a structure pointed to by a register, using an index and a value passed in registers, and conditionally clears two word fields and calls another routine.
- `reset_flags_and_maybe_load`: The routine clears two word fields in a structure to zero and -1, calls another routine, and if a set of short fields in the structure meet specific non-zero and zero conditions it computes values from the structure and input registers and calls another routine twice with adjusted arguments before returning a combined value.
- `maybe_load_track_object_2`: The routine checks a flag stored in a structure and, if the object is marked for loading, validates a distance check before copying some fields from a table.
- `initialize_track_object_state`: The routine initializes fields of a global variable, copies a block of data from a source area derived from the global variable into a destination area, computes a size value from the global variable and stores it, then sets up several counters and flags in the global variable.
- `process_track_object_state_4_2`: The routine walks a table of 16-bit entries, decrementing a counter and updating a 32-bit accumulator derived from the table values and a global variable.
- `process_track_object_state_5`: The routine walks a table of 16-bit entries, decrementing a counter and updating a 32-bit accumulator derived from the table values and a global variable, writing a byte derived from the accumulator back into the table and handling a 64512 marker and a 64768 terminator.
- `update_track_object_state_11`: The routine walks a table of 16-bit entries, decrementing a counter and updating a 32-bit accumulator derived from the table values and a global variable.
- `process_track_object_state_6`: The routine walks a table of 16-bit entries, decrementing a counter and updating a 32-bit accumulator derived from the table values and a global variable.
- `update_track_object_state_10`: The routine repeatedly calls another routine with arguments derived from a global variable and updates fields of a global variable using values from registers and the result of those calls.
- `process_track_object_state_4`: The routine repeatedly calls another routine with arguments derived from a global variable and updates fields of a global variable using val.
- `process_track_state_loop_2_2`: The routine repeatedly calls another routine with arguments derived from a global variable and updates fields of a global variable using values from the global variable and from the results of the calls.
- `process_track_object_state_3`: The routine repeatedly calls another routine with registers derived from a global variable and a table, combines words from the global variable and updates a global counter, and conditionally invokes a helper to reset the track object state.
- `process_track_data`: The routine repeatedly calls another routine with registers derived from a global variable and a table, combines words from the global variable with values returned by the called routine, and updates several words in the global variable.
- `process_game_state_loop_2`: The routine repeatedly calls another routine with registers and a pointer, then updates several short fields in a global variable using values derived from the call results and from other short fields in the same global variable.
- `process_game_state_loop`: The routine repeatedly calls another routine with registers and a pointer, then updates several short fields in a global variable using values derived from the call results and from table entries.
- `process_track_state_loop_2`: The routine repeatedly calls another routine with arguments derived from a global variable and updates fields of a global variable using val.
- `process_track_state_loop`: The routine repeatedly calls another routine with arguments derived from a global variable and updates fields of a global variable using values read from that variable and from the return value of the called routine.
- `update_track_object_state_4`: The routine updates fields in a structure pointed to by a register, using an index passed in a data register and a value passed in another data register.
- `reset_flags_and_maybe_load_2`: The routine clears two word fields in a structure to zero and -1, calls another routine, and if a different set of short fields in the structure meet specific non-zero and zero conditions it computes values from the structure and input registers and calls another routine twice with adjusted arguments before returning a combined value.
- `reset_object_flags_and_maybe_load`: The routine clears two flag words in a structure, calls another routine, then checks a set of short fields in the same structure.
- `cycle_state_flag_values_2`: The routine reads a short flag from a structure and, if it is non-zero or zero, checks the value of another short field against four specific constants.
- `state_machine_step`: Reads a short from a global variable via a pointer and, depending on whether it equals 64, 32 or 0, calls another routine and writes a new value back to the same global location.
- `check_flag_and_clear`: The routine reads a short from a global variable via a pointer and if it equals 32 it calls another routine and clears the word, otherwise if it is zero it calls another routine and writes 65504 to the word.
- `set_flag_to_negative`: Checks a global word variable; if it is zero, calls another routine and sets the variable to a negative value.
- `return_immediately`: The routine contains no instructions and returns immediately.
- `cycle_state_flag_values`: The routine reads a short value from an offset in a global variable and, for each of four specific values, calls another routine and writes the next value in a fixed sequence to the same offset before returning.
- `reset_state_flag_on_condition_2`: The routine reads a short value from an offset in a global variable and, if it matches one of three specific values, calls another routine and then writes a new short value to the same offset before returning.
- `reset_state_flag_on_condition`: The routine checks a short value stored at an offset from a pointer in a global variable.
- `clear_flag_after_call_when_zero`: Tests a 16-bit field at an offset from the A3 base register; when it is zero, calls another routine and then sets that field to minus thirty-two.
- `empty_routine_19`: The routine contains no instructions and performs no operations.
- `loop_until_flag_cleared`: The routine reads a short flag from a structure and, if it is non-zero or zero, checks the value of another short field against four specifi.
- `empty_routine_20`: The routine contains no instructions and performs no operations.
- `copy_track_object_state_2_2`: The routine checks a short field computed from two fields of a structure pointed to by an unreferenced register.
- `check_flag_and_clear_timer`: The routine tests a flag byte in a table indexed by a global variable and a global offset, calls another routine if the flag is negative, then clears a 16-bit timer field in the structure pointed to by a register and returns the original inputs.
- `decrement_counter_load_word`: The routine repeatedly decrements a short counter in a global variable, loads a word from a pointer that is moved backwards each iteration, stores the word into a field of a global variable and masks it, continuing while the masked value is within a range, and finally writes a sentinel when the counter underflows.
- `decrement_pointer_loop_store_word`: The routine decrements a pointer in a loop, loads a word from it and stores the word through a pointer to a global variable.
- `iterate_table_copy_fields`: The routine decrements a short counter in a global variable, copies words from a table pointed to by an argument into fields of a global variable, computes a difference with another global value, and when the difference is in range stores a scaled value split into two shorts; it writes a sentinel when the counter underflows.
- `iterate_table_compute_scaled_offset`: The routine iterates over a table pointed to by a pointer, copies words from the table into fields of a global variable and computes a difference with a global value.
- `index_table_copy_fields`: The routine decrements a short counter in a global variable, indexes a table using the counter to obtain an entry, copies words from that entry into fields of a global variable, computes a masked difference with another global value, and when the difference is in range stores a scaled value split into two shorts; it writes a sentinel when the counter underflows.
- `index_table_compute_scaled_offset`: The routine indexes a table using a short from a global variable, copies words from the indexed entry into fields of a global variable and computes a masked difference with a global value.
- `iterate_table_scaled_product`: The routine decrements a short counter in a global variable, copies words from a table pointed to by an argument into fields of a global variable, computes a difference with another global value, and if the difference is within a range stores the difference scaled by a constant; otherwise it writes a sentinel.
- `iterate_table_compute_scaled_product`: The routine iterates over a table pointed to by a pointer, copies words from the table into fields of a global variable and computes a difference with a global value.
- `sort_five_pairs`: The routine reads five pairs of values from a global structure, performs a series of pairwise swaps to order the pairs by their first component, and writes the five pairs back to another area of the same structure in sorted order.
- `copy_word_and_branch_2`: Copies a word from an offset in a global structure to another offset and stores the high byte of the source word to a third offset.
- `copy_track_object_state_2`: The routine stores a word from an input register into two fixed offsets of a structure pointed to by a register, with the second store receiving the high byte of the input.
- `copy_word_to_table`: The routine copies a 16-bit value from a global variable to another global variable and also stores its high byte to a third global variable.
- `store_word_and_branch`: The routine stores a 16-bit value from a register into two different offsets of a structure pointed to by a register, and stores the high byte of that value into another offset.
- `copy_state_fields_conditional`: The routine copies a short from one offset of a global structure to another, then conditionally copies a word, clears a flag word, and calls another routine when a table entry indexed by the copied short is non-negative.
- `set_status_and_delegate`: The routine conditionally copies a word, clears a flag, sets a status word to a constant, calls another routine, clears the status word, and then calls another routine with three arguments.
- `reset_track_object_state`: The routine checks a short field in a global structure and, if the condition holds, copies a word from one field to another, clears two word fields, writes a constant to a word field, calls another routine, then clears that word field.
- `copy_short_field_and_conditional_update`: The routine copies a short field from a global structure into another field of the same structure, then checks a short value from a table indexed by that field.
- `unpack_word_to_fields`: The routine reads a word from a global structure, stores its low byte masked, stores masked low bits and high byte parts into separate words, copies another short, and conditionally calls another routine based on a table lookup.
- `copy_track_state_fields`: The routine copies a byte from a global variable into a field of a structure, masks and stores parts of a 16-bit value into several fields of the same structure, copies another short field, and conditionally calls another routine based on a table lookup using an index derived from the copied short.
- `check_two_short_flags`: This routine checks two short values at fixed offsets from a base pointer.
- `empty_routine_21`: The routine contains no executable instructions and performs no operations.
- `return_immediately_2`: The routine consists solely of a return instruction.
- `update_track_object_state_5`: The routine copies several fields from one area of a global structure to another, clears a flag and conditionally sets a status word using pointers taken from the structure.
- `update_track_object_state_6`: The routine copies fields from a global structure into another area of the same structure, clears a flag, and conditionally selects a pointer from the structure.
- `copy_track_object_state`: The routine copies a few words from fixed offsets within a base structure to other fixed offsets, then selects a pointer from a table using an index derived from a short field in the structure and optionally overrides the pointer and an index value based on that field.
- `update_track_object_state_7`: The routine updates fields in a structure pointed to by a register, using an index passed in a data register and a value passed in another data register.
- `combine_track_object_state`: The routine extracts parts of two input values, calls another routine, reads several words from a structure pointed to by a pointer argument, and conditionally calls another routine up to three times with values taken from that structure and from registers.
- `update_track_object_state_8`: The routine updates fields in a structure pointed to by a register, using an index passed in a data register and a value passed in another data register.
- `compute_track_object_parameters`: The routine builds a 32-bit value from two 16-bit halves derived from input registers and a global table, then calls another routine to update a track object state using that value and several other arguments, and returns a combined result.
- `update_track_object_state_9`: The routine copies several words from fixed offsets of a base structure to other offsets, selects a pointer from a table using an index derived from a short field, and conditionally updates that field and a related pointer.
- `update_track_object_state_2`: The routine updates fields in a structure pointed to by a register, using an index passed in a data register and a value passed in another data register.
- `update_track_object_state`: The routine updates fields in a structure pointed to by a register, using an index passed in a data register and a value passed in another data register.
- `do_nothing`: The routine is empty; it contains no instructions and performs no operation.
- `initialize_input_buffer_state`: The routine updates fields in a structure pointed to by a pointer argument using values from registers and a global variable, and calls another routine.
- `generate_track_polygons`: Calculates polygon vertices and attributes for track rendering based on object state and input data.
- `maybe_load_track_object`: It checks a flag stored in a structure and, if the object is marked for loading, validates a distance check before copying some fields from a pointer argument into the structure and computing a few derived values from those fields and a global variable.
- `process_input_state`: The routine updates a set of fields in a structure pointed to by a pointer argument using values from registers and a global variable, and it calls another routine multiple times under certain conditions.
- `fill_struct_by_type`: The routine writes groups of values through a pointer, using an input word, an input value, an unused word and accumulated sums of an unused word with an unused value.
- `fill_struct_fields`: Stores an input word to a pointer, then writes a sum of an input word and an input value, two copies of a sum of an unused word and an unused value, and a word from a register through the same pointer.
- `fill_struct_repeated`: Writes several groups of values through a pointer, reusing an input word and accumulating sums of an unused word with an unused value across multiple fields, and stores a word from a register into a specific offset.
- `mask_flags_and_fill`: Masks and sets bits in a word located before a pointer, then stores an input value and two copies of a sum of an unused word and an unused value, followed by a word from a register through the pointer.
- `fill_struct_varied`: Writes multiple groups of values through a pointer using an input word, an input value, an unused value and accumulated sums, and stores words from registers into specific offsets.
- `empty_routine_22`: The routine is empty.
- `fill_struct_with_offsets`: The routine computes two base offsets from its inputs and writes a repeated pattern of those offsets, an unchanged value from a global variable, and a constant word into a structure pointed to by a pointer.
- `fill_struct_with_offsets_variant`: The routine builds an offset from its inputs and writes it together with the raw input and a value from a global variable into a structure pointed to by a pointer.
- `fill_struct_with_offsets_variant2`: The routine calculates an offset from its inputs and writes it and a value from a global variable into a structure pointed to by a pointer.
- `fill_struct_with_extended_offsets`: The routine computes offsets from its inputs and writes them into a structure pointed to by a pointer, using a repeated layout with an additional block of fields.
- `fill_struct_with_extended_offsets_variant`: The routine builds an offset from its inputs and writes it and a value from a global variable into a structure pointed to by a pointer, with an extended layout.
- `fill_transformed_quad_table`: The routine writes a set of derived integer values into a table pointed to by a pointer argument, using an input value, an accumulator, and two untouched words.
- `fill_struct_from_global_table`: The routine reads a short value from a global variable, scales it, and combines it with inputs to produce offsets.
- `update_input_state`: The routine copies bytes from a table indexed by a register into global offsets and stores input words into global locations with adjustments.
- `conditional_input_update`: Copies bytes from a table indexed by a register into global offsets, stores input words into global locations with adjustments, and conditionally calls another routine multiple times based on a global short.
- `call_three_subroutines_2`: The routine calls another routine three times in sequence and then returns.
- `update_car_state_from_inputs`: The routine writes an input word plus a constant into a field of a structure pointed to by a pointer, copies an input word into another field, derives an index from a field by shifting, stores the derived value, then loads a two-byte value from a table addressed relative to the structure using that index and stores it.
- `clamp_inputs_conditional_calls`: The routine clamps four input words against zero and limits based on a global short, then conditionally calls another routine multiple times depending on comparisons of the clamped values and their nibble parts, and returns without further action when the conditions are not met.
- `call_sprite_writer`: Calls another routine unconditionally.
- `conditional_sprite_writer_call`: Calls another routine and then conditionally calls another routine when the low four bits of an input word are non-zero.
- `dual_conditional_sprite_writers`: Conditionally calls another routine based on the low four bits of two input words, then calls another routine unconditionally, and finally calls a sprite descriptor writer.
- `write_sprite_descriptor_table`: The routine writes a descriptor into a structure pointed to by a pointer, using values from a global variable and inputs to compute positions.
- `write_sprite_descriptor_table_variant`: The routine writes a descriptor into a structure pointed to by a pointer, using values from a global variable and inputs to compute positions.
- `decode_hunk_data`: The routine builds a small header in a buffer and then copies and decodes a block of data from a table into a work area using a byte-wise transformation, after which it performs a bit-level permutation on the resulting words.
- `decode_and_scatter_data_3`: The routine builds a small header string in a buffer, fills it with spaces, then copies and XORs bytes from a table indexed by input bytes into a destination area using fixed offsets.
- `decode_and_decompress_hunk_2_3`: The routine writes a small header into a buffer, builds a short filler area, then expands a compressed block by indexing a table with bytes from a source area and copying transformed bytes into a destination area.
- `initialize_animation_entry_6`: The routine initializes fields of an animation entry structure pointed to by a register, copying values from an animation table pointer and a global counter, and sets a global variable to point to itself.
- `initialize_animation_entry_5_3_2`: The routine stores a long from a pointer into a structure field at offset 84, stores a constant word at offset 156, copies a word from the pointer into offset 88 of the structure, and writes the routine address into a global variable.
- `initialize_animation_entry_5_3`: The routine initializes fields of a structure pointed to by a register, copies values from a pointer-based table into fixed offsets of the structure, writes constant words to several offsets, updates a global variable to point to another routine, and returns early on several conditions.
- `store_long_and_words_from_pointer`: The routine stores a long from a pointer into an offset of a structure pointed to by a register, writes a constant word to another offset of the structure, and copies a word from the pointer into a third offset of the structure.
- `initialize_animation_entry_5_2_2`: The routine stores a long from a pointer into an offset of a structure pointed to by a register, writes a constant word to another offset of the structure, stores a constant word to a third offset, copies a word from the pointer into a fourth offset, and writes the routine’s address to a global variable.
- `initialize_animation_entry_5_2`: The routine stores a long from a pointer into an offset of a structure pointed to by a register, writes a constant word to another offset of the same structure, copies a word from the pointer into a further offset of the structure, and writes the routine’s address to a global variable.
- `initialize_animation_entry_5`: The routine clears several fields of a structure pointed to by a register, copies values from a pointer-based table into fixed offsets of that structure and sets a global variable to the address of another routine.
- `copy_animation_entry_fields_2`: The routine copies a long from a pointer into a fixed offset of a structure pointed to by a register and copies the next long from the pointer into another fixed offset of the same structure.
- `copy_animation_entry_fields`: The routine stores a byte from a register through a pointer.
- `initialize_animation_entry_4`: The routine copies four fields from a pointer-based table into a structure pointed to by a register, writes a constant to one word field of the structure, and stores the routine’s own address into a global variable.
- `initialize_animation_entry_3_2`: The routine initializes fields of a structure pointed to by a register, copies values from a pointer-based table into the structure, and sets a global variable to point to itself.
- `initialize_animation_entry_3`: The routine writes a constant word to an offset of a register-based structure, copies a long word from a pointer to the same structure, stores a word from the pointer into another offset of the structure, and writes the routine’s address to a global variable.
- `initialize_animation_entry_2`: The routine writes a constant word to an offset of a register-based structure, copies a long word from a pointer to the same structure, stores a word from the pointer into another offset, writes a constant word to another offset, writes a constant word to another offset, and stores the routine’s address into a global variable.
- `initialize_animation_entry`: The routine writes a constant word to an offset of a register-based structure, copies a long word from a pointer to the same structure, stores the routine’s own address into a global variable, writes another constant word and copies a word from the pointer into the structure.
- `process_animation_table_2`: The routine walks a table of animation entries referenced by a pointer and a counter in a global structure, decrementing counters and copying fields from the table into fixed offsets of a target structure.
- `copy_descriptor_to_context`: Copies several words and longs from a source pointer into fixed offsets of a target structure pointed to by a register, stores a routine address into a global variable, and writes constant and copied words into the target structure.
- `copy_descriptor_fields_3`: The routine copies a word and three longs from a source pointer into fixed offsets of a target structure pointed to by a register, stores the address of another routine into a global variable, writes a constant word and copies a word from the source into the target, then returns.
- `copy_descriptor_fields_2`: The routine copies three 32-bit values from a source pointer into fixed offsets of a target structure, stores a routine address into a global variable, and writes a constant word and a word from the source into the target.
- `copy_descriptor_fields`: Copies three 32-bit values from a source pointer into offsets of a target structure, stores a routine address into a global variable, writes a constant word and copies a word from the source.
- `copy_struct_fields_to_context_3_2`: The routine copies several fields from a source structure pointed to by a register into fixed offsets of a target structure pointed to by another register, writes a constant word to one offset of the target, stores the address of another routine into a global variable, and returns.
- `copy_struct_fields_to_context_3`: The routine copies four longs from a source pointer into fixed offsets of a target structure pointed to by a register, stores the address of another routine into a global variable, writes a constant word and copies a word from the source into the target, then returns.
- `copy_struct_fields_to_context_2`: The routine copies a word and three 32-bit values from a source pointer into fixed offsets of a context structure, stores a routine address into a global variable, and writes a constant word and a word from the source into the context.
- `copy_struct_fields_to_context`: The routine copies three 32-bit values from a source pointer into fixed offsets of a context structure pointed to by a register, copies a 16-bit value from the source into another offset of the context, writes a constant 16-bit value into a further offset of the context, stores the address of another routine into a global variable, and returns.
- `copy_mixed_descriptor_fields_2_2_2`: The routine copies four 32-bit values from a source pointer into fixed offsets of a target structure, stores a routine address into a global variable, and writes several 16-bit and 32-bit fields from the source into the target, including a constant 16-bit value.
- `copy_mixed_descriptor_fields_2_2`: The routine copies four 32-bit values from a source pointer into fixed offsets of a target structure, stores a routine address into a global variable, and writes two 16-bit values into the target structure, one constant and one taken from the source.
- `copy_mixed_descriptor_fields_2`: The routine copies four 32-bit values from a source pointer into fixed offsets of a target structure, stores a routine address into a global variable, and writes a constant word and a word from the source into the target.
- `copy_mixed_descriptor_fields`: Copies a word and three 32-bit values from a source pointer into different offsets of a target structure, stores a routine address into a global variable, writes a constant word and copies a word from the source.
- `copy_animation_descriptor_fields`: Copies multiple fields from a source animation descriptor structure into a target structure at specific offsets.
- `copy_descriptor_fields_and_set_handler_2`: The routine copies four 32-bit values from a source pointer into consecutive offsets of a target structure, stores a routine address into a global variable, writes a constant word into an offset of the target, and copies a word from the source into another offset of the target.
- `copy_descriptor_fields_and_set_handler`: The routine copies four 32-bit values from a source pointer into consecutive offsets of a target structure, stores a routine address into a global variable, writes a constant word to an offset of the target structure, and copies a word from the source pointer into another offset of the target structure.
- `copy_four_descriptor_fields`: Copies four 32-bit values from a source pointer into consecutive offsets of a target structure, stores a routine address into a global variable, writes a constant word and copies a word from the source.
- `copy_small_descriptor_to_context`: Writes constant words and longs into fixed offsets of a target structure pointed to by a register, copies words and a long from a source pointer into the structure, stores a routine address into a global variable, and writes a copied word into the structure.
- `copy_two_descriptor_fields_3`: The routine copies a word and a long from a source pointer into fixed offsets of a target structure pointed to by a register, stores the address of another routine into a global variable, writes a constant word and copies a word from the source into the target, then returns.
- `copy_two_descriptor_fields_2`: The routine copies a word and a 32-bit value from a source pointer into fixed offsets of a target structure, stores a routine address into a global variable, and writes a constant word and a word from the source into the target.
- `copy_two_descriptor_fields`: Copies a word and a 32-bit value from a source pointer into offsets of a target structure, stores a routine address into a global variable, writes a constant word and copies a word from the source.
- `initialize_descriptor_with_constants`: The routine writes constant words and a constant long into fixed offsets of a structure pointed to by a register, copies a word and two longs from a source pointer into fixed offsets of the same structure, stores the routine address into a global variable, and writes two more words from the source pointer into the structure.
- `copy_three_descriptor_fields_3`: The routine copies a word and two longs from a source pointer into fixed offsets of a target structure pointed to by a register, stores the address of another routine into a global variable, writes a constant word and copies a word from the source into the target, then returns.
- `copy_three_descriptor_fields_2`: The routine copies a word and two 32-bit values from a source pointer into fixed offsets of a target structure, stores a routine address into a global variable, and writes a constant word and a word from the source into the target.
- `copy_three_descriptor_fields`: Copies a word and two 32-bit values from a source pointer into offsets of a target structure, stores a routine address into a global variable, writes a constant word and copies a word from the source.
- `initialize_descriptor_from_source`: The routine writes constant words into fixed offsets of a structure pointed to by a register, copies two words and three longs from a source pointer into fixed offsets of the structure, stores the routine address into a global variable, and writes one more word from the source pointer into the structure.
- `initialize_descriptor_with_constant`: The routine writes a constant word into a target structure, copies three 32-bit values from a source pointer into fixed offsets of the structure and stores a word from the source pointer into another offset.
- `copy_descriptor_with_constant_2`: The routine writes a constant word into a target structure, copies three 32-bit values from a source pointer into fixed offsets of the structure, stores a routine address into a global variable, and writes a constant word and a word from the source into the target.
- `copy_descriptor_with_constant`: Writes a constant word into a target structure, copies three 32-bit values from a source pointer into offsets of the structure, stores a routine address into a global variable, writes a constant word and copies a word from the source.
- `initialize_descriptor_from_table_5`: The routine writes constant words into fixed offsets of a structure pointed to by a register, copies two words and three longs from a table pointer into fixed offsets of the structure, stores the routine address into a global variable, and writes one more word from the table pointer into the structure.
- `initialize_descriptor_from_table_4`: The routine writes a constant word into a target structure, copies three 32-bit values from a table pointer into fixed offsets of the structure and stores a word from the table pointer into another offset.
- `initialize_descriptor_from_table_3`: The routine writes a constant word into a target structure, copies three 32-bit values from a table pointer into fixed offsets of the structure, stores a routine address into a global variable, and writes a constant word and a word from the table into the structure.
- `initialize_descriptor_from_table`: Stores several words and longs from a table pointed to by an unassigned register into a structure pointed to by another unassigned register, writes a constant word into the structure, copies a word from the table into the structure, writes the routine address into a global variable, and returns.
- `initialize_object_descriptor_4`: The routine writes constant words into fixed offsets of a structure pointed to by a register, copies two words and three longs from a pointer into fixed offsets of the structure, stores the routine address into a global variable, and writes one more word from the pointer into the structure.
- `initialize_object_descriptor_3`: The routine writes a constant word into a target structure, copies three 32-bit values from a pointer into fixed offsets of the structure and stores a word from the pointer into another offset.
- `initialize_object_descriptor_2`: The routine writes a constant word into a target structure, copies three 32-bit values from a pointer into fixed offsets of the structure, stores a routine address into a global variable, and writes a constant word and a word from the pointer into the structure.
- `initialize_object_descriptor`: The routine writes a constant word and several words from a pointer into a structure pointed to by a register, copies a word from the pointer, sets a global variable to point to another routine, and writes two more constant words into the structure.
- `process_animation_table`: The routine walks a table of animation entries referenced by a pointer and a counter in a global structure, decrementing counters and copying fields from the current entry into a working block.
- `initialize_animation_descriptor`: The routine walks two indexed tables using counters in a shared structure and copies fields from the selected table entry into a target descriptor, setting several descriptor words and a global variable on each successful match.
- `initialize_descriptor_from_table_2_3`: The routine copies four 32-bit values from a table pointer into fixed offsets of a target structure and stores a word from the table pointer into another offset.
- `initialize_descriptor_from_table_2_2`: The routine copies four 32-bit values from a table pointer into fixed offsets of a target structure, stores a routine address into a global variable, and writes a constant word and a word from the table into the structure.
- `initialize_descriptor_from_table_2`: Copies words and longs from a table pointed to by an unassigned register into a structure pointed to by another unassigned register, writes a constant word into the structure, copies a word from the table into the structure, writes the routine address into a global variable, and returns.
- `initialize_animation_entry_7_2_2_2`: The routine initializes fields of an animation entry pointed to by a register, copying values from a table pointed to by another register and setting several constant words in the entry.
- `initialize_animation_entry_7_2_2`: The routine stores a constant word to an offset in a structure pointed to by a register, copies three words from a pointer into three offsets of that structure, stores the routine’s own address into a global variable, writes a constant word to another offset of the structure, and copies a word from the pointer into another offset of the structure.
- `initialize_animation_entry_7_2`: The routine stores a constant word to an offset in a structure pointed to by a register, copies three words from a pointer into three offsets of that structure and stores a fourth word from the pointer into another offset, writes the routine’s address to a global variable, stores a constant word to another offset, and copies a word from the pointer into an offset.
- `initialize_animation_entry_7`: The routine stores a constant word to an offset in a structure pointed to by a register, copies three words from a pointer into three offsets of the structure, writes the routine’s own address to a global variable, stores a constant word to another offset, and copies a word from the pointer into another offset of the structure.
- `process_animation_table_entry`: The routine walks an animation table pointed to by a register, decrementing a counter in a structure and copying fields from the table into an animation entry structure pointed to by another register.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `store_word_and_iterate_table`: Stores a word from a register into a global variable and then iterates over a table derived from another routine’s result, updating a value and calling another routine for each entry, returning early if a flag in a global variable is set.
- `decode_input_buffer`: The routine writes a word from a register into a global variable, calls another routine to obtain a value, then iterates over a 32-byte table derived from a global variable.
- `decode_input_key_buffer`: The routine stores an incoming word into a global variable and then repeatedly reads a byte from a global table, combines it with a running value to form a character, and calls another routine with the updated value and a pointer into the table.
- `decode_input_string`: The routine stores a word from a register into a global variable, calls another routine to obtain a value, builds a 32-bit word from parts of that value and a byte from a table, and then iterates twenty times updating the word with bytes from the table and invoking another routine with the updated word and several preserved arguments.
- `decode_input_key_buffer_3`: The routine stores an incoming word into a global variable, calls another routine and then repeatedly reads bytes from a global table, combines them with a running value and dispatches to another routine with the combined value.
- `call_decoder_and_return_inputs`: Calls another routine and returns the concatenation of two input registers unchanged.
- `decode_input_key_buffer_2`: The routine stores an incoming word into a global variable and then repeatedly reads a byte from a global table, combines it with a running value, and writes the result back to the global variable.
- `store_input_and_dispatch`: The routine stores a combined input value into a structure field and then calls another routine.
- `conditional_store_and_call`: Tests a word input against several ranges, stores a register value into a structure via an unassigned register, calls another routine, stores a register value into a global structure, and conditionally overwrites that field by calling another routine when two short fields are zero.
- `store_values_and_call`: Stores a value from a register into a global variable, calls another routine, and stores a register value into a different global variable.
- `search_table_and_copy_bytes`: Returns early if a flag is clear, then scans a global table for a word matching an input word, and if found copies a byte buffer either to a fixed memory area or to a caller-supplied pointer, updating a global field and optionally calling another routine.
- `call_single_routine`: Calls a single other routine and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_by_index`: Compares a value with a global variable and calls a library routine, then dispatches through a table indexed by an input value.
- `configure_interrupt_and_memory`: The routine clears two global variables, conditionally writes to several global variables and calls another routine, then computes an index from an input word and updates bytes and words in tables derived from a global base.
- `empty_routine_26`: Calls two other routines in sequence and returns without performing any other observable work.
- `empty_stub_2`: The routine is empty.
- `store_scaled_values`: The routine computes scaled values from words read via a pointer derived from a register and a global variable, stores several of those values into a structure pointed to by an input register, and writes bytes from the source into the structure.
- `store_word_to_global`: The routine stores a word from a register into a global variable and returns.
- `set_global_byte`: Stores a constant byte value into a global variable and returns.
- `empty_routine_3`: The routine contains 11 instructions and performs no observable work.
- `check_interrupt_and_update_state`: The routine checks a bit in a hardware register and a bit in another hardware register, then conditionally writes a masked value from the second register to a global variable and returns an input value.
- `mask_and_store_word`: Masks an input word and stores the result through a pointer, then returns a value from the stack.
- `set_interrupt_flag_and_call`: The routine reads a byte from a global variable and stores it through a pointer, then if the low bit is set it optionally calls another routine based on a global variable and writes a constant to a global variable before returning.
- `jump_vector_entry_2`: Is a jump table entry that transfers control to a fixed address elsewhere in memory.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `initialize_audio_and_state`: The routine sets several global variables based on an input pointer and a signature check, finds the maximum byte in a small block, and writes constants to hardware registers related to audio and DMA.
- `scale_buffer_entry_limits`: The routine iterates over a series of entries derived from an input pointer and computes scaled limits from words in each entry.
- `process_buffer_entries`: The routine loops over entries derived from an input pointer, computes scaled limits from words in the entry, and writes those limits and bytes from the entry into a structure pointed to by registers, repeating until a counter expires.
- `update_audio_channels_and_flags`: The routine increments a counter and conditionally resets it using a global variable, then manipulates several global bytes and a global word.
- `write_word_and_call`: Writes a word formed by ORing a memory value with a constant to another memory location and calls another routine.
- `copy_conditional_fields`: Copies several word and long fields between two memory areas conditionally and updates a byte field in a loop until a termination byte is reached.
- `check_flag_and_increment`: The routine tests a byte at an offset from a register and increments a word at another offset from the same register, then recurses four times.
- `decode_tile_attribute`: The routine copies a 32-bit value from one pointer to another and derives a 16-bit field from it.
- `copy_fields_and_dispatch`: The routine copies two word fields within a structure, masks a word from the structure, indexes a table with the masked value and dispatches through a function pointer table.
- `unknown_routine`: The routine consists of seven 68000 instructions that are not recognized by the decompiler, so its control flow cannot be recovered.
- `select_table_entry_by_index`: The routine reads a byte from a global variable to choose a mode and then computes an index from input registers.
- `select_table_entry_by_index_2`: The routine reads a byte from a global variable to choose a mode and computes an index from an input register.
- `clamp_and_copy_short`: The routine subtracts an input short from a field in the input structure, clamps the result to a minimum value, and copies the clamped value to another field in the same structure.
- `clamp_short_to_fields`: The routine limits an input short to a maximum value and stores the limited value into two separate short fields of the input structure.
- `update_counter_and_flag`: The routine checks a nibble in the input structure and, if set, updates a byte field from a global table, clears a flag, increments a counter with a bound check, and negates a byte field in the structure.
- `store_masked_byte`: The routine masks an input byte and, if non-zero, stores the masked value into a byte in a global variable and clears another byte in the same global variable.
- `toggle_bits_with_mask`: The routine toggles a bit in a byte pointed to by a register and a bit in a byte in a global variable using a mask derived from an input byte, then truncates due to bad instruction data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_high_score_screen`: The routine relocates a block of code and data tables within the runtime segment, zeroes a large buffer, writes constants to several words and longwords at fixed offsets, copies a block of words from a global variable area into a table, and then writes a longword to a fixed offset before calling another routine.
- `single_instruction_stub`: The routine contains a single instruction and is entered from outside the static flow during the reference run.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_track_state_and_loop`: The routine writes constants to a set of words in a base structure, repeatedly invokes another routine, and loops while polling two status words in that structure, updating a counter and re-initialising the same words with different constant values on each pass.
- `call_external_function`: The routine calls another routine once and then returns.
- `call_external_function_2`: The routine calls another routine once and then returns.
- `store_byte_from_register`: The routine stores a byte from a register into a byte at an offset from a global variable and then returns.
- `read_and_clear_byte`: The routine reads a byte from an offset of a global variable, clears that byte to zero, and returns the original byte.
- `initialize_state_and_read_input`: The routine writes constant words to several offsets from a register, calls another routine, conditionally calls it again based on two short flags, and sets a byte global variable from a register-derived value combined with a mask.
- `set_camera_parameters`: The routine reads a set of words from a structure pointed to by a register, optionally calls another routine to scale two values, computes a scaled divisor from two words with a minimum clamp, stores the results into a second structure, and derives a short offset from a difference of two ints with sign handling and a conditional zeroing, then writes a pointer from a table plus a constant into the second structure.
- `scale_value_and_branch`: The routine scales an input word by a constant factor using a shift, clamps the result to a minimum, and conditionally calls another routine based on the scaled value and an input flag.
- `process_object_state`: The routine conditionally copies a small block of words from a table indexed by a field in a structure, decrements two short counters and optionally calls another routine, and updates several short and integer fields in the structure using comparisons and chained additions.
- `update_table_entries`: The routine conditionally reads short values from a table indexed by an input word and from a global variable, compares four short fields across entries, and on a match writes a short from the table, a constant word, and the input word into offsets of a global variable before returning.
- `process_audio_channels`: The routine iterates over four entries, reads a word from a global variable, and when the word indicates a specific state it writes to custom hardware registers AUD0LCH, AUD0LEN, AUD0PER, AUD0VOL and custom+0, updates pointers and words in a global variable, and returns a value derived from the processed entry.
- `dispatch_byte_based_on_flag`: The routine calls another routine to obtain a byte, stores a constant into a global variable, and then conditionally writes a byte or a word to memory locations derived from a global variable based on the returned byte value, calling another routine in some branches.
- `store_byte_and_call`: The routine stores a byte from D0 into a byte at an offset from a global variable and then calls another routine.
- `conditional_call_and_return`: Tests a word at an offset from an unassigned register and conditionally calls another routine, then returns a value from the stack.
- `set_global_flags`: The routine writes the constant 1 to a word at an offset of a global variable and to a global variable, then returns.
- `process_input_buffer`: Reads a byte from a global variable and, if it matches a sentinel, writes the constant word 65535 to another offset of the same global variable and returns.
- `write_constant_word`: The routine writes the constant word 65535 to an offset of a global variable and then returns.
- `initialize_control_bytes`: The routine writes zero to a byte offset, copies a byte from a register to another byte offset, and writes the constant word 1 to a word offset of a global variable before returning.
- `empty_routine_4`: The routine contains no instructions and returns immediately.
- `fetch_input_byte`: Returns a byte from a global variable, clearing it when a hardware register bit is set and the byte is non-zero, and recursively calls another routine to obtain the value.
- `wait_for_flag_and_buffer`: The routine waits for a bit in a word at an offset from A6, writes a word to an offset from A6, then conditionally writes a byte from a word at an offset from A6 into a buffer in a global variable and updates indices.
- `conditionally_update_memory`: The routine checks a byte from a register and, if it matches a specific value, stores a constant word through a pointer derived from a register and increments a word in a global variable via another register.
- `store_word_and_recurse_5`: The routine reads a word from an offset of a register-based structure, masks it, conditionally stores the masked value back to another offset of the same structure, calls another routine, and returns a value formed from two input registers.
- `store_word_and_recurse`: Stores an input word at an offset from an unassigned register, calls another routine, and returns a value from the stack.
- `check_flag_and_store_word`: The routine reads a flag from a word at an offset from a pointer register, and if the flag is clear it calls another routine; otherwise it stores the flag into another word at an offset from the same pointer register and then calls another routine.
- `store_word_and_recurse_2`: The routine stores a word from a data register into an offset from a pointer register, then calls another routine and returns the value that was on the stack at entry.
- `set_flags_and_call_updates`: The routine stores constant words to offsets of two register-based structures, invokes another routine three times between the stores, and sets a word in the first structure to a combined constant value before the final call.
- `invoke_update_routines`: The routine calls another routine twice in succession and then returns.
- `reset_counter_and_call`: Clears a word at an offset of a global variable and then calls another routine.
- `store_constant_word`: The routine writes the constant value one to a word located at an offset from a register and then returns.
- `increment_counter_and_recurse`: The routine writes a constant word to an offset from A6, increments a short counter in a global variable and recursively calls another routine while the counter is less than a limit.
- `call_other_routine_3`: The routine calls another routine and then returns.
- `wait_for_flag_then_mask`: The routine loops until a specific bit in a word accessed through a register is set, then returns the low byte of that word.
- `store_masked_word`: The routine stores a word formed from the low byte of an input register combined with a constant into memory accessed through a register.
- `call_update_game_state_and_timers`: The routine calls another routine that updates several short counters and eight 32-bit position values by decrementing them and copies two word fields between structures, then returns the concatenation of the two input registers.
- `loop_and_call_routines`: Loops eleven times calling another routine, then calls six other routines in sequence and returns.
- `call_another_routine`: The routine invokes another routine and then returns without further processing.
- `call_palette_update`: The routine invokes a single subroutine responsible for applying palette data to display planes.
- `conditional_indirect_call`: The routine returns zero unless a table entry indexed by a register is non-zero, in which case it calls another routine and returns its result.
- `conditional_indirect_call_2`: Returns zero unless a table entry indexed by a register is non-zero, in which case it calls another routine indirectly via the table and returns the result.
- `recursive_self_call`: The routine immediately calls itself and then returns.
- `recursive_call_self`: The routine calls another routine and then returns without performing any other operations.
- `apply_palette_to_planes`: The routine copies two 32-bit values from a pointer into a global structure and then processes a block of data using a table pointer and two base pointers.
- `call_two_routines_and_set_flag`: Calls a routine via a pointer stored in a global variable, stores the returned address in another global variable, calls the routine again, and stores the constant 1 into a word at an offset from a register.
- `conditional_call_via_flag`: Checks a word at an offset from a register and if it is non-zero calls a routine via a pointer stored in a global variable.
- `clear_track_state_and_reset`: The routine writes a constant word to an offset from a register-based pointer, clears a word at an offset from another register-based pointer, calls another routine, and returns.
- `reset_track_state_and_process_table_3`: The routine clears several words in a structure pointed to by a register, sets a few control words with constant values, calls other routines and waits on a short flag in the same structure to become non-zero, repeating the sequence with different constants and looping back if a status word is non-zero.
- `clear_table_and_process`: Zeroes a block of 10000 words pointed to by a global variable and then iterates over a small set of entries, reading bytes from a global buffer and calling another routine with values derived from the buffer and a code pointer.
- `empty_routine_5`: The routine contains no instructions.
- `initialize_memory_and_pointers`: Writes constant values to several memory-mapped locations, computes a word from a register offset and stores it, clears several words at register offsets, loads pointers from a global variable conditionally, stores pointer low and high parts to memory-mapped locations, and writes a constant to memory-mapped locations with a conditional variation.
- `wait_for_condition`: Stores a byte derived from an input register into a global variable and, if a flag is clear, waits in a loop that repeatedly calls another routine until a short counter matches a target or a flag changes, returning a sentinel value on exit.
- `wait_for_short_to_be_zero`: Loops while a short at an offset from a register is negative or greater than -1, decrements an input short until it reaches -1, and returns the short at the offset.
- `reset_track_state_and_process_table_2`: The routine clears a block of words in a structure, sets a few flags based on a short field and writes constant values to several fields.
- `chain_calls`: Calls four other routines in sequence and returns.
- `copy_and_compare_buffers`: If a byte at an offset from an argument is not a specific value, copies a block of bytes from the argument to a buffer at an offset from a register, then repeatedly compares two blocks of bytes from that buffer and swaps bytes between two sub-blocks when the blocks match.
- `compute_and_store_byte`: Repeatedly reads a word from an offset in a register, mixes it with an accumulator using shifts, and decrements a counter; loops until a condition on the accumulator is met, then stores the low seven bits of the accumulator into a byte at an offset from another register.
- `fill_bytes_from_register`: Reads a byte from an offset in a register and writes it repeatedly to a consecutive block of 32 bytes starting at another offset from the same register.
- `copy_blocks_and_call`: The routine writes constants and derived bytes to fields in a global variable, copies eight bytes from one area to another and eight bytes from another area to a third area, then calls another routine.
- `copy_bytes_with_conditional_load`: If a short at an offset from a register equals 2, copies a byte from one offset to another, then copies eight bytes from one buffer to another and copies eight bytes from a third buffer to a fourth using pointer increments.
- `copy_blocks_and_call_2`: The routine writes constants and derived bytes to fields in a global variable, copies nine bytes from one area to another and nine bytes from another area to a third area, then calls another routine.
- `update_counters_and_copy_bytes`: Decrements two bytes at offsets from a register, updates a word at an offset based on a comparison, then copies nine bytes from one buffer to another and copies nine bytes from a third buffer to a fourth using pointer increments.
- `write_fields_and_call`: The routine writes constant bytes and a byte from a global variable to fields in a global variable and then calls another routine.
- `copy_byte_from_offset`: The routine copies a byte from an offset within a block pointed to by a register to another offset within the same block.
- `copy_blocks_and_format_digits`: The routine copies four 10-byte blocks from offsets of a base pointer to other offsets of the same base, calls another routine, then writes eight ASCII digits derived from a byte table at an offset of the base into two output buffers pointed to by registers, calling another routine between the two writes.
- `format_two_numbers_into_buffer`: The routine calls another routine and then writes the decimal digits of two values returned in registers into a buffer pointed to by a pointer, three digits per value, by repeated division and remainder operations.
- `sort_and_normalize_strings`: The routine writes four constant four-byte markers into a table of four 24-byte entries in memory, builds a four-element flag array from two short flags in the same memory block, clears the entries whose flag is zero, then performs an in-place bubble sort of the four entries by comparing selected bytes and swapping whole entries when out of order, and finally replaces leading '0' characters in each entry with spaces.
- `store_pointer_and_wait`: The routine stores a pointer derived from a base register into a field at an offset of the base, stores a byte-derived short into another field, spins until that short becomes zero, then calls another routine.
- `initialize_globals_and_return`: The routine writes several constants to global variables and memory locations derived from an unassigned register, sets a global variable to the address of another routine, and returns a constant byte.
- `process_input_and_flags_2`: Reads a byte from hardware register CIAA_PRA and a byte from custom hardware, combines them into a derived byte and stores it through a pointer in A0.
- `process_input_and_flags`: The routine computes a byte from two bytes at offsets from A0, stores it in a global variable, sets a flag byte at an offset from A0, writes a constant byte at an offset from A0, and conditionally updates a flag byte in a global variable based on the computed byte, then writes a constant word to an offset from A6.
- `clear_bit_in_memory`: The routine clears bit six of a byte located at an offset from a pointer in A0 and writes the constant eight to a word at an offset from A6.
- `set_flag_or_one`: The routine ORs an input byte with one and stores the result to a byte at an offset of a base register, stores a constant word to an offset of another register, and returns a value from the stack.
- `set_flag_or_two`: The routine ORs an input byte with two and stores the result to a byte at an offset of a base register, stores a constant word to an offset of another register, and returns a value from the stack.
- `set_flag_or_eight`: The routine ORs an input byte with eight and stores the result to a byte at an offset of a base register, stores a constant word to an offset of another register, and returns a value from the stack.
- `set_flag_or_four`: The routine ORs an input byte with four and stores the result to a byte at an offset of a base register, stores a constant word to an offset of another register, and returns a value from the stack.
- `set_flag_or_sixteen`: The routine ORs an input byte with sixteen and stores the result to a byte at an offset of a base register, stores a constant word to an offset of another register, and returns a value from the stack.
- `clear_flag_and_one`: The routine ANDs an input byte with a mask that clears bit zero and stores the result to a byte at an offset of a base register, stores a constant word to an offset of another register, and returns a value from the stack.
- `clear_flag_and_two`: The routine ANDs an input byte with a mask that clears bit one and stores the result to a byte at an offset of a base register, stores a constant word to an offset of another register, and returns a value from the stack.
- `clear_flag_and_four`: The routine ANDs an input byte with a mask that clears bit three and stores the result to a byte at an offset of a base register, stores a constant word to an offset of another register, and returns a value from the stack.
- `mask_byte_and_set_word`: The routine masks a byte from a register with 251 and stores the result through a pointer derived from a global variable, then stores the word value 8 through another pointer derived from a global variable, and returns the incoming stack value.
- `mask_byte_and_set_word_2`: The routine masks a byte from a register with 239 and stores the result through a pointer derived from a global variable, then stores the word value 8 through another pointer derived from a global variable, and returns the incoming stack value.
- `move_byte_to_word`: The routine reads a byte from an offset in A3, writes 255 to that byte location, and stores the original byte as a word at a nearby offset in A3.
- `swap_two_words`: The routine swaps the contents of two long words located at consecutive offsets in A3 by loading one into a temporary and exchanging the two stores.
- `mask_word_and_store`: The routine masks a word at an offset from A6 with three and stores the result at another offset from A6, then returns the word passed in D0.
- `process_interrupt_state`: The routine checks a hardware interrupt request register and, if a specific bit is set, updates a global variable and repeatedly calls another routine.
- `update_two_digit_display`: The routine updates a two-digit display when a timer condition is met and a flag is clear.
- `update_two_digit_display_2`: The routine updates a two-digit display stored in two global bytes.
- `update_display_digits_and_clear_buffer`: The routine reads a range from a global variable and conditionally updates a short counter in a global variable, writes two ASCII digit characters to global variables, builds a masked word from those digits, clears a block of words in a buffer pointed to by a global variable, and then repeatedly calls another routine with data derived from the digits and loop counters.
- `copy_track_data`: The routine selects an index from a global variable using a table and copies blocks of data from four source tables into a destination buffer pointed to by a global variable, with two different layouts depending on a flag in a global variable.
- `conditional_double_call_2`: Checks a short value from a global variable and, if it matches a constant, calls another routine twice.
- `update_track_position`: The routine updates a two-byte track position field in a global structure using a control byte, applying wrap-around adjustments and bit masking, and then synchronizes a secondary position field.
- `adjust_track_position`: The routine adjusts a short track position stored in a global variable using a mode byte from another global variable, applying bit masks and offsets, and writes the result back.
- `recursive_self_call_7`: Calls itself three times in immediate succession and then returns.
- `store_constant_and_call_2`: Calls itself three times in immediate succession and then returns.
- `set_byte_in_table_and_recurse_2`: Calls itself three times in immediate succession and then returns.
- `update_text_buffer`: The routine updates a small text buffer pointed to by a global variable using a byte from a table indexed by a counter.
- `update_text_buffer_2`: The routine updates a second small text buffer in the same way as the first, clearing it on first use and handling backspace and character insertion with optional case conversion, using its own position and limit counters.
- `update_text_buffer_3`: The routine updates a third small text buffer similarly, initializing it with spaces on first use, processing backspace and character insertion from a table with optional case conversion, and maintaining its own position counter.
- `store_constant_and_call`: The routine stores a constant byte at an offset from a global variable indexed by a short in the same global variable and then calls another routine.
- `store_masked_byte_2`: Stores a byte derived from a masked short value into a byte at an offset from a global variable and then calls another routine.
- `set_config_flag_and_call`: The routine writes a constant byte to a byte in a table indexed by a short value from a global variable, then calls another routine.
- `set_flag_from_config_and_call`: The routine reads a byte from a table indexed by a short value stored in a global variable, masks a 16-bit value from another global variable, shifts the result and stores it as a byte into a global variable.
- `set_byte_in_table_and_recurse`: The routine stores a constant byte into a byte of a table addressed by adding an offset to a global variable and an index taken from another global variable, then calls another routine and returns.
- `set_flag_byte_from_table`: The routine reads a byte from a table indexed by a byte stored in a global variable and writes a transformed bit from a global word into a byte at a computed offset in a global structure, then calls another routine.
- `copy_hunk_data`: The routine copies blocks of data from a source table indexed by a byte value to a destination area in RAM, iterating over a fixed number of entries and writing three bytes per inner iteration.
- `nested_hunk_mask_apply`: Iterates over a table of words and bytes, calling a routine that applies a hunk mask for each combination, using nested loops.
- `indexed_hunk_mask_apply`: Indexes into a global variable using an input word, then iterates over bytes calling a hunk mask routine with incrementing counters.
- `copy_and_mask_data`: The routine reads a short from a fixed offset of a register-derived base and returns it.
- `conditional_copy_and_mask`: If a short value differs from a constant, copies bytes from a global variable to a buffer and then applies a hunk mask in nested loops.
- `apply_hunk_mask`: The routine reads bytes from a source table and combines them with bytes in a destination buffer using bitwise AND and OR operations, repeating the operation for a fixed number of entries.
- `apply_hunk_mask_2`: The routine performs the same bitwise AND and OR combination of source table bytes with a destination buffer as the previous routine, iterating over a fixed number of entries with the same masking logic.
- `conditional_decrement_check`: Compares two shorts and checks a flag byte; if conditions hold it calls a routine to decrement a counter and returns a non-zero value.
- `decrement_counter`: The routine decrements a short counter stored in a global variable by one.
- `copy_palette_entries`: The routine copies a series of short palette entries from a table to a global variable area, advancing both source and destination pointers for a fixed count.
- `initialize_table_entries`: The routine writes a series of entries into a global variable structure, storing a short value and a derived short from an input value, repeating for a fixed number of iterations.
- `generate_lookup_table`: The routine fills a table at a fixed address with computed values derived from a source table in a global variable, using arithmetic on nibbles and a varying multiplier across nested loops.
- `set_global_and_call`: Stores a value from a global variable into another global variable, calls another routine, then conditionally writes a constant to a global variable and calls another routine again, with a further conditional call if a short at an offset from an unreferenced register is zero.
- `store_self_pointer_and_call`: Stores the address of itself into a global variable, calls another routine, then conditionally writes a constant to a global variable and calls another routine again, with a further conditional call if a short at an offset from an unreferenced register is zero.
- `copy_global_to_global_then_loop`: The routine copies a value from one global variable to another, then calls another routine.
- `check_flag_and_recurse`: The routine stores a pointer to itself in a global variable and calls another routine.
- `initialize_and_process_hunks`: Performs several initialization steps, loops to compute a value, clears memory areas, applies a hunk mask over tables, and copies data from a computed source to a destination buffer.
- `single_call_wrapper`: Calls another routine once and returns.
- `call_other_routine`: Calls another routine and then returns.
- `call_other_routine_4`: The routine calls another routine once and then returns.
- `conditional_double_call`: Calls another routine and then conditionally calls another routine if two shorts at offsets from an unreferenced register are both zero.
- `check_two_flags_and_call`: The routine calls another routine, then checks two short values from a structure pointed to by an uninitialized register.
- `copy_table_on_condition`: Calls another routine and, if a short at an offset from an unreferenced register is zero and remains zero after another call, calls another routine and copies a block of data from a fixed address to a pointer obtained from an offset of the unreferenced register using a countdown loop.
- `indexed_hunk_mask_apply_2`: Calls itself once and then conditionally calls itself three more times based on a short field at an offset from an uninitialized register.
- `copy_pointer_block_2`: Calls itself once and then conditionally performs a nested sequence of self calls based on a short field at an offset from an uninitialized register.
- `conditional_single_call`: Calls another routine and then conditionally calls another routine if a short at an offset from an unreferenced register is zero.
- `copy_pointer_block`: The routine calls another routine, then copies 12000 long words from a pointer located at a fixed offset of a register-derived base to a fixed address using a decrementing loop.
- `recursive_self_call_3`: The routine calls itself twice in sequence and then returns.
- `fill_memory_with_calls`: The routine runs nested loops that repeatedly call another routine and store the returned value into several memory locations derived from registers and a global variable.
- `initialize_data_block`: Performs nested loops that read from a fixed address and write multiple fields into a destination area derived from a pointer at an offset of an unreferenced register, initializing a large block of data with repeated patterns.
- `mask_and_merge_buffers`: The routine iterates over a fixed number of words, building a mask from three words of a source buffer and AND-ing it with six words of a destination buffer, then OR-ing the source words into the same destination words.
- `rotate_right_16_bits`: The routine takes a 16-bit value from the input register, extracts its bits one by one and builds a new 32-bit value by shifting the accumulator right and inserting the extracted bit at the top.
- `branch_on_flag`: The routine calls another routine and then tests a short value from a global variable.
- `copy_and_xor_blocks`: The routine loads a base pointer from a global variable, copies a block of words to two offset locations, then performs three separate loops that compute the AND of three words from different offsets and XOR the result back into those words.
- `reverse_copy_and_merge_words`: The routine loads a base pointer from a global variable, copies a block of words backwards from one offset to another, then copies a block forwards, and finally merges four word buffers with bitwise OR and AND-NOT operations while updating pointers each iteration.
- `reverse_copy_and_merge_words_small`: The routine loads a base pointer from a global variable, copies a block of words backwards between two offsets, then copies a smaller block forwards, and merges four word buffers with bitwise OR and AND-NOT operations while updating pointers each iteration.
- `initialize_palette_tables`: The routine writes a set of constant values into a global table, derives a second table from it using a fixed arithmetic formula, and then fills several other global tables with computed values based on a global variable and a global short.
- `initialize_tables_and_copy`: The routine clears and initializes blocks of memory obtained from a global variable and a global variable table, updates two short values in a global variable area in a loop, and then copies a computed number of words from a source buffer to a destination buffer in reverse order.
- `copy_masked_bits_to_buffers`: Stores a pointer into a global variable table and then iterates a computed count, reading bytes from a source buffer with a rotating index and setting or clearing individual bits in two destination buffers at offset 42.
- `load_short_from_table`: Reads a short from a structure, optionally adds an offset from another field, and copies a word from a global variable table indexed by that value into a field of the structure.
- `call_another_routine_2`: Calls another routine and then returns.
- `rotate_word_blocks`: Computes two pointers from fields of a structure and repeatedly copies a fixed pattern of words between them with overlapping offsets, updating the pointers each iteration.
- `reset_track_state_and_process_table`: The routine clears a block of long words in a structure, resets a few fields and then iterates over a table of bytes indexed by a short counter.
- `copy_table_to_structure`: Iterates over a global variable table of shorts and copies corresponding long values into a structure area using the short as an index.
- `scan_structure_and_store_count`: Calls another routine and, if two flags in a structure are zero, scans a backward area of the structure for a marker value and stores a derived count into a field.
- `call_other_routine_2`: The routine consists of a single call to another routine and then returns.
- `swap_and_reorder_table_entries`: The routine copies two word fields within a structure, optionally swaps them, computes four index values from flags and zero tests, writes one of those indices to a field, then calls another routine ten times and conditionally swaps two dword fields returned by that routine.
- `clear_word_range`: Clears a fixed range of words starting at a constant address by writing zero to each word in a loop.
- `copy_and_zero_words`: Copies a block of words from a global variable area offset by a structure pointer into another area of the structure while zeroing a preceding long in each iteration.
- `call_two_helpers`: The routine calls another routine twice and then returns.
- `call_two_routines_3_2`: If a short field at an offset from an uninitialized register is non-zero, it loops a fixed number of times clearing a word at an offset and calling another routine four times, storing each returned word into successive offsets of four different output pointers.
- `validate_and_return_input`: Examines the high byte of an input long against a set of allowed characters and register values, conditionally sets a flag in a structure, and returns either zero or the original input.
- `set_track_flags_from_bytes`: Iterates over a table of structures in memory and updates a 16-bit flag field in each entry by masking it and OR-ing in a value derived from a byte in the structure.
- `update_track_bounds_and_clip`: The routine scans a table of 1024 entries, each with six bytes, to find per-entry minimum and maximum values for two bytes and stores the adjusted extremes.
- `copy_word_table_to_globals`: The routine computes a pointer from a global variable and a short offset, then copies sixteen words from that table into several global variables in an interleaved pattern.
- `sync_state_fields_4`: Writes constant word and long values to a series of offsets from an uninitialized register, then calls two other routines.
- `reset_game_state`: The routine clears many fields in a structure pointed to by a register, sets a few control words, and invokes another routine multiple times with a mode flag changed between calls.
- `return_concat_of_two_inputs`: The routine calls another routine and returns a 64-bit value formed by concatenating the two input words in D0 and D1.
- `read_joystick_and_update_state`: The routine reads a word from a memory area pointed to by a register, derives a byte from its bits and a global variable, stores it to a global byte, then swaps two bytes and computes a change mask.
- `update_game_state_and_timers`: The routine updates several short counters and eight 32-bit position values by decrementing them, copies two word fields between structures, manipulates flags in a structure pointed to by a register, conditionally clears custom hardware registers, and calls other routines in a fixed order while adjusting interrupt enable state via INTENA and INTREQ.
- `other_routine`: The routine calls another routine twice and then returns.
- `increment_timer_bytes_with_wrap`: The routine calculates a pointer from an input short and a base register, then loops eight times decrementing the pointer.
- `conditionally_update_counters_and_sum`: The routine checks a masked byte in a structure pointed to by a register and, if zero, clears two words and conditionally decrements two counters while adding a value from a global table indexed by another field to two accumulators, then calls another routine.
- `conditionally_set_flags_and_call`: The routine compares two pairs of integers from a structure pointed to by a register and sets two flag words to two or one depending on the comparison and on counter values.
- `conditional_copy_state_blocks`: The routine checks two short flags from a base register and, if conditions are met, copies a 32-bit value and a 16-bit value from one pair of global locations to another pair.
- `set_mode_flag_based_on_flags`: The routine tests several short flags from a base register and, depending on the combination, writes 1, 2 or 0 to a global word.
- `combine_and_store_position_data`: The routine builds two 32-bit values by combining words from several global locations with masks and stores them to global words.
- `update_position_and_flags`: The routine extracts nibbles from a byte and stores them to two global bytes, then adjusts two 32-bit values using short offsets and sign handling.
- `copy_state_to_backup_and_table`: The routine copies several 32-bit and 16-bit values from one set of global locations to another set.
- `increment_counter_with_reset`: The routine increments a short counter stored in a global location unless it equals a sentinel value, and resets it to a constant when it reaches a specific value.
- `decrement_counter_and_reset_2`: Calls six other routines in sequence, with one of them conditionally executed based on a short field at an offset from an uninitialized register.
- `swap_state_words_and_check_flag`: The routine swaps two word fields within a structure pointed to by a register, calls another routine before and after the swap and once more conditionally, and then checks a short flag field in the same structure to optionally call the routine again.
- `update_timer_and_flags_2`: If a short field at an offset from an uninitialized register is non-zero, it calls seven other routines, with one conditionally executed based on another short field.
- `compare_and_set_direction_flag`: The routine averages two signed bytes from a base register, compares the result with a short value from another register, and writes a byte with a fixed bit set plus a direction indicator to a byte in the second register.
- `swap_state_fields_and_recurse`: The routine checks a short flag in a structure pointed to by a register and, if set, copies a short field to another field, calls another routine, swaps two short fields using a temporary, calls another routine again, conditionally calls another routine based on a different short flag, and then calls another routine before returning.
- `increment_state_and_select_pointer`: Increments a counter in a global variable and selects a pointer from a global variable based on a short field.
- `process_track_object_state`: The routine updates several short fields in a structure pointed to by a register, increments counters and masks them, resets flags when a counter wraps, and conditionally calls another routine.
- `update_table_entries_3`: The routine walks a table indexed by a global variable and an input word, compares four short fields across the table and the variable, and conditionally updates a short field in the table and a short field in a global variable.
- `update_table_entries_4`: The routine iterates over a table using a global variable and an input word, compares short fields from the table and the variable, and conditionally clears a flag in the global variable.
- `perform_three_recursive_calls`: The routine calls another routine three times in a row and then returns.
- `self_recursive_triple_call`: The routine calls another routine three times in succession and then returns.
- `update_table_entries_5`: The routine checks a short value derived from an input register and a global variable against a range and an input word difference.
- `decrement_counter_and_reset`: Decrements a short counter in a structure and, when conditions on other fields hold and the counter reaches zero, writes a constant to a word field and decrements another short field.
- `accumulate_squared_offset`: Computes a value from a short field by squaring it, masking and shifting, then adds the result scaled to an integer field in a structure.
- `update_timer_and_flags`: Reads a value from a table using an index derived from a structure field, checks a marker, and if set updates a timer field, a flag word, and a combined value with clamping, then writes additional table-derived values to fields.
- `update_track_position_2`: The routine updates fields in a structure pointed to by a register, handling a short counter and a short velocity field with sign changes and clamping, and it calls another routine to adjust a position value using a table lookup and scaling based on a short index and a short divisor.
- `update_car_position`: The routine updates fields in a structure pointed to by a register, handling a short counter and a short velocity field with sign changes and clamping, then computes a new short position value from a base short and a table lookup indexed by a short from a table, using a global table and a global variable for scaling.
- `update_table_entries_6`: The routine updates fields in a structure pointed to by a pointer argument using values from a global variable and a table, computes a scaled value from several short fields, writes a short to a custom hardware register and to a table entry, and stores a 32-bit fixed-point value back into the structure.
- `store_scaled_input_words`: The routine calls another routine and then stores three word values from registers into three fixed offsets of a structure pointed to by a pointer.
- `scale_input_word`: Returns the input word shifted right by fourteen bits and multiplied by two.
- `update_checksum_block_a`: Updates four consecutive integer fields in a structure with a chained addition using constant offsets.
- `update_checksum_block_b`: Updates four consecutive integer fields in a structure with a chained addition using constant offsets, operating on a different set of fields than the previous routine.
- `update_track_state_and_swap_buffers`: The routine updates a set of fields in a structure pointed to by a register, swaps a pair of 32-bit values and copies several 32-bit and 16-bit values from one area of the structure to another, and maintains a small rotating index and a circular buffer of short values.
- `sync_state_fields`: The routine compares several fields from a base structure with shadow copies and updates the shadows when they differ, invoking another routine after each update.
- `sync_state_fields_2`: The routine mirrors the field comparison and shadow update pattern of the first routine but uses a different set of offsets within the base structure.
- `invoke_copy_structures_loop`: The routine calls two other routines and then returns.
- `copy_structures_loop`: Copies a fixed number of words from one structure to another using a loop that advances both pointers, copying several offsets per iteration.
- `copy_bytes_with_fallback`: The routine calls another routine and then copies eight bytes from a table at a fixed offset into a destination buffer, comparing each byte and replacing it with a value from another routine when they differ.
- `invoke_copy_table_entries`: The routine calls three other routines and then returns.
- `copy_table_entries_by_index`: Copies a set of bytes from a table indexed by an input word into a destination structure at multiple fixed offsets.
- `clamp_scaled_value`: Scales an input integer by shifting, adds a constant, and clamps the result between zero and a maximum, returning the clamped short.
- `copy_struct_fields`: The routine copies a fixed set of fields from a source structure selected by an index and a flag into a destination pointed to by a register and an offset.
- `copy_table_entries`: The routine copies four 32-bit words from a source table entry selected by an index into four spaced locations of a destination structure.
- `copy_struct_fields_2`: The routine copies the same fixed set of fields as in R1 from a source structure selected by an index into a destination pointed to by a register and an offset.
- `store_waveform_parameters`: The routine calls another routine and then stores a short value plus a constant and a word value into two fixed offsets of a structure pointed to by a pointer.
- `store_sprite_frame_value`: The routine calls another routine, captures its returned word, and stores it into a fixed offset of a structure pointed to by a pointer.
- `store_waveform_parameters_2`: The routine writes a short value derived from a register plus a constant and a word from another register into two consecutive fields of a structure pointed to by a pointer.
- `store_returned_value`: The routine calls another routine and stores the returned 16-bit value into a global variable.
- `compute_waveform_samples`: The routine iterates over a sequence of entries indexed by an input count and a base pointer derived from another routine's data.
- `build_sprite_frame_buffer`: The routine builds a small sprite frame buffer in a caller-supplied output area and prepares a set of per-frame parameters in a work structure.
- `select_pointers_and_dispatch`: The routine selects four pointers based on a short flag in a structure, computes an index from structure fields, writes pointer words into a structure, and then performs an indirect call through a table using the computed index before returning.
- `select_palette_and_dispatch`: The routine selects among several global pointers based on a short flag, computes an index from structure fields, writes pointer words into a buffer and then dispatches indirectly to another routine chosen by the computed index.
- `expand_palette_table_2`: The routine writes a byte from a register to the output buffer at a series of offsets and then copies three 16-bit words from a palette table into the output buffer at the following offsets, repeating the pattern for many entries.
- `expand_palette_table`: The routine copies a byte from a source table to the output buffer and then writes three 16-bit words from a palette table into the output buffer at offsets 6, 10 and 14, using an index derived from the next byte of the source table shifted left by three.
- `store_two_words_from_call`: The routine calls another routine and stores the returned word and an extra output word into two offsets of a structure pointed to by a register.
- `store_track_vertex_offsets`: The routine calls another routine and stores the returned 16-bit value and an extra 16-bit output into two fields of a structure pointed to by a register.
- `generate_track_vertices`: The routine uses an input value to set up a count and a base pointer, then iterates over a table of short offsets to accumulate X and Y deltas and writes a series of vertex records into a destination buffer.
- `fill_track_gap_segments_2`: The routine walks a short-indexed table of track segments and writes a packed representation into a destination buffer.
- `fill_track_gap_segments`: The routine walks a short-indexed table of track segments and writes a packed representation into a destination buffer.
- `call_other_routine_5`: The routine calls another routine once and then returns.
- `recursive_self_call_2`: The routine immediately calls another routine, which is itself.
- `clamp_and_split_range`: The routine copies a short from an offset in a structure pointed to by a register into another offset, then computes a signed delta from two shorts in a global table and the copied value.
- `fill_sprite_descriptor_table`: The routine reads several short fields from a global structure and writes a series of 32-bit and 16-bit values into a table pointed to by a pointer argument.
- `populate_sprite_table`: The routine fills a table pointed to by a pointer argument using values read from a global structure.
- `initialize_track_data_tables`: The routine copies a pointer from a global variable into another global variable and then repeatedly calls another routine, storing the returned word through a pointer and clearing a word in the pointed-to structure.
- `loop_call_other_routine`: The routine calls another routine, then repeatedly calls another routine and loops while a masked value differs from an uninitialized short.
- `self_recursive_call_2`: The routine immediately calls itself and then returns.
- `recursive_self_call_4`: The routine contains a single unconditional call to itself, forming an infinite recursion with no other operations.
- `self_recursive_call_3`: The routine immediately calls itself once and then returns without performing any other operation.
- `recursive_self_call_5`: The routine contains only a single call to itself and then returns.
- `call_two_routines_2`: The routine calls two other routines in sequence and then returns.
- `call_three_routines`: The routine calls three other routines in sequence and then returns.
- `call_four_routines_2`: The routine calls four other routines in sequence and then returns.
- `call_five_routines`: The routine calls five other routines in sequence and then returns.
- `copy_table_to_buffer`: The routine copies a pointer from one field of a structure to another field, then repeatedly calls another routine and stores the returned word into a buffer, and then copies a fixed number of words from a table referenced by a pointer to a second buffer, updating a pointer field with the end address.
- `loop_until_low_nibble_match`: The routine calls another routine, then repeatedly calls another routine until the low nibble of the returned value is not less than a short value held in a register.
- `self_recursive_call_4`: The routine calls another routine, then repeatedly calls another routine until the low nibble of the returned value is not less than a short.
- `empty_routine_6`: The routine contains no instructions and returns immediately.
- `fill_structure_with_offsets_2`: The routine is empty.
- `call_other_routine_five_times`: The routine calls another routine five times in a row and then returns.
- `call_other_routines`: The routine calls seven other routines in sequence and then returns.
- `compute_scaled_index`: The routine uses an input value and data from a global variable and two unmodified registers to compute a scaled index and a remainder, then builds a 32-bit result from those values with conditional adjustment and returns it.
- `fill_structure_with_offsets`: The routine reads several values from a global variable and input registers, computes two intermediate values, and conditionally writes multiple words and long words through a pointer, filling a structure with repeated offset calculations.
- `chain_calls_2`: The routine performs a sequence of six calls to other routines and then returns.
- `empty_routine_24`: The routine contains 37 instructions that perform no observable work and return immediately.
- `populate_table_entries_2`: The routine computes several intermediate values from input registers and a global variable, then writes a series of words and longs through a pointer argument to fill a table.
- `populate_table_entries`: The routine combines input registers and values from a global variable to compute several intermediate results and writes a series of words and long words through a pointer, populating a table with repeated fields.
- `decode_missiles`: This routine reads missile data from a table at a fixed offset and writes it to memory locations accessed through a pointer stored in a global variable.
- `clear_stack_frame_and_return`: The routine has 27 instructions that set up a stack frame, clear a small number of bytes on the stack, and then unconditionally return.
- `decode_missiles_2`: The routine reads multiple shorts from two structures referenced by registers and a global variable, performs nested conditional checks, and returns a short value derived from those checks, sometimes invoking another routine and using its result.
- `copy_digit_tables`: The routine reads a short from memory, clamps it, selects two base addresses based on a comparison, then copies 26 blocks of bytes from one table to another using two loops, effectively duplicating digit tables.
- `call_other_routines_2`: The routine calls four other routines in sequence and then returns.
- `decode_and_scatter_track_data`: The routine copies up to ten bytes from a source pointer into a global buffer, building a ten-byte string that starts with 'W' 'l' 'y' and ' ' 'a' 'h' 'e' followed by spaces, then uses the first bytes of that buffer as indices to read from a table and scatter bytes into a work area with XOR and complement operations.
- `initialize_structure_from_index`: The routine obtains an index from another routine and, if the index is valid, reads values from a table and a global variable and writes a series of computed offsets and constants into a structure pointed to by a register; otherwise it clears the first two words of the structure.
- `fill_structure_with_offsets_3`: The routine reads an index from another routine and, if valid, loads values from a table and a global variable, then writes a series of computed shorts and ints through a pointer, using offsets derived from the loaded values.
- `process_tile_block`: Iterates over a fixed number of entries in a table pointed to by a register-derived pointer.
- `initialize_track_node_state_2`: The routine initializes a per-track state structure pointed to by a register argument, copying words from a table pointer argument and several register-derived values into the structure and setting up counters and flags.
- `initialize_track_node_state_3`: The routine initializes a per-track state structure pointed to by a register argument, copying words from a table derived from a global variable and setting several fields in the structure.
- `cycle_state_value`: The routine checks a flag in a structure and a state short, then calls another routine and updates the state short to the next value in a fixed sequence, returning after the update.
- `loop_until_counter_zero`: The routine repeatedly calls another routine while a short field in a structure remains greater than or equal to zero, then returns.
- `conditionally_call_and_store_word`: The routine tests a signed short read from a computed offset of a register-based pointer and conditionally calls another routine, then stores the constant word value to an offset from the same pointer before returning the concatenation of the two input registers.
- `check_signed_word_and_store`: The routine tests a signed word computed from a global variable and a table indexed via a register-based pointer; if the word is negative it calls another routine.
- `decrement_and_load_word`: Repeatedly decrements a counter stored in a global variable and loads a word from a pointer that is moved backwards each iteration.
- `fetch_and_normalize_entry`: Decrements a counter in a global variable and, while the counter remains valid, copies words from an input pointer into several global variables and computes a difference against a global base value.
- `select_and_scale_index`: Decrements a counter in a global variable and uses it to index a table via a register-derived base.
- `find_near_entry`: Decrements a counter in a global variable and, while valid, reads words from an input pointer into global variables and computes a difference against a global base.
- `sort_four_pairs_by_first`: The routine reads four pairs of values from a global structure, performs a series of pairwise swaps to order the pairs by their first component, and writes the four sorted pairs back to another area of the same structure.
- `copy_word_and_branch`: The routine copies a word from one offset of a structure pointed to by a register to another offset and stores the high byte of the source word to a third offset.
- `store_indexed_bytes_and_branch`: Stores a word from an input register into two fields of a global structure and stores its high byte into another field.
- `copy_word_to_table_entry`: The routine copies a 16-bit value from an offset in a structure to two other offsets, storing the low and high bytes separately.
- `store_words_and_conditional_call`: The routine stores a word from an input register into two offsets of a register-based structure, then conditionally calls another routine based on a signed short from a table indexed by an input word, and returns without further action.
- `copy_short_and_conditional_update`: Copies a short from one field of a global structure to another field.
- `conditional_setup_then_dispatch`: If a table entry indexed by a short field in a global structure is not negative, it copies a word, clears a word, sets a word to a constant, calls another routine, then clears the word.
- `reset_track_node_state`: The routine checks a short value derived from a global variable and, if the condition holds, copies a word from one offset of a global structure to another, clears two word fields and sets a status word, calls another routine, then clears the status word.
- `copy_short_and_conditional_update_2`: The routine copies a short from one offset of a global structure to another offset, then checks a short from a table indexed by the copied value.
- `decode_word_into_fields_and_branch`: Loads a word from a global structure, stores its low byte masked into a field, masks the field, stores low bits and high byte parts into other fields, copies a short field, and conditionally calls another routine based on a table lookup indexed by the copied short.
- `copy_track_node_flags`: The routine copies a byte from a global variable to a field in a structure, masks the destination word, stores the low two bits of the source word into another field, stores the high byte of the source word into another field, copies a short from one field to another, and conditionally calls another routine based on a table lookup using the copied short as an index.
- `check_two_state_words_noop`: Reads two 16-bit fields at offsets from the A3 base register and returns early if the first is nonzero or the second is zero; every path ends in a return with no visible side effect.
- `update_track_node_state_3`: The routine copies fields from one area of a global structure to another, selects a pointer from the structure based on flags, optionally calls another routine, and updates per-track timing and interpolation values using table lookups and arithmetic on input registers.
- `update_track_node_state_4`: The routine copies fields from one area of a global structure to another, clears a flag word and conditionally sets a status word based on a short flag and input values.
- `process_track_node_state_2`: The routine copies fields from a global structure into another area of the same structure, selects a table pointer based on a short field in the structure and calls another routine to update a per-track state.
- `copy_track_state_and_dispatch`: The routine copies several fields from one area of a global structure into another area of the same structure and then selects a table pointer based on a short field in the structure.
- `update_track_node_state`: The routine updates a per-track state structure pointed to by a pointer argument, using data from a table pointer argument and several register values.
- `initialize_track_node_state`: The routine stores two pointers from the argument into a global structure and copies several fields from a table entry pointed to by the first pointer into that structure, computing offsets and sizes from register values.
- `update_track_node_state_2`: The routine stores two pointers from a pointer argument into a global structure, extracts several fields from a table pointed to by the first argument, computes a set of derived offsets and sizes, writes those values into the global structure, and then conditionally writes a small fixed-size record through a pointer argument using the computed values.
- `process_track_node_state`: The routine updates a per-track state structure pointed to by a pointer argument, using data from a table pointer argument and several registers.
- `fill_table_with_computed_offsets_2`: The routine clamps four input values against zero and a global variable, then performs conditional checks on the clamped values and calls other routines accordingly before returning.
- `fill_table_with_computed_offsets`: The routine writes constants and computed values into a table pointed to by a register, using an input word and several fields from a structure to calculate base offsets.
- `fill_collision_box_table`: The routine writes a series of collision box descriptors into a table pointed to by a pointer argument.
- `decode_and_scatter_data`: The routine builds a small ASCII header in a buffer, then copies and XORs bytes from a table indexed by a byte from another buffer into a destination area using a fixed stride, and finally performs an in-place bit rotation on 16-bit words in the destination region.
- `decode_and_scatter_data_2`: The routine builds a small ASCII header in a buffer and then copies and XOR-scatters bytes from a table addressed through a pointer into a work area, using offsets derived from input bytes.
- `decode_and_scatter_track_data_2`: The routine builds a ten-byte string that starts with 'W' 'l' 'y' and a space in a global buffer, then copies up to ten bytes from a source pointer into that buffer.
- `initialize_struct_fields_2`: The routine is nine instructions long and is entered via an absolute move.
- `initialize_struct_fields`: The routine writes a pointer from a source pointer to an offset in a structure, sets a global variable, stores a constant word at an offset in the structure, and copies a word from the source pointer to another offset in the structure.
- `copy_dispatch_entry_2_2_2`: The routine is seven instructions long and is entered because its address is taken and it is executed.
- `copy_dispatch_entry_2_2`: The routine copies a pointer from a source structure into a dispatch table entry, writes a constant to a global variable, and stores two 16-bit values from the source structure into the dispatch table entry.
- `copy_dispatch_entry_2`: The routine copies a pointer from a source structure into a dispatch table entry, writes a constant to a global variable, and stores two 16-bit values from the source structure into the entry.
- `copy_dispatch_entry`: The routine copies a pointer from a source structure into a dispatch table entry, writes a constant to a global variable, and stores two 16-bit values from the source structure into the dispatch table entry.
- `initialize_dispatch_struct_3_2_2_2`: The routine is eleven instructions long and is entered because its address is taken.
- `initialize_dispatch_struct_3_2_2`: The routine copies two words from a pointer into two offsets of a structure pointed to by a register, writes a constant to a global variable, writes a constant word to an offset of the structure, and copies a word from the pointer into another offset of the structure.
- `initialize_dispatch_struct_3_2`: The routine copies two words from a pointer into two offsets of a structure pointed to by a register, writes a constant to a global variable, stores a constant word to an offset, and copies a word from the pointer into another offset.
- `initialize_dispatch_struct_3`: The routine copies two words from a pointer into two offsets of a structure pointed to by a register, writes a constant to a global variable, stores a constant word to an offset of the structure, and copies a word from the pointer into another offset of the structure.
- `initialize_dispatch_struct_2_2_2`: The routine is eight instructions long and is entered because its address is taken.
- `initialize_dispatch_struct_2_2`: The routine stores a long from a pointer into a structure field at offset 84, stores a constant word at offset 64, writes a constant address to a global variable, and copies a word from the pointer to a structure field at offset 88.
- `initialize_dispatch_struct_2`: The routine stores a long from a pointer into a structure field at offset 84, stores a constant word at offset 64, writes a constant address into a global variable, stores a constant word at offset 156, and copies a word from the pointer into offset 88 of the structure.
- `initialize_dispatch_struct`: The routine stores a constant word at offset 64 of a structure pointed to by a register, copies a long from a pointer to offset 84 of the structure, writes a constant address to a global variable, stores a constant word at offset 156 of the structure, and copies a word from the pointer to offset 88 of the structure.
- `dispatch_struct_copy_variants`: The routine is 45 instructions long and is entered via an absolute move.
- `copy_struct_fields_and_set_constants_2`: The routine copies a word and three longs from a pointer to fixed offsets from a register-based pointer, writes a constant word to two of those offsets, stores a pointer field from the source into another offset, and writes a constant to a global variable.
- `copy_struct_fields_and_set_constants`: The routine copies a word and three longs from a pointer to fixed offsets from a register-based pointer, writes a constant word to two of those offsets and stores a constant word to a global variable.
- `copy_fields_to_struct_and_set_constants_5`: The routine copies a word and three longs from a pointer to fixed offsets from a register-based pointer, writes a constant word to two of those offsets and stores a constant to a global variable.
- `copy_fields_to_struct_and_set_constants_3`: The routine copies a word and three longs from a pointer to fixed offsets from a register-based pointer, writes a constant word to two of those offsets, and stores a constant to a global variable.
- `copy_fields_to_struct_and_set_constants`: The routine copies a word and three longs from a pointer to fixed offsets from a register-based pointer, writes a constant word to two of those offsets, stores a word from the pointer to another offset, and writes a constant to a global variable.
- `copy_fields_to_struct_3`: The routine copies a word and three longs from a pointer to fixed offsets from a register-based pointer and writes a constant word to two of those offsets.
- `copy_fields_to_struct_2_2`: The routine copies a word and three longs from a pointer to fixed offsets from a register-based pointer and writes a constant word to two of those offsets.
- `copy_fields_to_struct_2`: The routine copies a word and three longs from a pointer to fixed offsets from a register-based pointer and writes a constant word to two of them.
- `copy_fields_to_struct`: The routine copies a word and three longs from a pointer to fixed offsets from a register-based pointer and writes a constant word to two offsets, then stores a constant to a global variable.
- `copy_fields_to_struct_5`: The routine copies four consecutive longs from a pointer into offsets 72, 76, 80 and 84 of a structure pointed to by a register, writes a constant address to a global variable, stores a constant word at offset 156 of the structure, and copies a word from the pointer at index 4 to offset 88 of the structure.
- `copy_four_longs_and_word_to_struct`: The routine copies four longs from a pointer into consecutive offsets of a structure pointed to by a register, writes a constant word to an offset of the structure, and copies a word from the pointer into another offset of the structure.
- `copy_struct_fields_swapped_2_2_2_2`: The routine copies four longs from a pointer into consecutive offsets of a structure pointed to by a register, writes a constant word to an offset of the structure, copies a word from the pointer into another offset of the structure, and stores a constant to a global variable.
- `copy_struct_fields_swapped_2_2_2`: The routine copies a word from a pointer into an offset of a structure pointed to by a register, copies a long from the pointer into another offset of the structure, writes a constant word to a further offset, copies a word from the pointer into another offset, and writes a constant to a global variable.
- `copy_struct_fields_swapped_2_2`: The routine copies a word from a pointer into a fixed offset of a structure pointed to by a register and copies a long from the pointer into another fixed offset of the same structure.
- `copy_struct_fields_swapped_2`: The routine copies a word from a pointer into a fixed offset of a structure pointed to by a register and copies a long from the pointer into another fixed offset of the structure.
- `copy_struct_fields_swapped`: The routine copies a word and two longs from a pointer to fixed offsets from a register-based pointer with the long destinations swapped relative to their source order, writes a constant to one word field of the structure, copies another word from the source pointer, and stores a constant into a global variable.
- `copy_struct_fields_with_swapped_offsets_2`: The routine copies a word and two longs from a pointer to fixed offsets from a register-based pointer with the long destinations swapped relative to their source order, writes a constant word to an offset, copies another word from the source pointer, and stores a constant value into a global variable.
- `copy_struct_fields_with_swapped_offsets`: The routine copies a word and two longs from a pointer to fixed offsets from a register-based pointer with the long destinations swapped relative to the source order, writes a constant word to two offsets, stores a pointer field from the source into another offset, and writes a constant to a global variable.
- `initialize_object_from_source`: The routine writes a constant word to an offset from a register-based pointer, copies three longs from a source pointer to three offsets from the same pointer, stores a constant to a global variable, writes another constant word to an offset from the pointer, and copies a word from the source pointer to an offset from the pointer.
- `store_constant_and_copy_fields_3`: The routine stores a constant word to an offset from a register-based pointer, copies three longs from a pointer to three offsets from the same register-based pointer, writes a constant word to another offset and stores a constant to a global variable.
- `store_constant_and_copy_fields_2`: The routine stores a constant word to an offset from a register-based pointer, copies three longs from a pointer to three offsets from the same pointer, writes a constant word to another offset, copies a word from the pointer, and stores a constant to a global variable.
- `store_constant_and_copy_fields`: The routine stores a constant word to an offset from a register-based pointer, copies three longs from a pointer to three offsets from the same pointer, writes a constant to a global variable, stores a constant word to another offset, and copies a word from the pointer to a further offset.
- `store_struct_fields_from_pointer_3`: The routine stores a constant word to an offset from a register-based pointer, copies three longs from a pointer to three offsets from the same base, writes a constant word to another offset and copies a word from the source pointer to an offset.
- `store_struct_fields_from_pointer_2_2_2`: The routine stores a constant word to an offset from a register-based pointer, copies three longs from a pointer to three offsets from the same pointer, writes a constant word to another offset, copies a word from the source pointer to an offset, and stores a constant value into a global variable.
- `store_struct_fields_from_pointer_2_2`: The routine stores a constant word to an offset from a register-based pointer, copies three longs from a pointer to three offsets from the structure and writes a constant word to another field.
- `store_struct_fields_from_pointer_2`: The routine stores a constant word to an offset from a register-based pointer, copies three longs from a pointer to three offsets from the same pointer, writes a constant word to another offset and copies a word from the pointer, then stores a constant to a global variable.
- `store_struct_fields_from_pointer`: The routine writes a constant word to an offset from a register-based pointer, copies three longs from a pointer to offsets from the same pointer, stores a constant to a global variable, writes a constant word to another offset and copies a word from the pointer to an offset.
- `call_copy_and_copy_routines`: The routine contains fourteen instructions, is entered via an absolute move, is called by copy_fields_to_struct_4 and calls copy_fields_to_struct_and_set_constants_4 and copy_fields_to_struct_4.
- `copy_fields_to_struct_and_set_constants_4`: The routine copies four consecutive longs from a source pointer into fixed offsets of a structure pointed to by a register, writes a constant word to two of those offsets and copies a word from the source pointer into another offset.
- `copy_fields_to_struct_and_set_constants_2`: The routine copies four longs from a pointer to fixed offsets from a register-based pointer, writes a constant word to two of those offsets, stores a word from the pointer to another offset, and writes a constant to a global variable.
- `copy_fields_to_struct_4`: The routine copies several fields from a source pointer into a structure pointed to by a register and writes constant words into two fields.
- `fill_struct_from_pointer_2_2`: The routine copies several fields from a source pointer into a structure pointed to by a register and writes constant words into two fields of that structure.
- `fill_struct_from_pointer_2`: The routine copies several fields from a source pointer into a structure pointed to by a register and writes constant words into two fields of the structure.
- `fill_struct_from_pointer`: The routine copies several fields from a source pointer into a structure pointed to by a register, writes constant words into two fields of the structure, stores a constant pointer into a global variable, and returns.
- `call_fill_struct_fields`: The routine has four instructions, is address taken and called by two unnamed routines, and calls a routine that stores an input word to a pointer and writes sums of input words.
- `patch_globals_and_dispatch`: The routine touches hardware registers CIAA_CRA, COP1LCH, DMACON and custom+0, writes to DMACON with bit 15 set to enable DMA channels, stores values to a register-based area, conditionally calls other routines, and stores a pointer to a global variable.
- `wait_for_bit_toggle`: The routine spins on a word at an offset from an uninitialized address until bit zero is set, then spins until that bit is cleared, and returns zero.
- `iterate_table_and_dispatch`: The routine walks a word table in a global variable until a zero terminator, builds a value from an input register and the table word, stores an address pointer into a global table and optionally calls another routine with the built value, then updates a global pointer and repeats.
- `initialize_table_entries_2`: The routine clears a short counter in a register-based area, sets a pointer to a start label, then loops while a word from a global table is non-zero, copying a pointer into a global table entry, calling another routine, advancing a pointer and incrementing the counter.
- `call_another_routine_3`: The routine calls another routine once and returns.
- `call_two_routines`: The routine calls another routine twice in succession and then returns.
- `copy_memory_block`: The routine copies a fixed number of long words from one absolute address to another by repeatedly storing a word from a source pointer to a destination pointer and advancing both pointers.
- `patch_globals_and_call`: The routine stores a routine address into a global variable, calls another routine, stores a constant into another global variable, then calls another routine twice before returning.
- `self_recursive_call`: The routine consists of a single call to another routine and then returns.
- `invoke_external_function`: The routine calls an external function at a fixed address and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_7`: The routine contains no instructions and returns immediately.
- `empty_routine_8`: The routine contains no instructions and performs no operations.
- `empty_routine_9`: The routine contains no instructions and returns immediately.
- `empty_routine_10`: The routine contains no instructions and returns immediately.
- `empty_routine_placeholder`: The routine contains no instructions and returns immediately without modifying memory or registers.
- `empty_routine_11`: The routine contains no instructions and returns immediately.
- `empty_routine_12`: The routine contains no instructions and returns immediately.
