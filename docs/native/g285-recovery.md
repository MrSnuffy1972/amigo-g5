# Game 285 (North&South v1.5 2397): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00285). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (937534 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 222 of 222 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 216 of 216 named (0 author symbols), 216 of 216 described (20 by rule, 194 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2928 of 2928 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4560 of 4560 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 209062 |
| code | 14758 |
| data | 713663 |
| padding | 51 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| North&South.Slave | 1 | 343 | 39 |
| Disk.1:bootblock | 1 | 189 | 13 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1000 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 141 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2844 | 124 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 7 |

## Cases handled

- North&South_v1.5_2397/North&South/Disk.1: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- 214 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 472 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 19 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 119 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1900 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- North&South_v1.5_2397/North&South/Disk.1: 214 logged loads cover 255488 bytes of the image
- North&South_v1.5_2397/North&South/Disk.1: blocks 4-10 (bytes 2048-5632) outside the filesystem hold data, not code (opcode density 0.031, entropy 6.37)
- North&South_v1.5_2397/North&South/Disk.1: blocks 12-43 (bytes 6144-22528) outside the filesystem hold data, not code (opcode density 0.015, entropy 4.20)
- North&South_v1.5_2397/North&South/Disk.1: blocks 241-258 (bytes 123392-132608) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.42)
- North&South_v1.5_2397/North&South/Disk.1: blocks 485-496 (bytes 248320-254464) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.47)
- 1 packed track region(s), 610304 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_interrupt_mask` (runtime segment an address): Performs timing delays and modifies hardware registers to update interrupt masks based on a global state. Kind: checksum/integrity trap. Pass path: the routine continues execution normally after the loop finishes without triggering a crash or reset.
- `configure_interrupt_mask` (runtime segment an address): Performs bitwise operations to calculate a mask and writes to hardware registers to enable or disable interrupts. Kind: disk hardware/sync check. Pass path: the routine continues execution after the loop/check completes, likely proceeding to the next stage of the loader if the hardware registers match expected values.

## Functions


### North&South.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `dispatch_based_on_return_value`: Calls a routine via a global pointer and branches to one of two different indirect calls depending on the returned value.
- `modify_jumptable_values`: Checks specific values within a table and overwrites them with a constant if they match a certain criteria, then calls a function from a jump table.
- `execute_sequence_of_calls`: Stores a value in a global variable and executes a sequence of five indirect calls using offsets from that value and other global pointers.
- `execute_indirect_jump`: Performs an indirect jump using a value from a jump table.
- `execute_indirect_jump_2`: Executes a jump to an address stored in a jump table.
- `mask_global_variable`: Returns a global variable after applying a bitwise AND mask.
- `read_interrupt_request_register`: Reads the interrupt request register.
- `store_interrupt_request`: Stores a value into a global variable after reading the interrupt request register.
- `return_zero_constant`: The routine immediately returns zero without performing any operations or modifying data.
- `set_flag_in_struct`: The routine receives a pointer in a register and sets bit 1 in the byte at offset 14 of the pointed-to structure.
- `clear_d0_register_2`: This routine performs a logical operation to set the d0 register to zero.
- `copy_list_elements`: Iterates through a list of data, copying elements from one structure to another while calling a function for each entry.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `find_node_by_string`: Iterates through a linked list and returns a pointer to a node if a string stored within the node matches a provided input string.
- `call_subroutine_and_return`: Calls a single unnamed subroutine with no arguments and returns when that call finishes, forwarding control without additional logic.
- `store_register_to_global`: The routine copies the value from D0 into a global variable.
- `call_indirect_via_global`: The routine copies a global variable to a register and performs an indirect call to that address.
- `copy_byte_to_offset`: Stores a byte from one memory location into another location at a fixed offset.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_global_constant`: Compares a global variable to a specific constant value.
- `initialize_pointer_chain`: The routine writes a sentinel value into a pointer and stores the address of the next entry into three fields in two different structures.
- `copy_nested_pointers`: Copies 32-bit values from pointers referenced at offsets four and eight of a source structure into fields of a destination structure.
- `check_collision_flags`: XORs a global state word with a value from a linked list node and checks for specific bit flags.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 8 to a7, then returns.
- `copy_word_if_valid`: Checks if two pointer arguments are non-null.
- `compute_and_store_byte_logic`: Calls a function pointer from a global structure and combines its result with register values using bitwise logic.
- `negate_nonzero_flag`: The routine receives a value in a register and returns zero if the value is non-zero, and returns a negative one if the value is zero.
- `calculate_table_offset`: Reads values from a global table and performs arithmetic operations including division, multiplication, and addition.
- `update_status_byte`: The routine checks various bits in a structure and updates a byte at a specific offset based on those conditions and a bitmask.
- `toggle_byte_flag`: Flips the top bit of the byte at offset 65 in the structure addressed by A3.
- `set_flag_from_bit`: Reads a bit from a global variable using an index derived from a byte in another memory location.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
- `toggle_bit_and_increment`: The routine checks a bit in a global variable, clears it, and if it was set, modifies a byte in a structure and increments a value at an offset.
- `clear_d0_register_3`: This routine clears the contents of a data register by merging zero into it.
- `clear_d0_register`: Merges zero into the d0 register.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `empty_routine`: This routine contains no executable instructions.

