# Game 2119 (wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02119). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907456 top-level bytes in 125 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 65 of 65 named (0 author symbols), 65 of 65 described (11 by rule, 33 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 893 of 893 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 533910 bytes, 6 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1657 of 1705 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 23788 |
| code | 5830 |
| data | 766695 |
| padding | 19495 |
| unused | 91648 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:2000AD | 1 | 80 | 4 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:wizkid | 1 | 573 | 13 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 886 | 39 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:AstCode#unpacked (raw code file, load address unknown) | 1 | 50 | 1 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:wizkiddemo5e#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:AstCoC0#unpacked (raw code file, load address unknown) | 1 | 50 | 1 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:AstCo80#unpacked (raw code file, load address unknown) | 1 | 50 | 1 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:WIZKIDDEMOC0#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:WIZKIDDEMO80#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |

## Cases handled

- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:Wiz+Awave: IMP: File Imploder-packed, unpacked with ancient (3200 -> 4398 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:BDSprites: IMP: File Imploder-packed, unpacked with ancient (1404 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:eeorr1b.s: IMP: File Imploder-packed, unpacked with ancient (5098 -> 5716 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:AstCode: IMP: File Imploder-packed, unpacked with ancient (14224 -> 30258 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:sfx1: IMP: File Imploder-packed, unpacked with ancient (6542 -> 9368 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:WizCry: IMP: File Imploder-packed, unpacked with ancient (3182 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:eeeek.s: IMP: File Imploder-packed, unpacked with ancient (3484 -> 4960 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:mod.hbd: IMP: File Imploder-packed, unpacked with ancient (27536 -> 40651 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:creeek.s: IMP: File Imploder-packed, unpacked with ancient (3108 -> 3236 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:cherry2c.s: IMP: File Imploder-packed, unpacked with ancient (4698 -> 5114 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:Arcade: IMP: File Imploder-packed, unpacked with ancient (1158 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:coininslot.s: IMP: File Imploder-packed, unpacked with ancient (6130 -> 6336 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:jahr.s: IMP: File Imploder-packed, unpacked with ancient (7800 -> 9803 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:letterbox2.s: IMP: File Imploder-packed, unpacked with ancient (4332 -> 5312 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:AShip5: IMP: File Imploder-packed, unpacked with ancient (1860 -> 16000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:canopen2.s: IMP: File Imploder-packed, unpacked with ancient (2848 -> 3212 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:raspberry.s: IMP: File Imploder-packed, unpacked with ancient (14196 -> 14266 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:wizkiddemo5e: IMP: File Imploder-packed, unpacked with ancient (73224 -> 131840 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:bell.s: IMP: File Imploder-packed, unpacked with ancient (6734 -> 6972 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:BDScreen: IMP: File Imploder-packed, unpacked with ancient (3828 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:chew2.s: IMP: File Imploder-packed, unpacked with ancient (1496 -> 2242 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:AstCoC0: IMP: File Imploder-packed, unpacked with ancient (14226 -> 30258 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:AstCo80: IMP: File Imploder-packed, unpacked with ancient (14224 -> 30258 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:bog_1shot.s: IMP: File Imploder-packed, unpacked with ancient (26096 -> 29472 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:UsingRam: IMP: File Imploder-packed, unpacked with ancient (4914 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:DiskPrompt: IMP: File Imploder-packed, unpacked with ancient (1266 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:doggie.s: IMP: File Imploder-packed, unpacked with ancient (1788 -> 1842 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:splash3.s: IMP: File Imploder-packed, unpacked with ancient (2484 -> 2680 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:Stage: IMP: File Imploder-packed, unpacked with ancient (6226 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:Arcade+s: IMP: File Imploder-packed, unpacked with ancient (7378 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:touchtone1.s: IMP: File Imploder-packed, unpacked with ancient (2098 -> 3650 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:pull.s: IMP: File Imploder-packed, unpacked with ancient (2758 -> 3076 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:thunder.s: IMP: File Imploder-packed, unpacked with ancient (10650 -> 12084 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:cash.s: IMP: File Imploder-packed, unpacked with ancient (4778 -> 5954 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:StageIntro: IMP: File Imploder-packed, unpacked with ancient (13228 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:WIZKIDDEMOC0: IMP: File Imploder-packed, unpacked with ancient (76900 -> 155648 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:door.s: IMP: File Imploder-packed, unpacked with ancient (946 -> 982 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:pop1.s: IMP: File Imploder-packed, unpacked with ancient (1414 -> 1662 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:keydisk: high-entropy data file (7.77 bits/byte), classified as packed data
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:WIZKIDDEMO80: IMP: File Imploder-packed, unpacked with ancient (76866 -> 155648 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:draclafeko.s: IMP: File Imploder-packed, unpacked with ancient (11902 -> 15446 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:1812bit2: IMP: File Imploder-packed, unpacked with ancient (6664 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:mod.1812: IMP: File Imploder-packed, unpacked with ancient (116956 -> 151496 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:TheWizCastA: IMP: File Imploder-packed, unpacked with ancient (13318 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:fuse1b.s: IMP: File Imploder-packed, unpacked with ancient (2132 -> 2530 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:TheWizCastB: IMP: File Imploder-packed, unpacked with ancient (11712 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:magic1d.s: IMP: File Imploder-packed, unpacked with ancient (11776 -> 13922 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:chew3.s: IMP: File Imploder-packed, unpacked with ancient (1464 -> 2128 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:coininslot2.s: IMP: File Imploder-packed, unpacked with ancient (5262 -> 5828 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:ghost2b.s: IMP: File Imploder-packed, unpacked with ancient (4340 -> 6538 bytes)
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-rs-x25k): 622 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:AstCode#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:wizkiddemo5e#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:AstCoC0#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:AstCo80#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:WIZKIDDEMOC0#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:WIZKIDDEMO80#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- wizkid (1992)(ocean)(m3)[cr tkk][t +2 2kad](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:AShip5.pal: raw file found loaded verbatim in runtime segment an address (play-rs-x25k) at an address; classified through the recovered segment
- 6 never-loaded region(s), 533910 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `read_named_disk_file` (wizkid): Upper-cases a file name, hashes it to choose a bucket, and walks that bucket's chain comparing names. Kind: track/sync/checksum check. Pass path: the branch where iVar4 != 0 and subsequent data loading/comparison proceeds (the loop containing u1__sub_0A0A and string comparison/copying or checksum-like logic).
- `initialize_data_tables` (runtime segment an address (play-rs-x25k)): Initializes several memory tables by filling them with incrementing values and copying data from existing source tables. Kind: data table initialization for disk/hardware verification. Pass path: u3__sub_08D8.

## Functions


### Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:2000AD

- `entry_set_background_color`: Program entry with 64 instructions that calls two unnamed routines and writes the background colour register.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `rotate_bit_stream_right_once`: Returns the longword just before the input pointer shifted right one place, with its top bit set from a test of a flag bit in the D7 register.
- `shift_bitstream_by_count`: Repeats a shift of the bit buffer right by one place for a count held in D1 (counting down to zero).

### Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:wizkid

- `entry_open_graphics_intuition_libraries`: Program entry with 56 instructions that opens the graphics and intuition libraries through exec, calls two unnamed routines, reads the CIA-A port A register for button and disk status, and closes the libraries again.
- `draw_centered_text_lines`: Walks newline-terminated strings in a table, measuring each line and computing a start column that centres it in an 80-column width.
- `draw_glyph_into_two_planes`: Converts a character code to a glyph index by subtracting 32 (clamped at zero), then writes eight glyph rows into two byte planes at a row stride of 80 bytes.
- `read_named_disk_file`: Upper-cases a file name, hashes it to choose a bucket, and walks that bucket's chain comparing names.
- `pulse_control_bit_and_wait`: Clears and then sets the lowest bit of the byte register addressed by the pointer, calling the unnamed routine between and after the changes.
- `forward_call_preserving_d0`: Calls the unnamed routine and returns the D0 register value it was given, unchanged.
- `relay_call_return_d0`: Calls the unnamed routine and returns the D0 register value it was given, unchanged.
- `delay_polling_register_count`: Repeats a timed wait for the number of passes passed in the data register, spinning on a memory read from the device register block on each pass.
- `read_disk_track_copy_match`: Drives the disk control register for each attempt, searches the directory for a matching name, and on success copies the matched entry's data block into the destination buffer.
- `unpack_imp_packed_data_backwards`: Checks the four-character IMP! tag at the start of a packed block, then unpacks it by reading bits from the end of the data.
- `wait_for_cia_status_clear`: Busy-waits on a status byte at a fixed device offset until its bit five is clear, then returns.
- `decode_mfm_disk_sector`: Searches for the MFM sync word, rebuilds the data longwords from the odd and even bit fields using an alternating-bit mask, and checks an XOR checksum.
- `stub_bad_instruction_data`: Contains only bad instruction data, so the decompiler truncates control flow here.

### Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_tables`: Initializes several memory tables by filling them with incrementing values and copying data from existing source tables.
- `wait_for_status_bit_transition`: Spins until a status bit in a hardware byte becomes set, then spins until it is clear again, and then returns.
- `negate_byte_register`: Negates the value of a byte in a data register.
- `set_bit_in_register`: Sets a specific bit in a byte within a data register.
- `load_word_from_table`: Clears a byte in a data register and then loads a word from a table into that register.
- `call_and_write_control_value`: Calls another routine, writes a fixed value to a custom chip register, and returns the two input arguments unchanged.
- `execute_complex_sequence`: Increments a value and executes a sequence of routines from a table based on a threshold.
- `execute_conditional_sequence`: Increments a value and, if it reaches a specific threshold, executes a sequence of other routines and updates a global variable.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d4.
- `update_mouse_value`: Reads a value from the mouse port, performs arithmetic on it, clamps the result between two bounds, and stores the result in a global variable.
- `tiny_ori_2`: Small routine of 1 instruction that merges 166 into (a7).
- `tiny_or_ori`: Small routine of 2 instructions that merges d0 into d3, then merges 0 into d0.
- `copy_memory_block_offset_2c`: Copies several bytes from one memory location to another using specific offsets.
- `copy_struct_members`: Copies a series of specific bytes from one memory block to another at fixed offsets.
- `copy_data_blocks`: Iterates through a loop to copy multiple blocks of data from a source table to a destination table based on a value in a global variable.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d0.
- `clear_data_register`: Performs a logical OR operation on a data register with zero.
- `increment_global_counter`: Checks if a global variable equals a specific value, calls another routine and clears the variable if so, then increments the global variable.
- `update_index_pointers`: Loads three 16-bit values from a memory location and increments three global index variables using bitwise masking.
- `modify_global_variable`: Performs arithmetic on a global variable and stores the result back into it.
- `call_blitter_setup_routine`: Calls a routine that configures and initiates hardware blitter operations using channel pointers and control values.
- `process_coordinate_table`: Iterates through a table of values to calculate new coordinates and updates a byte in a memory buffer based on those calculations.
- `tiny_ori_4`: Small routine of 1 instruction that merges 140 into d6.
- `modify_status_byte`: Increments a byte at a specific offset from a pointer.
- `fill_table_with_increments`: Fills a table with values that increase by a fixed amount in each step.
- `fill_table_with_division`: Fills a table with the results of a constant divided by an incrementing value.
- `generate_pseudo_random_sequence`: Fills a table with values generated by a linear congruential generator.
- `start_blit_with_channel_pointers`: Waits while a busy flag is set, then writes blitter registers for all four channels, including pointers, modulos, control values and the size, and returns.
- `setup_cia_b_timer_interrupt`: Initialises a block of globals and programs CIA-B timer A through its control, high and low count and interrupt registers, then enables the interrupt through INTENA and touches CIAA_PRA.
- `clear_flag_set_dma_interrupts`: Clears one bit of a shared flag byte, then writes fixed constants to the DMA control register and the interrupt enable register.
- `process_audio_and_data`: Manages a counter and executes a series of routines from a table, while also updating various fields in a data structure and modifying audio hardware registers.
- `write_timer_and_request_registers`: Writes a fixed counter value, that value plus 40, and a shared word to the CIA-B timer, DMA control and interrupt request registers named in the facts, then writes a final constant.
- `reset_audio_channel_registers`: Adjusts a shared value by minus 40, then writes eight table entries to the pointer and length registers of the four audio channels, and finally writes a constant to the interrupt request register.
- `decrease_position_with_floor_2`: A 60-instruction routine that calls another routine and performs a decrease position with floor operation.
- `select_tracker_effect_nibble`: Works out a modulo-three step from two channel bytes, then either writes a stored word into a record field or picks a nibble of its argument and processes a 72-byte table entry.
- `decrease_position_with_floor`: Subtracts an input delta from a position field in the record and raises the position to 113 when it falls below that value.
- `store_byte_to_offset`: Stores a byte from a register into a memory location at a fixed offset from a pointer.
- `apply_speed_to_position`: Copies a nonzero speed byte into a stored field, then adds scaled multiples of that byte to a 16-bit and a 32-bit position field.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:AstCode#unpacked (raw code file, load address unknown)

- `start_raw_code_region_astcode`: Start of a 50-instruction raw code region in the AstCode file.

### Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:wizkiddemo5e#unpacked (raw code file, load address unknown)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:AstCoC0#unpacked (raw code file, load address unknown)

- `start_raw_code_region_astcoc0`: Start of a 50-instruction raw code region in the AstCoC0 file.

### Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:AstCo80#unpacked (raw code file, load address unknown)

- `start_raw_code_region_astco80`: Start of a 50-instruction raw code region in the AstCo80 file.

### Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:WIZKIDDEMOC0#unpacked (raw code file, load address unknown)

- `start_raw_code_region_wizkiddemoc0`: Start of a one-instruction raw code region in the WIZKIDDEMOC0 file.

### Wizkid (1992)(Ocean)(M3)[cr TKK][t +2 2KAD](Disk 1 of 2).adf:WIZKIDDEMO80#unpacked (raw code file, load address unknown)

- `start_raw_code_region_wizkiddemo80`: Start of a one-instruction raw code region in the WIZKIDDEMO80 file.
