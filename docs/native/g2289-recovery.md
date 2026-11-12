# Game 2289 (star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02289). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 20 of 20 named (0 author symbols), 20 of 20 described (0 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 330 of 330 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 660 of 660 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 152977 |
| code | 3154 |
| data | 719900 |
| padding | 1 |
| unused | 25088 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf:bootblock | 1 | 16 | 1 |
| logged disk load from Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf boot-block load | 1 | 644 | 19 |
| logged disk load from Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf boot-block load | 1 | 0 | 0 |

## Cases handled

- star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf: custom-format (non-DOS) disk, 1624 non-empty blocks: matched against track-loaded segments
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4ED5) A7=an address (A7)=an address, slid to exec.library -an address): 556 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf: the boot block reads 146432 bytes from disk offset 450560 to an address (found statically); recovered as a unit
- star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf: 2 logged loads cover 157696 bytes of the image
- star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf: blocks 2-46 (bytes 1024-24064) outside the filesystem hold data, not code (opcode density 0.038, entropy 5.28)
- star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf: blocks 66-66 (bytes 33792-34304) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.02)
- star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf: blocks 68-539 (bytes 34816-276480) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.90)
- star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf: blocks 542-610 (bytes 277504-312832) outside the filesystem hold data, not code (opcode density 0.037, entropy 5.32)
- star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf: blocks 617-630 (bytes 315904-323072) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.12)
- star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf: blocks 635-639 (bytes 325120-327680) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.49)
- star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf: blocks 641-647 (bytes 328192-331776) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.59)
- star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf: blocks 651-655 (bytes 333312-335872) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.76)
- star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf: blocks 666-669 (bytes 340992-343040) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.21)
- star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf: blocks 673-879 (bytes 344576-450560) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.96)
- star wars - the empire strikes back (1988)(domark)[cr vf - beyonders]/Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf: blocks 1166-1737 (bytes 596992-889856) hold ASCII text

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf:bootblock

- `boot_load_two_blocks_halt`: Boot block entry that fills two load request blocks with fixed addresses and lengths, calls the library routine through a negative jump-table offset for each, and shifts one bit into a stored word.

### logged disk load from Star Wars - The Empire Strikes Back (1988)(Domark)[cr VF - Beyonders].adf boot-block load

- `initialize_and_run_graphics_engine`: Initializes several tables for ramp values, cumulative sums, and packed twelve-bit words, then enters a loop that performs word table copying, stores results, builds copper lists, and shifts word tables while waiting for a vertical retrace and checking a bit in a CIA register.
- `write_colour_move_copper_entries`: Writes seven pairs of copper move words into a list.
- `dispatch_interrupt_request_bits`: Calls one of two routines when a status bit is set.
- `build_copper_list_plane_pointers`: Writes bitplane pointer moves from a base address, followed by mode and colour move words.
- `store_eight_routine_results`: Stores eight values returned by an unnamed routine into consecutive longword locations in the table.
- `copy_word_list_offset_table`: Looks up a destination offset from a table indexed by the input value, then copies words from the source into that destination until a minus-one sentinel.
- `copy_word_table_with_fallback`: Copies 300 words from a table into a fixed buffer, replacing a minus-one sentinel with a fallback value.
- `fill_rise_fall_ramp_table`: Fills a word table with a profile that rises in steps of two, four and six, then falls back to zero in the reverse steps.
- `build_cumulative_sum_table`: Writes a running total into a 272-entry table, adding one source word for every fifteen entries, and ends the table with a minus-one longword.
- `shift_word_table_by_one`: Shifts 266 words of a table back by one entry, then stores a saved control value in the final slot.
- `countdown_and_reset_flag`: Decrements a countdown.
- `execute_blitter_area_fills`: Configures and executes multiple blitter area fill operations using specific source and destination parameters.
- `wait_for_blitter_idle`: Busy-waits on a global status word until its blitter-busy bit, bit six, clears.
- `decode_run_length_copy_rows`: Decodes a run-length stream into a temporary buffer, where a negative count repeats the next byte and a positive count copies literal bytes.
- `unpack_packed_twelve_bit_words`: Reads three bytes per iteration, combines them into a twelve-bit value, and writes it as one word through the output pointer.
- `update_scroll_and_plane_pointers`: Moves a scroll offset forward or backward by 40 bytes and reverses direction after 85 steps.
- `reset_channel_pointers_and_state`: Points fifteen channel pointers at a shared location and zeroes the matching state longwords.
- `advance_music_step_six_ticks`: Counts ticks and, on every sixth tick, recomputes the sample position from a sequence byte.
- `load_audio_channel_from_entry`: Copies a per-channel entry into the channel's location and length fields.
