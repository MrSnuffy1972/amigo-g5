# Game 2349 (KingsQuest4 v1.1 0822): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02349). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (8121258 top-level bytes in 45 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 180 of 180 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 2351 of 2351 named (139 author symbols), 2351 of 2351 described (433 by rule, 1901 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1304 of 1304 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 31892 of 31892 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 270737 |
| code | 109192 |
| data | 7244231 |
| padding | 8650 |
| unused | 488448 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk.1:install | 3 | 4265 | 159 |
| Disk.1:endcli | 2 | 208 | 4 |
| Disk.1:prog | 62 | 24150 | 1692 |
| Disk.1:amigasnd.drv | 3 | 1250 | 103 |
| Disk.1:kq4 | 5 | 1101 | 67 |
| game.slave | 1 | 501 | 53 |
| Disk.1:bootblock | 1 | 10 | 1 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.3 | 1 | 0 | 0 |
| logged disk load from Disk.3 | 1 | 0 | 0 |
| logged disk load from Disk.3 | 1 | 0 | 0 |
| logged disk load from Disk.4 | 1 | 0 | 0 |
| logged disk load from Disk.4 | 1 | 0 | 0 |
| logged disk load from Disk.4 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 76 | 3 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 294 | 232 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 31 | 31 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |

## Cases handled

- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822_hdf/KingsQuest4/data/Disk.1:resource.000: high-entropy data file (7.81 bits/byte), classified as packed data
- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822_hdf/KingsQuest4/data/Disk.1:resource.001: high-entropy data file (7.90 bits/byte), classified as packed data
- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822_hdf/KingsQuest4/data/Disk.2:resource.000: high-entropy data file (7.81 bits/byte), classified as packed data
- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822_hdf/KingsQuest4/data/Disk.2:resource.002: high-entropy data file (7.90 bits/byte), classified as packed data
- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822_hdf/KingsQuest4/data/Disk.3:resource.000: high-entropy data file (7.81 bits/byte), classified as packed data
- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822_hdf/KingsQuest4/data/Disk.3:resource.003: high-entropy data file (7.92 bits/byte), classified as packed data
- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822_hdf/KingsQuest4/data/Disk.4:resource.000: high-entropy data file (7.81 bits/byte), classified as packed data
- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822_hdf/KingsQuest4/data/Disk.4:resource.004: high-entropy data file (7.86 bits/byte), classified as packed data
- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822.hdf: hard-disk image (AmigaDOS OFS, 8704 blocks): its filesystem parses and all 12 files in it are byte-identical to the extracted tree KingsQuest4_v1.1_0822_hdf, which is recovered file by file; no block outside the filesystem holds data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 170 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 220 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 22 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822_hdf/KingsQuest4/data/Disk.1: 161 logged loads cover 0 bytes of the image
- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822_hdf/KingsQuest4/data/Disk.2: 3 logged loads cover 0 bytes of the image
- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822_hdf/KingsQuest4/data/Disk.2: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822_hdf/KingsQuest4/data/Disk.3: 3 logged loads cover 0 bytes of the image
- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822_hdf/KingsQuest4/data/Disk.3: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822_hdf/KingsQuest4/data/Disk.4: 3 logged loads cover 0 bytes of the image
- KingsQuest4_v1.1_0822/KingsQuest4_v1.1_0822_hdf/KingsQuest4/data/Disk.4: boot area holds bytes but the checksum is invalid, so Kickstart never runs it

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Disk.1:install

