# Game 3 (4th & Inches): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00003). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 24 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 31 of 31 named (0 author symbols), 31 of 31 described (7 by rule, 23 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 81 of 81 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 806 of 806 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 93583 |
| code | 2288 |
| data | 101974 |
| padding | 5931 |
| unused | 697344 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| assign | 2 | 117 | 2 |
| copy | 2 | 117 | 2 |
| Ram-Handler | 2 | 10 | 1 |
| ramdrive.device | 3 | 345 | 17 |
| 4th & Inches.adf:foot1 | 2 | 207 | 8 |
| 4th & Inches.adf:bootblock | 1 | 10 | 1 |

## Cases handled

- 4th & Inches/4th & Inches.adf:data/wckick: high-entropy data file (7.71 bits/byte), classified as packed data
- 4th & Inches/4th & Inches.adf:foot1: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### assign

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_d0_to_d1_and_branch`: Copies the 32-bit value from d0 to d1 and then branches to a location 104 bytes ahead within the same routine.

### copy

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then writes that value into a header at offset 50 of a structure, fills the following words with an increasing sequence starting at 3, and sets up a stack frame for a call.
- `transfer_and_branch`: Copies the value in d1 into d0 and then branches into the middle of another routine, which continues by walking a linked list to find the maximum value in a field and, when a condition is met, writes a small header.

### Ram-Handler

- `call_copymem_routine`: Saves registers a1 and a6 on the stack, moves the arguments from d1 and d2 into a0 and a1, copies d3 into d0, locates the exec library via a global variable, and calls its CopyMem routine before restoring the saved registers and returning.

### ramdrive.device

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `initialize_ramdrive_device_3`: Opens the ramdrive.device, walks its device list to find a matching entry, then initializes a ram drive structure by copying its buffers and clamping its size to 16 words before calling another routine to finish setup.
- `initialize_ramdrive_device_2_2`: Copies a length-prefixed byte block from a source buffer into a table, then reads a size from a structure, clamps it to 16, and copies that many words into another table.
- `initialize_ramdrive_device_2`: Copies a block of longwords from one buffer to another, then computes a table index and a byte offset from a global counter, allocates several memory blocks, and copies two lookup tables into them before filling in device structure fields and calling the dispatch routine.
- `initialize_ramdrive_device`: Initializes a device structure by storing pointers to internal buffers and control fields, then links it into a global list.
- `call_ramdrive_dispatch`: Loads the current library base pointer into a1, stores the decimal constant 624 into d0, and calls the library dispatch routine, then repeats the same steps with the constant 150.
- `process_timer_interrupts`: Enables interrupts by writing to the interrupt enable register and increments a counter at offset 294 of the structure in a6, then walks a linked list of entries, skipping those whose mask does not match the value in d1 and selecting the one whose timer value is greater than or equal to the value in d0, and finally calls the handler stored at offset -204 of the structure in a6.
- `adjust_and_call_interrupts`: Checks whether d3 is zero and otherwise subtracts d0 from it, masks the result to a multiple of 32, adds it to the pointer in a3, and calls the routine at that adjusted address.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_zero_and_branch`: Loads the decimal constant zero into d0 and then branches to the target routine, which it does not contain.
- `store_longword_offset`: Stores a longword from register d1 into the structure pointed to by a1 at offset 24, then returns.
- `process_device_message`: Saves a2 and a6, loads the library base into a6, and calls the initialization routine.
- `jump_and_return_value`: Jumps forward to a nearby routine and returns the value it produces, which is the negative of one.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 253 into d0, then returns.
- `unlink_node_and_scan`: Increments the byte at offset 295 of the structure in a0 and decrements the counter at offset 34 of the library base, returning early when that counter stays non-negative or when a flag at offset 48 is clear.
- `copy_entry_and_lookup`: Copies the longword stored at offset 4 of the structure pointed to by a2 into the location pointed to by a1, then loads the address of a string table entry at offset 546.
- `terminate_string_with_colon`: Copies a longword from the entry pointed to by a3 into the location pointed to by a1, then stores the result of calling a library function at offset 554 of the library base in a6.

### 4th & Inches.adf:foot1

- `initialize_and_load_hunks`: Initializes the Amiga environment by allocating memory and locating the exec library, then iterates over a table of hunk type codes copying data blocks and applying relocations, and finally invokes the entry point before freeing the allocated memory.
- `increment_counter_if_nonzero`: Tests the value in d5 and returns immediately if it is zero, otherwise increments a counter in d6, sets d5 to one, and returns.
- `decrement_loop_counter`: Decrements a loop counter in d1 and executes the loop body once for each non-zero count, then returns.
- `tiny_ori`: Small routine of 1 instruction that merges 232 into d0.
- `decode_run_length_backwards`: Reads two little-endian words from the stack, combines their bytes into a bit offset and a bit count, and sets d7 to a space character minus the offset.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_cycle_counter`: Decrements a byte counter in d7 for each iteration of a loop controlled by d0, resetting it to a space character whenever it reaches zero, and returns.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### 4th & Inches.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
