# Game 2462 (Waterloo v1.1 1938): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02462). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2192475 top-level bytes in 32 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 225 of 225 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 698 of 698 named (0 author symbols), 698 of 698 described (123 by rule, 570 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 7349 of 7598 (96.72%) |
| Recovered without load address (counted in proof 1, flagged) | 1152 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 19025 of 19025 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 36591 |
| code | 65140 |
| data | 1405156 |
| padding | 4116 |
| unused | 681472 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk-Validator | 1 | 386 | 1 |
| Disk.1:preferences | 45 | 6587 | 226 |
| go | 2 | 233 | 8 |
| Disk.1:CD | 2 | 117 | 2 |
| game.slave | 1 | 268 | 35 |
| Disk.1:bootblock | 1 | 10 | 1 |
| logged disk load from Disk.1 | 1 | 10 | 1 |
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
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 106 | 59 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 246 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 987 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 259 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2679 | 68 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4499 | 104 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2633 | 152 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |
| logged disk load from Disk.1 | 1 | 98 | 1 |
| logged disk load from Disk.1 | 1 | 113 | 3 |
| logged disk load from Disk.1 | 1 | 32 | 1 |
| logged disk load from Disk.1 | 1 | 137 | 1 |
| FRENCH.ORD (raw code file, load address unknown) | 1 | 0 | 0 |
| ALLIES.ORD (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- Waterloo_v1.1_1938/Waterloo_v1.1_1938.hdf: hard-disk image (AmigaDOS OFS, 2272 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree Waterloo_v1.1_1938_hdf, which is recovered file by file; no block outside the filesystem holds data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 205 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 78 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 173 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 48 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 208 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1802 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (ext): 3438 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (ext): 1247 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Waterloo_v1.1_1938/Waterloo_v1.1_1938_hdf/Waterloo/Disk.1: 88 executed PCs whose RAM was overwritten before any snapshot match its bytes 332304-332672: recovered as a unit run at an address
- Waterloo_v1.1_1938/Waterloo_v1.1_1938_hdf/Waterloo/Disk.1: 87 executed PCs whose RAM was overwritten before any snapshot match its bytes 343184-343600: recovered as a unit run at an address
- Waterloo_v1.1_1938/Waterloo_v1.1_1938_hdf/Waterloo/Disk.1: 24 executed PCs whose RAM was overwritten before any snapshot match its bytes 520096-520240: recovered as a unit run at an address
- Waterloo_v1.1_1938/Waterloo_v1.1_1938_hdf/Waterloo/Disk.1: 44 executed PCs whose RAM was overwritten before any snapshot match its bytes 342544-343072: recovered as a unit run at an address
- Waterloo_v1.1_1938/Waterloo_v1.1_1938_hdf/Waterloo/Disk.1:Waterloo/FRENCH.ORD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Waterloo_v1.1_1938/Waterloo_v1.1_1938_hdf/Waterloo/Disk.1:Waterloo/ALLIES.ORD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Waterloo_v1.1_1938/Waterloo_v1.1_1938_hdf/Waterloo/Disk.1:system-configuration: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- Waterloo_v1.1_1938/Waterloo_v1.1_1938_hdf/Waterloo/Disk.1: 209 logged loads cover 0 bytes of the image
- Waterloo_v1.1_1938/Waterloo_v1.1_1938_hdf/Waterloo/Disk.1:Waterloo/go: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 1152 bytes, recorded as UNVERIFIED DATA (0 bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and stores it in a buffer.

### Disk.1:preferences

- `parse_preference_file`: Reads a configuration file and parses its contents into a structured format.
- `invoke_conditional_function_pointers`: Stores a parameter into a local stack location and invokes two function pointers from a global variable if certain conditions are met.
- `invoke_two_function_pointers`: The routine invokes two function pointers located at fixed negative offsets from a register and then returns.
- `invoke_conditional_function_pointers_2`: The routine stores a parameter into a local stack location and invokes two function pointers from a global variable if certain conditions are met.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_rename`: Library stub: loads its arguments and the dos library base, calls Rename and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_date_stamp`: Library stub: loads its arguments and the dos library base, calls DateStamp and returns its result to the caller.
- `call_delay`: Library stub: loads its arguments and the dos library base, calls Delay and returns its result to the caller.
- `initialize_preference_data`: Initializes several global variables, copies data from a table into a buffer, and performs conditional updates to specific memory ranges.
- `initialize_preference_system_2`: Initializes the preference management system by setting up internal structures, loading library vectors, and configuring a task for handling user settings.
- `initialize_preference_system`: Manages complex preference initialization by handling semaphores, message ports, and grid-based data structures.
- `cleanup_preferences`: The routine performs a sequence of calls to other routines with pointers to global variables and integer arguments, then returns without further processing.
- `set_preference_values_2`: Calls another routine twice with different first arguments, passing a calculated address from a global variable and the input parameter.
- `decode_input_words`: The routine selects one of two offsets from a base pointer depending on a flag and then loops four times calling another routine with a constant, an index and a word read from the selected offset.
- `decode_input_word`: The routine checks whether a 16-bit value exceeds four and clears the first argument if so, then forwards the arguments to another routine after splitting the third argument into three 4-bit fields.
- `initialize_preference_structures`: Builds a linked list of 112 entries, each 44 bytes long, by copying six 16-bit fields from a source table.
- `handle_preferences_input`: Dispatches on a selector from a global structure to perform preference-related updates by writing values to global variables and fields.
- `initialize_preference_handlers_2`: Initializes preference handlers by calling another routine with a global variable and values from a table, updates control parameters, decodes nibbles from a global table into global variables, and applies preference masks.
- `initialize_preference_handlers`: The routine calls another routine several times with a global variable and values taken from a table indexed by the parameter, then returns zero.
- `update_control_parameters`: Calculates several values from a structure and a global variable, then calls another routine multiple times to update specific fields.
- `clamp_and_update_control_values`: The routine clamps two signed byte parameters to a specific range, optionally stores them into a structure and updates two short fields in a structure obtained from another routine.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_add_task`: Library stub: loads its arguments and the exec library base, calls AddTask and returns its result to the caller.
- `call_rem_task`: Library stub: loads its arguments and the exec library base, calls RemTask and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_wait`: Library stub: loads its arguments and the exec library base, calls Wait and returns its result to the caller.
- `call_signal`: Library stub: loads its arguments and the exec library base, calls Signal and returns its result to the caller.
- `call_alloc_signal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `call_free_signal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `call_add_port`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `call_rem_port`: Library stub: loads its arguments and the exec library base, calls RemPort and returns its result to the caller.
- `call_get_msg`: Library stub: loads its arguments and the exec library base, calls GetMsg and returns its result to the caller.
- `call_reply_msg`: Library stub: loads its arguments and the exec library base, calls ReplyMsg and returns its result to the caller.
- `call_find_port`: Library stub: loads its arguments and the exec library base, calls FindPort and returns its result to the caller.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_open_device`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `call_close_device`: Library stub: loads its arguments and the exec library base, calls CloseDevice and returns its result to the caller.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `call_send_i_o`: Library stub: loads its arguments and the exec library base, calls SendIO and returns its result to the caller.
- `call_abort_i_o`: Library stub: loads its arguments and the exec library base, calls AbortIO and returns its result to the caller.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_init_semaphore`: Library stub: loads its arguments and the exec library base, calls InitSemaphore and returns its result to the caller.
- `call_obtain_semaphore`: Library stub: loads its arguments and the exec library base, calls ObtainSemaphore and returns its result to the caller.
- `call_release_semaphore`: Library stub: loads its arguments and the exec library base, calls ReleaseSemaphore and returns its result to the caller.
- `call_attempt_semaphore`: Library stub: loads its arguments and the exec library base, calls AttemptSemaphore and returns its result to the caller.
- `allocate_message_entry`: The routine attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port`: The routine removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `format_two_values`: The routine reads a global variable, calls another routine and combines the result with the variable to form two decimal digit pairs, stores the resulting characters into global variables and returns zero.
- `find_and_format_index`: The routine loops over a range using a global variable and another routine, selects an index based on a table lookup, adjusts the index with wrap-around arithmetic and calls another routine with three computed values.
- `format_three_digits`: The routine converts the three integer parameters into decimal digits by division and subtraction, stores the resulting characters into a global variable sequentially, and returns zero.
- `configure_preferences`: The routine calls another routine with the argument and several constants, performs assignments to a global variable, passes addresses of the global variable to another routine with constants 2 and 5, indexes a table using arithmetic on the global variable to obtain a value for another call, and returns zero.
- `calculate_message_parameters`: Computes a series of values based on a global variable and performs conditional arithmetic to prepare parameters for a message allocation routine.
- `increment_and_clamp_numeric_values`: Increments a two-digit value while clamping its components and wrapping the total at ninety-nine.
- `increment_and_format`: The routine clamps two uninitialized values, wraps the second value at 99, and calls another routine with the adjusted values.
- `clamp_and_format_digits_2`: The routine clamps two input values, wrapping the first value to a minimum of zero and the second value to a range with a lower bound, then calls another routine.
- `clamp_and_format_digits`: The routine clamps two input values, wrapping the first value to a minimum of zero and the second value to a range with a lower bound, then calls another routine to format the two values as three decimal digits.
- `setup_timer_message`: The routine attempts to allocate a message entry, populates fields of the allocated buffer with parameters and a computed value, and returns a status flag indicating success.
- `process_preference_loop_2`: Allocates a message buffer, populates it with parameters, and enters a loop that performs bitwise operations and updates a global variable.
- `process_preference_loop`: Performs bitwise operations on values from memory and enters a loop that conditionally calls other routines while incrementing a global variable.
- `process_preference_updates`: Performs bitwise operations on values retrieved from memory and enters a loop that conditionally calls other routines and increments a global variable.
- `return_from_subroutine_5`: The routine performs a return from a subroutine.
- `setup_preference_task`: Initializes message ports and adds a task to the system using a global variable as a parameter.
- `cleanup_resources`: Conditionally calls another routine based on a bit mask from a global variable, clears several global variables, and calls other routines to release resources.
- `fill_io_request_fields`: The routine stores a constant into a word field of a structure pointed to by a parameter, writes two pointer arguments into following fields, then calls another routine with the structure pointer.
- `write_system_configuration`: Attempts to open a file, and if unsuccessful, attempts to delete it and open it again before writing a specified amount of data to the file and closing it.
- `write_system_configuration_file`: The routine attempts to open the file named DEVS:system-configuration for writing using library stubs, writes 232 bytes from an input pointer, and closes the file.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `process_preference_selections`: Iterates through a data structure to process preference selections, updates a global selector, and dispatches updates via various routine calls.
- `dispatch_preference_updates_2`: Dispatches updates based on a global selector and processes preference selections through a switch-based logic.
- `process_preference_selection_2`: Evaluates a value from a pointer offset to execute a switch-based logic that calls another routine with various parameters and updates a global variable.
- `process_preference_input`: Performs a loop that processes data through another routine and eventually calls a routine to copy fields from a global variable to itself.
- `copy_word_fields_2`: The routine copies four 16-bit words from a source pointer to a destination pointer at the same offsets.
- `copy_word_fields_3`: Copies four 16-bit words from a source pointer to a destination pointer at the same offsets.
- `process_preference_selection_3`: Retrieves messages and iterates through a data structure to process preference selections and dispatch updates based on a global selector.
- `return_void`: This routine is empty and performs no operations before returning.
- `copy_word_fields`: Copies four 16-bit words from offsets 168, 170, 172 and 174 of the second argument to the same offsets in the first argument.
- `process_preference_selection`: Evaluates a value from a pointer offset to execute a switch-based logic that calls another routine with various parameters and updates global variables.
- `return_constant_one`: The routine returns the constant value one to the caller.
- `return_immediately_2`: This routine performs no operations and returns control to the caller immediately.
- `conditionally_reset_global_flag`: The routine checks a global variable and a global byte for specific constant values and, if they match, calls another routine with a constant argument and clears the global variable.
- `check_not_one`: Compares a passed value against one and returns a boolean result indicating if the value is not equal to one.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `return_passed_value`: Returns the value passed in the first argument directly to the caller.
- `set_signal_and_store_byte`: The routine calls another routine twice with arguments derived from a global variable and a parameter, stores a byte from a register through a pointer, and then stores a different byte value through the same pointer.
- `find_table_index`: Iterates through a table of 16-bit words to find the index of the largest value smaller than the provided parameter.
- `search_word_table`: The routine iterates through a table of 16-bit words until it finds a value greater than or equal to the value in a register, returning the index.
- `decrement_register_value`: The routine returns the value of a specific register minus one.
- `initialize_table_and_copy_data`: The routine clears a global variable, loops over a table to call another routine and copy word data from a table into memory, then copies several global values and inverts a block of words in a global variable.
- `call_routine_with_argument`: The routine calls another routine with a pointer to a global variable and an integer argument.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `build_preferences_list`: Initializes globals, locks a device, and iterates through directory entries to copy filenames into a temporary buffer.
- `compare_string_prefixes`: Compares two null-terminated strings character by character.
- `initiate_preferences_buffer`: The routine allocates a block of memory for a preferences structure, stores the pointer in a global variable and writes the first word of the block with a pointer to the global variable itself.
- `free_linked_list_entries`: Iterates through a linked list of memory blocks pointed to by a global variable, freeing each block by calling another routine, and finally resets the global list pointers to zero.
- `iterate_table_and_compare`: The routine walks a table of routine pointers stored in a global variable, calling each in turn until one returns non-zero, then clears a 30-byte global buffer and returns a flag.
- `compare_padded_string`: Builds a 30-byte stack buffer filled with spaces and a null terminator, copies up to 29 characters from a global variable into it, then compares the buffer byte-for-byte with the input string and returns one on a full match or zero otherwise.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_init_code`: Library stub: loads its arguments and the exec library base, calls InitCode and returns its result to the caller.
- `call_init_struct`: Library stub: loads its arguments and the exec library base, calls InitStruct and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `call_add_head`: Library stub: loads its arguments and the exec library base, calls AddHead and returns its result to the caller.
- `call_add_tail_2`: Executes a function pointer stored at a location relative to a global variable.
- `execute_indirect_function_pointer`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_set_signal`: Library stub: loads its arguments and the exec library base, calls SetSignal and returns its result to the caller.
- `execute_indirect_function`: Fetches a function pointer from a global variable and executes it.
- `execute_indirect_function_2`: Fetches a function pointer from a location offset from a global variable and executes it.
- `call_alloc_trap`: Library stub: loads its arguments and the exec library base, calls AllocTrap and returns its result to the caller.
- `call_free_trap`: Library stub: loads its arguments and the exec library base, calls FreeTrap and returns its result to the caller.
- `execute_indirect_function_3`: Fetches a function pointer from a location calculated by an offset from a global variable and executes it.
- `process_preference_grid`: Iterates through multiple loops to extract values from tables and calls another routine with derived parameters based on a grid structure.
- `compute_cstring_length`: The routine scans a null-terminated character string byte by byte until the terminator is found and returns the number of bytes from the start to the terminator.
- `call_library_with_constant_args`: The routine calls another routine with a global variable, the incoming parameter and the constant value -1.
- `configure_trap_with_semaphore`: The routine calls another routine with a global variable, adjusts a parameter, then performs several calls to another routine with computed offsets and constant values, and finally calls the first routine again with the global variable.
- `apply_preference_masks`: Calls another routine twice with a global variable and with only the first two parameters before and after a series of conditional calls.
- `return_value_by_param`: Returns zero when the short parameter equals nine and returns eight otherwise.
- `return_value_by_param_2`: Returns zero when the short parameter equals one and returns sixteen otherwise.
- `call_other_routines_with_params`: Calls another routine repeatedly with different combinations of the four short parameters and values derived from a global variable.
- `call_with_conditional_constant`: The routine selects a constant based on the input value and calls another routine with that constant and three additional fixed arguments.
- `call_routine_with_constants`: The routine calls another routine once with four constant short arguments and then returns.
- `format_number_to_string`: The routine converts a 16-bit numeric parameter into an ASCII decimal string at a character pointer, handling hundreds and tens with conditional zero padding, and terminates the string with a null byte.
- `iterate_and_process_entries_2`: The routine loops over a range defined by two registers, building a pointer from a global variable and an index and calling another routine.
- `iterate_and_process_entries`: The routine loops over a range defined by two registers, building a pointer from a global variable and an index and calling another routine with a constant and that pointer on each iteration using a library vector call.
- `iterate_and_clear_entries_2`: The routine loops over a range defined by two registers, building a pointer from a global variable and an index and calling another routine with a constant value.
- `iterate_and_clear_entries`: The routine loops over a range defined by two registers, building a pointer from a global variable and an index and calling another routine with a constant, that pointer and an all-ones argument on each iteration using a library vector call.
- `dispatch_preference_updates`: Dispatches updates based on a global selector.
- `setup_message_ports`: The routine calls another routine twice with the first argument and different constants, stores the results, and if either result is zero it calls another routine with the first argument and 1 and returns zero.
- `wait_for_signal`: The routine checks a global variable for non-zero, sets a bit in another global variable, calls another routine with a signal mask derived from a global variable, then repeatedly calls a delay routine while the first global variable remains non-zero, and optionally calls another routine with the parameter and 1 before returning.
- `modular_inverse_32bit`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `recursive_call`: The routine calls itself and then returns.
- `call_and_return_d1`: The routine calls another routine and returns the value found in the D1 register.
- `negate_on_flag`: The routine calls another routine, checks a second return value, and returns the negated first return value if the flag is non-zero.
- `negate_on_flag_alt`: Calls another routine and returns its first return value, or the negation of that value if the second return value is non-zero.
- `store_encoded_value_and_forward`: The routine stores an encoded byte derived from three parameters into a global variable and then calls another routine with the parameters.
- `decode_parameter_and_update_tables`: The routine extracts nibbles from a word taken from a global table using an input parameter, stores the nibbles in global variables, computes a 32-bit product via a helper, and updates three separate data structures using library calls.
- `create_std_io`: The routine calls another routine to create an extended I/O structure for the supplied pointer, passing a constant size argument.
- `delete_std_io`: The routine receives a pointer argument and forwards it unchanged to another routine that deletes an extended I/O. It performs no other work and then returns.
- `parse_preference_data_2`: Iterates through a data structure to parse configuration values and updates a global variable based on specific type flags.
- `parse_configuration_data_3`: Iterates through a data structure to parse configuration values, performing conditional copies and dispatching updates based on identified flags.
- `parse_configuration_data_2`: Iterates through a data structure, performing conditional copies and processing various data types based on identified flags.
- `copy_global_structure_2`: Performs a loop involving function calls and conditional logic before copying a global structure onto itself.
- `copy_preference_struct_2`: The routine copies a set of fields from one structure to another and returns zero.
- `copy_global_structure`: The routine calls another routine to copy a structure onto itself using a global variable as the source and destination.
- `parse_preference_data`: Iterates through a data structure to parse configuration values and updates a global variable based on the results.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `copy_preference_struct`: The routine copies a set of fields from one structure to another, including several word fields, a byte field and an array of words, then calls another routine three times with fixed data pointers.
- `initialize_preference_structures_2`: Initializes and populates preference data structures by copying values from a table and performing iterative processing.
- `parse_configuration_data`: Reads a data structure through a pointer and branches based on a type identifier.
- `return_zero`: This routine is empty and simply returns a zero value to the caller.
- `process_preference_tables_2`: The routine reads a word from a table using an index, splits it into four nibbles into a global variable, and calls another routine three times.
- `process_preference_tables`: The routine reads a word from a table using an index, splits it into four nibbles into a global variable and then calls another routine three times with data from three different tables and a computed value.
- `store_computed_byte_and_recurse`: Stores a byte computed from three short parameters into a global variable at an offset derived from the first short parameter and then calls another routine with the first parameter increased by four.
- `set_preference_values`: The routine repeatedly calls another routine with arguments derived from a global variable table and a fixed data table, using the same handler pointer stored in a global variable.
- `call_other_routines_with_constants`: Calls another routine three times with the same arguments derived from a global variable and a constant.
- `handle_preference_signal`: The routine adjusts a short parameter with wrap-around and then calls another routine repeatedly with offsets derived from two word parameters and a global variable, conditionally invoking another routine based on a comparison with a global variable.
- `handle_preference_signal_2`: The routine calls a library vector function and then loops over a 16 by 16 grid, calling another routine with values derived from two words in a global variable.
- `modify_two_word_bits`: Clears a bit in two words of a global variable selected by the short parameter and then sets bits in those words based on the ushort parameter.
- `set_preferences_signal_handler`: The routine configures a signal handler for a preferences structure by invoking another routine with offsets derived from a global variable, checks the returned status, and if a specific condition is met updates the handler with a value obtained from the same helper.
- `create_ext_io`: The routine allocates memory for an I/O request structure when the first argument is non-zero, initializes specific fields with constants and arguments, and returns the pointer to the block or zero.
- `delete_ext_io`: Clears specific fields in an I/O request structure and frees the memory block using the size stored within the structure, performing no operation if the pointer is zero.
- `reset_preference_settings`: Clears a global configuration buffer and reinitializes several configuration parameters from a data table.
- `parse_preference_command_2`: Reads a command byte from a data structure and executes a switch-case logic to update various global configuration values.
- `clear_preference_buffer`: Clears or copies a thirty-byte block of memory based on a global flag and updates a global variable.
- `execute_callback_table`: Iterates through a table of function pointers stored in a global variable, calling each until one returns a non-zero value.
- `clear_or_copy_buffer`: Either copies a 30-byte block to itself or clears it with zeros based on a global flag, then updates a global variable and calls another routine.
- `clear_buffer_and_copy`: The routine clears a 30-byte global buffer by writing zeros, updates a global variable with a value from another global, and then calls another routine with the same buffer as both arguments.
- `parse_preference_command_3`: Reads a command byte from a data structure to update global configuration values and clears a buffer based on a global flag.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `copy_struct_fields_and_clear_flag`: The routine copies several word and byte fields between two memory blocks, copies a block of bytes between the blocks, and clears a global variable.
- `parse_preference_command`: Reads a command byte from a data structure and executes a switch-case logic to update various global configuration values.
- `return_zero_2`: The routine is empty and simply returns a value of zero to the caller.
- `dispatch_multiple_operations`: The routine conditionally calls another routine multiple times with different arguments derived from global variables and then calls it again with fixed arguments.
- `return_zero_3`: This routine is empty and simply returns a value of zero to the caller.
- `update_display_bounds_high`: Compares global variables to update a specific global value and calls another routine to update display fields.
- `update_display_fields`: The routine calls another routine with a pointer to a global structure and a global variable, stores a global value into a global variable, calls another routine with a global variable and a global value, calls another routine with a global structure and a global variable and a constant, and calls another routine with an offset address and a global structure.
- `update_display_bounds_low`: Compares global variables to update a different global value and calls another routine to update display fields.
- `update_ui_numeric_displays`: Converts a numeric value into an ASCII string at a specific memory location and updates multiple display fields using various parameters, including global structures and constants.
- `initialize_preference_entry`: Checks a global variable and, if it is less than one, sets a global flag and calls another routine.
- `update_display_fields_2`: The routine calls another routine with a pointer to a global structure and a global variable, stores a value from a register into a global variable, calls another routine with a global variable and a value, calls another routine with a global structure, a global variable, and a constant, and calls another routine with a calculated offset and a global structure.
- `apply_preference_settings`: Recursively traverses a hierarchical preference tree, updates library vector settings and flags, and processes a preference grid to extract values into a global variable.
- `traverse_preference_tree`: Recursively traverses a hierarchical data structure by checking specific type flags and following pointers.
- `copy_preference_data`: Processes data through a loop and calls a routine to copy fields from a global variable to itself.
- `copy_struct_fields_2`: The routine copies a 16-bit value from an offset in the second argument to the first argument and copies three bytes from offsets 182 and 183.
- `copy_struct_fields_3`: The routine copies a 16-bit value from an offset in the second argument to the first argument and copies three bytes from offsets 182 and 18.
- `traverse_preference_tree_2`: Recursively traverses a hierarchical data structure by checking type flags and following pointers to copy preference data.
- `return_immediately_3`: This routine is empty and performs no operations before returning to the caller.
- `copy_struct_fields`: The routine copies a 16-bit value from an offset in the second argument to the first argument and copies three bytes from offsets 182, 183 and 184 of the second argument to the same offsets in the first argument.
- `update_preference_settings`: Updates various configuration settings by cycling through index values or toggling bitmasks in a global variable.
- `return_one`: The routine returns the value one.
- `return_two`: The routine returns the value two.
- `return_constant_one_2`: The routine returns the integer value one to the caller.
- `return_null_value`: An empty routine that performs no operations and returns without modifying any registers or memory.
- `configure_library_vector_settings`: The routine calls another routine multiple times with a base pointer from a global variable and constants 1, 1, 0 and 130 with 36, then with a value from a table indexed by a global variable and constant 5.
- `update_library_vector_flags`: The routine calls another routine multiple times with a base pointer from a global variable and constants 1, 1, 0 and 430 with 36, then with a value from a table indexed by a global variable and constant 5.

### go

- `parse_amiga_hunk_loader`: Parses and processes a data stream containing AmigaDOS hunk type codes to allocate memory and relocate code segments.
- `decrement_counter_loop`: Decrements a value in a loop until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decompress_bit_stream`: Performs bitwise shifts and logical operations to manipulate data extracted from a table and registers.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: The routine iterates through a loop to perform bitwise shifts and logical operations on data extracted from a table and registers.
- `execute_trap`: Triggers a software interrupt via a trap instruction.

### Disk.1:CD

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_block`: Copies a block of data from a table to a destination pointer and then calls another routine.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `call_indirect_functions`: Stores values into global variables and executes three different functions via pointers stored in registers or globals.
- `perform_indirect_call`: Performs an indirect call to an address stored in a jump table.
- `execute_indirect_jump`: Performs an indirect jump to an address retrieved from a memory location.
- `set_register_and_return_offset`: If a register is zero, it copies the value of another register into it, then returns that register's value decremented by eight.
- `mask_global_variable`: Returns a bitwise AND of a global variable and a constant mask.
- `clear_interrupt_request`: Clears a bit in the interrupt request register.
- `store_interrupt_request_to_global`: Stores a value from the stack into a global variable while touching the interrupt request register.
- `return_zero_constant`: The routine immediately returns zero without performing any operations or modifying data.
- `set_flag_in_struct`: The routine receives a pointer in a register and sets bit 1 in the byte at offset 14 of the pointed-to structure.
- `return_immediately_6`: This routine is empty and performs an immediate return to the caller.
- `store_register_to_global`: The routine copies the value from D0 into a global variable and touches the CIAB_PRA register.
- `update_cop_register`: Stores a value into a coprocessor register.
- `copy_byte_to_offset`: Copies a byte from an offset of one pointer to an offset of another pointer.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `check_global_constant`: The routine compares a global variable to a specific constant value.
- `initialize_pointer_chain`: The routine writes a sentinel value into a pointer and stores the address of the next entry into three fields in two different structures.
- `copy_nested_pointers`: Copies 32-bit values from pointers referenced at offsets four and eight of a source structure into fields of a destination structure.
- `negate_nonzero_flag`: The routine receives a value in a register and returns zero if the value is non-zero, and returns a negative one if the value is zero.
- `set_vhposr_bit`: The routine sets a specific bit in the vertical position register via a pointer offset.
- `calculate_table_offset`: Reads values from a global table and performs arithmetic operations including division, multiplication, and addition.
- `update_status_byte`: Checks bits in a structure to determine a value, updates a byte in a structure, and optionally calls a function or converts a value to a character.
- `toggle_byte_flag`: The routine reads a byte from memory, toggles its high bit, writes the modified byte back, and returns a value indicating whether the original high bit was clear.
- `set_flag_from_bit`: Reads a bit from a global variable using an index derived from a byte in another memory location.
- `return_empty`: The routine performs a return without executing any instructions.
- `return_from_subroutine_4`: This routine performs a return from a subroutine.
- `toggle_bit_and_increment`: Checks and clears a specific bit in a global variable, then modifies bits in a structure and increments a value at an offset.
- `return_immediately_7`: This routine is empty and performs an immediate return to the caller without executing any instructions or modifying any state.
- `do_nothing_stub`: The routine executes five instructions but performs no observable work, ending without modifying memory, registers, or calling another routine.
- `perform_no_operation_2`: This routine is empty and performs no actions before returning to the caller.
- `clear_offset_and_save_register`: The routine stores zero to a word-sized location at an offset from a pointer in a register, then saves the value of another register into a global variable before returning.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `search_string_table_2`: The routine parses a table to find a specific string by comparing characters and may call another routine if a match is found.
- `return_empty_2`: The routine consists of three instructions that result in an immediate return.
- `search_string_table`: The routine parses a table to find a specific string by comparing characters, potentially invoking a function if a match is found.

### Disk.1:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### logged disk load from Disk.1

- `check_dos_status`: The routine calls a function from a library and returns zero if the call succeeds or a specific constant if it fails.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `or_data_with_constant`: Performs a bitwise OR operation between the contents of a data register and a constant value.
- `return_data_register`: Returns the current value held in a data register.
- `bitwise_or_constant`: Performs a bitwise OR operation between a byte in a data register and a constant value.
- `return_data_register_2`: Returns the contents of a data register.
- `return_masked_data`: Returns the contents of a data register bitwise ORed with a constant value.
- `return_data_register_3`: Passes the value from a data register back to the caller.
- `return_data_register_4`: Returns the byte held in a data register.
- `return_data_register_5`: Returns the value currently in a data register.
- `return_masked_data_2`: Performs a bitwise OR on a data register value with a constant and returns it.
- `return_data_register_6`: Returns the byte stored in a data register.
- `return_data_register_7`: Returns the value of a data register.
- `return_masked_data_3`: Returns a data register value after applying a bitwise OR with a specific constant.
- `return_data_register_8`: Returns the byte from a data register.
- `return_data_register_9`: Returns the byte value currently held in a data register.
- `return_modified_byte`: Returns a byte value from a data register after performing a bitwise OR operation with a constant.
- `pass_register_value`: Passes the contents of a data register back to the caller.
- `return_register_byte`: Returns the byte contained in a data register.
- `return_d0_value`: Returns the value stored in a data register.
- `transfer_register_byte`: Returns a byte from a data register.
- `return_byte_value`: Returns the byte value from a data register.
- `return_uninitialized_register`: This routine returns the current value of a data register without performing any operations.
- `return_uninitialized_register_2`: The routine returns the current value held in a data register without performing any operations.
- `return_uninitialized_register_3`: This routine returns the current value held in a data register without performing any operations.
- `or_data_register`: Performs a bitwise OR operation on the contents of a data register with a constant value and returns the result.
- `or_data_register_2`: Applies a bitwise OR to a byte in a data register using a specific constant.
- `or_data_register_3`: Returns a byte resulting from a bitwise OR between a data register and a constant.
- `or_data_register_4`: Combines a byte from a data register with a constant using a bitwise OR operation.
- `return_data_register_10`: Returns the current byte value stored in a data register.
- `or_data_register_5`: Uses a bitwise OR to combine a data register value with a constant.
- `return_data_register_11`: Passes the byte value from a data register back to the caller.
- `return_data_register_12`: Returns the byte held in a data register without modification.
- `return_data_register_13`: Outputs the byte value currently residing in a data register.
- `return_data_register_14`: Returns the byte content of a data register.
- `or_byte_with_constant`: Performs a bitwise OR operation between a byte in a register and a constant value before returning.
- `unknown_single_instruction`: This routine consists of a single instruction and is entered from outside the static flow.
- `decrement_word_indirect`: The routine calls another routine that decrements a 16-bit value located at a fixed positive offset from a base pointer register.
- `decrement_global_word`: Decrements a 16-bit value stored at a fixed memory location.
- `invoke_decrement_routine`: Calls a routine that decrements a 16-bit value at a fixed offset from a base pointer and returns a value.
- `decrement_word_at_offset_2`: Decrements a 16-bit value located at a fixed positive offset from a base pointer register.
- `decrement_word_at_offset_3`: Reduces a 16-bit value at a specific offset from a base pointer by one.
- `decrement_word_at_offset_4`: Performs a 16-bit decrement on a value stored at a fixed offset from a base pointer.
- `decrement_word_at_offset_5`: Subtracts one from a 16-bit word located at a fixed offset from a base pointer.
- `decrement_word_at_offset_6`: Decrements a 16-bit integer found at a fixed offset from a base pointer register.
- `decrement_word_at_offset_7`: Decrements a 16-bit value at a fixed offset relative to a base pointer.
- `return_zero_4`: This routine is empty and returns a zero value to the caller.
- `increment_word_at_offset`: The routine increments a 16-bit value located at a fixed positive offset from a base pointer register in place and then returns.
- `decrement_word_at_offset`: The routine decrements a 16-bit value located at a fixed positive offset from a base pointer register and then returns a zero value.
- `store_register_to_memory`: The routine stores a single byte from a register into a memory location pointed to by another register.
- `unknown_single_instruction_2`: This routine consists of a single instruction and is unreached.
- `nop_instruction_execution`: This routine consists of a single no-operation instruction.
- `execute_single_instruction`: This routine consists of a single instruction executed upon entry from an external flow.
- `execute_single_instruction_2`: This routine consists of a single instruction executed upon entry from an external flow.
- `copy_stack_to_register_2`: Copies a value from the stack into a register.
- `copy_stack_to_register_3`: Moves a value from the stack pointer or an offset into a register.
- `copy_stack_to_register_4`: Loads a value from the stack into a register and returns.
- `nop_padding_sequence`: This routine is empty and consists of three instructions that do not perform any functional operations.
- `copy_stack_to_register`: This routine copies a value from the stack pointer or an offset or a pointer to a register and then returns.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a5, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_amiga_hunk_data`: Iterates through a data stream to identify and process AmigaDOS hunk type codes.
- `decrement_register_loop`: decrements a 16-bit value in a register until it reaches zero.
- `process_data_stream`: Iterates through a data stream using bitwise manipulations, toggles a bit in a memory location, and performs an indirect jump.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `perform_bit_stream_manipulation`: Processes a bit stream through a loop, performing bitwise shifts and rotations while updating a pointer and a destination value.
- `process_bit_stream_2`: performs complex bitwise manipulations and shifts on a data stream using a loop and a counter.
- `toggle_bit_and_jump`: Triggers a software interrupt, toggles a specific bit within a byte at a memory location, and then calls a function pointer before entering an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop`: An empty routine that enters an infinite loop.
- `initialize_runtime_state`: Resets multiple global variables to zero and executes a long sequence of subroutine calls before performing conditional assignments based on a stack value.
- `reset_global_flag`: Checks if a global variable is non-zero, calls another routine if it is, and then sets that global variable to zero.
- `update_global_counter`: increments a global variable and resets it to one if it reaches a specific value or becomes a multiple of four.
- `reset_and_update_globals`: Sets a global variable to zero and enters a loop that calls several routines to update counters and entity states based on the value of that variable.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `handle_game_event`: checks conditions in a table to trigger text displays and user input prompts regarding battle status.
- `update_entity_states`: Iterates through a table of entities to evaluate conditions and update their specific numerical attributes and status flags.
- `increment_global_counter`: Executes nested loops that call two different routines to evaluate conditions, incrementing a global variable by either one or ten based on the results.
- `check_table_condition`: Checks a byte in a calculated table offset and, if it is zero, calls another routine to verify a value at a different offset within the same table structure.
- `verify_table_entry`: Evaluates a condition based on a byte at a calculated table offset and calls another routine to check a secondary value at a nearby offset.
- `check_status_flag`: Compares two consecutive bytes at a calculated offset in a table and returns one if a specific pattern is matched, otherwise returns zero.
- `return_zero_value`: The routine performs no operations and returns zero to its caller.
- `update_unit_statistics`: Iterates through a nested data structure representing entities to reset their state and then updates specific numerical attributes based on conditional logic.
- `initialize_global_data`: Initializes a block of global variables with specific constant values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `compute_division_remainder`: Takes a 32-bit parameter, adds one to the lower 16 bits, and returns the quotient of that value divided by two in the upper 16 bits and the remainder in the lower 16 bits.
- `validate_parameter_range`: Checks if a provided parameter falls within a specific range based on a value returned by another routine.
- `process_hunk_data_stream`: Iterates through a data stream to identify hunk type codes and updates a value at a memory location using a table-derived value through a pointer.
- `update_pointer_value`: Adds a value from a table to a value at a memory location and stores the result through a pointer.
- `check_conditional_bounds`: Compares two parameters against specific constant values to return a boolean result.
- `load_value_from_table`: loads a 16-bit value from a memory offset into a destination location.
- `update_area_data`: Iteratively calculates offsets and stores multiple bytes into a large data table based on values retrieved from a function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_conditional_interpolation`: Uses a global variable to select between different combinations of arguments passed to another routine.
- `execute_interpolation_segments`: Passes various offsets from a pointer-based structure to another routine depending on the value of a global variable.
- `execute_conditional_interpolation`: Calls an interpolation routine with different parameter combinations based on the values of a global variable and a value in a table.
- `compute_modulo_division`: Adds three to a 16-bit parameter and returns the quotient and remainder of the result divided by four in a 32-bit value.
- `calculate_scaled_index`: Calls three different routines to determine a divisor and then returns a 32-bit value containing the quotient and remainder of a parameter plus an offset divided by that divisor.
- `compute_modulo_remainder`: Calculates the remainder of a division operation by subtracting the product of the quotient and divisor from the dividend.
- `populate_tile_grid_from_lookup`: Clears a 5x5 grid in memory before iteratively fetching tile attributes from multiple lookup tables based on current state values.
- `process_complex_logic_block`: Performs a substantial amount of processing over 61 instructions before returning.
- `check_table_value`: Fetches a short value from a table using two parameters as indices and returns one if the value matches a specific set of constants and a secondary table entry is zero.
- `search_table_for_match`: iterates through a table to find a non-zero entry and stores the resulting byte values into provided pointers.
- `increment_conditional_value`: Increments a 16-bit value if it is not four and if another part of the parameter matches one of several specific odd integers.
- `process_tile_data_loop`: Iterates through a 5x5 grid of tile data from a global table, applying coordinate transformations based on direction parameters.
- `compute_complex_scaled_result`: Performs a complex calculation involving multiple global variables, table lookups, and bitwise operations to return a 48-bit result.
- `process_clamped_coordinates`: Calculates a scaling factor from a table and uses a switch statement to call another routine with various combinations of clamped parameters.
- `interpolate_coordinate_pairs`: Calls an interpolation routine twice using specific offsets from a pointer to provide coordinate data.
- `interpolate_offset_points`: Calls an interpolation routine twice using values retrieved from a pointer with various signed offsets.
- `interpolate_mixed_offsets`: Calls an interpolation routine twice using a mix of positive and negative offsets from a pointer.
- `interpolate_asymmetric_offsets`: Calls an interpolation routine twice using different offsets from a pointer to process coordinate data.
- `interpolate_boundary_points`: Calls an interpolation routine twice using specific offsets from a pointer to handle boundary values.
- `call_clamping_routines`: Calls a clamping routine multiple times using various values retrieved from a pointer and a global variable.
- `process_line_count_parameters`: Determines a loop count from a global variable and iteratively calls a line-clamping routine, handling cases where two global counters differ.
- `calculate_interpolation_offsets_2`: Calculates destination parameters by adding offsets to retrieved values and dispatches calls based on a global variable.
- `prepare_interpolation_parameters_2`: Calculates sums of coordinates and offsets to pass to an interpolation routine for storing values into global variables.
- `calculate_interpolation_offsets`: Calculates destination parameters by adding an offset from a pointer-based structure to values retrieved from memory or other registers.
- `recurse_with_increment`: A recursive routine that calls another routine with incremented parameters based on the value of an input integer.
- `store_clamped_values`: Retrieves two values from a table via a function, stores them in a global structure, and calls another routine with a sum.
- `update_clamped_coordinate_pairs`: Fetches two values from a structure using offsets, processes them through a clamping routine, and stores the results back into the structure.
- `pass_offset_parameters`: Calls another routine using four values calculated by adding offsets to a base value.
- `store_clamped_values_2`: Clamps two values from a pointer and stores them into a structure, then calls another routine with a calculated sum.
- `iterate_table_data`: Reads a count and a series of offsets from a table to iteratively call another routine with specific memory values.
- `generate_interpolated_points_direct`: Calculates a series of intermediate points between a start and end position.
- `dispatch_conditional_interpolation_2`: Calculates a scaling factor from a table and dispatches interpolation calls using various coordinate parameters.
- `generate_interpolated_points_direct_2`: Conditionally calls point generation routines based on a parameter flag, then executes a sequence of three additional helper routines.
- `dispatch_conditional_routine_calls_2`: Dispatches a series of calls to another routine using modified pointer-based arguments or calculated offsets depending on a global variable.
- `dispatch_interpolation_calls`: Dispatches a series of conditional calls to another routine using pointer-based arguments, calculates interpolation parameters, and stores the results in global variables.
- `dispatch_conditional_routine_calls`: Dispatches a series of calls to another routine using or modified pointer-based arguments or calculated offsets, depending on the value of a global variable.
- `prepare_interpolation_parameters`: Calculates the sum of the start and end coordinates with their respective offsets and passes these four values to an interpolation routine to generate a list of intermediate points.
- `store_interpolated_values`: Calculates and stores several values into a set of global variables and calls another routine.
- `execute_empty_loops`: Performs nested loops that iterate a fixed number of times without modifying any data.
- `increment_and_clear_counters`: Increments a value at a memory location and clears another byte through a pointer within a loop that terminates when a specific threshold is reached.
- `calculate_offset_value`: Computes a value based on a global variable and an input parameter, returning a concatenated result.
- `process_variable_count_lines`: Determines a loop count from a global variable and iteratively calls a line-clamping routine, handling cases where two global counters differ.
- `initialize_pointer_values`: Assigns the two halves of an input parameter to two separate pointer targets and sets a third pointer target to one.
- `interpolate_coordinate_arrays`: Fills two arrays with linearly interpolated values between two coordinate pairs based on a calculated step count.
- `select_pointer_pairs`: Determines a loop count from a global variable and iteratively calls a line-clamping routine, handling cases where two global counters differ.
- `generate_interpolated_point_list`: Calculates a series of intermediate points between a start and end position.
- `process_lines_by_count_branch`: Determines a loop count from a global variable and iteratively calls a line-clamping routine, handling cases where two global counters differ.
- `process_lines_from_globals_branch`: Determines a loop count from a global variable and iteratively calls a line-clamping routine, handling cases where two global counters differ.
- `empty`: This routine is empty and performs no operations.
- `check_alphanumeric_status`: Checks if a byte is a letter or a specific symbol and returns one if it matches, otherwise returns zero.
- `return_zero_5`: This routine performs no operations and returns a zero value to the caller.
- `parse_numeric_string`: Iteratively reads characters from a source and converts a sequence of digit characters into a single integer value.
- `check_punctuation_status`: Returns one if the input byte matches a carriage return or a period, otherwise returns zero.
- `return_zero_value_2`: The routine performs no operations and returns a zero value to its caller.
- `check_character_match`: Compares the second byte of an input parameter against a specific character constant.
- `validate_character_properties`: Compares the second byte of an input parameter against a specific character constant and checks bit flags in a global table.
- `check_specific_byte_range`: Returns one if the input byte matches one of four specific values, otherwise returns zero.
- `return_zero_6`: This routine is empty and returns zero to the caller.
- `calculate_wrapped_distance`: Calculates a distance or offset relative to a global value, incorporating modulo arithmetic for wrapping behavior.
- `append_to_buffer`: Shifts elements in a global array to make room at a specific index, then stores the input byte at that index.
- `process_buffer_segments`: Iterates through a global list to find specific character properties and shifts elements in a global array to insert new data.
- `calculate_coordinates`: Calculates two values based on a comparison between a parameter and a value from a global table, then stores the results in two provided pointers.
- `shift_data_left`: Shifts a sequence of bytes backward through a memory range and stores a new byte at the end of the sequence.
- `copy_byte_from_list`: Copies a byte from a source list into a global variable and increments a pointer.
- `render_scaled_list`: Iterates through a list of values from a global table and passes them to another routine for scaled rendering.
- `call_subroutine_with_argument`: Calls another routine with a constant value of seven passed as an argument.
- `render_scaled_text`: Calculates offsets and scales coordinates before passing them to a graphics library function.
- `iterate_and_call_subroutines`: Executes a loop that calls two different routines repeatedly based on input parameters and a global variable value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_no_operation`: An empty routine that performs a self-assignment of a value at a calculated memory location.
- `fetch_table_value`: Retrieves a 32-bit value from a table using two input parameters as indices.
- `fetch_masked_byte`: Retrieves a byte from a table using two input parameters as indices and applies a bitwise mask.
- `compare_to_constant_47`: Compares a 16-bit input value to a constant and returns the result of the equality check.
- `compare_byte_to_constant`: Checks if a specific byte within a provided data structure matches a fixed value.
- `check_range_31_to_42`: Returns one if the input value is between 31 and 42 inclusive, and zero otherwise.
- `check_range_43_to_46`: Returns one if the input value is between 43 and 46 inclusive, and zero otherwise.
- `check_range_1_to_12`: Returns one if the input value is between 1 and 12 inclusive, and zero otherwise.
- `check_range_inclusive`: Checks if a byte within a passed parameter falls between two specific values, returning one if it does and zero otherwise.
- `compare_byte_equality`: Compares a byte in a passed parameter to a specific value and returns a boolean result.
- `check_range_inclusive_large`: Tests if a byte in a parameter is within a specific range and returns one if the condition is met.
- `check_multiple_values`: Returns one if a byte in a parameter matches one of two specific values, otherwise returns zero.
- `compare_byte_equality_15`: Checks if a byte in a parameter is equal to a specific constant value.
- `compare_byte_equality_16`: Compares a byte from a parameter against a specific value and returns the result.
- `compare_byte_equality_17`: Returns a boolean indicating whether a byte in a parameter matches a specific value.
- `validate_byte_range`: Checks if the second byte of a provided word falls within a specific inclusive range.
- `compare_byte_equality_11`: Tests if a byte within a parameter is equal to a specific constant.
- `compare_byte_equality_13`: Checks if a byte in a parameter matches a specific value and returns the result.
- `compare_byte_equality_12`: Compares a byte in a parameter to a specific value and returns a boolean.
- `compare_byte_to_constant_2`: Compares a specific byte within a passed parameter to a constant value.
- `return_zero_constant_2`: An empty routine that returns zero.
- `compare_byte_a8`: Compares the second byte of the input parameter to a specific constant and returns the result.
- `compare_byte_a9`: Checks if the second byte of the input parameter matches a specific constant.
- `compare_byte_aa`: Tests whether the second byte of the input parameter is equal to a specific value.
- `check_byte_range`: Returns one if the second byte of the input parameter falls within a specific range, otherwise returns zero.
- `check_byte_range_extended`: Checks if the second byte of the input parameter is between two specific values and returns the result.
- `verify_triple_condition`: Checks if three consecutive calls to another routine all return zero.
- `return_zero_value_3`: This routine returns a zero value to its caller.
- `validate_multiple_conditions`: Performs a series of checks on a parameter and returns zero only if all specific byte comparisons and range validations pass.
- `verify_byte_sequence`: Checks a sequence of bytes against specific values and returns zero if all comparisons match.
- `check_multiple_conditions`: Performs a series of equality checks on various bytes and returns one based on whether all conditions are met.
- `verify_byte_ranges`: Checks if multiple bytes within a parameter fall within specific inclusive ranges.
- `check_two_ranges`: Verifies if two separate byte values fall within specified inclusive ranges.
- `check_dual_byte_ranges`: Verifies if two consecutive byte values fall within specific inclusive ranges and returns zero if both conditions are met.
- `verify_quadruple_condition`: Checks if four consecutive calls to another routine all return zero.
- `verify_double_condition`: Checks if two consecutive calls to another routine both return zero.
- `check_coordinate_bounds`: Verifies if two components of the input parameter are within specific non-negative bounds and returns the result.
- `reduce_value_by_subtraction`: Repeatedly subtracts the upper word of the input parameter from the lower word until the lower word is less than the upper word, returning the remainder in the lower word.
- `compute_sum_div_mod`: Adds the lower and upper words of the input parameter, then divides the sum by two, returning the remainder in the upper word and the quotient in the lower word.
- `compute_product_div_mod`: Multiplies the lower and upper words of the first parameter, then divides the product by the lower word of the second parameter, returning the remainder in the upper word and the quotient in the lower word.
- `dispatch_structure_permutations`: Uses a selector to pass different permutations of data from a structure to another routine.
- `process_coordinate_triplets`: Extracts and passes two sets of three-word sequences from a pointer-based structure to another routine.
- `process_coordinate_permutations_a`: Passes two different combinations of three-word values from a structure to another routine.
- `dispatch_interpolated_structure_data`: Retrieves three distinct values from a nested data structure via a pointer and passes them as arguments to another routine in two different sequences.
- `dispatch_interpolated_coordinate_data`: Extracts three values from a structure pointed to by an offset within another structure and passes them to a routine.
- `dispatch_structure_fields`: Retrieves a pointer from an offset within a structure and passes the values stored at four subsequent offsets within that pointed-to structure to another routine.
- `pass_structure_data`: Loads a pointer from a structure and passes its contents or associated data to another routine.
- `calculate_grid_coordinates`: Calculates a quotient and remainder based on the difference between a constant and the lower word of the input divided by a global variable plus two, ensuring the quotient is at least two.
- `clamp_and_decrement_value`: Subtracts specific constants from a parameter if it exceeds thresholds, then decrements the result by one before returning.
- `perform_math_operations`: Executes a sequence of subtraction and complex division-based calculations.
- `calculate_offset_value_2`: Calculates a value using a combination of multiplication, division, and pointer offsets, then stores the result in a memory location.
- `fetch_memory_word`: Calculates an offset and stores a word from a table into a memory location.
- `add_divided_value`: Calculates a value by adding a quotient to a word retrieved from a table and stores it in a memory location.
- `compute_remainder_difference`: Calculates a value based on the difference between a sum and a quotient, then returns a concatenated result containing a remainder.
- `compute_complex_value`: Calculates a new value using division and addition, storing it in a memory location, and returns a combined value containing a remainder.
- `calculate_modular_offset`: Calculates a difference between two 16-bit values from the first parameter and multiplies it by a value from the second parameter.
- `populate_and_process_index_buffer`: Iterates through ranges derived from global table values to populate a local stack buffer with indices.
- `search_table_entries`: Iterates through a global table, validating entries against multiple range checks and conditional logic, then stores indices of valid entries into three output pointers.
- `branch_on_nested_validation`: Performs a series of nested conditional checks based on the return values of multiple sequential routine calls.
- `interpolate_buffer_values`: Calculates a series of intermediate values using linear interpolation and passes them, along with a modified pointer, to another routine within a loop.
- `interpolate_coordinate_sequence`: Iteratively calculates and passes interpolated coordinate values to another routine.
- `read_four_bytes`: Loops four times to read bytes from a pointer, returning the value from the final read operation.
- `load_terrain_map`: Reads bytes from a source pointer into a structured global buffer, performing range checks and handling read failures.
- `report_terrain_map_read_failure`: Handles a failure to read a terrain map by logging the specific error message with the map index.
- `fetch_masked_coordinates`: Retrieves masked byte values from a table using indices and stores them into four separate output pointers.
- `clamp_scaled_product`: This routine multiplies two 16-bit halves of a 32-bit parameter, subtracts one, and clamps the result between 1 and 4.
- `copy_grid_to_buffer`: Copies a 5x5 grid of bytes from a source pointer into a global buffer, then conditionally sets a specific entry in that buffer to one based on scaled calculations.
- `dispatch_grid_buffer_copy`: Compares a byte parameter against several constant values and calls another routine with different addresses based on the match.
- `switch_on_stack_value`: Evaluates a 16-bit value from the stack in a switch statement with cases ranging from 30 to 34, then calls two other routines sequentially.
- `compute_modular_offset`: This routine calculates a difference between two 16-bit values from the first parameter, multiplies it by a value from the second parameter, and then computes a modular arithmetic result.
- `compute_interpolated_line_params`: Calculates intermediate values by calling a routine to generate interpolated points and another to compute modular offsets, then adjusts these values using division and addition.
- `conditional_pointer_increment`: This routine conditionally adds a 16-bit value to four different short pointers based on the value of a 16-bit field in a fifth parameter.
- `compute_scaled_remainder`: Determines a multiplier based on a parameter value, fetches a value from a table, and returns a 32-bit value containing a remainder and a quotient-based offset.
- `clamp_polygon_edges`: Checks a parameter against several specific constant values to determine which pairs of coordinates to process, then calls a line-clamping routine and an interpolation routine.
- `dispatch_parameter_values`: Calculates a multiplier using a scaled remainder routine and then passes four 16-bit values from a memory location to another routine.
- `pass_memory_values`: Fetches four 16-bit values from offsets of a pointer and passes them to another routine.
- `call_other_routine`: Passes four 16-bit values from a memory location to another routine.
- `adjust_line_coordinates`: Computes new coordinate values by calling a routine to calculate modular offsets and adding scaled parameters, then passes these adjusted values to another routine.
- `generate_interpolated_line_points`: Calculates a series of intermediate points between start and end positions by calling routines to compute modular offsets and scale parameters.
- `process_scaled_coordinates`: Performs several calculations involving scaled remainders and modular offsets, then passes various derived values to multiple other routines.
- `execute_conditional_drawing`: Calculates scaled values and uses a parameter to choose between different sets of calls that pass modified coordinates and parameters to other routines.
- `rasterize_interpolated_polygon`: Generates a grid of interpolated 2D coordinates within a 6x6 bounding area and maps them onto a surface defined by a transformation table.
- `dispatch_conditional_logic`: Performs a series of conditional checks and nested routine calls based on input parameters and values retrieved from a table.
- `compute_position_from_nibbles`: Calculates a position offset by combining a base value with scaled inputs, retrieves lower nibbles from a global table, and classifies four 16-bit values for equality.
- `classify_four_values`: This routine compares four 16-bit values from two 32-bit parameters to determine their equality relationships.
- `compute_absolute_difference`: Calculates the absolute value of an integer input, likely used for distance or difference metrics, as indicated by its call to a signed absolute value routine.
- `process_unit_position`: Computes a position from nibbles and checks a condition; if true, it dispatches unit action logic using computed coordinates and parameters.
- `iterate_coordinate_grid`: Calculates a starting coordinate based on input parameters and iterates through a grid, calling another routine for each step.
- `process_coordinate_loop_2`: Performs arithmetic on values from a stack frame to calculate offsets and iterates through a loop calling another routine.
- `process_coordinate_loop`: Calculates a series of coordinates and iterates through them to perform actions via another routine.
- `test_boundary_conditions`: Compares two parameters against specific numeric thresholds and returns one if certain conditions are met, otherwise returns zero.
- `return_zero_value_4`: The routine performs no operations and returns a value of zero to the caller.
- `perform_recursive_calculation`: Performs a series of recursive calls with arguments derived from a global variable and various arithmetic transformations.
- `trigger_graphics_update`: Calls another routine passing a global variable and the constant 1 as arguments, likely to initiate a graphics update or dispatch operation.
- `process_coordinate_buffer`: Retrieves masked values from a table using indices and processes them through a loop that calls another routine with a descending counter.
- `process_coordinate_stack`: Retrieves values from a stack-based structure and iterates through a loop to call another routine with varying parameters.
- `process_buffer_with_indices`: Calculates a new pointer, stores a value, and enters a loop that calls another routine with a descending counter and calculated parameters.
- `iterate_grid_values`: Performs a loop that calls another routine twice per iteration using a descending counter and values from a table.
- `iterate_grid_with_counter`: Iterates through a coordinate grid twice per loop using two different values for a global variable.
- `update_global_table`: This routine iterates 19 times, updating a global table by performing division and subtraction operations on values from the table and a global variable.
- `copy_string_to_buffer`: Copies characters from a source string to a global buffer, optionally converting them to uppercase, and appends a space character at the end.
- `append_uppercase_string`: Calls a helper routine to copy a source string into a global buffer, converting characters to uppercase and appending a space.
- `reset_global_flags`: Sets one global variable to zero and another global variable to one.
- `execute_conditional_sequence`: Executes a sequence of several routine calls and then either calls another routine or a library function based on a comparison of a global variable against a constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `scale_coordinates_and_draw_string`: Shifts two input parameters left by three bits and passes them to a graphics library function, then configures graphics state and renders a string.
- `invoke_draw_call`: Calls another routine to perform a graphics drawing operation using global state parameters.
- `modify_global_pointer`: Performs several operations including passing a pointer to a global variable to another routine.
- `display_austerlitz_intro_text`: Renders the introductory text string 'Even before the Battle of Austerlitz.' to the screen.
- `invoke_library_vector_loop`: Loops sixteen times, calling a library vector routine with parameters derived from a global table and the loop index.
- `open_graphics_libraries`: Loads the graphics and intuition libraries, displaying error messages if they fail to open or if the screen cannot be accessed.
- `call_graphics_draw`: Invokes a graphics library function using a global base pointer and a single integer parameter passed from the caller.
- `validate_and_dispatch`: Checks if two integer parameters fall within specific positive ranges before invoking an indirect library function with those parameters.
- `check_coordinate_bounds_2`: Checks two 16-bit values within a 32-bit parameter against specific constants and sets bits in a return byte if the values fall outside these ranges.
- `clamp_line_to_bounds`: Iteratively adjusts the coordinates of a line segment to fit within specific screen boundaries by calculating intersection points with edges.
- `call_line_clamping`: The routine passes its two parameters to another routine responsible for adjusting line segment coordinates to fit within screen boundaries.
- `decrement_and_dispatch_graphics`: Decrements a global counter and invokes graphics library functions using parameters derived from input registers and global state.
- `dispatch_parameterized_calls`: Validates two integer parameters against specific ranges and then invokes a routine using a value derived from the sum or difference of two short values retrieved from a structure.
- `dispatch_offset_values`: Retrieves two short values from a pointer-based structure and invokes another routine using their sum or their difference.
- `dispatch_delay`: Multiplies an input parameter by a constant and passes the result to a routine that invokes a delay function.
- `scale_and_draw`: Shifts two input parameters left by three bits and passes them to a graphics library function via a global base pointer.
- `setup_and_print_string`: Configures a graphics state, calculates the length of a zero-terminated string, and invokes library functions to render the string.
- `setup_graphics_library_calls`: Initializes graphics library pointers and executes a sequence of indirect calls to library functions.
- `return_to_caller`: This routine performs no operations and immediately returns to the calling function.
- `get_absolute_value`: Returns the absolute value of the lower 16-bit word of a 32-bit parameter.
- `get_maximum_value`: Compares two 16-bit values and returns the larger of the two.
- `get_maximum_value_alt`: Compares two 16-bit values and returns the larger of the two, specifically updating the second value if it is smaller than the first.
- `return_from_subroutine_2`: This routine performs a single return instruction to exit the current function.
- `return_from_subroutine_3`: The routine performs no operations and immediately returns to the caller.
- `store_value_in_ring_buffer`: Stores a 16-bit value into a buffer and increments a global index, resetting the index to a base address if a limit is reached.
- `decrement_and_fetch_global`: Decrements a global variable and returns a value from the memory location it previously pointed to, resetting the global to a constant if it reaches a specific state.
- `return_zero_7`: The routine is empty and simply returns zero.
- `check_ring_buffer_limit`: Checks a value at an offset within a structure against a limit and calls another routine if certain conditions are met.
- `accumulate_and_branch`: Adds two values from a structure to a global variable and calls another routine if a specific field matches one of two constant values.
- `execute_recursive_call`: The routine performs a direct call to itself before returning.
- `check_structure_and_branch`: Checks a byte at an offset within a structure and calls another routine if the byte matches one of two specific values, provided a global variable is zero.
- `initialize_input_device`: The routine allocates a data structure, initializes its fields with constants and pointers, and invokes a sequence of helper routines to set up the input.device interface.
- `convert_to_uppercase`: The routine checks if an input byte represents a lowercase ASCII letter; if so, it subtracts a constant to convert it to uppercase, otherwise it returns the byte unchanged.
- `convert_to_lowercase`: The routine checks if an input byte represents an uppercase ASCII letter; if so, it adds a constant to convert it to lowercase, otherwise it returns the byte unchanged.
- `increment_pointer_and_fetch_byte`: Compares a value at a pointer to a second value in a list.
- `update_file_status`: Checks bit flags in a structure, iterates through a table of objects, and updates a pointer and status flags based on the result of another routine.
- `search_table_entry`: Calls another routine to find a value and then uses that value to perform a search within a table.
- `search_string_table_3`: Iterates through a table of strings to find a match and stores a result in a provided structure.
- `update_object_flags_wrapper`: Stores a parameter into a global variable and calls a helper routine to read a byte from an offset, passing the result to a flag-updating routine.
- `initialize_manx_object_2`: Interacts with a library to allocate an object and performs conditional calls based on the state of a global variable.
- `nop_operation`: The routine is empty and performs no operations.
- `initialize_manx_object`: Allocates a new object structure and writes the magic identifier 'MANX' into it, configuring internal pointers and flags.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `tokenize_string_into_array`: Parses a null-terminated string into individual tokens by skipping whitespace and handling quoted substrings.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: The routine copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `recursive_call_3`: The routine calls another routine with the same argument it received and then returns.
- `manage_file_descriptor_table`: Iterates through a table of file descriptors to find an empty entry or a specific status.
- `perform_file_read`: A routine consisting of 42 instructions that validates a memory block and performs a file read operation.
- `copy_string_with_limit_2`: Calculates the length of a source string and copies bytes to a destination until a null terminator is reached or a maximum count is exceeded.
- `copy_string_with_limit`: The routine copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches -1.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `copy_string_to_buffer_2`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: The routine takes two 32-bit values from registers and returns a 64-bit product formed from the high and low 16-bit halves of each operand.
- `load_window_data`: Uses library functions to open a file and retrieve specific data into a structure, involving indirect calls to library vectors and storing results in a parameter-provided buffer.
- `store_byte_increment_pointer_2`: Stores a pointer from a parameter into a global variable.
- `store_byte_increment_pointer`: Stores a byte from a parameter into the address held in a global pointer variable.
- `invoke_flag_update_handler_2`: Executes a function pointer retrieved from a data table to update object flags, passing a parameter and stack address to the handler.
- `divide_and_concatenate_hex`: Performs 32-bit integer division and concatenates results, referencing hexadecimal digit text for formatting.
- `parse_format_string`: Parses a format string containing specifiers to convert various data types into a character buffer.
- `perform_signed_division`: Reads two signed inputs, determines the sign of the result, calls another routine to perform division, and returns a 32-bit value combining the result and the second input.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original contents of a register with the value left in that register by the call.
- `recursive_call_2`: Calls itself recursively without performing any other operation.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length`: The routine receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `invoke_flag_update_handler`: Executes a function pointer retrieved from a data table to update object flags.
- `execute_unspecified_logic`: A routine consisting of twenty-one instructions that is called by three different routines and performs calls to routines that manage buffers and store incremented bytes.
- `jump_to_execute_unspecified_logic`: Jump stub: passes control straight on to execute_unspecified_logic.
- `store_incremented_byte`: An unnamed routine that is called by other routines to perform data storage or buffer updates.
- `increment_and_store_byte`: Increments a pointer stored in a global variable and stores a byte from an offset of another structure into the location pointed to by that pointer.
- `iterate_and_call_loop`: The routine iterates through a memory range starting at a global variable address, calling another routine with the current address in each step.
- `clear_nested_structure`: Recursively processes a structure based on bit flags, calls other routines to handle nested elements, and finally clears the fields of the structure.
- `update_file_buffer`: Manages a buffer by checking status flags, updating pointers, and performing file write operations.
- `find_empty_table_entry`: The routine iterates through a table of four-byte elements, checking a specific byte offset within each element for a null value.
- `update_structure_status`: Updates fields within a structure based on the result of a table lookup, including setting specific bit flags and pointer values.
- `traverse_and_call_list_2`: Traverses a linked list starting from a global variable, calling another routine for each node with an offset, and clears the global variable upon completion.
- `traverse_and_call_list`: The routine traverses a linked list by following pointers stored in a register, calling another routine with the current node and an offset in each step.
- `insert_into_linked_list`: The routine updates a global variable with a new node address and inserts a provided value into a structure, then returns a pointer to a subsequent memory location.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_list_node`: The routine searches a linked list for a specific node, unlinks it from the list if found, and then calls another routine with the removed node and an offset.
- `check_table_entry_validity`: The routine verifies if an index is within the bounds of a global table and checks if the entry contains a non-zero value.
- `delete_file_with_error_check`: The routine attempts to delete a file and, if the operation fails, retrieves an error code from a library and updates a global variable.
- `perform_file_write`: A routine consisting of 41 instructions that validates a memory block and performs a file write operation.
- `validate_memory_block`: Checks a memory block for a specific bit flag and executes another routine if a global variable is non-zero.
- `process_global_data_block`: Passes a global variable and a constant size to another routine, then passes the returned value to a second routine before calling a third routine.
- `call_function_pointer`: Checks if a global function pointer is non-null and invokes it.
- `process_global_table_entries`: Iterates through a global table, calls various library functions and other routines, and performs conditional stack-based operations based on a flag in a global variable.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_table_entry`: The routine validates an index against a global table, checks a bit flag in the entry, and calls another routine to close a resource.
- `call_close_2`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay_2`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file_2`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_input_2`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock_2`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open_2`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output_2`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read_2`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_lock_2`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write_2`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_function_offset_414`: The routine performs an indirect call to a function located at a specific offset from a base address stored in a global variable.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_and_initialize_node`: The routine performs several library calls to manage memory and then populates a structure with a provided integer, a byte, and other values before updating a pointer.
- `reset_object_state`: Recursively processes a linked list if a specific field is non-zero, sets a byte flag, clears a four-byte field, and calls two other routines to finalize the state.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_subroutine_with_offset`: Passes the input parameter and a fixed constant to another routine.
- `call_recursive_wrapper`: The routine invokes another routine using the parameter passed to it.
- `assign_structure_fields`: The routine calls another routine to obtain a pointer and then stores a byte, a word, and an integer into that pointer at specific offsets.
- `initialize_structure_values_2`: Sets a byte at an offset to one and two dwords to all ones within a structure, then calls another routine.
- `initialize_structure_values`: Sets a byte at an offset to one and two dwords to all ones within a structure.
- `execute_indirect_function_6`: Loads a function pointer from a global variable with a negative offset and executes it.
- `execute_indirect_function_4`: Loads a function pointer from an offset within a global variable and jumps to it.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `execute_table_function`: Fetches and executes a function pointer from a table using an offset from a global variable.
- `execute_indirect_function_5`: Fetches a function pointer from a table using an offset from a global variable and executes it.
- `call_library_function`: Calls a function from a library using an address stored in a global variable at a specific offset.
- `call_library_function_2`: Loads a library base from a global variable and calls the function at a specific offset.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `update_linked_list_node`: Stores the address of a pointer argument into the memory location it points to, increments that stored address by four, clears the subsequent word, and stores the original pointer address in the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_function_3`: Executes a library function by dereferencing a pointer located at an offset within a global variable.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_function_4`: Performs an indirect call to a library function using an offset from a global variable.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_function_5`: Calls a library routine via a pointer stored in a global variable at a specific offset.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_function_6`: Uses a global variable to find and execute a library function at a specific offset.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_function_7`: Calls a function from a library by accessing a pointer at a fixed offset from a global variable.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_function_8`: Executes a library function through an indirect pointer stored in a global variable.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_function_9`: Calls a library function using an address retrieved from a global variable at a specific offset.
- `execute_indirect_call`: Performs an indirect call to a function pointer stored at an offset within a global variable.
- `execute_indirect_call_2`: Loads a function pointer from a global variable and jumps to that address.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_4`: This routine performs no operations and returns immediately to its caller.
- `return_no_operation`: The routine consists of a single instruction that returns immediately, so it performs no work and has no side effects.
- `return_no_op`: The routine consists of a single instruction that returns immediately, so it performs no work.
- `return_immediately_5`: This is an empty routine that performs no operations and returns to the caller immediately.
- `return_from_subroutine_6`: This is an empty routine that performs a return.

### logged disk load from Disk.1

- `process_data_block`: A routine consisting of ninety-eight instructions that is executed during runtime.

### logged disk load from Disk.1

- `process_data_block_2`: A routine consisting of twenty-nine instructions that is executed during runtime.
- `process_data_block_3`: A routine consisting of twenty-six instructions that is executed during runtime.
- `execute_unknown_routine`: A routine consisting of fifty-eight instructions that is entered from outside the static flow.

### logged disk load from Disk.1

- `execute_unknown_routine_2`: A routine consisting of thirty-two instructions that is entered from outside the static flow.

### logged disk load from Disk.1

- `execute_unknown_routine_3`: A routine consisting of one hundred and thirty-seven instructions that is entered from outside the static flow.