### Disk.1:bootblock

- `call_loader_routine`: This routine performs a tail call to a routine that loads and relocates data chunks into memory.
- `load_and_relocate_hunks`: Allocates memory and processes a sequence of data chunks by applying relocation offsets and loading executable code based on AmigaDOS hunk types.
- `process_hunk_data_stream`: Iterates through a buffer to handle various AmigaDOS hunk types, performing memory relocations and data copying as specified by the hunk codes.
- `fill_memory_with_hunks`: Reads data from a buffer and populates a destination memory area by processing specific AmigaDOS hunk type codes and relocation instructions.
- `process_hunk_data`: Iterates through a buffer and performs different operations based on AmigaDOS hunk type codes, such as loading data, performing relocations, or incrementing pointers.
- `process_hunk_types`: The routine iterates through data and performs different operations based on values that match AmigaDOS hunk type codes.
- `update_io_buffer`: Updates several fields in a structure and calls a routine to perform I/O operations.
- `read_next_buffered_long`: Calls a helper, decrements a remaining-count field, and returns the next 32-bit value from a buffer while advancing the buffer pointer.
- `call_helper_repeatedly`: Calls a helper routine a specific number of times based on the value of its argument until the count reaches zero.
- `fill_memory_from_buffer`: Reads a sequence of 32-bit values from a buffer and stores them sequentially into a destination memory location.
- `call_free_mem`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `call_do_i_o`: Wrapper that calls the exec library function DoIO and returns its result to the caller.
- `call_alloc_mem`: Wrapper that calls the exec library function AllocMem and returns its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `execute_unreached_code`: A twelve instruction routine located in an unreached code gap.
- `return_d0_value`: Returns the value currently held in the d0 register.
- `return_bitwise_or_value`: Returns the value in the d0 register bitwise ORed with a constant.
- `execute_unreached_code_2`: A single instruction routine located in an unreached code gap.
- `bitwise_or_register_value`: Performs a bitwise OR operation between a byte from a data register and a constant value, then returns the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_byte_change`: Calls another routine and then enters loops that wait for a specific byte at a memory location to change from a specific value to zero and then to non-zero.
- `copy_data_blocks`: The routine performs a nested loop to copy blocks of data from a source to a destination.
- `copy_memory_block`: Performs nested loops to copy data between memory locations using calculated offsets and increments.
- `apply_descending_scaling`: Iterates through a table of sixteen words, applying a descending multiplier to specific bitfields within each word and passing the resulting table to another routine.
- `apply_ascending_scaling`: Iterates through a table of sixteen words, applying an ascending multiplier to specific bitfields within each word and passing the resulting table to another routine.
- `update_global_pointers`: Fetches a pointer from a global variable and stores the pointer and its subsequent data elements into several global locations.
- `fetch_and_pass_struct_members`: Retrieves a pointer from a table using a constant index, then extracts and stores three specific members from the pointed-to structure into local memory before passing two of those members as arguments to another routine.
- `pass_pointer_to_routine`: Passes a pointer stored in a global location to another routine.
- `extract_data_structure`: Retrieves a pointer from a global variable and copies several consecutive data elements from that pointer into a set of global variables.
- `initialize_scrolling_data`: Loads a data structure from memory, populates a stack with a null-terminated string, and initializes several pointers and scaling parameters.
- `fetch_and_scale_data`: Fetches several values from a table via indirect calls, performs arithmetic on a pointer, and stores the results into various global variables.
- `reverse_string_into_stack`: Reads a null-terminated string and stores its characters in reverse order into a destination buffer.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 97 into d2, then loads 0 into d2.
- `initialize_media_buffer`: Executes a sequence of indirect function calls to allocate memory, copy data, and populate a buffer through a loop.
- `load_and_initialize_media_assets`: Loads multiple data files into memory and initializes global pointers and scaling parameters for their playback or display.
- `search_byte_array`: Compares a single byte against a sequence of twenty-seven bytes in a table and returns one if a match is found.
- `advance_byte_stream_pointer`: Calls a helper with a fixed argument, then advances a byte-stream pointer by one and wraps it back to a fixed start when the next byte is a negative terminator.
- `search_word_array`: Compares a single word against a sequence of seventeen words in a table and returns one if a match is found.
- `update_counters_and_draw`: Iterates five times to call a search routine and a drawing routine, while incrementing and wrapping two separate counter variables.
- `process_scrolling_data_stream`: Iterates through a byte stream to update a scrolling window and manage data pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `generate_random_quotient_remainder`: Advances a 32-bit seed using a timing value from another routine and a constant, then returns the seed's quotient and remainder by a divisor taken from the argument, packed into the two halves of the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `register_callback`: Passes the address of itself to another routine.
- `register_callback_alternative`: Passes the address of itself to a different routine.
- `poll_mouse_and_update_pointer`: Reads the joystick and mouse port counters and button registers, updates a stored pointer position from the movement deltas with clamping to the 320 by 200 screen, and writes the new position into a display-list word.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `store_word_one`: Stores the value one into a memory location pointed to by a register.
- `setup_function_pointer`: Stores a pointer and a constant value into a structure and then calls a routine using a pointer from that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_raster_then_swap_pointers`: Waits for the blitter to go idle and for the raster position to pass a fixed scanline, then swaps two saved pointers in the data area and calls a helper.
- `swap_table_words_recursively`: Swaps two consecutive 32-bit values in a table and then calls itself recursively.
- `copy_table_word_data`: This routine performs a 32-bit data transfer from a calculated offset within a table to a base address located in the same table.
- `copy_word_to_table`: Copies a 32-bit value from a calculated offset in a table to a base address in the same table.
- `insert_and_reorder_table_entry`: Inserts a new data entry into a structured table and performs a reordering operation on associated index arrays.
- `store_two_words`: Stores two consecutive words from a parameter into a memory location calculated from a register.
- `calculate_accumulated_offset`: Iterates through a buffer of bytes to calculate a sum based on values retrieved from a table using offsets derived from the buffer contents.
- `perform_blitter_operation`: Configures and executes a blitter transfer by writing to various blitter control and dimension registers.
- `decompress_bitstream_data`: Expands a compressed data stream by reading bits from a bitstream to determine jump offsets or to trigger the reconstruction of values from a lookup table.
- `expand_compressed_data`: Decompresses a data stream using a bitstream-based algorithm.
- `generate_bitstream_from_table`: Iterates through a data structure to perform bitwise manipulations and shifts, constructing a sequence of values stored in a destination buffer.
- `update_dma_hardware_parameters`: Updates DMA controller and hardware control registers by reading values from a table and a global structure.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `search_three_word_record_list`: Iterates through a list of three-word records until a zero-word is encountered or a match is found for the value in a data register.
- `store_byte_to_memory`: Stores a byte from a register into two consecutive memory locations pointed to by a value in another register.
- `update_dma_parameters`: Updates DMA controller registers and hardware control registers by reading values from a table and a global structure.
- `configure_dma_channels`: Configures Amiga DMA controller registers by writing values from a table to the DMA control and address registers.
- `update_state_structure`: Modifies various fields within a structure, including incrementing counters and updating pointers based on specific bitwise flags.
- `update_entity_state_and_animation`: Updates entity status flags and increments internal counters, potentially triggering a state transition or animation frame change.
- `clear_dma_control_register`: The routine writes zero to the DMACON hardware register and sets a global variable to a bit mask derived from an input value.
- `parse_sprite_attribute_data`: Iterates through a data stream to extract sprite attributes and animation frame indices.
- `parse_entity_data_stream`: Iterates through a data stream to extract entity attributes and sprite metadata.
- `parse_entity_data_stream_2`: Iterates through a data stream to extract entity properties and animation frame information.
- `process_entity_animation_data`: Iterates through a data structure to extract animation frame indices and sprite properties.
- `convert_int_to_ascii`: Converts a numeric value into a null-terminated ASCII string and stores it at a destination offset.
- `convert_integer_to_string`: Converts a numeric value into a string of ASCII characters by repeatedly calculating remainders and quotients.
- `copy_data_with_offset`: Copies bytes from one memory location to another using an index and an offset, then stores a zero byte at a calculated destination.
- `zero_byte_at_offset`: Stores a zero byte into a memory location calculated from a register and an offset.
- `traverse_linked_list`: Iterates through a linked list of pointers and executes a function pointer found at a specific offset within the list elements.
- `execute_callback_function`: Invokes a function pointer stored at a specific offset within each element of a linked list during iteration.
- `set_four_plane_pointers`: Writes a base address into four per-plane pointer slots, each 8000 bytes further along than the last, splitting each 32-bit address into high and low words.
- `initialize_table_data`: Writes a 32-bit value into a table using two words and then populates subsequent entries with a repeating pattern of two words.
- `execute_callback_list`: Iterates through a list of seven pointers and executes each non-zero function pointer found.
- `search_and_clear_pointer`: Iterates through a list of seven pointers and sets a pointer to zero if its contents match the provided parameter.
- `search_and_clear_list_entry`: Iterates through a list of pointers until a value matches the contents of a pointer, then sets that pointer to zero.
- `release_memory_resources`: Calls functions via pointers stored in a global variable and updates a global pointer with a value from a table.
- `initialize_and_call_function`: Fills a memory area with zeros, clears a global variable, and then executes a function pointer retrieved from memory.
- `execute_indirect_calls`: Executes several functions through pointers stored at offsets from a global variable and returns a comparison result.
- `update_interrupt_mask`: Performs timing delays and modifies hardware registers to update interrupt masks based on a global state.
- `configure_interrupt_mask`: Performs bitwise operations to calculate a mask and writes to hardware registers to enable or disable interrupts.
- `update_interrupt_enable`: Sets a global variable to the value of a register with the highest bit enabled.
- `wait_for_bit_clear`: Loops until a specific bit in a byte located at an offset from a pointer is cleared.
- `write_word_to_offset`: Stores a 16-bit constant into a memory location at a fixed offset from a pointer.
- `set_blitter_parameter`: Writes a 16-bit constant value to a specific offset within a memory block.
- `swap_memory_blocks`: Swaps bytes between two memory locations within a nested loop structure based on provided dimensions.
- `and_mask_column_four_planes`: ANDs a 16-bit mask into a column of words across four bitplanes, stepping down the rows with a 40-byte pitch to clear the masked bits.
- `write_two_words`: Stores two consecutive 16-bit values into memory at a fixed offset from a pointer.
- `wait_blitter_idle_keep_value`: Spins while the blitter busy flag is set, then returns its data register unchanged.
- `compare_strings_with_limit`: Compares two strings up to a specified length and returns the difference between the first non-matching bytes.
- `compare_case_insensitive_strings`: Compares two strings by converting characters to a common case and returns the difference between the first non-matching bytes.
- `search_string_table`: Iterates through a table of strings, performing case-insensitive comparisons until a match is found or a limit is reached.
- `fetch_table_entry`: Retrieves a 32-bit value from a table using an index and a base pointer.
- `fetch_and_store_table_value`: Retrieves a 32-bit value from a table using an index and a base pointer, then stores the result into a memory location.
- `fetch_table_value`: Retrieves a 32-bit value from a table based on a search result, returning zero if the search fails.
- `update_structure_and_call`: Checks a value in a structure, calls a function with parameters from that structure, and then resets specific fields.
- `update_table_value`: Updates a value in a table using one of three modes: direct assignment, addition, or addition using a value from another table.
- `update_cached_value`: Compares an input value against a stored global value and calls another routine to update the cache if they differ.
- `copy_memory_block_2`: Copies a specified number of bytes from a source buffer to a destination buffer.
- `update_object_offset_and_bounds`: Calculates and updates an offset within a table structure by recursively processing data in fixed-size increments.
- `nop_padding_gap`: This routine is empty and consists of a single instruction that serves as padding within the executable segment.
- `update_string_table`: Searches for a string in a table and, if not found, attempts to register a new entry in a small buffer.
- `process_structure_elements`: Checks specific values within an array of structures and performs conditional updates to a global data area and other structures.
- `setup_and_call_function`: Populates a structure with four values, calls a function via a pointer, and returns a boolean based on a byte in that structure.
- `update_structure_fields`: Stores four parameters into a structure and then calls a function pointer using a pointer from that structure.
- `update_structure_and_call_function`: Sets a four-byte value to zero and a two-byte value to nine within a structure, copies a four-byte value from a global variable into another field of that structure, and then executes a function via a pointer located in a separate memory area using the structure's base address.
- `update_structure_and_call_function_2`: Writes a constant value and a value from a global variable into specific offsets of a structure, then executes a function pointer located within another structure.
- `set_and_get_value`: Writes a value to a structure field, calls a function pointer, and returns a value from a different structure field.
- `release_active_records`: Checks multiple pointers and, if non-zero, calls various routines to clear or reset data associated with those pointers.
- `initialize_device_records`: Queries a device for specific values, updates global pointers, and sets a flag based on the success of an indirect function call.
- `perform_signed_division`: Determines the sign of the result based on input registers, calls a routine to perform unsigned 32-bit division, and negates the quotient if the sign flag indicates a negative result.
- `calculate_division_remainder`: Performs integer division and calculates the remainder using bitwise shifts and arithmetic loops.
- `perform_arithmetic_loop`: Executes a series of bitwise shifts and arithmetic operations within nested loops based on input values.
- `execute_nested_loops`: Performs a series of bitwise shifts and arithmetic operations within nested loops.
- `copy_string_with_limit`: Copies a string from a source to a destination buffer up to a maximum number of characters.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `manage_object_properties`: Executes a sequence of library calls and indirect functions to populate a structure with provided parameters and pointers.
- `reset_record_and_release_resources`: Releases a record's sub-block when one is present, marks the record's state as inactive and clears its counter field.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `find_and_update_record`: Searches for a record using a pointer and an index, then updates the record's byte, word, and long fields if a match is found.
- `invalidate_record_data`: Sets a byte and two long fields in a record to specific constant values and calls another routine with a value from the record.
- `call_indirect_function`: Executes a function via an indirect call through a pointer stored in a table.
- `call_indirect_function_alt`: Executes a function via an indirect call through a different offset in a table.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_indirect_function_third`: Executes a function via an indirect call through a third offset in a table.
- `call_indirect_function_fourth`: Executes a function via an indirect call through a fourth offset in a table.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `execute_indirect_call`: Performs an indirect call to a function pointer retrieved through multiple levels of memory indirection.
- `skip_non_zero_byte_sequence`: Checks if a byte at a specific offset from a pointer is a specific value, then enters two loops to skip a sequence of identical bytes, first skipping all non-zero bytes and then skipping all zero bytes.
- `process_nibble_multiplication`: Iterates through a table of sixteen words, performing bitwise multiplication and masking operations on each element before passing the resulting array to another routine.
- `initialize_and_update_graphics_parameters`: Loads several values from a data table into global variables and uses them to configure hardware parameters or display settings.
- `update_global_value`: Calls a function via a pointer, modifies a global variable by adding a constant and a returned value, and returns a calculated quotient and remainder.
- `swap_data_pointers`: Waits for a bit in a global variable to clear and for a value to reach a threshold, then swaps two 32-bit values in memory.
- `swap_table_words_recursively_2`: Swaps two consecutive 32-bit values in a table and then calls itself recursively.
- `update_table_offsets`: Calculates a value based on a table lookup and then iterates through a list to add an offset to each entry.
- `copy_and_reorder_data`: Copies a block of data from a table to a local buffer and then performs a reordering operation on a table entry.
- `insert_and_reorder_entry`: Calls another routine to retrieve a value, then executes a sequence of calls to reorder data within a structure.
- `reorder_table_index`: Performs a series of calls to manipulate a value and a pointer within a structured table.
- `apply_bitmask_to_memory`: Iterates through a list of bytes to perform bitwise operations on multiple memory locations using values from a table.
- `initialize_bit_array`: Updates several global pointers and values, then populates a four-word array with bits derived from a provided integer.
- `execute_blitter_operation`: Configures and executes a blitter transfer by writing to various blitter control and dimension registers.
- `fill_memory_with_values`: Fills two arrays with incrementing values in a loop and then calls another routine.
- `setup_blitter_parameters`: Writes several values to specific memory locations and calls another routine multiple times.
- `configure_blitter_registers`: Sets multiple memory locations with specific values and calls another routine multiple times.
- `write_byte_to_dual_locations`: This routine stores a single byte from a register into two consecutive memory locations using a pointer held in another register.
- `process_bitfield_data`: Iterates through bits of a 64-bit value to update various memory locations and registers.
- `update_global_state`: Writes several values to global memory locations and executes a function through a pointer stored in a global structure.
- `copy_data_to_hardware`: Copies sixteen words from a source pointer to both a memory location and a hardware register address.
- `insert_into_linked_list`: Updates a function pointer in a global variable and inserts a new node into a linked list.
- `remove_from_linked_list`: Traverses a linked list to find a specific node and removes it by updating pointers.
- `initialize_system_state`: Sets multiple global variables, calls several function pointers, and populates a series of memory locations with specific values.
- `fill_callback_slot`: Iterates through a list of seven pointers and stores a value in the first pointer that contains zero.
- `update_blitter_graphics_parameters`: Calculates dimensions and memory offsets based on table lookups and global values, then updates a series of memory locations used to configure blitter operations.
- `intersect_and_mask_rectangles`: Calculates the intersection of two rectangular coordinate sets and performs a bitwise AND operation on the overlapping memory region.
- `compare_string_case_insensitive`: Compares two strings by converting characters to a common case and returns a boolean result based on their equality.
- `move_memory_block`: Shifts a block of memory from a source to a destination by copying bytes forward or backward depending on the relative order of the pointers to prevent data corruption.
- `calculate_32bit_product_sum`: Performs a series of multiplications and additions using the high and low words of two 32-bit registers.
- `execute_indirect_call_offset`: Performs an indirect call to a function located at a specific offset from a pointer stored in a global variable.
- `execute_indirect_pointer_chain`: Performs a multi-level pointer dereference to call a function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `nop_instruction_placeholder`: This routine is empty and performs no operations.
- `do_nothing_2`: the routine is empty and performs no operations before returning.
- `return_no_operation`: The routine consists of a single instruction that returns immediately, with no body and no side effects.
- `do_nothing`: this routine is empty and performs no operations.
- `return_immediately_2`: The routine is empty and performs no operations before returning to its caller.
- `empty_stub_2`: An empty stub routine that immediately returns to the caller.
