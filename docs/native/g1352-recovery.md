# Game 1352 (ghouls 'n ghosts (1989)(u. s. gold)[b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01352). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1808304 top-level bytes in 64 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 40 of 40 named (1 author symbols), 40 of 40 described (13 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 172 of 172 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 433678 bytes, 4 functions |
| Independent decoder cross-check (Ghidra against capstone) | 725 of 725 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 559633 |
| code | 3650 |
| data | 526939 |
| padding | 15618 |
| unused | 702464 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 1 of 2).adf:boot | 1 | 70 | 3 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 1 of 2).adf:nofast | 1 | 19 | 1 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 1 of 2).adf:transfer | 1 | 74 | 6 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 272 | 23 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 1 of 2).adf:GHOULS.PRG (raw code file, load address unknown) | 1 | 266 | 4 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 2 of 2).adf:HIGH.T1 (raw code file, load address unknown) | 1 | 0 | 0 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 2 of 2).adf:TITLE.T1 (raw code file, load address unknown) | 1 | 0 | 0 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 2 of 2).adf:COUNT.T1 (raw code file, load address unknown) | 1 | 0 | 0 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 2 of 2).adf:LEVEL2.T1 (raw code file, load address unknown) | 1 | 0 | 0 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 2 of 2).adf:LEVEL3.T1 (raw code file, load address unknown) | 1 | 0 | 0 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 2 of 2).adf:LEVEL4.T1 (raw code file, load address unknown) | 1 | 0 | 0 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 2 of 2).adf:LEVEL5.T1 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4AFC) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 118 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ghouls 'n ghosts (1989)(u. s. gold)[b](disk 1 of 2)/Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 1 of 2).adf:GHOULS.PRG: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ghouls 'n ghosts (1989)(u. s. gold)[b](disk 2 of 2)/Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 2 of 2).adf:HIGH.T1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ghouls 'n ghosts (1989)(u. s. gold)[b](disk 2 of 2)/Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 2 of 2).adf:TITLE.T1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ghouls 'n ghosts (1989)(u. s. gold)[b](disk 2 of 2)/Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 2 of 2).adf:COUNT.T1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ghouls 'n ghosts (1989)(u. s. gold)[b](disk 2 of 2)/Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 2 of 2).adf:LEVEL2.T1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ghouls 'n ghosts (1989)(u. s. gold)[b](disk 2 of 2)/Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 2 of 2).adf:LEVEL3.T1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ghouls 'n ghosts (1989)(u. s. gold)[b](disk 2 of 2)/Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 2 of 2).adf:LEVEL4.T1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ghouls 'n ghosts (1989)(u. s. gold)[b](disk 2 of 2)/Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 2 of 2).adf:LEVEL5.T1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ghouls 'n ghosts (1989)(u. s. gold)[b](disk 2 of 2)/Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 8 never-loaded region(s), 433678 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `program_disk_read_registers` (runtime segment an address): Programs the floppy disk controller for a read by writing a disk pointer, a sync word and a control setting, then enables disk DMA with a transfer length of two. Kind: disk hardware synchronization/validation check. Pass path: the code following the register writes, specifically the sequence involving d0 manipulation (lsr/ori) which suggests a status check or checksum verification of the disk controller state..
- `update_disk_parameters` (runtime segment an address): Compares four 16-bit values from a table, updates two of them if they differ, calls another routine, writes a value to offset 36 of a structure in a1, and enters a loop that checks bit 1 of the interrupt request register while decrementing a counter. Kind: disk hardware/timing check. Pass path: the loop completes (iVar3 reaches 0) and the function proceeds to u6__sub_011A().
- `frag_bsr_move_clr_rts` (runtime segment an address): Short code fragment that calls another routine, then copies 16384 to a fixed memory address, then clears d0, then returns. Kind: disk hardware/parameter check. Pass path: the code continues to the next instruction after u6__sub_00E6.
- `frag_bsr_move_rts` (runtime segment an address): Short code fragment that calls another routine, then copies 16384 to a fixed memory address, then returns. Kind: disk hardware/parameter check. Pass path: execution continues to CUSTOM_DSKLEN assignment.
- `step_drive_position_toward_target` (runtime segment an address): Moves a stored position one step at a time toward a target, writing a direction pattern to a drive control port on each step and calling a routine. Kind: disk hardware control/track positioning. Pass path: the loop completes and the function returns 0.
- `clear_selected_bit_in_global` (runtime segment an address): Combines a byte from the input with a constant mask, clears one bit chosen by a table index, and writes the result to a global variable that is tied to a control port. Kind: bitmask/status flag check. Pass path: btst.b #an address, an address.l (bit 2 being set/cleared depending on logic).
- `write_drive_control_until_ready` (runtime segment an address): Builds a control byte by clearing one bit chosen by a stored index, then setting or clearing a second bit according to a flag. Kind: disk hardware timing/status check. Pass path: the instruction immediately following the call (lea.l an address.l, a1).
- `clear_control_bit_then_delay` (runtime segment an address): Clears one control bit chosen by a stored index and then waits while a status bit stays set. Kind: timing/hardware-based delay check. Pass path: the code continues to the next instruction (bsr.w an address) if the delay loop and CIA bit polling complete as expected.
- `write_port_values` (runtime segment an address): Writes several fixed byte values to a hardware port at a specific memory address in sequence, with NOP instructions between some of them. Kind: hardware-based timing/sequence check. Pass path: the code continues execution normally if the hardware responds to the specific sequence of writes.
- `initialize_disk_hardware` (runtime segment an address): Calls three different routines, writes several values to custom hardware registers and a structure in a1, and then enters a loop that checks bit 1 of the interrupt request register while decrementing a counter. Kind: disk hardware initialization and sync check. Pass path: the loop completes and the function returns when CUSTOM_INTREQR bit 1 is cleared (indicating the hardware sync/interrupt is successful).

