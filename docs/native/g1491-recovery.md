# Game 1491 (deuteros - the next millennium (1991)(activision)(m3)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01491). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1818176 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 21 raw-word lines |
| Functions named and described | pass, 129 of 129 named (0 author symbols), 129 of 129 described (14 by rule, 108 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 753 of 753 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 108032 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2545 of 2545 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 20434 |
| code | 15962 |
| data | 1721677 |
| padding | 199 |
| unused | 59904 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf:bootblock | 1 | 45 | 2 |
| logged disk load from Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 302 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2198 | 114 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf bytes 13824-14336 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf bytes 23552-25088 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf bytes 28160-29184 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf bytes 497664-526336 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf bytes 566272-636928 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf bytes 23552-29184 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1682 non-empty blocks: matched against track-loaded segments
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1717 non-empty blocks: matched against track-loaded segments
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: the boot block reads 16896 bytes from disk offset 51200 to an address (found statically); recovered as a unit
- runtime segment an address: 246 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 466 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: bytes 13824-14336 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: bytes 23552-25088 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: bytes 28160-29184 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: bytes 497664-526336 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: bytes 566272-636928 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: bytes 23552-29184 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: 6 logged loads cover 119296 bytes of the image
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: 13 track-loaded blocks found in recovered runtime segments and classified through them
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 2-22 (bytes 1024-11776) outside the filesystem hold data, not code (opcode density 0.034, entropy 5.01)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 24-24 (bytes 12288-12800) outside the filesystem hold data, not code (opcode density 0.016, entropy 0.96)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 33-44 (bytes 16896-23040) outside the filesystem hold data, not code (opcode density 0.058, entropy 5.57)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 50-50 (bytes 25600-26112) outside the filesystem hold data, not code (opcode density 0.047, entropy 5.63)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 52-53 (bytes 26624-27648) outside the filesystem hold data, not code (opcode density 0.047, entropy 4.34)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 61-61 (bytes 31232-31744) outside the filesystem hold data, not code (opcode density 0.039, entropy 4.12)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 65-65 (bytes 33280-33792) outside the filesystem hold data, not code (opcode density 0.023, entropy 4.30)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 68-68 (bytes 34816-35328) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.57)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 77-87 (bytes 39424-45056) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.85)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 89-99 (bytes 45568-51200) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.54)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 143-170 (bytes 73216-87552) outside the filesystem hold data, not code (opcode density 0.026, entropy 6.78)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 172-194 (bytes 88064-99840) outside the filesystem hold data, not code (opcode density 0.044, entropy 6.24)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 198-360 (bytes 101376-184832) outside the filesystem hold data, not code (opcode density 0.002, entropy 7.09)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 363-873 (bytes 185856-447488) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.98)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 875-880 (bytes 448000-451072) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.23)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 883-884 (bytes 452096-453120) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.41)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 891-923 (bytes 456192-473088) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.93)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 931-931 (bytes 476672-477184) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.59)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 934-937 (bytes 478208-480256) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.47)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 951-970 (bytes 486912-497152) outside the filesystem hold data, not code (opcode density 0.035, entropy 5.48)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 1034-1104 (bytes 529408-565760) outside the filesystem hold data, not code (opcode density 0.038, entropy 6.59)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 1245-1745 (bytes 637440-893952) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.61)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf: blocks 1749-1759 (bytes 895488-901120) outside the filesystem hold data, not code (opcode density 0.055, entropy 5.54)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: 1 logged loads cover 5632 bytes of the image
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: 3 track-loaded blocks found in recovered runtime segments and classified through them
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: blocks 2-22 (bytes 1024-11776) outside the filesystem hold data, not code (opcode density 0.033, entropy 5.00)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: blocks 24-24 (bytes 12288-12800) outside the filesystem hold data, not code (opcode density 0.016, entropy 0.96)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: blocks 27-27 (bytes 13824-14336) outside the filesystem hold data, not code (opcode density 0.047, entropy 4.61)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: blocks 33-44 (bytes 16896-23040) outside the filesystem hold data, not code (opcode density 0.056, entropy 5.57)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: blocks 60-111 (bytes 30720-57344) outside the filesystem hold data, not code (opcode density 0.024, entropy 6.29)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: blocks 121-148 (bytes 61952-76288) outside the filesystem hold data, not code (opcode density 0.030, entropy 6.13)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: blocks 150-182 (bytes 76800-93696) outside the filesystem hold data, not code (opcode density 0.055, entropy 4.97)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: blocks 186-262 (bytes 95232-134656) outside the filesystem hold data, not code (opcode density 0.017, entropy 5.34)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: blocks 277-306 (bytes 141824-157184) outside the filesystem hold data, not code (opcode density 0.043, entropy 6.12)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: blocks 308-441 (bytes 157696-226304) outside the filesystem hold data, not code (opcode density 0.023, entropy 5.95)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: blocks 451-1488 (bytes 230912-762368) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.47)
- deuteros - the next millennium (1991)(activision)(m3)[a](disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 2 of 2).adf: blocks 1496-1759 (bytes 765952-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.01)
- 6 never-loaded region(s), 108032 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 6144 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `call_do_i_o` (runtime segment an address): Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller. Kind: track/disk integrity check. Pass path: execution continues to the next instruction (the code following the call).
- `initialize_system_and_io` (runtime segment an address): Performs multiple system initializations, manages memory buffers, and executes I/O operations using the exec library. Kind: disk/track integrity check. Pass path: The routine proceeds to initialize I/O structures and calls subsequent functions (likely the actual data loading or game entry) if the initial checks do not trigger a failure state..
- `initialize_and_copy_data` (runtime segment an address): Performs I/O operations to initialize data structures and then copies a block of data from a global variable into four separate destination tables using a loop. Kind: track/sector loading with specific parameter setup. Pass path: the loop that copies data from the loaded buffer into memory.
- `issue_doio_with_command_nine` (runtime segment an address): Sets fields in a global structure to request command nine with a length of one and a zero flag, then calls the I/O routine. Kind: disk command/track check. Pass path: the function returns normally (the call to the device driver completes without triggering a crash or jump to a failure routine).
- `issue_doio_with_command_8004` (runtime segment an address): Sets a command field in a global structure to a specific value and a flag byte to zero before calling the I/O routine. Kind: trackdisk.device I/O command (likely a custom/direct track read). Pass path: the code following the indirect function call (the return from the device driver).
- `issue_doio_with_command_five` (runtime segment an address): Sets the command field of a global structure to five and a flag byte to zero, then calls the I/O routine. Kind: track/sector validation check. Pass path: the branch where the return value (likely in D0) is compared against a specific expected value (e.g., the 'cmp.w #an address, d0' logic suggests a status check where a specific non-zero value indicates success/validity).
- `issue_doio_and_copy_words` (runtime segment an address): Fills fields of a global structure for an I/O request, calls the I/O routine, and if a status byte is zero copies a number of words determined by the input word into a destination pointer. Kind: disk/track integrity check and data copy. Pass path: the branch where the loop executes (the 'do' block) to copy data into memory.

