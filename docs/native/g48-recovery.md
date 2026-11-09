# Game 48 (StuntCarRacer v1.3 0222): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00048). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (619185 top-level bytes in 29 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 23 of 23 units identical (bytes and relocations), 91 raw-word lines |
| Functions named and described | pass, 568 of 568 named (0 author symbols), 568 of 568 described (25 by rule, 543 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 12293 of 12299 (99.95%) |
| Recovered without load address (counted in proof 1, flagged) | 256 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 15943 of 16153 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 511841 |
| code | 71050 |
| data | 35412 |
| padding | 114 |
| unused | 768 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| StuntCarRacer.Slave | 1 | 278 | 20 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 533 | 40 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 111 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6963 | 243 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4238 | 144 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3852 | 86 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 41 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 42 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Disk.1 bytes 0-256 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- StuntCarRacer_v1.3_0222/StuntCarRacer/Disk.1: raw disk image (468480 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- 2 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 256 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 97 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4636 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3519 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-menu): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-menu): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3438 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- RAM snapshot an address: 16 executed PCs whose RAM was overwritten before any snapshot match its bytes 113296-113408: recovered as a unit run at an address
- 90 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 4 first-run image unit(s) in 1 version group(s); 78 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- StuntCarRacer_v1.3_0222/StuntCarRacer/Disk.1: bytes 0-256 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: StuntCarRacer_v1.3_0222/StuntCarRacer/StuntCarRacer.Slave +3A2
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: StuntCarRacer_v1.3_0222/StuntCarRacer/StuntCarRacer.Slave +3A2
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-rs-menu)): both forms recovered; writer: runtime segment an address +5BB0, runtime segment an address +5B9C, runtime segment an address +5B8E, runtime segment an address +5B86
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-menu) and runtime segment an address): both forms recovered; writer: runtime segment an address +5BB0, runtime segment an address +5B9C, runtime segment an address +5B8E, runtime segment an address +5B86
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: runtime segment an address +5BB0, runtime segment an address +5B9C, runtime segment an address +5B8E, runtime segment an address +5B86
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-menu)): both forms recovered; writer: runtime segment an address +5BB0, runtime segment an address +5B9C, runtime segment an address +5B86, runtime segment an address +5B72
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: runtime segment an address +5BB0, runtime segment an address +5B9C, runtime segment an address +5B8E, runtime segment an address +5B86
- self-modifying code at an address (2 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- self-modifying code at an address (2 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-menu)): both forms recovered; writer: runtime segment an address +5B64, runtime segment an address +5B50, runtime segment an address +5B3A, runtime segment an address +5B26
- self-modifying code at an address (2 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- self-modifying code at an address (1 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-rs-menu)): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-rs-menu)): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- self-modifying code at an address (1 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- self-modifying code at an address (2 bytes change between runtime segment an address (play-rs-menu) and runtime segment an address): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- self-modifying code at an address (2 bytes change between runtime segment an address (play-rs-menu) and runtime segment an address): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- self-modifying code at an address (2 bytes change between runtime segment an address (play-rs-menu) and runtime segment an address): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- self-modifying code at an address (1 bytes change between runtime segment an address (play-rs-menu) and runtime segment an address): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-rs-menu)): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- self-modifying code at an address (1 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- self-modifying code at an address (1 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- self-modifying code at an address (1 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- self-modifying code at an address (1 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- self-modifying code at an address (1 bytes change between runtime segment an address and runtime segment an address (play-rs-menu)): both forms recovered; writer: runtime segment an address +5B8E, runtime segment an address +5B7A, runtime segment an address +5B64, runtime segment an address +5B50
- StuntCarRacer_v1.3_0222/StuntCarRacer/Disk.1: 3 logged loads cover 451328 bytes of the image
- StuntCarRacer_v1.3_0222/StuntCarRacer/Disk.1: chunks 4-43 (bytes 1024-11264) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.35)
- StuntCarRacer_v1.3_0222/StuntCarRacer/Disk.1: chunks 196-219 (bytes 50176-56320) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.22)
- 1 never-loaded region(s), 256 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### StuntCarRacer.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `dispatch_input_port_handlers`: Saves the incoming pointer, sets up the stack, and dispatches to two handler pointers stored at offsets 52 and 36 of that structure, then sets a global flag and calls another routine; when the flag was already set it calls handlers at offsets 28 and 80 and jumps to a routine at offset 256, otherwise it copies a block of longs and calls the handler at offset 100 before jumping to a routine.
- `reset_state_and_dispatch`: When a global long holds a specific marker it copies a block of longs from one table to another and then clears a second block of words, then it saves registers and calls two routines only when their respective global flags are nonzero, and finally calls a handler at offset 100 of a structure before storing a computed pointer and jumping to a routine.
- `store_constant_value`: Stores the constant a fixed address into a global variable at offset 36 and returns.
- `conditionally_set_and_xor`: Tests a byte at a fixed address and, when it is zero, sets it to 128.
- `invert_global_byte`: Saves a0 on the stack, loads the address of a global byte, inverts that byte with a bitwise NOT, restores a0, and returns.
- `wait_input_port_flags`: Saves registers and returns immediately when a global byte is zero, otherwise it calls two routines for each of two fixed addresses and then waits until a bit in the input port register clears before clearing a global byte, and when that bit is already set it calls the two routines and waits until the global byte clears.
- `conditionally_or_constants`: Compares a long at offset 4 of the structure in a1 with the constant a fixed address and, when equal, ORs five constants from a table into the longs at offsets 0, 40, 80, 120, and 160 of that structure before returning.
- `mask_structure_bytes`: Compares a long at offset 4 of the structure in a1 with the constant an address and, when they differ, clears the low byte of that long and of the four bytes at offsets 40, 72 and 104 of the same structure, then returns.
- `mask_low_24_bits_2`: Swaps the two bytes of the long in d0, keeps the low 16 bits as 65535, and returns that value in d0.
- `call_and_advance_pointer`: Calls the routine at offset 476, adds 2 to the pointer in a0, and returns.
- `mask_low_24_bits`: Clears the upper byte of the incoming long in d0, leaving the low 24 bits, and returns that value unchanged.
- `copy_player_string`: Checks the byte at offset 1 of a0 indexed by d0 and, when it is a space, copies eleven bytes from a global table into those locations, advancing the index, then returns.
- `classify_byte_values`: Rotates the byte in d0 right by one, inverts it, and compares it against the bytes 85, 86 and 95 stored in a table, branching to a different routine when each matches, otherwise it returns.
- `store_byte_call_handler`: Writes the byte at a global address into the structure at offset 2 of a1 indexed by d1, then calls a handler routine via a pointer stored at offset 12 of a structure in a2 before restoring registers and returning.
- `dispatch_savegame_index`: Multiplies the low 16 bits of d1 and d2 by 512, and dispatches based on the low word of d1: it calls the handler at offset 40 when the value equals 22016, the handler at offset 36 when it equals 5, and otherwise updates a stored maximum in a table and calls the handler at offset 64.
- `accumulate_adjusted_values`: Loops d0 times, adjusting a value in d7 by bit tests and incrementing it, then adds the result into consecutive longs at the address in a0 before restoring registers and returning.
- `wait_input_port`: Checks a global flag and, while the corresponding bit is set in the input port at offset a fixed address, waits, otherwise it stores a pointer at offset 1000, calls several handlers, and jumps to the address in that pointer.
- `call_structure_handler_slot`: Calls the handler routine whose pointer is stored at offset 36 of a structure in a2, then tests d0 and returns.
- `return_input_byte`: Pushes the address 65535 onto the stack as a return address, then returns the byte in d0 unchanged.

### logged disk load from Disk.1

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_words_then_jump`: Computes an address from the current program counter, copies words from one pointer to another while the source is below a limit, then jumps to a fixed memory address.
- `perform_system_initialization`: Initializes the system by calling the operating system, clears several memory regions and copies a block from one location to another, then repeatedly invokes a routine based on a status byte until it reports completion.
- `invoke_os_via_trap`: Writes the decimal value 59196 to the custom chip register at address 128 and then invokes the operating system trap, which transfers control to the init_custom_and_flip_buffer_2 routine.
- `init_custom_and_flip_buffer_2`: Initializes the custom chip and flip buffer by enabling interrupts, installing interrupt vectors and configuring the custom registers, then clears several memory regions and copies a block from one location to another.
- `xor_bit7_of_memory_block`: Loads several 16-bit values from a table by address into a global variable, then XORs bit 7 of every byte in a block starting at address 62648 until the pointer reaches an address, and finally calls two other routines.
- `data_table_of_zeros_b`: Consists entirely of instructions that OR a zero into a data register, which changes nothing, so this range is a data table referenced by address rather than executable code and does nothing when run.
- `copy_table_and_flip_bytes`: Loads a series of 16-bit values from a data table by address and stores them into a global variable, then calls two other routines.
- `init_custom_and_flip_buffer`: Enables interrupts and the blitter, installs interrupt vectors, sets up custom chip registers, and runs a loop that flips bit 7 of successive bytes in a global buffer before returning.
- `configure_cia_ports`: Writes control and data bytes to two different CIA port registers, configuring the port lines before returning.
- `check_and_clear_interrupt`: Tests bit 0 of the interrupt-enable register and, when it is not set, calls another routine, then writes the value 1 to the interrupt-mask register before returning.
- `handle_interrupt_flag`: Tests a bit in a custom interrupt-enable register and, when it is set, calls another routine, writes a value to an interrupt-mask register, and returns from the exception.
- `acknowledge_interrupt_rte`: Writes a value to an interrupt request register and returns from an exception using the return from exception instruction.
- `call_and_set_global`: Calls the routine at offset f002 and then stores the value 8 into a global variable before returning.
- `wait_vertical_blank`: Waits for the vertical blank by testing bit 4 of the interrupt request register, then sets the blitter enable bit in the CIA and returns.
- `handle_audio_interrupts`: Tests bits 7, 0, 1 and 2 of the interrupt-enable register and, for each set bit, calls a corresponding routine and stores the matching value 128, 256, 512 or 1024 into a global variable before returning.
- `check_and_clear_interrupt_2`: Tests bit 3 of the interrupt-enable register and, when it is set, calls another routine, then writes the value 2048 to the interrupt-mask register before returning.
- `call_rts_enable_interrupts`: Calls the routine at 62152 and then stores the word value 4096 into the interrupt-enable register, re-enabling interrupts, before returning.
- `call_and_acknowledge_interrupt`: Calls another routine, writes a value to an interrupt-mask register, and returns from the exception.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `read_ports_and_store`: Reads a byte from a custom register and, depending on its sign and specific bits, clears a global byte, reads another register, and stores a value into a table indexed by a masked byte.
- `init_when_flag_clear`: Checks a global byte at offset 61608 and, when it is zero, sets it to the negative 128 value while configuring four custom-port registers: setting bit 6 of the CIA parallel port, and writing the bytes 8, 130, 176 and 0 to the other three ports.
- `clear_when_port_negative`: Reads a byte from a custom parallel port register and, when that value is negative and has bit 1 set, clears bit 6 of the CIA parallel port and resets the global flag at offset 61608 before restoring the input data register and returning.
- `save_regs_and_check_serial`: Saves eight registers onto the stack, clears two data registers, calls another routine, then tests bit 5 of the serial-data register and compares two global bytes before calling a routine and restoring the saved registers.
- `run_pending_callbacks`: Tests a global flag and, when set, calls another routine and clears that flag, then tests a second flag and, when set, calls a further routine and clears it, finally restoring the saved registers and returning.
- `rotate_and_write_interrupt`: Tests a global flag and, when clear, sets it, then loops to rotate and conditionally flip bytes from a table before writing the result to an interrupt-request register and clearing the flag.
- `rotate_and_store_byte`: Loops to rotate and conditionally flip bytes from a register, then, when a bit is set and a counter differs, stores the result into a table and increments that counter.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `set_bits_and_compare`: Sets bits 57 and 84 into the byte in data register 0, compares the result against the global byte at offset 62037, and returns the modified value.
- `load_and_increment_counter`: Loads a byte from a table indexed by a counter, compares it with a stored value, and when they differ returns the byte while incrementing the counter.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `load_audio_period`: Copies the word stored in the global variable at offset 62578 into the audio period register for channel 0, then returns.
- `save_registers_and_branch`: Saves two registers onto the stack, loads a data register with a constant, and branches into another routine without returning.
- `push_regs_branch`: Pushes registers d0 and a0 onto the stack, loads the value 8 into d0, then branches to the routine at offset e58.
- `update_audio_state`: Reads the interrupt-enable register and, when a flag from a table at offset 3760 is not set, stores the shifted flag into the interrupt-mask register; otherwise it increments a counter and, once it exceeds one, loads both the audio period and the audio volume registers before returning.
- `program_audio_dma_channel`: Uses the input register to index a table, reads a value from it, and programs the audio channel period and volume registers while copying four consecutive table entries into the DMA channel setup area, then sets the high bits of both audio registers before returning.
- `set_audio_period_and_volume`: Stores the word value 15 into the audio period register and then stores the word value 1920 into the audio volume register before returning.
- `copy_blocks_derive`: Initializes a loop counter to 6 and a global variable to a fixed address, then runs two nested loops that copy bytes forward while advancing a second pointer by two each time.
- `tiny_ori_2`: Small routine of 1 instruction that merges 230 into d1.
- `set_register_bits_and_clear`: Sets several bits into data registers 0 and 1, then stores the values 4 and 2 into a global variable, compares them, and finally clears that variable before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_buttons_and_store`: Reads the input device register and derives a set of button flags by combining its bits, then sets a flag bit for each pressed button in a byte.
- `increment_global_store`: Reads a long global variable at offset a fixed address, adds 644 to it, stores the result back, then reads another global at offset a fixed address and writes it to a global at offset a fixed address, and finally stores the value 128 into the byte at offset a fixed address.
- `fill_table_incrementing`: Loads a 32-bit value from a global variable and writes its two halves into a table at a fixed address.
- `tiny_ori_3`: Small routine of 1 instruction that merges 148 into d7.
- `decrement_until_zero`: Sets the low bits of a value in a register and then decrements it in a loop until it reaches zero, otherwise it returns.
- `clear_and_copy_memory_blocks`: Clears a range of bytes in a global table and zeroes a block of words in another table, then copies a block of bytes from one memory region to a third and back.
- `copy_words_from_pointer`: Copies fifteen 16-bit words from the source pointed to by a1 to the destination starting at the global variable at address 59482, advancing both pointers, then returns.
- `transform_nibbles_to_table`: Transforms fifteen 16-bit values read from a global variable and writes the results to a table at 59248, setting bit 0 if the low nibble is nonzero, bit 4 if the next nibble is nonzero, and bit 8 if the high nibble is nonzero, then returns the last high nibble.
- `call_then_copy_block`: Calls another routine to initialize something, then copies 32000 bytes from a global variable to the block 32000 bytes later in that same variable, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_flag_set_bit`: Tests a byte global at offset a fixed address and jumps away if it is nonzero.
- `call_setup_routines_3`: Calls a setup routine and runs straight on into the following routine without returning.
- `call_setup_routines_2`: Calls the setup routine at offset 5714c and then calls the routine at offset 57140, running straight on into the next routine without returning.
- `call_setup_routines_chain`: Calls the routine at offset 57146 and then calls the routine at offset 5714c, running straight on into the next routine without returning.
- `call_setup_routine`: Calls the routine at offset 5714c and runs straight on into the next routine without returning.
- `invoke_routine`: Calls another routine and does not return.
- `push_val_branch`: Pushes d0 onto the stack, loads the value 7070 into d0, then branches to the routine at offset 72.
- `call_setup_routines`: Calls the routine at offset 5716c and then the routine at offset 57162, running straight on into the next routine without returning.
- `push_val_branch_2`: Pushes d0 onto the stack, loads the value 1414 into d0, then branches to the routine at offset 72.
- `configure_cia_registers`: Pushes d0 and a0, loads 707 into d0, points a0 at the CIA chip, writes control bytes 128 and 130 to CIA registers, stores the high and low bytes of 707 at CIA offsets 1536 and 1792, checks a status byte, and returns if it is nonzero.
- `initialize_table_values`: Clears a global word, loads the value 4694 into d0 and calls the routine at offset 57028, then loads a series of byte constants and values from a table into d0 and calls the routine at offset 5700c repeatedly, and finally calls the routines at offsets 4c and 56 before returning.
- `read_and_store_serial_command`: Reads a sequence of bytes from a serial input buffer, validates them against the expected command codes, and stores the received command bytes into a four-byte command buffer in a global variable, then returns.
- `build_flags_store`: Tests a byte global at offset 2876 and skips work if it is zero.
- `dispatch_serial_command`: Reads a command byte from a serial buffer and dispatches based on its value: for the 136 marker it calls a routine and returns, for the 193 marker it calls two routines and returns, and for the 227 marker it stores the marker, runs a 29-iteration loop that calls a routine each pass, then returns.
- `store_serial_index_byte`: Clears the low byte of d0, stores it at the global byte at offset 70071, sets bit 7 of the byte at offset 3 of the structure in a6, then loops from 3 down to 0 indexing a table at 39645 and calling the routine at 25278, saving the last index in that byte.
- `clamp_byte_in_range`: Reads a byte from a global variable, compares it with an incremented copy of another global byte, stores the value if it falls within range, and otherwise returns negative 128.
- `handle_flag_and_call_routines`: Tests a byte in a global variable and returns early if it is zero.
- `save_d1_call_restore`: Stores the 16-bit value in d1 into a global at offset a fixed address, loads 84 into d1, calls the routine at offset a fixed address, then restores d1 and the status register from the stack before returning.
- `empty_data_area_2`: Is an empty zero-filled data area that other code refers to by address and does nothing.
- `write_zero_byte`: It is a single instruction that writes the value zero to the byte at the address in a0, then returns.
- `store_byte_a1_d0`: Writes the low byte of the d0 register to the byte at the address in a1, then returns.
- `noop_byte_or_chain`: Consists of six consecutive zero-word instructions that decode as no-op byte OR operations on a data register, followed by a return.
- `copy_byte_pointer`: Reads a byte from the memory location pointed to by a0 and writes it to the memory location pointed to by a1.
- `noop_or_zero_d0`: It contains a single instruction that logically ors zero into the byte register d0, leaving its value unchanged, and then returns.
- `clear_low_byte_d0`: It clears the low byte of the 16-bit data register d0 and returns.
- `store_zero_byte_2`: Writes a zero byte to the memory location pointed to by d0.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `run_setup_and_process_flag`: Saves a global word onto the stack, overwrites it with the same value, then calls a setup routine and sets a flag byte.
- `save_restore_global_via_call`: Saves the current value of a global variable, overwrites it with that same value, calls the byte-processing routine, then restores the original value from the stack before returning.
- `process_table_bytes`: Loops over the bytes of a table pointed to by a0 indexed by d3, calling the routine at offset 594c6 for each byte until it reads the value 255, then increments the index and returns.
- `run_serial_setup_loop`: Clears a flag byte and sets a status byte, then loops calling a handler, copying a block of bytes between two tables, and invoking several setup routines before returning.
- `process_serial_command_loop`: Loads 5849 into d0 and calls the routine at 21288, loads 168 and calls the routine at 21260, clears the word at offset 2 of the structure in a6, then loops 8 times pushing a byte from a6 and calling the routine at 21260.
- `read_serial_byte_sequence`: Clears the counter at offset 28 of the structure in a6, then loops reading bytes from the routine at 21312 until it returns 22, 217, or 168, resetting the counter and writing 136 when it exceeds 10.
- `conditionally_call_with_globals`: Checks a global flag and a register value and, when both conditions hold, saves two global bytes, calls a routine, then restores the saved bytes.
- `save_restore_global_call`: It saves the current value of a global variable, calls a routine, sets that global variable to one, calls another routine, then restores the saved value before returning.
- `save_restore_global_and_call`: Saves the global word at an address onto the stack, overwrites it with the same value, calls the routine at 25774, sets the byte at offset 67942 to 1, calls the routine at 25614, loads 46 into d3 and calls the routine at 23554, calls the routine at 25652, then restores the saved word.
- `dispatch_serial_command_2`: Reads a command byte from a serial buffer and dispatches based on its value: for the 136 marker it calls a routine and returns, for the 193 marker it clears a global flag byte, then repeatedly checks a status byte and, depending on its value, sets a register, calls a handler routine, and returns.
- `dispatch_serial_commands`: Clears a global flag byte, then repeatedly checks a status byte and, depending on its value, sets a register, calls a handler routine, and returns early when the marker byte equals '@' or the byte is negative.
- `loop_check_and_copy_block`: Calls a routine and returns early if it reports zero, otherwise checks a global byte against zero and the character '@' to decide whether to keep looping.
- `compute_index_table`: Subtracts 4 from d0, multiplies the result by 5 using the original value, adds 4, stores the computed value into d2, loads 4 into d1, points a2 at the table at offset a fixed address, and returns.
- `copy_byte_range_loop`: It copies a byte from a source address to a destination address, then decrements both counters and repeats until each reaches zero, looping over the full byte range.
- `find_slot_index_loop`: For each of four slots it scans a table of up to 128 entries looking for a value matching the slot's stored value, then records the found index back into that slot and moves to the previous slot.
- `remap_index_table_loop`: It runs four iterations, each reading a byte from a table using the loop counter as an index and storing that byte as an index into a second table, then writing the remapped byte back to the original location.
- `set_status_byte_from_call`: Calls the routine at 25278 with 80 in d1, then sets the byte at offset 8 of the structure in a6 to 128 or 0 depending on the carry, and returns 128 if the byte at offset 60 of the structure in a6 is less than 2.
- `clamp_flag_from_globals`: Clears the byte at offset 1 and returns if the byte at offset 60 is zero.
- `copy_global_to_three`: It copies the contents of one global variable into three separate global variables at consecutive offsets, then returns.
- `conditional_copy_tables`: Checks a global flag and, depending on its sign, copies 71 bytes in one direction between a global variable and the address an address.
- `check_flags_and_update_byte`: Checks a flag byte, a status byte, and a control byte in global variables, then returns without acting if any check fails.
- `copy_table_bytes_2`: Tests a byte in a table and a byte in another table, and if the first is nonzero and the second is zero it copies one byte into three destinations, otherwise it copies three bytes from three source buffers into three consecutive locations of a destination buffer using the same offset.
- `copy_three_table_bytes`: Tests a byte in a table and a byte in another table, and if the first is nonzero and the second is zero it copies one byte into three destination tables indexed by the same offset before returning.
- `copy_three_bytes_dest`: It copies three consecutive bytes from a source buffer into three destination buffers indexed by the same offset, then returns.
- `copy_three_bytes_dest_2`: It copies three bytes from three source buffers into three consecutive locations of a destination buffer using the same offset, then returns.
- `copy_words_to_spaced_table`: Clears the carry of register d0 twice, loads a word from a global variable, and calls a routine with it.
- `call_routines_with_globals`: Calls a routine at offset 23174, loads a global byte into register d0, sets register d3 to 12, and calls the routine at offset 22716.
- `copy_buffer_with_mode_handling`: Uses the byte passed in d0 as an index into a lookup table to select a mode, then copies 16-bit words from a source buffer into a destination structure using two nested loops.
- `process_buffer_invert_and_mask`: Stores the byte 54 in a global variable, then repeatedly reads two words from a buffer pointed to by a0, inverts one word and ANDs it with two consecutive words from the buffer while writing the results to RAM, calls a routine, advances both pointers, and loops until a counter reaches zero before decrementing the global variable and returning.
- `test_bits_and_load_word`: Tests bit 1 of register d7 against register d4, loads a word from offset 96 of the structure in a1 into d5, clears bit 0 of d4, and finally tests bit 2 of d2 against itself before returning.
- `copy_and_mask_words_loop`: Stores the byte 54 into a global variable and sets up a pair of source pointers plus a loop counter.
- `load_and_mask_word_return`: Loads a 16-bit value from a global variable at offset a fixed address, advances that global variable by two bytes, and returns a masked combination of the original value with the incoming d0 register using the complemented value as a mask.
- `mask_d0_with_d4_complement`: Combines the incoming d0 register with the complement of the d4 register applied as a mask over a global variable at offset a fixed address, then returns the result.
- `store_and_call_routines_2`: Stores register d0 into a global variable, then calls a routine with register d0 set to 1 and another with register d1 set to 13.
- `process_flag_and_update_bits`: Checks a global flag, then calls a sequence of handler routines with different register values and clears or sets individual bits in four separate memory blocks using a computed shift amount.
- `iterate_table_calls`: Sets a global byte to 14, then repeatedly loads bytes from a table indexed by a counter and calls a routine with different register values.
- `call_process_wrapper`: Performs a bitwise OR of d0 with zero, calls the routine at offset 594c6, then increments d3 by one and runs straight on into the next routine without returning.
- `load_table_byte_check_marker`: Loads a byte from a table indexed by a register, compares it to 255, and returns immediately unless the byte differs, in which case it falls through to the following routine.
- `reduce_index_and_store_nibble`: Clears the high bit of d0 and loads a table base address, then loops without side effects while the value is below 32, subtracting 32 each time.
- `mask_nibble_and_loop`: Loads a byte from a global variable, clears its high nibble and masks the low nibble, then sets a counter to 5.
- `no_op_return`: Contains a single ori.b #0, d0 instruction that leaves d0 unchanged, so the routine does nothing and returns immediately.
- `scan_table_and_call_routines`: Saves a global byte, sets it to 128, advances a global pointer by 1760 bytes, and sets a flag byte at offset 34 of a structure to 128, then scans a table starting at offset 78986 by comparing consecutive bytes to 32 and counting how many match.
- `select_index_from_flags`: Tests two global bytes and loads a different constant into d0 for each combination, then calls a routine with that constant.
- `copy_structures_when_active`: Tests a global byte and, when it is positive, calls a routine and copies eleven structures from one table into another.
- `load_structure_from_buffer`: Reads a 16-bit index, looks up a pointer in a table, and calls a routine with a pointer to the start of the pointed-to structure.
- `copy_word_blocks_between_buffers`: Loads two base addresses from global variables, advances both by 320 bytes, and copies a block of words from the first buffer to the second using a pre-increment loop.
- `tiny_or`: Small routine of 1 instruction that merges d0 into d0.
- `copy_bytes_with_length_prefix`: Copies bytes from a source buffer to a destination buffer, handling both positive and negative length prefixes.
- `copy_compressed_tile_grid`: Reads a byte from the source pointer in a0 and, depending on its sign, copies either the byte or its negation to the destination pointer in a1, advancing that pointer by 1 or by 7 bytes.
- `pass_through_wrapper`: Preserves several registers, calls a routine, restores the registers, and returns the original input values.
- `update_palette_index_and_store`: Increments a byte counter and, based on its value and the input byte, stores the byte, sets a flag, decrements a counter, or returns.
- `call_table_and_store_registers`: Calls a routine using the address in the jump table at a fixed address, then stores the 16-bit values from registers d6 and d7 into two consecutive global words at a fixed address and a fixed address.
- `call_table_and_store_registers_offset`: Calls the same routine from the jump table at a fixed address, then stores the 16-bit values from registers d6 and d7 into two consecutive global words at a fixed address and a fixed address.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `call_lookup_until_carry_clear`: Calls the lookup routine with the value in d0 and, if the carry flag is set, calls it again until the carry flag is clear, then returns.
- `lookup_table_and_select_value`: Searches a table at 60118 for a byte matching one of the values in d0, decrementing d0 until it finds a match or underflows.
- `map_character_to_flag_byte`: Clears a global byte, then shifts the high byte of the 16-bit value in d0 right by 8 bits and compares it against the characters E, L, M, O and N, storing a different flag byte at the global location for each match before returning.
- `load_tables_and_set_flags`: Loads two 16-bit values from a table indexed by d2 into two global words, then calls two routines.
- `clamp_and_combine_values`: Clamps two 16-bit values from a table into two global words by offset 88 and offset 92, clamps two more into offset 138 and offset 142, then computes the clamped half-difference of two table values into offset 138 and the sum into offset 142.
- `call_routine_four_times`: Sets a global word to 1632, then calls a routine four times, adding 4 to the global word before each call after the first, and returns.
- `compute_and_scale_values`: Sets a global counter, calls two routines, computes eight signed combinations of four 16-bit values into another structure, increments four of them, and calls a routine four times.
- `scale_and_compute_values`: Computes eight signed combinations of four 16-bit values into another structure, increments four of them, calls a routine eight times, and finally calls two routines.
- `negate_and_store_pair_values`: Loads a 16-bit value from a global structure, negates it, and stores the result in another structure, then repeats the same operation with the next 16-bit value, and finally branches into another routine.
- `compute_differences_and_call_routine`: Computes differences between pairs of 16-bit values in a global structure and stores them in another structure, then adds and increments several of those values.
- `compute_clamped_half_difference`: Reads two pairs of values from tables indexed by a counter, computes the half-difference of each pair, clamps the magnitude to 255, and stores the signed result into a global variable.
- `compute_pointers_and_store_globals`: Computes the low and high 16-bit halves of two pointers by rotating and adjusting stored words, then repeatedly calls a routine to fill five consecutive global words with the derived values.
- `update_counter_and_store`: Copies a byte from a global variable at offset 1bb1d to the global byte at offset 1bb85, decrements the 16-bit counter at offset 1bb0c by the value in d0, and if the result is non-negative stores it back and returns.
- `shift_and_store_table`: Calls a routine, reads the 16-bit value at offset 1bb18, arithmetic-shifts it right by 3, and stores the result at offset d1 in the table starting at 1bd86.
- `store_differences_and_shift`: Reads four pairs of 16-bit values, stores their differences plus half-differences into globals, writes four results into a table, and shifts a global right by three when a flag is set.
- `compute_scaled_difference`: Loads the address of a table, subtracts the 16-bit value at offset d1 from the 16-bit value at offset d1 plus 4, multiplies by the constant 5a54e, and arithmetic-shifts the result right by 8.
- `add_store_structured_value`: Adds the 16-bit value at offset d1 in the structure in a0 to the 16-bit value in d0, stores the result at offset d2 in the same structure, and returns.
- `read_add_rotate_and_store_byte`: Reads a byte from a global location into d0, adds a byte from a table indexed by d1 to it, rotates d3 right by one bit, stores d3 into another global byte, masks d0 to its low 8 bits, and falls through into the following routine.
- `store_and_call_scaled`: Stores the value of register d0 into a global variable, calls compute_scaled_difference twice, adds 32 to registers d1 and d2, then branches to another routine.
- `update_table_globals_and_flags`: Reads two 16-bit values from a table at offset 73182 and offset 73166 using an index, stores them into a global variable, then calls two other routines.
- `compute_byte_and_call_routine`: Transforms a global word by rotating and offsetting it into a pointer, reads a byte there, adds 7, and stores it into a flag byte.
- `store_table_delta_values`: Calls the routine at offset 5c6c4, stores a word from a table into a structure at offset 1bd8e and another at offset 1bdae, then calls the routine at offset 594c6 and increments d1 by one before running straight on into the next routine.
- `check_table_byte_then_call`: Tests a flag byte and branches away if negative, then loads a byte from a table indexed by a register, returns unless it equals 255, and otherwise calls the processing routine and increments the index.
- `check_table_byte_marker`: Loads the address of a table, reads the byte at offset d1, and returns immediately unless that byte equals 255, in which case it falls through to the following routine.
- `process_font_table_entry`: Reads a byte from a global variable and, based on its value, either takes an early return, stores the constant 12 and adds it to another global byte, or otherwise computes a table index, copies two bytes from a decoded font table, and writes a block of transformed bytes to a buffer.
- `clear_and_fill_tables`: Clears a table by looping until an address is reached, clears another byte, fills a table with a constant, calls a routine, and sets several fixed globals.
- `clear_and_fill_table`: Clears a global variable and two other variables, then copies a byte from a register into a table and stores a constant byte at a fixed location for each index from 59 down to 0 while the index is below 12, otherwise it returns.
- `loop_routines_with_flag_check`: Loops over a range calling a routine, combining its result with a table entry and storing it, then loops again setting a global byte and calling three routines when a flag matches before returning.
- `handle_flag_or_index`: Reads a global byte and returns immediately if it is zero.
- `test_bits_d6_d1`: Tests bit 6 of the word in d6 against d0 and then tests bit 0 of d1 against d0, leaving both results in the condition flags and returning without storing anything.
- `handle_indexed_table_lookup`: Calls the routine at offset 3a00, then if a global byte is zero reads a table entry indexed by that byte, adds a global value to it, and stores the results into adjacent global bytes before returning.
- `or_and_test_bits`: Performs a bitwise OR of d0 with zero and then tests bit 0 of d1 against d0, leaving the result in the condition flags without storing anything.
- `test_bit_set_d2_bits`: Tests bit 0 of d1 against d0 and then sets bits 1 of d2 to one, leaving the modified value in d2 and the flags unchanged.
- `compare_and_update_flags`: Calls a routine, and if its result is not greater than 160 sets a flag byte at offset 1bbf7 to 64.
- `sort_and_copy_entries`: Loops over a range copying a byte and computing a scaled sum, then loops again comparing values and swapping them based on the comparison results.
- `read_byte_increment_index`: Reads a byte from the table indexed by d5 in a5, increments that index, masks the loaded byte to 8 bits, and returns it in d0.
- `adjust_d0_by_sign_bit`: Checks the sign bit of the global at offset a fixed address and, when negative, subtracts 16 from d0 or 1 depending on bit 6; when non-negative it adds 16 or 1 instead, returning the adjusted value.
- `load_and_adjust_table_byte`: Loads a byte from a table using an index in d5, applies sign-based adjustments and bit rotations to derive a pointer, then stores that pointer into a global variable and copies the byte into several indexed slots before returning.
- `decrement_loop_counter`: Decrements the byte at offset 1bb90 and returns immediately unless it becomes zero, in which case it falls through to the following routine.
- `copy_word_globals`: Copies a 16-bit value from the global at offset a fixed address into the global at offset a fixed address and returns without altering the flags.
- `extract_nibbles_from_table`: Computes a table index from a rotated word and a base, then reads two adjacent bytes from that table, storing the high nibble of the first and the low nibble of the second into a structure while returning the combined value with selected bits cleared.
- `dispatch_byte_update_globals`: Reads a byte from a global variable and returns it unchanged when that byte is zero.
- `scale_clamp_and_store`: Scales d0 by 256, subtracts two word globals, divides the result by 8, clamps it to 65024, subtracts it from a global at offset a fixed address, and returns a byte derived from the scaled value plus 2.
- `multiply_adjust_and_mask`: Negates d0 when a global at offset a fixed address is negative, multiplies it by a constant, adds the product to a global at offset a fixed address unless it matches a threshold, zeroes two globals, and returns a masked value.
- `scale_table_entry`: Reads a 16-bit value from a table at offset 4, negates it if a flag at offset 175 of a global structure is negative, then multiplies it by a byte at offset 187 of that structure (shifted left by 7) and adds the signed 16-bit result scaled by 64 to an entry selected by a counter in a second structure.
- `scale_and_clamp_byte`: Reads a global 16-bit counter and a status byte, derives a signed value from them, then divides it by eight and stores the result in a global byte.
- `setup_registers_and_call_routines`: Calls two library routines, stores the value 1 into a global byte, then sets up several register values and calls a third routine before saving a global pointer onto the stack.
- `process_and_render_text_buffer`: Initializes a global byte and a derived value from a table lookup, then processes characters from a buffer by copying them into a screen buffer at offset an address and toggling status bits based on character ranges.
- `empty_placeholder`: Contains a single instruction that ORs zero into a register, so it performs no data movement or control flow and is effectively empty.
- `process_indexed_byte_sequence`: Initializes a set of global variables, then repeatedly reads a byte from a table indexed by a running counter and calls a handler for it, updating the counter and a flag byte until a condition is met, otherwise it returns.
- `tiny_move`: Small routine of 1 instruction that copies 20 to d2.
- `decrement_global_byte_loop`: Decrements a byte in a global variable at offset 273 by one, then by one again, and loops back while either decrement reaches zero, repeating the outer loop until the value in d2 reaches zero, then returns.
- `dispatch_serial_command_3`: Reads a command byte from a serial buffer and dispatches based on its value, calling a routine to send a response byte and updating a status byte, then returns.
- `copy_table_bytes`: Clears a byte counter, copies one byte from a source table at offset 76800 to a destination table at offset an address, then decrements the counter and loops until all bytes have been copied.
- `store_indexed_byte`: Loads a byte from a table indexed by an 8-bit value, stores it at a global variable address, and if a status flag is set, loads another byte from a second table indexed by the value with its lowest bit toggled and stores that at a different global address.
- `process_counter_state`: Reads a global byte and returns immediately if it is zero, or if a flag byte is non-zero.
- `adjust_stunt_coordinates`: Computes a signed delta between two table-derived coordinates, scales it by a direction nibble, rotates the result by a counter value, and stores the adjusted values into global variables, branching to a second computation or a recursive call depending on a status byte.
- `combine_table_bytes_shift`: Loads a byte from a table indexed by a register using an offset in another register, combines it with the adjacent byte to build a 16-bit value with the high bit cleared, adds a 16-bit value from a global variable, and returns that sum shifted right by 5.
- `compute_clamped_offset`: Computes a scaled difference between two global values, stores it in a global byte, derives a second value from it, then subtracts a third global byte from 32 and stores the clamped result in a global word.
- `update_and_scale_table_values`: Compares a byte in a global variable with a stored copy and, if they differ, updates the copy and calls a routine.
- `adjust_global_words`: Examines a global status byte and, depending on its sign and bit 6, stores either the negation or the sum with 2048 of two global words selected by that flag, then returns.
- `xor_and_adjust_counters`: XORs the byte at offset 60 of one global structure with the byte at offset 50 of another, and if the result is negative it calls two routines and then clears the byte at offset 131 of the first structure.
- `increment_global_counter`: Increments a global byte counter, resets it to zero if it does not exceed a value in a global variable, and returns.
- `decrement_global_counter`: Decrements a global byte counter, and if it becomes negative reloads it from a global variable decremented by one, then returns.
- `clamp_scaled_delta`: Computes the difference between two 16-bit values at offsets 2 and 4 of a structure in a6, scales each by the low byte of a counter at offset a fixed address, and stores the signed magnitude of the result at offset a fixed address, clamping the magnitude to 32767.
- `scale_and_store_delta`: Doubles the byte in d1, clears bit 7 of a global byte, and if that bit was set it calls a routine.
- `accumulate_table_lookup`: Calls the routine at a fixed address three times, sums the first two results, subtracts the third, indexes a table at 21944 by a byte at offset a fixed address, and adds the scaled table entry to the value at offset a fixed address.
- `apply_table_delta`: Builds a 16-bit index from a word at offset a fixed address, reads a pair of bytes from a table using an index register, combines them into a 16-bit value, and adds or subtracts it from two words at offsets a fixed address and a fixed address depending on a flag at offset a fixed address.
- `clamp_and_reset_counter`: Reads a signed 16-bit global variable and computes its absolute value, clamping to 24 when negative.
- `init_and_dispatch_game_state_2`: Copies a block of memory from one location to another in both directions, initializes a global variable, then calls a series of setup routines and dispatches based on a status byte, otherwise it returns.
- `init_struct_and_copy_bytes`: Initializes a global structure by storing 64 and 128 into two of its bytes, then copies 2000 bytes from a source address into a destination address using a loop, and finally stores 9 and 10 into two adjacent global bytes.
- `init_and_dispatch_game_state`: Initializes a game structure by calling setup routines and clearing its fields, then dispatches based on a mode byte to update counters, flags and tables, and finally copies a byte from a table into a queue and calls the queue handler.
- `clear_and_copy_tables`: Writes zero to each byte of a table from offset 0 to 672, then copies eleven bytes from a source table into a destination table indexed by a global byte.
- `initialize_table_entries`: Loops over a range of indices, writing the word 32768 to a table at offset a fixed address and the byte 128 to a table at offset a fixed address for each index, then returns.
- `toggle_flag_by_index`: Reads a flag byte, indexes a table at offset a fixed address by another byte, and then clears or sets a bit selected by a passed byte in four flag tables, returning via a jump.
- `check_status_and_call`: Calls the routine at a fixed address, inspects several status bytes and returns early if certain conditions hold, otherwise sets two bytes and calls the routine at a fixed address.
- `poll_status_bits`: Writes a mask, calls the routine at a fixed address in a loop, and returns a masked result after testing individual bits of a status byte, decrementing or incrementing a counter depending on which bit is set.
- `manage_flag_and_push_values`: Checks a flag byte and returns early if it is set, otherwise clears it, sets two other global bytes to 128, pushes several values onto the stack, calls a routine, and then restores the stack and registers.
- `call_twice_with_twelve`: Loads the constant twelve into d0 and calls two routines in sequence, then returns.
- `flag_and_vector_dispatch`: Calls a routine at a fixed address, toggles bit 0 of a flag byte, adds 5 to a value copied from that flag, writes 6 to a byte at a fixed address, then branches based on the sign of a modified status byte to set a control byte to 128 or 0 and add 32000 to the values.
- `update_counters_and_flags`: Increments a global counter byte, adds a constant to another byte with carry handling, decrements a third counter, and calls several handler routines while testing multiple flag bytes.
- `decrement_and_flag`: Reads a counter byte, clears it and sets a flag when it reaches one, stores a bit derived from it, and returns the counter or a flag depending on several status bytes.
- `sync_table_bytes`: Compares the byte at offset d1 with the byte at offset d2 in each of three tables and, if the three comparisons are consistent, returns without changes.
- `store_byte_call_nibbles`: Stores the byte value from register d1 into a global variable, then calls a routine that processes table nibbles using that stored value.
- `conditional_update_table`: Tests register d1 and returns if it is zero or a global flag is clear.
- `process_indexed_table_loop`: Loops over a table indexed by d1, storing a byte into a global when the entry matches another global, calling the routine at offset 6fc4, then setting a global byte based on the value in d2 and calling the routine at offset 73ec before decrementing the index and returning.
- `invoke_routine_with_byte_values`: Passes the byte value 31 in register d0 to another routine, stores the byte it returns into d0, calls the routine again with that byte, then loads byte 2 into d0 and jumps to the routine.
- `check_byte_call_nibbles`: Loads a byte from a global variable into register d0 and returns immediately if that byte is zero, otherwise stores a byte into a global variable and calls a routine that processes table nibbles.
- `add_48_to_d0_and_call`: Adds 48 to the byte value in register d0 and then jumps to another routine with the modified value.
- `increment_d0_then_call_wrapper`: Adds the constant 48 to the byte in d0 and then jumps to the wrapper routine with the modified value.
- `clear_table_bytes`: Writes the value zero into the byte at offset d1 of each of three tables and returns.
- `bcd_adjust_table_bytes`: Adds 19 to a byte from a table indexed by register d1 using BCD adjust and stores the result back.
- `update_and_scale_global_byte`: Updates a byte in a global variable by adding a shifted copy of itself and calling a routine at offset 66106 when the value is nonzero, then decrements a second global byte and calls a routine at offset 6091a if it was 69.
- `store_byte_select_value`: Stores the byte 11 into register d2, loads a byte from a global variable into register d0, and returns immediately if that byte is zero.
- `store_and_select_value`: Stores the byte in d2 into a global variable, sets d0 to seventeen, and if d2 equals seven it sets d0 to eighteen and branches; otherwise it leaves d0 at seventeen and branches, forming a small loop that only repeats when a branch returns inside it.
- `store_byte_call_routine`: Stores the byte 11 into register d2, calls a routine, and returns immediately if the carry flag is clear.
- `invoke_routine_twice`: Stores a value into a global variable, then calls a routine twice with a base address first unchanged and then increased by 32000, restoring the original address after each call.
- `convert_word_to_bcd`: Returns the low two bits of a global variable if they are nonzero, otherwise converts a word from a global variable into BCD digits stored in three global variables, sets two flag bytes, calls a routine four times with different values, calls another routine five times with those stored bytes, clears the flag bytes, and calls a final routine unless a status byte is set.
- `copy_table_blocks`: Clamps a value derived from a global variable, then if two values differ it copies four spaced words from one table into a second table eight times in a loop.
- `process_table_nibbles`: Stores two input bytes into global variables, sets a flag byte, then reads nibbles from a table using an index and calls a routine for each, clearing the flag and variables as it finishes.
- `tiny_move_move_andi`: Small routine of 3 instructions that copies (a7) to d3, then copies (a7) to d3, then masks d4 with 4.
- `set_globals_and_call_wrapper`: Stores the byte 2 into the global variable at offset 68494 and the one at offset 68493, then calls the wrapper routine with the bytes 31, 5, 22, and 76.
- `store_flag_bytes`: Stores the value 128 into a global variable, then stores the byte from register d2 and the byte from register d0 into two adjacent global variables and returns.
- `call_flag_and_store`: Calls a single routine and returns.
- `conditional_table_loop_setup`: Checks a flag at offset 68590 and returns if it is set, then inspects a byte at offset 68556 and returns unless it is zero, negative, or at least 3.
- `adjust_counter_wrap`: Reads a counter from a global variable and, depending on the sign of another global variable, either decrements or increments it.
- `map_table_entries`: For four entries it reads a value from a table, clamps it, maps it through a lookup table, clamps the result again, and stores it into two destination tables.
- `init_and_call_zero_write`: Reads a global byte and, if it is zero, copies the byte at offset 1796 into another global, then stores the constants two, one hundred and forty-two, one hundred and thirty, and sixty into globals in sequence, and finally calls a routine that writes zero to the byte at the address in a0.
- `process_indexed_sequence`: Initializes two global bytes to 128 and 0, then calls a routine and branches on its result.
- `store_initial_globals_and_branch`: Stores the byte 16 into a global variable and the byte 14 into another global variable, then reads a global byte, masks it with a table byte, and branches back to the masking if the result is negative.
- `run_racer_setup_sequence`: Calls a routine, then branches on a global flag to call different handler routines, sets a global byte, and loops calling a handler while incrementing a register until it wraps to zero.
- `call_routines_increment_loop`: Sets register d2 to 11 and calls a routine, then calls another with register d0 set to 1 and a third with register d1 set to 140.
- `call_routine_and_increment`: Stores 5 into a register, loads a byte from a global variable into another register, calls a routine, increments a global byte by 1, and returns.
- `conditionally_call_routines`: Stores a register value into a global variable, calls several routines, then conditionally calls two more routines if the global variable is nonzero and returns the resulting value.
- `load_byte_check_255_2`: Loads a byte from a table indexed by register d1 into register d0, and returns immediately if that byte equals 255, otherwise falls through into the following routine.
- `process_table_byte_loop`: Loads a byte from a table indexed by register d1 and returns early if it is at least 9 or the combined table bytes are zero, otherwise loads a second byte, calls several routines with different constant values, jumps to another routine with a third byte, and otherwise decrements a counter in a loop calling a routine until it reaches zero.
- `save_byte_and_jump`: Saves the byte in d0, calls a routine, restores the byte, and jumps to another routine.
- `compare_byte_ten`: Compares register d0 against the value 10 and returns immediately if d0 is less than 10, otherwise falls through into the following routine.
- `compare_byte_ten_loop`: Compares register d0 against the value 10 and returns early if d0 is at least 10, otherwise pushes d0 onto the stack, calls a routine, jumps to another routine, and calls a final routine that increments a BCD counter in a loop.
- `shift_and_call_routine`: Pushes a register value onto the stack, shifts it right by 4 bits, calls a routine, restores the original value, masks it with 15, and jumps to the same routine without returning.
- `bcd_add_table_bytes`: Adds the byte at offset d1 to the byte at offset d2 in each of three tables using BCD addition.
- `copy_and_combine_tables`: Sets d1 to six and, if a flag byte is nonzero, loops while decrementing a counter, calling a routine and combining its result with a stored value.
- `set_value_and_call_wrapper`: Loads the decimal value 32 into the d0 register and then invokes the pass_through_wrapper routine with that value.
- `call_wrapper_with_32_value`: Stores the byte 32 into d0 and then jumps to the wrapper routine with that value.
- `flag_and_call_pair`: Tests a global byte and, unless it is zero or greater than two, sets a global byte to 128, calls two routines, clears that byte, and returns.
- `flag_and_call_routine`: Sets a global variable to 128, calls a routine, clears the global variable, and returns.
- `sort_and_update_counters`: Resets a global variable and a couple of other globals, then increments two counters indexed by a value read from a table and calls the associated handlers.
- `increment_bcd_counter`: Increments a BCD counter in d3 for each iteration controlled by d0, propagating the carry, then restores d1 and returns.
- `process_status_flag_sequence`: Reads a byte from a global variable and returns immediately if it is zero, otherwise sets a flag bit in a structure at offset 52 and calls a wrapper routine.
- `conditionally_set_flags`: Pushes d1, sets d2 to twelve, and calls a routine; if the carry is set it returns after popping d1.
- `copy_table_block`: Copies a block of bytes from one table to another, scaling an index by sixteen and decrementing a counter until it reaches zero, then restores d1 and returns.
- `copy_table_pairs_to_globals_2`: Reads a byte from a table using an index in a global variable, adds 8 to it and multiplies by 16, then stores the result into a table indexed by a second global variable.
- `copy_table_pairs_to_globals`: Copies a byte from a table indexed by d1 into a table indexed by d2, then does the same for a second table, incrementing both indices until d2 reaches twelve, after which it stores six indexed bytes from each table into a global structure and returns.
- `copy_table_block_conditional`: Compares a global variable against a control byte and, depending on the result, copies a block of bytes from one table to another in either direction, then calls a routine and returns.
- `multiply_global_by_five`: Multiplies the 16-bit value in a global variable by five, stores the product back into that variable, and returns the value shifted right by two.
- `copy_table_entries`: Initializes a global variable to negative 128 and a flag to 31250, then copies 256 bytes from a source table into a destination table using a loop that increments the pointers and decrements a counter until all bytes are transferred.
- `process_lookup_table_entries`: Initializes a global variable to negative 128 and a flag to 31250, then copies 256 bytes from a source table into a destination table using a byte counter.
- `increment_then_process_table`: Calls the wrapper routine, adds 256 to a global variable, and jumps to another routine that uses the updated value to clear several bytes and process a table with nested loops.
- `swap_table_bytes_conditional`: Swaps pairs of bytes between two tables using a counter that decrements from 127, guarding the direction with a flag, then transfers a byte between two global variables, calls a wrapper routine, and returns.
- `manage_nested_decrement_loop`: Stores the value 80 in d1, calls a routine at a fixed address, and returns immediately if that routine reports zero.
- `load_byte_and_jump`: Loads the immediate byte 51 into d0 and jumps to another routine without returning.
- `copy_byte_range_loop_2`: Stores a passed byte into a global, then either calls handler routines or copies a byte range from a source table to a fixed destination table while decrementing two counters until they reach zero.
- `copy_blocks_negate_flag`: Copies a four-byte block from one structure to another, negates a byte at offset 16417, and repeats this four times before clearing a global flag bit and returning.
- `update_flag_and_call_handler_2`: Stores the byte 128 into a global variable and clears its least significant bit, then returns early if that bit was already set.
- `copy_tables_and_process_entries`: Copies two bytes from a source table into two other tables using a decrementing index loop, then runs a sequence of setup calls that clear and set flag bytes in a global structure.
- `accumulate_globals_read_table`: Adds two global bytes, and if the sum stays below sixteen stores the shifted result in a global flag, otherwise it adds a second pair of globals, masks the low nibble into the flag, reads a byte from a table indexed by the combined value, sets or clears the carry flag, and returns.
- `expand_table_into_globals`: Reads a byte from a table indexed by d1 and stores it plus several derived values into a global structure, then indexes a second table using a computed offset and stores more bytes before returning.
- `adjust_nibble_coordinates`: Splits a byte from a table into its high and low nibbles, subtracts two global values, and depending on the sign and a flag bit negates or swaps the results, scales them by nine, adds base values, and stores the coordinates before returning.
- `adjust_nibble_wrap`: Stores two input bytes into a global structure, compares them, and adjusts a value by incrementing or decrementing either its low or high nibble, returning a byte from a table indexed by the result or 255 when the adjustment would overflow.
- `split_value_into_globals`: Splits the high byte of a 16-bit value from d0 into a global byte, then increments two 16-bit counters in a structure at a0 and stores the high and low bytes of each into separate global byte variables.
- `adjust_and_store_globals`: Adds 8192 to the 16-bit value in a global variable, masks it with 49152, stores the result back, then calls another routine.
- `clamp_and_store_value`: Reads a 16-bit value from a global variable, subtracts 192, takes its absolute value, and stores the clamped magnitude (capped at 128) back.
- `drive_library_polling_loop`: Sets a global byte to zero, calls a library routine, then loops it eight times.
- `process_and_call_libraries`: Negates two global bytes based on a flag bit, then calls another routine to read a value.
- `init_and_call_routines`: Sets a word at offset 68702 to 96 and a byte at offset 68552 to 128, indexes a four-entry table with a masked byte to store three bytes, sets several control bytes, and calls three wrapper routines before clearing the flags it set.
- `tiny_subi`: Small routine of 1 instruction that subtracts 8 from d0.
- `tiny_ori_4`: Small routine of 1 instruction that merges 4 into d4.
- `swap_table_entries`: Swaps pairs of 16-bit and 8-bit values in two tables using a loop that decrements a counter, then calls another routine.
- `process_table_indexed_globals`: Uses a byte index to look up values in two tables, then stores the low and high nibbles into two global variables and computes two scaled coordinates from them.
- `compute_table_difference`: Subtracts two global words, adds a scaled difference, looks up two entries in a table using shifted indices, and stores the signed result split across a global word and a global byte.
- `compare_and_load_value`: Compares pairs of global bytes and, if they match and a global word is zero, loads a value from another routine's address into a register and returns it.
- `bcd_adjust_and_call_wrapper`: Adds a byte from a global variable to d0 with a BCD adjust, compares the result with a byte at offset 68613, and if the result is not greater it stores the byte back and calls the wrapper routine with the bytes 31, 9, and 22 before storing the byte 4 into a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_routine_with_constants`: Copies a global byte into another global variable, then calls a routine three times with the byte values 31, 9 and 22, and finally stores the value 4 into a global byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `split_byte_and_call_routine`: Copies a global byte into another global variable, calls a routine three times with the values 31, 9 and 22, stores 4 and 2 into two global bytes, then splits the byte into its high and low nibbles and calls a routine for each before clearing both globals.
- `check_flags_then_call_routine`: Combines two global bytes and returns early unless several conditions hold, then decrements a global byte counter, calls a routine with the value 153, sets a global byte, and doubles a global word.
- `store_index_and_branch`: Stores the byte value 23 into a register, then branches to another address.
- `store_index_and_branch_2`: Stores the byte value 25 into a register, then branches to another address.
- `store_index_and_call_routine`: Computes an index from a register, stores a byte into a table at a computed offset, saves two global words on the stack, calls a routine twice with an offset that is increased by 32000, then restores the saved values.
- `loop_call_two_routines`: Loops eight times, comparing a byte in d2 with a global variable and calling one of two routines depending on the result, decrementing d2 each iteration, then returns.
- `set_and_test_global_byte`: Stores the value 3 into a global byte, tests it, and returns if it is non-zero, otherwise falls through to a return.
- `process_flag_byte_and_call`: Calls the routine at 1D44, then loops eight times reading a byte from a table at offset 4 of the structure in a4 and calling the routine at 0A8E, ORing the results together while shifting a bit mask.
- `copy_long_and_clear_bytes`: Copies the long word pointed to by a2 into the long word pointed to by a0, then clears the byte in d1 and the byte in d3.
- `clamp_and_scale_racer_byte`: Clamps a byte value to a maximum of 50, scales it into a 0-31 range, and stores it in a global variable.
- `update_racer_state_tables`: Clamps a byte value to a maximum of 50, scales it into a 0-31 range, and stores it in a global variable.
- `store_index_and_set_bits`: Saves the low byte of d1 into a global variable, then if the word at offset 64 of the a4 structure is below 128, the word at offset 0 is below 256, and that word is not zero, it sets bit 6 of two global bytes, sets a bit selected by the low four bits of that word in four global bytes, and returns.
- `store_two_routine_results`: Calls the routine at 62574 twice, storing the low byte of the first result into a word at offset 0 of the structure in a4 indexed by d1 and storing the low three bits of the second result plus 118 into a word at offset 64 of that structure, then returns.
- `compute_indexed_offsets_and_jump`: Shifts d1 right by one, adds a byte from a global variable at offset 1bbac and masks the low four bits to index a table, then uses that index to subtract a table value from a word at offset 0 of the structure in a4 and adds 32, computes a second value from a word at offset 64 plus 16, and jumps to the routine at 6A0BE.
- `set_stack_bits_and_fall_through`: Sets bits in a stack location before and after the pointer, sets bits in an incremented stack location, and then falls through into the next routine.
- `test_bits_and_clear_register`: Tests bit 1 of the longword in d6, bit 3 of d2, bit 0 of d0, bit 0 of d2, and bit 3 of d6 in that order, sets bits 0, 1, and 2 of d4, and clears d7, then returns.
- `scale_and_clamp_global_word`: Loads a signed word from a global variable, negates it if negative, and stores the result back.
- `clamp_and_apply_timer_delta`: Computes a signed value from a difference, clamps it between zero and 32767, and stores it in a 16-bit global variable and its negation in another.
- `call_sequence_and_store_byte`: Calls the routine at 64C0E, reads a byte from a table at offset 6129a indexed by a global byte at offset 1ca33 into d0 and stores it into a global byte at 643ad, then calls the routines at 64340, 64C3E and 64834 before returning.
- `scale_store_high_byte`: Reads a byte from a global variable at offset 556, masks the low byte of the incoming value in d0, multiplies the two together, stores the low byte of the product back to the global variable at offset 557, and returns the high byte of the product in d0.
- `compare_table_byte`: Loads a byte from a table at offset 6056 using the offset in d1, compares it with the value 179, and returns without storing anything.
- `check_sign_and_negate_if_needed`: Checks the sign of the value in d0 and, if negative, jumps to a routine that negates a byte.
- `negate_and_store_byte`: Negates a byte, checks a global flag and branches if set, calls another routine, then negates a register, stores it into a global byte and a register, and shifts the register before returning.
- `load_scaled_value`: Loads the 16-bit value from the table at offset 7424 using the doubled offset in d1, multiplies it by the signed value in d0, and stores the result in d0.
- `load_constant_317`: Loads the constant 317 into d0 and returns immediately.
- `scaled_table_lookup`: Doubles the offset in d1, reads a 16-bit value from a table at offset 7424 using that offset, multiplies it by the signed value in d0, and returns the 32-bit result sign-extended into d0.
- `return_absolute_square`: Squares the value in d0, takes its absolute value if the product is negative, and returns the result in d0.
- `scale_and_store_structure_values`: Reads two values from global variables and calls two routines that map them through a table with linear interpolation, storing the results at fixed offsets of a structure.
- `sum_and_store_results`: Loops three times, calling another routine for each of three global values, summing the results, storing the sum into a table, and decrementing a counter before returning.
- `store_scaled_constants`: Loads the scaled value for offset 7424 using the doubled constant 15 in d1 and stores it into the 16-bit value at offset 48400, then loads the scaled value for offset 7424 using the constant 4 and stores it into the 16-bit value at offset 48402, then loads the scaled value for offset 7424 using the constant 14 and stores it into the 16-bit value at offset 48398, and returns.
- `accumulate_table_lookup_sum`: Accumulates the three signed results returned by scaled_table_lookup for offsets 9, 12 and 15 of the table at a fixed address, stores their sum at offset 2 of the structure at a fixed address, and repeats this once more before returning.
- `accumulate_scaled_table_values`: Reads two bytes from a table at offsets 18 and 20 of the structure in a5 using an index in d2, looks up a scaled value for each, accumulates both results into d5, stores the sum as a word at offset 0 of the structure in a4, then decrements the index and repeats until it reaches zero.
- `compute_audio_divisor_and_store`: Decrements two byte counters in a global variable when they are nonzero, then, when a flag is set, computes a divisor from another value, clamps it, and writes the resulting shift count and a derived word into custom registers.
- `adjust_table_values_loop`: Toggles bit 1 of a byte at offset 7140, then loops twice reading a 16-bit value from a table at offset 7368, taking its absolute value, and if it exceeds 160 adding a shifted amount to a 16-bit value in a table at offset 7424.
- `add_scaled_value_d6`: If the value in d0 is nonzero it multiplies it by 276, shifts the result right by 8 bits, adds the value in d6, and returns the sum in d0.
- `store_inverted_table_byte`: Doubles an incoming value to use as a table index, and when the byte at that index is zero it stores the bitwise complement of the following byte into a global variable and calls a second routine.
- `call_routines_conditional_block`: Calls the routines at 0B38, 109E, 5C1D0, 1340, 0D5C, 078E, 0DB6 and 139C, then if a global byte at offset 1bb72 is nonzero calls the routines at 1888, 07E2, 0DE8, 19C4, 1908, 12F6, 0E42 and 12AC, and finally calls the routine at 1120 before returning.
- `store_computed_coordinates`: Reads pairs of 16-bit values from a global structure, computes shifted differences and scaled values, negates some of them, and stores the resulting coordinates into a structure at offsets 4806 through 4811.
- `clamp_and_accumulate_values`: Scales several global values by 238 and accumulates them into a structure, clamps one value to 1000, applies clamping against a table using a flag byte, and stores a final clamped byte into a structure at offset 3138.
- `add_scaled_global_values`: Multiplies three 16-bit global values by 238, discards the low byte, and adds each scaled result into three separate 16-bit global variables.
- `scale_global_words`: Multiplies a signed word read at offset 188 of the global structure by the byte 238, scales the product right by eight bits, and adds the result to the signed words at offsets 144, 146 and 148 of the same structure.
- `compute_scaled_offsets`: Reads a 16-bit value from a global variable and stores it into another 16-bit global variable, then reads a second 16-bit value from another global variable and stores it into a third global variable, then computes scaled differences of a 32-bit global variable with the two values and stores three 32-bit results into separate global variables, and returns.
- `clamp_and_average_time_deltas`: Clamps three time deltas to a fixed range, then for each one scales the previous value by the delta, clamps the result, and occasionally increments a shared counter and writes a byte.
- `update_clamped_difference`: Adds a global word at offset 16 to another global word at offset 66 and stores the result at offset 52, then subtracts a scaled value from the byte at offset 10 and stores its absolute value back.
- `apply_scaled_adjustments`: Reads a signed word at offset 240 and subtracts one sixteenth of it, then reads the byte at offset 126 and, when it is nonzero, adds one quarter of the original value, and finally subtracts one sixteenth of the value read at offset 244.
- `store_clamped_difference_value`: Computes the absolute difference of two global values, looks up a scaled value, and stores either the difference minus that value with a flag set or the difference unchanged with the flag cleared.
- `return_doubled_value`: Returns the word at offset 258 shifted left by one bit when the byte at offset 126 is nonzero, otherwise returns zero.
- `apply_clamped_scaling`: Selects a scale factor from a table based on flags and a signed byte, clamps a computed magnitude, then subtracts the scaled value from the words at offsets 150, 152 and 154 of the global structure.
- `scale_and_store_triple_product`: Clears a global word, derives two shifted values from a pair of globals, stores them, then repeatedly scales a masked byte by a shifted value and stores the high word of each product into three consecutive global words.
- `index_lookup_table`: Takes the absolute value of the incoming word, caps it at 255, stores it at offset 299, halves it as a table index, and stores the byte at that index into offset 301.
- `store_shifted_difference`: Reads two 16-bit values from global variables and a status byte, and if the byte is negative it shifts both values right by one, stores a modified copy at offset 118 of a structure, then returns.
- `store_scaled_difference_and_minimum`: Looks up a scaled value, stores the arithmetic shift of the difference between it and a global into a table indexed by a register, then stores the smaller magnitude of two values into a global word.
- `compute_scaled_difference_2`: Loads a word from a table indexed by a register, subtracts a global word, negates the result, shifts it right by two, multiplies by a constant, and stores the value into a structure at offset 54.
- `store_scaled_difference_value`: Looks up a scaled value, stores the arithmetic shift of the difference between it and a global into a table indexed by a register, and returns.
- `check_table_word_sign`: Loads a table base into a register, reads a word from the table at an index in a register, and returns without branching back into itself.
- `mix_table_scalars`: Reads two signed words from a table indexed by an incoming register and two words at offsets 52 and 50 of a structure, then writes a scaled sum plus 128 to offset 50 and a scaled difference plus 64 to offset 240.
- `merge_shifted_globals`: Shifts a global word right by four and another right by one, combines them with an exclusive or, and stores the combined byte and original value into two adjacent global bytes.
- `decrement_index_and_clear_registers`: Decrements an index in a structure, clears two registers, copies a register into another, and returns without branching back into itself.
- `loop_table_marker`: Calls another routine repeatedly, incrementing an index after each call, until the byte read from a table at that index equals 255, then returns.
- `call_routine_and_return_zero`: Loads a table base and a length into registers, calls a routine with four prepared arguments, clears two registers, and returns zero.
- `reset_table_index_loop`: Clears a global byte at offset 238c and a byte at offset 5b83e, then loads a byte from a table at offset 2384 into d5.
- `adjust_table_index`: Clears a global byte at offset 238c and a byte at offset 5b83e, then loads a byte from the table at offset 2384 into d5.
- `copy_table_bytes_loop`: Adds 11 to d5, then calls the routine at 594C6 three times with d0 set to 31, d4 and d5, and finally clears d0, indexes a table at offset 2384 by shifting a value left by four, and loops eight times copying a byte from a table at 1ED6B into a second table while storing a space and calling the routine at 594C6.
- `store_byte_when_nonzero`: Tests a byte at offset 10 of a structure in a0 and returns without changes when it is nonzero, otherwise loads a byte from a global variable, stores the value 32 into a byte indexed by a register, and copies a byte from one table into another.
- `store_byte_and_call`: Stores the byte value 32 into the byte pointed to by a1 indexed by d3, then loads a byte from a global variable into d0 and calls the routine at 59806.
- `call_then_jump`: Calls another routine and then jumps to a fixed address elsewhere in memory, passing control on without returning.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `copy_globals_forward`: Copies a long word and a byte from one global variable into another global variable, then returns.
- `copy_global_state_pair`: Copies the long value stored at the global variable at offset a fixed address to the global variable at offset a fixed address, then copies the byte at offset a fixed address to the byte at offset a fixed address, and returns.
- `clamp_and_update_table`: Clamps a byte read from a global variable into the range 32 to 224 and stores the result into the byte at offset 32 of the structure, then returns.
- `check_byte_range`: Loads a byte from a global variable into d0 and returns when it is at least 56.
- `clamp_byte_value`: Clamps a byte read from a global variable at offset 56 into the range 32 to 224, storing the clamped value into the byte at offset 32 of the same structure, and returns.
- `copy_byte_offset_20`: Copies a byte from the global variable at offset 32 into the byte at offset 32 of the same structure and returns.
- `compute_scaled_accumulator`: Returns immediately when a global byte at offset 1bbc4 is zero or a byte at offset 1bb74 is nonzero.
- `accumulate_offset_addition`: Calls the routine at 5A186 and the routine at 1D44, masks the result to seven bits and adds 104, then loops three times adding that value to a word at offset 6 of a table and storing the sum into a word at offset 6 of a second table before returning.
- `conditionally_update_table_state`: Returns immediately when a byte at offset 5eb76 is zero or a byte at offset 1ca35 is nonzero, otherwise loads 255 into d4 and loops calling the routine at 1D44, then reads two bytes from a table indexed by a global byte, combines them with a masked value and stores the results into a routine's data area before returning.
- `compute_scaled_clamped_value`: Clears a word at offset 18 of a structure, scales a global byte and stores a clamped result at offset 26, then calls the routine at 0A72.
- `decrement_and_store_bytes`: Reads a byte at offset 16 and subtracts 25 from it, then if the result is nonzero stores the original byte at offset 31 and the modified byte at offset 30, otherwise stores zero in both and returns.
- `clamp_table_byte`: Returns immediately when a global byte is zero.
- `noop_instruction`: Contains a single ori.b #0, d0 instruction that does nothing and is part of a zero-filled data area referenced by address.
- `compute_and_update_table`: Computes an absolute difference, clamps and scales it, then adds the scaled result into a byte in a table indexed by a counter and decrements the counter until it reaches zero.
- `decrement_and_call_routine`: Decrements a counter, then if it is nonzero clears a set of words and calls a routine before setting the counter to five and returning.
- `zero_fill_area`: Contains a run of zero words that other code refers to by address; it performs no operations and does nothing.
- `clamp_and_scale_accumulators`: Clamps three 10-bit values to a range of 0 to 1023, combines their low bytes with the sign bit into a status byte, computes three weighted averages, and applies a fixed-point scaling factor of 230/256 to four accumulator globals.
- `compute_absolute_difference`: Subtracts a word indexed by one register from a word indexed by another register, takes the absolute value, and returns it.
- `store_byte_and_index_table`: Stores a passed byte into a global variable and calls the routine at 39E8.
- `average_two_words`: Adds two words indexed by registers, divides the sum by two, and stores the result into both words.
- `fill_and_copy_table`: Fills a table with 255, then copies a byte from another table into it while calling a routine when a counter reaches a limit.
- `load_byte_check_255`: Loads a byte from a table at the offset given by d1 into d0, then returns immediately when that byte equals 255.
- `adjust_nibbles_and_call_handler`: Initializes a global variable to 30 and copies 30 words into a table, decrementing the source index while it stays positive.
- `copy_table_blocks_2`: Calls a routine at 3C1E, sets a global byte at offset 1bbbc to 128, and copies a long from offset 6a584 to offsets 6a58c and 6a588 with 32000 added.
- `update_flag_and_call_handler`: Copies a byte from a table at the offset in d1 into a global byte at offset an address, then calls the table lookup routine and the handler at 59806.
- `set_d0_to_128_and_branch`: Loads the constant 128 into register d0, then branches to another routine which doubles the value stored in a global variable into a flag byte and clears or sets individual bits in four places.
- `update_flag_and_clear_bits`: Clears a global byte, calls a handler, doubles a value from another global into a flag byte, and then clears or sets individual bits in four memory blocks based on a computed shift amount.
- `set_flag_by_table_index`: Reads a byte from a table, then clears or sets bit 4 of that value.
- `clear_global_byte`: Stores the byte zero into the global byte at offset an address and returns.
- `copy_word_blocks`: Copies a block of word-sized entries from a source table to a destination table, copying four words per inner step and repeating the copy for both a 13-iteration inner loop and a 126-iteration outer loop.
- `wait_flag_states`: Loops calling the routine at 60bae until its result has bit 4 clear, then stores the byte 5 in d2 and calls the routine at 5ba42.
- `test_flag_and_jump`: Tests a flag byte and branches away if it is set, otherwise jumps to a handler routine, leaving the routine as a short sequence of a test, a conditional branch, and an unconditional jump.
- `process_byte_and_toggle_flags`: Stores the incoming byte in d0 into a global byte at offset an address, then calls the table lookup routine and the handlers at 64b0e, 64c2e, 64340, and 64c2e again.
- `copy_and_rotate_tables`: Copies eleven bytes from a source table to a destination table, then calls a routine to rotate the table and shift a byte.
- `set_flags_via_pointer`: Sets bit 168 of a global variable and sets a bit within a byte addressed by a global variable using a shift amount derived from a register, then returns.
- `rotate_table_and_shift_byte`: Compares a counter byte against eleven and, when it is not eleven, calls two routines and then rotates a byte within a table indexed by that counter.
- `load_byte_and_store`: Calls the routine at 64AD2, then loads a byte from a table indexed by d1 into d0 and stores it into a global variable at offset an address.
- `set_global_to_ten`: Computes the byte value ten from the constant six by shifting left one bit and subtracting two, then stores it into the global byte at offset 78319.
- `find_byte_in_table`: Walks a byte table at offset an address starting at index eleven, comparing each entry with the byte in d0 and returning early when a match is found, otherwise decrementing the index until it goes negative.
- `copy_memory_blocks`: Clears a byte at offset 84 of a structure, calls the routine at 594C6 twice with the constants 31 and 5, and stores a table byte into a global variable.
- `set_global_to_four`: Stores the byte four into the global byte at offset an address and returns.
- `set_flag_bits_in_structure`: Sets bit 7 of the byte at offset 6 of the structure in a0, then sets bit 5 of the byte at offset 7 of the same structure, and returns.
- `jump_to_routine`: Loads the address 1ECAA into a0, sets d2 to 13, and jumps to the routine at 64C48.
- `call_routine_eleven_times`: Sets the global byte at offset an address to fifteen, then loops eleven times calling the routine at an address while incrementing a counter until it equals the stored value.
- `fill_interleaved_table`: Calls three external routines, sets the global byte at offset an address to 128, then fills a destination table with four interleaved words from d6 and d7 across an outer loop of one and an inner loop of eleven, finishing a partial inner iteration and clearing d1 and d2.
- `clear_and_branch`: Clears d5 and branches to the routine at 44E4.
- `interpolate_table_value`: Reads the word in d0, maps its lower fourteen bits through a table at offset an address with linear interpolation, applies a sign based on the bit at offset 16384, and returns the resulting word.
- `scale_table_by_ratio`: Takes the absolute values of the words in d0 and d3, compares them, and returns a value from a table at offset an address scaled by their ratio with a sign derived from their XOR and the sign of d3.
- `swap_min_abs_values`: Takes two signed values, converts both to their absolute magnitudes, then returns the smaller magnitude as the high word and the larger as the low word of a 32-bit result.
- `clear_globals_and_call`: Clears a byte in a global variable, copies that zero into two other global bytes, sets a global word to 65535, clears another global byte, and then calls a helper routine before returning.
- `process_racer_state_update`: Clears two global bytes and calls a routine that returns a status byte; if that byte is not the end marker it calls another routine, stores a byte from a register into a global, copies a 16-bit value and a byte into globals, then returns.
- `update_racer_state_tables_2`: Calls several handler routines, stores and reloads a byte between two globals, sets bits in a structure, and runs a long sequence of setup and update calls before returning.
- `handle_carry_and_call`: Adds a byte to another byte and, when a carry occurs, calls a routine; then calls a third routine once unconditionally and a second time when a comparison against 126 passes.
- `update_vehicle_movement_state`: Reads a vehicle index from a global variable and loads its position fields from a per-vehicle table, then updates the vehicle's movement state, sprite pointer, and animation frame.
- `compare_table_and_scale_difference`: Loads two table bases into registers and masks a byte from d1, then reads a byte from a table indexed by that value.
- `shift_and_copy_tables`: Shifts a byte left by four and, when it is negative, copies twelve words from three paired tables using decreasing source and increasing destination indices, then stores two into a global byte.
- `index_table_and_call`: Clears the word in d0, loads a byte from a global variable into it, scales that value by four, and uses it to index a jump table.
- `adjust_index_and_update_table`: Adjusts a byte argument by subtracting four, then checks two 16-bit entries in a table indexed by that byte in a6.
- `store_and_call_routines`: Stores the byte from d1 into a global variable, writes the value 512 to a word at offset 120 of a table indexed by that byte, then calls two routines before restoring the registers and returning.
- `update_vehicle_position_fields`: Decodes two 16-bit values from a global table into pointer registers, then walks a per-vehicle table updating two 16-bit fields with data from two other tables, using a stack slot to compare adjacent entries and setting a flag bit when the difference is large enough.
- `increment_byte_call_routine`: Returns immediately when a global byte is nonzero, otherwise increments that byte by 2, 120, and 122 in turn while calling another routine with the words 8, 9, 10, and 11.
- `update_slot_velocity_and_clamp`: Shifts the incoming value in d3 left by two and stores the result at a global long, saves the byte in d1 to a global byte, then compares that byte against 120 and, when it is below 120 and its table entries stay under the 256 and 128 limits, calls two other routines; otherwise it writes the constant an address to a long indexed by the shifted value.
- `update_slot_position_and_clamp_tables`: Initializes a slot's velocity to zero or 512 depending on a flag, updates the slot's position by a scaled velocity, and then clamps two table values to 256 and 128 by repeatedly subtracting their stored deltas until they fall within range.
- `update_vehicle_position_fields_2`: Initializes a per-vehicle table by decoding two 16-bit values from a global table into pointer registers, then repeatedly reads a byte from a table at offset 48768, adjusts it using values from the vehicle table, and stores a packed 16-bit result into a growing output table until a counter reaches its limit.
- `init_slot_table_and_copy`: Fills a table at a fixed address with the byte 120 for every slot counted down from a global value, then reads pairs of bytes from another table using a shifted global index and copies the second byte into the first table while stopping early if a sign bit is set.
- `process_linked_list_entries`: Walks a linked list of structure pointers, copying each entry's value into a table and updating status flags and linked fields based on comparisons, then returns when a counter condition is met.
- `process_table_values`: Reads a 16-bit value from a table indexed by a global variable and stores it at offset 346 of a structure, then applies a scaling multiply and compares the result against a stored value to set a status flag at offset 300 or 512 of that structure, otherwise it returns.
- `is_empty_routine`: This routine is empty; it consists of a single ori.b #0, d0 instruction that is part of a zero-filled data area referenced by address.
- `process_scaled_table_entries`: Loads a value from a table indexed by a global variable and stores it into a second table and a structure field, then scales the global variable by a factor derived from itself.
- `noop_write_zero`: Performs a no-op write of zero into the byte pointed to by d0, then returns.
- `store_byte_via_pointer_2`: Writes the low byte of the d0 register into the byte at the address in a0, then returns.
- `update_global_byte_and_toggle_table_bits`: Reads a global byte and, depending on its low bits, increments or decrements it, then sets one of several constant values in a second global byte.
- `copy_byte_indexed_loop`: Reads a byte from a global variable and stores it into a global byte array indexed by a counter, then decrements the counter and repeats while the counter is non-negative.
- `combine_masks_from_table`: Masks a register by 15, shifts it left by two, and uses it as an offset into a global table while combining three word values read from a structure via a pointer, then returns the result.
- `update_counters_and_call`: Checks two global bytes and returns early if they are zero or equal to 96, otherwise decrements a global counter, adds 8 to another, and calls a routine four times in two pairs while looping until the values drop below 16.
- `set_bits_from_register`: Tests the low bits of a register one at a time and clears or sets bit 6 of two global bytes accordingly, then returns the high nibble of the register.
- `set_bit_in_four_locations`: Masks a register and, depending on whether the result is below 8, sets the same bit in four global byte locations at offsets 1, 8001, 16001 and 24001, or otherwise sets it at offsets 0, 8000, 16000 and 24000.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_bits_in_structure`: Clears a bit at offset 1, sets a bit at offset 8001, clears a bit at offset 16001 and sets a bit at offset 24001 of a structure via a pointer, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_bits_in_structure`: Sets the same bit in a structure via a pointer at offsets 8001, 16001 and 24001, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_bit_pair_via_d0`: Clears one bit selected by the low three bits of d0 in the byte at offset 3e81 of the structure in a0, then sets the same bit in the byte at offset 5dc1 of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_bit_at_5dc1`: Sets one bit selected by the low three bits of d0 in the byte at offset 5dc1 of the structure in a0, then returns.
- `set_bit_at_four_offsets`: Reduces d0 to its low three bits, then sets that same bit in the bytes at offsets 0, 1f40, 3e80 and 5dc0 of the structure in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `mixed_bit_set_clear_a0`: Clears the bit selected by the low three bits of d0 in the byte at offset 0, sets it in the byte at offset 1f40, clears it in the byte at offset 3e80 and sets it in the byte at offset 5dc0 of the structure in a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_bit_at_three_offsets`: Sets one bit selected by the low three bits of d0 in the bytes at offsets 1f40, 3e80 and 5dc0 of the structure in a0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_set_bit_pair`: Clears one bit selected by the low three bits of d0 in the byte at offset 3e80 of the structure in a0, then sets the same bit in the byte at offset 5dc0 of that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_bit_at_5dc0`: Sets one bit selected by the low three bits of d0 in the byte at offset 5dc0 of the structure in a0, then returns.
- `loop_routines_increment_d4`: Calls the routine at 66348, then loops calling the routine at 663f2 while incrementing d4 until d4 reaches d6, and finally returns a value combining d4 with the last result masked to clear its low nibble.
- `invert_and_clear_word_block`: Inverts d4 into d2, then ANDs that inverted value into the words pointed to by a4 and its peers at offsets 1f3e, 3e7e and 5dbe, advancing a4 by one word.
- `or_and_clear_word_block`: ORs d4 into the word pointed to by a4 and advancing it, then ANDs d2 into the words at offsets 1f3e, 3e7e and 5dbe of that structure.
- `mask_structure_words`: Clears bits from the 16-bit word at the base of the structure in a4 and from the words at offsets 16030 and 24030, then sets bits from d4 in the word at offset 8030, and returns.
- `set_and_clear_words`: Sets bits from d4 in the 16-bit word at the base of the structure in a4 and in the word at offset 8030, then clears bits from d2 in the words at offsets 16030 and 24030, and returns.
- `clear_and_set_words`: Clears bits from d2 in the 16-bit word at the base of the structure in a4 and in the words at offsets 16030 and 24030, then sets bits from d4 in the word at offset 8030, and returns.
- `combine_flags_via_bitwise`: Combines the 16-bit value in d4 with the word at the address in a4, then combines the 16-bit value in d2 with the words at offsets 4000 and 12000 of that same structure, and finally combines d4 with the word at offset 8000 of the same structure before returning.
- `combine_masked_words`: Clears bits from d2 in the 16-bit word at the base of the structure in a4 and in the word at offset 24030, then sets bits from d4 in the words at offsets 16030 and 8030, and returns.
- `apply_set_and_clear`: Sets bits from d4 in the 16-bit word at the base of the structure in a4 and in the words at offsets 16030 and 8030, then clears bits from d2 in the word at offset 24030, and returns.
- `clear_three_set_fourth`: Clears bits from d2 in the 16-bit word at the base of the structure in a4 and in the words at offsets 16030 and 8030, then sets bits from d4 in the word at offset 24030, and returns.
- `set_and_clear_triple`: Sets bits from d4 in the 16-bit word at the base of the structure in a4 and in the word at offset 24030, then clears bits from d2 in the words at offsets 16030 and 8030, and returns.
- `interleave_mask_ops`: Clears bits from d2 in the 16-bit word at the base of the structure in a4 and in the word at offset 8030, then sets bits from d4 in the words at offsets 16030 and 24030, and returns.
- `set_three_clear_fourth`: Sets bits from d4 in the 16-bit word at the base of the structure in a4 and in the words at offsets 16030 and 24030, then clears bits from d2 in the word at offset 8030, and returns.
- `set_flag_bits_via_registers`: Clears bit 15 of the 16-bit word at the address in a4 and of the word at offset 8000, then sets bit 15 of the words at offsets 16000 and 31006, and returns.
- `toggle_flag_bits_via_registers`: Sets bit 15 of the 16-bit word at the address in a4 and of the word at offset 8000, then clears bit 15 of the word at offset 16000 and of the word at offset 31006, and returns.
- `set_flag_bits_single_register`: Sets bit 15 of the 16-bit words at the address in a4 and at offsets 8000, 16000 and 31006, and returns.
- `fill_word_table_blocks`: Compares two words and returns when the second is not greater, then writes four swapped halves of d6 and d7 into a table at offsets 0, 4000, 8000 and 12000, looping while a decremented counter is non-negative, and finally calls a routine when the low nibble of the second word is nonzero.
- `apply_masked_comparison_to_blocks`: Compares two 16-bit values from two structures and, when they differ, looks up a 16-bit mask pair in a table, applies it to both values, and stores the masked results into a four-word block of memory.
- `empty_no_operation`: Contains a single instruction that modifies register d0 with no effect, so the routine does nothing.
- `noop_d0_or_zero`: This routine is a single instruction that ORs the value zero into register d0, which leaves the register unchanged and has no lasting effect on memory.
- `adjust_and_store_track_data`: Reads four 16-bit values from two tables using two indices, swaps the pairs so the first is not greater than the second, and sets bit 6 of the status byte at offset 0 of the structure in a1.
- `initialize_memory_and_globals`: Stores the value an address into seven consecutive 32-bit words starting at offset an address, writes a zero byte at offset 48 of that area, stores the value 15744 in a global variable, stores the value 32 in another global variable, and returns.
- `set_global_byte_and_load_word`: Writes a value of 255 to a global byte at offset 71644, then loads a 16-bit value from a global word at offset an address into register d0, and falls through to the next routine.
- `mask_and_store_index`: Subtracts 32 from the 16-bit value in d0, then reads a global word; if it is greater than -256 and bit 5 of the byte at offset 30 plus the adjusted value in a1 is clear, it subtracts 32 from d0 a second time.
- `process_blitter_callbacks`: Sets a global byte to 255 and, if it matches a stored value, scales it by four and stores it at offset 124 of a table, clearing the byte at offset 116 and writing 48 to the register at offset 12 of the blitter.
- `process_track_slot_flags`: Checks a global counter against 1248 and returns if it is larger, then clears a flag byte and a table entry.
- `store_and_advance_track_indices`: Clears a global byte and, while a 16-bit counter is not greater than 1535, indexes two tables at offsets 7000 and 7010 using two 8-bit indices, and for each of seven index combinations either sets bit 15 of a global long at offset 31500 or calls a routine to store the indices and advances the counter by 4.
- `compute_byte_and_shift_bits`: Computes a byte value from register d0, stores it in a byte at offset 7 of another routine, then clears registers d6 and d7, shifts d0 right by one bit, and conditionally inverts d6 or d7 and swaps them for each subsequent bit before returning the value of d0 shifted right by four bits.
- `clamp_and_store_track_data`: Clamps the 16-bit values in d4 and d6 to the 0-256 range, then sets a flag bit in a byte at offset 0 of the structure in a1.
- `copy_track_data_blocks`: Stores a value and its negation into two global variables, then reads a pointer and copies 16-bit values from a source structure into a destination structure until it encounters a value of 129, after which it returns.
- `swap_word_halves_registers`: Swaps the high and low halves of the 16-bit values in d6 and d7, storing each result back into its own register, then returns.
- `apply_table_mask_to_indexed_values`: Checks a global variable against a range and, when it is in range, sets it to 1504 and calls a routine, then sets it to 128 and 1504.
- `store_sorted_marker_block`: Swaps two 16-bit values so the larger is first, then stores them plus a repeated marker byte into a growing table, terminating the block with the value 32768 and linking the new block into a doubly linked list, returning 0 or 1 depending on the flag at offset 1 of the status byte.
- `return_if_bit_set`: Tests bit 6 of the byte in d0 and returns immediately if that bit is set, otherwise falls through and returns.
- `dispatch_by_counter_byte`: Clears two global bytes, calls another routine, then inspects flag bytes and a signed value to decide whether to swap registers and set a flag before falling through.
- `store_marker_block_when_d6_equals_256`: Compares the value in d6 with 256 and, when it equals that constant, loads 256 into d0, copies d5 into d1, loads 128 into d2, and calls the sorting routine; if that call reports a nonzero result it returns, otherwise it stores 128 into d5 and returns.
- `store_marker_block_when_d6_equals_zero`: Compares the value in d6 with zero and, when it equals that constant, loads zero into d0, copies d5 into d1, loads zero into d2, and calls the sorting routine; if that call reports a nonzero result it returns, otherwise it stores zero into d5 and returns.
- `clear_global_if_threshold`: If the word in d7 equals 128 and the word in d6 is less than or equal to the global variable, it returns, otherwise it stores 0 into the global variable and returns.
- `set_global_if_zero`: If the word in d7 equals 0 and the global variable is less than or equal to the word in d6, it returns, otherwise it stores 256 into the global variable and returns.
- `compare_scaled_value_flags`: Calls the combine routine with the word pairs in registers d0 and d5, then again with d6 and d7, subtracts the first result from the second, takes its absolute value, and sets the flag byte at offset 1 of the status structure to 128 when that value is at least 384, otherwise to 0.
- `combine_values_and_clear`: Computes a value in d3 from the words in d1 and d2 by adding 256, 128, or 200 depending on which of those constants d1 or d2 holds and subtracting the other, then clears d1 and d2 and returns.
- `abs_value_conditional_ccr`: Loads the base address 7368 into a0, reads the 16-bit value at that address plus the offset in d1 into d0, negates it when the sign bit is set, and clears the carry, zero and extend condition codes when the result is below 4800, then returns.
- `store_constant_to_offset`: Stores the 16-bit constant 32768 into the 16-bit word at the address formed by adding the offset in d1 to the base in a6, then returns.
- `noop_or_byte_d0`: Performs a no-op by ORing the byte in d0 with 0, then falls through into the following routine.
- `search_and_update_marker_table`: Reads a signed word from a table indexed by a counter loaded from a global variable and, when that word is negative, decrements the counter by ten repeatedly until it goes below zero and returns negative 32768.
- `find_min_and_update_status`: Scans a linked list of paired values to find the smallest one, toggling a status bit for each matching bit in a counter, then calls a routine with the result and returns.
- `load_and_call_from_index`: Loads a value from a global variable into a4, calls a routine with a fill byte of 15 in d0, then reads a 24-bit value from offset 24 of the structure in a4 indexed by that value.
- `iterate_marker_entries_a`: Loops over a table of 32-byte entries starting from a global index, and for each entry whose byte at offset 29 is positive and not equal to 3, calls the marker routine and then the processing routine when the pointer at offset 16 is nonzero, decrementing the index by 32 each time until it matches a comparison value, then returns.
- `iterate_marker_entries_b`: Loops over a table of 32-byte entries starting from a global index, and for each entry whose byte at offset 29 is positive and not equal to 3, calls the marker routine and then the processing routine when the pointer at offset 20 is nonzero, decrementing the index by 32 each time until it matches a comparison value, then returns.
- `iterate_marker_entries_c`: Loops over a table of 32-byte entries starting from a global index, and for each entry whose byte at offset 29 is positive, calls the marker routine and then the processing routine when the pointer at offset 0 is nonzero, decrementing the index by 32 each time until it matches a comparison value, then returns.
- `iterate_marker_entries_d`: Loops over a table of 32-byte entries starting from a global index, and for each entry whose byte at offset 29 is positive, calls the marker routine and then the processing routine when the pointer at offset 4 is nonzero, decrementing the index by 32 each time until it matches a comparison value, then returns.
- `process_record_table_entries`: Walks a table of 32-byte records from offset an address downward, skipping entries whose low 24 bits are zero, then copies the data of each non-empty record into a destination buffer while clearing bit 6 of the first byte and recording the byte in status fields.
- `process_records_backward`: Walks a table of 32-byte records backwards, copying each record's byte fields into a destination buffer and calling a handler for any record whose low 24 bits are non-zero, then calls two more handlers and returns.
- `process_linked_value_list`: Clears a global flag and a status byte, then loads a counter and scans a linked list of paired 24-bit values, storing the non-zero ones into a table while toggling status bits.
- `iterate_marker_entries_b_2`: Decrements a global 16-bit counter by 32 and, while it is not zero, walks a table of 32-byte entries using that counter as an index, calling a sequence of marker routines for each entry whose byte at offset 29 is positive.
- `test_global_byte_flags`: Tests a byte in a global variable and branches away when it is zero or negative, otherwise it returns.
- `test_global_byte_negative`: Tests a byte in a global variable and branches away when it is negative, otherwise it returns.
- `check_and_set_flag_value`: Checks whether a byte at offset 19001 of a global structure is zero and whether a 16-bit value at offset 92850 is greater than the 16-bit value stored at offset 73554; if so, it calls a routine, sets the value at offset 73554 to 65535, and returns.
- `apply_masked_comparison_blocks_a`: Reads a byte from a structure and, when a flag bit is set, replaces it with another stored byte, then reads four 32-bit fields from that structure and, when all their low 24 bits are nonzero and the high nibble is zero, builds a byte value, stores it into a global variable, and calls the masking routine.
- `apply_masked_block_comparison`: Reads a byte from a global variable and, when a flag at offset 34 of the structure in a4 is set, loads four 24-bit values from offsets 128, 136, 144 and 152 of that structure.
- `process_marker_table`: Walks through a table of 16-bit values in a3, skipping entries that are zero or the 256 marker, then repeatedly calls the routine at 663F2 and combines its result with a running counter in d5, decrementing the count until it is exhausted or a negative marker is reached.
- `sort_three_values`: Reorders three 16-bit values pointed to by a0, a1 and a2 so they are sorted in ascending order, then sets a global variable to 1 and returns.
- `process_and_store_track_data`: Negates and stores a scaled value, then builds two arrays by combining pairs of 16-bit values from indexed tables, and finally walks a packed data buffer calling lookup-table handlers for each non-zero entry.
- `copy_table_entry_bytes`: Reads a byte from a global variable at offset a fixed address, scales it by four, indexes a table at offset a fixed address, saves the resulting byte at offset 13177 of a structure, then copies two bytes at a time from that table entry into two other structures until the scaled value reaches zero.
- `empty_data_area`: Contains a single zero word that decodes as ori.b #0, d0 and serves only as a data area referenced by address; it does nothing.
- `store_byte_d0_to_a0`: It writes the low byte of data register d0 to the byte at the address in address register a0, then returns.
- `store_zero_byte`: The routine contains a single instruction that writes the value zero to the byte at the address in a0, then returns.
- `zero_out_memory_block`: Fills a contiguous area of memory with zero bytes, writing a byte value of zero at each address in the range and advancing through the region.
- `triple_noop_byte_or`: Consists of three consecutive ori.b #0, d0 instructions, which are zero words that perform no operation and leave the register unchanged, then the routine ends without returning.
- `zero_memory_range`: Fills a run of memory with zeros, writing a zero byte at each successive address.
- `store_byte_via_pointer`: Writes the low byte of the 16-bit value in register d0 to the memory location pointed to by register a0.
- `zero_memory_block`: Fills a run of memory with zero bytes, starting at the address in a0 and continuing until the counter in d0 reaches zero.
- `noop_d0_register`: It is a single instruction that performs a bitwise OR of the value zero onto register d0, which leaves the register unchanged, and it does nothing else.
- `repeat_routine_calls`: Loops a routine thirty-two times, resetting the counter to fifteen after it reaches ten, then runs the routine ten more times before returning.
- `loops_routine_calls`: It repeatedly calls a routine while incrementing a counter that resets to 15 when it reaches 10, then calls the same routine again while incrementing a counter that starts at 10 and stops when it reaches 15.
- `run_byte_table_loops`: Executes a loop that repeatedly calls the byte-table routine while incrementing a counter, resetting it to 15 when it reaches 10 and exiting when it reaches 52, then calls the same routine with a counter preset to 10 and a loop that runs until the counter reaches 15.
- `process_byte_table_loop`: Checks a value in a register against the bounds 37 and 48, sets a flag byte in a global variable, then indexes a table of pointers and a second table of words to compute offsets into a third table.
- `copy_words_seven`: Initializes two pointers to a global variable and another address, then copies 16-bit words from the source to the destination seven times using a decrementing counter, before returning.
- `repeat_word_pair_store`: Initializes a counter to 7, then repeatedly calls the word-pair routine while decrementing the counter until it reaches -1, and finally returns.
- `index_pointer_table`: Masks a value in a register to 8 bits, scales it by four, indexes a table of pointers using the result, and loads the pointed-to value back into the register, then returns.
- `store_word_pair_offset`: Scales a value in a register by eight, stores the low word of another value at an offset from a base address, then stores the high word at the preceding offset, before returning.
- `update_byte_block_loop`: Loops over a block of data advancing by four each time, reading a pointer from a table, comparing stored and current byte values, and writing the updated value and a computed difference back through the pointer until the counter reaches 196.
- `combine_four_areas_loop`: Indexes a table of pointers and a table of words using a masked register value, then combines masked values from four source areas into four destination areas using bitwise operations, looping over rows and columns before restoring registers and returning.
- `rotate_and_combine_areas`: Calls a library routine, rotates and splits a returned value into words, then shifts and combines them into four destination areas depending on a counter in a global variable, incrementing or decrementing that counter and calling another library routine on the alternate path before returning.
- `compute_pointer_offsets`: Builds a byte-indexed offset from the low byte of a register, multiplies it by four, and uses it to fetch a pointer from a table, then computes a second pointer from another register and a third from a global pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `return_then_branch`: Returns immediately, then falls through to a branch instruction that jumps to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `transform_and_copy_words`: Reads a word from the source pointer in a1, shifts it left by one, and sets bits based on which nibble groups are nonzero before storing the modified word to the destination pointer in a0 and advancing both pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_copy_screen`: Clears a block of words in a global structure via a loop, then calls two other routines and copies a table of words into fixed offsets of a screen buffer at a high memory address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_blitter_registers`: Fills a Blitter FIFO structure with a pair of source addresses in a loop, then writes a series of values to the Blitter, copper, and display control registers and stores a pointer into a hardware register.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `loop_and_return`: Loops on the counter in d4 using the branch, then returns; the branch target lies outside this pair of instructions so no loop body is contained here.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_stack_and_trap`: Sets the stack pointer to a fixed address, loads a data register with another address, stores that register into a global location, and then triggers a software interrupt.
- `wait_and_setup_interrupts`: Waits in a loop until a bit in a global variable is clear, then writes a sequence of values to interrupt enable and request registers to configure interrupt handling.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