## Functions


### Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 1 of 2).adf:boot

- `load_program_file_from_disk`: Program entry that opens and reads a program file from the disk device, allocates memory, opens dos.library and enters supervisor state.
- `start_copper_list_handler`: Writes the copper list pointer and copper jump registers, and is referenced as an exception handler stored in data.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.

### Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 1 of 2).adf:nofast

- `initialise_display_and_interrupts`: Program entry of 19 instructions that writes display and colour registers and the DMA and interrupt control registers, and reads the CIA-A port A register.

### Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 1 of 2).adf:transfer

- `enter_supervisor_and_start_transfer`: Program entry of 12 instructions that calls exec SuperState and then calls the routine precieve_start.
- `precieve_start`: Waits for a bit of a port register and then for a flag to read minus one, then writes interrupt, DMA and display registers and clears a global.
- `loop_transfer_until_port_clear`: Loops while a bit of the CIA-A port A register is set, calling two unnamed routines to move data, and branches back to the start routine when the bit clears.
- `poll_until_result_one`: Repeatedly calls an unnamed routine and stores its low word in memory and a global variable until the routine returns one.
- `wait_for_status_then_restart`: Waits for a port flag bit and a flag value to become set, then reinitialises the start-up registers and branches to the start routine.
- `store_word_and_call_routine`: Stores a fixed word at the address held in an address register, then calls one unnamed routine.

### Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_moveq_movea_jsr_bra`: Short code fragment that copies 18440 to d1, then loads 0 into d2, then loads 0 into d3, then copies 5120 to a6, then calls another routine, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bra_jsr_eori_clr_lea_move`: Short code fragment that jumps to another point in the code, then calls another routine, then exclusive-ors 2 into a fixed memory address, then clears a fixed memory address (2 times in a row), then loads the address of a fixed memory address into a7, then copies an address to a fixed memory address, then copies an address to a fixed memory address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `split_value_by_twenty_two_2`: Performs the same split as its neighbouring routine: divides a value by twenty-two into quotient and remainder, and stores a flag when the remainder exceeds ten, subtracting eleven from it in that case.
- `program_disk_read_registers`: Programs the floppy disk controller for a read by writing a disk pointer, a sync word and a control setting, then enables disk DMA with a transfer length of two.
- `update_disk_parameters`: Compares four 16-bit values from a table, updates two of them if they differ, calls another routine, writes a value to offset 36 of a structure in a1, and enters a loop that checks bit 1 of the interrupt request register while decrementing a counter.
- `frag_bsr_move_clr_rts`: Short code fragment that calls another routine, then copies 16384 to a fixed memory address, then clears d0, then returns.
- `frag_move_bra`: Short code fragment that copies 65535 to a fixed memory address, then jumps to another point in the code.
- `frag_bsr_move_rts`: Short code fragment that calls another routine, then copies 16384 to a fixed memory address, then returns.
- `frag_bsr_rts`: Short code fragment that calls another routine (2 times in a row), then returns.
- `scan_table_until_routine_succeeds`: Repeatedly finds the next entry in a fixed table whose first word matches a counter, calls a routine with it, and increments the counter.
- `interleave_longword_bits`: Reads 128 longwords from one table and the longwords 128 entries later from the same table, combining their bits so the even bits of one stream fill the even positions and the even bits of the other stream are shifted into the odd positions.
- `pack_bit_interleave_table_entries`: Calls a routine, then for ten table entries combines two longword fields by interleaving their bits.
- `locate_sync_words_in_buffer`: Scans a data buffer for the disk sync word and records up to ten positions in a table, backing up one word when the match is not exact.
- `step_drive_position_toward_target`: Moves a stored position one step at a time toward a target, writing a direction pattern to a drive control port on each step and calling a routine.
- `poll_input_and_clear_flag`: When two stored values differ, repeatedly calls two routines while a status bit from the input port register stays set and a counter is nonzero.
- `frag_subi_bne_rts`: Short code fragment that subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `clear_selected_bit_in_global`: Combines a byte from the input with a constant mask, clears one bit chosen by a table index, and writes the result to a global variable that is tied to a control port.
- `write_drive_control_until_ready`: Builds a control byte by clearing one bit chosen by a stored index, then setting or clearing a second bit according to a flag.
- `clear_control_bit_then_delay`: Clears one control bit chosen by a stored index and then waits while a status bit stays set.
- `write_port_values`: Writes several fixed byte values to a hardware port at a specific memory address in sequence, with NOP instructions between some of them.
- `frag_bsr_rts_2`: Short code fragment that calls another routine (3 times in a row), then returns.
- `frag_bsr_cmp_bcc_bcs_rts`: Short code fragment that calls another routine (2 times in a row), then compares d0 with 768, then branches if the carry flag is clear, then compares d0 with 512, then branches if the carry flag is set, then returns.
- `initialize_disk_hardware`: Calls three different routines, writes several values to custom hardware registers and a structure in a1, and then enters a loop that checks bit 1 of the interrupt request register while decrementing a counter.
- `frag_lea_clr_move_addi_cmp_bne_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of the memory at an offset from a0 into a0, then clears d0, then copies 16676 to d1, then adds 1 to d0, then compares d1 with the memory at a0 (advancing a0), then branches if the last result was not zero, then returns.
- `frag_lea_move_clr_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 4095 to d0, then clears the memory at a0 (advancing a0), then loops back while counting down d0, then returns.

### Ghouls 'N Ghosts (1989)(U. S. Gold)[b](Disk 1 of 2).adf:GHOULS.PRG (raw code file, load address unknown)

- `touch_ciaa_port_region`: Raw code region of 149 instructions that touches the CIA-A port A register and calls one unnamed routine.
- `run_update_sequence_until_done`: Repeats a sequence of helper calls, clearing a flag byte when it is set and waiting until a global reaches three, until a counter reaches a value from a table.
- `stub_thirteen_instruction_routine`: Thirteen-instruction routine called by one unnamed routine.
- `stub_large_undecoded_routine`: Routine of 69 instructions called by two unnamed routines.
