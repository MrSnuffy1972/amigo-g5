# Game 849 (push-over (1992)(ocean)(m4)[cr crack inc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00849). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1804416 top-level bytes in 217 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 17 raw-word lines |
| Functions named and described | pass, 283 of 283 named (0 author symbols), 283 of 283 described (86 by rule, 155 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 368 of 368 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5133 of 5142 instructions identical in start and length, 3 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 89170 |
| code | 16612 |
| data | 1484508 |
| padding | 57966 |
| unused | 156160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SetMap | 13 | 653 | 31 |
| EndCLI | 2 | 117 | 2 |
| LoadWB | 12 | 538 | 30 |
| Disk-Validator | 1 | 386 | 1 |
| usa0 | 1 | 13 | 1 |
| usa2 | 1 | 3 | 1 |
| gb | 1 | 4 | 1 |
| Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:ANT | 3 | 235 | 8 |
| Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:PATCH | 2 | 86 | 4 |
| Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:INTRO | 1 | 84 | 5 |
| icon.library | 10 | 1375 | 109 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 15 | 1040 | 76 |
| Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 500 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 47 | 1 |

## Cases handled

- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:MUSIC/COLIN.MOX: high-entropy data file (7.98 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:MUSIC/SFX.MOX: high-entropy data file (7.97 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:MUSIC/OPTION.MOX: high-entropy data file (7.95 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:DATA/CC4.HMX: high-entropy data file (7.97 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:DATA/TITLE.RUX: high-entropy data file (7.97 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:DATA/CC2.HMX: high-entropy data file (7.98 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:DATA/ANT47_65.ANX: high-entropy data file (7.95 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:DATA/CC0.HMX: high-entropy data file (7.97 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:DATA/DOMINOE1.SPX: high-entropy data file (7.89 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:DATA/ANT31_46.ANX: high-entropy data file (7.95 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:DATA/ANT00_16.ANX: high-entropy data file (7.95 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:DATA/THEME.DAT: high-entropy data file (7.90 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:DATA/CC5.HMX: high-entropy data file (7.97 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:DATA/CC3.HMX: high-entropy data file (7.97 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:DATA/CC1.HMX: high-entropy data file (7.98 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:DATA/ANT17_30.ANX: high-entropy data file (7.95 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:DATA/DOMINOE0.SPX: high-entropy data file (7.94 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:THEMES/OPTION.BCX: high-entropy data file (7.94 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:MUSIC/AZTEC.MOX: high-entropy data file (7.95 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:MUSIC/ELECTRO.MOX: high-entropy data file (7.96 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:MUSIC/JAPANESE.MOX: high-entropy data file (7.96 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:MUSIC/DUNGEON.MOX: high-entropy data file (7.96 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:MUSIC/CASTLE.MOX: high-entropy data file (7.96 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:MUSIC/SPACE.MOX: high-entropy data file (7.96 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:MUSIC/GREEK.MOX: high-entropy data file (7.97 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:MUSIC/PRIZE.MOX: high-entropy data file (7.93 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:MUSIC/SFX.MOX: high-entropy data file (7.97 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:MUSIC/MECHANIC.MOX: high-entropy data file (7.97 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:MUSIC/OPTION.MOX: high-entropy data file (7.95 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:MUSIC/TOXCITY.MOX: high-entropy data file (7.96 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI01.PIX: high-entropy data file (7.91 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI06.DLX: high-entropy data file (7.96 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI09.PIX: high-entropy data file (7.92 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI04.DLX: high-entropy data file (7.95 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI02.DLX: high-entropy data file (7.89 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI06.PIX: high-entropy data file (7.91 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI00.DLX: high-entropy data file (7.97 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI04.PIX: high-entropy data file (7.91 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI02.PIX: high-entropy data file (7.91 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI00.PIX: high-entropy data file (7.91 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI07.DLX: high-entropy data file (7.97 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI05.DLX: high-entropy data file (7.97 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI08.PIX: high-entropy data file (7.91 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI03.DLX: high-entropy data file (7.97 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI07.PIX: high-entropy data file (7.91 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI01.DLX: high-entropy data file (7.97 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI05.PIX: high-entropy data file (7.91 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI03.PIX: high-entropy data file (7.90 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:DATA/ANI09.DLX: high-entropy data file (7.97 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:THEMES/MECHANIC.BCX: high-entropy data file (7.86 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:THEMES/OPTION.BCX: high-entropy data file (7.94 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:THEMES/TOXCITY.BCX: high-entropy data file (7.94 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:THEMES/AZTEC.BCX: high-entropy data file (7.93 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:THEMES/ELECTRO.BCX: high-entropy data file (7.96 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:THEMES/JAPANESE.BCX: high-entropy data file (7.95 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:THEMES/DUNGEON.BCX: high-entropy data file (7.95 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:THEMES/CASTLE.BCX: high-entropy data file (7.95 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:THEMES/SPACE.BCX: high-entropy data file (7.95 bits/byte), classified as packed data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 2 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:THEMES/GREEK.BCX: high-entropy data file (7.95 bits/byte), classified as packed data
- runtime segment an address: 53 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 47 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:ANT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- push-over (1992)(ocean)(m4)[cr crack inc](disk 1 of 2)/Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:INTRO: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:INTRO

## Copy protection found

- none found

## Functions


### SetMap

- `parse_command_line_arguments`: Parses a command-line argument into individual tokens, converting escape sequences like asterisk-n to newlines and asterisk-e to escape characters, then stores the resulting token array and its count in a global structure.
- `restore_patch_state`: Loads a pointer from a global structure and stores the incoming parameter into a slot within the pointed-to structure, then returns the previously stored value.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `invoke_optional_handle_routines`: Calls a routine with each of several global handle values when they are set, passing one global by reference, then always calls it with a fixed handle and the incoming argument.
- `store_arguments_and_call_helpers`: Stores its two arguments into two global variables, then calls one helper with a fixed set of arguments that includes a global address and constants.
- `resolve_keymap_name_and_open`: Resolves a keymap name from icon tool types, command-line arguments, or a default value, then converts the string to lowercase.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `pass_params_to_subroutine`: Calls another routine with a global value, its parameter and the address of an eight-byte stack slot, then restores the caller's registers and returns.
- `call_global_library_function`: Loads a library base from a global variable at a fixed offset, forwards the incoming arguments into registers, calls the library function, restores the saved registers, and returns.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### EndCLI

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_command_line_value`: Copies the 32-bit value from one register into another and then branches into the middle of another routine.

### LoadWB

- `parse_command_line_tokens`: Opens dos.library, then parses a command-line string by splitting it into tokens, handling quoted sections and backslash escape sequences, storing the resulting token pointers in a table.
- `restore_patch_state_2`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `store_args_and_call_loader`: Reads two arguments from the stack, stores them into two global variables, then pushes a fixed set of constants and two zero longs before calling a library routine and finally calls the workbench loader with a single constant.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte and returns minus one when the first is lower, one when it is higher, and zero when they match.
- `build_linked_list_from_table`: Walks a table of pointer and value pairs, allocating a fixed-size node for each entry through a memory allocation call and storing the looked-up value in it.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `validate_and_copy_table`: Checks whether the register holds a valid pointer and whether the byte at offset 20 of the pointed-to structure equals the decimal value 50, then compares that structure against a global table byte by byte.
- `write_period_and_spaces`: Writes the period character to a byte at offset 16 of the structure in a2, then writes five space characters to the following five bytes, and finally branches to the code at offset 92.
- `compare_byte_sequence`: Copies the pointer from a2 into a0, loads the address of a table into a1, returns immediately if the table's first byte is zero, otherwise compares bytes from the source and table and steps both pointers forward until they match or the table ends.
- `load_pointer_and_return`: Loads a pointer from a relative offset into a register, restores two registers from the stack, and returns.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_dispatch_2`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_reschedule_2`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_other_routine_with_stack`: Calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `call_global_and_checksum_buffer_2`: Invokes the function pointer stored in a global variable, then computes a rolling checksum over a 140-byte stack buffer until a null terminator is found.
- `store_byte_and_advance`: Stores the byte in d0 through the pointer in a3, advancing the pointer, and returns.

### Disk-Validator

- `initialize_structure_and_buffer`: Initializes a global structure by storing a computed value at offset 4 of the block pointed to by a1, writes two ASCII digit characters into a buffer at offsets 10 and 11 indexed by that value, decrements two counters in the structure, zeroes a flag, and then calls a library routine repeatedly while clearing and filling an array of 32-bit words.

### usa0

- `store_registers_via_pointers`: Stores the 16-bit value from the caller's a4 register into the structure at offset 256 pointed to by a1, then stores the 16-bit value from d4 into a global variable at offset 46, and returns.

### usa2

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### gb

- `noop_register_operations`: Performs a no-op increment and decrement on the d0 register and the value pointed to by a6, then returns.

### Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:ANT

- `validate_and_copy_memory_block`: Verifies a 4-byte magic number at the start of an allocated memory block, then copies a block of 32-bit words from a source to a destination while aligning both pointers to even addresses.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a condition on a global mask is met, then reads a byte from a table entry and uses it as an index into a global mask to extract a bit, calling another routine if a borrow condition is detected.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: Masks a word in d0 with a word in d6 and, if a byte in d7 is less than a byte in d1, calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `extract_and_adjust_registers`: Extracts two bytes from a table pointed to by a3 into d6, adjusts the a3 pointer and d1 by a computed amount, loads the constant 16 into d7, and returns.
- `fill_figure_table`: Reads a count from a routine, decrements it, and fills a stack buffer with that many values, then iterates over a key list writing bit masks, derived words, and offset bytes into a table at offsets 0, 32, 33, and 34 for each matching entry.
- `free_memory_block_if_valid`: When the pointed-to value is 10044, it zeroes successive fields, walks a linked list to compute a pointer, and merges it into a table, otherwise it frees the memory block and returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:PATCH

- `unpack_bitstream_into_buffer`: Reads bits from a packed stream and writes literal bytes or backward copies into a destination buffer, refilling the bit buffer as needed, then returns.
- `sweep_word_counter_minus_one`: Writes each 16-bit count value to the color register in turn over one full wrap of the counter, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:INTRO

- `initialize_system`: Initializes the system by calling another routine, stores the address of a global variable in two consecutive longwords, and calls that routine again before returning.
- `shift_and_copy_bytes`: Loads a 32-bit value from memory pointed to by a0, shifts it right by one bit, advances a pointer in a3 by the high byte, and copies a number of bytes backward from that pointer, repeating until the pointer reaches its starting address.
- `test_d5_then_return`: Tests whether the d5 register shifted right by one is nonzero, then returns in either case without doing anything else.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `shift_and_load_bit`: Shifts the 32-bit value in d5 right by one bit, carrying the old low bit into the high bit of the next 32-bit word loaded from the memory pointed to by a0, and repeats this for the number of iterations specified by d0, then returns.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `link_library_and_call_entry`: When the library flag is clear it links the library into a list and calls the library entry point, otherwise it sets bit 3 of a status byte and returns zero.
- `set_library_flag`: Sets bit 3 of a byte at offset 14 in the library structure and returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_file_and_store_pointers`: Stores two pointers into a structure, then calls a library function to open a file.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `empty_placeholder`: Contains a single instruction that does nothing and returns immediately.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `manage_icon_chain_3`: Saves two registers on the stack, calls another routine with the same first argument, restores the stack, and returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `save_regs_call_handler`: Saves registers a0 and a6 on the stack, calls a routine at offset 192 of the library base in a6, restores both registers, and returns.
- `save_registers_call_routine`: Saves registers a0 and a6 on the stack, calls another routine, restores the saved registers, and returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `handle_icon_change`: Copies the two arguments from the caller's stack frame into registers, calls a routine with the first argument, and branches to the cleanup code when the result is zero.
- `call_other_routine_3`: Loads two addresses from the caller's stack frame into registers and calls another routine, then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `noop_instruction_only`: Consists of a single ori.b #0, d0 instruction that performs no operation, then the routine ends without returning.
- `propagate_icon_structure_change_2`: Reads two pointers from offsets 8 and 12 of the structure in a6, calls a routine with the first pointer, and if the result is nonzero it copies ten longs from the stack buffer to the structure at offset 96, copies eight more fields by fixed offset, then calls a routine with the second pointer and the result, and finally calls another routine with the second pointer and the first result.
- `copy_structure_fields`: Copies ten longs from one structure to another via incrementing pointers, then copies eight more fields by fixed offset and stores the destination base pointer into a linked field.
- `return_zero_stub`: Restores registers d2-d4 and a2 from the stack, restores the frame pointer, and returns the constant zero in d0.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `propagate_icon_structure_change`: Loads a pointer from offset 16 of the structure, calls a routine with the value at offset 16 and a stack buffer, then calls another routine with a global variable and a third routine with the constant 1005; if the last result is zero it clears d4 and returns, otherwise it calls a fourth routine with the first argument and the result.
- `validate_and_propagate_icon_change`: Validates a library base pointer by checking its magic number and version, then clears two result slots and, depending on a flag, calls a library routine, stores its results, and runs a loop that repeatedly invokes the routine while decrementing a counter.
- `store_two_library_results`: Calls a library routine twice, storing each non-negative result at offset 18 and offset 22 of the structure in a4, and returns early when a call yields negative one.
- `loop_table_and_check_flag`: Loops calling a function pointer from a table, storing each result and advancing the table index until the index reaches four, then checks a flag and calls another routine before returning a value.
- `load_zero_into_d4`: Loads the constant zero into data register d4 and branches into the following routine.
- `empty_routine`: Copies the value from d4 to d0, restores registers from the stack, restores the frame pointer, and returns.
- `retrieve_icon_bitmap_dimensions`: Loads a bitmap pointer from a global variable and calls the icon library to obtain its dimensions, then calls a routine at offset 100 of that bitmap.
- `iterate_icon_table_entries_2`: Walks a null-terminated table of pointers stored in a structure, calling a callback for each entry and accumulating their results, and returns the accumulated value.
- `dispatch_icon_table_handler`: Calls a library routine twice, then branches into a handler depending on whether the flag at offset 30 is set, otherwise it returns.
- `iterate_table_with_callback`: Advances two pointers by four until the first reaches a zero entry, then invokes a callback and walks a null-terminated table of pointers, updating a counter and invoking the callback for each entry.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `advance_and_branch`: Pushes a2 onto the stack, loads a pointer from the stack into a0, advances it by two, and branches into another routine.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `init_pool_block_handler`: Allocates a pool block and, if successful, calls a library vector, then branches into a helper when the result is zero, otherwise it returns.
- `call_and_branch_on_result`: Pushes the constant 168 and the address at offset 140 in a2 onto the stack, saves a2, then calls a routine.
- `return_zero_and_restore_registers`: Resets the return value in d0 to zero and restores the caller's d2, d3, and a2 registers from the stack, then returns.
- `decrement_and_free_block`: Decrements a 16-bit counter stored through a pointer and, when it reaches zero or was already negative, calls a pool routine to free the block and stores the returned pointer at offset 14 of the structure, returning zero on failure.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `allocate_and_process_block`: Allocates a block from the memory pool and, if successful, calls a routine with the block pointer.
- `fall_through_routine`: This routine is empty and does nothing but fall through into another routine.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `call_subroutine_with_params`: Pushes a value from the stack into d2 and calls another routine with a global value, its parameter and the address of an eight-byte stack slot, then restores the caller's registers.
- `compare_byte_and_branch`: Saves d3, d2 and a3, then calls a routine and restores the registers.
- `load_pointer_and_branch`: Loads a pointer from the address in a2 into a3, copies that pointer into d0, and branches when the value is non-zero, otherwise it returns.
- `save_registers_and_call_subroutine`: Saves registers d2 through d5 and d6 onto the stack, loads two addresses from the caller's stack frame, pushes one of them as an argument, and calls another routine.
- `call_and_compute_difference`: Pushes a constant and the value in d2 onto the stack, calls a library routine, and when the result is non-zero computes the difference between the return value and d2, otherwise it returns.
- `check_two_calls_and_return`: Calls a routine with the value in d2 and compares the result with d5, branching away when they differ.
- `increment_d4_if_set`: Tests whether data register d4 is zero and, otherwise, increments it by one before branching into the following routine.
- `return_when_d2_is_zero`: Clears d2 and returns immediately when that value is zero, otherwise it loads d0 with zero and restores the caller's registers before returning.
- `parse_digits_and_copy_string`: Saves registers d2 through d4 and a2 through a4 onto the stack, loads two addresses from the caller's stack frame, and clears a byte at offset 30 of the first structure.
- `call_routine_and_branch_on_result`: Advances a pointer by five, pushes two addresses onto the stack and calls a routine, then returns with d3 set to one when the result is zero and otherwise advances the pointer by three.
- `reset_register_and_branch`: Sets the register d3 to zero and branches into the following routine.
- `parse_digits_and_call`: Multiplies d3 by ten, adds d2, advances a4 and reads a byte there into d0.
- `clamp_value_to_range`: Clamps the caller's value into the range 48 to 57 by rejecting values below 48 and above 57, then subtracts 48 from it and returns the result.
- `load_negative_one`: Loads the value negative one into the data register and returns to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `allocate_pool_block`: Allocates a new block from the 16K memory pool and, on success, stores the value ten at offset 14 of the returned structure and at the caller's pointer, then returns the block pointer.
- `return_zero_value`: Sets the data register to zero, restores the saved registers from the stack, and returns.
- `compare_and_return_zero`: Reads two long values from the stack frame at offsets 8 and 12, pushes them along with a third long value from offset 16 onto the stack, then calls another routine.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `copy_d1_to_d0_2`: Copies the return value from data register d1 into data register d0 and returns, forwarding the result of a library call.
- `copy_d1_to_d0`: Copies the return value from d1 to d0 and returns.
- `rotate_or_divide_value`: If the 32-bit divisor is greater than 65535 it rotates a 32-bit value left by one bit per iteration for sixteen iterations, otherwise it splits the value into high and low halves and computes their remainders and quotients.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_at_pointer`: Stores the byte in the data register at the address in the pointer register and advances that pointer, then returns.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `copy_bytes_until_null`: Loads a0 with a value from the stack and d0 with a value from the stack, then copies bytes from the source in a0 into d0 while comparing each byte and decrementing the pointer until a zero byte is reached, then returns.
- `load_zero_into_d0`: Loads the constant zero into data register d0 and branches into the following routine.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `test_byte_at_pointer`: Loads two pointers from the stack, tests the byte at the first pointer, and decrements that pointer if the byte is not zero.
- `compare_bytes_until_mismatch`: Decrements a counter loaded from the stack and, while it stays non-negative, compares successive bytes from two source pointers and returns zero as soon as a differing or non-zero byte is found, otherwise continuing until the counter is exhausted.
- `load_one_or_255`: Loads the value one into data register d0, and if that value is not greater than zero it instead loads the value 255 before returning.
- `skip_copy_when_null`: Tests the byte at the source pointer in a0 and, while a counter loaded from the stack remains greater than zero, decrements that counter and returns, skipping the copy when the first byte is zero.
- `decrement_pointer_and_copy`: Decrements a pointer in a0 by one and calls the copy routine to transfer bytes from that source to a destination, then returns.
- `copy_bytes_then_clear`: Copies bytes from the source pointer to the destination pointer while decrementing a counter, then clears a byte at the destination when the source byte is zero.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `load_stack_args_and_branch`: Loads three consecutive values from offsets 16, 20 and 24 on the stack into registers d2, d3 and a2, then branches into the following routine.
- `loop_icon_routine_check`: Calls a library routine and, if the result is zero, returns zero; otherwise reads a byte from a structure, scales it, and calls the routine again, repeating until a zero result or a null pointer is reached.
- `return_zero_after_pop`: Restores registers d2-d3 and a2 from the stack, then returns the constant zero in d0.
- `propagate_icon_change_2`: Reads four parameters from the stack frame and, if the fourth is non-zero, calls a library routine and, if that returns non-zero, calls it again with the result, then performs further propagation calls.
- `return_minus_one_after_pop`: Restores registers d2-d5 and a2 from the stack, then returns the constant minus one in d0.
- `load_stack_params_and_branch`: Copies four consecutive stack values from offsets 28, 32, 36 and 40 into registers d2, d3, d4 and a2, sets d6 to zero, then branches into the following routine.
- `call_and_check_result`: Pushes the pool size and a parameter onto the stack, calls a library routine, and if the result is zero or a later check fails it returns minus one.
- `propagate_icon_change_3`: Loops over a chain of structures, forwarding parameters to a library routine and returning minus one if any call fails or a field becomes zero, otherwise advancing to the next structure.
- `return_minus_one_after_pop_more`: Restores registers d2-d6 and a2 from the stack, then returns the constant minus one in d0.

### version.library

- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `store_library_pointers`: Stores the base pointer from one register into a structure at offset 34 and writes two other register values into offsets 38 and 42, then returns.
- `increment_counter_and_clear_flag`: Increments a 16-bit counter at offset 32 from a base pointer and clears bit 3 of a flag byte at offset 42 from the same base pointer, then returns.
- `decrement_counter_and_call`: Sets the return value to zero, decrements a 16-bit counter at offset 32, and if that counter reaches zero and bit 3 of the flag byte at offset 42 is set it calls another routine before returning.
- `decrement_library_refcount_2`: Saves several registers on the stack, swaps in a base pointer from a structure field, decrements a reference counter, and if that counter is not zero it sets bit 3 of a flag byte before returning.
- `call_library_via_pointers`: Saves a long value at offset 38 of the structure in a5, then calls two library functions via pointers at offsets -252 and -210 of a6, computing the second call address from the words at offsets 16 and 18 of a5, and finally returns the saved value in d0.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.

### diskfont.library

- `check_and_set_font_handle_2`: Calls a library function via a pointer at offset -84 of a6, stores the incoming value in a0 to offset 34 of the structure in a6 when the result is nonzero, then calls another library function via a pointer at offset -380 of a6 and returns whether that result was nonzero.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `open_libraries_and_init`: Stores the incoming base pointer into the library structure, opens two libraries by calling their open routines, and on success initializes a newly allocated structure by setting its self pointer, advancing a field by four, clearing a following field, and storing a pointer to it, returning the base pointer otherwise returning zero.
- `compute_address_and_call_functions`: Saves the library base pointer, calls the first library function, then computes an address by subtracting a 16-bit field and adding another 16-bit field from the library structure.
- `remove_empty_nodes_list`: Walks a doubly linked list of nodes starting at offset 42 of the structure in a6, removing each node whose flag at offset 84 is zero by rewriting the surrounding pointers and decrementing a counter at offset 40.
- `unlink_node_if_empty`: Checks the two flags at offsets 38 and 40 of the structure in a6 and, when both are zero, calls a routine at offset 42, then unlinks a node from a doubly linked list by rewriting the surrounding pointers.
- `set_flag_and_return_zero`: Sets bit 7 of the byte at offset 14 of the structure in a6 and places zero in d0, then falls through into the following routine.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_conditional_jump`: Decrements the word at offset 38 of the structure in a6 and returns if it becomes zero, otherwise checks bit 7 of the byte at offset 14 of a6 and jumps to the routine at offset -18 of a6 when that bit is set.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_font_names_into_buffer`: Clears a 16-bit value at the start of the buffer in a0, then, if its length is greater than one and bit 0 of the flag in d3 is set, walks a null-terminated list of font names and copies each name into the destination buffer while recording its length, offset and metadata.
- `load_a1_and_d4_values`: Loads a1 with a value from a global variable and loads d4 with the value one before branching into the following routine.
- `process_string_buffer`: Increments a 16-bit counter, then when a NUL byte is found in the string pointed to by a1 it subtracts the counter from a value in d5 and copies a fixed number of bytes from a source table into a destination buffer, storing the result and a flag into a structure.
- `copy_font_bytes_and_names_2`: Copies font bytes from the source buffer into a destination buffer while writing a header consisting of a 16-bit value, a 32-bit value and two 8-bit values, then advances the destination pointer.
- `open_and_process_font`: Reads a font name from a structure and asks the diskfont library to open it, then scans the font's byte sequence to copy its name into a buffer and records its size and attributes in a font table, repeating for each available font.
- `parse_font_byte_sequence`: Increments a byte counter and scans the source buffer byte by byte until it finds a zero, then subtracts the counter from two stored lengths and copies that many bytes to a buffer at offset 8 of the structure in a4 if the counter does not exceed the first length.
- `copy_font_byte_and_check`: Copies a byte from the source buffer pointed to by a1 to a destination using a3 and d5, then advances both.
- `copy_font_bytes_and_names`: Copies the font's byte sequence into a buffer while writing a header (a 16-bit value, a 32-bit value and two 8-bit values) at the start, then repeatedly reads directory entries and copies font names into a buffer until the source is exhausted, returning the remaining byte count or zero.
- `restore_registers_and_return`: Restores registers d2 through d6 and a2 through a4 from the stack, restores the stack pointer, and returns without modifying any data.
- `compute_font_glyph_offset`: Computes a signed difference between a value at offset 256 of the second structure and a value at offset 4 of the first, scaling it by 64 or negating it on borrow, then subtracts up to seven 32-bit entries from a table selected by bits of the bytes at offsets 258 and 259 of the second structure.
- `compute_xor_checksum`: Combines d5 and d2 into d0, then reads bytes from offsets 6 and 7 in a0 and 258 and 259 in a1, XORing them.
- `subtract_table_entry_by_index`: Subtracts the 32-bit value at the offset selected by d4 into a table of 32-bit values from d2, then repeatedly masks d3 with bits of a byte at offset 259 of the structure in a1 and subtracts the same table entry while the corresponding bit is set, returning when d4 reaches 8 or the mask becomes zero.
- `index_table_by_counter`: Multiplies the value in d4 by four and uses it as an offset into a table of 32-bit values, subtracting the referenced value from d2, then increments d4 and repeats the process until d4 reaches seven, at which point it exits.
- `return_constant_value`: This library routine returns a constant value without using any arguments or registers.
- `select_font_glyph_and_get_pointer`: Copies the first two longwords of a font structure into a local buffer, calls a routine to select a glyph, then reads the width, height and ascent fields from the selected font entry and calls a library vector to obtain a glyph pointer, returning it if the call does not report failure.
- `store_icon_pointer`: Loads a pointer from offset 8 of the structure in a6, calls a routine with it, and stores the returned pointer in a2.
- `load_and_scan_font_glyphs`: Loads a font file from the current directory, allocates a buffer sized from the file's height, and scans its glyph table to find the glyph with the largest metric, then stores glyph metadata (offset, width, flags) into the font's glyph table and updates its counters.
- `prepare_font_scan_entry`: Clears the 16-bit value in a register, loads the byte value 255 into another register, clears a third register, and then branches into the middle of another routine.
- `select_font_glyph_2`: Scans a font glyph table to find the entry with the smallest maximum glyph width, then stores that width and marks the font as loaded by setting bit 1 of its status byte and recording its address.
- `unload_and_clear_font_flag`: Pushes d4 onto the stack, calls the library stub at offset 96, computes a scaled index from d2 to locate a font entry, clears the 16-bit flag at offset 256 of that entry, then adds 4 to the stack pointer and falls through into the following routine.
- `select_font_glyph`: Clears the 16-bit flag at offset 256 of the font entry selected by the index in d2, then searches the font's glyph table for the smallest matching glyph index and, if found, marks that glyph as used and records its address.
- `build_disk_font_table`: Locks a directory, scans its entries for a file whose name starts with the text '.font', loads that file's font data, and builds a glyph table containing each character's metrics and bitmap bytes, returning a pointer to it.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `call_forbid_2`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit_2`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
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
- `load_stack_pointers`: Loads a pointer from the stack at offset 4 into a0, loads a long from the stack at offset 8 into d0, then falls through into the following routine.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `load_stack_pointer_and_zero`: Loads a pointer from the stack at offset 4 into a0, places zero in d0, then falls through into the following routine.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `compare_bytes_via_caller`: Copies the byte at the address in a0 into d0 and compares it with the byte at the address in a1, then branches to the caller's compare routine to finish the comparison.
- `check_byte_at_a0`: Checks whether the byte at the address in a0 is nonzero, returning immediately if it is, otherwise clearing d0 and falling through into the following routine.
- `compare_byte_at_a0_with_a1`: Copies the byte at the address in a0 into d0, compares it with the byte just read from a1, and if they are equal decrements a1 and compares its new byte with the one in a0, returning the all-ones value when the a0 byte is greater.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `load_stack_pointer_and_byte`: Loads a pointer from the stack at offset 4 into a0, loads a byte from the stack at offset 11 into d0, then falls through into the following routine.
- `compare_and_copy_pointer`: Compares d0 with d1 and returns if they differ, otherwise copies a0 into d0 and falls through into the following routine.
- `increment_a0_load_byte`: Increments a0 by one, loads the byte at the new address into d1, and returns if that byte is zero, otherwise clearing d0 and falling through into the following routine.
- `call_and_adjust_pointer`: Pushes d2 and d3 onto the stack, loads d3 from offset 4 of the stack and d2 from offset 19 of the stack, then calls another routine and uses the returned pointer to decrement an address, add d3 to it, and restore the stack before returning.
- `compare_byte_at_a0`: Compares the byte at the address in a0 with d2 and returns if they differ, otherwise loads that address into d0 and falls through into the following routine.
- `decrement_and_check_bound`: Decrements a0 by one, compares it with d3 and returns if a0 is greater than or equal to d3, otherwise places zero in d0, restores the two registers it saved on the stack, and returns.
- `call_and_increment_result`: Saves d2, loads a pointer from offset 8 of the stack, pushes a constant, calls the routine at offset 126, then if its result is nonzero increments that value and returns it, otherwise repeats the call with a second constant and returns the incremented result.
- `increment_returned_value`: This library routine returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `call_other_routine_2`: Saves registers a0 and a1 on the stack, calls a routine whose address is taken from the data segment, restores the stack, and returns without performing any other operation.
- `call_another_routine`: This library routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Push-Over (1992)(Ocean)(M4)[cr Crack Inc](Disk 2 of 2).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_structure_and_fill_tables`: Initializes a global structure by storing a computed value at offset 4, then fills several lookup tables: a sequence of ASCII digit characters, a pair of 16-bit values per entry, a linearly increasing series, and a repeating pattern of three 32-bit words, before returning.
- `update_counter_and_fill_tables`: Increments a 16-bit counter at offset 5810 of the structure in a5, masks it to 9 bits, and stores it back, then looks up a value using that counter and copies a word from a table into offset 2172.
- `clear_and_set_bitmap_flags`: Clears a 1504-byte region of the 60000 memory area, then sets pairs of bits in a bitmap for a range of values, inverting 23 bytes per iteration and advancing through the bitmap until the region is fully processed.
- `wait_for_flag_bit`: Checks bit 14 of the byte at offset 2 in the structure in a6 and loops while that bit is set, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_and_tables`: Fills a table with a repeating 30-word pattern, finds the maximum byte in a source block, and initializes a series of pointer and value pairs in a global structure.