## Functions


### Deuteros - The Next Millennium (1991)(Activision)(M3)[a](Disk 1 of 2).adf:bootblock

- `boot_issue_io_with_delay`: Boot block entry that prepares I/O request structures, calls exec's DoIO through a library offset, waits through a fixed busy loop, and sends a second request.
- `send_io_request_and_return`: Fills in fields of an I/O request structure, including a callback pointer, then calls exec's DoIO and returns its result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_and_io_2`: Sets up function pointers in a global table and populates data structures with specific configuration values.
- `open_trackdisk_and_create_port`: Calls exec to open trackdisk.device, fills in a message port structure with a name and flags, adds the port, finds a task, and if the lookup fails it loops forever; otherwise it sets a field to all ones and clears a byte.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `reset_request_and_do_io`: Clears the length field of the global IO request, sets its command word to nine and its flag byte to zero, then invokes exec DoIO on that request.
- `initialize_system_and_io`: Performs multiple system initializations, manages memory buffers, and executes I/O operations using the exec library.
- `return_constant_pair`: Returns a fixed pair of constant values in the data registers without touching memory or calling anything.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d1.
- `initialize_data_structures`: Sets up two distinct sets of data structures by zeroing out memory blocks and populating them with specific values.
- `initialize_memory_buffers`: Initializes and clears two distinct blocks of memory by setting up structure pointers, zeroing out specific byte ranges, and defining memory offsets for sequential data segments.
- `initialize_memory_buffers_2`: Initializes two distinct sets of data structures by zeroing out memory blocks and setting up a series of pointers and offset values.
- `setup_bitplanes_and_load_libraries`: Opens the graphics library, checks available memory, and initializes two sets of display data structures.
- `initialize_and_copy_data`: Performs I/O operations to initialize data structures and then copies a block of data from a global variable into four separate destination tables using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_bitstream_decompressor`: This routine calls a bitstream decompression function and returns the concatenated results of the registers used during the call.
- `decompress_bitstream_data`: Decompresses a bit-packed stream into a destination buffer using a variable-length coding scheme.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `initialize_system_state`: Sets multiple global variables and pointers to specific values, then executes a series of subroutine calls.
- `open_graphics_library`: Attempts to open a library and stores the resulting pointer in a global variable, entering an infinite loop if the operation fails.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_function_repeatedly`: Calls a library function four times using a pointer retrieved from a global variable.
- `clear_global_word`: Calls a library function and then stores zero into a global word.
- `advance_index_from_table`: Adds a table entry indexed by the current global value to that value plus a constant, storing the result back into the global variable.
- `tiny_ori_3`: Small routine of 1 instruction that merges 136 into d2.
- `copy_global_word`: Copies the value from one global word to another.
- `update_global_register_state`: Copies the contents of a data register into a global variable.
- `assign_routine_pointer`: Stores the address of another routine into a global variable.
- `store_register_to_global`: Copies the contents of a data register into a global variable.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
- `tiny_ori_4`: Small routine of 1 instruction that merges 176 into d2.
- `invoke_bytecode_parser`: Calls a routine that processes a stream of bytes to execute specific handlers based on the byte values encountered.
- `store_pointer_from_low_nibble`: Computes a pointer from the low nibble of the input word shifted left three bits and stores it into a global variable.
- `store_offset_pointer`: Computes a pointer by adding a value to a global variable, where the value is derived from the low nibble of an input word shifted left by three bits.
- `clear_absolute_byte_variable`: Clears a single byte in absolute memory to zero; the instruction touches only that byte and does not alter any register.
- `parse_bytecode_instruction_stream`: Iterates through a memory buffer, executing different routines or updating global variables based on the value of each byte encountered.
- `parse_data_stream`: Iterates through a memory buffer, performing different actions based on the value of each byte encountered.
- `set_global_and_call`: Stores the input word ORed with a constant into a global location, then calls another routine.
- `compute_offset_from_bytes`: Reads two bytes from a structure, clamps the second to a maximum of 48, and computes a pointer by adding the first byte plus the clamped value times 160 to a base address.
- `call_byte_returning_routine`: Calls another routine and returns the value it produces in the data register.
- `blit_masked_bytes_into_bitmap`: Uses a byte from a table to combine two source bitmaps with a mask and writes the result into a destination bitmap across a small block, then increments a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_bitmap_structures_at_offset`: Calls another routine using a pointer derived from an address register plus two.
- `repeat_blit_until_counter_zero`: Repeatedly calls a bitmap-blitting routine and loops until a returned word value becomes zero.
- `execute_self_and_return`: Calls itself once and then returns the value stored in a register.
- `poll_status_and_dispatch`: Loops reading a global status byte, calling another routine each pass, and branching on individual bits of the status.
- `setup_and_call_function`: Stores two values and two pointers into consecutive global memory locations before calling a function via a pointer.
- `store_interrupt_server_table`: Writes a small set of fixed values into consecutive global words and longs, then calls another routine through a pointer derived from a global variable.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `initialize_trackdisk_port`: Opens the trackdisk device and creates a message port by adding a new port to the system.
- `issue_doio_with_command_nine`: Sets fields in a global structure to request command nine with a length of one and a zero flag, then calls the I/O routine.
- `issue_doio_with_command_8004`: Sets a command field in a global structure to a specific value and a flag byte to zero before calling the I/O routine.
- `issue_doio_with_command_five`: Sets the command field of a global structure to five and a flag byte to zero, then calls the I/O routine.
- `process_io_requests`: Iterates through a series of values, performing I/O operations with command nine for each non-zero entry found.
- `issue_doio_and_copy_words`: Fills fields of a global structure for an I/O request, calls the I/O routine, and if a status byte is zero copies a number of words determined by the input word into a destination pointer.
- `issue_io_and_load_pointer`: Calls two routines that issue I/O requests, then loads a longword from a fixed offset in a global structure into a global variable.
- `process_compressed_asset_stream`: Decodes and expands a compressed data stream into a destination buffer using nested loops for block and word-based copying.
- `blit_compressed_graphics`: Decodes a custom compressed bitstream to fill a destination buffer in memory.
- `blit_run_length_encoded_tile`: Decodes a run-length encoded byte stream into a planar bitplane buffer, expanding literal, repeated-byte, and word-copy runs while advancing through rows of a fixed stride and wrapping at the buffer end.
- `interrupt_server_data_table`: The listed bytes are a data table reached by address; decoded as instructions they are meaningless immediate operations and moves, not executable code.
- `tiny_ori_5`: Small routine of 1 instruction that merges 188 into d3.
- `update_event_queue`: Copies entries from a source table into a destination buffer and processes them through a series of conditional checks.
- `tiny_ori_6`: Small routine of 1 instruction that merges 2 into d4.
- `tiny_ori_7`: Small routine of 1 instruction that merges 130 into d4.
- `synchronize_interrupt_server_table`: Copies a set of values from a source table into a destination buffer and updates several global variables.
- `update_active_object_slots`: Iterates through a table of object slots, decrementing counters and executing specific handlers based on a type byte.
- `parse_bytecode_stream`: Iterates through a data stream, interpreting control codes to perform operations such as updating pointers, modifying global values, performing arithmetic on specific memory locations, or jumping to new stream offsets.
- `clear_buffer_alternating`: Increments a global counter, selects one of two buffer pointers based on the counter's low bit, stores that pointer in a global variable, then writes zero to four consecutive longwords per iteration for two thousand iterations, clearing the selected buffer.
- `call_library_and_wait_vblank`: Calls a routine through a pointer offset from a global pointer, then polls a global variable until its bit 32 becomes set, waiting on a hardware status flag.
- `execute_infinite_loop_2`: An infinite loop that performs no operations.
- `execute_infinite_loop`: An infinite loop that may call another routine twice before entering a permanent stall.
- `wait_for_potgor_bit`: Stores a word from a register into two global variables, calls a routine through a table indexed by that word, and then loops until bit 2 of a global variable is set, returning the value of another global variable.
- `tiny_ori_8`: Small routine of 1 instruction that merges 4 into d0.
- `monitor_state_and_stall`: Checks a global byte value to determine whether to enter an infinite loop or execute a sequence of subroutine calls.
- `close_device_and_remport`: Sets a global variable to five, then loops calling a routine and a library vector until a returned value matches a constant; afterwards it calls two library vectors through a base pointer, stores that base into two globals, and polls a global bit while waiting.
- `clear_2000_longwords`: Clears a block of memory by writing zero to 2000 consecutive longwords starting at a global pointer.
- `infinite_loop_data_byte`: The single byte decodes as an OR immediate of zero into d0 and is data, not executable code, reached through an address.
- `decrement_global_counter`: An instruction that decrements a specific global variable.
- `decrement_buffer_counter`: Decrements a global counter in a loop and modifies a global variable by performing arithmetic operations on its bytes.
- `set_flag_and_call_helpers`: Sets bit 1 of a global variable, writes a constant to another global variable, and then calls three helper routines to process an event queue, decrement a counter, and initialize audio and scan tables.
- `initialize_audio_and_scan_table`: Stores pointers derived from an incoming base pointer into globals, clears a series of global variables and sets one to a small constant, then scans a word table to find the maximum value while counting down, and finally calls another routine and stores a constant pointer.
- `process_table_entries`: Iterates over a table of entries, skipping zero entries, and for each nonzero entry updates fields based on a running value and a constant, possibly resetting a field to a fixed address.
- `silence_audio_channels`: Writes zero to the volume registers of all four audio channels and enables DMA for those channels.
- `execute_audio_sequence`: Executes a series of function calls through pointers stored in a global structure while updating a custom hardware register and several data fields.
- `increment_and_reset_counter`: Increments a global variable and, if it reaches a certain threshold, resets it and performs a table-based update of a global pointer.
- `update_internal_counters`: Increments a global counter and, upon reaching a specific value, performs a loop to reset a counter and update a global pointer based on a table.
- `initialize_global_pointers`: Sets a global variable to one and initializes several global pointers and values based on an offset from a global base address.
- `update_audio_dma_control`: Adds a constant value to a global variable and writes to the DMA control register.
- `update_audio_parameters_2`: Calculates a pointer to a data structure from an input value and updates several global variables and audio hardware registers based on a selection value.
- `update_audio_parameters`: Calculates a pointer to a data structure and, depending on a register value, updates various global variables and audio hardware registers using data from that structure.
- `update_audio_channels_2`: Depending on a register value, writes data from a pointer to various global variables and audio hardware registers for different channels.
- `update_audio_channel_three`: Checks if a register equals three, then writes values from a pointer to several global variables and audio hardware registers.
- `store_audio_registers`: Stores values from a structure into global variables and writes to audio hardware registers for channel 3 (length, period, volume, and high bits of pointer).
- `dispatch_audio_command`: Parses a command word to update various global variables, modify specific bits in a global variable, or return a value from a register.
- `update_joystick_flag_2`: Checks a register value and either updates a global variable with a word or modifies specific bits within a global variable.
- `update_joystick_flag`: Checks a hardware register and sets or clears a specific bit in a global variable based on the register's value.
- `set_flag_if_register_matches`: If a register equals a specific value, sets bit 1 of a global variable.
- `calculate_offset_pointers`: Calculates and stores a new value and a pointer into global variables based on an input value.
- `store_negated_word`: Negates a word from a register and stores the result into one of several global variables based on a register value.
- `store_word_to_various_globals`: Checks a register value and stores a word from a register into one of three different global variables.
- `store_word_to_global_sub_2bf0`: Checks if a register equals three, and if not, stores a word from a register into a global variable.
- `store_word_to_global`: Stores a word from a register into a global variable.
- `assign_word_to_global`: Stores a word from a register into one of several global variables depending on the value of another register.
- `store_conditional_word`: Checks a register value and stores a word from another register into one of several global variables based on that value.
- `store_word_to_global_sub_2c10`: Checks if a register equals three, and if not, stores a word from a register into a different global variable.
- `store_word_to_global_alt`: Stores a word from a register into a different global variable.
- `update_audio_channels`: Updates audio channel registers (pointer, length, period, volume) for channels 0-3 based on flags, enables DMA channels, and computes scaled values from global variables.
- `calculate_offset_value`: Performs an arithmetic operation involving a global variable and a value derived from a register and memory.
- `modify_global_pointer`: Updates a global variable using a calculation based on a register and memory contents.
- `update_address_register`: Calculates a new value using a global variable and memory contents, then stores the result.
- `perform_arithmetic_operation`: Performs a complex calculation involving a global variable and a value from a data register, then updates a global variable with the result.
- `perform_arithmetic_operation_2`: Calculates a new value using a global variable and a register, then stores the result back into a global variable.
- `perform_arithmetic_operation_3`: Updates a global variable by adding a value derived from a register and another global variable.
- `perform_arithmetic_operation_4`: Computes a value based on a global variable and register contents, then modifies a global variable.
- `tiny_ori_9`: Small routine of 1 instruction that merges 0 into d5.
- `perform_arithmetic_operation_5`: Modifies a global variable by adding a value calculated from a register and a global variable.
- `perform_arithmetic_operation_6`: Uses a global variable and register data to calculate a value, then updates a global variable with the result.
- `perform_arithmetic_operation_7`: Performs a calculation using a global variable and register data, and stores the result in a global variable.
- `perform_arithmetic_operation_8`: Calculates a value using a global variable and register contents, then updates a global variable.
- `perform_arithmetic_operation_9`: Updates a global variable by adding a value derived from a register and a global variable.
- `perform_arithmetic_operation_10`: Calculates a value using a global variable and a register, then stores the result in a global variable.
- `add_indexed_byte_to_global`: Reads a byte from a table indexed by the low 15 bits of a global variable, masks off one bit, adds it to that global, and stores the result back into the global while returning the low byte of the sum.
- `update_memory_buffers`: Calculates a new value using a global variable and stores it in multiple global locations.
- `copy_selected_entries`: Selects a table entry based on a word argument, then for each set bit in a byte argument copies four words from that entry into fixed memory locations and sets corresponding bits in a global variable.
- `fetch_table_entry_and_distribute_fields`: Calculates an offset into a table using a word value multiplied by fourteen and retrieves a pointer from that entry.
- `select_entry_and_store_fields`: Uses a word argument times fourteen as an offset into a table to fetch an entry; if the first longword is negative it follows the second longword as a pointer, then conditionally copies a longword from offset ten of that entry into four separate global locations according to bits of a byte argument, returning the first longword.
- `update_dma_control_state_machine`: Processes a bit-flagged control word to update counters, pointers, and a state block, while writing to the DMA control register to enable specific channels.
