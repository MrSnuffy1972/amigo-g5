# Game 404 (Maniac Mansion): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00404). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802240 top-level bytes in 120 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 24 of 24 named (0 author symbols), 24 of 24 described (6 by rule, 16 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 83 of 83 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 631 of 631 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1554306 |
| code | 1892 |
| data | 211812 |
| padding | 32694 |
| unused | 1536 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SPN | 3 | 133 | 7 |
| Cls | 2 | 20 | 1 |
| Wait | 2 | 117 | 2 |
| Maniac Mansion (1988)(LucasFilm Games)(Disk 1 of 2).adf:maniac.zip | 2 | 233 | 8 |
| text | 2 | 24 | 4 |
| Maniac Mansion (1988)(LucasFilm Games)(Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| Maniac Mansion (1988)(LucasFilm Games)(Disk 2 of 2).adf:bootblock | 1 | 94 | 1 |

## Cases handled

- maniac mansion (1988)(lucasfilm games)(disk 1 of 2)/Maniac Mansion (1988)(LucasFilm Games)(Disk 1 of 2).adf:maniac.zip: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### SPN

- `call_six_sequential`: Calls six routines in sequence via subroutine branches, passing a different input to each, and does not return from this routine itself.
- `trim_trailing_spaces`: Trims trailing spaces from a string by walking backward from the position given by the first input and overwriting the last non-space byte with a zero terminator.
- `open_libraries_and_copy`: Opens dos.library and intuition.library through the library vector table, storing each handle in a global variable, and returns if either open fails or the second input equals eight.
- `allocate_and_lock`: Allocates memory through the exec.library, locks a directory through the dos.library, and verifies a field of the returned structure.
- `open_file_and_close`: Opens a directory through the dos.library, opens a file within it, and closes the directory.
- `write_and_signal`: Calls the dos.library write function to output a block of data, then branches on the second input to copy one of two fixed byte tables into a buffer at offset 54.
- `release_libraries`: Checks the second input and the fourth input, then calls two library functions through pointers stored in a global variable.

### Cls

- `open_dos_library`: Opens the dos.library through the library vector table and stores the returned base pointer in a local variable.

### Wait

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_and_branch_forward`: Copies the 32-bit value from register d1 into register d0, then branches to the routine located 260 bytes ahead.

### Maniac Mansion (1988)(LucasFilm Games)(Disk 1 of 2).adf:maniac.zip

- `process_and_relocate_records`: Parses a data stream of tagged records, copying the payload of each record into a destination buffer and rewriting the record tags, then calls another routine to relocate the copied data.
- `decrement_and_compare`: Decrements a counter in register d1 in a loop until it reaches zero, then compares the original value against the decimal 1010 and branches if they are equal.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `shift_and_store_bytes`: Reads a 32-bit value from the address in a0, then repeatedly shifts it right by eight bits while copying bytes from that address downward into a stack buffer until the pointer reaches the saved value in a2.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `rotate_64bit_value`: Rotates a 64-bit value held across two registers right by a variable amount, pulling in new bytes from a stack array as needed, and accumulates the shifted bits into an output register.
- `trap_and_or`: Loads a word from a memory location into a register, raises trap 6, and combines a register with a logical or into another register before falling through into the following routine.

### text

- `call_library_entry`: Calls a routine and checks the zero flag.
- `open_dos_library_2`: Calls the exec.library OldOpenLibrary function to open dos.library and stores the returned handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Maniac Mansion (1988)(LucasFilm Games)(Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Maniac Mansion (1988)(LucasFilm Games)(Disk 2 of 2).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
