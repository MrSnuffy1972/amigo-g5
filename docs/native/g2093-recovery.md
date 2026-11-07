# Game 2093 (settlers, the (1993)(blue byte)[cr flt](disk 1 of 3)(intro)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02093). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (910432 top-level bytes in 40 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 57 of 57 named (0 author symbols), 57 of 57 described (3 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 553 of 553 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 69041 bytes, 4 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2117 of 2136 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 732068 |
| code | 6558 |
| data | 162097 |
| padding | 9709 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:Install | 8 | 45 | 2 |
| Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:mcp | 5 | 253 | 7 |
| run | 2 | 117 | 2 |
| endcli | 2 | 117 | 2 |
| Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:Introecho | 1 | 45 | 2 |
| Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:Loader | 1 | 43 | 3 |
| Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1236 | 34 |
| Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:PIB1 (raw code file, load address unknown) | 1 | 0 | 0 |
| Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:DISP (raw code file, load address unknown) | 1 | 0 | 0 |
| Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:SOUN (raw code file, load address unknown) | 1 | 0 | 0 |
| Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:USER (raw code file, load address unknown) | 1 | 0 | 0 |
| Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:EFEC (raw code file, load address unknown) | 1 | 0 | 0 |
| Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:SD00 (raw code file, load address unknown) | 1 | 0 | 0 |
| Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:FUNC (raw code file, load address unknown) | 1 | 270 | 4 |

## Cases handled

- runtime segment an address: 318 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- settlers, the (1993)(blue byte)[cr flt](disk 1 of 3)(intro)[a]/Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:PIB1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- settlers, the (1993)(blue byte)[cr flt](disk 1 of 3)(intro)[a]/Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:DISP: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- settlers, the (1993)(blue byte)[cr flt](disk 1 of 3)(intro)[a]/Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:SOUN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- settlers, the (1993)(blue byte)[cr flt](disk 1 of 3)(intro)[a]/Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:USER: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- settlers, the (1993)(blue byte)[cr flt](disk 1 of 3)(intro)[a]/Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:EFEC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- settlers, the (1993)(blue byte)[cr flt](disk 1 of 3)(intro)[a]/Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:SD00: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- settlers, the (1993)(blue byte)[cr flt](disk 1 of 3)(intro)[a]/Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:FUNC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- settlers, the (1993)(blue byte)[cr flt](disk 1 of 3)(intro)[a]/Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:Install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- settlers, the (1993)(blue byte)[cr flt](disk 1 of 3)(intro)[a]/Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:mcp: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 6 never-loaded region(s), 68291 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:Install

- `initialise_system_memory`: Calls several routines, then runs a three-pass loop that invokes a library function through a global base.
- `decrement_until_minus_one`: Decrements a local register in a tight loop until it reaches negative one, then returns.

### Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:mcp

