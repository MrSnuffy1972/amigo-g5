# Game 1868 (Flashback, Delphine / U.S. Gold 1992, Interpol crack): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01868). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

Status, 2 Nov 2026: proofs 1 and 2 pass; proofs 3 and 4 wait for the local-model naming pass, which shares Ryzen's GPU with other agents and is still running. Packed parts: the game itself is an Imploder V6 library-mode executable and the cracktro has its own packer; both depackers are recovered as code and were ported line by line to produce the unpacked units (evidence in the cases below). Unreached routines placed after tables inside gaps are found by a prologue scan (4 found). Notes: B2-1868-RECOVERY-NOTE.md on Ryzen.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3604480 top-level bytes in 178 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | FAIL, 52 of 672 named (0 author symbols), 52 of 672 described (4 by rule, 48 by the local model) |
| Executed addresses (reference run) in recovered, described code | FAIL, 262 of 15148 (1.73%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 29958 of 29959 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3001688 |
| code | 3540 |
| data | 406498 |
| padding | 46322 |
| unused | 146432 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad | 1 | 222 | 21 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:INTERPOL | 2 | 267 | 8 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:INTERPOL#unpacked | 1 | 1200 | 28 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:flashback | 7 | 63 | 1 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:flashback#unpacked | 5 | 27535 | 576 |
| explode.library | 1 | 475 | 30 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:Failat | 2 | 117 | 2 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:endrun | 1 | 117 | 4 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:bootblock | 1 | 10 | 1 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:INTERPOL: cracktro self-loader: PowerPacker-style backward bit stream (offset widths 9,10,11,11) and compact relocations, unpacked by our port of the game's own depacker (14180 -> 33324 bytes): unpacked by tools/ppunpack.py, a line-by-line port of INTERPOL's own decruncher (+1CC shift_byte_buffer) and loader (+2C..+1A0, compact 03EC relocation groups): the 13,496-byte stream is consumed exactly, the 33,076-byte image parses to its last byte (one 32,940-byte code hunk, 83 relocations), and relocated at an address it equals the reference run's chip RAM in 32,543 of 32,940 bytes (the rest are the cracktro's own variables changed while it ran)
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:flashback: Imploder V6 library mode (needs libs/explode.library), unpacked by our port of the game's own depacker (83312 -> 190628 bytes): exploded by tools/explode.py, a line-by-line port of explode.library's own routines (+2FC descriptor, +3CA hunk filler, +44E decruncher): the 82,920-byte packed hunk is consumed to its first byte, the 190,564-byte output is consumed exactly by the hunk filler (5 hunks, 5,288 relocations), and the reference harness runs the result through the protection, intro, title and level 1
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad +9C..A8: data by evidence ("dos.library", taken by the start-up code with pea); 0 instructions decoded there were removed
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad +B8..CA: data by evidence ("intuition.library": inline argument after a call to the open-the-library-named-after-the-call helper); 8 instructions decoded there were removed
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad +FA..10E: data by evidence (" already resident!": inline text after a call to the print-the-text-after-the-call helper); 6 instructions decoded there were removed
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad +116..126: data by evidence ("explode.library": inline argument after a call to the open-library helper); 7 instructions decoded there were removed
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad +132..142: data by evidence (" initialized!": inline text after a call to the print helper); 7 instructions decoded there were removed
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad +14C..15E: data by evidence (" not available!": inline text after a call to the print helper); 7 instructions decoded there were removed
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad +298..2A2: data by evidence ("*** BREAK": text printed on Ctrl-C); 0 instructions decoded there were removed
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:flashback: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- INTERPOL: the Interpol cracktro, run by the startup-sequence before the game. Its code hunk is a self-loader that decrunches a backward bit stream into a hunk image with compact relocations; that depacker was ported line by line, and the unpacked cracktro is recovered as its own unit (INTERPOL#unpacked). Kept, to be ported natively as the intro piece. No trainer in this variant.

## Copy protection found

- Code-wheel check before the logos (read in the code, protection_code_wheel_screen): a random wheel shape and code, a six-character answer compared with a table; three failures end in a deliberate crash (the routine that clears the system pointer). The Interpol crack made the comparison a no-op. Native: removed (no screen).
- No disk checks: every file is read through dos.library by name.

## Functions


### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad

- `initialize_expansion_loader`: The routine sets up a small stack frame and calls a loader function from a table pointed to by a global variable.
- `sub_00A8`: undescribed.
- `sub_00CA`: undescribed.
- `sub_00DA`: undescribed.
- `sub_0126`: undescribed.
- `sub_015E`: undescribed.
- `sub_0180`: undescribed.
- `self_recursive_call_with_callback`: The routine calls itself and then invokes a function pointer stored in a global variable table.
- `sub_01B4`: undescribed.
- `call_indirect_via_table`: The routine calls another routine through an indirect pointer obtained from a global variable and returns a 64-bit value formed by concatenating the two input registers.
- `self`: The routine writes a byte from a register through a pointer and then updates a global variable to point to the next byte in the buffer, wrapping to the start if the next byte would be past the buffer end.
- `sub_022C`: undescribed.
- `sub_0240`: undescribed.
- `parse_string_table_entry`: The routine reads a zero-terminated string from a pointer table, advances the pointer past the string and returns the byte offset of the next entry.
- `sub_0270`: undescribed.
- `sub_02A4`: undescribed.
- `sub_02B8`: undescribed.
- `sub_030C`: undescribed.
- `sub_0318`: undescribed.
- `sub_032C`: undescribed.
- `sub_033C`: undescribed.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:INTERPOL

- `sub_0_0000`: undescribed.
- `decrement_until_zero`: The routine receives a 16-bit value in D1 and repeatedly subtracts one from it until it reaches zero, effectively consuming the counter.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `shift_byte_buffer`: The routine moves bytes within a memory buffer pointed to by a register, using values read from the buffer and from a global variable.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `self_2`: The routine extracts a sequence of bits from a bitstream held in a pair of registers and a byte table, using a bit counter and a signed loop count derived from an input word, and assembles the extracted bits into a 32-bit accumulator.
- `extract_bits_to_word`: The routine extracts a series of bits from a byte stream pointed to by a pointer, using a bit buffer in a register, and assembles them into a 32-bit word in a register.
- `negate_byte_via_pointer`: The routine negates a byte pointed to by a parameter using a register value, ORs a byte pointed to by a pointer with a constant, copies an int from a global variable to a pointer target, and writes a byte derived from a comparison to memory.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:INTERPOL#unpacked

- `sub_0000`: undescribed.
- `sub_034A`: undescribed.
- `sub_03A2`: undescribed.
- `sub_03A4`: undescribed.
- `sub_0400`: undescribed.
- `sub_0402`: undescribed.
- `sub_0404`: undescribed.
- `sub_0472`: undescribed.
- `sub_06BC`: undescribed.
- `sub_0734`: undescribed.
- `sub_0742`: undescribed.
- `sub_076A`: undescribed.
- `sub_0778`: undescribed.
- `sub_07FE`: undescribed.
- `sub_081A`: undescribed.
- `sub_084C`: undescribed.
- `sub_0890`: undescribed.
- `sub_0A20`: undescribed.
- `sub_0A40`: undescribed.
- `sub_0B2A`: undescribed.
- `sub_0B88`: undescribed.
- `sub_0C7A`: undescribed.
- `sub_0CDE`: undescribed.
- `sub_0D7C`: undescribed.
- `sub_0F80`: undescribed.
- `sub_1042`: undescribed.
- `sub_11B6`: undescribed.
- `sub_1206`: undescribed.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:flashback

- `initialize_game_state`: The routine checks a global variable and a flag, then calls another routine from a table and handles an error path.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:flashback#unpacked

- `sub_0_0000`: undescribed.
- `sub_0_0006`: undescribed.
- `sub_0_018A`: undescribed.
- `sub_0_0602`: undescribed.
- `sub_0_060E`: undescribed.
- `sub_0_06A4`: undescribed.
- `sub_0_08C6`: undescribed.
- `sub_0_0C6C`: undescribed.
- `sub_0_0CBA`: undescribed.
- `sub_0_17EA`: undescribed.
- `sub_0_18AA`: undescribed.
- `sub_0_1A34`: undescribed.
- `sub_0_1A46`: undescribed.
- `sub_0_1B72`: undescribed.
- `sub_0_1C04`: undescribed.
- `sub_0_1CB0`: undescribed.
- `sub_0_27FC`: undescribed.
- `sub_0_30DE`: undescribed.
- `sub_0_3140`: undescribed.
- `sub_0_314E`: undescribed.
- `sub_0_3164`: undescribed.
- `sub_0_31E2`: undescribed.
- `sub_0_321E`: undescribed.
- `sub_0_3232`: undescribed.
- `sub_0_3236`: undescribed.
- `sub_0_3240`: undescribed.
- `sub_0_32D4`: undescribed.
- `sub_0_3306`: undescribed.
- `sub_0_333A`: undescribed.
- `sub_0_3366`: undescribed.
- `sub_0_33D4`: undescribed.
- `sub_0_3414`: undescribed.
- `sub_0_347A`: undescribed.
- `sub_0_34A4`: undescribed.
- `sub_0_34CC`: undescribed.
- `sub_0_34F2`: undescribed.
- `sub_0_3516`: undescribed.
- `sub_0_352C`: undescribed.
- `sub_0_3594`: undescribed.
- `sub_0_35D8`: undescribed.
- `sub_0_35DE`: undescribed.
- `sub_0_35E8`: undescribed.
- `sub_0_3626`: undescribed.
- `sub_0_362C`: undescribed.
- `sub_0_3636`: undescribed.
- `sub_0_3666`: undescribed.
- `sub_0_3678`: undescribed.
- `sub_0_3724`: undescribed.
- `sub_0_375C`: undescribed.
- `sub_0_389C`: undescribed.
- `sub_0_38AC`: undescribed.
- `sub_0_3900`: undescribed.
- `sub_0_3BA0`: undescribed.
- `sub_0_3BBE`: undescribed.
- `sub_0_3BF0`: undescribed.
- `sub_0_3C36`: undescribed.
- `sub_0_3DB0`: undescribed.
- `sub_0_3DF2`: undescribed.
- `sub_0_3E24`: undescribed.
- `sub_0_3E40`: undescribed.
- `sub_0_3E74`: undescribed.
- `sub_0_3EE2`: undescribed.
- `sub_0_3F08`: undescribed.
- `sub_0_3F40`: undescribed.
- `sub_0_3FA2`: undescribed.
- `sub_0_3FAC`: undescribed.
- `sub_0_3FC6`: undescribed.
- `sub_0_3FC8`: undescribed.
- `sub_0_3FD8`: undescribed.
- `sub_0_3FE6`: undescribed.
- `sub_0_401C`: undescribed.
- `sub_0_4024`: undescribed.
- `sub_0_4028`: undescribed.
- `sub_0_4088`: undescribed.
- `sub_0_4094`: undescribed.
- `sub_0_41E6`: undescribed.
- `sub_0_41FA`: undescribed.
- `sub_0_420E`: undescribed.
- `sub_0_431C`: undescribed.
- `sub_0_43A0`: undescribed.
- `sub_0_43BA`: undescribed.
- `sub_0_448A`: undescribed.
- `sub_0_44C0`: undescribed.
- `sub_0_454A`: undescribed.
- `sub_0_4598`: undescribed.
- `sub_0_45A6`: undescribed.
- `sub_0_45C2`: undescribed.
- `sub_0_4600`: undescribed.
- `sub_0_4648`: undescribed.
- `sub_0_46B4`: undescribed.
- `sub_0_46EA`: undescribed.
- `sub_0_4706`: undescribed.
- `sub_0_470A`: undescribed.
- `sub_0_4722`: undescribed.
- `sub_0_473A`: undescribed.
- `sub_0_4752`: undescribed.
- `sub_0_476A`: undescribed.
- `sub_0_478C`: undescribed.
- `sub_0_47AE`: undescribed.
- `sub_0_47D0`: undescribed.
- `sub_0_47F2`: undescribed.
- `sub_0_482A`: undescribed.
- `sub_0_4836`: undescribed.
- `sub_0_486A`: undescribed.
- `sub_0_48A0`: undescribed.
- `sub_0_48AE`: undescribed.
- `sub_0_48C4`: undescribed.
- `sub_0_48C8`: undescribed.
- `sub_0_48EA`: undescribed.
- `sub_0_4938`: undescribed.
- `sub_0_4986`: undescribed.
- `sub_0_49E4`: undescribed.
- `sub_0_4A24`: undescribed.
- `sub_0_4A44`: undescribed.
- `sub_0_4AAC`: undescribed.
- `sub_0_4AC2`: undescribed.
- `sub_0_4AD6`: undescribed.
- `sub_0_4ADA`: undescribed.
- `sub_0_4AF2`: undescribed.
- `sub_0_4B06`: undescribed.
- `sub_0_4B3C`: undescribed.
- `sub_0_4B52`: undescribed.
- `sub_0_4B62`: undescribed.
- `sub_0_4B66`: undescribed.
- `sub_0_4C3C`: undescribed.
- `sub_0_4C50`: undescribed.
- `sub_0_4C70`: undescribed.
- `sub_0_4C98`: undescribed.
- `sub_0_4D0A`: undescribed.
- `sub_0_4D30`: undescribed.
- `sub_0_4D6C`: undescribed.
- `sub_0_4D9A`: undescribed.
- `sub_0_4D9E`: undescribed.
- `sub_0_4DB6`: undescribed.
- `sub_0_4DBA`: undescribed.
- `sub_0_4DD2`: undescribed.
- `sub_0_4DD6`: undescribed.
- `sub_0_4E06`: undescribed.
- `sub_0_4E34`: undescribed.
- `sub_0_4E5A`: undescribed.
- `sub_0_4ED8`: undescribed.
- `sub_0_4F4A`: undescribed.
- `sub_0_4F7C`: undescribed.
- `sub_0_4F9E`: undescribed.
- `sub_0_4FD0`: undescribed.
- `sub_0_4FF2`: undescribed.
- `sub_0_5040`: undescribed.
- `sub_0_5050`: undescribed.
- `sub_0_5068`: undescribed.
- `sub_0_507E`: undescribed.
- `sub_0_50B2`: undescribed.
- `sub_0_50C0`: undescribed.
- `sub_0_510E`: undescribed.
- `sub_0_511E`: undescribed.
- `sub_0_5136`: undescribed.
- `sub_0_514C`: undescribed.
- `sub_0_5180`: undescribed.
- `sub_0_518E`: undescribed.
- `sub_0_519A`: undescribed.
- `sub_0_51BA`: undescribed.
- `sub_0_51F0`: undescribed.
- `sub_0_51F4`: undescribed.
- `sub_0_5202`: undescribed.
- `sub_0_5210`: undescribed.
- `sub_0_5214`: undescribed.
- `sub_0_5222`: undescribed.
- `sub_0_5226`: undescribed.
- `sub_0_5234`: undescribed.
- `sub_0_5240`: undescribed.
- `sub_0_5258`: undescribed.
- `sub_0_5264`: undescribed.
- `sub_0_527A`: undescribed.
- `sub_0_5288`: undescribed.
- `sub_0_52A4`: undescribed.
- `sub_0_52DE`: undescribed.
- `sub_0_52F0`: undescribed.
- `sub_0_5300`: undescribed.
- `sub_0_5310`: undescribed.
- `sub_0_5322`: undescribed.
- `sub_0_5332`: undescribed.
- `sub_0_5342`: undescribed.
- `sub_0_5354`: undescribed.
- `sub_0_5364`: undescribed.
- `sub_0_5374`: undescribed.
- `sub_0_538E`: undescribed.
- `sub_0_5392`: undescribed.
- `sub_0_53AA`: undescribed.
- `sub_0_53AE`: undescribed.
- `sub_0_53C6`: undescribed.
- `sub_0_53CA`: undescribed.
- `sub_0_53E8`: undescribed.
- `sub_0_5404`: undescribed.
- `sub_0_541C`: undescribed.
- `sub_0_5420`: undescribed.
- `sub_0_543E`: undescribed.
- `sub_0_545A`: undescribed.
- `sub_0_5476`: undescribed.
- `sub_0_54BE`: undescribed.
- `sub_0_5506`: undescribed.
- `sub_0_554E`: undescribed.
- `sub_0_5596`: undescribed.
- `sub_0_55E2`: undescribed.
- `sub_0_5612`: undescribed.
- `sub_0_5616`: undescribed.
- `sub_0_564A`: undescribed.
- `sub_0_567E`: undescribed.
- `sub_0_56A0`: undescribed.
- `sub_0_56C8`: undescribed.
- `sub_0_56D6`: undescribed.
- `sub_0_56E6`: undescribed.
- `sub_0_570E`: undescribed.
- `sub_0_5730`: undescribed.
- `sub_0_5762`: undescribed.
- `sub_0_5794`: undescribed.
- `sub_0_5810`: undescribed.
- `sub_0_5844`: undescribed.
- `sub_0_5848`: undescribed.
- `sub_0_5868`: undescribed.
- `sub_0_589C`: undescribed.
- `sub_0_58A0`: undescribed.
- `sub_0_58C0`: undescribed.
- `sub_0_58CA`: undescribed.
- `sub_0_58EA`: undescribed.
- `sub_0_5938`: undescribed.
- `sub_0_593C`: undescribed.
- `sub_0_5968`: undescribed.
- `sub_0_59B6`: undescribed.
- `sub_0_59BA`: undescribed.
- `sub_0_59E6`: undescribed.
- `sub_0_5A20`: undescribed.
- `sub_0_5A24`: undescribed.
- `sub_0_5A44`: undescribed.
- `sub_0_5A76`: undescribed.
- `sub_0_5A96`: undescribed.
- `sub_0_5AD0`: undescribed.
- `sub_0_5AF0`: undescribed.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `sub_0_5B18`: undescribed.
- `sub_0_5B68`: undescribed.
- `sub_0_5B6C`: undescribed.
- `sub_0_5BBC`: undescribed.
- `sub_0_5BC0`: undescribed.
- `sub_0_5BD8`: undescribed.
- `sub_0_5C0A`: undescribed.
- `sub_0_5C0E`: undescribed.
- `sub_0_5C3C`: undescribed.
- `sub_0_5C68`: undescribed.
- `sub_0_5D3C`: undescribed.
- `sub_0_5D62`: undescribed.
- `sub_0_5D6C`: undescribed.
- `sub_0_5D9C`: undescribed.
- `sub_0_5DAE`: undescribed.
- `sub_0_5ED0`: undescribed.
- `sub_0_5FBC`: undescribed.
- `sub_0_5FC6`: undescribed.
- `sub_0_6244`: undescribed.
- `sub_0_627C`: undescribed.
- `sub_0_6356`: undescribed.
- `sub_0_637E`: undescribed.
- `sub_0_638A`: undescribed.
- `sub_0_63DC`: undescribed.
- `sub_0_63E0`: undescribed.
- `sub_0_63FE`: undescribed.
- `sub_0_642A`: undescribed.
- `sub_0_647C`: undescribed.
- `sub_0_6480`: undescribed.
- `sub_0_64AC`: undescribed.
- `sub_0_64E2`: undescribed.
- `sub_0_65BA`: undescribed.
- `sub_0_65E8`: undescribed.
- `sub_0_65F2`: undescribed.
- `sub_0_6664`: undescribed.
- `sub_0_6668`: undescribed.
- `sub_0_6694`: undescribed.
- `sub_0_6706`: undescribed.
- `sub_0_670A`: undescribed.
- `sub_0_6736`: undescribed.
- `sub_0_673A`: undescribed.
- `sub_0_6766`: undescribed.
- `sub_0_6792`: undescribed.
- `sub_0_68D2`: undescribed.
- `sub_0_6932`: undescribed.
- `sub_0_6974`: undescribed.
- `sub_0_6978`: undescribed.
- `sub_0_69A6`: undescribed.
- `sub_0_69D2`: undescribed.
- `sub_0_6A08`: undescribed.
- `sub_0_6AE0`: undescribed.
- `sub_0_6B08`: undescribed.
- `sub_0_6B12`: undescribed.
- `sub_0_6B6C`: undescribed.
- `sub_0_6BAE`: undescribed.
- `sub_0_6BB2`: undescribed.
- `sub_0_6BDE`: undescribed.
- `sub_0_6C88`: undescribed.
- `sub_0_6C9E`: undescribed.
- `sub_0_6CE2`: undescribed.
- `sub_0_6CF2`: undescribed.
- `sub_0_6D0C`: undescribed.
- `sub_0_6F3C`: undescribed.
- `sub_0_71F0`: undescribed.
- `sub_0_7642`: undescribed.
- `sub_0_76A4`: undescribed.
- `sub_0_7798`: undescribed.
- `sub_0_792C`: undescribed.
- `sub_0_7958`: undescribed.
- `sub_0_7984`: undescribed.
- `sub_0_79AA`: undescribed.
- `sub_0_7A1E`: undescribed.
- `sub_0_7A7C`: undescribed.
- `sub_0_7D14`: undescribed.
- `sub_0_7D1E`: undescribed.
- `sub_0_7DD6`: undescribed.
- `sub_0_8048`: undescribed.
- `sub_0_804A`: undescribed.
- `sub_0_809A`: undescribed.
- `sub_0_8228`: undescribed.
- `sub_0_828E`: undescribed.
- `sub_0_834C`: undescribed.
- `sub_0_9AF0`: undescribed.
- `sub_0_A254`: undescribed.
- `sub_0_A286`: undescribed.
- `sub_0_A31A`: undescribed.
- `sub_0_A82C`: undescribed.
- `sub_0_A97A`: undescribed.
- `sub_0_AB38`: undescribed.
- `sub_0_AC72`: undescribed.
- `sub_0_AC7A`: undescribed.
- `sub_0_ACD4`: undescribed.
- `sub_0_AD6C`: undescribed.
- `sub_0_AD9A`: undescribed.
- `sub_0_ADDA`: undescribed.
- `sub_0_AE1A`: undescribed.
- `sub_0_AE7C`: undescribed.
- `sub_0_AEF0`: undescribed.
- `sub_0_AF74`: undescribed.
- `sub_0_AFC8`: undescribed.
- `sub_0_AFCA`: undescribed.
- `sub_0_B016`: undescribed.
- `sub_0_B050`: undescribed.
- `sub_0_B0F0`: undescribed.
- `sub_0_B1B2`: undescribed.
- `sub_0_B1EC`: undescribed.
- `sub_0_B226`: undescribed.
- `sub_0_B63C`: undescribed.
- `sub_0_B74A`: undescribed.
- `sub_0_B7A2`: undescribed.
- `sub_0_D694`: undescribed.
- `sub_0_DA06`: undescribed.
- `sub_0_DA20`: undescribed.
- `sub_0_DA24`: undescribed.
- `sub_0_DB76`: undescribed.
- `sub_0_DC46`: undescribed.
- `sub_0_DD4A`: undescribed.
- `sub_0_DD50`: undescribed.
- `sub_0_DD80`: undescribed.
- `sub_0_DD86`: undescribed.
- `sub_0_DD90`: undescribed.
- `sub_0_DD9A`: undescribed.
- `sub_0_DDB0`: undescribed.
- `sub_0_DDC8`: undescribed.
- `sub_0_DDD0`: undescribed.
- `sub_0_DE5E`: undescribed.
- `sub_0_DEEE`: undescribed.
- `sub_0_DFBE`: undescribed.
- `sub_0_E0B8`: undescribed.
- `sub_0_E5F8`: undescribed.
- `sub_0_EC94`: undescribed.
- `sub_0_ED06`: undescribed.
- `sub_0_EDE4`: undescribed.
- `sub_0_F232`: undescribed.
- `sub_0_F560`: undescribed.
- `sub_0_F5D8`: undescribed.
- `sub_0_F672`: undescribed.
- `sub_0_F750`: undescribed.
- `sub_0_F814`: undescribed.
- `sub_0_F82A`: undescribed.
- `sub_0_F958`: undescribed.
- `sub_0_F96C`: undescribed.
- `sub_0_F98E`: undescribed.
- `sub_0_F9B8`: undescribed.
- `sub_0_FAF6`: undescribed.
- `sub_0_FB34`: undescribed.
- `sub_0_FB38`: undescribed.
- `sub_0_FF64`: undescribed.
- `sub_0_10172`: undescribed.
- `sub_0_1028C`: undescribed.
- `sub_0_1039C`: undescribed.
- `sub_0_103F0`: undescribed.
- `sub_0_10430`: undescribed.
- `sub_0_104FC`: undescribed.
- `sub_0_10558`: undescribed.
- `sub_0_10578`: undescribed.
- `sub_0_105D8`: undescribed.
- `sub_0_10678`: undescribed.
- `sub_0_10692`: undescribed.
- `sub_0_1081E`: undescribed.
- `sub_0_1085A`: undescribed.
- `sub_0_1085E`: undescribed.
- `sub_0_10A38`: undescribed.
- `sub_0_10A94`: undescribed.
- `sub_0_10AC6`: undescribed.
- `sub_0_10AEA`: undescribed.
- `sub_0_10B24`: undescribed.
- `sub_0_10C4E`: undescribed.
- `sub_0_10C70`: undescribed.
- `sub_0_10D6A`: undescribed.
- `sub_0_11004`: undescribed.
- `sub_0_112B8`: undescribed.
- `sub_0_11376`: undescribed.
- `sub_0_11378`: undescribed.
- `sub_0_11386`: undescribed.
- `sub_0_113E2`: undescribed.
- `sub_0_113E4`: undescribed.
- `sub_0_11400`: undescribed.
- `sub_0_11638`: undescribed.
- `sub_0_1198C`: undescribed.
- `sub_0_1198E`: undescribed.
- `sub_0_11AD8`: undescribed.
- `sub_0_11B64`: undescribed.
- `sub_0_11B68`: undescribed.
- `sub_0_11B6C`: undescribed.
- `sub_0_11CAC`: undescribed.
- `sub_0_11D00`: undescribed.
- `sub_0_11D18`: undescribed.
- `sub_0_11D1C`: undescribed.
- `sub_0_11D20`: undescribed.
- `sub_0_11D24`: undescribed.
- `sub_0_11D28`: undescribed.
- `sub_0_11D30`: undescribed.
- `sub_0_11D9A`: undescribed.
- `sub_0_11D9E`: undescribed.
- `sub_0_11DA2`: undescribed.
- `sub_0_11DAE`: undescribed.
- `sub_0_11DF4`: undescribed.
- `sub_0_11E0E`: undescribed.
- `sub_0_1203A`: undescribed.
- `sub_0_1222A`: undescribed.
- `sub_0_1222E`: undescribed.
- `sub_0_12232`: undescribed.
- `sub_0_12242`: undescribed.
- `sub_0_1230A`: undescribed.
- `sub_0_1236E`: undescribed.
- `sub_0_1237C`: undescribed.
- `sub_0_12400`: undescribed.
- `sub_0_1244A`: undescribed.
- `sub_0_12474`: undescribed.
- `sub_0_124B6`: undescribed.
- `sub_0_124DC`: undescribed.
- `sub_0_1257A`: undescribed.
- `sub_0_125BA`: undescribed.
- `sub_0_125BE`: undescribed.
- `sub_0_125C2`: undescribed.
- `sub_0_12CE6`: undescribed.
- `sub_0_12DFC`: undescribed.
- `sub_0_13054`: undescribed.
- `sub_0_130E8`: undescribed.
- `sub_0_13340`: undescribed.
- `sub_0_13384`: undescribed.
- `sub_0_1338A`: undescribed.
- `sub_0_133BA`: undescribed.
- `sub_0_133CC`: undescribed.
- `sub_0_133EE`: undescribed.
- `sub_0_13488`: undescribed.
- `sub_0_13516`: undescribed.
- `sub_0_13550`: undescribed.
- `sub_0_135B2`: undescribed.
- `sub_0_13616`: undescribed.
- `sub_0_13642`: undescribed.
- `sub_0_13666`: undescribed.
- `sub_0_13708`: undescribed.
- `sub_0_171B2`: undescribed.
- `sub_0_1721C`: undescribed.
- `sub_0_1778C`: undescribed.
- `sub_0_177E8`: undescribed.
- `sub_0_17960`: undescribed.
- `sub_0_17BE8`: undescribed.
- `sub_0_17C6C`: undescribed.
- `sub_0_17C8A`: undescribed.
- `sub_0_17E82`: undescribed.
- `sub_0_1802E`: undescribed.
- `sub_0_18154`: undescribed.
- `sub_0_18168`: undescribed.
- `sub_0_1817C`: undescribed.
- `sub_0_181DA`: undescribed.
- `sub_0_18284`: undescribed.
- `sub_0_1828C`: undescribed.
- `sub_0_182B2`: undescribed.
- `sub_0_182B6`: undescribed.
- `sub_0_184FA`: undescribed.
- `sub_0_18698`: undescribed.
- `sub_0_18806`: undescribed.
- `sub_0_1885C`: undescribed.
- `sub_0_18A56`: undescribed.
- `sub_0_18A78`: undescribed.
- `sub_0_18B92`: undescribed.
- `sub_0_18C7E`: undescribed.
- `sub_0_18D6C`: undescribed.
- `sub_0_19212`: undescribed.
- `sub_0_198A8`: undescribed.
- `sub_0_198CC`: undescribed.
- `sub_0_198F2`: undescribed.
- `sub_0_19A70`: undescribed.
- `sub_0_19CC0`: undescribed.
- `sub_0_19CF0`: undescribed.
- `sub_0_19D14`: undescribed.
- `sub_0_19D4C`: undescribed.
- `sub_0_19E1C`: undescribed.
- `sub_0_19E3A`: undescribed.
- `sub_0_19EAE`: undescribed.
- `sub_0_19EEA`: undescribed.
- `sub_0_19F5E`: undescribed.
- `sub_0_1A2F0`: undescribed.
- `sub_0_1A306`: undescribed.
- `sub_0_1A32C`: undescribed.
- `sub_0_1A342`: undescribed.
- `sub_0_1A35A`: undescribed.
- `sub_0_1A39A`: undescribed.
- `sub_0_1A3A0`: undescribed.
- `sub_0_1A3AE`: undescribed.
- `sub_0_1A402`: undescribed.
- `sub_0_1A43C`: undescribed.
- `sub_0_1A450`: undescribed.
- `sub_0_1A454`: undescribed.
- `sub_0_1A992`: undescribed.
- `sub_0_1B094`: undescribed.
- `sub_0_1B098`: undescribed.
- `sub_2_0000`: undescribed.
- `sub_2_00F4`: undescribed.
- `sub_2_01AA`: undescribed.
- `sub_2_028E`: undescribed.
- `sub_2_02C6`: undescribed.
- `sub_2_0330`: undescribed.
- `sub_2_0342`: undescribed.
- `sub_2_048A`: undescribed.
- `sub_2_0650`: undescribed.
- `sub_2_07F6`: undescribed.
- `sub_2_083E`: undescribed.
- `sub_2_08E0`: undescribed.
- `sub_2_0B16`: undescribed.
- `sub_2_0D0E`: undescribed.
- `sub_2_0E04`: undescribed.
- `sub_2_0EC2`: undescribed.
- `sub_2_0ED0`: undescribed.
- `sub_2_108E`: undescribed.
- `sub_2_1134`: undescribed.
- `sub_2_117A`: undescribed.
- `sub_2_11CE`: undescribed.
- `sub_2_125A`: undescribed.
- `sub_2_1280`: undescribed.
- `sub_2_12B8`: undescribed.
- `sub_2_1350`: undescribed.
- `sub_2_15D2`: undescribed.
- `sub_2_163A`: undescribed.
- `sub_2_1744`: undescribed.
- `sub_2_17B4`: undescribed.
- `sub_2_185E`: undescribed.
- `sub_2_1A70`: undescribed.
- `sub_2_1B68`: undescribed.
- `sub_2_1CF4`: undescribed.
- `sub_2_2536`: undescribed.
- `sub_2_290C`: undescribed.
- `sub_2_2930`: undescribed.
- `sub_2_2954`: undescribed.
- `sub_2_2978`: undescribed.
- `sub_2_29A0`: undescribed.
- `sub_2_3344`: undescribed.
- `sub_2_33D8`: undescribed.
- `sub_2_342A`: undescribed.
- `sub_2_366E`: undescribed.
- `sub_2_36C6`: undescribed.
- `sub_2_3714`: undescribed.
- `sub_2_39D8`: undescribed.
- `sub_2_3A2C`: undescribed.
- `sub_2_3AC4`: undescribed.
- `sub_2_3E0E`: undescribed.

### explode.library

- `return_constant_value`: The routine returns a constant value without performing any other operations.
- `handle_message_request`: Reads a request structure from the caller, resolves a dos.library base via an indirect call and stores a handle, then extracts a short flag and a pointer from the request, and dispatches based on the flag value using table lookups, storing a result pointer and invoking further library routines before returning.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrement_and_poke_handlers`: The routine decrements a counter at an offset from a passed pointer and, if the counter reaches zero while a flag bit is set, it validates a nearby jump table and patches two entries in that table with values from the passed structure.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `self_3`: The routine checks a library header for a specific signature and a pointer to itself, then copies a short and a long from a caller-supplied structure into a library structure, optionally overwrites a function pointer, calls three functions through offsets in a global variable, and returns a value from the caller-supplied structure.
- `set_flag_in_structure`: The routine receives an address in a register and sets a flag bit in a byte field of the pointed-to structure.
- `validate_library_name`: The routine obtains a handle from another routine and checks a header field for a specific value.
- `check_library_name`: The routine obtains a library base pointer from a global variable, validates the library header magic and then checks the library name field for the string 'explode.library' in either forward or reversed byte order.
- `validate_library_name_2`: The routine obtains a library base pointer from a global variable, checks a version field and then validates the library name stored in the structure, accepting both forward and reversed byte order forms.
- `check_library_name_2`: The routine receives a pointer in a0 and checks whether the four words pointed to by that pointer spell a library name string.
- `decrement_counter_and_dispatch`: The routine decrements a 16-bit counter stored at an offset from a register, calls another routine twice, then conditionally calls a function pointer taken from a global variable if a 16-bit value in that variable exceeds a threshold, and returns its input argument.
- `initialize_explosion_descriptor`: The routine fills an explosion descriptor structure from an input buffer, copies and transforms a small set of bytes into mask fields, builds a small table of bit flags from a byte array, and then calls another routine twice while iterating over an integer array, accumulating sums and updating entries via pointer arithmetic.
- `copy_memory_blocks`: The routine copies a block of memory from a source area to a destination area, handling full 8-word chunks and a remaining byte tail, and updates a running sum into a table while looping until a counter in a global variable reaches a limit, then clears a flag and returns.
- `self_4`: The routine compares two pointers and conditionally calls another routine, then returns the value pointed to by one of the pointers.
- `copy_block_words`: The routine copies a variable number of 32-bit words from a source area to a destination area, handling full 8-word blocks first and then any remaining words one by one.
- `copy_memory_blocks_2`: The routine copies 32-bit words from a source area to a destination area in a loop, handling a signed count and a termination condition based on a pointer reaching the end of a buffer.
- `copy_struct_array_loop`: The routine copies blocks of eight words from a source area into a destination area, then copies any remaining words one by one, using a global variable as a loop counter and a table of function pointers for repeated callbacks.
- `copy_bytes_with_carry`: The routine copies a block of bytes from a source pointer backwards into a destination buffer while maintaining a carry-based checksum.
- `decode_explode_byte`: The routine decodes a byte stream from a pointer in A0 using a shift-and-carry loop driven by D0 and D1, producing a small index in D0 and a length value in D1 for each iteration.
- `decode_bitstream_from_table`: The routine decodes a bitstream pointed to by a pointer in D1, using a table pointed to by A4, and writes the decoded bits into a destination buffer via A0.
- `extract_bits_from_source_buffer`: The routine walks a source byte buffer backwards, extracting bits from each byte with carry handling and using a table indexed by a counter to select offsets and lengths.
- `decode_bitstream_to_buffer`: The routine decodes a bit-packed stream pointed to by a pointer in D1, using a table pointed to by A4, and writes the resulting bytes into a buffer pointed to by A3 with bounds checked against A2 and D4.
- `decode_and_copy_bytes`: The routine decodes a byte stream by repeatedly extracting bits from a source buffer using carry-based bit extraction and uses the decoded values as indices into a table to compute offsets, then copies a block of bytes from a source area to a destination area using those offsets.
- `decode_bitstream_from_table_2`: The routine decodes a variable-length bitstream that is read from a byte pointer, using carry flags to extract bits and a table pointed to by a pointer to perform table lookups.
- `decode_and_copy_bytes_2`: The routine decodes a byte stream using a bitstream extracted from a source buffer and a table in memory, then copies the decoded bytes into a destination buffer while updating a counter in the table.
- `decode_and_copy_bytes_3`: The routine decodes a byte stream using a carry-propagating shift and a table indexed by a global variable, then copies the decoded bytes into a destination buffer while updating counters in a control block.
- `decode_table_shift_blocks`: A routine in the explode library that decodes a compressed block from a source buffer into a destination buffer using a byte stream interpreted as variable-length bit codes.
- `execute_callback_loop`: The routine reads a counter from a structure, increments it, and repeatedly calls another routine and a callback taken from a table indexed by a global variable until the counter condition is met, then calls the routine once more and clears a word in memory.
- `decrement_loop_counter`: The routine receives an unsigned value in an input register, converts it to signed, subtracts one, and if the result is positive, repeatedly decrements it until it reaches -1.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:Failat

- `initialize_memory_table`: The routine walks a linked list of entries to find the maximum of a field and then builds a small table in memory with a repeating pattern.
- `copy_table_to_stack`: The routine copies sixteen 32-bit values from a table located at an offset from a pointer in a2 into the memory area pointed to by a pointer on the stack, then calls another routine.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:endrun

- `process_command_line_input`: The routine checks two function pointers from a global variable and, if present, clears fields in a structure and updates several fields with a value returned by another routine, then copies a command-line string from the input buffer into a global buffer, skipping leading spaces and truncating at space, newline or null, and stores the length and a result from a library call into the structure.
- `empty_routine`: The routine contains no instructions and returns immediately.
- `infinite_loop_routine`: The routine contains an empty infinite loop and performs no operations.
- `clear_dma_control_block`: The routine clears two fields in a DMA control block pointed to by a pointer derived from an input value, writes all-ones to two other fields, then calls another routine via an indirect pointer.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4).adf:bootblock

- `check_boot_block_flag`: The routine calls another routine through a pointer stored in a global variable and returns a flag value based on its result.
