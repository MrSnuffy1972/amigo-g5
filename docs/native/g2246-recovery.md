# Game 2246 (eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02246). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3666368 top-level bytes in 311 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 28 of 28 units identical (bytes and relocations), 15 raw-word lines |
| Functions named and described | pass, 724 of 724 named (0 author symbols), 724 of 724 described (34 by rule, 684 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3983 of 3984 (99.97%) |
| Recovered without load address (counted in proof 1, flagged) | 79786 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 21644 of 21655 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2566437 |
| code | 67438 |
| data | 798393 |
| padding | 70772 |
| unused | 163328 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 1 of 4).adf:install | 3 | 7444 | 264 |
| Disk-Validator | 1 | 386 | 1 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 1 of 4).adf:eobii | 55 | 255 | 7 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 1 of 4).adf:diamonds | 1 | 1242 | 36 |
| 6 | 1 | 2 | 1 |
| 8 | 1 | 2 | 1 |
| diskfont.library | 15 | 1040 | 75 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 1 of 4).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 422 | 24 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 656 | 33 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7405 | 202 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2787 | 76 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 2 of 4).adf:CHARGEN2.CPS (raw code file, load address unknown) | 1 | 0 | 0 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL3.SAM (raw code file, load address unknown) | 1 | 0 | 0 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL7A.SAM (raw code file, load address unknown) | 1 | 0 | 0 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL8.SAM (raw code file, load address unknown) | 1 | 0 | 0 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL5.INF#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL11.INF#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL3.INF#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL1.INF#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL6.INF#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL12.INF#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL15.INF#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL7.INF#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL9.INF#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL8.INF#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 1 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 1 of 4).adf:DECORATE.CPS: BK: ByteKiller-packed, unpacked with ancient (7004 -> 40000 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 1 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 1 of 4).adf:MENU.CPS: high-entropy data file (7.66 bits/byte), classified as packed data
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 1 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 1 of 4).adf:TEXT.CPS: BK: ByteKiller-packed, unpacked with ancient (13876 -> 22872 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 1 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 1 of 4).adf:INVENT.CPS: BK: ByteKiller-packed, unpacked with ancient (4152 -> 40000 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 2 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 2 of 4).adf:TEXT2.CPS: BK: ByteKiller-packed, unpacked with ancient (13876 -> 22872 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 2 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 2 of 4).adf:INVENT2.CPS: BK: ByteKiller-packed, unpacked with ancient (4152 -> 40000 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 2 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 2 of 4).adf:WESTWOOD3.CPS: BK: ByteKiller-packed, unpacked with ancient (10560 -> 40000 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 2 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 2 of 4).adf:PANFLUTE5.CPS: high-entropy data file (7.61 bits/byte), classified as packed data
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 2 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 2 of 4).adf:CHARGENB.CPS: BK: ByteKiller-packed, unpacked with ancient (12712 -> 40000 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 2 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 2 of 4).adf:LEVEL4.INF: BK: ByteKiller-packed, unpacked with ancient (1440 -> 2456 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 2 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 2 of 4).adf:WESTWOOD1.CPS: BK: ByteKiller-packed, unpacked with ancient (10336 -> 40000 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL13.INF: BK: ByteKiller-packed, unpacked with ancient (3516 -> 5738 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL5.INF: BK: ByteKiller-packed, unpacked with ancient (2512 -> 4236 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL11.INF: BK: ByteKiller-packed, unpacked with ancient (5108 -> 8806 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL3.INF: BK: ByteKiller-packed, unpacked with ancient (4208 -> 7001 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL1.INF: BK: ByteKiller-packed, unpacked with ancient (4528 -> 7557 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:INVENT3.CPS: BK: ByteKiller-packed, unpacked with ancient (4152 -> 40000 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL14.INF: BK: ByteKiller-packed, unpacked with ancient (4180 -> 7642 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL6.INF: BK: ByteKiller-packed, unpacked with ancient (5524 -> 9142 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL12.INF: BK: ByteKiller-packed, unpacked with ancient (5076 -> 8775 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL10.INF: BK: ByteKiller-packed, unpacked with ancient (828 -> 1103 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL2.INF: BK: ByteKiller-packed, unpacked with ancient (4764 -> 8657 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 4 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL15.INF: BK: ByteKiller-packed, unpacked with ancient (4336 -> 7450 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 4 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL7.INF: BK: ByteKiller-packed, unpacked with ancient (5316 -> 9524 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 4 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:CREDITS2.CPS: BK: ByteKiller-packed, unpacked with ancient (12288 -> 40000 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 4 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL9.INF: BK: ByteKiller-packed, unpacked with ancient (3996 -> 6790 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 4 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL16.INF: BK: ByteKiller-packed, unpacked with ancient (1700 -> 3277 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 4 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL8.INF: BK: ByteKiller-packed, unpacked with ancient (4884 -> 8633 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 4 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:text4.cps: BK: ByteKiller-packed, unpacked with ancient (13876 -> 22872 bytes)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 4 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:INVENT4.CPS: BK: ByteKiller-packed, unpacked with ancient (4152 -> 40000 bytes)
- runtime segment an address: 300 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 382 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2229 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 55 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 2 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 2 of 4).adf:CHARGEN2.CPS: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL3.SAM: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 4 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL7A.SAM: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 4 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL8.SAM: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL5.INF#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL11.INF#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL3.INF#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL1.INF#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL6.INF#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf:LEVEL12.INF#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 4 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL15.INF#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 4 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL7.INF#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 4 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL9.INF#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 4 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf:LEVEL8.INF#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 2 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 2 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 3 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 3 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 4 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 4 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- eye of the beholder ii - advanced dungeons& dragons (1992)(ssi)(de)[cr d&r](disk 1 of 4)/Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 1 of 4).adf:eobii: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 14 never-loaded region(s), 79786 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 1 of 4).adf:install

- `initialize_installation_environment_2`: Clears a memory buffer and initializes several global pointers and structures.
- `invoke_routines_from_tables`: It calls a routine stored in a global pointer when that pointer is non-null, then invokes several routines from tables based on the state of specific fields.
- `traverse_and_clear_linked_list`: Iterates through a linked list by following pointers, invoking a function pointer located at a fixed offset within each node.
- `tokenize_input_string`: Parses a character buffer into an array of up to thirty-two pointers by splitting the string at whitespace or using double quotes to group characters.
- `parse_command_line_arguments`: Parses a string of command-line arguments, handling both quoted strings and space-delimited tokens.
- `parse_quoted_string_table`: Iterates through a table of strings, skipping whitespace and extracting tokens either enclosed in double quotes or delimited by whitespace.
- `loop_until_non_zero_return`: Repeatedly calls another routine with a specific constant value, continuing to call a second routine with a different constant if the first routine returns zero.
- `nop_padding_sequence_2`: This routine is empty and consists of three instructions that serve as padding within the executable.
- `verify_installation_integrity`: Checks available system memory and verifies the presence of required game files and fonts.
- `pass_name_after_last_colon`: Scans a string to find the last colon character and passes the substring following it, along with the original string, to another routine.
- `execute_conditional_installation`: Performs a series of conditional checks and function pointer executions based on the results of nested routine calls.
- `execute_conditional_function`: Fetches a pointer from a global structure and calls a function at an offset, then conditionally calls another function at a different offset based on the first result.
- `call_with_scaled_offset`: Divides a 16-bit argument by eight, multiplies the result by nine, and passes it with the other arguments to another routine.
- `forward_three_arguments`: Passes its three arguments unchanged to another routine and returns.
- `render_recursive_text_3`: This routine scans a string for a carriage return and calls several subroutines to perform graphics operations, recursing if a carriage return is found.
- `render_recursive_text_2`: Performs graphics operations using a pointer from a global structure and recursively calls itself if a carriage return character is encountered in a string.
- `render_recursive_text`: Iterates through a string to find a terminator, performs multiple graphics operations using a pointer from a global structure, and recursively calls itself if a carriage return is found.
- `copy_string_to_buffer_2`: Finds the terminating zero byte of a destination buffer and copies bytes from a source pointer into that location until a zero byte is reached.
- `copy_structure_data`: Copies a sequence of words from a source structure to a destination structure after verifying a pointer via a function call.
- `process_flagged_list`: Iterates through a linked list of structures and calls another routine if specific bits in a status byte are set and a calculated difference between two integer fields is non-zero.
- `multiply_fixed_point`: Calculates the product of two 32-bit fixed-point values using their high and low words.
- `free_two_pointers`: Calls a library function to free memory using a pointer stored in a global variable and then calls it again using a second pointer.
- `free_pointer_list`: Iterates through a list of four pointers and calls a routine to free each non-null pointer, then calls a routine to free a global pointer.
- `conditionally_free_memory`: Checks if a passed pointer is not null and if a certain offset within that pointer contains a non-zero value, then calls a library function to free memory.
- `free_memory_from_pointer`: Checks if a passed pointer is non-null, and if so, calls a library function to free memory using a pointer stored in a global variable.
- `call_free_mem_if_valid`: Checks if a pointer passed as an argument is non-null and, if so, calls the memory deallocation library routine using an offset from a global pointer.
- `traverse_and_free_recursive`: Recursively traverses a pointer-based structure, checking a bitmask in a byte at an offset, and calls a deallocation routine while decrementing a counter in a structure.
- `recursively_decrement_and_deallocate`: This routine recursively traverses a linked structure, checking a bitmask in a byte at a specific offset to decide whether to continue the traversal via a pointer.
- `free_structure_recursively`: This routine recursively processes a pointer, decrements a value at a specific offset within that structure, and calls several routines from a global table.
- `free_memory_conditionally`: Checks if a pointer is non-null to call a deallocation routine, then conditionally decrements a value and calls two other routines.
- `conditionally_free_memory_2`: Calls another routine with a pointer, decrements a value at a specific offset within that pointer if it is non-zero, and then calls two different routines using a global pointer.
- `traverse_and_free_lists`: Recursively traverses two linked lists stored in a global structure and calls a deallocation routine on each node.
- `decrement_and_clear_tables_2`: This routine decrements values in a structure through nested loops and clears two bytes in several structures based on a table.
- `process_complex_data_structure`: Performs nested loops to manipulate values in a data structure, updates a table of pointers, and clears specific bytes in another structure before calling another routine.
- `decrement_and_clear_tables`: Decrements values in a structure through nested loops and clears two bytes in several structures based on a table.
- `decrement_and_clear_table`: Decrements values in a structure through a loop, updates a table of pointers, and clears two bytes in several structures based on a table.
- `decrement_and_update_table`: Decrements values in a structure through a loop, updates a table of pointers, and clears specific bytes in a structure before calling another routine.
- `clear_structure_bytes`: Iterates through a table to clear two specific bytes in four different structures and then sets a global variable to one.
- `execute_final_routine`: Calls another routine and then sets a global variable to one.
- `traverse_and_free_lists_2`: This routine recursively traverses two linked lists stored in a global structure and calls a deallocation routine on each node.
- `process_table_entries_2`: Iterates through a table to copy data between memory locations, increments a value if non-zero, and sets a bit in a byte based on a calculated position.
- `process_table_entries`: Iterates through a table to copy values from one memory location to another and increments a value at a specific offset if it is non-zero.
- `set_bit_from_value`: Calculates a bit position based on a value at a pointer offset and stores a single bit at a specific byte location.
- `process_linked_list_nodes_2`: This routine recursively traverses a linked structure while managing global pointers and updating values during the traversal.
- `process_linked_list_nodes`: Recursively traverses a linked structure while managing global pointers and updating values during the traversal.
- `traverse_linked_list`: Recursively traverses a linked structure, updating pointers and values while managing a global state during the traversal.
- `traverse_linked_list_recursive`: Performs a recursive traversal of a structure or linked list, updating a value or pointer with the return value of each recursive call.
- `process_list_nodes`: Iterates through a linked list, updating pointers and potentially calling a function via a pointer from a global variable when a null pointer is encountered.
- `enable_interrupts`: Calls a library function using an offset from a global variable.
- `clear_linked_list_and_table`: Disables interrupts, traverses a linked list to clear a specific bit in each node, and then clears entries in a table before re-enabling interrupts.
- `execute_conditional_callback`: Checks a bit within a value at a memory offset and executes a function pointer if a specific bit is not set, then clears that memory location.
- `copy_fixed_bytes`: Copies sixty-four bytes one at a time from one pointer to another.
- `execute_callback_if_valid`: Checks if a global function pointer is non-zero and, if so, calls it before returning.
- `process_string_data`: Iterates through a data structure to process values and calls a routine to retrieve a value based on a bitwise operation.
- `execute_graphics_calls`: Calls three function pointers retrieved from a structure pointed to by a value in a global variable.
- `free_data_structures`: Iterates through a table of pointers to call a deallocation routine on non-null entries and then processes a separate array of ten pointers.
- `search_string_table_3`: Iterates through a table of pointers and performs a case-insensitive string comparison against a provided parameter.
- `process_data_table_and_sort`: Iterates through a table of data structures to extract specific byte values and indices.
- `call_add_int_server`: Library stub: loads its arguments and the exec library base, calls AddIntServer and returns its result to the caller.
- `execute_recursive_callback_chain`: Performs a recursive traversal of a structure or linked list, updating a value or pointer with the return value of each recursive call.
- `execute_recursive_call`: This routine performs a direct recursive call to itself.
- `execute_callback_loop_2`: Calls another routine repeatedly and returns its return value.
- `execute_callback_loop`: Executes a loop that repeatedly calls another routine, using its return value to determine whether to continue or terminate.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d7 to d0, then copies (a7)+ to d7, then returns.
- `process_table_loop`: Iterates through a sequence of routine calls and increments a global variable using a modulo sixteen operation if a specific condition is met.
- `increment_circular_index`: Increments a global variable and applies a modulo operation to keep it within a range of sixteen.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `map_character_index`: Transforms an input value into a new index by performing table lookups and applying arithmetic adjustments based on specific bit flags.
- `transform_index_via_table_and_flags`: This routine processes an input value by performing a table lookup or arithmetic transformation, applies conditional subtractions based on specific bit patterns, and adds various bit-based offsets to the final result.
- `map_flagged_values`: Maps an input value through a table or arithmetic transformation and applies additive offsets based on several bitwise flags.
- `calculate_flagged_value`: Modifies a value by adding specific offsets based on various bitwise flags and applies bounds checking.
- `map_argument_flags`: Modifies a value based on bitwise flags in an argument and applies several additive offsets to the result.
- `map_table_value_flags`: Retrieves a value from a table or performs an addition, then modifies it using bitwise flags and additive offsets.
- `map_argument_table_flags`: Reads a byte from a table using an index, remaps specific values, and sets high bits in the result based on flags in the argument.
- `map_argument_table_flags_2`: Reads a byte from a table using the low bits of the argument, remaps some values, then sets result bits according to flag bits in the argument.
- `update_circular_table_index`: Checks if two global variables are unequal, then executes a function via a pointer from a global variable.
- `load_table_entry`: Reads a four-byte value from a table indexed by a global variable and returns it, or zero when two global variables are equal.
- `get_font_handle`: Opens a font library if not already open and returns a handle from either a graphics library or a font library.
- `update_table_entry_via_callback`: Calculates an offset into a table based on an input parameter and checks a value at a specific table index.
- `clear_linked_list`: Iterates through a linked list and calls another routine on each node before setting a global pointer to zero.
- `validate_table_index`: Checks a value returned by another routine against a table of indices and updates a pointer within a structure if conditions are met.
- `find_max_index`: Iterates through a table of structures to find the index of an entry that satisfies certain conditional checks involving a comparison or a range.
- `call_avail_mem`: Library stub: loads its arguments and the exec library base, calls AvailMem and returns its result to the caller.
- `call_callback_directly`: Checks if a value at a specific memory offset is zero and then executes a function pointer retrieved from a table.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `set_font_parameters`: Updates several global variables with data from a pointer and calculates two values based on table entries and the new parameters.
- `execute_indirect_function_calls`: Retrieves a pointer from a structure and executes the function located at a specific offset within that structure twice.
- `update_bitmask`: Checks a bit in a global variable and then sets a different bit in that same variable.
- `toggle_bit_in_global`: Checks a bit in a parameter and returns its state from a global variable, then clears a different bit in that same global variable.
- `initialize_global_context`: Sets several global variables to specific values and stores a pointer to a provided address.
- `initialize_data_structure`: Sets three consecutive global bytes to specific values and stores a pointer to a global variable into a fourth.
- `set_short_values`: Sets one global byte to one and another global byte to zero.
- `copy_table_entry`: Copies a block of data from one location in a table to another based on an index, then calculates and stores two values derived from the new data.
- `execute_sequential_routines`: Calls two routines in sequence and returns the result of the first one.
- `execute_graphics_calls_3`: Retrieves a pointer from a structure to call three functions via offsets and then clears two values in a table and a global variable.
- `initialize_context_and_parse_string`: Sets several global variables to specific values and stores a pointer to a provided address.
- `parse_formatted_string`: Iterates through a character stream to build a buffer, handling special control characters and format specifiers.
- `perform_block_copy`: Checks several conditions and global pointers before calling a routine to copy a calculated memory block area.
- `parse_and_copy_string_data`: Searches for a non-space character within a memory range, temporarily clears it, and calls another routine to copy a calculated block of data.
- `fetch_buffered_byte`: Reads bytes from two different pointer-based buffers, performs a bitwise transformation on the value if a specific bit is set, and stores the result in a destination.
- `shift_global_byte_buffer`: Moves a byte from one global variable to another and clears the second variable.
- `update_table_values`: Modifies four different values within a table at an offset determined by a parameter.
- `search_string_table_2`: Iterates through a table of strings to find a match, returning a combined value from a table or a bit pattern if no match is found.
- `combine_data_segments`: Concatenates a high word derived from a calculation with a low word retrieved from a table.
- `search_string_table`: Iterates through a table of pointers to compare strings and returns a value from a table if a match is found or a specific bit pattern if no match is found.
- `return_all_ones`: Returns a value where all bits are set.
- `initialize_installation_environment`: Sets up memory structures and font parameters before executing a sequence of installation callback functions.
- `manage_installation_callback_chain`: Executes a sequence of callback functions to manage installation progress and updates a status buffer based on their return values.
- `execute_installation_callback_chain`: Iterates through a sequence of callback functions to manage installation progress and status.
- `process_installation_callback_chain`: Executes a series of recursive callbacks and updates a status buffer based on returned values.
- `process_installation_data_stream`: Reads data from a source through a pointer-based structure and writes it to a destination memory location.
- `execute_complex_callback`: Performs a series of indirect function calls and updates multiple global variables and table entries using values derived from a pointer and a structure.
- `preserve_global_word`: Saves a two-byte value from a global variable, calls another routine, and then restores the original value to that global variable.
- `log_error_and_terminate`: Formats an error message using a string and a variable, then performs a system teardown and exits the program.
- `release_system_resources`: Performs a complex teardown by executing various callback functions, clearing linked lists, and releasing system resources such as screens, windows, and graphics objects.
- `report_memory_allocation_error`: Triggers an error message regarding memory allocation failure.
- `allocate_linked_node`: Iterates through a list to find an available memory block, then initializes a new structure with a provided parameter and inserts it into a linked list.
- `free_linked_list_item`: Searches a linked list for a specific address, replaces a pointer if found, calls a function via a pointer, and decrements a global counter.
- `sum_callback_results`: Checks a global flag and repeatedly calls a function pointer via a table lookup, accumulating the returned integer values into a total sum.
- `execute_graphics_calls_2`: Retrieves a pointer from a structure and calls three different functions through offsets within that structure.
- `find_string_index`: Iterates through a table of strings to find a case-insensitive match for a provided string and returns the index or a failure code.
- `clear_short_globals`: Sets two consecutive global bytes to zero.
- `call_copy_block_area`: Calls a routine that copies a rectangular block of memory from one location to another.
- `copy_block_area`: Copies a rectangular block of memory from one location to another using nested loops and specific stride offsets.
- `scatter_bytes_to_buffers`: Copies bytes from a source pointer to multiple destination buffers using a nested loop structure.
- `copy_interleaved_data`: Copies data from a source buffer to a destination buffer using a pattern of five-byte increments and large memory strides.
- `update_indexed_value`: Stores a 32-bit value into a table at an index derived from a global variable, then increments that index modulo sixteen.
- `update_input_counters`: Decrements and resets internal counters within a structure, modifies several global values based on bitwise flags, and updates a multi-level counter hierarchy.
- `update_state_flags_2`: Updates a global variable by performing bitwise OR operations based on a comparison between a value at a pointer offset and a previously stored value.
- `update_state_flags`: Updates a global variable by performing bitwise OR operations based on a comparison between a value at a pointer offset and a previously stored value.
- `process_input_and_update_table`: Modifies a hardware register based on certain bit patterns and updates a table or a global index using a loop.
- `call_delay_routine`: Calls a delay routine if the provided argument is non-zero.
- `align_buffer_nibbles`: Adjusts a sequence of nibble values in a buffer to match a reference buffer, computing the signed differences for each digit.
- `compress_lz_data_stream_3`: Performs LZ-style compression on a source buffer by scanning for repeating byte sequences.
- `encode_lz_match_sequences`: Scans a source buffer for repeating byte sequences and writes either literal bytes or encoded match descriptors consisting of length and offset values to a destination buffer.
- `compress_data_stream_3`: Performs data compression by searching for repeating sequences within a buffer to replace them with length and offset descriptors.
- `compress_data_stream`: Performs a run-length encoding or dictionary-based compression on a memory buffer.
- `compress_data_stream_2`: Performs a variation of the Lempel-Ziv compression algorithm on a memory buffer.
- `decompress_data_stream`: Performs a custom decompression algorithm on a data stream, utilizing run-length encoding and variable-length bit/byte sequences to expand compressed data into a destination buffer.
- `compress_lz_data_stream_2`: Performs LZ-style compression by scanning a source buffer for repeating byte sequences and writing either literal bytes or encoded match descriptors to a destination buffer.
- `compress_lz_data_stream`: Performs LZ-style compression by scanning a source buffer for repeating byte sequences and writing either literal bytes or encoded match descriptors containing length and offset information to a destination buffer.
- `set_bit_and_offset`: Writes a constant bit pattern to a memory location and returns a pointer offset by a certain amount.
- `calculate_scaled_product`: Multiplies the upper and lower halves of a 32-bit value together, masks the product to sixteen bits, and scales the result by five.
- `call_interleaved_copy`: Calls a routine that transfers data from a source to a destination using a loop that processes five-byte chunks.
- `copy_interleaved_data_alt`: Transfers data from a source to a destination using a loop that processes five-byte chunks with large memory offsets.
- `traverse_and_process_linked_list`: Recursively traverses a linked list by calling itself with a pointer obtained from a previous call.
- `allocate_memory_with_checks`: Attempts to allocate memory and returns a status code based on the success of the allocation and the properties of the returned pointer.
- `decompress_variable_length_stream`: Decompresses a data stream by reading length-prefixed segments that specify either a sequence of bytes to copy or a single byte to repeat multiple times.
- `copy_variable_length_data_2`: Copies data from a source to a destination using a loop that handles both direct byte transfers and repeated byte sequences based on length values found in the source.
- `copy_variable_length_data`: Copies bytes from a source pointer to a destination while handling variable length segments and repeating single bytes based on values found in the data stream.
- `release_graphics_resources`: Clears two global variables and calls three different routines to release resources associated with a pointer.
- `process_installation_data_table`: Iterates through a table of data structures to retrieve and copy string values into a local buffer.
- `execute_conditional_function_2`: Fetches a pointer from a structure and executes a function, then conditionally executes a second function if the first returned a non-zero value.
- `update_event_command_stream`: Iterates through a command stream to modify bitmasks in a data structure or perform coordinate clamping.
- `process_event_command_stream_logic`: Processes a command structure by either updating bitmasks in a data structure based on lookup values or performing coordinate clamping and state updates.
- `process_event_command_stream_4`: Iterates through a linked list of command structures to update bitmasks in a data structure and manage a circular buffer of event identifiers.
- `update_event_state_and_buffer`: Iterates through a command stream to modify bitmasks in a data structure and update a circular buffer of event identifiers.
- `process_event_command_stream_2`: Iterates through a linked list of command structures to update bitmasks, modify coordinate values, and manage a circular buffer of event identifiers.
- `process_command_event_stream`: Iterates through a linked list of command structures to perform coordinate clamping and bitmask updates.
- `process_event_command_stream_7`: Iterates through a linked list of command structures to update bitmasks in a target buffer and manage a circular buffer of event data.
- `process_event_command_stream_6`: Iterates through a linked list of command structures to update bitmasks in a target buffer and manage a circular buffer of event data.
- `update_event_command_buffer`: Processes a stream of command data to modify bitmasks in a target buffer and manage a circular buffer of event identifiers.
- `update_event_command_bitmasks`: Iterates through a linked list of command structures to modify bitmasks in a target buffer based on encoded values.
- `update_event_command_buffer_2`: Iterates through a command stream to modify bitmasks within a data structure and manage a circular buffer of event identifiers.
- `process_event_command_list`: Iterates through a linked list of command structures to update bitmasks and manage a circular event buffer.
- `process_event_command_stream_3`: Parses a stream of command data to update bitmasks and modify coordinate values.
- `update_event_command_buffer_3`: Iterates through a linked list of command structures to modify bitmasks and update a circular buffer of event data.
- `process_event_command_stream`: Parses a sequence of command data to update bitmasks, modify coordinate values, and manage a circular buffer of event identifiers.
- `process_event_command_stream_5`: Iterates through a linked list of command structures to update bitmasks, modify coordinate values, and manage a circular buffer of event data.
- `execute_conditional_function_alt`: Retrieves a pointer from a structure to call a function and executes another function if the result is non-zero.
- `check_empty_stub`: The routine contains four instructions but performs no observable work and returns zero, acting as an empty stub that always returns zero.
- `fetch_and_copy_string_from_table`: Retrieves a pointer from a table using an index and copies the resulting null-terminated string into a local buffer.
- `initialize_resource_management_structures`: Initializes a series of data structures by iterating through a table, allocating memory for new nodes, and populating them with specific values and offsets.
- `call_indirect_function`: Loads a function pointer from an offset relative to a value stored on the stack and executes it.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `check_entity_flag_and_process`: Retrieves a pointer from another routine and, if valid, checks a specific bit in the second byte of the pointed-to structure.
- `copy_string_recursively`: Calculates the length of a null-terminated string and then copies its characters into a global buffer using a recursive mechanism controlled by a counter.
- `copy_string_to_buffer_3`: Iterates through a null-terminated string and copies each byte into a global buffer.
- `execute_recursive_call_2`: The routine performs a self-referential call to itself before returning.
- `parse_format_string_6`: Iterates through a string to identify and process format specifiers such as decimal, string, and pointer types while managing a buffer for characters.
- `parse_format_string_4`: Iterates through a string to identify and process format specifiers.
- `parse_printf_format_string`: Iterates through a format string to identify and process specifiers such as decimal, string, and pointer types.
- `parse_format_string_5`: Iterates through a string to identify format specifiers such as decimal, string, or pointer types.
- `parse_format_string_3`: Parses a string containing format specifiers to build a formatted output.
- `parse_escaped_string_tokens`: Iterates through a character string to identify specific control characters and escape sequences.
- `parse_command_string`: Parses a character string to identify specific command tokens and executes corresponding logic.
- `parse_format_string_2_2`: Iterates through a string to identify format specifiers and extracts corresponding data from a pointer-based argument list.
- `parse_format_string_2`: Parses a string containing format specifiers to perform data extraction and pointer manipulation.
- `invoke_linked_list_traversal`: Calls a routine that traverses a linked list structure to update node pointers and field values based on bitwise flags.
- `parse_format_string`: Parses a format string containing tokens such as hash, plus, minus, asterisk, leading zeros, decimal points, l, h, p, x, and digits, while advancing a pointer through a parameter list.
- `process_format_string`: Walks through a character string one byte at a time and stops when it finds a percent sign.
- `initialize_structure_fields_2`: Checks specific bits and values within a structure, then either calls another routine to update fields or clears several words and sets a bitmask within the structure.
- `update_structure_fields`: Allocates a memory block and updates several fields within a structure using the returned pointer, including bitwise operations and clearing specific words.
- `modify_structure_fields`: Writes a value to a structure, clears two words at specific offsets, and performs a bitwise AND on another word within that structure.
- `process_linked_list_nodes_4`: Traverses a linked list structure to update node pointers and field values based on bitwise flags.
- `process_linked_list_nodes_3`: Traverses a linked list structure to update node pointers and field values based on bitwise flags.
- `pop_byte_from_list_2`: Decrements a counter in a structure and, if the counter is greater than zero, stores a byte from a register into a memory location pointed to by a pointer in the structure.
- `pop_byte_from_list`: Decrements a counter in a structure and, if not zero, increments a pointer and stores a byte from a register into the memory location pointed to by that pointer.
- `forward_argument_and_return`: Forwards its argument to another routine and returns the result.
- `manipulate_linked_list_nodes`: Traverses and modifies a linked list by updating node pointers and field values based on conditional comparisons.
- `update_entity_status_flags`: Updates status flags in a structure based on the result of another routine and compares a value against a stored short.
- `update_structure_values_2`: Compares a value from another routine against a stored short and updates status flags in a structure.
- `update_linked_list_nodes`: Modifies values within a linked list structure by comparing field differences and traversing nodes.
- `process_structure_difference`: Calculates the difference between two fields in a structure and, if non-zero, potentially traverses a linked list while decrementing a counter.
- `update_structure_field_2`: Updates a status flag in a structure based on a comparison, then modifies a value at a specific offset within that structure.
- `update_structure_field`: Compares a byte to a stored value, updates a flag in a structure, and conditionally copies a word or decrements a counter based on a byte value.
- `update_structure_counter`: Updates a structure by copying values between offsets and decrements a counter if a byte differs from a stored value, potentially calling another routine when the counter reaches zero.
- `pop_byte_if_different`: Compares a byte to a value in a structure and, if they differ, decrements a counter and stores the byte into a pointer within that structure.
- `perform_immediate_recursion`: This routine immediately calls itself before returning to the caller.
- `return_immediately_2`: This routine is empty and performs an immediate return to its caller.
- `return_from_subroutine`: Performs a standard return from a subroutine to the calling function.
- `iterate_table_entries`: Steps backward through a global table of structures, skipping entries that are empty or have a particular flag bit set, and calls another routine for each matching entry.
- `parse_string_buffer`: Parses a string from a buffer, handling quoted substrings and whitespace, and stores the resulting pointers and count into a structure.
- `tokenize_quoted_string`: Walks through a text buffer, skipping whitespace and splitting the content into up to 32 tokens while treating double-quoted runs as single tokens, writing a pointer to each token into a stack table and null-terminating each token in place.
- `execute_callback_chain`: Invokes a series of function pointers stored in various global locations and structures based on the state of specific flags.
- `tokenize_command_line_arguments`: Parses a string of command-line arguments into an array of pointers.
- `convert_uppercase_chars`: Iterates through a null-terminated string and converts uppercase ASCII characters to lowercase based on a flag in a global lookup table.
- `compare_case_insensitive_strings`: The routine compares two byte strings character by character, converting uppercase letters to lowercase for comparison, and returns the difference between the first mismatching characters or zero if equal.
- `copy_string_to_buffer`: The routine finds the terminating zero byte of the destination buffer and copies bytes from the source pointer into that location until a zero byte is copied.
- `find_byte_in_string`: Searches a null-terminated string for a specific byte and returns a pointer to its location, or zero when the byte is not present.
- `call_and_return_value`: Forwards its argument to another routine and returns whatever that routine produces.
- `find_last_occurrence`: Searches a null-terminated string for the last occurrence of a specific byte and returns a pointer to it.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `reverse_engineer`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `convert_number_to_ascii_2`: Converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `convert_number_to_ascii`: The routine converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `expand_packed_nibbles`: Expands a value by looking up nibbles in a table and writes the resulting bytes into a destination buffer in reverse order.
- `reverse_nibbles_from_table`: The routine expands a packed nibble value using a global lookup table and writes the resulting bytes in reverse order to a destination.
- `convert_signed_integer`: Converts a signed integer into an ASCII string by writing a minus sign if negative and appending digits, then reversing the string in a buffer.
- `format_signed_number`: Writes a minus sign to a buffer if a value is negative, then converts the absolute value to ASCII digits and reverses them into the buffer.
- `format_integer_string`: Converts a value into a null-terminated ASCII string by iterating through digits and reversing them within a buffer.
- `parse_signed_integer`: Converts an ASCII string representing a signed decimal integer into a 16-bit value, handling optional leading plus or minus signs.
- `increment_and_store`: Increments a global pointer and stores a byte from a parameter into the location pointed to by the incremented global pointer.
- `set_global_pointer`: Assigns a global pointer to a supplied address, invokes another routine to walk a string, and finally stores a zero byte at the address held in the global pointer.
- `allocate_heap_block`: It rounds the requested size up to a four-byte boundary and searches a linked list of free blocks for a suitable gap, splitting or merging blocks as needed.
- `call_and_return_value_two_args`: Forwards two arguments to another routine and returns the result.
- `traverse_linked_list_2`: Retrieves a pointer from another routine and, if valid, follows the pointer's third field to continue processing.
- `execute_callback_logic`: Checks a global variable and calls another routine through a pointer at a specific offset.
- `nop_sequence_padding`: This routine is empty and consists of three instructions that perform no operation.
- `copy_memory_block`: Copies a specified number of bytes from a source address to a destination address.
- `free_linked_list_nodes`: It walks a linked list held in a global pointer, invoking a destructor stored at a fixed offset within each node.
- `validate_linked_entry`: Calls another routine to retrieve a pointer.
- `allocate_linked_list_node`: It obtains a memory block and initializes its fields with a provided value and a global pointer.
- `allocate_free_memory_block`: Traverses a linked list of free memory blocks to find a segment large enough for the requested size, splitting the block if necessary to fit alignment requirements.
- `reduce_and_return_value`: Calls itself with a reduced value derived from the input parameter and then returns that value.
- `copy_and_close_structure`: Loads a value from a routine pointer and returns failure if it is missing, then copies a fixed number of words from one structure into another and calls a cleanup routine when the two pointers no longer match.
- `process_linked_list_flags`: Walks a global linked list of structures, testing two flag bits in a status byte and computing the difference between two integer fields.
- `invoke_global_callback`: Invokes a callback through a pointer in a global table and, when a global flag is set, recursively calls itself first.
- `dispatch_callback_with_adjustment`: Invokes a callback through a pointer in a global table and, when a global flag is set, recursively calls itself first.
- `execute_global_callback_offset_2a`: Checks a global variable and recursively calls another routine if it is non-zero, then invokes a function pointer at an offset from the global variable.
- `initialize_structure_fields`: It writes a full-byte bitmask at a fixed offset of a structure and writes full-word bitmasks at two other offsets.
- `cleanup_display_context`: It conditionally invokes a callback when a structure field is nonzero, then writes reset values at two fixed offsets.
- `invoke_method_if_present`: Checks whether a global pointer is non-null and calls another routine when it is, then invokes a function pointer stored within a structure.
- `copy_and_call_table_function`: Calls a function via a pointer in a global variable, copies a block of memory from a source to a stack-based buffer, and then calls two other routines.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `calculate_fixed_point_remainder`: Performs a complex division-based calculation using normalized fixed-point arithmetic to determine a remainder.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `allocate_and_set_node`: It allocates memory for a structure through a library call and, if successful, initializes certain bytes and a dword with a provided parameter.
- `create_message_node`: It obtains a global variable and calls a routine through a table; if the result is a sentinel, it returns zero.
- `allocate_and_initialize_node`: Allocates memory for a structure via a library call and initializes several bytes and words within that structure using values from the caller.
- `allocate_and_set_node_2`: It allocates memory for a structure through a library call and, if successful, initializes certain bytes and dwords from caller-held values.
- `validate_table_entry_index`: It checks if an index is within bounds and if the corresponding entry in a global table is non-zero.
- `check_and_clear_global_pointer`: Invokes a routine through a pointer in a global table, and when a result has certain bits set and a global pointer is non-null, calls another routine and clears that global pointer.

### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and stores it in a buffer.

### Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 1 of 4).adf:eobii

- `initialize_system_state`: Performs initial setup by calling a subroutine, executing a loop of function calls, clearing a value in a table, and copying a block of data to a new location.
- `clear_and_copy_data`: Performs a series of memory operations including clearing a pointer in a table and copying a sequence of words from one location to another.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `process_memory_blocks`: Executes a loop that performs memory copies and updates values in a table based on calculated offsets, followed by a conditional library call.
- `decode_compressed_data`: Performs bitstream decoding to expand compressed data into a destination memory buffer.
- `copy_memory_block_2`: Copies a block of memory from a source address to a destination address.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 1 of 4).adf:diamonds

- `initialize_system_hardware`: Performs several initialization tasks including clearing audio DMA, configuring interrupt enable registers, and setting hardware control registers.
- `increment_nibbles`: Increments the nibble values of words in a destination table based on the corresponding nibbles in a source table.
- `wait_for_input_and_clear_table`: Polls a hardware register for a specific bit status while iterating through nested loops, eventually zeroing out a large block of memory.
- `decrement_nibbles`: Decrements the nibble values of words in a destination table based on the corresponding nibbles in a source table.
- `expand_string_data`: Iterates through a string and replaces each character with a series of values fetched from a large data table.
- `execute_sequential_callbacks`: Executes a sequence of several different routines and updates a global variable and a local value.
- `swap_global_values`: Swaps the values of two global variables and then calls another routine seven times before setting a global variable to a constant value.
- `shift_and_fill_table`: Shifts a sequence of values through a series of global variables and then populates a table with incrementing values.
- `multiply_table_values`: Iterates through a sequence of eight words and multiplies each value by four.
- `initialize_structure_data`: Waits for a specific bit in a byte to clear before initializing a structure with several constant values and pointers.
- `reset_structure_fields`: Waits for a specific bit in a byte to clear before resetting specific fields within a structure to zero or constant values.
- `set_structure_pointers`: Waits for a specific bit in a byte to clear before setting two identical pointers in a structure to a calculated address and initializing other fields.
- `setup_blitter_parameters`: Waits for a specific bit in a structure to clear before writing several constant values and offsets to memory locations and calling other routines.
- `apply_bitmask_to_buffer`: Calculates a bitwise XOR operation on a word at a memory location determined by a coordinate-based pointer calculation.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
- `empty_routine`: This routine performs no operations and immediately returns.
- `perform_coordinate_rotation`: Performs complex fixed-point arithmetic to rotate or transform a set of coordinates using values from a table.
- `initialize_graphics_library`: Opens the graphics library, retrieves its base address, and copies a block of data into a global buffer before storing that buffer's address in another location.
- `add_to_global_table`: Retrieves a value from a function call and adds it to several consecutive longwords in a global table.
- `update_status_structure`: Waits for a specific bit in a structure to clear, then writes several constant values and calculated offsets into that structure.
- `update_object_state_parameters`: Polls a status flag in a data structure until a specific bit is cleared, then writes a series of values including a calculated offset, a constant, and a large integer to various offsets within that structure.
- `set_structure_parameters`: Waits for a specific bit in a structure to clear and then populates several fields of that structure with calculated values and constants.
- `compare_area_dimensions`: Compares the product of two sets of dimensions stored in a table against another set of dimensions to determine if a condition is met.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d7, then returns.
- `configure_hardware_registers`: Waits for a status bit to clear and then writes several constant values to a structure before iterating through a list to perform conditional calls.
- `execute_conditional_calls`: Performs a series of calls to another routine based on the value of a local variable.
- `modify_bitmapped_data`: Calculates a memory address based on multiple input values and performs a bitwise XOR operation on the data at that location within a loop.
- `perform_coordinate_transformation`: Uses a series of multiplications and bit shifts to transform values from a table and stores the results into a destination buffer.
- `load_string_data`: Finds the maximum byte value in a sequence, uses it to calculate an offset into a data table, and copies strings into a global buffer.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `update_audio_and_memory`: Updates several hardware registers with values from memory and performs a loop that modifies a global variable and checks a byte against it.
- `update_audio_hardware_registers`: Updates the four audio channel pitch/frequency registers and the video beam position register.
- `update_entity_state_and_attributes`: Retrieves data from a table based on a nibble-derived index and updates a structure with new coordinate or attribute values.
- `update_structure_values`: Modifies a shortword within a structure by adding or subtracting a byte value, and performs bounds checking against another shortword in the structure.
- `update_coordinate_data`: Calculates a new value using bitwise operations and multiplication from a structure, updates a field in that structure, and stores the result in another field.
- `modify_attribute_via_command`: Processes a command byte to update a numeric value within a structure using addition, subtraction, or table-based lookups.

### 6

- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d0, then returns.

### 8

- `return_constant_3`: An empty routine that simply returns a constant value.

### diskfont.library

- `set_font_handle`: As library code, this routine checks a return value from a table entry and, if non-zero, stores a value into a structure and calls another routine via a table entry.
- `check_and_set_font_handle`: As library code, this routine calls a routine via a table entry using the library base.
- `initialize_data_structure_2`: Library code that invokes two functions via pointers in a table and, upon success, initializes a new data structure by setting a pointer to itself and updating several fields.
- `traverse_and_free_list`: Library code that iterates through a linked list to remove elements based on a specific condition and performs memory management operations or sets a status bit.
- `manage_library_state`: Library code that either sets a bit in a byte or performs a series of memory and pointer operations involving a linked list structure.
- `set_high_bit`: As part of library code, this routine sets the most significant bit of a byte at a fixed offset from a pointer.
- `empty_routine_2`: This library routine is empty.
- `clear_flag_and_increment_counter`: As part of library code, this routine clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_check_flag`: As part of library code, this routine decrements a short integer and, if it reaches zero while a specific bit is set, calls a function pointer stored in a table.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `parse_font_metadata_stream_2`: Iterates through a linked list of font metadata entries to extract character definitions and properties.
- `copy_linked_font_strings`: Iterates through a linked list of data structures to calculate string lengths and copy the character data into a destination buffer.
- `parse_linked_font_strings`: Iterates through a linked list of data structures to extract and copy string data into a destination buffer.
- `process_font_metadata_stream`: Iterates through a data stream to identify and extract font character definitions.
- `parse_font_metadata_table_2`: Iterates through a data stream to identify specific font character definitions.
- `parse_font_metadata_table`: Iterates through a data stream to identify specific font character definitions.
- `load_font_data_from_disk`: Reads a block of data from a disk using a file read routine and parses it to populate a structured table.
- `parse_font_metadata_stream`: Iterates through a data stream to identify and extract font character definitions.
- `return_zero_stub`: As part of library code, this routine returns zero and performs no operations on registers, memory, or hardware.
- `calculate_glyph_offset_2`: Library code that calculates a numerical offset by comparing values from two structures and subtracting elements from a global table based on bitwise flags.
- `calculate_glyph_offset`: As library code, this routine iterates through a table of values using bitwise comparisons and arithmetic shifts to locate a specific entry and calculates a resulting offset.
- `search_glyph_index_table`: Iterates through a table of values using bitwise comparisons and arithmetic shifts to locate a specific entry.
- `calculate_font_glyph_offset`: Iterates through a table of values to calculate a specific offset based on bitwise comparisons and arithmetic shifts.
- `return_constant_value`: This library routine returns a constant value without using any arguments or registers.
- `update_glyph_metadata_table_2`: Iterates through a collection of font glyphs to find the entry with the highest metadata value.
- `update_glyph_metadata_table`: Iterates through a font glyph collection to identify the entry with the highest metadata value and updates its properties, including incrementing a counter and modifying bitflags.
- `update_font_glyph_metadata`: Iterates through a font data structure to find the glyph with the highest metadata value and updates its properties, including incrementing a counter and modifying bitflags.
- `find_max_font_metadata_entry`: Iterates through a collection of font data structures to identify the entry containing the highest value in a specific field.
- `reorganize_font_metadata_table`: Iterates through a collection of font data structures to find the entry with the highest value in a specific field.
- `update_font_metadata_and_unload`: Iterates through a list of font entries to find the one with the highest value in a specific field.
- `search_and_update_font_table`: Iterates through a collection of font data to find a specific entry based on a comparison loop.
- `parse_disk_font_data`: Parses a font data stream from a file to build an in-memory font table.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `find_string_terminator`: Library code that increments a pointer until it reaches a null byte and then calls another routine.
- `skip_to_string_terminator_and_call`: This library code increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `count_null_terminated_bytes_2`: This library code increments a counter while advancing a pointer and reading bytes until a zero byte is encountered.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `compare_byte_sequences_2`: As library code, this routine compares two sequences of bytes and returns a constant if they are not identical.
- `compare_string_sequences`: This library code compares two sequences of bytes and returns one if they are not identical or a specific constant if they are.
- `compare_byte_sequences`: Library code that compares two sequences of bytes and returns a constant if they are not identical.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_first_occurrence_3`: As library code, this routine searches forward through a null-terminated string for the first occurrence of a specific byte.
- `find_first_occurrence`: This library code searches forward through a null-terminated string for the first occurrence of a specific byte.
- `find_first_occurrence_2`: Library code that searches forward through a null-terminated string for the first occurrence of a specific byte.
- `find_last_occurrence_4`: As library code, this routine searches backward through a string to find the last occurrence of a specific byte.
- `find_last_occurrence_2`: This library code searches backward through a string to find the last occurrence of a specific byte.
- `find_last_occurrence_3`: Library code that searches backward through a string to find the last occurrence of a specific byte.
- `find_delimiter_position`: Library code that searches for the last occurrence of specific characters within a string and returns the incremented position.
- `increment_returned_value`: This library routine returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: This library code invokes another unnamed function and returns, serving as a simple wrapper or dispatch point within the diskfont.library.
- `call_external_handler`: This library routine calls another routine and returns.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_another_routine`: Library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Eye of the Beholder II - Advanced Dungeons& Dragons (1992)(SSI)(De)[cr D&R](Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_indirect_pointer`: Executes a routine via a pointer retrieved from a table.
- `update_data_structure_2`: Modifies a value within a table and clears subsequent entries in a list.
- `process_string_buffer`: This routine manipulates a string buffer by copying data from a source pointer and inserting specific control characters.
- `execute_indirect_function_pointers`: Retrieves a pointer from a global variable and executes it, followed by a sequence of indirect calls using offsets derived from that pointer.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `evaluate_conditional_callback`: Executes a function pointer from a table and compares its return value against a value in a structure to decide whether to store a register value and execute another function.
- `parse_string_buffer_2`: Processes a character stream to populate a structured string buffer.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `render_font_character`: Retrieves character data from a font table and processes bitwise modifications to a byte at a calculated memory location.
- `toggle_bit_and_call`: toggles a specific bit within a byte at a calculated memory location and then calls a function from a table.
- `nop_padding_2`: This routine is empty and consists of a single non-operational instruction used for memory alignment or padding.
- `sum_function_results`: checks two conditions and then iteratively calls a function through a pointer, accumulating the returned values into a sum.
- `return_immediately_4`: This routine performs no operations and immediately returns to its caller.
- `return_immediately_5`: The routine performs no operations and returns to its caller.
- `reset_global_values_2`: calls another routine with calculated parameters and then clears two words in a table before resetting a global variable to zero.
- `return_conditional_value`: returns a value based on whether a byte is zero.
- `conditionally_transfer_register`: Checks if a byte in a register is zero, and if so, copies the contents of one register to another.
- `flip_bit_and_call_indirect`: Flips a single bit within a byte at a calculated memory location using a bitmask derived from a register.
- `render_font_data`: Retrieves character data from a font table and performs bitwise modifications to a byte at a calculated memory location.
- `execute_conditional_call`: Checks a stack pointer and either enters a loop to call a function pointer or performs a conditional word copy.
- `execute_indirect_call`: Performs a nested loop that calculates an offset and calls a function pointer stored in a global table.
- `conditional_move_word`: If a byte input is zero, it copies a word from one location to another.
- `conditional_move_word_2`: Copies a word from one location to another if the input byte is zero.
- `manipulate_bitmask_buffer`: Iterates through memory to perform bitwise operations on a buffer using a source byte and a bitmask.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_bit_and_jump`: Toggles a single bit within a byte at a calculated memory location and then calls a function from a table.
- `return_immediately_6`: This routine performs no operations and returns to its caller immediately.
- `modify_array_elements`: Updates four consecutive elements in an array based on a provided value using different arithmetic multipliers.
- `parse_scripted_stream`: Processes a structured data stream and linked lists to decode variable-length sequences, update array elements, and manage memory blocks.
- `update_global_pointers`: Checks if a global variable points to a valid memory location, then updates that global variable with values from two registers.
- `clear_global_structure`: Sets two word-sized fields at a specific offset within a structure to zero and then clears a global variable.
- `conditional_move_word_3`: Moves a word from one location to another if the input byte is zero.
- `conditionally_return_register_value`: Checks if a byte is zero and, if so, returns a value from an undefined register.
- `parse_scripted_data_stream`: Iterates through a structured data stream to extract and decode variable-length sequences.
- `check_byte_zero`: Checks if a byte is zero and returns a value from another register based on that condition.
- `check_byte_zero_2`: Evaluates a byte for zero and returns a value from a register.
- `check_byte_zero_3`: Returns a register value if a specific byte is not zero.
- `return_immediately_7`: This routine performs no operations and returns to its caller.
- `calculate_table_offset`: Performs arithmetic operations using values from a table and a global variable to pass a calculated result to another routine.
- `return_immediately_stub_3`: The routine consists of a single return instruction and performs no work.
- `process_compressed_data`: Iterates through a data stream, performing bitwise operations and pointer increments to decode values and update a bitmask in a global structure.
- `parse_input_string`: Processes a character stream into a buffer by handling control characters and escape sequences.
- `manage_linked_list`: Manipulates a linked list by updating pointers and transferring data between nodes based on specific conditions.
- `return_immediately_8`: The routine performs no operations and returns to the caller immediately.
- `return_immediately_stub`: The routine consists of a single return instruction with no other operations, so it performs no work and returns control to its caller.
- `copy_data_block`: Iteratively copies a block of data from one memory location to another and updates several global variables and a table with specific values.
- `return_immediately_9`: This routine performs no operations and returns to the caller immediately.
- `conditionally_return_value`: Checks a byte and returns a value from a register if the byte is zero.
- `count_set_bits`: An unreached routine that calls a routine to count set bits and find the next value.
- `count_set_bits_and_find_next`: Decrements a counter by iterating through a bitmask to count the number of set bits.
- `find_first_set_bit`: Iteratively shifts a bitmask to find the position of the first set bit.
- `nop_padding_sequence_3`: This routine is empty and consists only of non-operational instructions used as padding within the executable segment.
- `conditionally_return_register_value_2`: Checks if a byte is zero and, if so, returns a value previously held in a register.
- `return_immediately_10`: This routine performs no operations and returns control to the caller immediately.
- `parse_scripted_data_stream_2`: Iterates through a linked list of data structures to parse encoded byte streams.
- `return_register_value`: Checks if a byte in a register is zero and returns a value from another register.
- `zero_allocated_memory`: Allocates a block of memory and fills it with zeros using a loop.
- `flip_bit_and_call_2`: Calls multiple functions via pointers and flips a specific bit in a memory location based on a returned value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `flip_bit_and_call`: Flips a specific bit in a byte through a pointer and then calls a routine from a table.
- `return_on_zero`: Returns a value based on whether a byte is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_on_flag`: Returns a value based on the state of a processor flag.
- `parse_string_tokens`: Parses a string into tokens by skipping whitespace and handling quoted substrings, storing the resulting pointers into a table.
- `parse_string_tokens_2`: Iterates through a null-terminated string to extract tokens, handling whitespace and quoted substrings, and stores the resulting pointers into a table.
- `parse_command_string_3`: Parses a space-delimited string into an array of pointers, supporting quoted substrings.
- `traverse_linked_list_and_recurse`: Iterates through a linked list of structures, checking if a specific bit pattern is present in a status byte and if the difference between two short integers is non-zero.
- `parse_quoted_string_tokens`: Parses a string into a list of tokens, handling whitespace and quoted substrings.
- `trigger_system_event`: Calls a library function to cause a system event and returns zero.
- `update_linked_list_node`: If a global variable is non-zero, it adds the value of a pointer to the value at a memory location and calls another routine three times.
- `process_linked_list_nodes_5`: Iterates through a linked list, clearing a bit in a byte and searching a table for a minimum value, then updates a pointer in that table and modifies certain fields in a structure or object.
- `traverse_linked_list_3`: Traverses a linked list, checking bit flags in a node's property byte and updating head or certain pointers while removing nodes from the list.
- `iterate_and_modify_bit`: Iterates a loop a certain number of times based on a value in a pointer, calls another routine, and potentially clears a bit in a global byte if a condition is met.
- `process_data_list`: Iterates through a list of structures to perform conditional updates, coordinate scaling, and bitwise flag manipulations based on specific byte values.
- `calculate_scaled_coordinate_offset`: Calculates a scaled coordinate or offset using bitwise shifts and division based on a divisor found in a data structure.
- `find_and_prepend_to_list`: Iterates through a list of structures to find an entry where a specific bit in a flag byte is clear, then updates that entry's flag and inserts it at the head of a linked list using a pointer from a global variable.
- `populate_entity_attribute_structure`: Updates a newly allocated structure with data retrieved from a linked list of entities.
- `process_event_script_data`: Iterates through a linked list of structures to parse and execute encoded instruction streams.
- `check_available_memory`: Calls a function via a pointer stored in a global variable and returns whether the result is greater than ten thousand.
- `free_dual_pointer_blocks`: Retrieves a value from a global variable and uses it as a pointer to call a function with an offset, then repeats this process using the original value as a base for a second function call.
- `free_pointer_list_2`: Iterates through a list of pointers and calls a routine to free each non-null pointer, then calls a library function to free a global pointer.
- `free_memory_if_offset_nonzero`: Checks if a provided pointer is non-null and if the value at a specific offset within that pointer is non-zero.
- `conditional_free_memory`: Checks if a provided pointer is non-null and, if valid, executes a function pointer retrieved from a global variable to release that memory.
- `free_memory_if_not_null`: Checks if a provided pointer is non-zero and, if valid, executes a deallocation routine retrieved from an offset of a global pointer.
- `free_linked_structure_recursively`: Recursively traverses a linked list by checking specific bits in a byte at a fixed offset and following a pointer.
- `free_linked_structure_2`: Recursively processes a pointer from a structure, decrements a counter, and executes function pointers to deallocate memory.
- `free_linked_structure`: Recursively processes a pointer from a structure and then decrements a counter before calling a deallocation routine.
- `decrement_and_free_resource`: Decrements a value at a specific offset within a structure and executes a function pointer stored at a relative offset from a global variable.
- `decrement_and_free_memory`: Calls another routine, decrements a value at a specific offset within a pointer if it is non-zero, and then calls a library function to free memory and another routine via a global pointer.
- `initialize_audio_system_resources`: Allocates memory for a system structure and populates it with specific configuration values and function pointers.
- `clear_linked_lists`: Iterates through two separate linked lists stored in a global structure and calls a routine to process each node.
- `decrement_table_values_3`: Iterates through a table to extract values from pointers, decrements those values in a loop, and then clears specific offsets within the original pointers.
- `process_nested_data_loops`: Performs complex nested loops to manipulate values in a table, decrement specific short integers, and clear two bytes at an offset within several pointers retrieved from a global table.
- `decrement_table_entries_2`: Decrements short values in a table and clears two-byte offsets within pointers stored in a separate table.
- `decrement_table_values_2`: Decrements non-zero short values in a table and clears two bytes at a specific offset within pointers stored in that table.
- `decrement_table_entries`: Iterates through a loop to decrement non-zero short values in a table and then clears two bytes at a specific offset within pointers stored in a global table.
- `clear_pointer_offsets`: Iterates through a table to clear two bytes at a specific offset within four pointers.
- `set_global_flag_and_return`: Sets a global variable to one and returns.
- `cleanup_context_and_lists`: Checks a global pointer, executes a callback, and then recursively clears two linked lists and resets specific fields within a structure.
- `process_data_table`: Iterates through a table to copy data between structures, increments a value at a specific offset in another structure, and sets a bit in a byte based on the table size.
- `parse_resource_data_stream_2`: Iterates through a data stream containing tagged blocks to identify specific resource types.
- `parse_resource_stream`: Iterates through a data stream containing tagged chunks to copy or manipulate memory segments.
- `allocate_and_copy_buffer`: Allocates memory via a library function and copies a specified number of bytes from a source pointer to the new buffer.
- `parse_tagged_data_stream`: Iterates through a data buffer to identify and process specific four-character tags.
- `remove_linked_list_nodes`: Traverses a linked list and removes nodes by comparing strings case-insensitively.
- `insert_into_linked_list`: Inserts a structure into a linked list at a specific position.
- `calculate_string_length`: Recursively determines the length of a null-terminated string.
- `parse_tagged_resource_stream`: Iterates through a data stream containing tagged blocks to identify specific resource types and process them.
- `modify_byte_at_offset`: Calls another routine and, if the result is non-zero, clears a specific bit in a byte at a fixed offset from that result.
- `clear_linked_list_and_data`: Disables interrupts, traverses a linked list to clear a specific bit in each node, clears four entries in a table, and then re-enables interrupts.
- `zero_memory_block`: Calculates a length and a destination pointer from global data and fills that memory range with zeros.
- `process_flagged_callback`: Checks a byte in a table for a specific bit, executes a function pointer from a structure if the bit is clear, and then clears the table entry.
- `copy_sixty_four_bytes`: Copies sixty-four bytes from a source memory location to a destination memory location using a decrementing loop.
- `execute_cleanup_callback`: Checks if a global function pointer is non-zero and executes it.
- `execute_callback_via_pointer`: Traverses a structure, updates a field with the result of another routine, and then calls a function pointer using a value derived from a register.
- `execute_indirect_calls`: Fetches a pointer from a table and executes three different functions stored at specific offsets within the pointed-to structure.
- `process_command_tables`: Iterates through a table of structures to execute function pointers stored within them and then processes a second fixed-size list of pointers.
- `search_string_table_5`: Iterates through a table of pointers and performs a case-insensitive string comparison against a provided parameter.
- `process_command_tokens`: Iterates through a table of command structures, executes callbacks, and sorts a subset of flagged entries into new arrays based on a specific bit.
- `call_library_function`: Calls a function located at a negative offset from a global pointer.
- `perform_recursive_call`: This routine calls itself indefinitely, creating an infinite recursion loop.
- `perform_infinite_recursion`: This routine calls itself indefinitely without performing any other operations.
- `execute_recursive_calls`: Performs a series of recursive calls within a loop until a specific condition is met.
- `null_operation_sequence`: This routine consists of three instructions that perform no functional operation and return immediately.
- `increment_global_index`: Iterates through a loop, making recursive calls and incrementing a global variable modulo sixteen when a specific condition is met.
- `map_input_to_index`: Performs a complex series of range checks and bitwise transformations on an input parameter to return a calculated index or value.
- `translate_encoded_value`: Translates an input value by looking up a byte in a table and applying several bitwise offsets based on the input bits.
- `transform_index_value`: Retrieves a byte from a table using a masked input, applies several conditional substitutions, and adds bit-based offsets to produce a final value.
- `update_table_index`: Uses a global variable to access a table, potentially updates the global variable with new values from the table, and wraps the index within a range of sixteen.
- `fetch_conditional_table_entry`: Checks if two global variables are unequal, and if so, retrieves a four-byte value from a table using one of the global variables as an index.
- `execute_callback_sequence`: Calls three different routines in sequence and returns the result of the second call.
- `handle_structured_input`: Performs arithmetic on values retrieved from a table, executes function pointers, and processes command input and status.
- `process_complex_data_stream`: Uses memory copying and compressed stream decoding to manipulate values in a table and executes function pointers retrieved from a global structure.
- `decode_compressed_stream`: Processes a byte stream by interpreting specific control characters to perform recursive decoding or data copying.
- `initialize_and_call_font_function`: Checks if a global variable is zero to initialize it via a function call, then uses two parameters to call another function from a structure.
- `validate_and_execute_callback_2`: Performs a series of checks against a table and memory availability to validate an index and execute a function pointer from a structure.
- `traverse_linked_list_4`: Walks through a null-terminated linked list of pointers and calls another routine for each element before clearing a global pointer.
- `store_multiple_words`: Calculates an offset based on a parameter and stores four consecutive words into a table at that location.
- `validate_and_execute_callback`: Checks a value returned by another routine against a table of indices and, if valid, executes a function pointer from a structure and clears it.
- `find_min_value_index`: Iterates through a null-terminated table of structures to find the index of the entry containing the smallest byte value at a specific offset, provided certain integer and byte comparisons are met.
- `call_avail_mem_2`: Library stub: loads its arguments and the exec library base, calls AvailMem and returns its result to the caller.
- `call_structure_function`: Checks if a value in a table is non-zero and then executes a function pointer retrieved from a structure.
- `call_rem_int_server_2`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `execute_callback_sequence_2`: A 41-instruction routine that executes a cleanup callback, updates a structure field, and performs indirect calls based on a structure value.
- `update_minimum_value`: Compares a value from a table with a register and stores the smaller of the two into a different table offset.
- `store_structure_value`: Stores a word from a register into a memory location calculated from a pointer and a table offset.
- `update_clamped_sum_or_min`: Compares a value from a table against the sum of a register and that table value, storing either the table value or the clamped sum into a table offset.
- `store_clamped_sum`: Calculates the sum of two values and stores the result into a table offset if the sum is negative, otherwise stores zero.
- `store_summed_value`: Calculates the sum of two values and stores the result into a memory location calculated from a pointer and a table offset.
- `update_structure_field_3`: Calls another routine and then stores a value into a memory location calculated from a pointer and a table offset.
- `execute_indirect_callbacks`: Checks a value and, depending on its state, performs two indirect calls to a function pointer retrieved from a structure.
- `call_indirect_function_4`: Retrieves a pointer from a structure and executes the function at that address twice.
- `execute_indirect_pointer`: Fetches a value from an offset within a structure and performs two indirect calls to the resulting address.
- `invoke_structure_callback`: Loads an address from a structure and executes the code at that location twice.
- `call_unnamed_routine`: Calls another routine and returns.
- `execute_callback_list`: Iterates through a list of function pointers stored in a structure and executes each one.
- `swap_global_word`: Stores the current value of a global word into a local variable and replaces it with a new value provided in a parameter.
- `call_graphics_function`: Retrieves a function pointer from a table and executes it using a value retrieved from memory.
- `shuffle_buffer_data_2`: Performs a nested loop to swap bytes within a buffer using values from a lookup table.
- `shuffle_buffer_data`: Performs a nested loop to swap bytes in a buffer using a lookup table and toggles a bit in a structure.
- `shuffle_data_buffer`: Performs a complex shuffle of bytes within a buffer by swapping values retrieved from a lookup table.
- `perform_nested_shuffles`: Executes nested loops to swap bytes in a buffer using values from a translation table and toggles a bit in a structure upon completion.
- `toggle_byte_bit`: Toggles the value of a specific byte at an offset from a pointer using a bitwise XOR operation.
- `iterate_string_and_draw_text`: Iterates through a null-terminated string, searching for specific characters and updating a global value based on data retrieved from a memory structure.
- `set_bit_in_byte`: Checks if a specific bit is set in a global byte and then sets a different bit in that same byte based on the input parameter.
- `clear_bit_in_byte`: Checks if a specific bit is set in a global byte and then clears a different bit in that same byte based on the input parameter.
- `assign_global_pointers`: Assigns multiple parameters to global variables and returns a pointer.
- `reset_global_values`: Sets a global variable to zero and then assigns its own address to itself.
- `set_global_one`: Sets a global variable to one and then sets it to zero.
- `clear_global_byte`: Sets a global variable to zero.
- `copy_table_entry_2`: Copies a block of eight words from one location in a table to another location in the same table.
- `copy_and_execute_table_data`: Copies a block of eight words within a table and then executes a function via an indirect call using a calculated offset from a global variable.
- `execute_indirect_calls_4`: Calculates an offset from a global variable to access a table, executes a function via a pointer from that table with a calculated argument, and clears two words in the table.
- `assign_and_parse`: Assigns multiple parameters to global variables and parses a string to identify and process format specifiers and control characters.
- `parse_format_string_8`: Parses a string to identify and process format specifiers and control characters.
- `copy_rect_buffer_blocks_2`: Calculates dimensions and offsets based on a pointer to a structure and performs a block memory transfer using a calculated destination and source.
- `manipulate_string_buffer_elements`: Locates a specific character within a string buffer by scanning backward from a given offset, temporarily null-terminates the string, and calls another routine with calculated parameters.
- `read_string_buffer`: Iterates through a buffer pointed to by a global variable to find non-zero bytes.
- `clear_global_byte_2`: Checks if a global byte is zero, and if not, copies its value to a local variable before setting the global byte to zero.
- `cleanup_system_resources`: Performs extensive teardown by calling various cleanup routines, resetting fields in a structure, and invoking multiple function pointers stored within global structures.
- `execute_indirect_calls_2`: Loads a pointer from a table and executes three function pointers stored at specific offsets within that structure.
- `search_string_table_4`: Iterates through a table of pointers to null-terminated strings, performing a case-insensitive comparison against a target string.
- `clear_global_variable`: Sets a global variable to zero twice.
- `initialize_clipped_bitmap`: Checks if a specified rectangular area or pointer-based structure falls within certain bounds and initializes a bitmap.
- `validate_and_populate_buffer`: Performs boundary checks on two sets of coordinate or dimension data before populating a stack-allocated array with a calculated integer value.
- `update_buffer_and_struct`: Calculates a value using a loop, writes it to a memory buffer, and updates several fields in a structure based on a global variable.
- `copy_rect_buffer_blocks`: Performs a block memory copy or transfer if a source and destination rectangle or certain boundary limits are valid.
- `copy_rectangular_block`: Copies a rectangular block of memory from a source to a destination by iterating through rows and columns.
- `nop_padding_sequence_4`: This routine is empty and consists of non-functional instructions used as padding within the executable segment.
- `fetch_table_value`: Retrieves a value from a table using an offset and checks a bit in a structure before calling another routine.
- `fetch_and_process_data`: Retrieves a value from a table using an offset, checks a bit in a structure, and calls another routine with the retrieved value.
- `update_bit_state`: Performs bitwise shifts and logical operations on three consecutive global bytes to update their state.
- `copy_memory_block_fast_2`: Copies a block of memory from a source address to a destination address using a loop that transfers five words at a time.
- `copy_memory_block_fast`: Copies a block of memory to a destination using a loop that transfers five words at a time.
- `store_value_and_increment_index`: Stores a 32-bit value into a table at an offset determined by a global variable, then increments that global variable modulo sixteen.
- `update_input_counters_2`: Monitors hardware input registers and increments or decrements various counters in a structure based on state changes and bitmask conditions.
- `update_joystick_state_and_bitmask`: Modifies a hardware register based on bit patterns from an input value and updates a table or a global index using a loop.
- `decode_copy_byte_sequence`: Reads a length-prefixed byte sequence and copies the referenced bytes into a destination buffer, decoding several escape codes to repeat or extend copies.
- `swap_word_bytes`: Takes a 16-bit value and returns a new value where the high and low bytes have been swapped.
- `rotate_word_bits`: Performs a complex bit rotation and byte swapping on a 32-bit or 16-bit input value.
- `multiply_and_scale_halves`: Multiplies the high and low sixteen-bit words of a thirty-two-bit value, masks the result to sixteen bits, and multiplies that product by five.
- `distribute_buffer_data`: Copies data from a source buffer into five distinct destination memory regions using nested loops.
- `fill_memory_blocks`: Uses nested loops to copy data from a source buffer into five different destination memory regions offset by large increments.
- `scatter_bytes_to_buffers_2`: Copies bytes from a source pointer to multiple destination buffers using a nested loop structure where the base destination address is derived from a global.
- `fill_memory_blocks_offset`: Copies data from a source buffer into five different destination memory regions, where the base destination address is derived from a global variable.
- `divide_shift_register_output`: Reads a value from a shift register routine, divides it by a computed range derived from the input, and returns the quotient and remainder combined into a single value.
- `evaluate_status_code`: Calls several functions to retrieve values and returns a specific integer code based on the results and comparisons of those values.
- `copy_and_expand_byte_stream`: Copies bytes from an input buffer to an output buffer for a length given in the input, interpreting special codes to repeat or invert runs, and returns the original length.
- `release_interface_resources`: Sets a global variable to zero and calls three functions retrieved from a pointer structure.
- `process_command_input_and_status`: Parses a string into tokens and performs conditional logic based on status codes and input parameters.
- `check_and_execute_callback`: Calls a function through a pointer and, if the result is non-zero, calls another function through a different pointer.
- `check_and_execute_callback_alt`: Executes a function via a pointer and calls a second function if the first one returns a non-zero value.
- `return_zero_constant`: An empty stub that performs no operations and returns zero to the caller.
- `format_string_buffer`: Retrieves a string from a table and copies it into a local stack buffer after performing several conditional checks and function calls.
- `initialize_command_and_message_nodes`: Initializes a series of command structures and message nodes by allocating memory and populating them with function pointers and data.
- `execute_indirect_stack_pointer_offset`: Retrieves a function pointer from an offset relative to a value currently on the stack and executes it.
- `return_immediately_stub_2`: The routine consists of a single return instruction and performs no work.
- `return_immediately_3`: This routine is empty and performs no operations before returning to the caller.
- `check_entity_flag_and_process_2`: Retrieves a pointer from another routine, checks a specific bit in the second byte of the pointed-to structure, and recursively processes the third field of that structure.
- `parse_format_string_7`: Parses a format string containing tokens like #, +, -, *, leading zeros, ., l, h, p, x, and digits, advancing a pointer through the string while reading values from a parameter list.
- `execute_format_specifiers`: Iterates through a character string, skipping doubled percent signs and calling another routine to process individual format specifiers found after a single percent sign.
- `iterate_and_call_structures`: Iterates through a list of structures based on a global value, checking a bit in a field and calling another routine if the condition is met.
- `execute_indirect_calls_3`: Checks a global pointer and executes several functions at various offsets relative to that pointer.
- `initialize_global_state`: Sets a global variable to a series of specific values and performs conditional logic based on register contents.
- `transform_string_case`: Iterates through a null-terminated string and modifies each character by subtracting thirty-two if a specific bit is set in a global lookup table indexed by the character value.
- `modify_string_bytes`: Iterates through a null-terminated sequence of bytes, modifying each byte by adding thirty-two if a specific bit is set in a lookup table, and stores the result back in the original location.
- `copy_modified_string`: Iterates through a sequence of bytes, copying them to a destination while conditionally modifying the value based on a bit check in a table.
- `nop_sequence`: This routine consists of three instructions that perform no operation and return immediately.
- `compare_case_insensitive_strings_2`: Compares two null-terminated byte strings character by character, treating uppercase and lowercase letters as equal.
- `append_string_to_buffer`: Locates the null terminator of the first buffer and begins copying bytes from the second buffer into that position.
- `search_string_for_byte`: Iterates through a null-terminated string to find a specific byte.
- `search_string_for_byte_2`: Iterates through a null-terminated string to find the first occurrence of a specific byte.
- `find_last_occurrence_5`: Iterates through a null-terminated string to find the last occurrence of a specific byte.
- `nop_sequence_padding_2`: This routine consists of three instructions that perform no operation and serve as padding within the executable segment.
- `convert_and_reverse_buffer`: Repeatedly calls another routine to convert values into ASCII characters and stores them in a buffer.
- `convert_octal_to_ascii_2`: Converts an integer into a null-terminated ASCII string of octal digits by extracting values through bitwise shifts and masking.
- `convert_octal_to_ascii`: Converts an integer into ASCII digits by extracting octal values using bitwise shifts and masking.
- `convert_nibbles_to_ascii`: Expands a value by looking up nibbles in a table and writes the resulting bytes to a destination in reverse order.
- `unpack_nibbles_and_reverse`: Expands a packed nibble value into bytes using a lookup table and writes them to a destination in reverse order.
- `format_fixed_point`: Converts a numeric value into an ASCII string, adding a negative sign if the value is less than zero and performing a string reversal to order the digits correctly.
- `format_fixed_point_string`: Converts a 32-bit fixed-point value into a null-terminated ASCII string, including a negative sign if the value is less than zero.
- `convert_to_ascii`: Transforms a value into a null-terminated ASCII string by converting digits and then reversing the character sequence in memory.
- `parse_signed_decimal_string`: Converts a pointer to a string containing a signed decimal integer into a 16-bit signed value stored at a destination pointer.
- `increment_pointer_and_store_byte`: Increments a global pointer and stores a byte from a parameter into the new location, then increments the pointer again.
- `set_global_pointer_2`: Assigns a supplied address to a global variable, invokes a routine to process a string, and stores a zero byte at the address held by the global variable.
- `copy_memory_block_3`: It copies a given number of bytes from a source address to a destination address.
- `destroy_and_free_linked_list`: Iterates through a linked list by following pointers, calling a destructor function located at a fixed offset within each node before freeing the memory.
- `traverse_linked_list_with_error_check`: Retrieves a pointer from a linked structure and recursively follows the next entry.
- `iterate_linked_list_status_flags`: Traverses a global linked list of structures, checking for a specific bit pattern in a status byte and a non-zero difference between two integer fields.
- `execute_conditional_callback_4`: Recursively calls itself if a global pointer is non-null, then clears that pointer before invoking a callback via a table offset.
- `execute_recursive_callback_adjustment`: Invokes a callback via a pointer in a global table and, if a global flag is set, recursively calls itself.
- `invoke_callback_with_offset`: Invokes a callback through a pointer in a global table and, when a global flag is set, recursively calls itself first.
- `set_structure_bitmasks`: Writes a single byte bitmask and two word-sized bitmasks to specific offsets within a provided structure.
- `reset_and_cleanup_context`: Checks if a specific field in a structure is non-zero to trigger a callback, then overwrites two fields in that structure with fixed values.
- `invoke_method_if_present_2`: It checks whether a global pointer is non-null and calls another routine when it is, then invokes a function pointer stored within a structure.
- `copy_and_execute_stack_data`: Copies a block of data from a global variable to a local stack buffer and then executes a routine using a pointer retrieved from a global variable.
- `multiply_fixed_point_values`: Multiplies two 32-bit fixed-point numbers by calculating the product of their high and low 16-bit components and summing the cross-products.
- `divide_32bit_fixed_point`: Performs 32-bit division of a dividend in d0 by a divisor in d1 using a normalization process to align the decimal point.
- `allocate_and_initialize_node_2`: Allocates a memory block and, if successful, sets two specific bytes at an offset and stores a provided dword at a third offset.
- `create_message_node_2`: It obtains a global variable and calls another routine through a table.
- `fetch_table_entry_pointer`: Checks if an index is within the bounds of a global table and verifies that the entry at that index is non-zero.
- `dispatch_conditional_global_callback`: Invokes a routine via a pointer in a table and, if specific bits are set in the result and a global pointer is non-null, executes the routine pointed to by that global pointer before clearing it and calling another routine.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `process_data_structure`: It compares values within a data structure, clears memory in a loop, and executes several routines through pointers stored in a global table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine_3`: This routine performs a single return instruction to exit the current execution flow.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_3`: This routine is empty and contains no executable instructions.
- `initialize_item_data`: Calls several routines to process data and copies sixteen words of data from a buffer into a destination table.
- `process_item_table_updates`: Fills a table with specific values and iterates through a list to execute conditional callbacks and indirect functions based on table entries.
- `process_itemtype_data`: Iterates through a table to execute conditional callbacks and indirect functions based on the number of entries specified in a local variable.
- `initialize_item_and_entity_tables`: Populates multiple data tables by iterating through memory blocks and performing bitwise operations and pointer assignments.
- `initialize_data_tables`: Clears several memory tables and arrays by writing zero to them and populates a specific table with incremental byte values.
- `initialize_data_tables_2`: Populates several tables by reading values from specific memory locations and performing a series of function calls with various parameters to initialize global state.
- `execute_indirect_calls_5`: Performs a series of complex indirect function calls using values retrieved from a global variable and various tables.
- `set_global_values`: It performs a series of writes to a global variable with various constant values.
- `execute_indirect_call_2`: Sets a global variable to zero and performs an indirect call using an address offset from that global variable.
- `execute_indirect_jump`: Performs a series of routine calls and then executes an indirect jump using a value from a global variable.
- `copy_table_data`: It calls several routines through a table and then copies sixteen words of data from a provided buffer into a destination table using a loop.
- `extract_and_calculate_values`: It extracts several fields from a provided pointer, iterates through a list of pointers, and performs arithmetic operations using values from a global table.
- `call_subroutine`: This routine calls another subroutine and then returns.
- `iterate_string_and_call_subroutine`: Iterates through a null-terminated string, calling a subroutine for each character and performing pointer arithmetic to advance through the sequence.
- `tokenize_string_input`: Parses a source string into a list of individual tokens by splitting on whitespace or handling quoted substrings.
- `nop_instruction`: The routine consists of a single instruction and is currently unreached.
- `calculate_product_offset`: It calculates a value by multiplying three values retrieved from specific offsets within a provided memory block and adding a constant.
- `execute_conditional_callback_2`: It checks a value in a table to determine whether to call another routine or execute a function pointer, then clears the checked value.
- `process_entity_interaction_logic`: Evaluates interaction states between entities based on provided parameters and updates internal status flags.
- `update_linked_list`: Compares a value in a table and, if different, clears a linked list and updates a data structure through a series of function calls.
- `test_bit_and_return`: Checks if a specific bit is set in a byte and returns a value based on that condition.
- `modify_bit_via_pointer`: Calculates an address using a pointer and an offset, then performs a bitwise XOR operation on a byte at that location.
- `parse_command_string_2`: Parses a space-delimited string into an array of pointers to individual tokens.
- `call_indirect_function_2`: Checks a global flag and, if zero, calls a function pointer retrieved from a table offset by a value in a parameter.
- `copy_table_entries`: Copies eight consecutive words from one location in a table to another and performs arithmetic calculations using the values.
- `call_self`: An empty routine that calls itself.
- `call_conditional_function`: Compares the lower bits of two parameters and calls a function pointer from a global table if they do not match.
- `sync_table_entry`: Checks a table entry against a value and performs a list clearing and data structure update if they do not match.
- `extract_bitfield`: Reads specific bits from a memory location calculated from two parameters and packs them into a returned value.
- `null_routine`: The routine is empty and contains no executable instructions.
- `execute_multiple_callbacks`: Clears a global variable and calls three different functions using pointers retrieved from a table.
- `call_repeated_function`: Calls a routine with a specific constant and then calls a function pointer from a table twice.
- `update_data_structure`: Compares a value in a table, clears a linked list of entries if they differ, and then calls a complex routine to update a value.
- `process_event_logic`: Evaluates a pointer from a table to determine whether to execute a sequence of nested routine calls or to call a specific routine with modified arguments.
- `setup_environment_parameters`: Executes a series of setup routines with fixed addresses and bitwise-masked parameters if a global variable is zero.
- `process_conditional_loop`: Checks a global variable and enters a loop that calls another routine repeatedly until a condition is met.
- `check_global_flag`: Returns a global variable after conditionally calling another routine and updating the variable if a specific condition is met.
- `process_event_loop`: Iterates through a loop using a parameter and a global variable, performing various function calls and bit manipulations based on specific loop indices.
- `process_argument_list`: Iterates through a list of pointers, calling routines based on the values found in the list and storing the results.
- `process_event_table_entries`: Iterates through a linked list of data structures to execute conditional logic based on a type byte.
- `parse_string_and_update`: Iterates through a null-terminated string, performing calculations based on string length and updating a table with derived values.
- `reset_system_state`: If a global variable is zero, it calls a routine to clear specific bytes and then executes a loop to update a table.
- `empty_routine_4`: This routine contains no executable instructions and performs no operations.
- `initialize_audio_resource_tables`: Sets up a series of pointers in a table to track various audio resource segments and initializes a global state.
- `call_setup_routine`: Calls a specific routine twice with a provided parameter and a fixed address.
- `update_bitmask_state`: performs bitwise operations using a parameter to modify a global variable and returns a boolean result based on a bitwise check.
- `swap_word_value`: stores the current value of a global variable in a local variable, updates the global variable with a new value, and returns the original value.
- `call_subroutine_2`: calls another routine and then returns.
- `parse_resource_data_stream`: Iterates through a data stream to identify and process tagged resource blocks.
- `execute_empty_routine`: This routine performs no operations and returns immediately.
- `process_string_data_2`: This routine iterates through a null-terminated string and executes several function calls via a pointer table for each character.
- `unpack_compressed_data`: Identifies a data block by checking for specific magic signatures and performs bitwise decompression or bit-shifting on the payload.
- `pass_through_value`: Calls another routine and returns its result.
- `call_external_routine`: Executes a specific routine located at a fixed address.
- `execute_conditional_callback_3`: Checks a value in a table and calls a function pointer if a specific bit is not set, then clears that table entry.
- `traverse_string_recursively`: Recursively processes a null-terminated string.
- `nop_padding`: This routine is empty and consists of a single no-operation instruction.
- `compare_and_update_table`: Compares two tables of words bit by bit and updates a destination table with modified values until they match, potentially calling other routines during the process.
- `process_dialogue_or_event_branching`: Evaluates conditional logic based on input parameters to determine branching paths, potentially copying strings from a data table into a local buffer.
- `pass_through_value_2`: The routine calls another routine and returns its result.
- `decrement_table_values`: The routine iterates through a table of pointers to extract values, decrements them in a loop, and then clears specific fields in the referenced structures.
- `process_bitwise_flag`: The routine performs a bitwise XOR operation on a byte at a calculated memory location using a bit shift determined by a value in a table.
- `parse_string_arguments`: Parses a space-delimited string into an array of pointers, supporting quoted substrings.
- `call_indirect_function_3`: Checks a value at an offset from a pointer and, if it is zero, calls itself recursively before executing a function pointer stored in a table.
- `clear_memory_block`: Iterates through a memory range defined by a pointer and a count, setting each byte to zero.
- `move_memory_block`: Copies a specified number of bytes from one memory location to another, handling both forward and backward copy directions based on the relative positions of the pointers.
- `execute_nested_calls`: Calls two other routines sequentially and returns the result of the first call.
- `update_data_table`: Copies a series of word-sized values from one table to another based on an index, then performs arithmetic calculations using the updated values.
- `clear_global_variables`: Sets two global variables to zero.
- `initialize_engine_structures`: Initializes a primary data structure by calling a constructor and populating it with various pointers, offsets, and configuration values.
- `process_nested_callbacks`: Checks a global variable and executes a function pointer from an offset within it, followed by recursive calls and several other function pointer executions based on values in a table.
- `call_table_function`: Calculates an offset using a global variable to fetch two values from a table, passes them to another routine, and then clears two words in that table.
- `execute_offset_function`: Executes a function pointer located at a specific offset relative to a global variable.
- `call_double_function`: Calls a routine with two parameters and then executes a function pointer twice using a value retrieved from a table.
- `empty_routine_5`: This routine contains only four instructions and no visible logic in the provided pseudo-code.