- `__H0_org`: Jump stub: passes control straight on to _read.
- `_main`: Opens the graphics and intuition libraries, creates and centers a window on the screen, and displays text.
- `_abort`: Removes buttons, sets a raster, refreshes a window frame, sets a pen color, formats a string, moves a window, calculates a string length, displays text, delays, and sets up an exit.
- `_exit_setup`: Removes buttons, sets a rast value, refreshes a window frame, sets a pen color, formats a string, moves a window, prints a string of a specific length, delays, closes the window, closes two libraries, frees memory, and exits.
- `_check_for_abort`: Calls a message retrieval routine and, if a message is returned, calls a reply routine with data from the message before returning 1 or 0.
- `_check_for_gadget`: Waits for a port and calls a message retrieval routine, then calls a reply routine and returns a value from a table based on the message content.
- `_open_window`: Populates a structure on the stack with various parameters and calls a window opening routine.
- `_make_filename_button`: Opens a file, reads two null-terminated strings into buffers, allocates memory for two gadget structures and their string contents, calls a routine to create a new button gadget, and copies a string from a global variable into a newly allocated memory block.
- `_make_device_button`: Allocates memory for a gadget and a string, copies a string into the new memory, initializes gadget fields, calls a routine to create the gadget, and then stores a copy of the string in a global table.
- `_make_generic_button`: Allocates memory for a gadget and a string, copies a string into the new memory, initializes gadget fields, and calls a routine to create the gadget.
- `_remove_buttons`: Loops through a table of pointers and calls a gadget removal routine for each, then clears a counter in a global variable.
- `_get_button_filename`: Returns a longword from a table at an offset calculated from a 16-bit input value.
- `_get_button_devicename`: Takes a 16-bit value from a stack-based structure, shifts it left by two bits, and uses the result as an index into a global table to load a 32-bit value into d0 before returning.
- `_new_button_gadget`: Allocates memory for a gadget and its text, calculates dimensions using text length, populates a structure with specific attributes, calls another routine to add the gadget, and stores the resulting pointer in a table.
- `_set_copy_source_path`: Copies a string from a pointer passed in a register to a global variable using a string copy routine.
- `_set_copy_destination_path`: Copies a string from a pointer passed in a register to a different global variable using a string copy routine.
- `_do_copy`: Concatenates two strings to form file paths, opens the resulting files, and copies 4096-byte chunks in a loop; if the final read is less than 4096, it writes the remaining bytes before closing both files.
- `close_files_and_write`: If a value in a local variable is not 4096, it calls a write routine and then calls a close routine twice on two different file handles.
- `_copy_it`: Concatenates two strings to form source and destination file paths, opens both files, and transfers data in 4096-byte chunks using a buffer; if the final read is less than 4096 bytes, it writes the remaining bytes before closing both files.
- `_finddevs`: Opens a library, accesses a structure to find device information, and enters a loop that uses string operations and memory allocation to process device names and examine them, calling various library functions for memory management, locking, and error reporting throughout the process.
- `_matcher`: Locks a resource, allocates memory, and enters a loop that uses an examine routine to iterate through entries, calling a wildcard matching routine and a callback function for each match, while checking for an abort signal and managing memory and locks.
- `branch_and_move_long`: Branches if the carry bit is set, otherwise it moves a longword from a location offset from a4 into a location pointed to by a2.
- `_matchwild`: Compares two strings using case-insensitive matching and supports a wildcard character by iterating through the input strings.
- `jump_to__matchwild`: Jump stub: passes control straight on to _matchwild.
- `_MyAllocMem`: Calls a routine to check available memory, and if the requested size plus a constant is less than the available amount, calls another routine to allocate the memory, updates a linked list in a global variable, and returns the pointer.
- `_MyFreeAllMem`: Iterates through a linked list stored in a global variable and calls a library routine to free each block until the list is empty.
- `_MyFreeMem`: Searches a linked list stored in a global variable for a specific block and, if found, removes it from the list and calls a library routine to free the memory.
- `_strcpy`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `_strlen`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `_toupper`: Checks if an input byte is a lowercase ASCII letter and subtracts a constant to convert it to uppercase if it is.
- `_tolower`: Checks if an input byte is an uppercase ASCII letter and adds a constant to convert it to lowercase if it is.
- `_fgets`: Sets up a stack frame, calls a routine to get a character, and if the character is not 65535 and not 10, stores it in a buffer pointed to by a2 and increments the pointer; it also checks if the buffer reaches the end of a specified range and returns 0 if a condition is met.
- `_fscanf`: Stores a longword and a word into a structure at offsets minus 7d0c and minus 7d0e relative to a4, then calls another routine with a pointer to a value on the stack.
- `test_and_set_flag`: Checks a word at an offset of 8 from the stack frame; if not zero, it branches, otherwise it checks bit 3 of a byte at an offset of 12 from a pointer retrieved from an offset of negative 31980 from a4, and if that bit is set, it writes 65535 to an offset of negative 31982 from a4 before branching.
- `call_and_store_result_3`: Pushes a longword from an offset of negative 50852 from a4 onto the stack, calls a routine, stores the result in d0 into a 16-bit word at an offset of negative 50850 from a4, adjusts the stack pointer, and branches to another routine.
- `update_ungetc_value`: Calls a routine to push a character, stores the result in a 16-bit word at an offset of a global variable, and returns that value.
- `_scanfmt`: Parses a format string to extract data types and widths, then calls other routines to read integers, strings, or characters into a destination buffer.
- `_skipblank`: Calls a routine via a pointer in a global variable in a loop until a specific bit is not set in the character at the returned position, then calls the routine again and returns zero or negative one based on the result.
- `_getnum`: Reads a value from a source, calls another routine to process it, and updates a structure with the result, potentially performing multiplication and index lookups.
- `_agetc`: Calls a routine to get a character, and if the character is not 65535, it checks if it is 0 or 4; if it is 4, it decrements a longword at a pointer in a2 and sets bit 3 of a byte at offset 12 of that pointer before returning -1; otherwise it returns the character as a longword.
- `_getc`: Compares a value at a pointer in a2 with a value at an offset of 4 from that pointer; if the first is smaller, it increments the pointer and returns a byte from the original location, otherwise it calls another routine.
- `__fill`: Checks bits in a byte at offset 12 of a structure in a2, calls _getbuff if a pointer is null, potentially calls _flsh_ in a loop, and calls _read to update a pointer and a length field in the structure.
- `_fopen`: Calls _newstream, and if the result is not zero, calls _freopen using a pointer from a2 and arguments from the stack.
- `_freopen`: Calls a routine to close a file, then iterates through a list of strings to find a match; if a match is found, it calls a routine to open the file and updates a structure with the new file descriptor and a status byte.
- `_fprintf`: Pushes a value from the stack onto the stack and calls another routine via a PC-relative address.
- `call_aputc_via_pointer`: Loads a pointer from a global variable and a word from the stack, then calls a routine using those values.
- `_ungetc`: Compares a 16-bit value at offset 8 of a structure in a5 to 65535, and if not equal, moves a pointer from a2 to a0, compares a0 with a value at offset 8 of the pointer, and if a0 is not higher, writes the byte 255 to the address in a0.
- `write_byte_and_branch`: Decrements a long value pointed to by a2, moves the new value to a0, writes a byte from offset 9 of a5 to the address in a0, and branches to another location.
- `_index`: Scans a zero-terminated string from its start for the first occurrence of the given byte and returns its position, or zero when the terminator comes first.
- `_creat`: Pushes three values onto the stack and calls the table-searching routine.
- `_open`: Iterates through a list of file pointers, attempting to open files with specific flags, handling locks, deletions, and library operations, and returns a status code.
- `_read`: Calls a routine to check for aborts, calculates a table offset, and checks if a pointer is valid.
- `_geta4`: Loads a constant value into register a4 and returns.
- `__main`: Allocates a memory block based on a value from a global variable, searches for a task by a four-byte constant, and then conditionally calls routines to parse commands, wait for a port, get a message, change a directory, or perform input and output operations before calling another routine.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `__cli_parse`: Allocates memory for a new string by calculating a required size from a source buffer, copies the source string, and appends a suffix.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `_strcat`: Loads the word value 32767 into register D0 and then performs a short branch to a fixed address.
- `_strncat`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `_strncpy`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `_mulu`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `__wb_parse`: Opens a library, retrieves a disk object, finds a tool type, and opens files to populate a structure before freeing the disk object and closing the library.
- `copy_register_to_register`: Copies the long value from register D0 to register D5.
- `_sprintf`: Calls a formatting routine using parameters from a5, then clears a byte at a location stored in a global variable.
- `update_pointer_and_byte`: Increments a long value at a location relative to A4, then stores a byte from the stack into the address pointed to by the original value.
- `_printf`: Pushes a frame offset and a value from offset 8 of the frame onto the stack, then calls another routine via a PC-relative address, cleans up the stack, restores the frame and returns.
- `perform_complex_division`: Compares a value at offset 18 of a5 to 4, then performs a series of conditional operations involving modulo and division routines, potentially writing a byte to a buffer at a calculated address.
- `_format`: Parses a format string to convert various data types into a character buffer, supporting integer width and precision modifiers, floating-point precision, and string length limits.
- `_divs`: Reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `_mods`: Negates d0 if it is negative, negates d1 if it is negative, and calls a routine to perform 32-bit division.
- `_modu`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `_divu`: The routine calls another routine and then returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `_putchar`: Pushes a pointer from a global variable and a word from the stack onto the stack, then calls a routine.
- `_aputc`: Calls a routine to output a character, then checks if the character is a newline and if a specific bit is set in a byte from a structure; if both conditions are met, it calls a routine to flush the buffer.
- `jump_to__aputc`: Jump stub: passes control straight on to _aputc.
- `_putc`: Copies a byte from offset 9 of a structure in a5 to the address in a0, calls a flush routine, and returns.
- `increment_pointer_and_write`: Increments a long value at the address in a2, copies a byte from offset 9 of a5 to the address in a0, and branches to another location.
- `close_file_loop`: Calls a routine to close a file and loops while a pointer in a2 is less than a value derived from a4.
- `_fclose`: Checks a structure in a2, calls a flush routine if a bit is set, calls a close routine, and optionally calls free or unlink routines before clearing the structure's fields.
- `_flsh_`: Checks a status byte in a structure to see if it is already marked as error-state, otherwise it writes remaining buffered data to a file using a write routine and updates the structure's pointer and length fields.
- `_newstream`: Iterates through a table starting at a location relative to A4, checking a byte at offset 12 of each entry, and clears three long values at the entry if a condition is met.
- `_getbuff`: Allocates a 1024-byte block of memory, initializes a structure with the pointer and size, and sets a bit in a byte at offset 12 based on the result of a terminal check.
- `branch_to_middle`: Saves registers a2 through a3 on the stack, loads a value from a pointer at an offset of negative 31706 from a4 into a2, and branches to another routine.
- `free_linked_list`: Iterates through a linked list, calling a routine to free each node using a size stored at offset 4 plus 8, and clears a global variable when finished.
- `_lmalloc`: Calls a memory allocation routine with the value at offset 8 of a5 plus 8, then if successful, updates a global pointer and a linked list structure with the new address.
- `_malloc`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `_free`: Searches for a specific pointer within a linked list, updates the pointers of the surrounding nodes to remove it, and calls a routine to free the memory of the removed node.
- `_isatty`: Calculates an offset based on a value in a structure, checks if a pointer at that offset is non-zero, and calls another routine to determine if the device is interactive.
- `_unlink`: Calls a routine to delete a file using a pointer from the stack, and if the result is zero, calls an error handler and stores the error in a global variable.
- `_write`: Checks an abort condition, calculates an address based on a value in a5 and a global variable, and if certain conditions are met, calls a write routine with parameters from the stack or calls an error routine and updates a global variable.
- `_Chk_Abort`: Sets a signal using a routine, checks a bit in the return value, and if a specific global variable is non-zero, calls an abort routine.
- `__abort`: Passes the result of a function call to a write wrapper followed by an output wrapper.
- `_exit`: Checks if a pointer at a specific offset from a4 is non-zero, calls that routine if it exists, then calls another routine and returns.
- `__exit`: Closes files, frees memory, unlocks resources, closes multiple libraries, and handles system messages and further memory deallocation.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `_close`: Calculates a pointer to a structure using a value from the stack, checks if the index is within valid bounds and if the pointer is non-null, then either calls another routine to close the file or sets a global error value and returns -1.
- `_Close`: Jump stub: passes control straight on to __Close.
- `__Close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `_CreateDir`: Wrapper that calls the dos library function CreateDir and returns its result to the caller.
- `__CurrentDir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `_Delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `__DeleteFile`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `_Examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `_ExNext`: Wrapper that calls the dos library function ExNext and returns its result to the caller.
- `_Info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `__Input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `_IoErr`: Jump stub: passes control straight on to __IoErr.
- `__IoErr`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `__IsInteractive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `_Lock`: Jump stub: passes control straight on to __Lock.
- `__Lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `_Open`: Jump stub: passes control straight on to __Open.
- `__Open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `__Output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `_Read`: Jump stub: passes control straight on to __Read.
- `__Read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `__Seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `_UnLock`: Jump stub: passes control straight on to __UnLock.
- `__UnLock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `_Write`: Jump stub: passes control straight on to __Write.
- `__Write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `_Alert`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `_AvailMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `_CloseLibrary`: Jump stub: passes control straight on to __CloseLibrary.
- `__CloseLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `_AllocMem`: Jump stub: passes control straight on to __AllocMem.
- `__AllocMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `__FindTask`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `_Forbid`: Jump stub: passes control straight on to __Forbid.
- `__Forbid`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `_FreeMem`: Jump stub: passes control straight on to __FreeMem.
- `__FreeMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `_GetMsg`: Jump stub: passes control straight on to __GetMsg.
- `__GetMsg`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `_OpenLibrary`: Jump stub: passes control straight on to __OpenLibrary.
- `__OpenLibrary`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `_Permit`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `_ReplyMsg`: Jump stub: passes control straight on to __ReplyMsg.
- `__ReplyMsg`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `__SetSignal`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `_WaitPort`: Jump stub: passes control straight on to __WaitPort.
- `__WaitPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `_Move`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `_SetAPen`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `_SetRast`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `_Text`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `_FindToolType`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `_FreeDiskObject`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `_GetDiskObject`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `_AddGadget`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `_CloseWindow`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `_IntuiTextLength`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `_MoveWindow`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `_OpenWindow`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `_RefreshWindowFrame`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `_RefreshGadgets`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `_RemoveGadget`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `_SizeWindow`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.

### Disk.1:endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of register d1 to d0 and then branches to another location.
- `process_data_and_call_functions`: Performs several arithmetic operations on a pointer in a1, calls multiple functions via a5 with various arguments, clears a memory location based on a calculated index, and finally performs a decrement on a value in a table before jumping to an indirect address.
- `traverse_and_copy_linked_list`: Calls a function, copies a value from a table into a structure, and then enters a loop that follows a chain of pointers to copy subsequent values until a null pointer is encountered, finally jumping to an indirect address.

### Disk.1:prog

- `jump_to_entry_point`: Performs an indirect jump to a longword value.
- `process_linked_node_list`: Iterates through a linked list of structures, performing various conditional updates and bitwise manipulations on flags and values within each node.
- `test_and_move_pointer`: Tests a word at an offset from a5; if it is zero, it branches, otherwise it moves a pointer from a structure into a location at an offset from a5 and branches.
- `process_linked_list_nodes_5`: Iterates through a linked list of nodes, updating a global pointer and clearing specific fields in the node structures.
- `process_linked_list_nodes_4`: Iterates through a linked list of structures, updating node data and performing comparisons with values from a table.
- `update_object_node_state`: Checks specific bits in a status byte at offset 47 of a structure to increment a counter at offset 50 and clear bit 6.
- `process_object_node_list`: Iterates through a list of object nodes, updating their data from a table and calling several routines to validate, allocate, and link node properties.
- `process_object_list_recursively`: Iterates through a list of objects using a counter, copying data from a source structure to a destination structure.
- `update_object_node_data`: Calculates and passes three pointers to another routine by scaling word values from a table, sets bit 7 of a status byte in a structure, and copies a word from a structure to a destination address.
- `process_node_list_loop`: Decrements a 16-bit counter in a structure in a5 and loops to update a node's data from a table, calling several routines to perform data copying and bitmask updates based on bits in the structure.
- `process_linked_list_table_entries`: Checks if a pointer at offset 8 of a structure is null, otherwise it calls two routines to process data, clears a 16-bit value at offset 28 of the structure, and loads a 16-bit value from the address pointed to by offset 8 into offset 14 of the structure before branching.
- `populate_and_process_data_table`: Iterates through a linked list of pointers to populate two separate tables, then loops through the newly populated entries to perform a series of calculations and conditional updates to the structure's internal state.
- `iterate_table_entries_2`: The routine walks a table of entries indexed by a counter stored in a global variable, loading each entry and passing it through a series of other routines.
- `pop_and_test_pointer`: Pushes the value in d4 to the stack, tests a word at an offset of minus 10391 from a4, and if non-zero, restores d4 from the stack before returning; otherwise it restores d4 from the stack and returns.
- `load_and_branch_2`: Loads a pointer from a global variable into a0, moves a word from that address into d4, and branches to another location.
- `process_linked_list_nodes`: Iterates through a linked list by calling two routines and a third routine to process data for each node, continuing until a word at the start of the current pointer is zero, then clears the pointer in a global variable.
- `setup_stack_frame`: Initializes a stack frame and modifies a 16-bit value at a negative offset of a5 before branching to another location.
- `clear_structure_fields_and_branch`: Clears two 16-bit words at offsets -6 and -2 relative to a5 and performs a short branch to another routine.
- `sort_and_swap_data`: Compares values in two tables and swaps elements in a third table and two word tables if certain conditions are met, then loops until a counter reaches a limit.
- `decrement_and_branch_5`: Decrements a 16-bit value from offset 16 of a structure in a5 by one, stores the result at offset -2 of a5, and branches to another routine.
- `process_linked_list_nodes_3`: Iterates through a list of structures by following pointers, updates a value in each node based on an index, and conditionally calls another routine to handle specific flags or recursive data processing.
- `update_buffer_and_modify_flags`: Sets a byte to 1 in a destination buffer using an index and a pointer from a structure, clears a 16-bit word in a buffer at an offset derived from a global variable, and modifies specific bits in a status byte within the structure.
- `update_property_value_from_table`: Checks a status bit in a structure, modifies a 16-bit value by clearing bit 0 and setting bit 2, then stores this value into a table at an index derived from a global variable and a field in the structure before decrementing a counter and branching based on the result.
- `process_object_property_list`: Iterates through a list of entries in a structure, fetching values from a table and updating properties based on status bits.
- `update_object_property_list`: Iterates through a list of pointers using an index stored at offset -2 of a structure, fetches data from a table based on a global offset, and updates a property byte at offset -5.
- `update_object_property_list_entry`: Clears bit 7 of a status byte at offset -5 of a structure and fetches a 16-bit value from a table using an index.
- `process_table_entries`: The routine iterates through a table of entries, performing data transfers and conditional routine calls based on flags found within the table data.
- `execute_multiple_subroutines_2`: Pushes several values from a structure in a5 onto the stack to call various routines, then moves a word from a pointer retrieved from a global variable into offset minus 22 of a5.
- `execute_iterative_calls_2`: Performs a loop where it calls another routine using a word from offset minus 22 of a5, stores the result in offset minus 8 of a5, calls a second routine, and continues until a word at offset minus 16 of a5 is zero.
- `execute_iterative_calls`: Performs a loop where it calls another routine using a word from offset minus 22 of a5, stores the result in offset minus 8 of a5, calls a second routine using a pointer from offset 14 of the new pointer in a0, calls the first routine again, and updates offset minus 22 of a5 with a word from offset 2 of the result until the value at offset minus 22 is zero.
- `store_and_execute`: Pushes a word from a global variable to the stack, calls another routine, pushes a pointer from a structure in a5 to the stack, and calls a second routine before returning.
- `call_other_routine_with_constant`: Calls another routine with a constant argument and then returns.
- `call_subroutines_and_extend_word`: Pushes a pointer from offset -12 of a5 onto the stack and calls a subroutine, then pushes a longword from a global variable onto the stack and calls another subroutine, finally extends the word at offset 10 of a5 to a longword in d0 and branches to another routine.
- `call_subroutine_twice`: Pushes the word 1 and the word from offset 8 of a5 onto the stack and calls a subroutine, then clears the top of the stack, pushes the word from offset 8 of a5 onto the stack, calls a subroutine, and branches to another routine.
- `call_multiple_functions`: Pushes two words and two zeros onto the stack, calls a routine with a pointer to offset -8 of a5, then pushes a word from offset 8 of a5 and calls another routine with a pointer to offset -8 of a5.
- `branch_based_on_comparison`: Compares a longword in d0 to 18, and if the result is less than 18, it shifts d0 left by 1 and jumps to an offset determined by a word loaded from a table using the modified d0 as an index; otherwise, it jumps to a different offset.
- `call_routine_via_pointer`: Loads a pointer from a global variable and calls another routine with that pointer as an argument.
- `fetch_clamped_byte`: Compares a 16-bit value at offset 8 of a structure in a5 with a 16-bit value at a specific offset from a4, decrements the value in a5 if it exceeds the limit, and returns a byte from a table at an index that is clamped by a global limit.
- `compare_and_load_word`: Compares a word at offset 8 of a structure in a5 to a constant and, if it is not less than that constant, loads a word from a global variable into d0 before returning.
- `clear_and_branch_3`: Clears a word at an offset of minus 2 from a5 and branches to another location.
- `loop_and_increment_index`: Uses a word from the stack as an index to read a byte from a table, compares it to a value, and enters a loop that increments the index if the comparison fails.
- `calculate_and_store_loop`: Performs arithmetic using values from a structure, calls routines through pointers to perform multiplication and division, and enters a loop that stores results in a global table up to a maximum index of 14.
- `loop_and_write_byte`: Enters a loop that writes a byte from the stack into a table at an offset determined by another value from the stack, incrementing the index until it reaches a constant value.
- `call_conditional_helpers`: Calls a routine with arguments 1 and 5, and if the word at offset 10 of a5 is non-zero, calls a routine with arguments 190, 140, and 0, then calls a routine with a pointer to offset -8 of a5, and finally calls a routine with arguments 65535 and 5.
- `forward_argument_via_stack`: Pushes two words from a structure in a5 onto the stack, calls another routine, and then returns.
- `recursive_call_wrapper_2`: Sets up a stack frame, clears the stack, pushes a longword from a structure in a5 onto the stack, calls a routine, cleans the stack, and returns.
- `fill_byte_table_recursive`: Loads a byte from a structure, decrements a value derived from a word at an offset, and calls itself with that decremented value and a pointer from a structure.
- `fill_memory_with_word_2`: Loops through a memory range starting at the address in a2, storing a word constructed from two bytes at offset 15 of a structure at each location, and decrements a counter in d4 by half each iteration.
- `fill_memory_with_word`: The routine loops through a memory range starting at the address in A2, storing the word from D5 at each location, and decrements D4 as a counter until it reaches zero.
- `validate_and_update_state`: Checks a value in a local stack frame against a constant, then calls another routine and updates a structure via a pointer in a4 based on the result.
- `call_and_update_a4`: Pushes a longword onto the stack, calls another routine, adjusts the stack pointer, and if d0 is less than an address, writes three values to specific offsets in a4 and branches to another location.
- `execute_library_call_2`: If a value at offset -6048 of a4 is non-zero, pushes two values from offsets -6044 and -6048 of a4 onto the stack, calls a routine, clears the value at offset -6048 of a4, and then sets three specific values before branching.
- `execute_library_call`: Pushes a PC-relative address onto the stack and calls a routine, then pushes the pointer at offset -4 of a5 and a pointer from a global offset of a4 onto the stack to call another routine, and finally stores the result in a global variable at a negative offset of a4.
- `update_and_call_library`: Adds two values from a structure into a third location in that structure, calls another routine, pushes a constant onto the stack to call a library function, and if the returned d0 value is greater than or equal to 61766, stores the value 5 at offset -4 of a5; otherwise it stores the value 3 at offset -4 of a5.
- `execute_conditional_logic_6`: Pushes a value from a memory location onto the stack, calls another routine, compares the result to a constant, and if the result is greater than or equal to that constant, stores the value 3 at a negative offset from a5 and branches.
- `execute_conditional_library_call`: Tests a long value at a negative offset from a pointer in a4, and if it is non-zero, pushes two values from that same offset onto the stack, calls a routine via a pointer, cleans the stack, clears the long value, sets a value of 3 at a negative offset from a5, and branches.
- `update_global_pointers_2`: Calls two routines, stores a result in a global variable, performs arithmetic on a pointer derived from that result to update three other global variables, and calls a third routine before returning.
- `call_multiple_library_functions`: Calls a function retrieved from a table at offset minus 6060 of a4 with a constant value, then calls another function retrieved from a table at offset minus 6004 of a4 using a value retrieved from offset minus 5ff8 of a4.
- `update_global_values`: The routine calls a routine to fill a buffer, updates a global pointer, and performs arithmetic to store a calculated value in a global variable.
- `call_another_routine`: The routine calls another routine and then returns.
- `call_library_function`: Calls another routine and then returns.
- `check_heap_and_allocate`: Calls another routine and, if the result is zero, calls another routine with the constant 48571, then returns the result of the first call.
- `calculate_index_and_fetch_pointer`: Calculates a value based on a word from the stack, aligns it to an even number, and compares it to 6.
- `traverse_and_split_list`: Compares a word at offset 4 of a structure with d4, and if certain conditions are met, calculates an offset, stores a longword from a structure into a new location, and updates a word at offset 4 of that location.
- `update_global_and_memory_2`: Loads a word from offset 4 of a2 and a longword from a2 into d4 and d5, then if a3 is not zero, stores the longword from d5 into the address in a3.
- `update_global_and_memory`: Stores the value in D5 at an offset of minus 6828 from A4, stores the word from D4 at the address in A2, and returns the address in A2 incremented by two.
- `compare_and_branch_pointer_3`: Copies a pointer from a2 to a3, copies the longword at the address in a2 to a2, copies a2 to d0, and branches to another routine if d0 is not zero.
- `update_node_pointer`: Loads a pointer from an offset of the stack frame into a2, subtracts 2 from it, writes the word at the resulting address to an offset of 4 from that same address, and then loads a value from a global variable into d4 before branching to another routine.
- `update_list_node_2`: Copies d4 to d5, copies d4 to a0, loads a longword from the address in a0 into d4, and if d4 is not zero, compares d4 to a3 and enters a loop that continues as long as d4 is less than or equal to a3.
- `update_list_node`: Stores d4 into a location pointed to by a3, and if d5 is not zero, checks if a specific address matches a calculated value to either update a node's pointer and length or update a global pointer.
- `store_pointer_to_global_2`: Moves the address in d5 to a0 and stores the address in a3 into the location pointed to by a0, then branches to another routine.
- `store_pointer_to_global`: Stores the value in A3 at an offset of minus 6828 from A4.
- `update_node_list`: Calculates a new value based on a word at an offset of a pointer, writes this value to a local stack offset, and if the value is between 6 and 15, writes it to a calculated address and calls another routine.
- `store_pointer_and_clear`: Stores a pointer from a2 into an offset of a5, clears a longword at another offset of a5, loads a longword from a global variable into an offset of a5, and branches to another routine.
- `search_and_modify_list`: Traverses a linked list by comparing pointers, and if a condition involving a word at offset 4 and a negated word at offset 2 is met, updates a global pointer and modifies the list structure.
- `add_value_to_memory`: Loads a pointer from offset minus 6 of a5 into a0 and stores the value at that pointer into offset minus 6828 of a4, then loads a word from offset 4 of a structure pointed to by offset minus e of a5 and adds it to the value at a2.
- `clear_d0_and_branch_4`: Sets register d0 to zero and performs a short branch to another routine.
- `initialize_pointer_from_global`: Sets d5 to zero and loads the address from offset minus 6828 of a4 into a2, then branches to another routine.
- `find_maximum_in_linked_list`: Walks a singly linked list of 32-bit pointers, finds the largest 16-bit value stored at an offset of two bytes from each pointer, and returns that value minus two if it is non-zero.
- `setup_loop_parameters`: Sets up a stack frame, saves several registers to the stack, clears d5 and d6, loads a pointer from a global variable into a2, and branches to another routine.
- `loop_and_decrement_counter`: Loads a word from offset 4 of a2 into d4, adds d4 to d6, increments a3 by 6, decrements d4 by 6, and loops back to an earlier instruction.
- `loop_and_decrement_counter_2`: Loops through a memory range by storing d5 at the address in a3 and incrementing a3 by 2, decrementing d4 until it reaches zero, then updates a2 and d0 based on the contents of a2.
- `read_word_from_pointer_offset`: The routine receives a pointer in a register and returns the 16-bit word stored two bytes before the pointed-to address.
- `copy_pointer_to_global`: The routine copies a 32-bit value from a global variable at an offset of minus 6828 from A4 to another global variable at an offset of minus 61ec from A4.
- `remove_from_list_and_calculate`: The routine updates a global pointer, calculates the difference between two values, stores that difference in a specific offset of a node, and clears the first word of that node.
- `initialize_list_node`: The routine initializes a new node with zeroed words, sets a specific word to a provided value, and stores the node pointer and a calculated address in global variables.
- `call_and_branch_on_zero`: Calls another routine and, if the result is zero, calls another routine with a constant argument, then returns the result of the first call.
- `calculate_masked_address`: Adds 25 to a word from a pointer in a5, masks the result with a bitmask, stores it in a stack location, and moves a value from an offset of a4 into a2 before branching.
- `compare_and_call_routine_2`: Subtracts a longword at offset -14 of the stack frame from a longword at offset 4 of a2, stores the result at offset -10 of the stack frame, and if the result is not negative, calls another routine and stores the result at offset -18 of the stack frame.
- `copy_structure_data`: Compares a word at an offset of a5 to a constant, and if it is not less than that constant, calculates an address in a3, copies a longword to an offset of a3, copies a longword from a2 to a3, clears a word at an offset of a3, and branches to another routine.
- `update_structure_and_global_2`: Moves a longword from offset 4 of a2 to offset minus 14 of a5, sets a3 to the address in a2, and if a specific offset in a5 is not zero, stores a3 into the address pointed to by that offset.
- `update_structure_and_global`: Stores a3 into offset minus 61f0 of a4, clears the longword at a2, stores a word from offset minus e of a5 into offset 4 of a2, stores the word two into offset 8 of a2, and stores a pointer into offset 4 of a structure pointed to by offset minus 12 of a5.
- `process_a2_and_call`: Stores a pointer from a2 into a stack offset, updates a2 to point to the value at that pointer, and if d0 is not zero, checks a word at a stack offset; if that word is non-zero, it writes 1 to that stack offset, calls another routine, checks the same stack offset, and if it is zero, sets d0 to zero and branches to another location.
- `recursive_call_wrapper`: Receives a 32-bit value and immediately calls another routine with the same value, then returns.
- `fill_word_table_2`: Calculates a count by subtracting 10 from a value in a table and dividing by 2, then calls another routine to fill a table with a 16-bit value constructed from two bytes provided on the stack.
- `fill_word_table`: The routine copies a 16-bit value from a register to successive words in a table pointed to by a pointer, decrementing a counter each iteration until it reaches zero.
- `recursive_decrement_pointer`: Calculates a pointer by subtracting ten from the value at offset 8 of the structure in a5, calls another routine with that pointer, then calls another routine with the original pointer at offset 8 of a5.
- `initialize_linked_list_node_2`: Stores a word at a negative offset from a5, saves registers a2 and a3 to the stack, clears a word at offset 8 of the pointer in a2, clears a long at a negative offset from a5, loads a pointer from a negative offset from a4 into a3, and branches.
- `compare_and_call`: Compares two long values, and if they are not equal, calls a routine with a pointer from a constant address before branching to another location.
- `insert_into_sorted_linked_list`: Traverses a linked list to find the insertion point for a node, then updates the pointers and size field of the preceding node or the list head to maintain order.
- `update_linked_list_node_7`: Stores a value from a register into a location pointed to by a2, then checks a pointer at a negative offset from a5.
- `store_node_pointer_2`: Stores the value from a register into a location pointed to by a value stored at a negative offset from a5, then branches to another routine.
- `store_node_pointer`: The routine stores the value from a register into a global variable.
- `call_other_routine_with_stack_buffer`: Creates a four-byte stack buffer and passes its address to another routine, then returns.
- `shift_stack_value_right`: Calls another routine with the address of a stack variable and then returns the value stored at that stack location shifted right by four bits.
- `initialize_list_traversal`: Clears two registers and loads a pointer from a global variable into a2, then branches to another routine.
- `traverse_linked_list_accumulate`: The routine walks a singly linked list of integers via a pointer in a register, keeping the maximum element seen and the sum of the elements.
- `accumulate_linked_list_values`: Iterates through a linked list by comparing a pointer against a global limit, adding a 32-bit value from the current node to a running total if bit 0 of a byte at offset 9 is clear, and updates the pointer by adding a 32-bit value from offset 4.
- `return_value_from_stack`: The routine returns a 32-bit value stored at a negative offset from a register.
- `return_adjusted_value`: The routine reads an integer from a global variable pointed to by a table entry, subtracts a constant from it, and returns the result.
- `set_flag_byte`: The routine receives a pointer to a word and sets the least significant bit of the byte immediately before the pointed-to word.
- `clear_lowest_bit_of_word`: The routine receives a pointer in a parameter and clears the least-significant bit of the word located two bytes before the pointed-to address.
- `recursive_param_swap`: Loads a pointer from a location pointed to by a0, and if a value at a positive offset from a0 is zero, it calls another routine with a word from a negative offset from a0, then calls another routine using the result in d0 and the original pointer, and returns d0.
- `modify_and_copy_word`: Loads a pointer from a structure, copies a word from that pointer to the stack, and if a byte at offset 12 is non-zero, sets bit 7 of a byte at offset 11 on the stack.
- `clear_flag_and_copy_word`: The routine clears a bit in a word located at an offset from a register and copies that word to a location pointed to by another word at an offset from the same register.
- `compare_pointer_to_limit`: Clears a longword at offset -12 of a5, loads a pointer from offset -6004 of a4 into a0, and compares it to a value at offset -61f4 of a4, returning if the pointer is lower than the limit.
- `update_pointer_from_offset_3`: Checks bit 1 of a byte at offset 9 of a pointer retrieved from offset -8 of a5; if the bit is set, it adds a longword from offset 4 of that pointer to the value at offset -8 of a5.
- `traverse_linked_list_4`: Updates a pointer at offset -8 of a5 by adding a value from offset 4 of that pointer, compares the new pointer to a global variable, and if it is not less than that variable, checks a bit at offset 9 of the pointer to decide whether to call another routine or perform further arithmetic and branching.
- `push_stack_and_call`: Pushes three long values from negative offsets of a5 onto the stack, calls another routine, adjusts the stack pointer, stores the result in a negative offset of a5, and branches.
- `branch_to_target_address`: Performs a short branch to the address stored in a memory location.
- `insert_list_node_with_flag`: Saves registers to the stack, loads two pointers from a5, stores a word from a positive offset of the first pointer at a negative offset from a5, clears a word at a negative offset from a5, and if bit 7 of a byte at a positive offset from the first pointer is set, calls another routine, sets a word to 1 at a negative offset from a5, and loads values from a global variable into d4 and d5 before branching.
- `insert_linked_list_node_2`: Iterates through a list to find a matching pointer, updates a linked list node by copying data from a structure, calls two routines, and potentially updates a global pointer with the new node address.
- `insert_linked_list_node`: Checks if a node at a pointer in a3 is within a specific range and matches a value in a2; if so, it adds a value from a3 to a pointer in a2, updates the pointer in a2 with the contents of a3, and stores a3 into a global variable.
- `load_comparison_pointers`: Sets up a stack frame, saves a2 and a3 to the stack, loads a pointer from a global variable into a2, loads another pointer from a global variable into a3, and branches to another routine.
- `return_if_pointer_null`: Tests a pointer in a2; if it is not null, it branches to another routine, otherwise it moves the pointer to d0, restores registers from the stack, unlinks a5, and returns.
- `compare_and_clear_d0`: Increments a2 by four, compares it to a3, and if it is not lower, clears d0 and branches to another routine; otherwise it branches to another routine.
- `clear_output_pointer`: The routine stores zero into the memory location pointed to by its pointer argument.
- `process_linked_list_data`: Calls a routine to process a pointer from a structure, then calls another routine to process a second pointer, and finally calls a third routine to iterate through a linked list of pointers, adding the returned longword value to a longword at offset negative 8 of a structure in a5, before storing a longword from a global variable into offset negative 4 of a5 and branching to another location.
- `accumulate_string_length`: Iterates through a linked list of pointers, adding the length returned by a call to a longword at offset negative 8 of a structure and calling another routine to copy data from the current pointer to a destination buffer.
- `sync_pointer_offset_data_2`: Retrieves a pointer via a routine, checks a word at offset 2 of that pointer, and if it is zero, copies a word from the pointer to a location specified by a second pointer; otherwise, it performs a series of routine calls and copies a word to a location specified by a third pointer.
- `sync_pointer_offset_data`: Retrieves three pointers via a routine, copies a word from the second pointer to the first, and then conditionally updates a word at an offset of 2 within a structure pointed to by a value at offset 8 of the input structure.
- `copy_word_and_check_pointer`: Calls a routine three times using values from a structure, copies a word from the second result to the first, and returns one if a pointer in a structure is non-zero.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `clear_and_update_table`: Calls a routine to obtain a pointer, and if the word at offset 0 of that pointer is non-zero, it branches; otherwise, it calls the routine again to obtain a second pointer, clears the word at offset 0 of that second pointer, and writes a word from offset 14 of the stack frame into offset 2 of a location stored at offset 8 of the stack frame.
- `copy_and_update_two_word_table`: Copies a two-word value from one table entry to another, then writes two words from a global variable into two locations obtained from another routine, increments a pointer stored in a global variable, writes the incremented pointer value into a location obtained from another routine, and returns a word from the global variable.
- `clear_and_update_word`: Calls a routine to obtain a pointer, and if the word at offset 2 of that pointer is non-zero, it branches; otherwise, it calls the routine again to obtain a second pointer, clears the word at offset 2 of that second pointer, and writes a word from offset 14 of the stack frame into the location pointed to by offset 8 of the stack frame.
- `copy_word_pair_from_table`: Copies a 16-bit word from one table entry to another, then writes two words from a global variable into two locations obtained from another routine, updates a pointer stored in a global variable, writes a word from that pointer into a location obtained from another routine, and finally returns a word from the global variable.
- `update_linked_list_node_5`: Checks if the word at the pointer stored at offset 8 of the stack frame is zero; if so, it writes a word from offset 12 of the stack frame into offset 2 of that pointer, calls a routine to obtain a new pointer, and clears the word at that new pointer's offset 0.
- `update_linked_list_node`: Updates a structure by copying a value from a global variable into an offset of a pointer, increments a pointer at a negative offset, and returns a value from a global variable.
- `manage_linked_list_elements`: Calls a routine to obtain a pointer and checks if the word at offset 2 is zero; if not, it performs a sequence of calls to obtain multiple pointers, copies a word from one to another, and if a subsequent pointer's word at offset 0 is zero, writes a word from offset 12 of the stack frame into offset 2 of a location at offset 8 of the stack frame.
- `copy_two_word_pointers`: Copies a 16-bit word from one location to another and updates a pointer table entry.
- `update_list_node_value`: Checks if the word at the pointer stored at offset 8 of the stack frame is zero; if so, it writes a word from offset 12 of the stack frame into that pointer and calls a routine to obtain a new pointer, then clears the word at offset 2 of that new pointer; otherwise, it performs a different update to the pointer's offset 2 and offset 4.
- `update_linked_list_node_2`: Updates a structure by copying a 16-bit value from an offset of 12 in a structure to a destination pointer, then modifies a pointer at offset -4 of the structure by incrementing it and updates a 16-bit value at offset 4 of a new destination.
- `copy_word_to_list`: Calls a routine to obtain a pointer and checks if it is zero; if not, it calls the routine again and checks if the word at offset 2 is zero; if it is, it calls the routine a third time to copy a word from that result into a location pointed to by offset 8 of the stack frame.
- `copy_word_pairs_and_clear_flag`: The routine calls another routine to obtain a pointer, calls another routine to obtain a second pointer, and copies a word from the second pointer to the first.
- `store_pointer_to_stack`: Loads a pointer from offset 8 of the stack frame into a0 and stores the word at that pointer into offset -2 of the stack frame before branching.
- `search_linked_list`: The routine traverses a list of pointers until it finds a non-zero value or a value that matches a specific word.
- `retry_on_nonzero`: Calls another routine with a given argument and, if the returned short is non-zero, calls that same routine a second time with the same argument.
- `copy_word_block_optimized`: Copies a block of words from a source address to a destination address using a loop that handles both forward and backward memory directions depending on the relative order of the pointers.
- `copy_word_block`: Decrements a 16-bit value in d0, then copies words from a source pointer in a0 to a destination pointer in a1 in a loop until the counter reaches zero.
- `calculate_stack_offset`: Subtracts a long value stored at a negative offset from a4 from the current stack pointer and returns the result.
- `rotate_word_left`: Takes a 16-bit argument from the stack, rotates it left by eight bits, and returns the result.
- `combine_two_bytes_into_word`: Reads a byte from a pointer and the next byte in memory, concatenates them into a 16-bit value, and returns it.
- `combine_four_bytes_into_word`: Reads four bytes from a byte pointer argument and concatenates them into a 32-bit value, returning that value.
- `add_to_global_offset`: Adds a 16-bit value to a value stored in a global variable and returns the result if the input is non-zero.
- `subtract_global_offset`: Subtracts a value stored in a global variable from the provided argument if the argument is non-zero.
- `initialize_system_state`: Clears memory areas, initializes a table with incrementing values, and performs several indirect calls to set up global pointers and values.
- `initialize_structure_data`: Saves a global pointer, updates it with a provided parameter, performs a series of word writes to that parameter's memory, calls another routine, and then restores the global pointer and performs further word writes to the returned pointer.
- `load_global_value`: Loads a 32-bit value from a global memory location into the location pointed to by the input parameter.
- `store_global_value`: Stores a 32-bit value from the stack into a global memory location.
- `update_short_values`: Stores a word into a memory location pointed to by a value retrieved from a global offset, clearing the lowest bit of the first word.
- `store_word_pair`: Writes two consecutive words from a parameter to memory locations at offsets 18 and 20 from a pointer retrieved from a global offset.
- `add_to_word_pair`: Adds two words from a parameter to two existing words stored at offsets 18 and 20 from a pointer retrieved from a global offset.
- `store_word_offset_1c`: Stores a word into a memory location at an offset of 28 from a pointer retrieved from a global offset.
- `store_word_offset_1e`: Stores a word into a memory location at an offset of 30 from a pointer retrieved from a global offset.
- `store_word_offset`: Stores a word into a memory location at an offset of 32 from a pointer retrieved from a global offset.
- `store_word_offset_1a`: Stores a word into a memory location at an offset of 26 from a pointer retrieved from a global offset.
- `load_word_offset`: Returns a word stored at an offset of 22 from a pointer retrieved from a global offset.
- `store_word_offset_16`: Stores a word into a memory location at an offset of 22 from a pointer retrieved from a global offset.
- `process_swapped_word`: Calls a function pointer from a global offset and, if the result is not null, stores a rotated word and a second word into a memory location derived from a global offset.
- `fetch_indirect_word`: Loads a pointer from a global variable, applies a constant offset to it, and returns the word stored at that location.
- `call_indirect_pointers`: Loads several function pointers from global variables and calls them using values retrieved from other global memory locations.
- `validate_and_clear_object_pointer`: Calls a routine to process a pointer from a structure, then compares a value from a global structure with a value at an offset of 10 from another global structure; if the comparison result is not less than or equal to zero, it clears the pointer at offset 8 of the original structure.
- `update_object_coordinates_and_table`: Calculates a new coordinate value based on bitwise shifts of a 16-bit value from a structure, updates a pointer in that structure, and performs a loop to copy data and update a table of values using a calculated offset.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a2, so the target is only known at run time.
- `shift_and_return`: Shifts d1 right by one bit and, if the result is non-zero, loads a 32-bit value from offset 8 of a structure into d0 and returns.
- `update_bitmask_and_loop`: Calculates a bitmask from a structure in a0 and uses it to perform bitwise operations on a destination buffer in a loop.
- `update_bitmask_and_jump`: Clears bits in a destination pointed to by a1 using a mask from a global variable, then sets bits in the same destination using a mask from a source pointed to by a0, and finally jumps to the address in a2.
- `process_conditional_call`: Shifts d1 right by one bit and, if the result is zero, retrieves a 16-bit value from the stack, calculates an offset from a global base, and calls another routine with that offset and the constant 85.
- `swap_word_at_offset`: Reads a word from offset 32 of a structure pointed to by a value from a global variable, writes the value two to that same location, calls another routine, and then restores the original word.
- `call_with_offset_value`: Calls another routine using a value retrieved from offset 28 of a memory structure.
- `call_with_different_offset`: Calls another routine using a value retrieved from offset 30 of a memory structure.
- `modify_memory_via_bitmask_loop`: Checks a condition based on a value in a structure and, if met, enters a nested loop to modify a block of memory.
- `modify_byte_conditionally`: Compares a word at a negative offset from a4 with two and, if equal, performs a bitwise XOR on a byte at the address in a1.
- `modify_byte_via_nested_loops_2`: Performs bitwise operations on a byte at the address in a1 and branches based on a comparison with a value in a global variable.
- `modify_byte_via_nested_loops`: Performs a bitwise operation on a byte at the address in a1 using d0, then enters nested loops that use values from the stack to modify memory at an offset of 160 from a pointer.
- `loop_and_modify_byte_2`: Decrements d2 and performs a loop that conditionally XORs a byte at the address in a1 with d0, depending on a value at an offset of -30016 from a4.
- `loop_and_modify_byte`: Copies a byte from a7 to a destination pointed to by a1 and enters a loop that decrements d2, where the loop body performs a conditional bitwise XOR on the destination byte based on a comparison with a constant and a value at a global variable offset.
- `tiny_andi_or_rts`: Small routine of 3 instructions that masks (a1) with 15, then merges d5 into (a1), then returns.
- `call_indirect_via_offset`: The routine loads a pointer from a global variable, applies a constant offset to it and calls the resulting address as a function.
- `call_indirect_via_table_2`: The routine loads an address from a global variable and calls another routine through an indirect pointer.
- `load_long_from_global_2_2`: Checks bit 2 of d0 and, if set, loads a 32-bit value from a global variable into d0 before branching.
- `load_long_from_global_2`: Checks bit 1 of d0 and, if set, loads a 32-bit value from a global variable into d0 before branching.
- `load_long_from_global`: Loads a 32-bit value from a global variable into d0 and returns.
- `return_register_word`: The routine returns a word from a register.
- `call_and_store_result`: The routine calls an indirect routine and stores the returned value into a global variable.
- `execute_conditional_function`: Calls another routine and then executes a function pointer from a global location if a comparison of two status flags does not result in equality.
- `call_indirect_through_pointer`: Loads a pointer from a global variable and calls another routine through it.
- `call_indirect_via_table_3`: Loads an address from a global variable and calls the routine stored at an offset from that address.
- `call_indirect_via_pointer`: Loads an address from a global variable and calls another routine through that pointer.
- `call_indirect_via_offset_2`: Loads an address computed from a global variable plus a constant offset and calls the routine found there.
- `call_indirect_via_offset_3`: Loads an address from a global variable and calls another routine through an indirect pointer offset by a constant.
- `clear_three_memory_areas`: The routine calculates a length based on a value from a global variable and then fills three separate memory areas with zeros using loops.
- `process_bitmask_data`: Copies a four-byte structure to a global location and iterates through a data stream to build a bitmask based on nibbles found in the bytes.
- `sum_indexed_bytes_2`: Calls another routine with two arguments, then if the result in d0 is not zero, uses it as a pointer to access a table and calculates a sum using a loop.
- `sum_indexed_bytes`: The routine iterates through a buffer, uses each byte as an index into a table to find a word, and adds the byte at a specific offset within that table entry to a running total.
- `get_indexed_byte`: The routine retrieves a single byte from a table using an index provided in an argument and returns it as part of a larger value.
- `get_indexed_byte_offset`: The routine retrieves a byte from a table at an index provided in an argument, but accesses the byte at a one-byte offset from the calculated position.
- `increment_indexed_value`: Retrieves a value from a routine and adds it to a value stored at a location derived from a table pointer.
- `update_multiple_offsets`: Retrieves values from a table and performs several additions and stores, updating multiple global variables with the results.
- `add_offset_to_four_shorts`: The routine adds a 16-bit offset from the second argument to the first and third words of a short array pointed to by the first argument, and adds a different 16-bit offset from the same argument to the second and fourth words of the array.
- `add_offsets_to_word_pair`: The routine reads two 16-bit values from an input structure and adds each to a pair of 16-bit words in an output buffer.
- `add_short_pair_from_table`: The routine adds two consecutive 16-bit values taken from a table pointed to by a pointer in a1, offset by 6 and 8 bytes, to two consecutive 16-bit values pointed to by a pointer in a0, storing the results back through the a0 pointer.
- `call_other_routine`: Loads a pointer from a structure and calls another routine with two arguments, then returns.
- `adjust_four_short_values`: The routine receives a pointer to four consecutive 16-bit values and two 16-bit inputs.
- `adjust_pointers_by_delta`: The routine adjusts four 16-bit values pointed to by a pointer.
- `compare_and_store_word`: Compares a word from a pointer in a0 against values in a structure and, if all comparisons pass, stores a word from the stack into d0.
- `pop_and_set_ccr`: Pops a 16-bit word from the stack into d0 and sets the carry flag.
- `compare_and_set_one`: Loads three pointers from a structure, calls another routine, and if the carry flag is not set, sets d0 to one and branches to another location; otherwise it returns.
- `return_zero_stub`: Clears a 16-bit register, restores any number of registers from the stack, unlocks a semaphore, and returns zero.
- `clamp_value`: Loads a word from a memory location pointed to by a0 into d0, compares it to a word at a location pointed to by a1, and if d0 is smaller, loads the value from a1 into d0.
- `clamp_value_to_range_3`: Compares d0 against a word at offset 4 of a structure in a1 and updates a destination at offset 2 of a2 if d0 is larger, then compares d0 against a word at offset 2 of a1 and updates d0 with a word from offset 2 of a1 if d0 is smaller.
- `clamp_value_to_range_2_2`: Compares d0 against a value at offset 6 of a structure, updates a destination at offset 2 of a2 if necessary, and then compares d0 against a value at offset 0 of the same structure.
- `clamp_value_to_range_2`: Compares d0 against a threshold in a structure, updates a destination at an offset of 4 in a2 if d0 is larger, and then performs a second comparison and update using a value at offset 2 in a1.
- `clamp_value_to_range`: Compares a value in d0 to a threshold in a structure, updates a destination structure with the smaller value, and then returns either the first or second word of that destination structure based on a comparison.
- `store_long_words`: Stores two 32-bit words from a pointer in a0 into two 32-bit locations at offsets -7168 and -7164 relative to a4.
- `calculate_area_and_update_coords`: The routine updates two 16-bit values in a table by rounding them to even numbers and returns the product of the differences between the updated values and the original values.
- `update_nested_data_structure`: Checks a flag in a data structure and, if set, updates several local word values and calls another routine.
- `process_nested_data`: Checks a bit in a global variable, and if set, performs bitwise operations on a byte in a1 using d3, increments d0 and d1, and conditionally calls another routine while managing a stack-based counter and a pointer in a1.
- `decrement_and_load_words`: Decrements a byte at an offset of -28478 from a4 in a loop, otherwise it loads two words from the stack into d5 and d4 and returns.
- `clear_byte_at_offset_4`: Moves a byte from a pointer in a0 to d2 and then checks bits in two different global variables to conditionally write a byte to a third global variable.
- `clear_byte_at_offset_3`: Writes the value 2 to a byte at an offset of -28469 from a4 and then branches.
- `clear_byte_at_offset`: Clears a single byte at an offset relative to a4.
- `dispatch_handler_by_type`: Loads a pointer from offset 8 of a structure in a5, increments it, and reads a word from it; if the word is 2, it calls a routine at an offset from a4 and stores the return value at a large negative offset from a4.
- `dispatch_by_value`: Compares d0 to 3 and 4; if d0 is 4, it extracts several bytes from a structure in a0 and stores them at various offsets relative to a4, then calls another routine; otherwise, it returns.
- `process_conditional_values`: Compares d0 to the constants five, six, and seven, then either branches, pushes two values from a structure in a0 onto the stack to call another routine, or subtracts a value from a global structure from d0 and stores the result in another global structure.
- `branch_on_condition`: Compares d0 to 8; if not equal, it branches, otherwise it checks if a word at a0 is zero, and if not, it calls another routine with a pointer derived from a global variable.
- `compare_and_call_function`: Compares a word in d0 to 9, and if it is not equal, pushes the pointer in a0 onto the stack, calls another routine, and branches to another location.
- `call_routine_if_not_ten`: Compares d0 to 10, and if it is not equal, pushes the pointer in a0 onto the stack, calls another routine, and branches to another location.
- `push_structure_elements_and_call`: Compares a 16-bit value in d0 to eleven; if not equal, it pushes five words from a structure pointed to by a0 onto the stack, pushes the pointer a0 itself, and calls another routine before adjusting the stack and branching.
- `call_function_on_condition_3`: Compares d0 to twelve and, if not equal, pushes two values from a structure onto the stack and calls another routine.
- `call_function_on_condition_2`: Compares d0 to 13 and, if equal, pushes a0 to the stack, calls a routine at an offset of -32760 from a4, and then branches.
- `call_function_on_condition`: The routine checks if a 16-bit value equals a specific constant and, if so, calls a function pointer stored in a global variable using a value from a register.
- `search_and_return_word`: Searches through a list of words starting at a pointer derived from a structure in a5, and if a match for a provided word is found, it returns a 32-bit value composed of the address of the match and the match itself.
- `dummy_routine`: Loads a zero constant into a register and returns it.
- `search_and_update_word`: Searches through a list of words starting at a pointer derived from a structure in a5, and if a match for a provided word is found, it returns a 32-bit value and writes a new word to that location.
- `return_zero_stub_2`: Loads a zero constant into a register, copies a word from an offset of a register into that register, writes that word to a memory location, and returns.
- `search_and_return_address`: Searches through a list of words starting at a pointer derived from a structure in a5, and if a match for a provided word is found, it returns the address of that match.
- `return_zero_register`: Clears a register and returns a zero value in a register before returning immediately.
- `swap_global_value`: Swaps a value stored at a global memory location with a value held in a3, calls another routine, and then restores the original global value.
- `update_data_structure`: Calls a function through a pointer, stores three values into a memory structure at specific offsets, and decrements a 32-bit value at another offset.
- `increment_and_add_offset`: Clears a 16-bit register, moves a byte from a pointer in a0 to d0 and increments the pointer, then adds a word from a global variable to d3.
- `remove_list_element_and_reorganize`: Checks if a pointer in a3 is valid and contains a specific 16-bit marker, then increments a counter in a4, updates a list pointer, and performs a loop to search for a value within a linked structure to remove it and adjust subsequent elements.
- `write_word_to_pointer`: Loads a pointer from offset -26628 of a4 into a1, stores a word from offset 2 of a3 into the location pointed to by a1, clears a longword at offset -26624 of a4, and branches to another location.
- `search_and_remove_list_element_2`: Searches for a 16-bit value within a list structure and, if found, removes the element by adjusting pointers and calls another routine to handle the list modification.
- `calculate_address_from_word`: Loads a word from a location in a1 into d2, and if it is non-zero and its lower 16 bits are non-zero, it adds a longword from offset -25304 of a4 to d0 and moves the result into a1; otherwise it returns.
- `search_and_remove_list_element`: Calculates a target value by subtracting a value from a pointer in a3, then calls another routine to search for that value within a linked list structure.
- `copy_data_to_node`: Updates a node in a list with new data from a structure in a5 and copies a block of memory from the stack into the node's data area using a loop, then calls another routine.
- `copy_and_update_structure`: Checks if a pointer in a0 is valid and matches a specific constant, then calls a routine through a pointer, copies a block of data from a source to a destination using a loop, and updates a structure at a1 based on a value from a0.
- `clear_and_call_routine`: Checks if a pointer in a0 is valid and matches a specific constant, then if certain bits are set in a word at offset 4, clears a word at offset -8 and calls a routine using a pointer from a4.
- `search_nested_linked_lists`: Validates a structure by checking a magic constant, then performs a linear search through a linked list of 16-bit values to find a match for a provided parameter.
- `traverse_linked_list_2`: Traverses a linked list by repeatedly loading a pointer from a structure and adding an offset from a global variable, eventually returning either a pointer or a constant based on whether a target value is found.
- `process_pointer_and_branch`: Moves a word from a pointer in a1 to d0, and if d0 is zero, it performs a bitwise AND on d0 with a constant, then either adds a value from a global variable to d0 and branches or updates a1 with the result of the addition.
- `set_d0_and_branch`: If a condition is met, it sets d0 to 1 and branches to another instruction; otherwise it returns immediately.
- `return_constant_zero`: Clears a 16-bit register, restores a set of registers from the stack, unlocks a semaphore, and returns zero.
- `validate_range_and_pattern`: Checks if a parameter falls within a specific range, is even, and matches a specific short value at an offset, then increments a 16-bit register if all conditions are met.
- `search_word_in_list`: Calculates a count based on the difference between two values in a structure, then searches a list for the word 4660, updating the list pointer and count if found, and writes 42 or 43 to a global variable based on a bit test of a value at an offset from the list pointer.
- `process_conditional_logic`: Writes a constant to a global variable, calls another routine, and if a condition in d0 is met, performs several operations including calling another routine with a pointer and a value from a structure.
- `push_and_call_routine`: Pushes several values onto the stack, including a word from d3 and a pointer from a2, then calls a routine at an offset from a4.
- `fetch_global_word`: Returns a word from a global variable located at a fixed offset.
- `execute_conditional_logic`: Checks bit 2 of d0, and if not set, pushes a1 and a constant to the stack, calls another routine, and if d0 is non-zero, performs a series of pointer arithmetic operations and calls a routine through a pointer before returning 1.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `return_constant`: Returns the constant 5510 in d0 to the caller without any other effect.
- `initialize_complex_structure`: Calls another routine to allocate memory, stores the resulting pointer in a global variable, calls three more routines to initialize the memory, writes a word to offset 14 of the structure, calls another routine with a pointer from a global variable, clears two words in a global variable, calls another routine with the allocated pointer, and finally calls a routine to update a value at a global offset.
- `call_with_offset`: Loads a 32-bit value from a global variable at a fixed offset from a4, pushes it onto the stack, calls another routine, and returns.
- `load_value_from_offset`: The routine loads a 32-bit value from a memory location calculated by adding a negative offset to a register.
- `execute_nested_calls`: Initializes a stack frame, calls another routine, then calls a second routine, then calls a third routine using a value from a global offset; it then enters a loop where it calls a fourth routine, a fifth routine, a sixth routine, and a seventh routine until a value in d0 matches a value at offset -4 of the structure in a5.
- `call_with_offset_arg`: Loads a 32-bit value from a global variable located at a fixed offset from a register and passes it as an argument to another routine.
- `update_node_and_pointers`: Calls several routines to load and store values, compares a word to a global variable, and if they are not equal, performs further routine calls and updates a structure.
- `update_node_data_2`: Calls a routine using a value from a pointer offset and stores a 32-bit value into a global memory location.
- `swap_linked_item_pointers`: Checks if two 16-bit words at offsets 44 and 54 of a structure pointed to by offset 8 of a5 are non-zero, calls another routine with an offset of 34 from that structure, calls another routine with a pointer from offset 46, and stores the result of the first call into offset 46 of the structure.
- `compare_and_set_d0`: Links a5, pushes a word from offset -103 of the structure in a4 onto the stack, calls another routine, adjusts the stack pointer, compares a long value at offset 8 of the structure in a5 with d0, and if they are not equal branches to another routine; otherwise it sets d0 to 1 and branches to another routine.
- `unlock_address_register_and_return_2`: Clears the d0 register and unlocks the address register a5 before returning.
- `compare_and_update_table`: Compares a word from a subroutine result to a global variable and, if they differ, performs a series of operations involving calling subroutines to update values and pointers.
- `allocate_window_structure`: Attempts to allocate memory for a window structure via a heap check; if allocation fails, it calls another routine with a constant and returns zero, otherwise it returns the pointer to the allocated memory.
- `initialize_and_update_structure`: Initializes a structure by calling several routines to set up its members, clears specific fields, and performs conditional updates to a longword and a word based on bit tests and value comparisons.
- `initialize_window_buffer`: Clears a word at offset 44 of a structure, checks if a pointer at offset 12 is non-zero and bit 2 of a byte at offset 17 is set, then calls two routines to populate offset 50 of the structure with a returned value; if that value is zero, it calls another routine and returns zero, otherwise it continues with further processing and returns.
- `call_subroutine_with_offset_pointer`: Pushes a longword from offset 12 of a5 onto the stack, loads a pointer from offset 22 of a structure into a0, pushes a longword from offset 50 of that structure onto the stack, calls a routine, and then branches.
- `update_object_dimensions_and_bounds`: Clears a word at offset 50 of a structure, calls another routine with a pointer from the structure, and conditionally modifies two 16-bit values at offsets -10 and -6 of the structure based on bit tests and a comparison of a value at offset 42.
- `recalculate_bounding_box_dimensions`: Updates several 16-bit values in a structure by comparing current dimensions against values in a table and calling another routine to adjust them.
- `update_object_state_2`: Checks a flag at offset 54 of a structure to prevent re-entry, then updates various fields including a 16-bit value at offset 46 and an offset 4 value based on bit tests of offset 43.
- `dispatch_multiple_calls`: Performs a sequence of calls to various routines using values retrieved from a global variable and offsets from a structure in a5, and if a specific long value at offset 50 of that structure is non-zero, calls another routine twice with different parameters.
- `self_recursive_short_array_processor`: Receives a pointer to a short array and reads four consecutive elements from it, then performs four recursive calls using different combinations of the array elements incremented or decremented by one.
- `update_array_elements`: Loads values from global memory, performs arithmetic on them, and updates a short array using calculated offsets.
- `store_four_words_from_registers`: Writes four 16-bit values into a structure pointed to by a pointer in a5, using values or offsets derived from other arguments in a5.
- `compare_structure_values`: Compares a 16-bit word from a structure at offset 6 with a 16-bit word at offset 2 of another structure, and then compares a 16-bit word at offset 4 of the first structure with a 16-bit word at offset 0 of the second structure, returning 1 if the second comparison is not greater than the first.
- `unlock_address_register_five_and_return`: Clears the value in register zero and unlocks the address register five before returning.
- `copy_two_words`: Copies two consecutive words from a source pointer in a5 to a destination pointer in a5.
- `copy_word_conditionally`: Compares a 16-bit word from a structure pointed to by offset 8 of a5 with a 16-bit word from a structure pointed to by offset 12 of a5; if the first word is greater than or equal to the second, it copies the first word to the location pointed to by offset 16 of a5.
- `conditionally_copy_word_from_structure`: Compares a 16-bit word from a structure to a value in another structure; if the value is greater than or equal to the comparison word, it returns, otherwise it copies the word from the first structure to the second structure.
- `update_structure_fields_5`: Copies a 16-bit word from offset 2 of a structure pointed to by a pointer at offset 12 of a5 to offset 2 of another structure pointed to by a pointer at offset 16 of a5, then conditionally copies a 16-bit word from offset 4 of the first structure to offset 4 of the second structure.
- `update_structure_fields_3`: Copies a 16-bit word from offset 4 of one structure to offset 4 of another, then copies a 16-bit word from offset 6 of the first structure to offset 6 of the second structure if a comparison between offset 6 of the first and offset 6 of the second fails to be less than or equal.
- `copy_word_from_pointer`: Copies a 16-bit word from a location pointed to by a pointer in a5 plus an offset to another location pointed to by a different pointer in a5 plus an offset.
- `compare_structure_ranges`: Compares several 16-bit values between two structures pointed to by a5 and returns 1 if a specific set of range conditions is met, otherwise it returns 0.
- `branch_with_zero`: Sets register d0 to zero and branches to another location.
- `calculate_dual_differences`: Calculates two differences between 16-bit words from two different structures, calls another routine for each difference to process the result, and clears a 32-bit value at offset 20 of the stack frame if both processed results are zero.
- `calculate_clamped_offset`: Calls adjust_and_store_value and perform_signed_division using values from a stack frame, then checks a signed value at offset 12 to potentially subtract the result from 200 and store it at offset 20.
- `adjust_and_store_value`: Checks if a 32-bit value at offset 16 from a5 is negative, adds 400 to a value at offset 20 from a5, and then updates that memory location using the results of two routine calls.
- `push_byte_to_stack`: Links the stack frame, pushes a pointer from a2 onto the stack, and copies a byte from offset 9 of a5 into d0 before branching to another routine.
- `execute_multiple_calls_3`: Calls another routine with a constant, calls a second routine with a word from offset 10 of a5, updates a longword in a structure pointed to by the first result, calls a third routine, and then branches to another routine.
- `process_structure_pointer`: Pushes a word from offset 10 of a structure onto the stack, calls another routine, and if the result is non-zero, calls another routine with a pointer from the result, then calls a third routine using a value from a global variable, and finally updates a longword at offset 10 of a structure pointed to by the result.
- `execute_conditional_logic_8`: Calls another routine, stores the result in a2, and if a global variable is non-zero, performs two calls using values from offsets 9 and 10 of a5 as arguments.
- `clear_byte_and_branch`: Clears a byte at offset 7 of a structure pointed to by a2 and then branches to another routine.
- `subtract_and_branch`: Subtracts 133 from d0 and branches to one location if the result is zero, otherwise it branches to another location.
- `update_structure_fields_6`: Pushes a pointer from a2 onto the stack, calls another routine, pushes the result, calls a third routine using a value from a global variable, and then copies a byte from offset 9 of a5 to offset 6 of a structure and a word from offset 10 of a5 to offset 8 of that same structure.
- `traverse_linked_list_5`: Pushes a word from offset 10 of a structure onto the stack, calls a routine, and if the result is non-zero, recursively follows a pointer at offset -4 of the structure to call another routine, then calls a routine with a value from a global offset of a4, and finally calls another routine using the pointer at offset -4 of the structure.
- `update_byte_via_pointer`: Calls another routine with a byte from offset 9 of a structure in a5, and if the result is non-zero and a word at offset 12 of the structure is non-zero, sets a byte at offset 7 of a location pointed to by the result.
- `clear_byte_at_offset_2`: Loads a pointer from an offset of 4 in a structure in a5, then clears a byte at offset 7 of the pointed-to location and returns.
- `process_linked_list_elements`: Checks a global variable and, if non-zero, calls three routines to retrieve and process values, storing a result at offset -4 of a structure in a5.
- `traverse_linked_list_subset`: Follows a pointer at offset -4 of a structure to a second structure, calls a routine with the value at offset 10 of the second structure, calls another routine with the pointer at offset -4 of the first structure, calls a routine with a value from a global offset of a4, and calls another routine using the pointer at offset -4 of the first structure.
- `branch_to_mid_routine`: Performs a short branch to an instruction located within another routine.
- `set_and_branch`: Sets d0 to 1 and performs an unconditional branch.
- `accumulate_conditional_values`: Checks a global value and, if non-zero, enters a loop that calls several routines to process a structure, adding a value from a specific routine to a local accumulator if certain byte and word conditions are met at offsets 6 and 8 of the structure.
- `return_longword_at_negative_8`: Returns the longword stored at offset negative 8 of a5.
- `execute_sequence_of_calls`: Pushes a longword from offset -25020 of a4 onto the stack, calls another routine, moves the result from d0 to the stack, calls another routine, and finally stores the result from d0 into a longword at offset -4 of a5 before branching to another routine.
- `search_table_for_match`: Loops through a table by repeatedly calling other routines and updating a pointer until a byte at offset 6 of a pointed-to structure matches a byte at offset 9 of the current structure and a word at offset 8 matches a word at offset 10, or the pointer becomes zero.
- `execute_conditional_calls_3`: Calls a routine via a pointer from a structure, stores the result in a local variable, and if the result is zero, calls another routine and then potentially calls a third routine via a call table if certain conditions are met.
- `return_longword_at_offset_negative_4`: Returns the longword stored at offset negative 4 of a5.
- `store_and_check_result`: Pushes a pointer from a5 to the stack, calls another routine, stores the result in a local variable at a negative offset from a5, and if the result is not negative 65535, it sets d0 to 0 before returning.
- `execute_multiple_calls_2`: Calls another routine, stores the result in a local variable at offset minus 4 of a5, calls another routine using that result and a constant, calls a third routine using a value from offset minus 6 of a5, and finally calls a fourth routine using the value from offset minus 6 of a5 before returning the stored result in d0.
- `format_log_message`: Links a stack frame, compares a word at offset 8 of a5 to 133, and if equal, pushes a word from offset 10 of a5, calculates an offset from a word in offset 8 of a5, loads a pointer from a table, pushes two values onto the stack, and calls another routine.
- `execute_file_operations_2`: Pushes a word from offset 10 of a5 onto the stack, calculates an offset using a word from offset 8 of a5, pushes a pointer from a table into a0, pushes a word from offset negative 24778 of a4 onto the stack, pushes two arguments onto the stack to call another routine, cleans the stack, and branches based on a comparison of a word at offset negative 26094 of a4 with 3.
- `execute_file_operations`: Performs a sequence of calls involving pushing values to the stack, including a call that uses a value from a structure at offset -2, and several other calls that use values from a structure at offset -2 and a value from a global variable.
- `fetch_and_store_pointer_2`: Calls a routine with a pointer to a global variable, calls another routine, and stores the resulting pointer into offset -4 of a structure in a5 before branching to another routine.
- `traverse_linked_list_recursively`: Recursively traverses a linked list by following pointers stored within the nodes, calling three different routines and updating a value at offset -8 of a structure in a5.
- `calculate_and_apply_data_offsets`: Calculates several offset values by adding word values from a data structure to existing values in a global structure, then calls another routine and another routine to process the results.
- `expand_nibble_bitstream_data`: Reads a compressed stream of bytes from a buffer in a0 and expands them into destination buffers by splitting each byte into nibbles and processing bits based on a control value.
- `update_pointers_and_branch_2`: Checks a byte in d6, and if it is non-zero, extends d2 to a word, checks bit 0 of d5, and if bit 0 is clear, adds d0 to d5, shifts d0 right, and increments a1 and a6 by the new d0 value before branching.
- `update_pointers_and_branch`: Adds a 16-bit value in d0 to d5, shifts d0 right, and if the carry flag is set, increments two pointers in a1 and a6 by d0 plus one, then branches to another routine.
- `expand_compressed_nibble_stream`: Reads a stream of bytes from a source buffer and expands them into two separate destination buffers by splitting each byte into nibbles.
- `decompress_bitstream_data`: Reads a bitstream from a source buffer and expands it into a destination buffer by processing nibbles and bits based on values stored in a structure pointed to by a4.
- `modify_pointer_and_branch_2`: Checks a byte in d6, and if it is non-zero, extends d2 to a word, checks bit 0 of d5, and if bit 0 is clear, adds d0 to d5, shifts d0 right, and increments a1 by d0 before branching.
- `modify_pointer_and_branch`: Adds a 16-bit value from d0 to d5, shifts d0 right by one bit, and if the carry flag is clear, adds 1 to a1 and adds d0 to a1 before branching to another routine; otherwise it branches to another routine.
- `expand_bitstream_data`: Reads a byte from a source buffer and uses its nibbles to expand data into a destination buffer by either inserting nibbles into existing bytes or inserting new bytes based on a bit-count and index tracking system.
- `combine_two_bytes_from_pointer`: Calls another routine and treats the returned value as a pointer, then reads a byte from that pointer and the next byte, returning a 16-bit value formed by concatenating the second byte as the high byte and the first byte as the low byte.
- `extract_word_from_pointer`: Calls another routine and receives an integer result, then reads two bytes from the location pointed to by that result at offsets two and three, and returns them concatenated as a 16-bit value.
- `execute_unnamed_routine`: Calls another routine to fetch a table offset, then performs several arithmetic operations and writes or reads values to or from a structure pointed to by a register.
- `swap_word_bytes`: The routine reads a 16-bit word from the location pointed to by a pointer stored in its argument and returns the same word with its high and low bytes exchanged.
- `decode_table_entry`: The routine reads a pointer from a global variable, uses the first word as a count and the second word as an index, clamps the index to the count, then fetches a word from a table at an offset derived from the index and uses it as a byte offset from the base pointer to read two bytes and returns them as a combined word.
- `fetch_table_offset`: The routine uses a pointer from a global variable to find a count and index, clamps the index, fetches a word from a table to use as an offset, and then returns a pointer calculated by adding a value from the table to the base pointer.
- `check_conditional_byte`: Tests if d2 is zero, then tests if a byte at offset 7 of a structure in a0 is non-zero; if both conditions are met, it branches to another routine, otherwise it branches to another routine.
- `decompress_data_stream`: The routine performs a multi-pass decompression on a data buffer by reading nibbles to determine lengths and copying bytes into a temporary stack area.
- `call_recursive_handler`: Checks if a parameter is non-zero and, if so, calls another routine three times, with a constant, no arguments, and the original parameter before returning.
- `store_param_in_global`: The routine stores the incoming parameter into a global variable by writing it through a pointer derived from a register.
- `call_indirect_via_table`: The routine receives a single argument and immediately calls another routine through an indirect pointer obtained by adding a constant offset to a global variable.
- `execute_complex_sequence`: Clears a 16-bit word at offset negative 2 of a5 and performs a sequence of calls to various routines with different arguments pushed onto the stack, including passing offset negative 10 of a5, and finally compares values at offsets -24 and -22 of a5 to set a 16-bit word at offset negative 2 of a5 before branching.
- `branch_to_offset`: Performs a short branch to a specific offset.
- `decode_object_properties`: Calls routines to decode object properties and store parameters in global variables using offsets from a frame pointer, returning a 16-bit value from the stack frame.
- `process_indexed_data`: Checks a word at an offset of negative 25478 from a4; if not zero, it pushes several values onto the stack, calls another routine, adds a word from a pointer in a5 to a global variable, calls another routine, adds a result to a global variable, and if a byte at a pointer in a5 is not zero, it calls another routine; otherwise it sets a global variable to 1 and calls another routine.
- `update_global_offset_4`: Adds 1 to a 16-bit value calculated from an offset of negative 25950 from a4, stores the result at an offset of negative 25946 from a4, calls a routine, stores 1 at an offset of negative 25942 from a4, and calls another routine.
- `execute_offset_calls_2`: Sets up a stack frame, checks if a 16-bit value at an offset of negative 25942 from a4 is zero; if not zero, it calls a routine, clears that 16-bit value, calls another routine, and returns.
- `compare_and_branch_on_longword`: Calls a routine, compares the result to a longword at a negative offset from a pointer in a4, and either calls another routine or sets a 16-bit word at a negative offset from a4 to 1.
- `clear_and_call_routine_2`: Clears a 16-bit word in a global variable and calls another routine.
- `execute_offset_calls`: Performs two consecutive calls to another routine using an offset from a value in a register.
- `update_global_offset`: This routine calls another routine to retrieve a value and then stores that value plus a constant offset into a global variable.
- `check_a5_offset_8`: Links a5, tests a longword at offset 8 of a5, and if it is non-zero, branches to another location; otherwise it sets d0 to zero, unlinks a5, and returns.
- `call_and_extend_result`: Pushes a word, pushes a pointer from offset 8 of a5 onto the stack, calls a routine, extends the result in d0 to a longword, and performs an unconditional branch.
- `execute_conditional_logic_7`: Calls another routine and stores the result in offset -2 of a5, calls another routine with a pointer from offset 8 of a5, calls another routine, and if the value at offset 12 of a5 is not zero, calls another routine; otherwise it branches to another location.
- `call_search_routine`: Pushes a word onto the stack, pushes a long from offset 12 of a5 onto the stack, calls another routine, adjusts the stack pointer, sign-extends the result to a long, and branches.
- `search_and_extend`: Pushes a word onto the stack, calls a routine with a pointer from a structure, and then extends the result in d0 before branching.
- `execute_and_branch`: Calls another routine and then performs a short branch.
- `push_and_call_subroutine_2`: Pushes two longwords from offsets 12 and 8 of a structure in a5 onto the stack, calls a routine via a pointer, and then branches to another location.
- `compare_and_branch_8`: Subtracts 13 from d0 and branches if it is zero, otherwise subtracts 14 from d0 and branches if it is zero, otherwise it branches.
- `branch_to_offset_9`: Performs a short branch to a relative offset.
- `dispatch_indirect_function_call`: Pushes two 16-bit words from offsets 12 and 8 of a structure pointed to by a5 onto the stack, calls a routine via a pointer, and then branches to another routine.
- `branch_on_decrement`: Decrements d0 by 4, and if d0 is zero, branches; otherwise it branches to another location.
- `push_and_call_swapped`: Pushes a 16-bit word from a structure at a negative offset of a5 onto the stack, calls a routine via a pointer, and then branches to another routine.
- `decrement_and_branch_4`: Subtracts three from register d0 and branches if the result is zero; otherwise, it clears d0 and performs an unconditional branch.
- `search_and_extend_result`: Pushes several values onto the stack, calls another routine, pushes the result into a new stack frame to call another routine, pushes a word and a pointer to call another routine, and finally extends the result in d0 to a longword before branching.
- `call_with_offset_2`: Pushes a word from offset minus 8 of a5 to the stack, calls another routine, and then branches to another location.
- `branch_to_instruction`: Performs a short branch to a different location in the code.
- `branch_via_table_index`: Compares d0 to 6, and if it is not less than 6, shifts d0 left by 1 and uses it to fetch a word from a table to perform a jump.
- `decrement_stack_pointer_and_call`: Pushes a 16-bit value to the stack, pushes the address of a structure offset into a5, and calls another routine.
- `process_object_interaction_logic`: Calls several routines to fetch data and perform searches based on a value retrieved from a structure, then executes a series of conditional calls and data movements depending on that value.
- `process_nested_data_structures_2`: Pushes two words onto the stack and calls a routine, then pushes two more values and calls another routine, then pushes a longword from offset 8 of a5 and calls a third routine before branching.
- `branch_to_external_routine`: Performs an unconditional short branch to another routine.
- `branch_via_table`: Compares a longword in d0 to 7, and if it is not less than 7, shifts d0 left by 1 and uses the result to fetch a word from a table to perform an indirect jump.
- `execute_conditional_logic_3`: Pushes a word to the stack, calls another routine, and if bit 3 of d0 is clear, it loads a longword from an offset of a5 into d0 and branches to another location; otherwise, it branches to another location.
- `execute_sequence`: Calls three different routines with various arguments from a5 and the stack, and then branches to another routine.
- `branch_to_offset_13`: Performs an unconditional short branch to a specific offset.
- `push_argument_and_call`: Pushes the value at a negative offset from a5 onto the stack, calls another routine, increments a7 by 4, and branches to another location.
- `check_constant_values`: Decrements register d0 twice and branches to different locations if the value reaches one or zero, otherwise it branches to another location.
- `execute_conditional_offset`: Checks a global value and, if zero, calls another routine with a decremented argument.
- `process_nested_data_structures`: Performs a sequence of calls to various routines using parameters from a stack frame, modifies values at offsets -12, -8, -20, -18, and -16 of a structure in a5, and concludes with a loop that iterates based on a value in a register.
- `iterate_table_entries`: Iterates through a table of entries using a loop counter stored at offset 4 of a structure, repeatedly invoking another routine with offsets derived from the current entry and updating a running sum in the value at offset 12 of the structure, otherwise it returns.
- `subtract_global_offsets`: The routine subtracts two 16-bit values retrieved from a table via a pointer from the two 16-bit values provided in the argument.
- `add_global_offsets`: The routine adds two 16-bit values retrieved from a table via a pointer to the two 16-bit values provided in the argument.
- `process_stack_buffers`: Calls several routines to manipulate string buffers and copy fields from a table, and returns without storing a result.
- `execute_structure_command_logic`: Calls a subroutine using a pointer stored at offset 12 of the structure in a5, then updates a 16-bit value in the structure with the result and loads a 32-bit value from offset 20 of the structure into a0 before branching.
- `update_structure_command_id`: Fetches a 16-bit command ID from a sub-structure pointed to by an offset in the current structure, stores this ID into a field at a negative offset within the current structure, and branches to a command processing routine.
- `clear_word_at_offset_4`: Clears a 16-bit word at offset 16 of a structure in a5 and branches to another location.
- `copy_word_to_offset`: Copies a 16-bit word from offset -16 of a structure in a5 to offset 16 of the same structure and branches to another location.
- `reset_structure_fields`: Clears a 16-bit word at offset 16 of a structure in a5, clears a byte at the address pointed to by offset 12 of a5, sets a 16-bit word at offset -14 to one, and branches to another location.
- `decrement_and_check_word`: Sets a 16-bit word at offset -12 of a structure in a5 to one, decrements a 16-bit word at offset 16 if it is non-zero, and branches to another location.
- `decrement_word_if_nonzero`: Decrements a 16-bit word at offset 16 of a structure in a5 if it is non-zero and branches to another location.
- `set_flag_on_mismatch`: Compares a 16-bit word at offset 16 of a structure in a5 with a 16-bit word at offset -16, sets a 16-bit word at offset -12 to one if they differ, and branches to another location.
- `increment_word_if_less`: Compares a 16-bit word at offset 16 of a structure in a5 with a 16-bit word at offset -16, increments the word at offset 16 if it is smaller, and branches to another location.
- `validate_and_shift_table_entry`: Checks if a value is within a specific range and satisfies a length condition, then calls two routines to retrieve values and performs a comparison.
- `shift_table_bytes`: Decrements a 16-bit value at offset -10 of a structure in a loop, copying a byte from a table to the next byte in the table for each iteration, then copies a byte from offset -5 of the structure to the table at an offset determined by a 16-bit value at offset 16 and increments that value.
- `process_structure_command_id`: Compares a command ID in d0 against several constants to determine which operation to perform on a structure in a5.
- `update_structure_value`: If a 16-bit word at a negative offset of a5 is not zero, it sets a 16-bit word at a different negative offset to one and moves a 16-bit word from offset 16 of a5 to offset negative 10 of a5.
- `copy_bytes_in_loop`: This routine performs a loop that copies bytes from a source address to a destination address, using values from a structure in a5 to determine the source, destination, and loop count.
- `update_structure_values`: Loads a pointer from offset 20 of a structure, writes a 16-bit value from offset -2 of the structure to offset -2 of the pointer, loads the same pointer again to write a 16-bit value from offset -4 of the structure to offset -4 of the pointer, pushes a 32-bit value from offset 8 onto the stack, and calls another routine.
- `loop_and_call_routine`: Decrements a 16-bit value in a structure, calls another routine, and loops while a value in a5 is greater than a value derived from a table.
- `process_structure_command_logic`: Checks a value in d0 to branch to different subroutines, then evaluates a command ID from a sub-structure to perform various data manipulations on the structure in a5, including updating counters, shifting bytes within a buffer, or modifying flags based on specific command constants.
- `compare_and_call_subroutine`: Loads a word from a negative offset of a structure in a5 into d0, compares it to a word at offset 16 of the same structure, and either branches, calls another routine, or branches to another location based on the result.
- `retrieve_and_call_subroutine`: Calls another routine, then calls the same routine again using two values read from a global variable at specific offsets, and returns a 16-bit value read from a third offset in that same global variable.
- `restore_and_jump_indirect`: Restores d3 through d7 and a1 through a7 from the stack and then jumps to the address stored in a1.
- `jump_if_not_zero`: Checks a 16-bit value from the stack; if it is not zero, it jumps to the address in a1, otherwise it sets d0 to 1, restores d3 through d7 and a1 through a7 from the stack, and jumps to the address in a1.
- `initialize_system_state_2`: Calls another routine, clears a block of memory using a loop, updates global variables, and executes indirect function calls based on the state of a global variable.
- `load_constant_to_a4`: Loads the constant value 7ffe into a4 and returns.
- `parse_and_execute_command`: Allocates memory based on a value from a global variable, initializes a structure with specific constants, and performs several indirect calls to process data, handle windowing, and manage file I/O. It updates various fields in a structure at offset -24446 of a4 and returns a pointer in a2.
- `move_d0_to_d5`: Copies the long value from d0 to d5.
- `parse_and_tokenize_string`: Calculates a string length from a structure, calls another routine to allocate memory, and then performs a loop to tokenize the string into individual words by handling whitespace and quoted substrings.
- `set_limit_and_branch`: Sets d0 to a 16-bit constant and branches to another routine.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `process_window_data`: Calls several library functions to manipulate data, including a loop that stores a value from register d0 into a global variable, performs conditional calls based on the results, and updates a structure at offset 164 and 156 using values derived from register d6.
- `copy_long_word`: Copies a 32-bit value from register d0 to register d5.
- `call_format_string_parser`: Pushes a value from offset 8 of the current stack frame onto the stack and calls a routine via a PC-relative address to parse a format string, then cleans up the stack and returns.
- `load_conditional_word`: Creates a stack frame, saves registers, loads a pointer from offset 14 of a5 into a2, compares the word at offset 18 of a5 to 4, and if they are equal, loads a pointer from offset 8 of a5 into a0 and moves a longword from that pointer into d4.
- `load_word_from_pointer`: Checks a word at offset 12 of a5, and if it is not less than or equal to zero, loads a pointer from offset 8 of a5 into a0 and moves a word from that pointer into d0.
- `perform_division_logic_2`: Retrieves a longword from offset 8 of a structure in a5, checks if a value at offset 12 is negative, and if so, performs a series of calls to handle signed division and character mapping.
- `parse_format_string_specifiers`: Parses a format string to extract numeric values, floating-point numbers, or strings, and populates a local buffer with the results.
- `clear_word_and_branch_3`: Clears a 16-bit word at offset negative 14 of a5 and then branches to another routine.
- `parse_input_string_2`: Parses a numeric value from a string by multiplying the current accumulator by ten and adding the ASCII value of the current character, then checks for specific character markers like decimal points or special symbols to update a structure in a5.
- `clear_local_word_and_branch`: Clears a 16-bit word at offset 12 of a structure in a5 and branches to another location.
- `parse_numeric_format_string`: Calculates a value by multiplying a local 16-bit variable by ten and adding a value from d4, then subtracts 48 from the result.
- `fetch_next_format_specifier`: Compares a 16-bit value in d4 to 104; if they match, it increments a pointer in a3, reads a byte from that pointer into d0, sign-extends it to a 32-bit value in d0, and returns.
- `set_constant_and_branch`: Writes the word 8 to offset -18 of a5 and branches to another routine.
- `set_constant_and_branch_2`: Writes the word 10 to offset -18 of a5 and branches to another routine.
- `set_constant_and_branch_3`: Writes the word 16 to offset -18 of a5 and branches to another routine.
- `execute_formatted_string_function_calls`: Sets a 16-bit value to negative 22 at offset 18 of the structure in a5, pushes several values from the structure onto the stack, and calls another routine.
- `extract_string_pointer_and_test_byte`: Loads a pointer from a structure offset, updates that offset by four, and stores the value at the new pointer into a structure offset.
- `pop_format_specifier_byte`: Loads a word from a pointer stored at offset negative four from a5, increments that pointer by two, and stores the low byte of the loaded word into a local buffer at offset negative two hundred and nine from a5 before branching.
- `dispatch_printf_format_specifiers`: Evaluates a value in d0 to determine a format specifier type, writes a corresponding constant to offset negative 18 of the structure in a5, and branches to a specific handler routine based on the value.
- `execute_string_formatted_function_calls`: Iterates through a string to identify specific character sequences, decrementing a counter and calling a function via a pointer in a5, then continues to process subsequent characters in a loop until a termination condition is met.
- `branch_to_offset_12`: Performs a relative branch to a specific instruction offset.
- `execute_looping_call`: Pushes a 16-bit word from a location relative to a5 onto the stack, calls a routine at the address in a2, and loops if the result in d0 is not negative 32767.
- `increment_and_compare_counter`: Increments a word at offset -6 of a5, loads a word from offset -14 of a5 into d0, decrements the word at offset -14 of a5, and compares d0 to the word at offset -16 of a5; if d0 is greater, it clears the word at offset -18 of a5 and branches to another location.
- `increment_and_call`: Increments a 32-bit value at an offset relative to a5, loads a byte from the new address into d0, and calls a routine at the address in a2, looping if the result in d0 is not negative 32767.
- `increment_and_compare_offset`: Increments a word at offset -18 of a5, loads a pointer from offset -22 of a5, and if the byte at that pointer is non-zero, compares a word at offset -12 of a5 with a word at offset -4 of a5 and performs arithmetic on offset -6 of a5.
- `call_with_constant`: Pushes the constant 32 onto the stack, calls a routine at the address in a2, and loops if the result in d0 is not negative 32767.
- `increment_and_compare`: Increments a 16-bit value at offset -6 of a structure in a5, moves a 16-bit value from offset -14 of a5 to d0, decrements that value, and branches to another routine if d0 is greater than the value at offset -10 of a5.
- `execute_string_formatted_function_calls_2`: Pushes a 16-bit value from d4 onto the stack, calls a routine via a pointer in a2, compares d0 to 65535, adjusts the stack pointer, and branches if d0 is not 65535 or otherwise branches to another routine.
- `increment_offset_and_branch`: Increments a 16-bit value at an offset relative to a5 and branches to another routine.
- `load_and_branch_3`: Loads a 16-bit value from an offset of negative 6 from a5 into d0 and branches to another routine.
- `perform_signed_division`: Reads two signed inputs in d0 and d1, flips a sign flag based on their signs, calls another routine, and optionally negates the result before combining it with the original d1 value.
- `prepare_signed_division`: Saves d1 and d4 to the stack, clears d4, and checks the sign of d0 and d1.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original d1 contents with the value left in d1 by the call.
- `wrap_perform_division_logic`: Pushes the contents of register d1 onto the stack, calls perform_division_logic, restores the original value to d1, and then returns if d0 is non-zero.
- `perform_division_logic`: Swaps the high and low words of d1, then performs a series of conditional divisions and swaps using d0, d1, d2, and d3 before returning.
- `swap_and_rotate_words`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `call_buffer_state_update`: This routine calls another routine to manage buffer states and then returns.
- `process_conditional_pointer`: Loads a word from offset 8 of a structure in a5, compares it to 10, and if equal, checks bit 7 of a byte at offset 12 of a pointer found at offset 10 of the structure; if that bit is set, it calls another routine with a pointer and a value of negative 1.
- `branch_to_middle_of_routine_3`: Performs a short relative branch into another routine.
- `call_offset_relative_routine`: Loads a pointer from offset 10 of a structure in a5, compares a value at offset 4 of that pointer to a value at offset 0, and if the comparison does not branch, calls another routine using a 16-bit value from offset 8 of the structure in a5 masked with 255.
- `update_byte_from_offset`: Copies a byte from an offset relative to a5 to a location pointed to by a2, then increments the pointer in a2 and loops.
- `iterate_and_clear_structure`: Sets up a stack frame, calls a routine, and then compares two memory locations relative to a4 to determine whether to branch or return.
- `check_structure_bits`: Loads a pointer from an offset of 8 in a5, checks if the value at that pointer is non-zero, and if it is zero, sets d0 to 255 and returns; otherwise it returns immediately.
- `initialize_structure_fields_3`: Checks bits in a byte at an offset of a pointer, and depending on those bits, calls one or more subroutines and clears several fields at offsets of the pointer.
- `test_and_store_pc_relative_address`: Tests bit 4 of a byte at offset 12 of a structure pointed to by a5 plus 8, and if set, stores a relative address into a global variable at a negative offset from a4; otherwise it returns.
- `update_status_flags_4`: Tests bit 2 of a byte at offset 12 of a2; if set, it calculates a difference using a pointer at offset 8 of a2, calls another routine, and if the result matches the difference, it clears a pointer and two longwords at the start of a2 and sets bit 2 of offset 12 of a2.
- `update_conditional_byte_2`: Compares a 16-bit value at an offset of negative 3 from a5 to 65535; if equal, it clears bit 2 of a 16-bit value at an offset of negative 3 from a2, clears two longwords at a2 and an offset of 4 from a2, and branches to another routine; otherwise it branches to another routine.
- `update_conditional_byte`: Checks if a longword at offset 8 of a2 is zero; if non-zero, it branches to another routine; otherwise, it calls a routine with a pointer to a byte at offset 13 of a5, and if the returned value is not 1, it clears a longword at offset 0 of a2, sets a byte at offset 3 of a2, and returns a word of 65535; if the returned value is 1, it returns a word from offset 12 of a5.
- `update_structure_and_byte`: Updates a long value at offset 4 of a structure by adding a 16-bit value from offset 16 to a long value from offset 8, sets bit 2 of a byte at offset 12, and copies a byte from offset 13 of another structure to a location pointed to by a value at offset 0 of the first structure.
- `search_and_clear_structure`: Loads a pointer from a global variable, checks a byte at offset 12 of the pointed-to structure, and if non-zero, iterates through a list of structures by adding 22 to the pointer.
- `branch_to_offset_4662`: Performs a short branch to a location 4662 bytes ahead in the code.
- `clear_structure_fields_3`: Clears the long value at offset 0, the long value at offset 4, and the long value at offset 8 of a structure in a6, then branches to another routine.
- `update_table_entry_3`: Calls a routine with the word 1024, and if the result is zero, writes a word to an offset of a pointer and sets a longword at another offset; otherwise, it writes the result to an offset and calls another routine.
- `modify_and_call_table_entry`: Sets a word at offset 16 of a2, sets bit 1 of a byte at offset 12 of a2, loads a longword from offset -4 of a5 into offset 8 of a2, extends a byte from offset 13 of a2 to a word, pushes that word onto the stack, calls a routine, and then modifies a byte at offset 12 of a2 based on the result.
- `load_and_branch`: Sets a word in a5, saves registers a2 and a3 to the stack, loads a pointer from a global variable into a2, and branches to another routine.
- `traverse_linked_list`: Follows a chain of pointers in a table, calling a subroutine for each element, and sets a global variable to zero upon completion.
- `store_subroutine_pointer`: Stores the address of a local label into a global variable at offset -24412 of a4, calls another routine with a value from offset 8 of a5 plus 8, and if the result is non-zero, returns it; otherwise it returns zero.
- `store_and_increment_pointer`: Stores a pointer from a global variable into a structure, copies a long from offset 8 of another structure to offset 4 of the first, updates the global pointer with the first structure's address, and branches to another routine after adding 8 to a register.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `initialize_local_pointer`: Creates a stack frame, saves registers a2 and a3, clears a3, loads a pointer from an offset of a global variable into a2, and branches to another location.
- `traverse_pointer_chain_2`: Compares offset 8 of a5 with a2, and if they are not equal, follows a pointer chain starting at a2, potentially updating a3 and d0 before restoring registers and returning.
- `move_longword_conditionally`: Moves the value from a3 to d0, and if d0 is non-zero, copies the longword at the address in a2 to the address in a3.
- `call_library_with_stack_cleanup`: Loads a pointer from a structure, adds 8 to a value from offset 4 of that structure, pushes two values onto the stack, calls a routine, and then cleans up the stack before branching to another routine.
- `fetch_table_entry_2`: Calculates an offset by multiplying a word from offset 8 of a5 by six, adds it to a global pointer, and if the resulting pointer is non-zero and the value at offset 8 of a5 is within a specific range, calls another routine; otherwise, it stores a constant in a global variable and returns.
- `calculate_and_call_routine`: Multiplies a 16-bit value from offset 8 of a structure in a5 by 6, uses the result to fetch a pointer from a table into a0, pushes that pointer and the multiplied value onto the stack, calls another routine, and then either returns or sets d0 to 1.
- `clears_d0_and_branches`: Sets register d0 to zero and performs a short relative branch to another routine.
- `execute_callback_pointer`: Creates a stack frame, checks a global variable and, if non-zero, executes the subroutine pointed to by that variable, then pushes a word from offset 8 of a5 onto the stack and calls another routine before returning.
- `store_extended_word`: Pushes d4 onto the stack, extends a word from offset 8 of a5 to a longword, stores it at offset -4 of a5, and if a global pointer is non-zero, clears d4 and branches.
- `execute_conditional_sequence`: Increments d4 in a loop until it reaches a value from a table, then calls a routine with a multiplied value, followed by a series of conditional checks that either call various routines, move a value to a pointer, or call a library function based on the contents of a table and a bit test in a system structure.
- `execute_conditional_library_calls`: Checks a global variable; if it is non-zero, it calls another routine, otherwise it checks a second global variable, and if that is non-zero, it calls a routine with a 16-bit value from a global variable and then calls another routine with a 32-bit value derived from a global variable and a constant.
- `execute_library_calls`: Calls two library functions via offsets from global variables, then loads a pointer from a global variable into a7 and returns a value from a structure in a5.
- `pop_and_unlk`: Pops a 32-bit value from the stack into d4, unlinks a5, and returns.
- `call_close_3`: Jumps to a library function to close a file.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir_3`: Jumps to a library function to retrieve the current directory.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file_2`: Jumps to a library function to delete a file.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_exit_3`: Jumps to a library function to exit the program.
- `call_exit`: Wrapper that calls the dos library function Exit and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_load_seg`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `call_lock_3`: Jumps to a library function to lock a file.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open_wrapper`: Calls the DOS open function via a jump to an offset in a table.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read_wrapper`: Calls the DOS read function via a jump to an offset in a table.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek_wrapper`: Calls the DOS seek function via a jump to an offset in a table.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_load_seg`: Wrapper that calls the dos library function UnLoadSeg and returns its result to the caller.
- `call_unlock_wrapper`: Calls the DOS unlock function via a jump to an offset in a table.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write_wrapper`: Calls the DOS write function via a jump to an offset in a table.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_414_3`: Calls a library function at an offset of minus 414 from a base address stored in a global variable.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_with_parameter`: Creates a stack frame, saves registers, pushes a longword onto the stack, calls a routine, and if the result in d0 is not negative one, clears d0 and restores registers before returning; otherwise it restores registers and returns.
- `call_library_functions`: Pushes two values onto the stack, calls a routine, moves the result to a2, and if the result is not zero, saves d4 to the stack and calls another routine; otherwise it clears d0 and returns.
- `transfer_and_call_data`: Copies a 32-bit value and a byte from a structure in a5 to a structure in a2, sets a byte at offset 8 of the structure in a2 to 4, clears a byte at offset 14, moves d4 to offset 15, calls another routine, stores the result in offset 16 of the structure in a2, and then either returns or calls another routine using a pointer from a2.
- `setup_pointer_chain_2`: Passes the address of offset 20 of a structure to a routine, cleans up the stack, and branches to another routine.
- `release_and_reset_record`: If a long at offset 16 of a structure is non-zero, it calls another routine, then sets a byte at offset 8 to all ones, sets a long at offset 20 to all ones, calls a routine with a byte from offset 15, and calls another routine with a constant.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_198_4`: Calls a library function at an offset of minus 198 from a base address stored in a global variable.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_294_3`: Calls a library function at an offset of minus 294 from a base address stored in a global variable.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_3`: Calls a library function at an offset of minus 132 from a base address stored in a global variable.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210_4`: Calls a library function at an offset of minus 210 from a base address stored in a global variable.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `jump_to_library_function`: Performs an indirect jump to a location calculated from a global variable and a fixed offset.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `jump_to_library_function_2`: Performs an indirect jump to a location calculated from a global variable and a different fixed offset.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `store_call_result`: Calls a function via a pointer from a table and stores the returned value into a global variable.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 16 into d0, then returns.
- `initialize_audio_hardware`: Opens an audio library, allocates memory, and configures multiple audio channels by populating a structure with buffer addresses, sample rates, and playback parameters.
- `execute_multiple_calls`: Executes several indirect function calls and runs a loop that calls a function a specific number of times based on a condition.
- `decode_tile_bits`: Extracts bits from a source buffer of 16-bit tiles and writes them into four separate destination bitplanes using nested loops to iterate through a specified range of rows and columns.
- `convert_and_store_bytes`: The routine performs a byte swap on 16-bit values from a source buffer and stores the results into a destination buffer, then clears two words at the end of the destination.
- `increment_and_return_counter`: The routine increments a 16-bit value stored in a global variable and returns the new value.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_and_return_counter`: The routine subtracts one from a 16-bit value stored in a global variable and returns the new value.
- `restore_and_shake_object`: The routine saves a 16-bit pair from a structure pointed to by a global variable into two global variables, then repeatedly moves the object’s coordinates by a small offset, calls three callbacks and a looped callback, restores the coordinates with the opposite offset, and repeats the callbacks.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `process_structure_data`: Calls another routine and stores the result at offset 8 of a structure, then if a word at offset 16 is not negative one, calls another routine, clears two words at offset 0 and 2 of a pointer from offset 8, and if a word at offset 18 is negative, calls another routine to store results at offsets 4 and 6 of the pointer, otherwise it calls another routine and branches.
- `initialize_structure_member`: Tests a word at offset 18 of a5; if it is non-zero, it branches, otherwise it loads a pointer from offset 8 of a5 and writes the word 192 to offset 6 of that pointer.
- `process_string_data_structure`: Iterates through a null-terminated string, calling another routine for each character to calculate a sum and track a maximum value.
- `update_calculated_values`: Squares a 16-bit value in a structure, and if a condition is met, updates a word in a separate structure before storing the squared value in another field and calling another routine.
- `compare_string_and_call_routine`: Checks if a byte at offset 1 of a pointer (from offset 8 of a structure in a5) is a line feed or carriage return, increments the pointer, and if a specific word comparison fails, calls another routine with a word from offset -12 and a pointer from offset -8, then updates the pointer at offset 8 with a new value.
- `increment_pointer_and_load`: Loads a pointer from offset 8 of a structure in a5 into a0, increments the longword at the address in a0, loads a word from offset negative 12 of the structure into d0, and branches to another routine.
- `parse_input_token`: Iterates through a string to identify tokens separated by spaces, null terminators, or line endings, while handling carriage return/line feed sequences.
- `increment_pointer_and_branch`: Increments a long value at a location pointed to by offset 8 of the structure in a5, loads a word from offset -12 of the structure in a5 into d0, and branches to another routine.
- `parse_input_string`: Iterates through a string character by character, checking for null terminators, spaces, carriage returns, or line feeds.
- `load_word_and_branch_2`: Loads a word from offset -12 of the structure in a5 into d0 and branches to another routine.
- `parse_space_delimited_string`: Checks if a character at a specific offset in a structure is a space, then calls another routine to process a substring.
- `copy_long_and_branch`: Copies a long value from offset -4 of the structure in a5 to a location pointed to by offset 8 of the structure in a5, loads a word from offset -14 of the structure in a5 into d0, and branches to another routine.
- `parse_string_buffer`: Increments a counter and a pointer within a structure, then enters a loop that reads characters from a string buffer to identify delimiters such as carriage returns, line feeds, spaces, or null terminators.
- `jump_to_string_comparison`: Performs a short branch to a location within the routine that handles line feed and carriage return detection during string parsing.
- `branch_to_offset_574`: Performs a short branch to a location 574 bytes ahead in the instruction stream.
- `process_structure_data_2`: Links a stack frame, clears a word at offset -12 of a5, calls another routine, and if the result in d0 is not -65535, pushes a word from offset 20 of a5 and calls another routine; it then calculates a difference between words from two pointers in a5, updates offset -14 of a5, and calls another routine using a pointer from offset 8 of a5.
- `divide_offset_difference`: Subtracts a word at offset 10 from a word at offset 14 of a structure in a5, divides the result by two, and stores the result at offset 16 of the structure.
- `store_offset_difference`: Subtracts a word at offset 10 from a word at offset 14 of a structure in a5 and stores the result at offset 16 of the structure.
- `clear_offset_value`: Clears the 16-bit word at offset 16 of a structure in a5 and then branches to another location.
- `decrement_and_branch_3`: Subtracts 1 from d0 and branches if d0 is zero, otherwise it subtracts 1 again and branches if d0 is zero, otherwise it branches to another location.
- `calculate_scaled_offset_and_branch`: Multiplies a word at offset 12 of a structure by a value, adds it to a word from a pointer at offset 14, and calls another routine with the result.
- `decrement_and_branch_2`: Pushes two values from a structure onto the stack, calls another routine, increments a word at offset 14 of a structure, and branches to another routine.
- `call_with_stack_argument`: Pushes a word from offset -20 of the structure in a5 onto the stack, calls another routine, adjusts the stack pointer, unlocks a5, and returns.
- `call_and_recall_with_result`: The routine calls another routine with the argument stored at offset 8 of the structure in a5, stores the 16-bit result, then calls a second routine with the original argument.
- `self_calling_wrapper`: Receives a single argument, calls another routine with that argument, captures the returned 16-bit value, and then calls another routine using the original argument and the captured value.
- `wrap_and_call_with_result`: Calls a routine using a pointer stored at offset 8 of a structure, captures the returned 16-bit value, and then calls another routine using the original pointer and the captured value.
- `fetch_indexed_byte`: Decrements a word at offset 14 of a structure, and if it is not zero, increments a word at offset 12, fetches a byte from a table using an index from offset 14 and the pointer at offset 8, and calls another routine with that byte on the stack.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `update_global_values_2`: Retrieves two 16-bit values from a global structure, adds them together, and then calls another routine using the result and a pointer to a local variable.
- `update_task_state`: Stores a value into several consecutive global memory locations and calculates an offset based on an input parameter.
- `propagate_object_state_2`: Pushes a value from a negative offset of a4 onto the stack and calls a routine at a fixed address.
- `evaluate_pointer_condition`: Clears a 16-bit word at offset -2 of a structure in a5, calls two other routines, copies a longword from a global variable into offset -6 of the structure in a5, compares that value with another longword from a global variable, and if they are not equal, checks if a bit is set in a word at the address pointed to by the value in offset -6 of a5, setting offset -2 of the structure in a5 to 1 if the bit is set.
- `compare_and_branch_pointer_2`: Pushes a value from offset -6 of a structure in a5 onto the stack, calls a routine, stores the result back into offset -6 of the structure, and branches.
- `clear_and_store_word`: If a word at offset -2 of a structure in a5 is non-zero, it pushes three values onto the stack, calls another routine, clears a word at a pointer from offset -6 of a5, calls another routine with a pointer from a global variable, and stores the result in that global variable.
- `clear_and_fill_struct_2`: Pushes a value from offset 10 of a structure in a5 onto the stack, calls a routine, calls another routine, and returns a word from offset -2 of the structure in a5.
- `execute_conditional_branch_2`: Pushes a value from a structure onto the stack, calls another routine, and then branches to another routine if the result in d0 is not zero.
- `unlcknowledge_stack_pointer`: Returns immediately after unlocking the stack pointer in a5.
- `check_handler_condition`: Calls another routine, then compares a longword at offset negative 26724 of a4 with a longword at offset negative 26720 of a4, and if they are not equal, checks if a bit is set in a word at offset negative 6 of a structure in a0.
- `call_subroutine_with_stack_frame`: Pushes a 32-bit value from an offset of 6 bytes below a5 onto the stack, calls another routine, increments the stack pointer by 4, stores the result from d0 back into the same offset below a5, and branches to another location.
- `invoke_indirect_handler_2`: Checks if a longword at offset 10 of a structure in a5 is non-zero and if a word at offset -2 of the same structure is non-zero, then pushes three values onto the stack and calls another routine.
- `invoke_indirect_handler`: Pushes a value from offset 10 of a structure in a5 onto the stack, calls a routine using a branch-to-subroutine instruction, calls another routine, and returns a word from offset -2 of the structure in a5.
- `execute_conditional_branch`: Sets up a stack frame, pushes the address of a word at offset -14 of a structure in a5 onto the stack, pushes the constant 2 onto the stack, calls another routine, and if the result in d0 is non-zero, sets d0 to 1.
- `unlock_address_register_and_return`: Clears register d0 and unlocks the address register a5 before returning.
- `update_global_pointers`: Calls a routine, then calls another routine with a value from offset 8 of a structure and a value from a negative offset of a4, stores the result in the negative offset of a4, and if the new value matches a second value at a different negative offset of a4, calls a routine again and updates that second value before calling a final routine and returning.
- `clear_and_fill_struct`: Clears a two-byte field at the start of a structure pointed to by a register, then calls another routine with a pointer to a field, stores a returned 32-bit value at an offset, calls another routine, and stores a 16-bit value into a structure offset.
- `compare_and_branch_pointer`: The routine adds a constant to a parameter and compares it against a value in a global table, potentially replacing the parameter with a different value from the table if they match.
- `shuffle_and_store_data`: Performs a series of data transfers and calls between local buffers and other routines, finally storing a result in a global variable.
- `call_and_return_parameter`: Calls another routine with the incoming parameter and then returns that same parameter unchanged.
- `store_shuffled_data`: Stores five 16-bit words from a source pointer into a destination buffer using offsets retrieved from a global table.
- `load_shuffled_data`: Loads five 16-bit words from a source buffer into a destination pointer using offsets retrieved from a global table.
- `load_extended_word`: Sets up a stack frame, loads a pointer from offset 8 of a structure in a5 into a0, loads a 16-bit word from offset 2 of the address in a0 into d0, extends d0 to a 32-bit value, and branches to another location.
- `store_word_from_table`: Moves a pointer from offset 8 of a structure in a5 to a0, pushes a word from offset 4 of the structure in a0 onto the stack, calls another routine, and stores the result in a negative offset of a4.
- `subtract_and_unstack`: Subtracts a longword from d0 and, if the result is zero, performs an unstack operation and returns; otherwise it returns.
- `return_offset_longword`: Returns a longword stored at a negative offset from a value in a register.
- `fetch_word_value`: Returns a word value stored at a specific offset from a global pointer.
- `dereference_offset_pointer`: Loads a longword from a memory location calculated by adding a negative offset to a value held in a register.
- `increment_and_test_byte`: Increments a longword at offset negative 4 of a structure in a5, then checks if the byte at the address stored at offset 8 of a5 is non-zero.
- `subtract_and_decrement`: Subtracts the value at offset 8 of a5 from the value at offset negative 4 of a5, decrements the result by one, and returns it.
- `copy_byte_loop`: Loops while a byte at offset 12 of a structure in a5 is non-zero, copying bytes from offset 12 to offset 8 and incrementing both pointers.
- `return_long_word`: Returns a 32-bit value from offset negative 4 of a pointer in a5.
- `copy_byte_with_decrement`: Loops while a byte at offset 12 of a structure in a5 is non-zero, copying bytes from offset 12 to offset 8 and incrementing both pointers, and decrements a word at offset 10 of a5 if the byte is non-zero.
- `increment_pointer_and_clear_byte`: Loads a pointer from offset 8 of a5, increments the value at that offset, clears a byte at the loaded pointer, and returns a 32-bit value from offset negative 4 of a5.
- `call_three_routines`: Calls another routine with a pointer from a5, increments d0, calls another routine with the incremented value, and then calls a third routine with the result of the second call.
- `increment_pointer_if_not_zero`: Saves a pointer from offset eight of the structure in a5 to the stack, checks if the byte at that pointer is non-zero, and if so, increments the pointer at offset eight of the structure in a5 before branching to another routine.
- `copy_and_increment_byte_stream`: Copies a byte from a pointer at offset negative four of a structure to a destination at offset eight of the same structure, increments both pointers by one, and branches to another routine if the copied byte is not zero; otherwise it branches to a different routine.
- `fetch_and_unlock_pointer`: Loads a longword from an offset of negative four from a pointer in a5 into d0, unlocks the semaphore in a5, and returns.
- `decrement_and_increment_pointer_loop`: Saves a pointer from offset eight of the structure in a5 to the stack, decrements a word at offset sixteen of the structure in a5, checks if the byte at the pointer is non-zero, and if so, increments the pointer at offset eight and decrements the word at offset sixteen before branching to another routine.
- `copy_bytes_until_zero`: Loops while a word at offset 16 of a structure in a5 is greater than zero and the byte at offset 12 is non-zero, copying bytes from offset 12 to offset 8 and incrementing both pointers.
- `clear_pointer_and_return_value`: Clears a byte at an offset from a pointer in a5 and returns a 32-bit value from offset negative 4 of a5.
- `setup_link_and_branch`: Sets up a stack frame and branches to another routine.
- `compare_string_pointers`: Increments two 32-bit values at offsets 8 and 12 from a pointer in a5 within a loop that continues as long as the bytes pointed to by those values are equal and the byte at offset 8 is not zero, then returns the difference between the two values.
- `link_stack_and_branch`: Prepares the stack frame for the structure in a5 and performs a short branch to another routine.
- `compare_byte_streams`: Loops while a word at offset 16 of a structure in a5 is non-zero and the byte at offset 8 matches the byte at offset 12, incrementing both pointers, then returns the difference between the bytes or zero.
- `unlock_and_return_zero`: Unlocks the semaphore associated with the address in a5 and returns zero in d0.
- `branch_to_parser`: Creates a stack frame and branches to another location.
- `parse_numeric_string`: Skips spaces in a string, handles a sign character, and then iterates through characters to convert decimal or hexadecimal digits into a 16-bit value stored at offset negative 2 of a structure in a5.
- `branch_to_another_routine`: Performs a short relative branch to another routine.
- `process_hex_digit`: Increments a pointer at offset 8 of a structure in a5, loads a byte from the address in a0, calls another routine, and then stores the result into offset negative 3 of the structure in a5 after subtracting 48 if the byte is between 48 and 57.
- `adjust_byte_value_in_range`: Compares a byte at offset negative three of a structure in a5 to decimal 97 and 102, and if it is within that range, subtracts 87 from it before branching to another routine.
- `branch_to_multiplication_routine`: Performs a short branch to another routine that multiplies two 16-bit words located at offsets negative 6 and negative 2 from a pointer in a5.
- `modify_structure_value`: Loads a byte from offset negative 3 of a structure in a5 and sign-extends it to d0, loads a word from offset negative 2 of the structure into d1, shifts d1 left by 4 bits, adds d1 to d0, stores the result in offset negative 2 of the structure, and branches to another routine.
- `multiply_two_words`: Multiplies a 16-bit word from offset negative 6 of a pointer in a5 by a 16-bit word from offset negative 2 of a5 and returns the result in d0.
- `push_extended_value_and_call`: Extends a word from offset 8 of a structure in a5 to a long, pushes it onto the stack, and calls another routine; otherwise it returns.
- `set_and_call_function`: Stores the value 45 at offset 10 of a structure, calls another routine with a negated value from offset 8 and the value at offset 11, then returns the value at offset 10.
- `process_and_store_byte`: Loads a pointer from offset 12 of a structure in a5, extends a word from offset 16 of the structure to a long, calls another routine, stores the byte from d0 into offset -5 of the structure, and if that byte is greater than 9, sets d0 to 87.
- `calculate_and_store_value`: Adds a byte from offset negative 5 of a structure in a5 to 48, stores the result at a pointer in offset negative 4, calls another routine to update offset 8 of the structure, and if the result is not higher than zero, clears a byte at a pointer in offset negative 4 and calls another routine.
- `reverse_byte_range_in_structure`: Calls another routine, calculates a new value by adding a longword from offset eight of a structure in a5 to d0 and subtracting one, stores this in offset negative ten of the structure, and then performs a byte-swapping loop between two pointers derived from offset eight and the calculated value.
- `load_long_from_a5`: Loads a 32-bit long from an offset of 8 in a structure pointed to by a5 into d0 and returns.
- `transform_character_byte`: Compares a byte at offset 9 of a structure in a5 to 65 and 90, and if the byte is between these values, sign-extends it to d0, adds 32 to d0, sign-extends the result, and branches to another routine; otherwise it branches to another routine.
- `load_short_from_a5`: Loads a byte from an offset of 9 in a structure pointed to by a5, sign-extends it to a 16-bit word in d0, and returns.
- `increment_pointer_if_byte_mismatches`: Loads a pointer from offset eight of the structure in a5, checks if the byte at that pointer is non-zero and if it matches a byte at offset thirteen of the structure in a5, and if it does not match, increments the pointer at offset eight before branching to another routine.
- `compare_and_load_pointer`: Loads a pointer from offset 8 of a structure in a5 into a0, loads a byte from a0 into d0, compares d0 to a byte at offset 13 of the structure in a5, and if they are not equal branches to another routine; otherwise it loads the pointer from offset 8 of the structure into d0 and branches to another routine.
- `unlock_address_register_a5_2`: Clears the lock bit of address register a5 and returns with zero in register d0.
- `transfer_stack_pointer`: Links the stack frame and moves a longword from offset 8 of a structure in a5 to offset negative 4 of the stack, then branches to another routine.
- `transform_and_store_string_bytes`: Repeatedly calls another routine to read bytes, stores each byte into a memory location pointed to by a value on the stack, and increments a counter until a zero byte is encountered, then returns a 32-bit value from offset eight of the structure in a5.
- `increment_pointer_if_byte_mismatches_2`: Checks if a byte at a pointer from offset 8 of a5 is non-zero; if so, it calls another routine with the byte and a value from offset 16, and if the result is non-zero, increments the pointer in offset 8 and loops.
- `clear_byte_at_pointer_2`: Checks a byte at a pointer stored in offset 8 of a structure in a5; if the byte is non-zero, it returns, otherwise it clears the byte at the pointer in offset 12 of the structure and returns.
- `copy_bytes_loop_3`: Decrements a word at offset 20 of a structure in a5 and loops if it is not zero, provided a byte at a pointer in offset 8 is non-zero and a routine call returns zero, copying bytes from the pointer in offset 8 to the pointer in offset 12 while incrementing both pointers.
- `clear_byte_and_load_pointer`: Loads a pointer from offset 12 of a structure in a5 into a0, clears the byte at the address in a0, loads the pointer from offset 8 of the structure into d0, and branches to another routine.
- `test_and_branch_pointer`: Tests a long value at offset 8 of a structure in a5; if it is non-zero, it returns, otherwise it sets d0 to 0 and returns.
- `branch_to_middle_of_routine_4`: Performs a short branch to a location inside another routine.
- `check_byte_range`: Loads a pointer from offset 8 of a structure in a5 into a0, compares the byte at a0 to 32 and 126, and if the byte is not within this range, sets d0 to 0 and branches to another routine; otherwise it branches to another routine.
- `increment_pointer_and_test_zero`: Increments a pointer at offset eight of the structure in a5, checks if the byte at the new pointer is non-zero, and if it is zero, sets d0 to one before branching to another routine.
- `compare_and_copy_strings`: Compares two null-terminated or asterisk-terminated strings for equality and, if both end with an asterisk, recursively processes the remaining characters to copy segments into local stack buffers and then calls another routine three times with these buffers and the remaining string pointer.
- `increment_pointers_and_compare`: Loads a pointer from offset 12 of a structure in a5 into a0, increments the byte at offset 12 of the structure, loads a pointer from offset 8 of the structure into a1, increments the pointer at offset 8 of the structure, compares the byte at a0 to the byte at a1, and if they are equal branches to another routine; otherwise it sets d0 to 0, unlocks a5, and returns.
- `check_and_clear_counter`: Loads a pointer from offset 12 of a structure in a5, checks if the byte at that pointer is zero, and if not, checks if the byte at offset 12 of the structure is 42; if it is, it clears a 16-bit word at offset -2 of the structure in a5.
- `process_buffer_and_clear_byte`: Loops through a buffer pointed to by offset 16 of a structure in a5, copying bytes to a destination at offset -32 of a5 using an index from offset -2, and if a specific condition is met, clears a byte at an offset determined by that index and increments pointers at offsets 12 and 16.
- `clear_word_and_branch_2`: Clears a 16-bit word at offset 3 of the structure in a5 and branches to another routine.
- `loop_and_clear_buffer`: Pushes two pointers from offsets 8 and 12 of a structure in a5 onto the stack and calls another routine; if the return value in d0 is non-zero, it enters a loop that increments the pointer at offset 8 and a word at offset -2 of the structure while copying bytes from the pointer to a buffer at offset -62, then clears a byte in that buffer and calls the same routine again with the pointers from offsets 8 and 12.
- `clear_and_call_routines`: Clears a byte at a pointer in offset 20 of a structure in a5, calls another routine three times with different parameters and stack setups, and then sets d0 to 1 before branching to another routine.
- `strip_path_delimiter`: Calculates the length of a string by calling another routine, then checks if the character before the null terminator is a slash or backslash and replaces it with a null terminator if it is.
- `execute_recursive_calls`: Performs several calls using values from a structure in a5, and if a byte at an offset from a pointer in a5 is equal to 58, it calls another routine and stores the result in a structure at offset negative 8 of a5.
- `compare_and_store_flag`: Pushes the address of a label onto the stack, calls another routine, stores the result in a structure at a negative offset from a5, calls another routine, compares the new result in d0 with the previously stored value in d1, and stores a bitwise result in a global variable at a negative offset from a4 before returning.
- `copy_d0_to_d5`: Copies the contents of register d0 to register d5.
- `manage_file_lock`: This routine calls another routine to obtain a value, updates a global variable, and attempts to lock a file, setting a status flag based on the result.
- `process_and_store_result`: Calls a routine, stores the result in a global variable, and then branches to a relative offset.
- `jump_via_table`: Compares a longword in d0 to five, and if it is not less than five, shifts d0 left, uses the result to index into a table, and jumps to the resulting address.
- `unlock_address_register_five`: Unlocks address register five and returns immediately.
- `move_word_register`: Moves a 16-bit word from register d0 to register d5.
- `loop_until_match`: Increments a 16-bit value at offset minus 2 of a5 in a loop, using it as an index to test a byte at offset 8 of a pointer in a0, and exits the loop if the byte is 58 or if the byte is zero.
- `test_and_call_subroutine`: Tests a byte at an offset determined by a 16-bit value in a structure; if the byte is non-zero, it branches to another routine, otherwise it pushes a value from a structure onto the stack, calls a subroutine, adjusts the stack pointer, and branches to another routine.
- `increment_and_copy_byte_2`: This routine increments two counters stored in a structure pointed to by a5, loads the byte pointed to by the first counter, stores that byte into the location pointed to by the second counter, and loops if a byte at a specific address is not equal to 3a.
- `increment_and_copy_byte`: The routine increments two counters stored in a structure pointed to by A5, loads the byte pointed to by the first counter, stores that byte through the pointer held by the second counter, clears the byte at the destination, and returns the value of the second counter.
- `initialize_stack_buffer`: Allocates a 65-byte buffer on the stack and calls another routine twice, first with only the buffer as an argument and then with both the buffer and an incoming parameter.
- `initialize_local_variable`: Sets a word at offset -2 of a structure in a5 to 1 and then branches to another location.
- `search_and_open_file`: Iterates through a table at an offset from a4, checking for non-zero longwords, then calls another routine to open a file using a parameter from a5 and stores the result in the table.
- `clear_memory_at_offset`: Extends a 16-bit value to 32 bits, shifts it left by two, and clears a longword at that offset from a base address in a4, then branches to an offset of 80.
- `initialize_stack_frame_and_branch`: Allocates a stack frame and stores the value one in a local variable at a negative offset from the stack pointer, then performs a short branch to another routine.
- `increment_and_load_file_pointer`: Increments a 16-bit index in a structure, loops until a non-zero value is found at the corresponding offset in a table, calls another routine to open a file, and stores the resulting pointer back into the table.
- `clear_memory_at_offset_2`: Extends a 16-bit value to 32 bits, shifts it left by two, and clears a longword at that offset from a base address in a4, then branches to an offset of 184.
- `read_from_table_pointer`: Retrieves a pointer from a table using an index from a5, calls another routine with that pointer and a value from offset 10 of a5, and stores the result in offset 14 of a5.
- `branch_with_register`: Moves a 16-bit value from an offset of minus 2 in a5 into d0 and then performs a short branch to an offset of 166.
- `read_from_pointer_list`: Retrieves a pointer from offset 10 of a5, calls another routine with a pointer from that location and an index from a5, and stores the result in offset 14 of a5.
- `branch_via_offset`: Moves a 16-bit word from an offset of minus 2 from a5 into d0 and branches to another routine.
- `copy_byte_from_pointer`: Retrieves a pointer from offset 8 of a structure in a5, decrements a value at offset 12 of the structure twice, and if the value is non-zero and not 65535 or 10, calls another routine and copies a byte from offset -1 of the structure to the location pointed to by the retrieved pointer.
- `clear_and_compare_structure_field`: Clears a byte at a location pointed to by an offset of negative six from a5, then compares that same location to a long value at offset eight of the same structure; if they are not equal, it returns, otherwise it clears d0 and returns.
- `branch_via_pointer`: Moves a 32-bit long from offset 8 of a5 into d0 and branches to another routine.
- `get_table_entry`: Retrieves a value from a table using an offset and a parameter.
- `forward_call_result`: Calls another routine with a parameter and returns the returned value unchanged.
- `close_file_if_not_zero`: Checks a table entry and calls a DOS routine to close a file if the entry is non-zero, then clears the entry.
- `close_multiple_files`: Iterates through a table of pointers, calling a DOS routine to close each non-zero entry and clearing them.
- `seek_file_via_table`: Calls a DOS routine to seek within a file using a pointer retrieved from a table.
- `return_first_call_result`: The routine calls another routine, stores the returned value in a local variable, calls the same routine again and discards the second result, then returns the first stored value.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `copy_string_to_buffer`: Calls a routine, stores a word in a global variable, and if that word is non-zero, performs several indirect calls and copies a null-terminated string from a global source to a destination buffer provided in a1.
- `iterate_callback_list`: If the value in d1 is non-zero, it enters a loop that retrieves a function pointer from a global table using a value from a4, calls that function, and then continues the loop by updating d1 and d3 until d1 is zero.
- `execute_callback_loop_2`: The routine iterates through a list, calling a function pointer retrieved from a structure at each step and incrementing a pointer.
- `format_numeric_string_2`: Appends a numeric value and a colon to a buffer, then stores the resulting string length in a global variable.
- `write_zero_and_store_index`: Writes a zero byte to a location pointed to by a0 and d0 and stores that same byte as a short in a global variable.
- `format_numeric_string`: Writes a sequence of characters representing a number and a colon into a buffer, then stores the resulting string length in a global variable.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_3`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `calculate_difference_via_calls`: The routine performs several indirect calls and arithmetic operations to calculate a value based on the difference between two results.
- `process_file_info_and_lock`: Calls several routines to retrieve file information, performs division, checks if a value is less than 1000, and conditionally calls other routines to manage file locking and unlocking.
- `branch_via_stack_offset`: Moves a 16-bit word from offset minus 16 of a5 into d0 and branches to another routine.
- `copy_word_to_register`: Copies a 16-bit word from d0 to d5.
- `execute_complex_sequence_2`: Clears a word in a global variable, calls several routines to process data from a structure in a5, performs a comparison between a value from the stack and d0, and if the stack value is less than or equal to d0, clears the stack and calls another routine before updating a global variable and calling a final routine.
- `execute_conditional_sequence_3`: Calls another routine using a value from a4, and if the result in d0 is not zero, calls another routine with a value from the stack, then potentially calls another routine, clears a global word, sets a global word to one, and calls another routine.
- `execute_multiple_subroutines`: Performs a sequence of several subroutine calls using various pointers and values from a5 and a4, modifies several global variables, and ends with a branch to another routine.
- `execute_complex_logic`: Performs a sequence of calls, clears specific fields in a global structure, and uses the result of a routine to conditionally execute further calls and update a value in a structure.
- `execute_conditional_logic_9`: Clears specific structure members, performs conditional routine calls based on a parameter value, and updates global status flags.
- `copy_and_branch`: Copies a word from a global variable to an offset in a stack frame and then branches to another routine.
- `traverse_and_update_list_2`: Iterates through a linked list by fetching pointers, performs conditional updates to a structure, and finally clears a global variable and calls several other routines.
- `traverse_and_update_list`: Iterates through a linked list by fetching pointers, performs conditional updates to a structure, and finally clears a global variable and calls another routine.
- `store_offset_value`: Retrieves a 16-bit value from a memory location determined by a global offset and stores it into another global variable.
- `process_nested_data_2`: Allocates a stack frame, calls two different routines while pushing values from a0 and the stack, stores a result in a global variable at a negative offset of a4, and clears another global variable at a negative offset of a4 if d0 is -1.
- `process_input_and_file`: Validates a string against a table, updates a global offset, and closes a file if a table entry is non-zero.
- `validate_range_and_call`: Checks if the difference between two global variables matches a value returned by a routine, then calls another routine with a pointer from a structure and a value from a global variable, returning 1 if the subsequent call returns 0.
- `return_zero`: Clears register d0 and returns.
- `process_and_return_offset`: Sets a flag and a zero value in a structure, calls two other routines, and returns a value derived from a global variable plus four.
- `transfer_buffer_byte`: Moves two longwords from a structure in a5 to the stack, stores a byte from a pointer at offset 8 of the structure into a register, and branches to another routine.
- `write_constant_to_offset`: Writes the word 1 to the memory location at offset negative 12 of the structure pointed to by a5 and branches to another routine.
- `increment_and_scan_buffer`: Increments a 16-bit counter and a 32-bit pointer within a structure, then enters a loop that compares a byte from the structure against the byte at the current pointer location.
- `dispatch_byte_values_2`: Calls three different routines with values from a structure, then checks a word at a negative offset from a5 to decide whether to call more routines based on a byte at offset negative 9.
- `dispatch_byte_values`: Compares a byte from a structure at offset -9 to 250, and if it is greater than or equal, calls another routine three times with different values (252, the byte at offset -9, and the byte at offset -11) passed on the stack.
- `process_buffer_data`: Pushes two bytes from a structure in a5 onto the stack and calls another routine, then calculates a difference between a pointer and a value at offset -8 of a5, storing the result at offset -14 of a5, and calls another routine based on whether that difference is less than 256.
- `update_global_counters`: Calls several routines by passing a constant, a word from a structure at offset -14, a longword from a structure at offset -4, and another constant on the stack.
- `fetch_and_branch`: Calls a routine, moves the returned byte to offset -1 of a5, and if the byte in d1 is not 255, it loads the byte from offset -1 of a5 into d0 and branches.
- `parse_data_stream_tokens`: Reads a byte from a table and uses it as a command to drive a loop that either writes a sequence of bytes to a memory location pointed to by a table entry or calls another routine to update a pointer and a counter.
- `update_structure_byte`: Calls a routine, stores a byte from the result into a 16-bit value at offset negative 4 of a structure in a5, and if the result is non-zero, increments a pointer at offset 8 of a5 and stores a byte from that pointer into a location pointed to by a0 before branching back.
- `dispatch_bytecode_instruction_2`: Reads a byte from a data stream and executes a switch-based logic that either populates a buffer via a pointer and counter, or calls another routine with values extracted from the stream.
- `calculate_jump_offset`: Subtracts 251 from d0, and if the result is less than 4, shifts d0 left by 1, fetches a word from a table using d0 as an index, and jumps to an address calculated from that word.
- `branch_to_offset_1642`: Performs a short branch to a location at offset 1642 of the current code segment.
- `unlock_semaphore_and_return_2`: Unlocks the semaphore associated with the address held in register a5 and returns immediately.
- `read_byte_from_table`: Retrieves a single byte from a table using an index and returns it.
- `read_word_from_table`: Retrieves a two-byte word from a table using an index and returns it.
- `increment_global_variable`: Tests a global variable and, if it is non-zero, increments a different global variable by 1 before branching.
- `execute_table_function`: Retrieves a value from a table using an offset of 9 from a structure in a5 and a value from a global variable, then performs an indirect call to a function pointer if the retrieved value does not match 1.
- `increment_global_variable_by_two`: Tests a global variable and, if it is non-zero, increments a different global variable by 2 before branching.
- `execute_table_function_alt`: Retrieves a value from a table using an offset of 8 from a structure in a5 and a value from a global variable, then performs an indirect call to a function pointer if the retrieved value does not match 2.
- `branch_on_table_value`: Checks a word at a negative offset from a4, and if it is not zero, extends a word from offset 12 of a5 to a longword, adds it to a value at a negative offset from a4, and branches to another routine.
- `execute_conditional_table_call`: Retrieves a value from a table using an offset of 8 from a structure in a5 and a value from a global variable, then performs an indirect call if the retrieved value does not match a word stored at offset 12 of the structure in a5.
- `increment_and_test_byte_4`: Calls a routine with a pointer from offset 8 of a5, stores the result at offset negative 2 of a5, and if a global variable is non-zero, adds the sign-extended result to a global variable.
- `process_indirect_table_data`: Retrieves values from a table using offsets and performs an indirect call based on the results.
- `accumulate_data_segments`: Fetches three 16-bit values from offsets 2, 4, and 6 of a structure pointed to by a5, processes them through a series of calls, and stores the final result in a global variable at a negative offset of a4.
- `execute_sequence_and_set_flags`: Allocates a stack frame, calls two routines using values from a5, stores a result in a global variable at a negative offset of a4, and clears 16-bit words at offsets -72 and -68 of a5 based on a comparison.
- `process_data_loop`: Iterates through a loop, copying data from one location to another, updating several counters and pointers, and returning a word value.
- `execute_sequence_of_calls_5`: Calls three different routines using various values from a structure in a5, stores the result of the third call in a global variable at a negative offset of a4, and if the result is not negative 65535, it calls another routine and then returns 0.
- `execute_sequence_of_calls_3`: Pushes a word from offset 16 of the structure in a5 onto the stack, calls another routine, increments the stack pointer by 2, pushes a longword from offset 12 of a5 onto the stack, calls another routine, increments the stack pointer by 4, clears a word at offset negative 68 of the structure in a5, and branches to another routine.
- `iterate_and_call_dos`: Enters a loop that compares a value from a table to a word at offset 16 of a5, calls a routine with a pointer from a table, calls another routine with a calculated address, increments a word at offset -68 of a5, and continues until it reaches a value at offset -70, then calls a routine with 65535 and another routine with a pointer from a global variable.
- `increment_pointer_loop`: Pushes a longword from offset -24672 of a4 onto the stack, calls another routine, stores the result in a longword at offset -4 of a5, and enters a loop that increments a pointer and checks for a null terminator.
- `call_format_via_pointer`: Decrements a pointer at offset -8 of a5, and if the byte at that pointer is 58, pushes a word from offset 10 of a5, a pointer from offset 12 of a5, and a pointer from offset -4 of a5 onto the stack, then calls another routine.
- `format_string_via_pointer`: Pushes a word from offset 16 of a5, a pointer from offset 12 of a5, and a pointer from offset negative 4 of a5 onto the stack, pushes a pointer to a global variable, pushes a pointer from offset 8 of a5 onto the stack, calls another routine, cleans the stack, unlinks a5, and returns.
- `increment_pointer_and_check_byte`: Calls another routine, stores the result in a local variable, and then increments a pointer stored at offset eight of the local stack frame while checking if the byte at that pointer is zero.
- `call_format_with_args`: Decrements a pointer at offset -8 of a5, and if the byte at that pointer is 58, pushes a pointer from offset 12 of a5, a pointer from offset -4 of a5, and a pointer from offset 8 of a5 onto the stack, then calls another routine.
- `format_string_with_arguments`: Pushes a pointer from offset 12 of a5, a pointer from offset negative 4 of a5, and a pointer to a global variable onto the stack, pushes a pointer from offset 8 of a5 onto the stack, calls another routine, cleans the stack, unlinks a5, and returns.
- `update_table_entry`: If a word at offset 8 of a structure in a5 is 4, it iterates through a table using a counter in a stack location, compares a word from the table to a value at offset 2 of the structure, and if they match, updates offset 2 of the structure with a value from another table and increments the counter; otherwise it returns.
- `return_pointer_from_structure`: Loads a 32-bit word from an offset of 8 in a structure pointed to by A5 and returns it.
- `read_byte_from_offset`: The routine returns a 16-bit signed value obtained by reading a single byte from a specific offset relative to a register.
- `update_status_flags`: The routine clears a value at a specific offset and then conditionally updates that value and a secondary byte based on bits set in the input parameter.
- `fetch_indexed_word`: Compares a word at offset 8 of a structure in a5 with 96, and if it is less, checks bit 3 of offset 11 of the structure to decide whether to retrieve a word from a table using an index derived from offset 8, or to retrieve a word from a different table, or to subtract 64 from a value retrieved from a table.
- `calculate_indexed_value_3`: Masks a word from offset 10 of a structure in a5 with 3, and if the result is non-zero, retrieves a byte from a table using a value from offset 8 of the structure as an index and shifts the result left by 8 bits.
- `calculate_indexed_value_2`: Masks a word from offset 10 of a structure in a5 with 3, and if the result is not zero, it calculates an index by shifting a word from offset 8 of the structure left by one, fetches a word from a table at a negative offset from a4 using this index, and branches to another routine.
- `calculate_indexed_value`: The routine calculates an index based on a value from a structure, retrieves a word from a table using that index, and conditionally subtracts a constant from the word if certain bit flags are set.
- `update_status_flags_3`: Reads a value from the custom register at offset zero of the hardware address in a0, performs bitwise operations to determine status bits, and conditionally calls two routines to update a structure and a global pointer.
- `update_status_and_pointers`: If a word at a negative offset of a4 is non-zero and a calculation involving another negative offset of a4 is greater than a third negative offset of a4, it calls another routine, updates a pointer in a4, and then performs a series of operations involving a byte from a specific address to update a byte at a negative offset of a5 and potentially calls another routine.
- `check_word_and_return`: Compares a word at offset 8 of a structure in a5 to 65535, and if it is not equal, loads a word from a negative offset of a4 into d0 before returning.
- `store_word_to_offset_2`: Stores a word from offset 8 of a structure in a5 into a memory location at a negative offset from a4 and then branches to another routine.
- `process_object_interaction_event`: Retrieves an object pointer from a global structure, performs a bit-shifted lookup based on a byte at offset 21 of the object's data, and calls another routine to fetch the object.
- `branch_to_offset_2`: Performs a short branch to an offset of 42.
- `fetch_opcode_and_extend`: Loads a 16-bit word from an offset of 6 bytes before the address in a5 into d0, extends it to a 32-bit signed long, and branches to another routine.
- `initialize_and_update_object_state`: Sets two 16-bit values to one in a global structure and another structure, then calls a routine to retrieve a value into a register.
- `initialize_and_process_object_structure`: Clears a word in a global variable, sets a value of two in a stack-based structure, and calls another routine to populate that structure with data from a global pointer.
- `initialize_object_state_structure`: Sets two 16-bit values to one in a structure, calls another routine, and then populates a structure with a bitwise ORed value, a pointer, and two 16-bit words from global variables.
- `initialize_and_load_object_data`: Clears a word in a global structure, sets a value of two in another structure, and calls another routine to populate several fields in a local structure from a global pointer.
- `compare_and_branch_d0`: Subtracts various constants from d0 and branches to different locations based on whether d0 reaches zero.
- `process_object_data_structure_2`: Checks a bit in a structure and validates a type field, then calls another routine to fetch a value.
- `initialize_and_populate_context_structure`: Clears a word in a global variable, sets a 16-bit value to 4096 in a stack-based structure, and calls another routine to populate the structure with values from a global pointer and other memory locations.
- `decrement_and_branch_on_d0`: Subtracts various values from d0 and branches to different locations if the result is zero.
- `branch_to_offset_3`: Performs a short branch to an offset of 28.
- `return_from_frame`: Two-instruction routine: unlk a5 tears down a stack frame (restoring the saved frame pointer), then rts returns to the caller.
- `process_task_data`: Copies a string to a buffer, stores a value into a structure, and performs an indirect call using data from an input event.
- `create_object_entry`: Obtains two values from another routine, stores one and a sum into fields of a structure, copies a field within that structure, writes a byte, a byte, and a word from parameters into the structure, and calls another routine with the structure pointer and a parameter.
- `propagate_object_state`: Calls another routine repeatedly with values taken from fields of an argument structure and with constant arguments, or with the original argument unchanged.
- `validate_data_structure`: Loads a pointer from the stack, compares the word at that pointer to zero, four, and then compares a byte at offset 2 of that pointer to 45, and calls another routine if all conditions are met.
- `recursive_call`: The routine performs a recursive call to itself with no other operations.
- `set_two_words`: Stores the value one into two consecutive 16-bit words at offsets minus 6ac2 and minus 6ac0 from a pointer in a4.
- `fetch_two_words`: The routine calls two subroutines via a global pointer and copies two words from a memory location into a provided pointer.
- `fetch_coordinate_pair`: Retrieves two consecutive words from an offset within a structure pointed to by a value at a specific memory location.
- `call_two_subroutines`: The routine calls two subroutines using pointers stored within a global variable.
- `empty_c_function_4`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_5`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_6`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_7`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_8`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_9`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_10`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_11`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_12`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_13`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_14`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `handle_error_message`: Parses a format string and executes a callback routine to display an error message containing a version string.
- `copy_sprite_with_mask`: Pushes several values onto the stack, including two words of 65535 and a zero, pushes two long values from a structure in a5, calls another routine, and returns.
- `manage_structure_state`: Calls a routine and stores the result at offset -16 of a5, calls a second routine, calls a third routine with offset 12 of a5, calls a fourth routine with offset 16 of a5 and a constant, calls a fifth routine with offset 8 of a5 and offset -12 of a5, and if offset 20 of a5 is not -1, enters a loop that calls another routine until it returns a non-zero value.
- `check_structure_value`: Moves a word from offset 14 of a5 to d0, compares the word at offset 12 of a5 to -1, and if they are not equal, moves a word from offset -10 of a5 into d0.
- `process_structure_parameters`: Calls a routine with offset -12 of a5 and stores the result at offset -4 of a5, calls another routine using a pointer from a global variable, calls a third routine with offset -12 of a5, calls a fourth routine with offset 8 of a5 and a value from offset -12 of a5, and if offset 12 of a5 is not null, sets d0 to 4 and branches to another location.
- `execute_structure_sequence`: Calls three consecutive routines using parameters derived from a structure, stores the first result in the structure, and then conditionally enters a loop calling another routine until it returns non-zero.
- `loop_on_result`: Pushes a value from a global structure onto the stack, pushes a word, calls a routine, and if the return value in d0 is non-zero, branches to another location, otherwise it branches to itself.
- `call_and_clear_d0`: Pushes a longword from a structure in a5 onto the stack, calls a routine, cleans the stack, and clears register d0 before branching.
- `compare_and_fetch_bytes`: Fetches a pointer from a structure, compares a value derived from two bytes at that pointer with a word at offset 10 of a5, and returns zero if the comparison fails.
- `increment_and_copy_bytes`: Increments a 16-bit value at offset 10 of a structure in a5 by one and multiplies it by two, then uses the result to calculate an offset for a byte-copy loop that updates a pointer and a counter.
- `copy_bytes_loop`: Copies a byte from a source pointer to a destination pointer, increments both pointers, and loops until a 16-bit counter reaches zero, otherwise it returns.
- `clear_byte_at_offset_5`: Pushes three words from a5 onto the stack and calls another routine, then uses the returned word as an offset to clear a byte at the start of a pointer located at offset 12 of a5.
- `unlock_and_return`: Clears the value in register d0 and unlocks the semaphore associated with register a5 before returning.
- `update_global_offset_5`: Copies a null-terminated string from a structure in a5, performs arithmetic operations using global offsets via two different calls, and stores the resulting 32-bit value into a global variable.
- `copy_string_data`: Finds the end of a string by searching for a null terminator, then copies the string from a source pointer to a destination pointer.
- `branch_to_offset_4`: Performs a short branch to an offset of 424.
- `copy_bytes_loop_2`: Copies a byte from a source pointer to a destination pointer and increments both pointers in a loop if the byte at the source pointer is not zero.
- `clear_and_return`: Writes a zero byte to a location eight bytes before a pointer in a5 and returns the 32-bit value stored twelve bytes after that pointer.
- `calculate_pointer_offset`: Calculates a value based on a pointer and a global base address, returning a 32-bit result.
- `return_pointer_value`: The routine receives a pointer to a 16-bit value and returns the contents of that memory location unchanged.
- `get_word_at_offset`: The routine reads a 16-bit word from the address formed by adding two to the input pointer and returns it.
- `get_concatenated_value`: Retrieves a 16-bit value from a memory location and combines it with a high-word component derived from a pointer.
- `get_offset_word`: Retrieves a 16-bit word from an offset of a pointer and combines it with a high-word component derived from that pointer.
- `update_linked_list_nodes_2`: Updates a 16-bit value at an offset of two from a pointer, then swaps a value from a source pointer into a target pointer and clears a word.
- `update_linked_list_nodes`: Updates a 16-bit value at an offset of two from a pointer, then swaps a value from a source pointer into a target pointer and clears a word at an offset of two from a third pointer.
- `compare_and_call_routines`: Compares a word from a structure at offset 12 of the parameter block with a word at the beginning of the structure pointed to by offset 8 of the parameter block; if they differ, it calls another routine twice, once with the original pointer and once with the value from offset 12, then returns that value.
- `update_pointer_and_return_status_2_2`: Checks if a 16-bit value in a structure is non-zero, then if it matches a value in another structure, updates that structure with a value from a pointer-based table; otherwise, it updates a different location.
- `update_pointer_and_return_status_2`: Writes a 16-bit value from d3 to an offset of two from a pointer derived from a global base address, then returns a 32-bit value based on whether a 16-bit value at offset 12 of a structure matches a value at offset 2 of another structure.
- `update_pointer_and_return_status`: Writes a 16-bit value from d3 to an offset of two from a pointer derived from a global base address, then returns a 32-bit value based on whether a word at a pointer is zero.
- `initialize_memory_structure`: Stores a word into a memory location, optionally calls another routine, and then populates a series of bytes at a specific offset with a pointer and constant values.
- `dispatch_via_table`: Increments a pointer in a global table, reads a byte from that location, and performs an indirect jump to an address found in another table using the byte as an index.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `dispatch_via_table_and_index`: Increments a pointer in a global table, reads a byte from that location, and performs an indirect jump to an address found in a table at an offset determined by the byte multiplied by four.
- `check_byte_and_branch`: Loads a pointer from a global variable, increments that pointer, reads a byte from the new location, and branches based on whether that byte is less than 240.
- `move_byte_with_increment`: Masks a byte, increments a pointer in a global variable, and moves a byte from the address at that pointer to a destination specified by a register.
- `copy_bytes_in_loop_2`: Increments a global pointer, reads a byte, and enters a loop that copies bytes from the pointer to a destination at a negative offset from a4 until a counter reaches zero.
- `update_pointer_and_dispatch`: Adds 40 to a register, stores the result in a global variable, and calls a routine via a table dispatch.
- `increment_and_dispatch_pointer`: Increments a global pointer, reads a byte from the new location, extends it to a word, adds 40 to the pointer, and calls a routine via a table dispatch.
- `check_byte_and_branch_2`: Increments a global pointer, reads a byte from the new location, and branches if the byte is less than 240.
- `dispatch_via_branch`: Performs a branch to another routine.
- `fetch_bytecode_instruction`: Increments a pointer in a global variable, reads a byte from the new location, and compares it to 240.
- `dispatch_bytecode_instruction`: Increments a pointer in a global table, reads a byte from the location pointed to by that pointer, and performs an indirect jump to an address in a table based on the lower nibble of that byte if the byte is not 255.
- `increment_and_fetch_command_byte`: Increments a pointer in a global variable, reads a byte from the location pointed to by the updated pointer, and compares it to a constant value; if the byte is less than that value, it branches to another routine, otherwise it performs an unconditional branch.
- `fetch_table_byte_and_dispatch`: Stores a byte in a global variable, masks it, fetches a byte from a table using that value as an index, stores it in a global variable, and calls a routine via a table dispatch.
- `increment_and_fetch_command_byte_2`: Increments a longword in a global variable, reads a byte from the resulting address into a register, and branches to another routine if the byte is less than 240; otherwise, it performs an unconditional branch.
- `store_byte_and_dispatch`: Stores a byte from a register into a global variable and calls a routine via a table dispatch.
- `increment_and_fetch_dispatch_byte`: Increments a pointer in a global variable, reads a byte from the new location, and if that byte is less than 240, performs an indirect jump via a table; otherwise it returns.
- `update_dispatch_state`: Stores a byte from a register into a structure at a negative offset from a pointer in a4, increments a pointer in a global table, and performs an indirect jump to a table address based on the lower nibble of the byte read from that table if the byte is not 255.
- `store_constant_and_dispatch`: Stores the constant value 128 into a global variable and calls a routine via a table dispatch.
- `jump_to_handler`: Performs a short branch to another routine.
- `set_byte_and_jump`: Stores the value 128 at an offset of 26860 from a6, then performs a short branch to another routine.
- `set_byte_and_jump_alt`: Stores the value 128 at an offset of 26859 from a6, then performs a short branch to another routine.
- `increment_and_fetch_table_byte`: Increments a pointer in a global variable, reads a byte from the resulting address into a register, and branches to another routine if the byte is less than 240; otherwise, it performs an unconditional branch to another routine.
- `store_byte_and_jump`: Stores the byte from d1 at an offset of 26862 from a6, then performs a short branch to another routine.
- `store_results_and_call`: Calls a routine, stores the returned word and a second word into global variables at negative offsets from a4, calls two more routines, and branches to another location.
- `store_and_execute_subroutines`: Stores two 16-bit values from registers into a structure at offsets -27968 and -27966 relative to a6, then calls three subroutines in sequence before branching to another routine.
- `store_accumulator_values`: Calls a routine twice to update d0 and d1, stores these values into a structure at offsets negative 30368 and negative 30366 of a4, and then calls another routine.
- `call_two_routines`: Calls two different routines and then branches to another location.
- `dispatch_table_command`: Clears d3, checks bit 5 of a byte at offset negative 30366 of a4, and if set, reads a byte from a table pointed to by a0 and increments that pointer, then branches if the byte is not greater than or equal to 240.
- `execute_table_command`: Copies d1 to d3, calls two subroutines, stores d0 and d1 into a structure at a negative offset from a4, and if specific bits are set in a byte at a negative offset from a4, increments a pointer at a negative offset from a4 and performs an indirect jump based on the byte read from that pointer.
- `copy_byte_and_call`: Copies a byte from register d1 to d3, calls two other routines, and then branches to another location.
- `fetch_and_validate_script_byte`: Tests bit 5 of a byte at a specific offset in a structure, and if set, extracts a 4-bit value from that same byte.
- `process_script_byte_and_dispatch`: Stores two words from registers into a structure at offsets 28352 and 28354 relative to a pointer in a4, then calls two subroutines.
- `move_byte_and_call_subroutines`: Moves a byte from a register into a register and calls two subroutines before branching to another routine.
- `dispatch_byte_command`: Clears d3, checks bit 5 of a byte at a negative offset from a4, and if set, performs bitwise operations on a word from that same byte, increments a pointer at a negative offset from a4, and performs an indirect jump based on a byte read from the new pointer location if it is not less than 240.
- `transfer_byte_and_branch`: Copies a byte from register d1 to d3, calls a routine, reads a byte from an offset of a4, calls another routine, and then branches to another location.
- `process_buffer_byte`: Pushes an offset from a4 onto the stack, calls a routine, adjusts the stack, pushes a word onto the stack, calls another routine, adjusts the stack, loads a pointer from an offset of a4 into a0, increments that pointer, reads a byte from the new location into d1, and branches based on whether that byte is less than 240.
- `dispatch_indexed_function_pointer`: Iterates through a buffer by incrementing a pointer and calling functions stored at specific offsets from a base address until a byte reaches a threshold.
- `copy_memory_block`: Copies 39 bytes from a source address to a destination address using a loop.
- `call_indirect_function`: Calls a function through a pointer stored in a table using specific arguments.
- `process_pointer_and_byte`: Loads a pointer from an offset of 26858 from a6, increments that pointer by 1, reads a byte from the original location into d1, compares it to 240, and if it is less than 240, increments a7 by 4 before branching to another routine.
- `convert_nibble_sign`: Reads the high nibble of a byte from a register, applies a sign extension when the high bit of the nibble is set, and returns the signed result as a short.
- `fetch_and_validate_byte`: Increments a pointer in a global variable, reads the byte at that location into a register, and if the byte is less than 240, increments the stack pointer by 4 and branches to another routine; otherwise, it returns.
- `get_and_increment_byte`: Returns a byte from a memory location pointed to by a global variable and increments that pointer by one.
- `fetch_and_skip_byte`: Increments a pointer in a global variable, reads a byte from the new location, and if the byte is less than 240, increments the stack pointer by 4 and branches to another routine; otherwise, it returns.
- `get_and_increment_word`: Returns a word from a memory location pointed to by a global variable and increments that pointer by two.
- `update_nibble_via_table`: Saves d2 and d3 to the stack, reads a byte from an offset of 28359 from a6, checks bit 0 of a byte at offset 28225 from a6, rotates the byte if the bit is 0, stores the result at offset 26862 from a6, calculates a target address using several values from a6 and a table, and updates the byte at that address using bitwise operations with values determined by bit 0 of a word at offset 28224 from a6.
- `update_byte_low_nibble`: Replaces the low nibble of a byte at an offset in a table with a value from a global variable.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_nibble_via_table_alt`: Saves d2 and d3 to the stack, reads a byte from an offset of 28359 from a6, checks bit 0 of a byte at offset 28225 from a6, rotates the byte if the bit is 0, stores the result at offset 26865 from a6, calculates a target address using several values from a6 and a table, and updates the byte at that address using bitwise operations with values determined by bit 0 of a word at offset 28224 from a6.
- `update_byte_low_nibble_alt`: Replaces the low nibble of a byte at a specific offset in a table with a value from a different global variable.
- `update_byte_low_nibble_2`: Rotates a byte from a global variable, stores it back, and then updates a single byte in a table using a calculated pointer and nibble masks.
- `update_byte_low_nibble_alt2`: Replaces the low nibble of a byte at a specific offset in a table with a value from a third global variable.
- `update_two_byte_low_nibbles_2`: Rotates a byte from a global variable, stores it back, and then updates two bytes in a table using calculated pointers and nibble masks.
- `update_two_byte_low_nibbles`: Updates the low nibble of a byte at a specific offset and then updates another byte at a calculated offset within a table.
- `update_nibble_pair_buffer`: Rotates the nibbles of a byte from a structure and stores it in a local variable, then uses that byte and a calculated mask to update two specific bytes in a memory buffer via a pointer derived from a table and an offset.
- `update_nibble_pair`: Updates two bytes in a memory buffer by replacing their lower nibbles with values from a global source.
- `rotate_and_update_nibble_pair`: Rotates the nibbles of a byte based on a flag and then updates two bytes at a calculated memory location by masking and merging new nibbles from a global source.
- `update_nibble_pair_offset`: Modifies two bytes at a specific memory location by masking and merging new lower nibbles from a global source.
- `update_three_nibbles_2`: Rotates a byte from a global variable, stores it back, and then updates three bytes in a table using calculated pointers and nibble masks.
- `update_three_nibbles`: Performs a nibble-based update on three distinct byte locations within a memory buffer.
- `add_global_word_values`: Adds two 16-bit values from a global table to the contents of two other global 16-bit variables.
- `update_accumulators_and_dispatch_callback`: Calculates four new 16-bit accumulator values by adding offsets from a structure to existing values, updates a bitmask based on three signed byte checks, and calls a function pointer from a table indexed by that bitmask.
- `execute_callback_loop`: Compares two 16-bit values, swaps them if the first is smaller, and then enters a loop that calls a function pointer while incrementing a value.
- `calculate_accumulator_differences`: Sets a 16-bit value at an offset of a4 to 1, calculates the absolute difference between two pairs of 16-bit values at offsets of a4, and if a specific difference is less than another, performs an arithmetic shift right on a value and branches.
- `process_accumulator_loop`: Subtracts a value from a 16-bit variable, performs several arithmetic operations on other 16-bit variables, calls a function pointer, and then enters a loop using a decrementing register.
- `shift_accumulator_value`: Copies a 16-bit value from a global variable to d2 and d3, then performs an arithmetic shift right on d3 before returning.
- `update_accumulator_and_execute_callback`: Increments and wraps a value based on a threshold while updating two separate accumulation variables.
- `accumulate_and_load_byte`: Adds two 16-bit global variables to the contents of two other global 16-bit variables and then loads a byte from a global variable into register d2.
- `update_sprite_position_and_render`: Calculates new coordinate offsets for a sprite based on bounding box constraints and updates its position in memory.
- `update_interval_and_sync_data`: Updates interval values in a structure from a source table, then checks three specific byte-based conditions to select and copy data from different memory locations.
- `branch_on_carry_and_compare`: Calls another routine, and if the carry flag is set, compares a longword at a negative offset of a4 with a3 and branches if a3 is less than or equal to that value.
- `store_d0_to_global`: Stores the contents of d0 into a 16-bit global variable and branches to another location.
- `update_interval_structure`: Copies a series of 16-bit values from a source structure into a destination structure at a negative offset from a4, then branches to another routine if a control value is 1 or branches to a different location if the control value is 65535; otherwise, it returns.
- `update_interval_range`: Copies a 16-bit value from a structure into a temporary location, increments a counter at a specific offset, and performs bounds checking against a range defined in a structure to determine whether to update a value or branch to another routine.
- `compare_and_branch_2`: Calls a routine, branches if the carry flag is set, compares a long value at an offset of a4 with a3, branches if a3 is less than or equal to that value, otherwise sets d1 to 1 and branches.
- `store_temporary_word`: Stores a 16-bit value from d0 into a structure at offset negative 28176 of a4 and then branches to another location.
- `check_range_and_update`: Checks if a 16-bit value is within several specified ranges, and if so, calculates a table pointer to update a byte using bitwise operations and a mask.
- `set_ccr_status`: Sets the condition code register to 1 and returns immediately.
- `update_nibble_match_and_increment`: Compares a nibble from a data buffer against a target nibble and, if they match, updates up to three separate memory locations by merging new nibble values into their high or low nibbles based on a bitmask.
- `increment_nibble_data`: Checks a nibble match and updates multiple memory locations by masking and merging nibbles based on a bitmask, then increments a global counter.
- `update_nibble_masked_data`: Checks a byte in a data structure against a target byte, and if the high nibbles match, it updates up to three different memory locations by merging their low nibbles with new values from a table based on a bitmask.
- `decrement_nibble_data`: Checks a nibble match and updates multiple memory locations by masking and merging nibbles based on a bitmask, then decrements a global counter.
- `fill_memory_blocks`: Calculates offsets and fills three potential memory ranges with specific byte values based on a bitmask and calculated lengths.
- `dispatch_by_byte_sequence`: Checks four consecutive bytes starting at offset 8 of the stack frame; if they are all zero, it loads an address from an offset of -27448 from a4 and branches.
- `branch_to_offset_38`: Loads an address into a0 and branches to an offset of 56.
- `load_offset_and_branch`: Loads an address from a location at a negative offset from a4 into a0 and then performs a short branch to another routine.
- `count_leading_zero_bits`: Counts leading zero bits in a byte from a register, uses the count to index a table of divisors, and performs two successive divisions of a 32-bit value to return a combined remainder and adjusted divisor result.
- `conditional_branch_logic`: Saves registers, loads a word from an offset of a4, and if the word is non-zero, branches to an offset of 140; otherwise, it calls another routine and branches to an offset of 130.
- `modify_and_store_word`: Performs a bitwise operation on a 16-bit value based on its lowest bit and stores the result into a memory location.
- `return_absolute_value`: Returns the absolute value of a signed 16-bit argument by negating negative values and returning others unchanged.
- `conditional_branch_logic_alt`: Saves registers, loads a word from an offset of a4, and if the word is non-zero, branches to an offset of 192; otherwise, it calls another routine and branches to an offset of 182.
- `multiply_and_store_word`: Multiplies a 16-bit value by a constant and stores the result as a 16-bit word in a memory location.
- `add_constant_and_call`: Adds 360 to a word from offset 8 of a structure in a5, pushes the result onto the stack, and calls another routine; otherwise it returns.
- `calculate_table_difference`: If a 16-bit word at offset 8 of a structure in a5 is less than 91, it performs a series of table lookups and arithmetic operations involving division by 5 and subtractions, calls two other routines, and loops back to the start of this routine.
- `call_subtraction_routine`: Compares a 16-bit word at offset 8 of a structure in a5 to 180; if it is not greater, it calculates the difference between 180 and that word, pushes the result onto the stack, and calls another routine.
- `calculate_and_call_subtraction`: Compares a 16-bit word at offset 8 of a structure in a5 to 270; if it is not greater, it subtracts 180 from that word, pushes the result onto the stack, calls another routine, increments the stack pointer by 2, negates the result in d0, and branches to another location.
- `subtract_and_negate_result`: If the word at offset 8 of the structure in a5 is greater than 360, it calculates the difference between 360 and that word, pushes the result onto the stack, calls another routine, and then negates the result in d0.
- `push_subtracted_word_and_call`: Subtracts 392 from a word at offset 8 of the structure in a5, pushes the result onto the stack, calls another routine, adjusts the stack pointer, and branches to another routine.
- `add_offset_and_call`: Tests a word at offset 8 of a5; if it is non-negative, it branches, otherwise it adds 360 to that word, pushes the result onto the stack, and calls another routine.
- `calculate_absolute_difference`: Compares a 16-bit word at offset 8 of a structure in a5 to 90; if it is not greater, it subtracts that word from 90, pushes the result onto the stack, and calls another routine.
- `call_absolute_difference_routine`: Compares a 16-bit word at offset 8 of a structure in a5 to 180; if it is not greater, it calculates the difference between 180 and that word, pushes the result onto the stack, calls another routine, and then negates the result in d0 before branching.
- `clamp_and_call_angle_difference`: Compares a 16-bit value at offset 8 of the structure in a5 to 270; if it is greater, it calculates the difference between the value and 180, pushes this result onto the stack, and calls another routine before negating the result and branching.
- `subtract_and_branch_with_result`: If the word at offset 8 of the structure in a5 is greater than 360, it calculates the difference between 360 and that word, pushes the result onto the stack, calls another routine, and then branches to another location.
- `subtract_and_call_via_stack`: Subtracts 360 from a 16-bit value loaded from an offset of 8 in a structure pointed to by a5, pushes the result onto the stack, calls another routine, adjusts the stack pointer, and branches to another location.
- `initialize_structure_fields_4`: Stores a value returned by another routine into a global variable, calls a second routine using that value and a constant, and then sets two 16-bit fields at offsets 8 and 10 of the structure to 1.
- `allocate_and_initialize_token_structure`: Checks if a global pointer is null, otherwise increments a counter in a table and allocates a new structure.
- `parse_string_tokens_2`: Iterates through a string, incrementing a pointer and a counter when a colon is encountered, then allocates a structure to store parsed elements.
- `set_flag_and_clear_byte`: Stores a word of 1 into a location at offset -30 of a structure, clears a byte at the address stored at offset -28 of the structure, and branches to an offset of 958.
- `clear_structure_fields_2`: Clears a word at offset 14 of a pointer stored at offset -8 of a structure, clears a byte at the address stored at offset -28 of the structure, and branches to an offset of 958.
- `increment_and_copy_string_byte`: Increments a pointer and a 16-bit index stored in a stack frame, then copies a single byte from the pointed-to location into a buffer at an offset determined by the index.
- `update_offset_and_branch`: Decrements a value at an offset from a5, calls another routine, and then writes the result to an offset of a pointer retrieved from the stack before branching.
- `increment_pointer_and_clear_target`: Increments a pointer stored at a negative offset from a5, clears the byte at the address pointed to by that pointer, then increments the pointer again to read a byte and stores it as a word at a negative offset from a5 before branching.
- `convert_lowercase_byte`: Checks if a byte at a negative offset from a5 is between 97 and 122 inclusive, and if so, subtracts 32 from it and stores the result in d0.
- `update_sprite_attribute_from_table`: Reads a byte from a pointer in a local structure, subtracts 65 from it, and uses the result to index into a table to fetch a 16-bit value.
- `convert_lowercase_to_uppercase`: Checks if a byte at an offset in a structure is between lowercase 'a' and 'z', and if so, subtracts 32 from it and stores the result in a register before branching to another routine; otherwise it returns.
- `update_string_index_and_pointer`: Reads a byte from a structure, extends it to a 16-bit word, and if the value is 48, increments it by 10; it then calculates a new value by adding 10 and shifting it left by 8 bits, finally storing this result at offset 16 of a structure pointed to by a value at offset 8 of the current structure.
- `convert_lowercase_char_to_uppercase`: Checks if a byte at a pointer stored at offset negative 28 from a5 is between 97 and 122 inclusive; if so, it subtracts 32 from that byte and stores the result in d0, otherwise it branches to another routine.
- `update_offset_from_byte`: Reads a byte from a pointer stored at a negative offset from a5, sign-extends it to a word, subtracts 64, and stores the result into a word at offset 16 of a structure pointed to by a value at a negative offset from a5.
- `convert_lowercase_to_uppercase_2`: Decrements a pointer stored at offset 28 from the structure in a5, checks if the byte at that pointer is a lowercase letter, and if so, subtracts 32 from it before storing the result in a word at offset 16 of another structure.
- `update_character_attribute`: Fetches a byte from a pointer stored at offset negative twenty-eight of a structure, sign-extends it to a word, and stores the resulting word into offset sixteen of another structure pointed to by offset negative eight of the same structure.
- `subtract_constants_from_register`: Subtracts three different constant values from a register and branches if any of the results are zero.
- `branch_to_offset_6`: Performs a short branch to a specific instruction offset.
- `subtract_and_branch_on_d0`: Subtracts various values from d0 and branches to different locations if the result is zero, otherwise it increments a pointer at a negative offset from a5 and branches to another location.
- `jump_to_increment_logic`: Performs a short branch to the middle of the increment_and_check_value routine.
- `increment_and_check_value`: Increments a 16-bit word at a negative offset from a5, compares it to 10, and if it is less than or equal to 10, unlocks a5 and returns; otherwise it continues.
- `process_global_data`: Loads and stores global values, performs pixel blending, and copies two words between memory locations.
- `initialize_and_process_structure`: Sets a word at offset negative 12 of a structure in a5, calls two different routines, and if a word at offset 8 of a5 is non-zero, it clears a word at offset 8 of a structure in a0, calls another routine, and calls a third routine; otherwise, it branches to another location.
- `iterate_and_process_structure`: Calculates an address using a value from a5 and a global pointer, then enters a loop that calls four different routines using data from a structure, updates a counter, and repeats until a value in a structure is no longer less than a value in a global structure.
- `update_global_state`: Stores a constant value into a memory location pointed to by a global variable and executes a conditional function call.
- `calculate_table_index`: Calculates an offset using a word from a table and a global pointer, then stores a long value from a calculated address into a stack location and sets a word to two in a local variable.
- `fetch_table_entry`: Loads a 16-bit word from a table using an index derived from a 16-bit value at offset -6 of a structure in a5, where the table pointer is stored at offset 8 of that structure, and then branches to another routine.
- `jump_via_table_2`: Subtracts 109 from d0, compares the result to 5, and if it is not below 5, uses the result to fetch a 16-bit word from a table and jumps to the address at a specified offset from the program counter.
- `compare_table_entry`: Increments a 16-bit value at offset minus 6 of a structure in a5, loads a pointer from offset 8 of the structure into a0, compares the 16-bit value at offset minus 6 with the 16-bit value at the address in a0, and otherwise returns.
- `update_pointer_via_table`: Calculates an address by combining a value from a table at a negative offset from a4 with an index derived from a 16-bit word at offset 2 of a structure in a5, then stores a 32-bit word from offset 26 of the resulting address into offset -4 of the structure in a5, and finally branches to another routine.
- `dispatch_via_jump_table`: Subtracts 110 from a longword in register d0, compares the result to 5, and if the result is 5 or greater, shifts the value left by 1 and performs a jump to an address retrieved from a table using the modified value as an index; otherwise, it branches to another location.
- `release_a5`: Unlocks register a5 and returns.
- `check_global_flag`: Tests a longword at a fixed offset from a4; if it is non-zero, it branches to another routine, otherwise it clears d0 and returns.
- `fetch_scaled_index`: Fetches a 16-bit word from a global variable, sign-extends it to a 32-bit value, and shifts it left by one.
- `update_structure_indices`: Fetches two 16-bit values from a table using indices stored in a structure, scales those indices by two, and writes the resulting values back into different offsets of the same structure before branching to another routine.
- `update_item_status_flags`: Checks if a value at a negative offset of a5 is 256 or if a value at a different negative offset is 27; if so, it calculates an index from a global variable, sets a 16-bit word to 1 at that index within a structure pointed to by a5, and calls two other routines.
- `write_and_branch`: Stores a 16-bit word with the value 1 at offset 10 from a5 and then branches to another routine.
- `load_pointer_from_structure`: Loads a 16-bit word from a negative offset of a5, extends it to a 32-bit value, shifts it left by 2, adds it to a value from a4, loads a 32-bit word from the resulting address into a negative offset of a5, sets a 16-bit word at a negative offset of a5 to 1, and branches to a specific offset.
- `check_bit_and_branch`: Loads a 16-bit word from a negative offset of a5, extends it to 32-bit, shifts it left by 2, adds it to a value from a5, loads a 32-bit word from the resulting address into a negative offset of a5, and if bit 0 of the byte at offset 15 of that address is 0, branches to a specific offset; otherwise it extends a 16-bit word from a negative offset of a5 and branches to another offset.
- `process_range_value`: Compares a 16-bit value at offset minus 20 of a5 with 256; if it is not greater than or equal, it checks if a byte at offset minus 19 is between 97 and 122 inclusive, and otherwise calculates a new 16-bit value by subtracting 32 from the signed extension of the value at offset minus 20.
- `validate_and_set_flag`: Compares a 16-bit value stored at offset 20 from a5 with a 16-bit value at offset 16 of a structure pointed to by offset 8 from a5; if they match, it sets a 16-bit value at a calculated offset in a structure pointed to by offset 8 from a5 to 1 and returns a 16-bit value constructed from offsets 24 and 22 from a5.
- `branch_to_middle_of_routine`: Performs a short branch into the middle of another routine.
- `validate_and_mark_word_match`: Fetches a pointer from a structure, calls another routine to process it, and if successful, sets a 16-bit flag in a table at an offset determined by a global variable and a register value.
- `branch_to_target_routine`: Performs a short branch to an offset within another routine.
- `update_and_compare_values`: Decrements a value in a register, then if it is not zero, decrements a 16-bit word at offset 22 from a5, compares it to a 16-bit word at offset 16 of a pointer found at offset 4 from a5, and if the comparison passes, decrements a 16-bit word at offset 22 from a5 again, compares it to a 16-bit word at offset 10 of a pointer found at offset 26660 from a4, and finally branches to another routine.
- `update_table_entry_recursively`: Compares a value from a table with a local field, and if the table value is greater, sets a flag in the table, calls another routine twice to update two local fields, and then calls another routine again.
- `decrement_and_branch_on_d0_2`: Decrements a value in d0 and branches to different locations based on whether the result is zero or matches specific negative constants, otherwise it loads a 16-bit word from offset sixteen before the address in a5 into d0 and branches to another routine.
- `process_inventory_item_logic`: Initializes a local structure and processes a pointer from a global structure, then enters a loop that evaluates a linked structure to update a 16-bit value based on specific numeric codes.
- `update_status_word_2`: Sets a 16-bit word at offset minus 16 of the structure in a5, then checks a 16-bit word at offset minus 30; if that word is zero, it branches to another routine, otherwise it shifts a word from offset minus 28 left by 8 bits, performs a bitwise OR with the word at offset minus 30, stores the result at offset minus 18, and branches to another routine.
- `clear_and_branch_2`: Clears a 16-bit word at a negative offset from a5 and branches to a specific offset.
- `fetch_opcode_and_branch`: Loads a 16-bit word from an offset of 12 bytes before the address in a5 into d0, sign-extends it to a 32-bit long, and branches to another routine.
- `set_flags_and_branch`: Sets a 16-bit word at a negative offset of a5 to 1, clears a 16-bit word at a negative offset of a5, and branches to a specific offset.
- `modify_bitfield_value`: Writes 1 to a 16-bit word at offset minus 16 of the structure in a5, tests the 16-bit word at offset minus 30, and if it is not zero, calculates a new value by ORing a bit-shifted version of the word at offset minus 28 with the word at offset minus 30, stores this in offset minus 18, and branches to another routine.
- `clear_word_and_branch`: Clears a 16-bit word at offset minus 18 from a6 and branches to another routine.
- `branch_on_subtraction`: Subtracts 13 from d0 and branches if the result is zero, otherwise subtracts 14 from d0 and branches if the result is zero, otherwise it branches to another location.
- `fetch_opcode_and_branch_2`: Loads a 16-bit value from an offset of 12 bytes before the pointer in a5 into d0, sign-extends it to a 32-bit long, and branches to another location.
- `jump_via_table_index`: Compares d0 to 8, and if it is not less than 8, shifts d0 left by 1 and uses it to fetch a word from a table to perform a jump.
- `push_and_call_routine_2`: Pushes a word onto the stack, calls another routine, and then performs a short branch.
- `decrement_and_branch`: Decrements d0 by 4 and branches if zero, otherwise decrements d0 by 60 and branches if zero, otherwise decrements d0 by 192 and branches if zero, otherwise it branches to another location.
- `traverse_linked_structure_2`: Recursively traverses a linked structure and sets a byte to one at a specific offset within a structure pointed to by a global variable.
- `process_recursive_structure_3`: Clears a 32-bit value in a local structure, calls several subroutines to process data, and if a specific condition is met, performs a series of recursive calls and updates values in the structure before finally setting a word at offset 8 of a structure to 1.
- `process_recursive_structure_2`: Performs a loop involving recursive calls and conditional branches, eventually updating a value in a structure by bitwise ORing it with a shifted version of another value.
- `execute_recursive_calls_extended`: Performs a series of calls using parameters from a structure in a5, and if a word at offset 8 of a5 is non-zero, it calls another routine with a value of 1, calls another routine with the value from offset 8, and finally calls another routine with a value from offset negative 4 of a5.
- `increment_and_branch`: Sets a 16-bit value at offset minus 6 of the structure in a5 to 1 and branches to another location.
- `iterate_structure_elements`: Calculates an address from a table and index, calls another routine with a pointer from that address, increments a counter, and calls another routine with a different pointer from the same structure.
- `increment_and_compare_index`: Increments a 16-bit value at offset minus 6 of the structure in a5, loads a pointer from offset minus 100 of a4 into a0, and compares the value at offset 10 of the pointer in a0 with the incremented value; it branches to another routine if the comparison is less than, otherwise it clears d0 and branches to another location.
- `update_pointer_from_offset_2`: If a word at offset 8 of the structure in a5 is non-zero, it calculates a new address by adding a value from a global table to a pointer derived from that word, stores the result at offset minus 8 of the structure in a5, sets a 16-bit value at offset minus 10 of the structure in a5 to 1, and branches to another location; otherwise it branches to another location.
- `loop_and_call_routine_2`: Calculates an offset by multiplying a 16-bit value at offset minus 10 of a5 by 4 and adding it to a base address derived from offset minus 8 of a5, loads a 32-bit value from that address into offset minus 4 of a5, checks bit 0 of the byte at offset 15 of the value in a0, and if that bit is set, calls another routine with offset minus 14 of a5 and offset 10 of a5 as arguments.
- `update_and_validate_index`: Increments a 16-bit value at offset minus 10 of the structure in a5, loads a pointer from offset minus 8 of a5 into a0, and compares the value at offset 20 of the pointer in a0 with the incremented value; it branches to another routine if the comparison is less than, otherwise it clears d0 and branches to another location.
- `recursively_process_data`: Checks if two provided values are non-zero and then performs recursive calls using a value fetched from a table based on those parameters.
- `process_array_element_structure`: Retrieves a structure pointer from a table using an index, performs several subroutine calls with various parameters derived from that structure, and updates two word values at offsets 22 and 24 of the structure before returning.
- `update_structure_and_call_subroutines`: Calculates an address by scaling a word from offset -2 of a structure in a5 and adding it to a value at offset -8, then stores a longword from offset 26 of that new address into offset -12 of the structure in a5.
- `divide_and_decrement_counter`: Checks if a byte at a specific offset in a structure equals 45; if so, it calculates a value by subtracting one word from another, divides that result by the calculated value using a division routine, and then enters a loop that decrements the resulting quotient until it reaches zero.
- `branch_to_target`: Performs a short branch to a specific instruction location within another routine.
- `process_structure_data_3`: Checks a bit in a structure, calls several routines with various stack-allocated parameters, updates a word in a structure, and iterates through a table of entries by comparing a word in a structure to a value in another structure.
- `traverse_linked_structure`: Recursively traverses a linked structure by following pointers stored within nodes.
- `update_metrics_and_copy`: Calls another routine, increments a value at offset -18 of the structure in a5, and moves several words between stack offsets before branching to another location.
- `update_text_metrics_and_length`: Iterates through a table of entries, calculating or updating a running length or maximum value, and updates a pointer or offset in a structure.
- `format_conditional_string`: Clears a byte at the address in a0, then checks a word at offset 12 of a5 to decide whether to call a routine with a specific word value or branch to another routine.
- `format_string_partial`: Pushes a word from offset 12 of a5 onto the stack, calls another routine, and then branches to another routine.
- `divide_and_compare_word`: Divides a word at offset 12 of a5 by 256, stores the result back at that offset, and then compares the new value to 59.
- `compare_and_set_register`: Compares a word at offset -4 of a structure to 68 and, if the value is not greater than 68, sets register d1 to 1.
- `format_string_with_offset`: Subtracts 58 from a word at offset 12 of a5, pushes a word from offset 8 of a5 onto the stack, calls another routine, and then branches to another routine.
- `search_table_and_format`: Clears a word at offset -2 of a5, calculates an index by shifting the cleared value and using it to access a table at a negative offset from a4, compares the value at that index with a word at offset 12 of a5, and if they match pushes a longword from offset 8 of a5 and two words to the stack before calling another routine.
- `increment_and_check_limit`: Increments a 16-bit value at offset -2 of a5, compares it to 26, and if it is less than 26, moves a longword from offset 8 of a5 into d0 and unlinks a5 before returning; otherwise it returns.
- `iterate_and_process_table`: Increments a value in a structure, then enters a loop that fetches a pointer from a table and calls two routines if a specific bit is set in the fetched data.
- `branch_to_middle_of_routine_2`: Performs a short branch to a location within another routine.
- `load_word_and_branch`: Loads a 16-bit word from an offset of a5 into d0 and branches to another routine.
- `initialize_stack_frame_and_copy_pointers`: Initializes a local stack frame, clears a 16-bit value at a negative offset from a5, and copies several pointers from a structure pointed to by a4 into the local stack frame before branching to another routine.
- `update_array_elements_from_global_table`: Iterates through a loop to populate two arrays within a structure using values from a global table, based on an index stored at a negative offset of the stack frame.
- `initialize_array_elements`: Calls another routine to check a condition, then uses a value from the stack to calculate offsets into two arrays within a structure, writing specific 16-bit values to those locations before branching to another routine.
- `allocate_and_initialize_node`: Sets a word in a global variable, clears a word in a structure in a5, calls three different routines with various parameters including a pointer from a5, and finally branches to another location.
- `increment_and_check_value_2`: Increments a word at a negative offset from a5, pushes a value from a5 to the stack, and calls another routine.
- `clear_and_initialize_node_arrays`: Clears two 16-bit words in arrays located at offsets 80 and 100 from a structure using an index stored at offset 2, then calls another routine to process these arrays if the index is greater than 0 and a specific global flag is not set.
- `update_table_entry_4`: Writes a word to a global variable, clears a word at an offset in a5, and calls two different routines using values from the stack and a global variable.
- `update_table_entry_2`: Sets a 16-bit value at an offset in a global variable, clears a 16-bit value at offset 102 of a structure in a5, calls another routine to search a table, and returns the value from the cleared location in a5.
- `load_byte_from_pointer_4`: Stores a word into a global variable, increments a pointer in a stack location, and if that pointer is not zero, loads a byte from the address it points to into d0.
- `compare_d0_and_unlk`: Sets d0 to zero, compares it to 7, and if it is not equal, branches to another location; otherwise it sets d0 to zero, unlinks a5, and returns.
- `write_byte_and_check_pointer`: Writes the byte 6 to the address stored at offset -8 of a5, stores a pointer in a global variable, updates a pointer at offset -14 of a5, and if that pointer is non-zero, loads a byte from it into d0 and branches.
- `compare_byte_and_branch_4`: Compares register d0 to seven and branches if they are not equal; otherwise, it clears d0 and performs an unconditional branch.
- `modify_structure_and_load_byte`: Stores the byte five at the location pointed to by offset -4 of a5, copies a long from offset -4 of a5 to offset 2 of a location pointed to by offset -8 of a5, increments a value at offset -20 of a5 by ten, and if the new value at offset -16 of a5 is not zero, loads a byte from that location into d0.
- `compare_and_clear_d0_2`: Compares the value zero to seven in register d0 and branches to another routine if they are not equal; otherwise, it clears register d0 and branches to another routine.
- `update_linked_list_node_10`: Sets a byte to 6 at a location pointed to by a stack offset, updates a pointer in a structure, increments a value by 10, and loads a byte from the resulting pointer into a register.
- `compare_and_branch_9`: Clears d0, compares it to seven, and branches to another location if not equal; otherwise it clears d0 and branches to another location.
- `initialize_and_process_structure_2`: Sets a byte in a structure, stores a longword at an offset of another structure, writes 321 to an offset of a structure, calls a routine, updates a word in a structure with the result, and conditionally calls another routine.
- `push_pointer_and_call`: Sets up a stack frame, pushes a 16-bit constant and a 32-bit pointer from offset 8 of a structure in a5 onto the stack, calls another routine, and then stores the returned value into offset 8 of the structure.
- `fetch_nested_byte_2`: Traverses a pointer chain starting from offset 8 of a structure in a5, checking if the value at offset 6 of the second level is non-zero, and if so, loads a byte from the address pointed to by offset 6 of the third level into d0.
- `fetch_nested_byte`: If d0 is not 2, it follows a chain of pointers starting from offset 8 of a structure in a5 to reach a target, then loads a byte from that target into d0; otherwise it branches to another routine.
- `update_nested_pointer_value`: If d0 is not 6, it follows a chain of three pointers starting from an offset of 8 in a5 to reach a target, writes a 16-bit word from that target to an offset of -26742 from a4, and unlinks a5 before returning.
- `update_nested_pointer`: Pushes a word and a pointer to the stack, calls another routine, and then writes the result to an offset in a5 before checking if the new pointer is zero.
- `traverse_pointer_chain_4`: Loads a pointer from offset 8 of the structure in a5, loads a pointer from offset 6 of that pointer, and if that pointer is non-zero, it repeats this process to load a pointer from offset 6 of the second pointer, then loads a byte from the address pointed to by the third pointer into d0 and branches to another location; otherwise it performs similar checks and branches to another location.
- `traverse_pointer_chain_3`: Compares d0 to 2 and, if not equal, follows a chain of pointers starting from offset 8 of a structure in a5 through offsets 6 and 6 of subsequent structures to load a byte into d0.
- `compare_and_return_index`: Compares d0 to 6 and branches if equal, otherwise loads a pointer from offset 8 of a5, loads a pointer from offset 2 of that pointer into a1, compares a 16-bit value at offset 2 of a1 with a 16-bit value at offset 12 of a5, and returns the 16-bit value from offset 8 of a5 if they are equal.
- `branch_to_target_2`: Sets d0 to 0 and branches to another location.
- `traverse_pointer_chain`: Follows a chain of pointers starting from offset 8 of a structure in a5, updating offsets -4 and -8 of that structure along the way, and compares a 16-bit word from a nested structure to a value at offset 12 of the structure in a5.
- `branch_to_target_offset`: Performs a short branch to a specific instruction offset within another routine.
- `execute_with_parameters`: Pushes a 16-bit word, a constant, and a pointer from an offset of 8 in a5 onto the stack, calls another routine, cleans the stack, pushes the result from d0 onto the stack, calls another routine, cleans the stack, and branches to another location.
- `compare_and_update_value`: Calls a routine, moves a word from offset negative 120 of a5 to d0, compares it to a value in a global variable, and conditionally updates that global variable or sets d0 to zero before returning.
- `execute_callback_with_pointer_2`: Loads a pointer from offset 16 of a5 and branches if the word at that pointer is zero, otherwise it loads a second pointer from offset 8 of a5 and branches if its word is zero, then calls a routine using a value from offset 24 of a5.
- `clear_d0_and_branch`: Clears register d0 and performs a short branch to another routine.
- `update_structure_and_call_subroutine`: Calls a subroutine and stores the result in a structure at offset minus 82, clears three 16-bit words at offsets minus 74, minus 76, and minus 78, copies a longword from offset 20 to offset minus 72, and then calls another subroutine using a pointer from offset 16.
- `process_event_and_update_text`: Compares two values retrieved from a structure and, if they match, updates several pointers and counters within that structure.
- `compare_and_branch_7`: Clears d0 and compares it to 7, then branches to another routine if they are not equal, otherwise it clears d0 and branches to another routine.
- `dispatch_event_structure`: Sets a byte to 4 in an event structure, copies a word from a pointer into that structure at offset 2, and copies a longword from a pointer into the structure at offset 6.
- `dispatch_event_structure_2`: Increments a value at offset 68 of the structure by 2, then calls another routine with a pointer to the structure and a pointer to a table entry.
- `increment_pointer_and_load_byte`: Loads a pointer from offset 28 of a structure in a5 into a0, increments the longword at offset 28 of the structure in a5 by 10, stores the original pointer into offset -90 of the structure in a5, and if that pointer is not zero, loads a byte from the address in a0 into d0.
- `compare_byte_and_branch_2`: Compares the value zero to the value seven in d0 and branches to another routine if they are not equal, otherwise it clears d0 and branches to another routine.
- `update_linked_list_node_9`: Sets a byte at a specific offset in a structure to five, calls another routine to fetch a value, and stores that value into a two-byte field at an offset within the same structure.
- `compare_zero_constant`: Compares zero to the constant 7 and branches if they are not equal, otherwise it sets d0 to zero and branches.
- `initialize_linked_list_node`: Sets a byte at a location pointed to by an offset in a structure to 6, copies a 32-bit value from another offset into the second offset of that same location, and clears the sixth offset.
- `compare_byte_and_branch`: Compares d0 to 7 and, if not equal, branches to another location; otherwise it clears d0 and branches to another location.
- `update_linked_list_node_8`: Sets a byte at a location pointed to by a structure offset to 5, calls another routine to fetch a value, stores that value into a 16-bit field at offset 2 of the same location, increments a pointer in a structure by 10, and then retrieves a byte from the new pointer location.
- `check_constant_byte`: Compares d0 to 7 and branches if not equal, otherwise sets d0 to 0 and branches to another location.
- `update_linked_list_node_6`: Writes the value 6 to a byte at the location pointed to by an offset in a structure, then copies two 32-bit values from other offsets in the same structure into the memory at that location.
- `compare_and_branch`: If d0 is not 7, it returns; otherwise, it sets d0 to 0 and branches to another location.
- `initialize_and_process_event_structure`: Initializes a structure by setting a byte to 6, copying a longword from an offset, and clearing a 32-bit field.
- `modify_structure_fields`: Copies a word from offset -76 of a5 to offset -78 of a5, adds 20 to a longword at offset -72 of a5, and branches based on a comparison of a word at offset -74 of a5.
- `fetch_byte_from_pointer`: Clears a longword at offset minus 4 of the structure in a5, loads a pointer from offset 8 of the structure in a5, and if a word at offset 2 of that pointer is non-zero, it performs a second check on that same word and, if non-zero, loads a byte from the address pointed to by offset 2 of the pointer into d0 and branches to another location; otherwise it branches to another location.
- `process_pointer_value`: If d0 is 6, it pushes a word from offset 2 of a pointer found at offset 8 of a5 onto the stack, calls another routine, and stores d0 into offset -4 of a5; otherwise it stores d0 into offset -4 of a5 and potentially performs checks on the pointer at offset 8 of a5.
- `call_routine_with_pointer`: If d0 is 6, it loads a pointer from an offset of 8 in a5, pushes a value from offset 6 of that pointer onto the stack, calls another routine, cleans the stack, stores the result from d0 at offset -4 of a5, and branches to another location; otherwise it returns.
- `update_pointer_if_nonzero`: Loads a pointer from offset 8 of the structure in a5, tests the longword at offset 6 of that pointer, and if it is not zero, replaces the pointer at offset 8 of the structure with the value at offset -4 of the structure, otherwise it branches to another location.
- `load_and_test_pointer`: Loads a pointer from offset 8 of the structure in a5, tests the longword at offset 6 of that pointer, and if it is not zero, loads a byte from offset 6 of the pointer into d0 before branching.
- `call_and_store_pointer_offset_value`: If a register contains the value six, it loads a pointer from an offset of eight in a6, pushes a longword from an offset of six of that pointer onto the stack, calls another routine, cleans the stack, and stores the returned value into an offset of four less than the base address in a5.
- `clear_returned_value`: Clears a 32-bit value at an offset from a pointer and returns that cleared value.
- `branch_immediately`: Sets up a stack frame and branches to another location.
- `validate_and_copy_character`: Increments a pointer at offset 8 of a5 and enters a loop that checks if the byte at that pointer is a space, tab, newline, or carriage return, or if it falls within specific alphanumeric ranges, eventually copying a byte to a destination at offset 12 of a5.
- `validate_character_range`: Loads a pointer from offset 8 of the structure in a5 and performs multiple comparisons against the byte at that pointer to check if it falls within specific ranges for lowercase letters, uppercase letters, digits, or specific control characters, incrementing the pointer at offset 8 if a match is found.
- `branch_to_offset_11`: Performs a short branch to a different location.
- `branch_to_pointer_in_a22`: Performs a short branch to the address stored in the memory location pointed to by the second byte of the address in a22.
- `clear_byte_at_pointer`: Loads a pointer from an offset of 12 in a5, clears the byte at that address, and branches to another location.
- `skip_whitespace_chars`: Increments a pointer and advances it while the pointed-to character is a space, tab, newline, or carriage return, stopping when the character is not whitespace or is the null terminator, then returns the updated pointer.
- `setup_stack_and_branch_2`: Initializes a new stack frame and performs an unconditional short branch to another routine.
- `validate_ascii_range`: Loads a pointer from offset 8 of the structure in a5, compares the byte at that pointer to 48, and if it is not less than 48, compares it to 57; if it is not less than or equal to 57, it clears d0, unlocks a5, and returns.
- `increment_and_test_byte_3`: Increments a longword at offset 8 of the structure in a5, loads a byte from the pointer at offset 8 into a0, and if that byte is not zero, branches to another location; otherwise it sets d0 to 1 and branches.
- `search_word_list`: Stores a pointer in a global variable, checks a global variable, and if it is not zero, calls another routine twice with different word values, and if the second call returns a value with bit 7 set, calls another routine; otherwise it stores a value in a global variable and branches to another routine.
- `clear_and_return_2`: Clears a 16-bit word at a negative offset from a4, and if that word is zero, sets d0 to zero, unlinks a5, and returns; otherwise it returns.
- `compare_and_call_stack`: Compares a word in d0 to 300, and if it is not less than that value, pushes a pointer to a location relative to the program counter onto the stack, calls another routine, adjusts the stack pointer, and branches to another routine.
- `process_bitstream_data`: Copies a long value from a global variable to another global location, sets two global words to 1, calls a routine with offset 8 of a5, calls another routine with offset -100 of a5, and if the result is not 1, clears two global words and calls a third routine with a global pointer and a constant.
- `call_subroutines_with_parameters`: Pushes a word, a pointer, and a pointer to a structure in a5 onto the stack, calls a routine, then pushes a word and a pointer to a structure in a5 onto the stack to call another routine before branching.
- `parse_numeric_specifier`: Checks a pointer at offset 12 of a5 against specific thresholds, calculates a value based on the high byte of the word at that pointer, stores the result at offset -32 of a5, and calls another routine with a pointer from offset -20 of a5 and a byte from a table.
- `call_format_routine`: Pushes a pointer from offset -20 of a5 and a pointer from offset -26476 of a4 onto the stack, calls another routine, and then branches to another routine.
- `loop_format_string_specifiers`: Calls a routine using a pointer from offset 12 of a5, then calls another routine using a value from a global variable, increments the word at offset 12 of a5, and loops if that word is non-zero.
- `process_bitstream_byte`: Sets up a stack frame, increments a pointer at offset 8 of the structure in a5, loads a byte from the location pointed to by the original pointer, and if the high bit of that byte is set and the byte is not 255, clears a word at the location pointed to by offset 12 of the structure.
- `shift_and_store_byte`: Shifts a byte from offset -1 of the structure in a5 left by 8 bits, stores the result at offset -4 of the structure, increments a pointer at offset 12 by 2, stores the value from offset -4 into the location pointed to by that pointer, and calls another routine.
- `update_bitstream_buffer`: Reads a byte from a structure, shifts it, and performs a bitwise OR with a value retrieved from a secondary pointer, then writes the result to a destination pointer and calls another routine.
- `call_indirect_with_check`: Sets up a stack frame, calls another routine, and if the result in d0 is 300 or greater, calls a second routine using a pointer from the program counter, otherwise it clears d0 and returns.
- `update_global_pointer`: Copies a long value from a global variable to another location, increments the source pointer by 10, moves the new pointer to offset -12 of a5, and if that pointer is not null, loads a byte from its location into d0.
- `compare_and_branch_zero_3`: Compares d0 to 7 and branches if not equal, otherwise it clears d0 and branches to another location.
- `update_structure_and_pointer_2`: Writes a byte to a location pointed to by a5, updates a global pointer, increments a value in a global structure, and then either branches or moves a byte from a structure into d0.
- `compare_and_branch_13`: Compares the value in register d0 to seven; if it is not seven, it returns, otherwise it clears d0 and branches to another location.
- `update_linked_list_node_11`: Initializes a structure with a type value of 5 and a length of 513, then links it to a previous node.
- `check_byte_value_2`: Clears register d0 and compares it to 7, then either clears d0 and branches or clears d0 and branches to another location.
- `update_structure_and_pointer`: Sets a byte to six at a location pointed to by a structure, updates a pointer in that structure using a global value, and increments a global variable by ten before reading a byte from the new pointer.
- `compare_and_branch_12`: Compares the value in register d0 to seven and branches to another routine if they are not equal, otherwise it branches to a different location.
- `update_structure_and_call`: Sets a byte at offset 0 of a structure, a word at offset 2, and a longword at offset 2 of another structure, then calls a routine and conditionally calls another routine based on the result.
- `set_conditional_word`: Clears d0, compares it to one, performs a sequence to set d0 to zero or one, stores the result at offset -4 of the structure in a5, and branches to another location.
- `push_parameters_and_call`: Pushes a longword from the stack, pushes a longword onto the stack, calls a routine, adjusts the stack pointer, clears a word in a structure, and branches.
- `compare_and_call_routine_3`: Calls another routine, compares the result to 300, and if it is not less than 300, pushes a pointer to the stack and calls another routine before returning.
- `check_attribute_consistency`: Loads a pointer from an offset of a structure in a5, accesses a second pointer at offset 2 of that pointer, and compares the word at offset 2 of the second pointer to 331; if they match, it clears a word in a global variable and branches to another location.
- `validate_attribute_type_consistency_2`: Copies two 16-bit values from a structure into another structure and checks if a specific attribute type ID matches two constant values; if a match is found, it sets a value in the destination structure to one, otherwise it sets it to zero.
- `validate_attribute_type_consistency`: Checks if two nested attributes share the same type ID or match specific constant values, updating a status flag in a global variable and a result word in the structure at offset -2 of a5.
- `extract_nested_attribute_byte`: Traverses multiple levels of nested pointers starting from offsets 8 and 12 of a structure to retrieve a byte value.
- `compare_nested_attribute_values_2`: Compares byte values and shortword values retrieved from multiple levels of nested pointers within structures, then calls compare_nested_attribute_values or fetch_nested_attribute_byte_2 based on the results and stores a 16-bit result into a location at a negative offset from the input pointer.
- `fetch_nested_attribute_byte_2`: Checks if a byte at a deep offset within a nested structure is equal to six; if not, it traverses multiple levels of pointers to retrieve a byte from a specific offset and returns it in d0.
- `compare_structure_fields_2`: Compares d0 to 2 and branches if equal, otherwise dereferences a pointer from a structure to check if it is non-zero, and if so, dereferences further to load a byte into d0.
- `compare_structure_fields`: Checks if d0 is 6, then performs multiple nested pointer dereferences to compare words at offset 2 of various structures, eventually setting a word at offset negative 2 of a5 to either 1 or 0.
- `write_negative_one_to_offset`: Sets d0 to negative one, stores it at offset -2 of a5, and branches.
- `compare_and_call_subroutine_2`: Compares a word at offset 2 of a structure to 321, and if it matches a word from another structure, it calls a routine using a value from offset 6 of a structure and stores the result at offset -2 of a5.
- `clear_and_branch_4`: Clears a 16-bit word at offset -2 of a5 and then branches to another location.
- `compare_nested_attribute_values`: Traverses multiple levels of nested pointers to compare byte values and shortword values within structures, then calls another routine to update a value at a negative offset of the input structure based on the comparison results.
- `fetch_nested_attribute_byte`: Checks if a byte at a deeply nested pointer location is equal to two; if not, it traverses multiple levels of structures to load a byte into d0 and returns.
- `validate_and_clear_field`: Compares d0 to six and branches to another location if equal; otherwise it performs several pointer dereferences and comparisons involving offsets of a5, and if certain conditions are met, clears a word at offset -2 of the structure in a5 before branching to another location.
- `call_indirect_and_store`: Loads pointers from a5 and a0, pushes two longwords onto the stack, calls another routine at an indirect address, and stores the result in a 16-bit word at offset -2 of a5.
- `load_byte_from_nested_structure`: Loads a pointer from an offset of a structure, checks if a longword at an offset of that pointer is zero, and if not, loads a byte from an offset of a second pointer into d0.
- `check_nested_structure_values`: Checks if d0 is 5, then performs multiple pointer dereferences to compare words at offset 2 of nested structures, and if certain conditions are met, moves a byte from a nested location into d0.
- `fetch_nested_byte_3`: Traverses a multi-level pointer structure starting from an offset of 12 in a5 to retrieve a single byte from a nested location, returning zero if the target pointer is null or the byte value otherwise.
- `compare_and_call_nested`: Compares a word from a nested structure to a word in a5, and if they match, pushes two longwords onto the stack and calls another routine, storing the result in a 16-bit word at offset -2 of a5.
- `process_callback_data`: Pushes a longword from offset 12 of a5 onto the stack, calls another routine, stores the result in a word at offset -2 of a5, and updates several global variables if certain conditions are met.
- `call_stack_offset_routine`: Links a stack frame, calls another routine, compares the result in d0 to 300, and if d0 is not less than 300, pushes a constant address onto the stack and calls another routine before returning.
- `check_object_property`: Loads a pointer from offset 12 of a5 into a0, loads a second pointer from offset 2 of a0 into a1, compares the word at offset 2 of a1 to 331, and if they are not equal clears a word at a negative offset from a4 and branches to another routine.
- `validate_object_type_property`: Copies two words from a global structure to a structure in a5, then checks if a 16-bit value at offset 2 of a linked list element pointed to by offset 12 of the structure in a5 matches either 335 or 336, storing the result in a global variable.
- `validate_linked_list_property`: Checks if a 16-bit value at offset 2 of a pointer (found at offset 12 of the structure in a5) matches 338; if so, it stores the result in a local variable and proceeds to evaluate a byte at the same offset, potentially calling another routine recursively through a loop that traverses a linked list.
- `evaluate_linked_list_node_property`: Checks if a byte in d0 is five; if so, it traverses a linked list via a structure at offset 12 of a5, comparing values at offset 2 of the list nodes against constants 321 and 338.
- `fetch_linked_list_byte`: Compares a byte in d0 to 2; if not equal, it returns, otherwise it traverses a pointer chain through a structure at offset 12 of a5 to load a byte from a nested pointer into d0.
- `execute_conditional_subroutine`: Compares d0 to 6; if not equal, it returns, otherwise it loads a pointer from offset 12 of a structure in a5, loads a second pointer from offset 6 of that pointer, pushes a longword from offset 6 of the second pointer and a longword from offset 8 of a5 onto the stack, calls a subroutine, cleans the stack, stores d0 into offset -2 of the structure in a5, and branches to another routine.
- `load_byte_from_pointer_3`: Checks if a 32-bit pointer at offset 8 of a structure in a5 is zero; if not, it loads a byte from the address pointed to by that pointer into d0.
- `compare_and_load_byte`: Compares d0 to 6 and, if not equal, branches; otherwise it loads a pointer from offset 8 of the structure in a5, checks if its second word is zero, and if not, loads a byte from the address pointed to by the second word of that pointer into d0.
- `clear_values_on_mismatch`: Compares d0 to 6 and, if not equal, branches to another location; otherwise it clears a 16-bit value at offset minus 4 of the structure in a5 and a 16-bit value at offset minus 2 of the structure in a5, then branches to another location.
- `compare_and_call_routine`: Loads a pointer from offset 8 of the structure in a5, retrieves a longword from offset 2 of that pointer to store at offset -12 of the structure, compares a word at offset 2 of the new pointer with a word at offset 2 of a pointer at offset 12 of the structure, and if they are equal, calls another routine and stores the result in offset -4 of the structure.
- `process_conditional_data`: Loads a pointer from offset -12 of the structure in a5 and checks if the word at offset 2 of the pointer's target is 321; if so, it calls another routine using pointers from offset -12 and offset 12, stores the result in offset -4, and performs several conditional checks and assignments to offset -2 and global variables in a4.
- `update_pointer_from_offset`: Copies a word from offset -4 of a5 to offset -2 of a5, loads a pointer from offset 8 of a5, and replaces the longword at offset 6 of that pointer with the value from offset 8 of a5 before loading a byte from the new pointer into d0.
- `check_status_and_branch`: Compares d0 to 6, and if they are equal, compares a word at offset -2 of the structure in a5 to 1, then branches to another location based on the results.
- `execute_and_store_result`: Calls another routine using pointers from offset 8 and offset 12 of the structure in a5, stores the result in offset -2, and if a global variable in a4 is non-zero and the result is zero, it sets offset -2 to 1 and updates two global variables in a4 using values from offset -6 and offset -8.
- `call_and_validate_result`: Sets up a stack frame, calls a routine at an indirect address, compares d0 to 300, and if d0 is less than 300, it pushes a longword from a location relative to the program counter onto the stack, calls another routine, increments the stack pointer by 4, clears d0, tears down the stack frame, and returns; otherwise it tears down the stack frame and returns.
- `set_status_word`: Stores the word 1 into offset -2 of a5 and branches to another location.
- `traverse_and_call_routine`: Checks if a word at a specific offset in a nested pointer structure is 325; if not, it calls another routine using pointers from offset 8 and a pointer derived from offset 12, stores the result in offset -2, and checks a global variable in a4 to determine whether to continue or branch.
- `load_pointer_and_byte`: Compares a word at offset -2 of the structure in a5 to 1, and if equal, loads a pointer from offset -4 of the structure into a0, then loads a longword from offset 6 of that pointer into offset -4 of the structure, and if that longword is non-zero, loads a byte from the pointer in a0 into d0.
- `compare_byte_and_load_word`: Compares d0 to 6, and if they are not equal, loads a word from offset -2 of the structure into d0 before branching.
- `clear_two_words`: Sets two consecutive words in memory to zero.
- `retry_on_zero_result`: Calls another routine and, if the result is zero, calls it again to return the second result; otherwise, it returns the first non-zero result.
- `process_node_and_allocate`: Performs several operations involving a pointer, including calling another routine with a word-sized value and conditionally executing a routine if a specific field in a node is non-zero.
- `transfer_word_and_branch`: Sets up a stack frame, moves a word from a location at offset negative 26600 of a4 to offset negative 2 of the structure in a5, and branches to another routine.
- `copy_values_in_loop`: Calls a routine with a word from offset -2 of a5, calls another routine with the result in d0, and copies a word from a global variable to offset -2 of a5 while the global variable is non-zero.
- `call_and_conditional_call`: Calls a routine with a word from offset 8 of a5, stores the result in offset -4 of a5, and if the result is non-zero, calls another routine passing that value as an argument.
- `execute_pointer_sequence`: Performs a series of operations using values retrieved from a structure via multiple offsets.
- `check_and_dispatch_call`: Calls a routine with a word from offset 8 of a5, and if the result is non-zero, calls another routine with a parameter and that value, then calls a third routine.
- `update_global_offset_2`: Pushes a word from offset 8 of a5 onto the stack, calls another routine, adds the result to a global variable, and returns.
- `traverse_linked_list_3`: Pushes a word from a pointer in a2 onto the stack, calls another routine, and stores the result in a2, then continues to call another routine and update a2 by adding a value from a2 to itself in a loop.
- `execute_linked_list_node_function`: Checks if a pointer at offset negative 4 of a structure is non-zero, then performs a recursive call by pushing a 16-bit value from a calculated address onto the stack and storing the returned result at offset negative 10 of the structure.
- `process_linked_list_nodes_2`: Executes a loop that iterates through a linked list, performing calculations on pointer offsets and updating values in a local structure.
- `load_word_and_branch_3`: Loads a word from a structure in a2 into d0 and extends it to a longword, then branches to another routine.
- `jump_via_table_3`: Compares a long value in d0 to eleven, and if it is not less than eleven, shifts d0 left by one, fetches a word from a table using d0 as an offset, and jumps to an address calculated from that word.
- `branch_via_pointer_offset`: Loads a word from offset 2 of a2 and extends it to a longword, adds it to a2, and if the longword at the new a2 is non-zero, branches to another routine; otherwise, it restores a2 from the stack and returns.
- `adjust_stack_pointer`: Initializes a stack frame, modifies a pointer by subtracting 8, stores it on the stack, and performs a bitwise right shift on a word from the stack before branching.
- `process_data_loop_2`: Iterates through a list of pointers, calling a routine for each, and then performs a series of conditional calls and pointer updates based on a structure.
- `process_structure_updates_2`: Sets a 16-bit value at offset 2 of a structure in a0 to zero, copies a word from offset negative 2 of a0 to offset negative 16 of a5, and if that word is 1, loads a word from a location pointed to by offset negative 4 of a5 into d0.
- `process_structure_updates`: Calculates a value by adding a longword from offset 8 of a5 to a doubled word from offset -16 of a5, calls another routine, and stores the result at offset 8 of a5.
- `decrement_and_store_pointer`: Sets d0 to zero, loads a pointer from an offset of -8 from a5 into a0, adds 2 to the longword at that pointer, stores d0 at the pointer, and branches to another routine.
- `decrement_pointer_loop`: Repeatedly subtracts one from a short stored at a fixed offset relative to a register while adding two to a 32-bit value at another fixed offset and overwriting a 16-bit pointer target with the result of calling another routine.
- `check_word_equality`: Compares a word at an offset in a stack frame to negative one and returns if they are not equal, otherwise it returns with register d0 cleared.
- `fetch_indexed_value`: Calculates an index by shifting a word from a structure, fetches a word from a table using that index, and if the word is zero, calls another routine to fetch it again.
- `call_and_store_d0`: Pushes a word from a location relative to a5 onto the stack, calls another routine, stores the result in d0 at an offset relative to a5, and branches to another location.
- `fill_data_table_2`: Calls a routine to obtain a pointer, stores that pointer in a global variable, and then iterates through a list to store words into a table at an offset derived from a counter.
- `fill_data_table`: Iterates through a list, calling a routine to get a value and storing it in a table at an offset derived from a counter, then updates a global count.
- `clear_stack_word`: Links a5 and clears a word at an offset relative to a5 before branching to another location.
- `search_and_clear_table`: Iterates through a table to compare values and sets a word to zero if a match is found.
- `link_and_branch`: Creates a new stack frame and performs a short branch into another routine.
- `extend_pointer_value`: Loads a pointer from an offset relative to a5, fetches a word from that pointer, extends it to a long value in d0, and branches to another location.
- `clear_offset_word`: Loads a pointer from an offset relative to a5, adds four to it, stores this new pointer at an offset relative to a5, and clears the word at the address pointed to by the new pointer before branching.
- `extend_and_add_offset`: If d0 is neither 1 nor 5, loads a pointer from offset 8 of a5, extracts a word from offset 2 of that pointer, extends it to a longword, and adds it to the longword at offset 8 of a5; it then branches to another routine if the word at the pointer is not zero.
- `populate_global_array_2`: Retrieves a word from a structure, calls a routine, and then enters a loop that searches a list of pointers and stores values from offset 18 of those pointers into a global array if a counter is below 10.
- `populate_global_array`: Searches a table for values and stores them into a global array if a counter is below a specific limit.
- `copy_sequence_of_words`: Iterates forty-eight times to store words obtained from another routine into a global memory area.
- `initialize_and_call_subroutine`: Saves registers, checks a global word, and if zero, calls a routine, sets several global values including a pointer and a word, and then performs multiple calls to other routines while updating global variables.
- `call_indirect_with_stack_setup`: Pushes the word 65, the word 0, the value in d1, and the value in d2 onto the stack, calls a routine at an offset of -31984 from a4, and restores registers a0 through a3 and a6.
- `swap_global_pointer`: Saves a value from a global variable, replaces it with a value from another location via a routine call, and then restores the original value.
- `modify_pointer_and_value`: Calculates a new pointer address, updates the value at that address, and restores previous register states.
- `call_and_return_result`: Calls another routine and returns the value it leaves in a register.
- `compare_and_fetch_pointer`: Pushes d1 and other registers to the stack, calls a routine through a pointer, and then compares a word in d1 with a word at offset 10 of a structure to potentially fetch a new pointer into d1.
- `push_arguments_and_call`: Pushes the word in d1, the word 0, the longword in a2, and the longword in a0 onto the stack, then calls a routine at an offset of -31764 from a4.
- `execute_jump_table_entry`: Loads a byte from a0 and increments a0, doubles that byte, and uses it as an index into a table relative to the program counter to perform an indirect jump.
- `copy_and_jump_table_2`: Loops a specified number of times to copy words from a3 to a2 while incrementing both pointers, then uses a byte from a0 to index into a table.
- `copy_and_jump_table`: Loops a specified number of times to copy words from a3 to a2 while incrementing both pointers, then uses a byte from a0 to index into a table relative to a6 for an indirect jump.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `return_constant_3`: Returns the constant 0 in d0 to the caller without any other effect.
- `format_string_with_specifiers`: Iterates through a source string and copies bytes to a destination, but processes percent-sign specifiers by calling parse_signed_numeric_string and other routines to handle various format characters such as numeric conversions and variable substitutions.
- `parse_signed_numeric_string`: Increments a pointer in a structure, clears three 16-bit values in a global variable, and checks for a negative sign; if a minus sign is found, it sets a flag in the global variable and continues to check if the subsequent characters are numeric digits.
- `parse_numeric_token`: Calls a routine to fetch a byte from a pointer stored at offset 12 of a structure in a5, stores the result in a 16-bit value at a global variable, and increments the pointer at offset 12 if the fetched byte is a numeric digit between 0 and 9.
- `fetch_format_specifier_byte`: Increments a pointer stored at offset 12 of a structure in a5, loads the byte at the resulting address into d0, and extends it to a 32-bit value before branching.
- `parse_string_format_tokens_2`: Increments a pointer in a structure, calls another routine to fetch data, and then enters a loop to parse a string for format specifiers such as percent signs, hyphens, colons, and specific character markers to copy or process data into a destination buffer.
- `parse_string_tokens`: Iterates through a string to identify and process specific characters, such as percent signs, hyphens, and colons, while calling other routines to handle nested structures or specific command characters like 'U' or 'X'. It updates a pointer in a structure to track the current position in the string and stores results in a destination buffer.
- `parse_string_format_tokens`: Parses a string by iterating through its characters, handling specific control characters such as percent signs for variable substitution, hyphens for sign handling, and colons for numeric parsing.
- `parse_string_token`: Increments a pointer in a structure, calls another routine to process a value, and then enters a loop that parses characters from a string buffer into a destination buffer.
- `parse_format_string`: Parses a format string by iterating through characters and calling other routines to handle specific control characters such as percent signs, hyphens, colons, and letters.
- `increment_and_call_string_parser`: Increments a pointer stored at offset negative 4 of a structure by 2, copies a byte from the memory at the new pointer location to offset negative 28 of the structure, clears offset negative 27, and calls another routine using a pointer stored at offset negative 8.
- `increment_and_call_string_parser_2`: Increments a pointer stored at offset -4 of a structure, pushes the original pointer and a value from offset -8 of the structure onto the stack, calls another routine, and stores the result in offset -8 of the structure before branching.
- `copy_byte_to_buffer`: Copies a single byte from a source pointer located at offset 12 of a structure into a destination pointer located at offset -8 of the same structure, then increments the destination pointer by 1 and branches.
- `dispatch_format_specifier`: Compares a value in register d0 against several constants and branches to specific subroutines to process different format specifiers.
- `branch_to_offset_7`: Performs a short branch to an offset.
- `clear_byte_and_subtract`: Clears a byte at the address pointed to by the value at offset negative 8 of a5, then returns the difference between the value at offset negative 8 and offset 8 of a5.
- `compare_and_fill_buffer`: Calls a routine, stores the result in a structure at offset -10, and if the result is greater than or equal to a global variable, clears that global variable and sets d0 to 48; otherwise, it checks a global variable and branches.
- `update_structure_values_2`: Moves a byte from offset 0 of a structure in a5 to offset -1 of the same structure, moves a word from a global variable to offset -8 of the structure, and moves a longword from offset 8 of the structure to offset -6 of the structure.
- `copy_bytes_and_update_offset`: Copies a byte from a source at offset -1 of a structure in a5 to a destination pointed to by a longword at offset -6 of a5, decrements a 16-bit counter at offset -8, and increments the longword at offset -6 in a loop.
- `update_pointer_and_copy_byte`: Calculates a new longword by adding a value at offset 8 of a structure to the sign-extended word at offset -10, stores it at offset -6, and then loops through a source buffer to copy bytes to a destination buffer until a null byte is encountered.
- `return_longword_at_offset`: Returns the longword stored at offset negative 6 of a5.
- `initialize_audio_device`: Attempts to initialize the audio device and, if successful, loads and formats audio data, manages a list of active audio channels in a global table, and calculates an offset for audio playback.
- `load_and_format_data`: Calls two routines, stores a result in a global variable, and if that result is non-zero, performs arithmetic to calculate an offset and copies a longword from a specific location to a destination in a4.
- `format_error_message`: Pushes a pointer from offset negative 24608 of a4, a pointer to a global variable, and a pointer from offset negative 130 of a5 onto the stack to call another routine, cleans the stack, pushes a pointer from offset negative 130 of a5 onto the stack to call another routine, cleans the stack, clears d0, unlinks a5, and returns.
- `initialize_music_system`: Clears the stack, calls a routine, stores the result in offset -4 of a5, calls another routine with a value from offset -50 of a5, and if the result is not negative 1, calls a routine with a pointer from a4 and then calls another routine with a PC-relative address; otherwise it calls a routine with a pointer from a4 and then calls another routine with a PC-relative address.
- `update_global_counters_2`: Adds 46 to a word from a global variable and stores it back, clears two words in a global variable, pushes a word onto the stack, and calls another routine.
- `manage_linked_list_elements_3`: Calls a routine with a pointer, calls another routine with a word from a structure, and stores the result in an offset of that structure.
- `loop_and_call_with_check`: Compares a long value from a global variable with a value at an offset relative to a5, and if they are not equal, pushes a pointer and a constant onto the stack, calls another routine, and then loops if a word at an offset relative to the pointer is not equal to sixty-five-five-hundred-thirty-five.
- `execute_dispatch_call`: Loads a pointer from offset 8 of a5 into a0, pushes a word from offset 6 of that pointer and the constant 132 onto the stack, calls a routine, and then performs an unconditional branch.
- `call_with_arguments`: Pushes a pointer and a constant onto the stack, calls another routine, and then branches to another location.
- `decrement_and_unlk_a5`: Decrements a longword in d0 up to two times and, if the result is not zero, unlinks a5 and returns; otherwise it returns.
- `manage_linked_list_elements_2_2`: Calls a routine with a word from a structure, uses the result to index into a table to retrieve a word, and then calls another routine with that word and updates a field in the structure.
- `manage_linked_list_elements_2`: Calls a routine with parameters, updates a value at offset 22 of a structure in a0, calls another routine to retrieve a value, and then calls a third routine with a pointer and a value derived from a global variable.
- `process_recursive_structure`: Checks a value at an offset in a structure to perform recursive calls and then updates a value in a table based on a returned result.
- `update_table_entries_2_2`: Clears a global variable, pushes a word from a global variable onto the stack, calls a routine, stores the result at offset negative 4 of a5, clears offset negative 8 of a5, and branches to another location.
- `update_table_entries_2`: Iterates through a list, updating a pointer at offset negative 8 of a5, calls two routines to retrieve values, and if a comparison between two words at specific offsets fails, it calls another routine with a pointer and a value.
- `update_table_entries`: Copies a word from offset 4 of a pointer in a5 and a pointer from offset 8 of a5 to the stack, calls another routine twice with these values, calls a third routine with a value from a negative offset from a4, and sets a word at a negative offset from a4 to 1.
- `call_and_store_result_2`: Sets up a stack frame, moves a word from offset 8 of a5 to a negative offset from a4, pushes a word from a negative offset from a4 onto the stack, calls another routine, stores the result in offset -4 of a5, and branches to another routine.
- `branch_on_byte`: Tests a byte at offset 8 of a structure in a5 and branches to another routine if the byte is non-zero, otherwise it loads a word from a constant offset into d0 and branches.
- `update_and_call_structure`: Sets a word at offset 26 of a structure in a0 to zero, and if a word at offset 22 of that structure is 3, performs a series of calls using the structure in a0 as an argument before updating the pointer at offset -4 of a5.
- `execute_conditional_logic_extended`: Clears a global variable, performs conditional routine calls based on a parameter value, and updates global status flags.
- `manage_linked_list_elements_4`: Checks a pointer in a stack frame and, if non-zero, calls another routine to populate it, then checks a word at an offset in the new pointer to conditionally call a third routine.
- `execute_state_transition_2`: Calls another routine using a value from a parameter and then sets a global variable to zero.
- `modify_global_flag`: Sets a global variable to zero, calls another routine multiple times with different parameters, and then sets the global variable to one if a value at offset 8 of the stack frame is non-zero.
- `store_and_branch_on_result`: Stores a long value from d0 into a location at offset 4 of a structure in a5, then branches to another routine if the value is zero, otherwise it calls another routine and branches to a third routine.
- `unlock_address_register_a5`: Unlocks address register a5 and returns immediately.
- `clear_and_call_handler`: Resets a global flag and executes a function through a pointer, potentially clearing another pointer if it is non-zero.
- `check_global_status`: Checks a global variable and, if it is non-zero, branches to another routine; otherwise it stores the value one at a negative offset of a structure and branches to another routine.
- `process_conditional_pointer_3`: Clears two values, checks a word in a stack frame, and if non-zero calls a routine, stores the result in a stack frame, and if the result is non-zero calls another routine.
- `process_conditional_pointer_2`: Calls a routine and, if the result is zero, calls another routine and stores the result in a structure, then conditionally updates a global variable.
- `clear_and_return_word`: Stores a zero word at an offset from a pointer in a4 and returns a word from offset negative 2 of a pointer in a5.
- `execute_state_transition`: Sets a global variable to zero, performs a series of calls using values from a parameter, and updates a table entry and a global variable before returning.
- `clear_and_call`: Clears a word in a global variable, pushes a word from another global variable onto the stack, calls another routine, and then stores the result in a stack frame before branching.
- `process_list_and_transition`: Iterates through a linked list while updating values in a structure, then calls a routine to handle a transition and updates a global variable.
- `traverse_pointer_list`: Pushes the word 3 onto the stack, calls another routine, and returns.
- `return_from_subroutine`: Pushes the word 2 onto the stack, calls another routine, and returns.
- `call_subroutine_and_store_result`: Pushes a word from an offset of a structure in a4 onto the stack, calls another routine, and stores the result in a location at offset negative four of the structure in a5 before branching to another routine.
- `process_pointer_list`: Calls two routines to process a pointer from a structure, updates that pointer with the result, and loops while the pointer is non-zero and a byte at offset 22 of the pointed-to structure does not match a value at offset 8 of the structure in a5.
- `load_and_branch_via_offset`: Loads a pointer from offset eight of the structure in a5, retrieves a word from offset two of that pointer, sign-extends it, and branches to another routine.
- `jump_via_table_4`: Compares d0 to 13, and if d0 is not less than 13, shifts d0 left by one and uses it to index into a table to perform a jump.
- `release_a5_stack`: Returns immediately after unlocking a5.
- `update_table_indices`: Performs a table lookup and then increments and updates two indexed values within a data structure.
- `process_item_flag_state`: Reads a short value from an offset within a passed pointer, processes it through an intermediate routine, and then calls a flag-setting routine with the result.
- `evaluate_status_range`: Checks a bit in a table entry, fetches a value from a pointer, and determines a status code based on comparisons of the fetched value and a word at offset 12 of a structure.
- `set_word_and_branch`: Sets a 16-bit word at offset minus 2 of a5 to 255 and branches to another routine.
- `compare_threshold_values`: Compares a 16-bit word at offset 12 of the structure in a5 to 135 and branches if it is less than or equal to that value, then compares the same word to 225 and branches if it is greater than or equal to that value, otherwise it compares a 16-bit word at offset minus 4 to 4 and branches if it is less than 4, or sets d0 to 2 and branches to another routine.
- `set_word_via_offset_and_branch`: Sets a 16-bit word to negative 255 at an offset of negative 2 from a pointer in a5, then branches to another routine.
- `conditional_clear_and_branch`: Compares a 16-bit word at offset 12 of a5 to 180; if it is less than 180, it clears a 16-bit word at offset minus 2 of a5, otherwise it branches to another routine.
- `store_byte_to_indexed_location`: Stores the value one into a 16-bit word at a memory location calculated by adding an offset to a pointer, provided the word at offset minus two of the structure in a5 is not negative one.
- `calculate_vector_transformation`: Fetches two 16-bit values from a table using indices provided in a structure, scales them by a multiplier, and performs a series of arithmetic operations involving division and sign checks to compute a set of transformed coordinates and flags.
- `scale_vector_components`: Fetches two 16-bit values from a table using indices provided in a structure, scales them by a multiplier stored at offset 38 of the structure, and stores the results at offsets 32 and 34.
- `calculate_vector_intersection`: Calculates the intersection of two vectors by determining coordinate offsets from a table, performing cross-product arithmetic, and storing the resulting vector components and scaling factors into a local structure.
- `clear_stack_value`: Sets a 16-bit value at an offset in the stack frame to zero and then branches.
- `calculate_coordinate_offset`: Clears a 16-bit value, checks a value at an offset in a5, and if it is non-negative, performs a multiplication and division using a value from the stack.
- `calculate_coordinate_offset_and_store`: Calculates a coordinate difference by multiplying two 16-bit values, subtracting a second product of two 16-bit values, and shifting the result left by one, then stores the result in a 16-bit field at offset negative 18 from a5 and branches to another routine.
- `calculate_relative_coordinate_offset`: Calculates a coordinate difference by multiplying two 16-bit values, subtracting a second product of two other 16-bit values, and shifting the result left by one bit.
- `calculate_coordinate_difference`: Multiplies a 16-bit value at offset minus 12 of a5 by 2, subtracts this from a 16-bit value at offset minus 24 of a5, stores the result at offset minus 26 of a5, and if a 16-bit value at offset minus 28 of a5 is not zero, loads a 16-bit value from offset minus 10 of a5 into d0.
- `calculate_bresenham_line_coordinates`: Calculates integer-based line coordinates using a Bresenham-style algorithm by iterating through steps and updating a structure of coordinate values.
- `copy_structured_word_data`: Copies seven words from a source structure into a destination structure by using offsets and multipliers stored within a secondary structure.
- `branch_to_target_routine_2`: Performs a short branch to a specific location within another routine.
- `copy_multiple_words`: The routine copies several words from a source structure to a destination structure using offsets and multipliers.
- `update_node_structure_data`: Fetches two pointers from a structure, retrieves values from tables using those pointers, and updates several 16-bit fields within the structure.
- `update_structure_offsets`: Adds a 16-bit word from offset 34 of a structure in a5 to a 16-bit word at offset 30, adds a 16-bit word from offset 36 to a 16-bit word at offset 34, and if a 16-bit word at offset 22 is non-negative, adds a 16-bit word from offset 28 to a 16-bit word at offset 22 before branching.
- `update_offset_value`: Adds a 16-bit value from offset 20 of a structure in a5 to a 16-bit value at offset 22 of the same structure, then if a 16-bit value at offset 32 is zero, it adds a 16-bit value from offset 38 to a 16-bit value at offset 12 of the structure.
- `update_node_data`: Performs arithmetic on a value and copies several 16-bit words from a source structure to a destination structure.
- `empty_c_function_15`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `extract_and_sign_extend_values`: Sets up a stack frame, calls another routine with a word from a structure in a0, stores the result in a structure at a negative offset from a5, and then extracts and sign-extends three different words from the structure into d0 and stores them at various negative offsets in a5.
- `calculate_signed_difference`: Adds a longword from offset -12 of a structure in a5 to a longword from offset -8 of the same structure, then checks if the result is non-negative.
- `scale_and_negate_value`: Loads a longword from offset 12 of a structure in a5, shifts it left by 1, pushes the result onto the stack, and then checks a longword at offset 8 of the structure to either negate it or branch to another routine.
- `process_scaled_value`: Copies a longword from offset 8 of a structure in a5 to d0 and d1, loads a longword from the stack into d0, calls another routine, stores the result as a 16-bit word at offset 18 of the structure, and if the longword at offset 8 is negative, negates d0 before branching.
- `calculate_scaled_long`: Calls another routine with a longword and a sign-extended 16-bit value, adds a longword from offset 12 of a structure in a5 to the result, stores the result at offset 16 of the structure, and then either shifts that longword left by one bit or shifts it left and negates it based on the sign bit.
- `prepare_signed_long`: Shifts a longword from a structure left by one bit, pushes it to the stack, and then either branches or negates a value from the structure.
- `compare_and_increment`: Compares a longword from offset minus 8 of the structure in a5 with a longword from the stack, and if the stack value is greater than the value in a5, it increments a 16-bit value at offset minus 20 of the structure in a5 and branches to another location; otherwise it branches to another location.
- `execute_math_operations`: Checks if a longword at offset -16 of a structure in a5 is non-zero, then performs a series of four calls to other routines using values from offsets -18, -8, and -16 of the structure, and finally calls another routine with a value pushed onto the stack.
- `calculate_signed_product`: Computes a product of two values and applies sign logic based on the inputs before storing the result into a table, using two calls to other routines to handle specific sign and zero conditions.
- `execute_callback_with_pointer`: Sets up a stack frame, pushes a longword from offset 8 of a5 onto the stack, calls another routine, and if d0 is not zero, pushes the same longword again to call another routine before returning.
- `zero_d0_and_branch`: Sets d0 to zero and branches to another routine.
- `load_byte_from_pointer_2`: Sets up a stack frame, checks if the longword at offset 8 of a5 is non-zero, and if so, loads a byte from the address stored at that offset into d0.
- `compare_d0_to_six`: Compares the value in d0 to 6, and if they are not equal, it returns; otherwise it sets d0 to 1, unlinks a5, and returns.
- `call_indirect_pointer`: Pushes a value from a memory location onto the stack, calls a routine through an indirect pointer, adjusts the stack pointer, clears d0, and branches to another location.
- `copy_words_to_table`: Sets up a stack frame, clears a word at offset -2 of the structure in a5 and clears a word at the location pointed to by offset 16 of the structure, then if the longword at offset 8 of the structure is non-zero and the word at that pointer is non-zero, increments the pointer at offset 8 by 2 and copies a word from that pointer to an offset in the table pointed to by offset 16.
- `copy_word_to_table`: Checks if a pointer at offset 12 of the structure in a5 is non-zero and if the word at that pointer is non-zero, then if a word at offset -2 of the structure is less than 29, increments the pointer at offset 12 by 2, increments the word at offset -2 by 1, and copies a word from the original pointer to a destination table at an offset calculated from the incremented value in d0.
- `unlock_and_return_2`: Clears the d0 register and unlocks the semaphore associated with the address in a5 before returning.
- `branch_to_offset_64`: Performs a short branch to an instruction located 64 bytes ahead in the code segment.
- `clear_bit_at_offset`: Loads a 16-bit word from offset -2 of the structure in a5, extends it to a 32-bit long, shifts it left by 1, loads a pointer from offset 16 of the structure in a5, clears a word at the address in a0 plus the value in d0, sets d0 to 1, and branches to offset 112.
- `store_multiple_pointers`: Obtains three values from another routine and stores them into three different global variables.
- `store_pointer_indirectly`: Stores a value returned by another routine into a global variable and then passes that value as an argument to two subsequent routine calls.
- `store_pointer_indirectly_alt`: Stores a value returned by another routine into a different global variable and then passes that value as an argument to two subsequent routine calls.
- `execute_subroutine_sequence`: Links a stack frame, pushes a pointer from offset 8 of a5 and calls another routine, then pushes a pointer from offset -170 of a5 and calls another routine; it then updates offset 8 of a5 with a new pointer, pushes that pointer onto the stack, and calls another routine.
- `check_ascii_range`: Loads a pointer from offset 8 of a5 into a0 and checks if the byte at that address is between 48 and 57 inclusive; if it is, it clears d0 and branches to another location, otherwise it returns.
- `setup_structure_pointers`: Stores the address of offset -50 of a5 into offset -112 of a5, stores the address of offset -100 of a5 into offset -106 of a5, loads a longword from a pointer at offset -24888 of a4 into offset -120 of a5, and stores the address of offset -112 of a5 into offset -116 of a5.
- `push_and_call_subroutine`: Pushes two longwords from offsets minus 116 and 8 of the structure pointed to by a5 onto the stack, calls another routine, adjusts the stack pointer, and then either sets register d0 to 1 and branches to an earlier instruction or branches to another instruction based on whether register d0 is zero.
- `call_subroutine_and_clear_globals`: Pushes two longwords from offsets minus 116 and 8 of the structure in a5 onto the stack, calls another routine, and if the return value in d0 is not zero, clears two words at offsets minus 24882 and minus 24880 of a4.
- `update_pointer_based_on_byte`: Checks if a byte at a pointer in offset 8 of a5 is between 97 and 122 inclusive; if so, it calculates a new address by adding 52 to a pointer retrieved from a global table and stores this longword into offset -4 of the structure in a5, then branches to another routine; otherwise it branches to another routine.
- `calculate_pointer_offset_2`: Subtracts 97 from a byte loaded from a pointer, multiplies the result by two, and adds it to a value retrieved from a global table to store a new pointer at offset negative 4 of a structure.
- `calculate_and_store_offset_2`: Loads a byte from offset -4 of a5 into d0, adds it to a value derived from a byte at offset 1 of the same address and a pointer at offset -4, stores the result at offset -4 of a5, clears offset -6 of a5, and then compares the byte at offset -4 of a5 to the word at offset -6 of a5.
- `increment_pointer_on_condition`: Loads a byte from the address at offset -4 of a5 into d0 and, if d0 is not greater than the word at offset -6 of a5, increments the address at offset -4 of a5 and loops.

The function list continues in: g2349-recovery-functions-2.md.
