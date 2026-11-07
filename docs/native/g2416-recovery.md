# Game 2416 (conqueror (1989)(rainbow arts)(de-en)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02416). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 18 of 18 named (0 author symbols), 18 of 18 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 37 of 37 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 430 of 430 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 32032 |
| code | 1290 |
| data | 338683 |
| padding | 1755 |
| unused | 527360 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX].adf:CONQUEROR | 2 | 137 | 9 |
| type | 2 | 117 | 2 |
| Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX].adf:intro | 2 | 83 | 3 |
| Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX].adf:game | 2 | 83 | 3 |
| Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- conqueror (1989)(rainbow arts)(de-en)[cr qtx]/Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX].adf:intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX].adf:intro

## Copy protection found

- none found

## Functions


### Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX].adf:CONQUEROR

- `open_dos_and_configure_hardware`: Program entry that opens the dos library and writes output through dos.Write.
- `handle_pointer_entry_stub`: A four-instruction handler reached by absolute address through a pointer in data.
- `depack_stream_using_bit_reader`: Depacker routine that decodes a packed stream by calling a bit-reader refill routine and a bit-read routine, with two further unnamed calls.
- `dispatch_to_unnamed_routine`: A three-instruction entry reached by absolute address that makes one call to an unnamed routine.
- `read_packed_bit_field`: Reads a field of bits from the packed stream through the bit-reader routine and makes two further unnamed calls.
- `set_background_colour_from_bits`: Reads bits from the packed stream through the bit-reader routine and touches the background colour register, with two further unnamed calls.
- `write_countdown_to_colour_register`: Counts a 16-bit value down from minus one through every value until it wraps back to minus one, writing each value to a global location on each step.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_and_call_handler`: Copies sixteen longwords from a table into a stack-supplied buffer when that pointer is non-null, then calls through a stored code pointer.

### Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX].adf:intro

- `intro_bytekiller_depack`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX].adf:game

- `intro_bytekiller_depack_2`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `shift_longword_before_pointer`: Reads the longword four bytes before its input address, shifts it right by one bit, sets the top bit and returns the result.
- `refill_shift_bit_buffer`: Shifts a bit buffer right by one bit per call.

### Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
