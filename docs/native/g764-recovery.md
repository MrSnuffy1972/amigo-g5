# Game 764 (4th & inches (1988)(accolade)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00764). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| 4th & Inches (1988)(Accolade)[cr QTX].adf:foot1 | 2 | 207 | 8 |
| 4th & Inches (1988)(Accolade)[cr QTX].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- 4th & inches (1988)(accolade)[cr qtx]/4th & Inches (1988)(Accolade)[cr QTX].adf:data/wckick: high-entropy data file (7.71 bits/byte), classified as packed data
- 4th & inches (1988)(accolade)[cr qtx]/4th & Inches (1988)(Accolade)[cr QTX].adf:foot1: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### assign

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_and_branch`: Copies the 32-bit value from d1 into d0, then branches to the middle of another routine to continue its execution.

### copy

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_d1_to_d0`: Copies the value in d1 into d0 and then jumps into the middle of another routine, which continues by finding the maximum value in a linked list of entries and, if a condition is met, writing a small header.

### Ram-Handler

- `call_exec_copy_mem`: Saves registers a1 and a6 on the stack, places the arguments from d1 through d3 into a0, a1 and d0, then calls the Exec library routine at offset -624 in the library base and restores the saved registers.

### ramdrive.device

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `clone_and_initialize_ramdrive_device`: Opens the ramdrive.device and, for each registered RAM drive device that has a name and a device structure, copies the device name and its buffer pointers into a global table, initializes the drive structure, and links it into a global list.
- `clone_ramdrive_device`: Copies a byte array whose length is read from a buffer into a global table, then copies a device structure from a3 into that table and copies another routine's code and data into freshly allocated memory, patching the copied code's jump targets to the new location.
- `initialize_ramdrive_device_2`: Copies a block of longwords from one buffer to another using a counter, then computes a size from a stored value, allocates several buffers via a memory routine, copies two data tables into them, and installs handler pointers into the structure fields.
- `initialize_ramdrive_device`: Initializes the RAM drive device structure by storing pointers to its internal buffers and control fields, then links it into a global list.
- `init_ramdrive_device`: Loads the library base pointer into a6 and calls the RAM drive initialization routine, passing the buffer and control pointers stored in d0.
- `enable_interrupts_and_call_handler`: Enables interrupts by writing to the interrupt enable register and increments a counter at offset 294 of the structure in a6, then walks a linked list of pointers.
- `align_and_enable_interrupts`: Tests d3 and returns immediately when it is zero, otherwise aligns it down to a multiple of eight and subtracts it from a pointer in a3.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `load_zero_and_branch`: Loads zero into d0 and then branches into another routine without returning.
- `store_longword_offset`: Stores the longword passed in d1 into the location at offset 24 of the structure pointed to by a1, then returns.
- `process_device_message`: Saves a2 and a6, loads the library base into a6, and calls the initialization routine.
- `load_253_and_return`: Loads the value 253 into d0 and returns immediately.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 253 into d0, then returns.
- `unlink_entry_and_free_memory`: Increments a byte at offset 295 of the structure in a0 and decrements a counter at offset 34 of the structure in a6, returning early when the counter stays non-negative or a flag at offset 48 is clear.
- `copy_longword_and_compare`: Copies a longword from the entry pointed to by a2 into the location pointed to by a1, then loads a table address and compares it with a3.
- `terminate_string_with_colon`: Copies a longword from the entry pointed to by a3 into the location pointed to by a1, then stores the result of calling a library function at offset 554 of the library base in a6.

### 4th & Inches (1988)(Accolade)[cr QTX].adf:foot1

- `load_and_apply_relocations`: Initializes the system and loads an executable file from disk, then walks the resulting relocation table, applying each fix-up to the loaded code and data, and returns.
- `increment_counter_if_nonzero`: Tests the value in d5 and returns immediately if it is zero, otherwise increments a counter in d6, sets d5 to one, and returns.
- `decrement_counter_loop`: Decrements the counter in d1 by one on each pass and repeats the decrement until the counter reaches zero, otherwise it returns.
- `tiny_ori`: Small routine of 1 instruction that merges 232 into d0.
- `decode_run_length_backwards`: Reads two little-endian words from the stack, combines their bytes into a bit offset and a bit count, and sets d7 to a space character minus the offset.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_cycle_counter`: Decrements a byte counter in d7 for each iteration of a loop controlled by d0, resetting it to a space character whenever it reaches zero, and returns.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### 4th & Inches (1988)(Accolade)[cr QTX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
