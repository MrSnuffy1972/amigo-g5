# Game 267 (HeroQuest2 v1.0 1455): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00267). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2025035 top-level bytes in 119 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 87 raw-word lines |
| Functions named and described | pass, 470 of 470 named (0 author symbols), 470 of 470 described (204 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2158 of 2160 (99.91%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5218 of 5332 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 21289 |
| code | 18490 |
| data | 1985250 |
| padding | 6 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| HeroQuest2.Slave | 1 | 2158 | 139 |
| Installer | 3 | 223 | 8 |
| QuestII | 6 | 227 | 8 |
| Stack | 2 | 117 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 708 | 208 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1837 | 94 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 33 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 8 |

## Cases handled

- HeroQuest2_v1.0_1455/HeroQuest2/data/AudioG.Bin: high-entropy data file (7.99 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/CreditsG.Bin: high-entropy data file (7.99 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Deutsch.Bin: high-entropy data file (7.97 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/English.Bin: high-entropy data file (7.96 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/FastG.Bin: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Francais.Bin: high-entropy data file (7.96 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/GameG.Bin: high-entropy data file (7.99 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Gremlin.Pak: high-entropy data file (7.97 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/HQ2Game.Rnc: high-entropy data file (7.99 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/HQ2Title.Rnc: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (128511 -> 176716 bytes)
- HeroQuest2_v1.0_1455/HeroQuest2/data/HeroLogo.Pak: high-entropy data file (7.94 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Intro.Rnc: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (145364 -> 180922 bytes)
- HeroQuest2_v1.0_1455/HeroQuest2/data/OverG.Rnc: high-entropy data file (7.87 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Pack.Bin: high-entropy data file (7.88 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/ShopG.Rnc: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (49881 -> 88162 bytes)
- HeroQuest2_v1.0_1455/HeroQuest2/data/TitleG.Rnc: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (29568 -> 61566 bytes)
- HeroQuest2_v1.0_1455/HeroQuest2/data/Trader.Rnc: high-entropy data file (7.99 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Bits/Aldoth.Bit: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Bits/Castle.Bit: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Bits/Islands.Bit: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Bits/Oblivion.Bit: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Bits/Oracle.Bit: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Bits/Sewers.Bit: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Bits/Temple.Bit: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Bits/Venthrax.Bit: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Bits/Yaserat.Bit: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Assassin.Raw: high-entropy data file (7.90 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Barbrian.Raw: high-entropy data file (7.95 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Beholder.Raw: high-entropy data file (7.91 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/BigMouth.Raw: high-entropy data file (7.89 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Cleric.Raw: high-entropy data file (7.95 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Coatl.Raw: high-entropy data file (7.93 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/DarkElf.Raw: high-entropy data file (7.91 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/DarkElfG.Raw: high-entropy data file (7.91 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/DarkElfP.Raw: high-entropy data file (7.92 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/DkDwarfP.Raw: high-entropy data file (7.93 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/DkDwarfW.Raw: high-entropy data file (7.89 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/DragKing.Raw: high-entropy data file (7.90 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/DragLich.Raw: high-entropy data file (7.86 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Dwarf.Raw: high-entropy data file (7.94 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Elf.Raw: high-entropy data file (7.95 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Gargoyle.Raw: high-entropy data file (7.91 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Golem.Raw: high-entropy data file (7.91 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Hydra.Raw: high-entropy data file (7.93 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/MindFlay.Raw: high-entropy data file (7.88 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/MudMan.Raw: high-entropy data file (7.93 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Mummie.Raw: high-entropy data file (7.89 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Mystic.Raw: high-entropy data file (7.93 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Paladin.Raw: high-entropy data file (7.94 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Ranger.Raw: high-entropy data file (7.95 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/RatManG.Raw: high-entropy data file (7.92 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/RatManP.Raw: high-entropy data file (7.91 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/RatManW.Raw: high-entropy data file (7.91 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Skeleton.Raw: high-entropy data file (7.91 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Snake.Raw: high-entropy data file (7.91 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Vampire.Raw: high-entropy data file (7.88 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Wizard.Raw: high-entropy data file (7.94 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Wraith.Raw: high-entropy data file (7.90 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Zombie1.Raw: high-entropy data file (7.91 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Zombie2.Raw: high-entropy data file (7.92 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Chaps/Zombie3.Raw: high-entropy data file (7.92 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Charts/Aldoth.Map: high-entropy data file (7.79 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Charts/Castle.Map: high-entropy data file (7.80 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Charts/Islands.Map: high-entropy data file (7.84 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Charts/Oblivion.Map: high-entropy data file (7.80 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Charts/Oracle.Map: high-entropy data file (7.79 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Charts/Sewers.Map: high-entropy data file (7.77 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Charts/Temple.Map: high-entropy data file (7.82 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Charts/Venthrax.Map: high-entropy data file (7.83 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Charts/Yaserat.Map: high-entropy data file (7.78 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Glued/Aldoth.Glu: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Glued/Castle.Glu: high-entropy data file (7.97 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Glued/Islands.Glu: high-entropy data file (7.97 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Glued/Oblivion.Glu: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Glued/Oracle.Glu: high-entropy data file (7.97 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Glued/Sewers.Glu: high-entropy data file (7.97 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Glued/Temple.Glu: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Glued/Venthrax.Glu: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Glued/Yaserat.Glu: high-entropy data file (7.97 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Sketch/Aldoth.Pak: high-entropy data file (7.97 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Sketch/Castle.Pak: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Sketch/Gremlin.Pak: high-entropy data file (7.97 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Sketch/Islands.Pak: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Sketch/Oblivion.Pak: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Sketch/Oracle.Pak: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Sketch/Sewers.Pak: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Sketch/Temple.Pak: high-entropy data file (7.97 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Sketch/Venthrax.Pak: high-entropy data file (7.98 bits/byte), classified as packed data
- HeroQuest2_v1.0_1455/HeroQuest2/data/Sketch/Yaserat.Pak: high-entropy data file (7.98 bits/byte), classified as packed data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 291 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 631 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- HeroQuest2_v1.0_1455/HeroQuest2/data/Devs/system-configuration: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- HeroQuest2_v1.0_1455/HeroQuest2/data/Installer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- HeroQuest2_v1.0_1455/HeroQuest2/data/QuestII: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `init_audio_sample_state_and_silence` (runtime segment an address): Stores the sample pointer in a5, checks a magic value to pick offsets, sets up sample pointers and a byte-scan loop, then silences all four audio volumes and clears DMA and ADKCON bits. Kind: audio-based disk/track verification. Pass path: sub_2AFE.

## Functions


### HeroQuest2.Slave

- `return_minus_one_at_entry`: Loads the immediate value minus one into data register d0 and returns immediately to the caller, providing a constant return value at program entry.
- `subtract_eight_from_a5_offset_word`: Subtracts eight from a word at an offset from address register a5, then branches unconditionally to another location; it does not loop or call anything itself.
- `questii_startup_sequence`: Calls setup routines, opens a screen via library offsets, stores a value, computes an a5 pointer, runs further setup, then exits through the library with minus one.
- `install_jump_stub_at_27100`: Copies a5 to a0, computes a1 as a0 plus the offset an address, allocates 8 bytes via the allocator with a2 pointing to a pc-relative table, then writes a jump opcode and target address into the returned block.
- `restore_all_registers_and_return`: Overwrites the top stack word with d1, then pops d0 through d7 and a0 through a6 from the stack and returns, restoring the full register set.
- `install_nop_patch_stub_at_27100`: Reads a2 from 60(a7), allocates 6 bytes at a1 equal to a0 plus an address, writes a move opcode and a nop into the block, calls the patch routine, then restores all registers and returns.
- `load_constant_into_d1`: Loads the 16-bit constant 18 into data register d1 and then falls through into the following routine.
- `install_interrupt_vector_chain`: Loads the address of a new interrupt handler and a storage slot, saves the old level-1 vector from low memory into that slot, then stores the new handler address in the vector.
- `serial_interrupt_wrapper`: Sets the CIA serial-port interrupt bit, saves d0, calls the serial handler with d0 set to 2, clears the interrupt bit, restores d0 and jumps to the saved old vector.
- `zero_filled_data_area`: Contains a single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `wait_for_vertical_position_change`: Saves d0 on the stack, polls the VHPOSR register until its byte changes, restores d0 and repeats the delay loop d0 times before returning.
- `open_intuition_and_run_config_check`: Opens intuition.library, calls a config routine with a marker on the stack, verifies the marker, then opens devs/system-configuration and jumps to the library close vector.
- `zero_filled_data_word`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `call_allocator_then_jump_into_next`: Calls the routine at offset 386, then branches unconditionally into the code at offset 2b4 without returning to its caller.
- `allocate_and_init_task_structure`: Allocates memory via a helper, stores size and pointer fields in the structure at a2, builds a stack frame with task entry and stack pointers, calls AddTask, and restores registers.
- `find_task_and_build_stack_frame`: Calls exec FindTask with a null argument, reads the task's stack pointer fields to compute a size, then pushes arguments and a return address and clears registers a1-a6 and d1-d7 before returning.
- `push_global_and_adjust_stack`: Pushes a longword from a program-relative global onto the stack, then adds four to the stack pointer and returns, effectively discarding the pushed value.
- `discard_four_bytes_and_return`: Adds four to the stack pointer a7, removing one longword from the stack, then returns to the caller.
- `strlen_of_dos_output_string`: Calls the Amiga library function at offset -534 with a6, then counts the bytes of the returned string in a0 until a null terminator, returning the count in d0.
- `allocate_and_init_68_byte_structure`: Allocates 68 bytes via the memory routine, copies 68 bytes from a data table into it, stores d4 at offset 132, calls the list routine, then stores d1 at offset 10 and bytes 13 and d2 at offsets 8 and 9.
- `count_list_nodes_and_allocate_array`: Walks the linked list in d3 counting nodes, allocates four times that many bytes, stores the count shifted right twice at offset 128 of a2, then copies the list node pointers into the new array.
- `init_kickdos_process_and_exit`: Calls three setup routines, saves returned values, opens the KickDos process via exec, stores a value at offset 60 of the process, then calls a routine with a text pointer and exits via exec.
- `store_shifted_clock_ticks`: Saves all registers, calls the exec library routine at offset -492, shifts the returned d0 right by two bits and stores it at offset 172 of the structure pointed to by a3.
- `patch_code_with_jsr_and_fill`: Allocates memory, writes jsr instructions pointing to a target routine into the buffer, then fills the remaining space with a repeated word pattern before returning.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `zero_filled_data_area_2`: Contains a single zero word that decodes as ori.b #0,d0; the bytes are data, not executable code, referenced by address from other routines.
- `call_library_offset_thirtytwo`: Saves d0-d1 and a0-a1 on the stack, loads the library base from a pc-relative longword, calls the routine at offset 32 of it, restores the registers and returns.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `allocate_four_bytes_store_size`: Saves d2, adds four to d0, calls an allocation routine, and if it succeeds stores the adjusted size at the returned pointer and returns the pointer plus four.
- `pop_d0_from_a1_and_branch`: Pops a longword from the address in a1 into d0, decrementing a1 by four, then branches to a nearby routine.
- `store_a0_to_global_pointer`: Saves a1 on the stack, loads the address of a global variable into a1, stores a0 there, restores a1 and returns.
- `call_then_scale_d0_and_set_a1`: Calls a nearby routine, shifts the returned d0 left by two bits, copies d0 into a1, adds four to a1 and returns.
- `copy_workbench_title_to_buffer`: Saves registers, calls a setup routine, queries the Workbench library for its title string, copies that string into a global buffer while counting its length, then calls the string output routine and restores registers.
- `install_audio_vector_hooks`: Reads the audio interrupt vector table, saves the original handler addresses into globals, and replaces five vector entries with the game's own interrupt routines before returning.
- `get_stored_pointer_global`: Loads the longword stored at the global variable into d0 and returns it to the caller.
- `save_regs_call_848_return_a6`: Pushes d0-d7 and a0-a5 onto the stack, calls the routine at offset 848, moves its d0 result into a6, restores all saved registers and returns.
- `scan_buffer_for_matching_bytes`: Compares d0 bytes starting at a0 against bytes at a2, advancing a0 until a match or a1 limit, then returns a0 or zero in a0.
- `stack_frame_trampoline_to_1fd8`: Swaps d0 with a6 to adjust the stack pointer, reserves six bytes, pushes a6-relative value and a small constant, then jumps via the pointer at 1fd8.
- `push_pea_724_and_jump_via_1b92`: Saves d2 on the stack, copies a0 into d2, pushes the address of the literal at offset 724 relative to pc, then pushes the longword at offset 1b92 relative to pc and returns through it.
- `conditional_push_args_and_return_via_1fd8`: Tests d0; if nonzero it returns immediately, otherwise pushes word 1, saves d2, pushes word 10, pushes the longword at offset 1fd8 relative to pc, adds four to the stack pointer and returns.
- `check_dos_suffix_and_dispatch_0848`: Saves d2 and d3, builds a longword from the four bytes at a1 and compares it with the dos.
- `check_d0_and_tail_call_1fd8`: Tests d0 twice and returns if nonzero; otherwise pushes d2, pushes a word and a return address derived from the code near 1FD8, then jumps there.
- `compare_a1_long_to_dos_marker`: Compares the longword at a1 with the marker value for '.sod'; if equal, saves all registers, calls the routine at 848, restores registers and returns.
- `build_long_from_a1_bytes_compare_marker`: Assembles a longword from the four bytes at a1 into d0, compares it with the '.sod' marker, and if equal saves registers, calls the routine at 848 and restores them.
- `open_library_and_call_resident`: Builds a library name string on the stack from a1, opens that library via exec, then walks the returned base for a marker word and calls two library vectors before restoring the stack.
- `compare_a1_to_saved_library_base`: Loads the saved library base from pc-relative memory into d0, compares it with a1, and returns zero in d0 only when they are equal.
- `jump_to_pc_relative_vector`: Pushes the longword stored at a pc-relative address onto the stack and returns, transferring control to that address.
- `install_dos_vectors_and_init_tables`: Allocates a 1050-byte block via a helper, stores it in a global, then writes jump instructions and target addresses into fixed offsets of that block to hook many DOS vectors, and finally patches several pointer tables with shifted addresses before calling an init routine.
- `strip_spaces_and_lookup_dos_command`: Copies the string at d1 to a buffer replacing spaces with zero bytes, appends a newline to a second buffer, calls the DOS library function at offset -150 with the buffer, and if a nonzero result is returned indexes into the returned table.
- `call_a1_and_return_minus_one`: Calls the subroutine whose address is in a1, then sets d0 to minus one, copies it into d1 and returns to the caller.
- `return_zero_in_d0_and_d1`: Loads zero into d0 with moveq, copies d0 into d1, then returns to the caller.
- `zero_filled_data_area_twelve_words`: Twelve zero words decoded as ori.b instructions; this is a zero-filled data area referenced by address, not executable code.
- `zero_filled_data_table_sixteen_words`: Sixteen zero words decoded as ori.b instructions; this is a zero-filled data table referenced by address, not working code.
- `store_d1_to_shared_pointer`: Loads the address of a shared longword into a0, stores d1 there, copies d1 into d0 and returns.
- `load_shared_longword_into_d0_d1`: Reads the shared longword at the fixed address into d0, copies d0 into d1 and returns.
- `zero_filled_data_area_3`: Contains only zero words that decode as ori.b with zero immediate, so this is a zero-filled data area referenced by address, not executable code.
- `call_allocator_then_read_hook`: Moves d1 into a0, calls a subroutine, loads a pointer from pc-relative memory into a1, calls offset 88 of that structure, then calls another subroutine and returns its result in d1.
- `parse_iff_chunk_stream_into_linked_list`: Opens a file via dos.library, reads tagged chunks into a buffer, builds a linked list of chunk records, and dispatches on chunk type identifiers to process each entry.
- `apply_chunk_offsets_to_loaded_data`: Reads chunk records from the open file, validates a signature word and marker, then adds computed offsets into the loaded data block and frees temporary buffers.
- `decode_bit_flags_to_d1`: Tests bits 30 and 31 of d0 and sets d1 to 2, 4 or 0 accordingly, then sets bit 16 of d1 and returns.
- `walk_linked_list_via_d1_index`: Uses d1 as an index into a table, follows a chain of pointers shifted left twice, calls a subroutine on each node, and returns the last node in d1.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `load_progdir_string_and_jump`: Loads the address of the PROGDIR: string into a1, overwrites a1 with d1, and branches to another routine.
- `create_named_message_node`: Checks the name in d1 for reserved prefixes, allocates a node sized to the name plus 52 bytes, stores d2 and the copied name, then opens a message port and returns the node address shifted right by two.
- `load_magic_constant_into_d0_and_d1`: Loads the constant value into d0, copies it into d1, and returns to the caller.
- `multiply_divide_swap_d3`: Saves d3, multiplies d3 by the low word of d4, calls a helper, restores d3, then divides d0 by the low word of d3 and returns the quotient and remainder packed.
- `advance_object_position_by_delta`: Scales d1 by four to index an object, clamps d3 to the remaining distance to a limit, calls a library routine via a2, adds d3 to the object's position field and returns the delta in d0 and d1.
- `reset_object_position_and_reload`: Rejects the sentinel value in d1, otherwise scales d1 by four, adds d3 to the object's position field, calls a library routine to recompute a value and stores it in the object's field at offset 20.
- `return_d3_as_d0_and_d1`: Copies d3 into d0, then copies d0 into d1, returning the same value in both registers to the caller.
- `dispatch_object_method_by_index`: Checks d1 for the sentinel value, then uses the doubled index to fetch a method pointer at offset 24 of the object and calls it with the shifted index in d1.
- `call_object_vtable_entry`: If d1 is nonzero, doubles it into an address, loads the pointer at that address into d0 and calls it, then returns 1 in d0 and d1; otherwise returns 0.
- `adjust_object_field_by_delta`: Uses d1 as an object index and d3 as a mode selector to add or set the value at offset 48 from d2, comparing against the limit at offset 20.
- `init_dos_file_lock_struct`: Saves a1, writes zero to the first two longs of the struct pointed to by d2, stores 82, 10000, the ASCII tag DOS, and two more zero longs, then returns -1 in d0 and d1.
- `repeat_call_until_d1_exhausted`: Tests d1; if nonzero, calls the routine at the given address, decrements d1 by one and repeats until d1 reaches zero, then returns.
- `zero_three_longs_at_d1`: Copies d1 into a0, clears three consecutive longwords starting at that address, returns the original pointer in d0, and returns to the caller.
- `allocate_and_init_string_node`: Saves registers, measures the null-terminated string in d1, allocates a rounded block via the memory allocator, stores size and d2, copies two strings into the node, opens a resource through the library vector and returns the node address shifted right by two in d0 and d1.
- `return_zero_and_jump_back`: Loads zero into d0 as a failure return value and branches back into the caller's continuation, giving an empty allocation result.
- `clone_node_with_new_string`: Frees the previous node, allocates a fresh block, clears the string byte, copies the caller's string into the new node, stores d2 and a sentinel word, then frees the old node and jumps back to the allocation loop.
- `replace_node_string_and_reallocate`: Copies the node string, calls the library open routine with selector four, frees the old node, allocates a new block, copies the string from d5, stores d2 and a sentinel word, then jumps back to the allocation loop.
- `save_d5_clear_d0_and_redraw_a3`: Clears d0, calls a routine with d1 set to 204, saves the result in d5, then calls another routine with d0 from (a3) and a1 set to a3 before restoring d0 from d5 and branching onward.
- `store_magic_and_code_address_in_object`: Writes the constant marker to offset 12 of the object at a3, computes the address of the code near the current pc shifted right by two, and stores it at offset 16 before returning.
- `get_pc_relative_base_plus_offset`: Loads the longword stored at the PC-relative address into d0, adds 308 to it, and returns the resulting pointer in d0.
- `allocate_and_copy_block_by_index`: If d1 is zero it allocates a block and stores a marker; otherwise it allocates memory, copies d1 bytes from the indexed source into it, shifts the size right by two, and returns the pointer in d0.
- `return_zero_in_d0_and_d1_2`: Sets d0 to zero with a quick move, copies d0 into d1, and returns, giving both registers a zero value.
- `allocate_62_byte_block_and_mark_negative`: Calls the allocator with size 62 and flags 65536, then if it returns a pointer stores 62 at that block and writes -2 to offset 20 of a1 before branching back.
- `scale_offsets_and_test_string_byte`: Saves a2-a3, doubles d1 twice and d2 twice, loads a2 from 48(a3), tests the byte at a2 and branches to one of two calls, returning -1 in d0 and d1.
- `call_with_inverted_23_and_return_zero`: Loads 23 into d1, inverts its low byte, calls the routine at c9a, then sets d0 to zero and branches back to the shared exit.
- `load_file_into_d2_buffer`: Calls a helper with a1 from offset 48 of a3 and a0 from offset 52, invokes a library routine at offset 36, increments the word at offset 20 of a3, then fills the d2 buffer with status fields and copies a null-terminated string from offset 48 of a3.
- `store_file_size_and_copy_name`: Writes d0 into offset 124 of a2, sets status fields at offsets 0, 4, 120 and 128 of a2, then copies a null-terminated string from offset 48 of a3 into offset 8 of a2 and returns zero in d0.
- `return_minus_one_constant`: Loads minus one into d0 and returns immediately, providing a constant negative result to any caller.
- `zero_word_data_table`: Sixty-four words of zero bytes decoded as ori.b instructions; this is a data table referenced by address, not executable code, and it changes no registers.
- `append_path_with_slash`: Copies the null-terminated string at a0 into the buffer at a2, appends a slash if missing, then appends the string at a1, preserving a0-a2 on the stack.
- `build_file_entry_from_name`: Uses d1 and d2 to locate a structure, fills its name buffer with zeros, calls a path builder, and on success copies the name into the output record and sets its type fields.
- `open_file_and_fill_io_request`: Calls the open routine at offset 36 of the library vector with the name at a3+52, then stores the returned handle and status fields into the d2 io request block and copies the name into its buffer.
- `push_error_args_and_jump`: Pushes the word value -102 and the address of a nearby string onto the stack, then branches away to shared error-reporting code.
- `dispatch_command_by_colon_check`: Reads the current process structure, checks its command byte and field, scans the a0 string for a colon, then calls the parser and the follow-up routine with the resulting pointer.
- `data_table_of_zero_words`: A block of zero words decoded as ori.b instructions that change nothing; it is a data table referenced by address, not executable code.
- `scan_string_until_colon`: Saves d0 and a1, walks a0 forward copying bytes to d0 until a null or colon is found, then restores registers and returns with a0 at the terminator.
- `build_and_merge_indexed_record`: Scales d1 by four into a3, copies d2 to d3, builds a 52-byte offset pointer with length 1005, then calls three helpers to allocate, merge and free the record.
- `check_deaf1111_magic_in_d1`: Sets d0 to zero, compares d1 against the magic value -an address, and sets d0 to -1 only when d1 matches, then returns.
- `find_task_and_set_this_task_field`: Calls exec FindTask with a1 cleared, reads field 152 of the returned task, and stores d1 there when d1 is zero or its indexed field is negative, returning the old value in d1.
- `return_pcrel_address_in_d0`: Loads the address of a nearby data location into a0, copies it to d0 as the return value, and returns to the caller.
- `strip_filename_after_last_slash`: Scales d0 by four, checks the byte at offset 52 for colon or null, then scans the string backwards and replaces the last slash with a null terminator, returning the index in d1.
- `terminate_string_at_a0`: Writes a zero byte to the address in a0, copies d0 into d1, and returns to the caller.
- `return_zero_in_d0`: Loads zero into d0 as the return value and branches back into the preceding routine's tail.
- `find_last_path_separator`: Scans the null-terminated string pointed to by d1 forward, then walks backward until it finds a slash or colon, returning a pointer to that separator or d1 if none is found.
- `return_pointer_after_separator`: Adds one to the address in a0 and returns the resulting pointer in d0, skipping past a path separator character.
- `return_minus_one`: Loads the value minus one into d0 and returns immediately to the caller.
- `copy_string_from_index_or_default`: Copies a null-terminated string from a table entry selected by d1 (or a default) into the buffer at a0, stopping after d3 bytes or at the terminator, then returns -1 in d0 and d1.
- `copy_path_into_stack_buffer`: Saves d1-d2/a3, reserves 256 bytes on the stack, copies bytes from a0 into that buffer until a null, then checks whether the last copied character is a slash or colon.
- `copy_file_data_after_open_check`: Clears a byte before a3, calls a library routine via a2, then copies up to d2 bytes from the opened file buffer to a1, stopping at a null or when the count runs out, and restores registers.
- `init_library_and_jump_to_main`: Pops a1 from the stack, stores a0 in a global, sets a5 as the library base, opens the intuition library, sets up a screen, then jumps through the library vector to the main entry.
- `set_two_vectors_above_base_and_call`: Loads the base pointer from memory, adds an address to get a4, stores a4 at offsets 6 and 18 of a5, adds an address more, calls a subroutine, then jumps through a5.
- `load_a3_with_one_megabyte_and_jump`: Loads the immediate value an address into address register a3 and then jumps to the address held in a5.
- `read_and_mask_display_flags`: Loads the longword at the program counter relative address into d0, clears bits 4 through 6 with a mask, and returns the masked value.
- `acknowledge_interrupt_from_stack`: Reads a word from the top of the stack, writes it to the interrupt request hardware register, advances the stack pointer by two, and returns.
- `build_return_frame_with_a3_pointer`: Makes room on the stack, copies the return address and a3, loads a program counter relative pointer into a3, stores it, sets d0 to zero and d1 to a2, then returns.
- `set_a1_flag_bit1_and_chain`: Pops return address into d1, pushes a fixed pc-relative address, pushes d1 back, sets bit 1 of the byte at offset 14 from a1, then returns.
- `load_custom_chip_base_a0`: Loads the custom chip register base address into a0 and branches to a shared continuation routine.
- `store_d0_to_pc_relative_slot`: Stores d0 into a pc-relative memory slot, loads the cia-b base address into a6, then returns to the caller.
- `jump_through_cop2lch_vector`: Loads the longword at the program-counter-relative location into the Copper low interrupt vector at a fixed address, then jumps through address register a0.
- `copy_byte_from_a0_offset_to_a6_800`: Reads the byte at offset 6 from the structure pointed to by a0 and stores it at offset 2048 from a6, then returns.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `check_d0_1fc8_equals_21000`: Returns 4 in d0 if the longword at the program-counter-relative location equals a fixed address, otherwise sets d0 to 1 and returns.
- `init_negative_two_markers_and_clear_flag`: Writes the value minus two to (a3) and the next longword, stores a3 into offsets 132 of a4 and 50 and 54 of a2, clears the word at offset 386 of a4, skips the return address and returns.
- `copy_child_node_pointers_to_a3_offsets`: Reads longwords at a1 offsets 4 and 8; if nonzero, loads each as a0 and copies the longword at a0 offset 4 into a3 offsets 50 and 54 respectively, then returns.
- `fill_three_longwords_with_minus_one`: Advances a3 by twelve bytes, then stores minus one into three consecutive longwords below a3 using a dbra loop over d0, leaving a3 unchanged, and returns.
- `compute_scaled_position_into_a2_offset`: Reads three longs from the table pointed to by the pc-relative pointer in d0, multiplies the first by 1440, adds the second, scales by 60, adds the third divided by 50, and stores the result at offset 198 of a2.
- `select_tile_byte_from_flags_and_index`: Clears byte 31 of a1, tests bit 1 of byte 64 of a3 to set it to 29, otherwise calls a routine and returns byte 31 of a1 in d0.
- `dispatch_tile_action_when_type_two`: If byte 29 of a1 equals 2, loads a byte from a pc-relative table indexed by d1, then calls the routine at offset 40 of the pointer in a1 with d0, d1 and a0 set from a1 fields.
- `check_bit_and_set_error_code`: Reads a byte from a nearby constant table, tests the bit selected by d1, and if set stores the value 28 into offset 31 of the structure at a1 before branching away.
- `format_digit_and_call_disk_vector`: Loads a nearby string, converts the digit indexed by d1 into an ASCII character at offset 5, then loads fields from a1 and calls the function pointed to by the disk vector at a2.
- `return_absolute_address_plus_offset`: Pushes a longword from a nearby constant location onto the stack, adds a fixed offset to it, and returns, leaving the computed address as the return value.
- `get_1fd8_pc_relative_address_plus_32`: Pushes the longword at the program-counter-relative location 1fd8 onto the stack, adds 32 to that pushed value, and returns it to the caller.
- `wait_for_copper_vertical_blank_toggle`: Polls the custom chip register at offset 5 until bit 0 is set, then polls again until bit 0 clears, then returns.
- `zero_filled_data_area_4`: This single zero word decodes as an ori.b immediate instruction but is actually zero-filled data referenced by address, not executable code.
- `zero_filled_data_area_5`: A single zero word that decodes as ori.b #0,d0; this range is zero-filled data referenced by address, not executable code.

### Installer

- `rnc_decompressor_entry_loop`: Program entry that walks the resident module list, finds the RNC signature, copies the packed block backwards, then decompresses it into memory and repeats.
- `huffman_bitstream_decode_symbol`: Reads bit pairs from the a0 table, consumes bits from d6 using d7 as counter, refills via the bit reader and returns the decoded symbol in d0.
- `load_ff_and_10_into_d0_d1`: Loads the immediate value 255 into d0 and the immediate value 16 into d1, then falls through to the following routine.
- `mask_d0_with_d6_and_conditional_bsr`: ANDs d0 with d6, subtracts d1 from d7, branches past a subroutine call if the result is non-negative, otherwise calls it, then shifts d6 right by d1 and returns.
- `unpack_bytes_from_a3_into_d6`: Adds d1 to d7, shifts d7 right into d6, swaps d6, advances a3 by four, reads two bytes backwards from a3 into d6 with a byte rotate, adjusts d1 and d7, then returns.
- `build_huffman_decode_tables`: Reads symbol lengths from a stack buffer and fills a0 with per-length code counts, first-code values, symbol offsets and a bit mask table for decoding.
- `relocate_hunk_reference_blocks`: Walks a4 through hunk blocks, zeroing entries and adding the loaded base plus four to each referenced offset in the a5 relocation table.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### QuestII

- `decrypt_and_relocate_hunk_executable`: Scans the hunk list for the RN C magic, copies code blocks, decrypts bytes with a rotating key, then jumps to the relocated entry point.
- `decode_questii_bitstream_entry`: Reads masked word pairs from a0 until the masked value equals the next word, then consumes bit counts and masks from the record to extract a bit index, calling the refill routine when d7 runs short and returning the bit number in d0.
- `set_ff_d0_and_10_d1`: Loads d0 with minus one and d1 with sixteen as immediate values, then falls through into the following routine.
- `mask_d0_with_d6_and_refill`: Masks d0 with d6, subtracts d1 from d7, calls the refill routine if d7 went negative, shifts d6 right by d1 and returns the masked value in d0.
- `extract_three_bit_field_from_stream`: Reads bytes backwards from a3, combines them into d6 with swaps and rotates, adjusts d1 and d7 bit counts, and returns the extracted field in d6.
- `build_huffman_code_table_from_counts`: Calls the bit reader to gather symbol counts, then fills a0 with code lengths, bit patterns, symbol bytes and masks for each of the seventeen code lengths.
- `relocate_pointer_table_entries`: Scans a4 for a marker, clears entries, scales each stored index by four via a0, and adds the base offset into the a5 table.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### Stack

- `allocate_and_init_glc_resource`: Calls library vectors through a5 to find the highest resource id, allocate a block, fill it with incrementing tag values, copy sixteen longs into the structure at offset 536, and return zero or minus one.
- `return_d1_in_d0_then_jump`: Moves d1 into d0 as the return value and branches to the code at offset 260, which is the shared exit path.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_21`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_22`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_23`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_24`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_25`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_26`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_27`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_28`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_29`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_30`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_31`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_32`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_33`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_34`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_35`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_36`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_37`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_38`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_39`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_40`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_41`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_42`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_43`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_44`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_45`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_46`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_47`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_48`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_49`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_50`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_51`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_52`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_53`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_54`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_55`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_56`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_57`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_29`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_30`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_31`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_36`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_37`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_38`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_39`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_40`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_41`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_42`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_43`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_44`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_45`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_46`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_47`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_48`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_49`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_50`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_51`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_52`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_53`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_54`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_55`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_56`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_57`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_58`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_59`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_60`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_61`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_62`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_63`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_64`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_65`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_66`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_67`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_68`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_69`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_70`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_71`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_72`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_73`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_74`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_75`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_76`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_77`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_78`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_79`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_80`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_81`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_82`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_83`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_84`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_85`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_86`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_87`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_88`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_89`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_90`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_91`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_92`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_93`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_94`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_95`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_96`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_97`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_98`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_99`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_100`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_101`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_102`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_103`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_104`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_105`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_106`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_107`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_108`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_109`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_110`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_111`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_112`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_113`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_114`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_115`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_116`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_117`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_118`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_119`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_library_four_times_then_jump`: Calls the same absolute library routine four times in a row, then jumps to another absolute address; no registers or memory are touched directly here.
- `jump_vector_entry_120`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_to_runtime_handlers`: Calls the routine at the first absolute target, then jumps to the second absolute target, transferring control without returning to this code.
- `jump_vector_entry_121`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_122`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_123`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `discard_stack_frame_and_jump`: Moves the stack pointer forward by the given offset to discard a frame, then jumps to an absolute target instead of returning.
- `main_loop_with_task_and_copper_setup`: Finds a task, waits on and reads messages from a port, programs the copper list and DMA control registers, and calls seven unnamed subroutines in a loop.
- `jump_vector_entry_124`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_125`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `custom_chip_register_init`: Calls seven unnamed routines and touches the custom chip register base, setting up hardware state before returning to its two callers.
- `open_dos_graphics_and_save_dmacon`: Opens dos.library and graphics.library via OldOpenLibrary, stores both bases, copies a graphics field, calls a setup routine, saves DMACONR and enables all DMA channels.
- `shutdown_display_and_close_libraries`: Disables DMA, restores copper and interrupt vectors, removes interrupt servers, closes four libraries, frees a resource, calls a teardown routine and closes the two saved library bases.
- `enable_dma_and_set_copper_list`: Points a5 at the custom chip base, writes the DMA control register to enable all channels, reads and ORs two bits into a value from memory, then stores the copper list pointer and copper jump registers before returning.
- `install_serial_interrupt_server`: Loads the exec library base, fills an interrupt server node with priority, type and a code pointer, then calls AddIntServer, Disable and Enable before returning.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `reset_cia_and_enable_vertb_interrupt`: Sets the vertical blank interrupt vector, configures CIA port A bits and the interrupt enable register, reads a CIA register into d0, then enables interrupts and returns.
- `install_four_interrupt_vectors`: Loads ExecBase into a6 and installs four interrupt handlers for levels 7 to 10, storing each returned vector in a global variable.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `open_audio_device_and_start_io`: Creates a message port, opens the audio device, sends an I/O request with a reply port, then enables and starts the device.
- `stop_audio_device_and_close`: Disables the audio device, sends a stop command through its I/O request, removes the reply port and closes the device.
- `clear_global_pointer_field`: Saves a0, loads a global pointer, copies its field at offset 184 to another global, sets that field to minus one, restores a0 and returns.
- `copy_global_long_to_offset_b8`: Saves a0, loads the pointer stored at the global long address, copies the global long value into offset 184 of that structure, restores a0 and returns.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `open_libraries_and_init_custom_chipset`: Calls three Amiga library open functions via offsets minus 30, 36 and 42, touches the custom chip base register, calls one unnamed routine, then returns.
- `save_and_restore_custom_regs_around_load`: Saves the copper and interrupt state, loads a file through the dos library into a buffer, then restores the custom chip registers and calls three subroutines.
- `infinite_loop_writing_fff_to_1ac8`: Writes the immediate value 4095 to memory at address 6856 and branches back to itself forever, so it never returns.
- `custom_chip_register_setup`: Touches the custom chip register base at the hardware address and performs register setup; the exact instruction list was not provided for this routine.
- `call_five_runtime_subroutines`: Calls five absolute subroutines in sequence, then returns to the caller without touching any registers itself.
- `compute_tile_from_screen_coords`: Reads two words from memory, divides them by 48 and 24 to derive tile coordinates, checks a seven-entry table, and stores either minus one or adjusted coordinates to memory.
- `tile_offset_data_table`: The bytes are data, not executable code: six immediate values used as a table of tile offsets, read via lea by the coordinate routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_copper_list_pointer`: Saves d0/a1/a5, points a5 at the custom chip base, writes the current copper list pointer to COP1LCH, swaps two stored pointer values, toggles a stored flag between two constants, then calls a helper with a1 loaded from a table.
- `wait_for_vertical_blank_poll`: Saves d1, decrements d0, then polls the word at offset 6 of a5 waiting for its sign to change, looping with dbra until the condition is met, then restores d1 and returns.
- `compute_parity_bit_from_word`: Saves d2, reads the word at offset 12 of a5 into d0/d1/d2, shifts d1 right by one and XORs with d0, tests bit zero and sets d0 to one if set, then branches onward.
- `test_d1_bit8_set_d0_minus_one`: Tests bit 8 of d1; if clear it branches away, otherwise sets d0 to minus one and branches past the following code.
- `test_d2_bit1_set_d1_one`: Clears d0, tests bit 1 of d2; if clear it branches away, otherwise sets d1 to one and branches past the following code.
- `test_d2_bit9_set_d1_minus_one`: Tests bit 9 of d2; if clear it branches away, otherwise sets d1 to minus one and branches past the following code.
- `return_zero_in_d1_pop_d2`: Sets d1 to zero as a return value, pops d2 from the stack and returns to the caller.
- `unpack_256_bytes_bit_reversed_to_buffer`: Reads 256 bytes from a source, reverses each byte's bits into d1, stores each result descending into the buffer at a0, then returns.
- `cycle_counter_and_pointer_advance`: Increments a word counter, wraps it to zero after nine, reads two words from a table into d0-d1 and stores them at offset 1ba of a6, then resets a pointer when the table end is reached.
- `copy_15_palette_rows_to_384_a5`: Saves registers, calls setup, then for 15 rows copies 32 words from a PC-relative table into the buffer at offset 384 of a5, plus 4 more word pairs, then calls a finish routine and restores registers.
- `merge_nibbles_into_word_at_a0_a2`: Reads words at a0 and a1, compares nibbles from low to high, increments the differing nibble of the a0 word, then writes the merged word to a0 and a2 and returns.
- `clear_flag_and_copy_sprite_words`: Clears a byte flag, saves registers, then repeatedly copies word pairs from two tables into a5-relative memory across fifteen outer passes and four inner entries, restoring registers and returning.
- `decrement_bcd_digit_fields`: Reads a word from (a0), decrements its low, middle and high nibble fields by one when each is nonzero, writes the result back to (a0) and (a2), and returns zero in d6 if the word was zero.
- `set_copper_list_pointer_1ad6`: Saves d0 and a0, loads the address of the copper list area into a0, then writes the long value 1ad6 split across offsets 2 and 6 of that structure before restoring registers and returning.
- `set_copper_list_pointer_1ae6`: Saves d0 and a0, points a0 at the copper list area, writes the long value 1ae6 into offsets 2 and 6 of that structure, restores d0 and a0, and returns.
- `level5_interrupt_ack_and_dispatch`: Saves all registers, reads the interrupt request word from custom chip base, masks bit 5, writes it back to acknowledge, and if set calls the level 5 handler before restoring registers and returning from exception.
- `joystick_fire_and_pot_read`: Reads the CIA-A port A fire/disk bits and the POTGOR right/middle button bits, then calls five subroutines to process the resulting input state.
- `install_six_display_pointer_entries`: Stores a1 in a global, then writes two fixed longword pointers and five longwords from the a1 array into successive eight-byte display list entries at the table starting near the given offset.
- `ori_b_immediate_data_byte`: Performs a bitwise OR of the immediate byte value 182 into the low byte of d0, then falls through into the following code.
- `clear_and_shift_hero_quest_arrays`: Skips leading data bytes, then clears two blocks of 25 longwords and shifts entries from four arrays based on counts at offsets 19 and 23 of a selected record.
- `clamp_and_store_pan_values`: Reads two limits and a word from the entity, updates two stored bytes, calls a scaling routine twice, clamps results to the limits and stores them.
- `signed_byte_difference_wrap`: Sign-extends the low bytes of d1 and d0, subtracts d1 from d0, then wraps the result into the signed byte range by negating or offsetting before returning it in d0.
- `zero_filled_data_area_6`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `write_tile_words_to_two_buffers`: Reads two words from nearby data, adjusts them by mode, builds two word values in d0 and d1 with flag bits, then stores them to two buffers selected from a table indexed by a global word.
- `play_intro_sequence_with_100_frame_wait`: Saves all registers, sets up the custom chip base, runs several setup and display calls, then waits in a 100-iteration loop clearing a flag bit before restoring registers and returning.
- `play_intro_sequence_with_150_frame_wait`: Saves all registers, runs setup and display calls with a different data pointer, then waits in a 150-iteration loop clearing a flag bit before restoring registers and returning.
- `play_intro_sequence_without_wait_loop`: Saves all registers, runs setup and display calls with another data pointer, clears a flag byte, then restores registers and returns without any wait loop.
- `load_sound_sample_and_play`: Reads a word index from memory, scales it by four, loads a sample pointer and length from tables, then calls the sound routine to play the sample into the audio buffer.
- `configure_audio_channel_three`: Writes period and volume values to the third audio channel hardware registers to set up a sound.
- `clear_matching_inventory_slots`: Reads a count byte, indexes a record table, then scans twenty bytes clearing any that match one of six item codes, looping until done.
- `decode_rnc_packed_bitmap`: Saves all registers, allocates a 384-byte stack frame, checks the RNC1 signature, then unpacks the compressed bitmap into the caller's buffer using the stack scratch area and returns the result in d1.
- `decode_huffman_symbol`: Reads a 16-bit code word from the table at a0, masks it with d6, subtracts the next word, and walks the Huffman tree until a leaf, adjusting d7 and d6 and returning the symbol in d0.
- `decode_huffman_bit`: Masks d0 with the bit buffer d6, subtracts the bit count d1 from d7, refills d6 from the stream via the helper when d7 goes negative, and returns the decoded bit in d0.
- `extract_bitplane_byte_pair`: Adds d1 to d7, shifts d7 right by d6, swaps d6, advances a3 by four, reads two bytes backwards from a3 into d6 with byte swap and rotate, then computes d7 as sixteen minus d1 and returns.
- `read_four_bytes_into_d0`: Sets d1 to three, shifts d0 left by eight, then reads four bytes from the address in a0 into d0 while incrementing a0, and returns.
- `build_bitplane_run_table`: Calls a random routine twice to fill a sixteen-byte stack buffer, then loops over bit positions building run-length words and masks into the buffer at a0, storing counts at offsets sixty to sixty-two, and returns.
- `clear_globals_and_init_sound`: Clears the byte at a fixed global, zeroes d0 through d3, calls a routine that returns a pointer in a0, and stores that pointer in a global before returning.
- `configure_sound_channels_and_toggle_flag`: Calls a channel setup routine with channel numbers and values, writes a word and byte to a global, sets a flag byte, then clears the flag and byte and returns.
- `ori_word_ff01_data_word`: A single instruction that ORs the immediate value ff01 into the low word of d0; it is reached only through stored addresses, not as a normal call.
- `arm_display_timer_and_flag`: Writes the value 64 to the longword at the display timer location, sets the byte flag at the next location to all ones, stores 1 in the following byte, then returns.
- `decrement_timer_and_reload_on_expiry`: Decrements the byte counter, branches if still positive, reloads it with 1, subtracts 8 from the word timer and branches if negative, else loads 4 and a word constant and calls a routine.
- `dispatch_indexed_jump_table`: Jumps to a nearby entry, loads a table address into a6, compares d0 against successive words, and jumps through an indexed word offset from the table.
- `install_level_interrupt_and_restore_state`: Saves a0 to a data area, optionally hooks the level-3 interrupt vector and enables vertical-blank interrupts, then configures a level entry from the table and returns the new handler address in a0.
- `call_two_sound_setup_routines`: Calls two unnamed subroutines in sequence and then returns to its caller, touching no registers or memory itself.
- `set_interrupt_mask_from_d0`: Shifts d0 left by eight, compares the requested mask with the current status register interrupt bits, and writes the merged mask back into the status register.
- `check_intreq_bit5_and_acknowledge`: Saves d0/a0, reads INTENAR at custom+30, tests bit 5, then reads INTREQR at custom+28 and tests bits 14 and 5, calling the acknowledge routine if set, then restores registers and jumps to address zero.
- `mask_interrupt_and_store_level`: Masks d0 with 112, stores the result to offset 156 of a0, restores d0 and a0 from the stack and returns from the interrupt.
- `read_ciaa_icr_and_ack_intreq`: Saves d0/a0-a1, reads CIAA_ICR, stores it through a pointer, tests bit 0, optionally calls the acknowledge routine, clears INTREQ bit 3, restores registers and returns with rte.
- `restore_d0_a0_a1_and_jump`: Pops d0, a0 and a1 from the stack, then jumps to a fixed nearby address instead of returning normally.
- `jump_vector_entry_126`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `init_audio_sample_state_and_silence`: Stores the sample pointer in a5, checks a magic value to pick offsets, sets up sample pointers and a byte-scan loop, then silences all four audio volumes and clears DMA and ADKCON bits.
- `compute_sample_pointer_and_voice_params`: Advances a2 by a scaled index, computes a voice count from the distance between a5 pointers divided by thirty, and loads three word fields from a0 into d1, d2 and d3.
- `build_sprite_span_records`: For each of d0+1 entries, computes a span from d2 and d3, writes a2 pointer, half-lengths and two bytes from offsets 24 and 25 of a0 into (a3), then advances a2 by d1 and a0 by 30.
- `advance_animation_frame_and_tick`: Saves all registers, points a5 at a data block and a6 at custom chip base, increments a frame counter, calls a routine, shifts a byte pair and decrements a timer, subtracting 16 from a longword at offset 12.
- `update_copper_list_and_animation_state`: Writes four sprite pointers and positions into custom registers via a6, waits, sets bit 15 of a control word, then advances an animation index and computes a frame pointer from a5 tables.
- `advance_stream_offset_if_flag_set`: Tests the byte flag at offset 789 in the a5 frame; if set, adds 16 to the longword at offset 12 in the a5 frame, then returns.
- `copy_four_entries_and_set_callback`: Computes a source pointer from a5 offsets 8 and 12, advances offset 12 by 16, copies four entries from a5 to a6 via the copy routine, then loads a code pointer into a1 and returns.
- `decode_entry_and_apply_animation_step`: Reads a longword from a0 into the entry at a1, decodes nibble fields to copy a table row into the entry, then adjusts the entry's timer and pointer based on its type byte.
- `init_object_from_selector`: Stores 1 at offset 10 of a1, decodes d0 into a table index, looks up a value in the table at 776(a5) and writes it to 20(a1), then copies fields 6 and 10 of a1 into a2.
- `scan_table_for_next_entry`: Uses the index at 18(a1) to walk the 72-byte table at 776(a5), finds the first entry not below d0, stores it at 28(a1) and sets byte 30(a1) when it is below the value at 20(a1).
- `clear_object_field_1c`: Sets the word at offset 28 of a1 to zero and returns immediately to the caller.
- `dispatch_entity_action_by_type`: Saves a1 and a0-a1, calls a subroutine, clears the longword at offset 20 of a0, reads a masked word from offset 2 of a0, and if nonzero indexes a byte table at a1 to jump through a pc-relative offset table; then scales word at offset 26 of a0 by 25 times the word at offset 526 of a5 divided by 1600 and stores it back, restoring a1 and returning.
- `data_table_not_executable_code`: These seven words are data bytes decoded as instructions, not executable code; they perform meaningless immediate operations and bit tests on registers and memory and never return.
- `data_table_zero_fill_bytes`: These five words are data bytes decoded as instructions, not executable code; they consist of zero and one immediate operations and bit tests and never return.
- `divide_byte_by_three_pick_nibble`: Reads byte at offset 1 of a5, divides it by three, swaps to test the remainder, and branches to select either the low nibble of d1 or a shifted value in d0.
- `mask_low_nibble_of_d1`: Copies d1 into d0 and masks d0 with the low nibble value fifteen, then branches onward to the shared continuation.
- `load_word_at_a0_offset_twenty`: Loads the word at offset twenty from a0 into d2 and branches to the shared continuation point.
- `find_matching_sprite_frame_index`: Doubles d0, computes a table offset from words at offsets 18 and 20 of a0, then scans up to 37 word entries at a5+776 comparing each against the second value and returns the matching word in d2.
- `store_selected_frame_word`: Writes the word in d2 to offset 24 of the structure pointed to by a0 and returns immediately to the caller.
- `advance_animation_frame_counter`: Reads a nibble from byte 40 of a0, adds a table value at a5+728 to byte 41, and when it overflows resets it and advances the pointer at offset 42 within twice the word at offset 16, then toggles byte 0.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_vertical_blank_toggle`: Saves d0 on the stack, polls the VHPOSR hardware register until its high bit changes state twice, then restores d0 and returns.
- `copy_sprite_descriptor_to_scratch`: Saves a0-a1, stores d0 in a global, copies two longwords from a table indexed by d0 times eight into scratch memory, then calls a routine via a pointer.
- `build_level_tilemap_and_objects`: Sets up display pointers and scroll values in the a5 structure, fills several tile regions with repeated calls, loads two text strings, and adjusts a counter byte based on a global flag.
- `setup_hero_animation_frames`: Clears two flag bytes, stores d0 as an index, then fills animation frame descriptors in the a5 structure and calls the frame setup routine for each of five frames.
- `parse_string_into_offsets`: Saves registers, reads six words and a pointer from a0, indexes a table by the pointer's first longword, and scans bytes counting spaces into d5.
- `accumulate_nonspace_char_offset`: Subtracts 33 from d4, loads the byte at a2 indexed by d4 into d6, adds it to d5, increments d5 and branches back to the scanning loop.
- `adjust_d0_by_half_d5_and_set_a2`: Shifts d5 right by one, subtracts it from d0, pops d5 from the stack, pushes d0 back, loads a2 with a fixed address, tests d5 and either loads a4 from memory or branches away.
- `compute_tile_address_from_coordinates`: Builds a3 from the base pointer plus row and column offsets derived from d1 and d0, reads a byte from a0, and branches on zero or carriage-return values.
- `add_six_for_space_character`: Compares the byte in d5 with the space character, adds six to d0 when equal, then branches back to the shared continuation.
- `decode_letter_tile_and_merge_masks`: Uses d5 as a letter index to fetch a tile offset from a2, adds it to d0, then reads two rotated bit masks from a1 and a1+2160, combining them into a3 depending on bit 4 of d2.
- `merge_second_mask_into_a3`: Clears bits of the second mask in a3 with d5, restores the saved masks from a5 and a6, then conditionally sets bits in a3 with d4 based on bit 7 of d3.
- `advance_tile_row_and_loop_back`: Clears bits in a3, steps a3 back by 8000 bytes, decrements d7 as a row counter and branches back to the tile loop, then advances a1 and a3 and jumps to the outer loop.
- `pop_registers_and_return`: Adds 2 to the stack pointer to skip a word, then pops d5-d7 and a0-a6 from the stack and returns to the caller.
- `write_six_sprite_entries`: Writes six consecutive 8-byte entries to memory at a0, storing d0 split into words at offsets 2 and 6, adding 8000 to d0 each iteration, then writes a final entry from the constant 6754.
- `data_table_not_executable_code_2`: These bytes are data, not executable code: the ori.b and ori.w patterns with meaningless immediates are table values decoded as instructions.
- `or_d0_into_itself`: Performs a bitwise OR of data register d0 with itself, setting the condition codes from the result and leaving d0 unchanged.
- `custom_chip_register_setup_2`: Runs fifty instructions that touch the custom chip register block at custom+0, calling twelve unnamed subroutines to configure hardware registers.
- `branch_if_equal_else_compare_long`: Branches to a nearby address when the zero flag is set; otherwise compares the immediate value 4660 with the longword stored at the given absolute memory location.
- `check_flag_then_call_or_branch`: Branches if the zero flag is clear, otherwise calls one routine, compares the word at a fixed memory location with ten and branches if greater or equal, else calls another routine.
- `check_two_bytes_and_rts_opcode`: Compares two fixed memory bytes with values and a word with the rts opcode, returning early if any match, otherwise calls a routine and sets a status byte before returning.
- `initialize_game_state_and_tables`: Sets a flag bit, clears many state bytes and words, fills two tables via a counted loop, configures display entries, then copies three blocks through a shared copy routine.
- `copy_75_bytes_to_a5_then_disable_dma`: Copies 75 bytes from the address in a6 to the address in a5, then writes 32767 to DMACON to disable DMA and sets the status register to supervisor mode.
- `bit_test_immediate_data_table`: A sequence of immediate bit, add, subtract and logical operations on d0, d2 and d3 that decodes as data rather than meaningful executable code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `zero_filled_data_area_7`: A single zero word that decodes as ori.b #0,d0; this range is zero-filled data referred to by address, not executable code.
- `update_color15_from_scanline_counter`: Checks a signature word, decrements a counter, clamps it, then writes a shifted value to COLOR15 and a mirror address before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `build_palette_and_clear_bitmap_rows`: Reads a byte table, fills a color register list and clears or copies 128-byte rows into the bitmap buffer at the fixed destination, returning when all four rows are done.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
