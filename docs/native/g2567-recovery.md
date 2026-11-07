# Game 2567 (qwak (1993)(team 17)[cr atx][t +14 atx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02567). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901504 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 45 of 45 named (0 author symbols), 45 of 45 described (0 by rule, 45 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 144 of 144 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 4608 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 816 of 1169 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 22267 |
| code | 4230 |
| data | 865792 |
| padding | 511 |
| unused | 8704 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf:bootblock | 1 | 50 | 4 |
| logged disk load from Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf boot-block load | 1 | 891 | 31 |
| logged disk load from Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf boot-block load | 1 | 121 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 107 | 6 |
| Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf bytes 12288-16896 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- qwak (1993)(team 17)[cr atx][t +14 atx]/Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf: custom-format (non-DOS) disk, 1736 non-empty blocks: matched against track-loaded segments
- qwak (1993)(team 17)[cr atx][t +14 atx]/Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf: the boot block reads 11264 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- qwak (1993)(team 17)[cr atx][t +14 atx]/Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf: the boot block reads 15360 bytes from disk offset 788480 to an address (found statically); recovered as a unit
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 33 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- qwak (1993)(team 17)[cr atx][t +14 atx]/Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf: bytes 12288-16896 look like code no run loaded: recovered position-independently (load address unknown)
- qwak (1993)(team 17)[cr atx][t +14 atx]/Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf: 3 logged loads cover 31232 bytes of the image
- qwak (1993)(team 17)[cr atx][t +14 atx]/Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf: blocks 34-213 (bytes 17408-109568) outside the filesystem hold data, not code (opcode density 0.040, entropy 5.23)
- qwak (1993)(team 17)[cr atx][t +14 atx]/Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf: blocks 227-679 (bytes 116224-348160) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.75)
- qwak (1993)(team 17)[cr atx][t +14 atx]/Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf: blocks 682-1200 (bytes 349184-614912) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.64)
- qwak (1993)(team 17)[cr atx][t +14 atx]/Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf: blocks 1202-1539 (bytes 615424-788480) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.68)
- qwak (1993)(team 17)[cr atx][t +14 atx]/Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf: blocks 1570-1759 (bytes 803840-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.85)
- 1 never-loaded region(s), 4608 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `poll_flag_and_set_timer` (Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf boot-block load +an address to an address (11264 bytes)): Polls a byte in a structure for a specific bit flag, writing three constant values to a global variable in sequence, and returns zero or non-zero depending on whether the flag is set or clear. Kind: Disk hardware (DSKLEN) check. Pass path: bne.b an address.

## Functions


### Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf:bootblock

- `bootblock_init_system`: Restores the system by re-enabling interrupts and DMA, then configures a structure with several fixed constants and invokes a routine via a stored function pointer.
- `check_threshold_and_call_pointer`: It checks a signed 16-bit value stored at a fixed offset inside a global structure and, when that value exceeds a threshold, jumps through a function pointer held at another fixed offset to invoke it.
- `init_struct_and_call_routine_2`: It calls a routine, writes several fixed constants into fields of a structure passed in a register, invokes a function pointer stored in another register, and sets a global variable to a constant.
- `init_struct_and_call_routine`: It writes three fixed 32-bit constants into fields of a structure passed in a register, then calls a function pointer stored inside a global structure and finally invokes a fixed library entry point.

### logged disk load from Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf boot-block load