- `init_and_check_pattern`: Calls several routines and runs a loop three times, invoking a library function through a global base.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.
- `decode_stream_after_cache_clear`: Calls the cache-clear library function, then runs a two-pass loop that calls a decoder routine and applies its decoded copy operations to a buffer, adding values into the destination.
- `decode_backward_copy_runs`: Reads variable-length codes from a bit stream held in global bytes and copies runs of earlier output backward into the destination, bounded by a count.
- `decode_backward_copy_bytes`: Reads variable-length codes from a bit stream held in global bytes and copies byte runs backward into the destination, using a loop bounded by a count.
- `decode_quad_byte_back_copies`: Reads length and distance codes from a bit stream, then copies groups of four bytes from earlier output into the destination while moving backward through the source.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_longs_then_call_handler`: If the destination pointer is non-null, copies sixteen longwords from a table at a fixed offset into it.

### endcli

- `build_table_from_node_sizes`: Walks a linked list to find the largest size field, writes a table of increasing words sized to it, and stores a stack-relative pointer with a sentinel.
- `copy_memory_block`: Copies a sequence of words from a source address to a destination address based on a counter.

### Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:Introecho

- `process_intro_task_messages`: Finds the current task and, when a field in it is zero, toggles a flag, stores a console window specification pointer and calls library functions.
- `dispatch_indexed_handlers_then_release`: Looks up a shared handle through a library call and, when one is present, reads a count and calls a handler indexed by that count before releasing the handle.

### Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:Loader

- `load_named_program_segments`: Opens the DOS and exec libraries and loads named program segments through the DOS loader, which the text references show as two segment names.
- `stub_bad_instruction_data`: The decompiler reports only bad instruction data here, so this is a stub with no recovered behaviour.
- `empty_routine`: This routine contains no valid instructions.

### Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `parse_arguments_and_setup_display`: Opens graphics and intuition display resources, then reads a space-separated argument string, converting letters to upper case into a global buffer.
- `setup_copper_and_supervisor_state`: Reads a display setting from a global, stores copied configuration words and a list pointer into a per-task table, and calls library functions including one that enters supervisor state.
- `traverse_linked_list`: Walks a linked list of records when a count field in the owner is nonzero.
- `allocate_two_blocks_or_fail`: Allocates two memory blocks through library calls and stores their pointers in the record.
- `tiny_move`: Small routine of 1 instruction that copies a3 to d7.
- `dispatch_character_by_table`: Compares the current input byte against a table of entries, each holding a character and a relative handler offset.
- `lookup_value_for_key`: Searches a table of key and value pairs, ending at a minus-one key, for the key in the data register.
- `return_register_pair_unchanged`: Returns its two register inputs unchanged and does no other work.
- `initialise_memory_pool_headers`: Clears a two-word header and, for each non-empty memory region, links a free-list node spanning the region minus a sixteen-byte node header.
- `decrement_entry_countdown_by_key`: Searches a linked list for the entry matching the key in the data register and decrements its countdown field.
- `return_fixed_constant_value`: Returns the fixed value ten thousand ninety-six and does nothing else.
- `reset_two_state_words`: Sets one state word to zero and the word beside it to three, then returns zero.
- `decrement_record_countdown`: Traverses a linked list to find a specific entry and decrements a short value stored at an offset within that entry.
- `nop_padding`: This routine is empty and performs no operations.
- `decrement_countdown_and_release`: Decrements a countdown word in the record.
- `find_largest_list_size`: With no argument it runs a routine when a count field is set and returns the result of another routine.
- `run_two_routines_in_sequence`: Calls two routines in turn and returns, with no other work.
- `find_largest_list_size_2`: Walks a linked list of nodes, tracking the largest size value in each node's size field, then returns a field of the head record.
- `stub_immediate_return`: Does nothing and returns at once.
- `allocate_rounded_memory_block`: Rounds the requested size up to a multiple of eight bytes and rejects requests below a minimum size.
- `stub_immediate_return_2`: Its decompiled body is only a return, although the facts list colour and beam register references, so it is a stub with no recovered effect.
- `update_list_node_offsets`: Checks a request against the record at the head of a list and, depending on comparison results, adjusts the offset and counter fields of that record.
- `insert_block_into_free_list`: Rounds the block size up to a multiple of eight, selects the list head matching the address and inserts the block in address order into a free list.
- `carve_block_from_free_list`: Rounds the requested size up to a multiple of eight and walks a free list for a node containing the requested address range.
- `seek_node_at_or_after`: Walks a sorted linked list of nodes and stops at the first node whose address is at or beyond the value in the data register.
- `fill_memory_with_blitter`: Fills a memory range with the value in a data register.
- `copy_block_by_size_bits`: Copies a block by testing the low bits of the count, so that each set bit triggers one transfer of one, two, four or eight bytes in turn, followed by larger chunks.
- `dispatch_by_node_flag`: Tests bit zero of a flag byte in the node and calls one of two routines depending on the result, with no other logic.
- `remove_linked_item`: Removes an item from a linked list by updating the head and neighbour pointers.
- `copy_block_handling_overlap`: Copies a block of bytes, copying backward when the destination overlaps the end of the source and forward otherwise, and returns the count in the data register.
- `search_node_tree_for_key`: Searches a binary tree of nodes, following left or right links by comparing the key in the data register with each node's key.
- `return_stack_argument_value`: Returns the eight-byte value passed on the stack without any other work.
- `locate_node_covering_key`: Walks a list of nodes looking for one whose key range covers the key in the data register.
- `search_node_list_for_key`: Walks a node list in one direction, then follows child links while comparing each node with the key in the data register.

### Settlers, The (1993)(Blue Byte)[cr FLT](Disk 1 of 3)(Intro)[a].adf:FUNC (raw code file, load address unknown)

- `initialise_record_fields`: Stores the argument byte, ORed with a fixed mask, into the byte before the record, then clears one word and sets the next word to three.
- `store_value_into_record_fields`: Rejects zero and the most negative short value by setting the first record word to one and returning minus one.
- `format_number_as_decimal_digits`: Converts a number into decimal characters by dividing it by a table of place values and writing one character per place into the buffer.
- `divide_long_by_word`: Divides a 32-bit value in the data register by a 16-bit divisor in two steps, returning the 32-bit quotient.
