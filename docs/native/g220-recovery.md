# Game 220 (Turbo Cup): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00220). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 57 of 57 named (18 author symbols), 57 of 57 described (8 by rule, 48 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1031 of 1031 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2031 of 2031 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 454 |
| code | 6442 |
| data | 8846 |
| padding | 642 |
| unused | 884736 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Turbo Cup.adf:re | 3 | 1813 | 54 |
| Turbo Cup.adf:bootblock | 1 | 144 | 3 |
| logged disk load from Turbo Cup.adf boot-block load | 1 | 74 | 0 |

## Cases handled

- Turbo Cup/Turbo Cup.adf: the boot block reads 99328 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- Turbo Cup/Turbo Cup.adf: 1 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- `_main` (re): Allocates memory, creates communication ports and standard I/O structures, calls boot preparation and disk routines, and then cleans up the allocated resources. Kind: disk/port integrity check. Pass path: the routine continues to u0___prepare_boot() and u0___put_boot() if the disk port/IO creation succeeds.
- `_put_boot` (re): Manages disk device operations by opening and closing devices and performing input/output requests using a disk request structure. Kind: disk/track check. Pass path: The branch where _u0___error < 1 is false (the 'else' block) is the error path; the 'if' block containing the trackdisk.device I/O is the attempt to read/verify the disk. A successful check (error == 0) proceeds to the disk I/O loop and sub.

## Functions


### Turbo Cup.adf:re

- `initialize_application_environment`: Retrieves the current task and initializes global structures by calling various setup routines.
- `execute_callback_and_cleanup`: Calls a function pointer from a structure, then executes several other function pointers stored at various offsets within a global structure.
- `call_two_table_entries`: Calls two function pointers read from a table at fixed offsets from a base register, in sequence, then returns.
- `execute_callback_and_store`: Loads a pointer from a global structure, calls a function at that pointer, stores the return value in a global variable, and branches based on the result.
- `_main`: Allocates memory, creates communication ports and standard I/O structures, calls boot preparation and disk routines, and then cleans up the allocated resources.
- `_put_boot`: Manages disk device operations by opening and closing devices and performing input/output requests using a disk request structure.
- `_prepare_boot`: Iterates through a data table and stores the result of a transformation routine into a buffer.
- `_INVERSE`: Returns the bitwise logical NOT of the input parameter.
- `call_global_library_function`: Calls one library function through a library base read from a global variable at a fixed offset, passing no arguments, and returns.
- `validate_index_range`: Checks if a provided index is within the bounds of a table and returns either the calculated offset or a specific error value.
- `initialize_and_call_library`: The routine checks if a global variable is zero, then calls a library function through a pointer and stores the result in that variable.
- `traverse_and_process_list`: The routine traverses a linked list of pointers, checking specific bits in a flag byte for each entry.
- `forward_list_insertion`: Acts as a thin wrapper that passes its argument directly to another routine and returns.
- `manage_sorted_list`: Performs complex operations involving memory allocation, linked list insertion, signed 32-bit integer division, and 32-bit word multiplication.
- `check_and_forward_call`: Calls a routine with the first argument, and if the result is non-zero calls a second routine using a field of that result with the other two arguments, returning all bits set if a global flag is non-zero.
- `allocate_and_link_node`: Allocates a memory block using a library call and, if successful, stores a pointer into the new block and calls another routine to link it into a list.
- `negate_zero_return`: Calls another routine with an argument and returns that value, or returns negative one if the result was zero.
- `free_linked_list_memory`: Traverses a linked list and calls a memory free function for each node before clearing the list pointers.
- `update_linked_list_node`: Copies a value from a source pointer into a destination structure, clears the destination's head word, links the destination after the source's tail, and points the source's tail at the destination.
- `call_indirect_with_flag_check`: The routine calls another routine with the first argument, and if the result is non-zero it calls a second routine using a value taken from the result plus an offset together with the second and third arguments.
- `insert_into_sorted_list`: Inserts a new node into a sorted linked list while updating a global size counter and managing node connections.
- `append_string_with_limit`: Appends characters from a source string to the end of a destination string, copying at most the caller's limit and never more than the source length.
- `dispatch_action_by_object_flag`: Looks up an object by identifier, and if its flag byte has bit three set calls a routine with fixed arguments.
- `enqueue_command_string`: Iterates through a null-terminated string and copies each character into a circular buffer.
- `process_object_state_transitions`: Manages an object's internal state and status flags by recursively processing an event queue and updating counters.
- `process_object_event_queue`: The routine processes an event or command queue associated with a structure, updating internal counters and status flags based on the input parameter.
- `update_object_list_link`: The routine checks a field and a flag within a structure, then calls another routine to update several fields in the structure based on the result.
- `parse_command_string`: Parses a space-delimited or quoted string into an array of pointers.
- `divide_signed_32bit_integers`: Performs signed division of two 32-bit integers by normalizing the divisor and calculating the quotient and remainder.
- `normalize_and_divide_16bit`: Normalizes a 16-bit divisor by shifting it left until its top bit is set, tracking the shift count, then divides a 32-bit dividend by it and corrects the quotient and remainder.
- `call_library_functions`: Loads a pointer from a memory location into a stack pointer, calls a subroutine, pushes a constant onto the stack, and calls another subroutine.
- `multiply_32bit_words`: Multiplies two 32-bit values taken from the data registers using 16-bit partial products, adding the two cross terms to the low-word product.
- `iterate_and_execute_callbacks`: The routine iterates through a table of pointers, checking a bit in each entry before calling another routine with the pointer's value.
- `check_signal_bits_and_handle`: Reads the signal state from the system library, masks out two flag bits, and returns zero when both are clear.
- `initialize_library_and_set_field`: Checks if a global variable is zero, and if so, calls a library function to populate it.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `handle_signal_error_type_five`: Checks a global flag, calls a function via a pointer, and if that function returns negative one, calls another function to update a global variable and sets a global value to five.
- `check_signal_and_update_state`: Checks the signal state via a library call and updates a global variable with the result.
- `handle_signal_with_math`: Checks a global flag, calls a function via a pointer, and performs arithmetic on provided parameters based on the function's return value or specific input constants.
- `call_and_discard`: Checks a global flag and then calls a function via a pointer before returning zero.
- `_NewList`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `_CreatePort`: Obtains a slot index from an unnamed routine and, when valid, allocates a 34-byte block with flags 65537.
- `_DeletePort`: Recursively releases a linked chain of records, processing the linked record first when its link field is set.
- `_CreateStdIO`: Passes its one argument to another routine together with the constant 48.
- `_DeleteStdIO`: Passes its one argument on to another routine and returns nothing.
- `_CreateExtIO`: Calls a library-backed allocation routine with a second argument, and if it returns a non-null block, stores a byte value, a word from the second argument, and the first argument pointer into fixed offsets of that block.
- `_DeleteExtIO`: If the given pointer is non-null, it sets a byte field to 255 and clears two longword fields of the structure, then passes the pointer and a word field to another routine and returns its result.
- `_AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `_FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `_FindTask`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `_AllocSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `_FreeSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `_AddPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `_RemPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.

### Turbo Cup.adf:bootblock

- `initialize_boot_sequence`: A boot block routine that performs memory allocation, I/O operations, and superstate changes while interacting with CIAA and color registers.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `unpack_compressed_data`: Parses a custom structured data stream to perform memory operations including zeroing blocks, copying segments, and adding values from a table to a destination buffer.