- `call_memory_setup_routine`: Invokes a routine stored in a global variable twice, then masks the returned value with a base address and stores it back into that global variable before setting up further global values.
- `disable_interrupts_and_dma`: Writes a value into the interrupt-enable custom register to disable interrupts, then writes a value into the DMA-enable custom register to turn off DMA channels, and finally stores a value into a global variable.
- `lookup_and_copy_table`: Looks up a value in a table of ten-item records and, when it finds a match with a nonzero flag, copies a variable number of bytes from one area to another.
- `store_split_result`: Calls a routine that splits a value and loops over a table, storing the result into a global variable before returning.
- `split_value_and_loop_helper`: It splits a 16-bit quantity into two parts using division and remainder by nine, packs them into a single register, then loops over a table calling a helper routine while adjusting indices based on a stride and a remaining count.
- `convert_grid_coordinates`: Splits a 16-bit column offset into a base-12 quotient and remainder, doubling the quotient when the row index is not the first one.
- `split_accumulate_combine`: Splits a passed value by nine, packs the parts into a register, loops over a table accumulating a counter, then reverses the split and returns a combined value.
- `init_and_poll_timer`: Clears several structure fields, then conditionally writes constants into custom chip registers and loops polling a flag while updating timer registers.
- `poll_flag_with_retry`: Loops up to ten times calling a flag-polling routine and a value-extraction routine, returning the extracted value when the flag is set and decrementing a counter, or returning a constant when the counter expires.
- `loop_check_checksum`: Loops incrementing a counter, repeatedly checking a status bit and a table entry, and returns early on various conditions while computing checksums and setting timer registers.
- `empty_stub_routine`: It performs no observable work and simply returns.
- `set_flag_and_increment_counter`: It sets a bit in a 16-bit word located a few bytes before a pointer in A6, using a bit index derived from D3.
- `clear_table_entries`: It clears a table by writing zero to successive entries.
- `extract_high_byte_pair`: It calls another routine to obtain a 32-bit value and reassembles it so the low 16 bits come from the result and the high 8 bits come from the original high byte.
- `interleave_word_bits`: It reads two consecutive words from a pointer and returns a value formed from their low bits.
- `apply_interleave_xor_mask`: Reads two consecutive words from a pointer and combines their low bits into a 32-bit value, then passes that value to a routine that XORs nine consecutive 32-bit words starting at a global variable with an accumulator and keeps only the even bits.
- `xor_even_bits_of_words`: XORs nine consecutive 32-bit words starting at a global variable with an accumulator, then keeps only the even bits of the result.
- `compute_xor_checksum`: It iterates through a sequence of words pointed to by an input pointer, XORing each into an accumulator.
- `interleave_word_bits_2`: It interleaves the low and high bits of two 32-bit word streams pointed to by two input pointers.
- `set_audio_dma_control`: It writes two fixed constants into global variables, stores a pointer into a custom chip register, and writes a value with the high bit set into another global variable.
- `poll_flag_and_set_timer`: Polls a byte in a structure for a specific bit flag, writing three constant values to a global variable in sequence, and returns zero or non-zero depending on whether the flag is set or clear.
- `clear_bit_in_global_flag`: Stores a fixed constant into a global variable, then, when a signed value at a negative offset is negative, clears a bit selected from a shifted index in that global variable.
- `set_bit_in_global_flag`: It adds a constant to a 16-bit value fetched from a table through a register, reduces the result to a 5-bit index, and sets the corresponding bit in a global variable by OR-ing the derived mask into it.
- `wait_for_flag_clear`: Stores a fixed constant into a global variable, invokes a routine, then loops until a flag bit clears and a returned value equals one before returning.
- `update_table_difference`: Reads a word from a table indexed by a register, negates it when negative, computes the absolute difference against a passed value, and writes the result back to the table.
- `wait_flag_clear_table`: Polls a global flag bit in a loop up to a fixed count, clearing a table entry indexed by a register value when the flag is clear, and returning a constant when the count expires.
- `read_and_poll_input_port`: Reads a byte from a hardware input port and clears one bit when a looked-up table entry indicates the line is active, then clears the lowest bit and stores the result in a global variable.
- `store_input_port_byte`: Reads a byte from a hardware input port and stores it directly into a global variable before returning.
- `build_port_byte_from_table`: Builds a byte from a frame field and a bit index, clearing one further bit when a looked-up table entry has its lowest bit set, and returns that byte together with a data register value.
- `configure_ciab_timer`: When a global flag's lowest bit is clear and a passed value is not one, it writes three constant values to a global variable in sequence.
- `set_ciab_timer`: It writes three constant values to a global variable in sequence.

### logged disk load from Qwak (1993)(Team 17)[cr ATX][t +14 ATX].adf boot-block load

- `copy_and_shift_buffer`: Copies a block of words from a source table into a destination table.
- `select_half_or_input_word`: It returns half of a second word when a flag equals one, otherwise it returns the input word unchanged.
- `and_table_word_with_register`: Multiplies a passed value by two, indexes a table at that offset, and returns the fetched word ANDed with a register value.
- `store_flag_and_or_byte`: It stores a negated comparison of two flag bits into a byte through a pointer, then ORs a register value into a byte pointed to by another pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_and_return_reg`: Calls two routines in sequence and returns the value originally held in a register.
- `call_conditional_copy`: Performs three sequential calls to another routine, then checks a byte at a fixed offset in a structure and calls another routine when that byte is non-zero.
- `copy_words_between_pointers_2`: When a pointed-to byte is non-zero and two pointer arguments differ, it copies word-sized data from a source pointer to a destination pointer until the source reaches an end pointer.
- `copy_words_between_pointers`: It copies word-sized data from a source pointer to a destination pointer until the source reaches an end pointer.
- `initialize_system_resources`: Clears a memory block of 32-bit words, invokes a function pointer from a structure, writes several constants into global variables, and calls additional function pointers stored in structures.
- `invoke_structured_callbacks`: It copies a value from a structure field, invokes several function pointers stored in that structure, sets up a global variable with a derived address, and writes constants into custom chip registers and global variables.
