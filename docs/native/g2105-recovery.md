# Game 2105 (ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02105). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 48 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 38 of 38 named (0 author symbols), 38 of 38 described (4 by rule, 14 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1503 of 1503 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 110582 |
| code | 4192 |
| data | 761793 |
| padding | 12777 |
| unused | 11776 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:installu6 | 2 | 267 | 8 |
| endcli | 2 | 117 | 2 |
| Disk-Validator | 1 | 386 | 1 |
| Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:shut.txt | 2 | 267 | 8 |
| Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:shut.exe | 1 | 86 | 5 |
| Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:intro | 1 | 103 | 5 |
| Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:game | 2 | 267 | 8 |
| Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:bootblock | 1 | 10 | 1 |

## Cases handled

- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:INTRO_3.NEW: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:intro.new: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:mainmenu.new: high-entropy data file (7.64 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:paper.bmp: high-entropy data file (7.93 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:ULTIMA_Ba.MOD: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:INTRO_1.NEW: high-entropy data file (7.85 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:ULTIMA_Ga.MOD: high-entropy data file (7.80 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:ULTIMA_Fa.MOD: high-entropy data file (7.91 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:u: high-entropy data file (7.76 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:ultcli: high-entropy data file (7.68 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:intro_1a.new: high-entropy data file (7.79 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:ULTIMA_Da.MOD: high-entropy data file (7.84 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:intro_2.new: high-entropy data file (7.65 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:gypsy.new: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:Ultima6: high-entropy data file (7.68 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:ULTIMA_Aa.MOD: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:vellum1.new: high-entropy data file (7.83 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:intro_2a.new: high-entropy data file (7.90 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:title.new: high-entropy data file (7.77 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:ULTIMA_Ea.MOD: high-entropy data file (7.72 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:intro_1b.new: high-entropy data file (7.75 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:blocks.new: high-entropy data file (7.90 bits/byte), classified as packed data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:installu6: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ultima vi - the false prophet (1992)(mindscape)(disk 1 of 3)(install)[h panther]/Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:game: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:intro

## Copy protection found

- none found

## Functions


### Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:installu6

- `load_hunk_file_sections`: Parses AmigaDOS hunk records and allocates memory for each code, data and BSS section, copying section contents into it.
- `countdown_until_zero_loop`: Decrements a 16-bit register in a loop until it has counted past zero, then returns, so it only consumes time.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_backward_run_stream`: Decodes a packed stream into memory from the end toward the start, copying literal bytes when a flag is clear and copying runs of earlier bytes when it is set.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bit_buffer_loop`: Repeatedly shifts a multi-longword bit buffer by a count taken from the low bits of a value, refilling it from memory when it runs empty and carrying each outgoing bit into a flag.
- `trigger_software_interrupt`: executes a software interrupt and increments the value pointed to by a register by eight.

### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If the output pointer argument is non-null, copies sixteen longwords from a fixed offset in a base register into the memory it points to, then calls a function pointer held in a register.

### Disk-Validator

- `check_disk_validation_entry`: Program entry of a disk validation file, whose pseudo-code compares a stored count with a threshold to choose between a single character and a call through a function pointer, and then computes a value stored into a structure.

### Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:shut.txt

- `unpack_relocatable_data_stream`: Decodes a compressed data stream into memory by processing runs of bytes and literal values.
- `decrement_word_to_zero`: Decrements a 16-bit value in a register until it reaches zero using a loop.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `copy_memory_blocks`: Copies data between memory locations using variable lengths and offsets determined by a control value.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `enter_bit_decode_loop_compare`: Entered by a call from one unnamed routine, it compares two words and then runs a bit-buffer loop that shifts bits from a backward buffer into an accumulator.
- `decode_bit_stream`: Processes a bit stream by shifting values through a buffer and performing conditional operations based on bit flags and a counter.
- `execute_trap`: Triggers a processor trap instruction.

### Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:shut.exe

- `store_global_pointers_then_call`: Program entry that stores the address of a global variable into four fixed memory locations, then calls a routine that the decompiler could not name and one unnamed routine.
- `decode_backward_run_entry`: Entry to a backward run decoder that reads a header longword from its input and, when the header is nonzero, first calls one routine to adjust the count.
- `stub_immediate_return`: A test of a register value that returns in both branches, so it does no work.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `shift_bits_with_refill_loop`: Shifts a longword bit buffer right once per iteration for a count taken from a register, refilling it from memory with the top bit set when it empties.

### Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:intro

- `clear_memory_and_open_graphics`: Program entry that clears a block of longwords in memory, opens the graphics library by name through the exec library's OldOpenLibrary function, and then calls two unnamed routines.
- `write_constant_sequence_after_poll`: A routine decoded from a gap in the file that polls a global flag, calls two unnamed routines and an exec library function, and then writes a long sequence of constant values to a global location.
- `copy_four_bytes_from_table`: Copies four consecutive bytes from a table in its own data, starting at an offset scaled from a register, into the memory addressed by an address register.
- `poll_until_flag_negative`: Calls one unnamed routine repeatedly until a global value becomes negative, then returns.
- `measure_string_between_library_calls`: Makes one library call at a fixed negative offset, counts the bytes of a zero-terminated string stored in a structure, and then makes a second library call at another offset.

### Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:game

- `unpack_relocatable_executable_stream`: Decodes a compressed data stream into memory by interpreting specific control codes that represent data literals or runs of previously decoded bytes.
- `decrement_register_loop`: Decrements a value in a register until it reaches zero.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning to the caller.
- `copy_memory_blocks_2`: Copies data between memory locations using variable lengths and offsets, updating a global variable with a pointer value upon completion.
- `stub_shift_test_return`: Shifts a value by a count taken from a register and tests the result, but returns in both branches, so it does no work.
- `shift_bit_buffer`: Extracts bits from a multi-longword buffer by shifting and refills the buffer from a memory pointer when empty.
- `decode_bit_stream_2`: Processes a bit stream by shifting bits from a buffer and writing values to memory, potentially refilling the buffer from a pointer.
- `execute_trap_2`: Triggers a software interrupt via a trap instruction.

### Ultima VI - The False Prophet (1992)(Mindscape)(Disk 1 of 3)(Install)[h Panther].Adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
