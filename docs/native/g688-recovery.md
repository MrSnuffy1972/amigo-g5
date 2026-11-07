# Game 688 (Archer Maclean's Pool): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00688). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (979296 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 31 of 31 named (0 author symbols), 31 of 31 described (0 by rule, 29 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 21812 of 21812 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 23513 of 23513 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4983 |
| code | 78364 |
| data | 2449 |
| padding | 60 |
| unused | 893440 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Archer Maclean's Pool.adf:bootblock | 1 | 50 | 1 |
| logged disk load from Archer Maclean's Pool.adf boot-block load | 1 | 94 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 305 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23064 | 24 |

## Cases handled

- Archer Maclean's Pool/Archer Maclean's Pool.adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 164 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 21648 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Archer Maclean's Pool/Archer Maclean's Pool.adf: 1 logged loads cover 5632 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Archer Maclean's Pool.adf:bootblock

- `initialize_boot_block`: The routine sets up a message structure in memory, issues an I/O request via a library routine, waits for completion, then clears a global variable and repeatedly configures the same message with different values before issuing further I/O requests in a loop until a flag byte is set.

### logged disk load from Archer Maclean's Pool.adf boot-block load

- `disable_interrupts`: The routine clears the interrupt enable register, which disables interrupts.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_display_text_and_tiles`: The routine updates a text buffer and a tile map in memory.
- `analyze_memory_segment`: The routine consists of 35 instructions and appears to do nothing observable from its disassembly alone; it contains no library calls and accesses no registers in a way that reveals a purpose.
- `integer_sqrt_approximation`: The routine computes an integer square root approximation for an unsigned input value using a bit-length estimate followed by a binary search-like iteration.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `propagate_bitmask_fields`: The routine copies bits from a four-byte source pointed to by a pointer, using a set of bit indices derived from registers, into three output bytes pointed to by a base address plus an offset.
- `propagate_collision_bits`: The routine reads a small bitfield from a source byte pointer and, using a set of bit indices supplied in registers, sets corresponding bits in two bytes located at offsets eight and nine from a base pointer, and continues the same pattern into further bytes.
- `propagate_bitmask_bytes`: The routine reads a four-byte source block via a pointer and tests individual bits selected by several registers.
- `copy_collision_bitmap`: The routine copies bits from a source bitfield in a0 to target bytes at a1+7, a1+8 and a1+9 by testing individual bits selected by d0-d7 and setting corresponding bits in the destination bytes.
- `propagate_collision_bits_2`: The routine reads a small bitfield from a byte pointer and, using a set of bit indices supplied in registers, conditionally sets bits in two bytes located at offsets seven and eight from a base pointer, and continues the same pattern into further bytes.
- `propagate_bitmask_bytes_2`: The routine reads bits from three consecutive bytes pointed to by a pointer and, using several input bit positions, sets corresponding bits in three bytes located at offsets six, seven and eight from a base pointer.
- `copy_bitfield_to_bytes`: The routine tests individual bits in four consecutive bytes pointed to by a pointer and sets corresponding bits in three bytes at offsets six, seven and eight from a base pointer.
- `propagate_bitmask_bytes_3`: The routine copies bits from three bytes pointed to by a pointer using a set of bit indices supplied in registers, and ORs the corresponding bits into three consecutive bytes pointed to by another pointer.
- `map_bitfield_to_bytes`: The routine reads bits from four consecutive bytes pointed to by a pointer and, depending on the values of several registers, sets corresponding bits in three bytes located at offsets five, six and seven from a base pointer.
- `propagate_bitmask_flags`: The routine reads a four-byte bitmask from a pointer and, using several input registers as bit selectors, sets corresponding bits in three bytes located at offsets five, six and seven from a base pointer.
- `copy_bitfield_to_bytes_2`: The routine copies bits selected by eight bit indices from four consecutive bytes pointed to by a pointer into three consecutive bytes pointed to by an integer offset.
- `map_bitfield_to_bytes_2`: The routine copies bits from four source bytes pointed to by a pointer, using bit positions derived from registers, into three destination bytes pointed to by an integer offset.
- `map_collision_bits`: The routine reads a source byte stream through a pointer and, based on a set of bit tests driven by several registers, sets individual bits in three consecutive bytes pointed to by another pointer.
- `propagate_bitmask_to_bytes`: The routine copies bits from four bytes pointed to by a pointer into three bytes at an offset from a pointer.
- `propagate_bitmask_bytes_4`: The routine copies bits from three bytes pointed to by a pointer using a set of bit indices supplied in registers, and ORs the corresponding bits into three destination bytes located at offsets 3, 4, 5 and 6 from a base pointer.
- `copy_bits_to_bytes`: The routine copies bits from four consecutive bytes pointed to by a pointer using bit positions derived from registers, and sets corresponding bits in three consecutive bytes pointed to by another pointer.
- `propagate_bitmask_to_bytes_2`: The routine reads bits from four consecutive bytes pointed to by a pointer and, using several input bit positions, conditionally sets bits in four consecutive bytes pointed to by an integer base.
- `copy_bits_to_output_bytes`: The routine copies bits from four input bytes pointed to by a pointer, using bit positions supplied in registers, into four output bytes pointed to by an integer pointer.
- `copy_bitfield_to_bytes_3`: The routine copies bits selected by a set of bit indices from four consecutive bytes pointed to by a pointer into three consecutive bytes pointed to by an integer pointer, setting destination bits according to the source bits.
- `propagate_input_bitmask`: The routine copies bits from four consecutive bytes pointed to by a pointer using offsets derived from several registers, setting corresponding bits in four consecutive bytes pointed to by an index.
- `copy_bitplane_mask`: The routine copies bits from a four-byte source pointed to by a pointer, using bit positions supplied in registers, into four consecutive bytes pointed to by an integer pointer by setting individual bits with OR operations.
- `propagate_bitmask_bytes_5`: The routine reads a source byte stream via a pointer and, using a set of bit positions supplied in registers, conditionally sets bits in three destination bytes pointed to by an offset.
- `map_bitfield_to_bitfield`: The routine copies bits from a 32-bit source area pointed to by a pointer into a 32-bit destination area pointed to by another pointer, using two input bit indices and six additional bit indices as a permutation.
- `map_bitfield_to_bytes_3`: The routine maps bits from four input bytes pointed to by a0 into four output bytes pointed to by a1 using a fixed permutation defined by d0, d1 and d2-d7.
