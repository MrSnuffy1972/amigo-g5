# Game 740 (introgash (19xx)(a. margetts)(sw)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00740). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 25 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 35 of 35 named (0 author symbols), 35 of 35 described (14 by rule, 19 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 334 of 334 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1021 of 1021 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 419336 |
| code | 3386 |
| data | 207367 |
| padding | 6391 |
| unused | 264704 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Introgash (19xx)(A. Margetts)(SW).adf:Introgash | 17 | 782 | 22 |
| Introgash (19xx)(A. Margetts)(SW).adf:Shareware | 2 | 228 | 11 |
| Introgash (19xx)(A. Margetts)(SW).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Introgash (19xx)(A. Margetts)(SW).adf:Introgash

- `initialize_system_and_load_resources`: Initializes system libraries, checks for specific resource availability, and performs memory allocation for a data structure.
- `manage_resource_lifecycle`: Checks for the existence of specific resources and, if present, performs memory allocation, initializes a table of pointers, and iterates through a list of structures to perform cleanup or status updates.
- `initialize_resources_and_cleanup_memory`: Copies two null-terminated strings into local stack buffers, checks for a specific four-byte magic identifier in a structure, and calls an indirect routine.
- `validate_and_initialize_subsystem`: Checks for the existence of two specific resources by calling other routines, and if both are present, enters a loop to increment a counter until it wraps around.
- `call_and_branch`: Calls a routine at a specific address, sets register d7 to negative one, and then branches to another location.
- `branch_on_word_and_call`: The routine tests a word through a pointer against a constant and calls one of two indirect routines through a global variable depending on the result, then returns a concatenated value from two inputs.
- `call_indirect_via_global`: The routine calls an indirect routine through a global variable.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_copy_mem`: Library stub: loads its arguments and the exec library base, calls CopyMem and returns its result to the caller.
- `walk_byte_table_and_update_pointers`: The routine walks a byte table until a zero byte is found, and for each byte it either advances a pointer by a fixed amount or computes a new pointer from the byte value and adds an offset from a base pointer, storing the result back into the pointer.
- `clear_word_and_call_indirect`: The routine clears a word through a pointer and calls an indirect routine through a pointer in memory.
- `process_descriptor_and_chain`: The routine processes a descriptor pointed to by an argument, writes fields into a table using an index derived from a short value, updates a count and a flag byte, chains pointers in a loop based on a count, calls another routine, and adjusts two counters through pointers.
- `reallocate_and_copy_memory_block`: Iterates through a list of memory blocks to calculate a total size, then calls a routine to free memory and another to allocate a new block of the calculated size.
- `clear_d1_and_branch`: Clears register d1 and branches to an earlier instruction.
- `call_and_branch_short`: Calls a routine at a specific address and then branches to another location.
- `check_table_entry_and_prepare`: The routine examines a table entry pointed to by a global variable, conditionally calls an indirect routine through a global variable, and prepares output registers with values derived from the table and other routines.
- `reset_counters_and_call`: The routine calls another routine, clears two words through a pointer, and returns an input value.
- `signal_task_on_switcher`: The routine reads a global variable, calls another routine through a table and, if the result is non-zero, calls a second routine through the same table.
- `manipulate_pointer_and_data`: The routine adds a value in d3 to d1, sets a0 and a1 to the result, and then performs several memory reads and an eor operation using values from a pointer in a0.
- `decompress_bitstream`: The routine reads a packed bitstream using a bit reader refill and bit read helper, decodes control bits to choose copy or literal handling, and writes bytes into a destination buffer pointed to by a register while tracking a count and a repeat length.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Introgash (19xx)(A. Margetts)(SW).adf:Shareware

- `initialize_library_headers`: Verifies the presence of required libraries by checking for specific magic strings in the library headers and parses a resource table.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Introgash (19xx)(A. Margetts)(SW).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
