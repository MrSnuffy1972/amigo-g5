# Game 1255 (power drift (1989)(activision)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01255). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802240 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 40 of 40 named (0 author symbols), 40 of 40 described (2 by rule, 38 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 291 of 291 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 790 of 790 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1147 |
| code | 3126 |
| data | 1278799 |
| unused | 519168 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Power Drift (1989)(Activision)(Disk 1 of 2).adf:bootblock | 1 | 44 | 2 |
| logged disk load from Power Drift (1989)(Activision)(Disk 1 of 2).adf boot-block load | 1 | 746 | 38 |

## Cases handled

- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: custom-format (non-DOS) disk, 934 non-empty blocks: matched against track-loaded segments
- power drift (1989)(activision)(disk 2 of 2)/Power Drift (1989)(Activision)(Disk 2 of 2).adf: custom-format (non-DOS) disk, 1566 non-empty blocks: matched against track-loaded segments
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: the boot block reads 4608 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: 1 logged loads cover 4608 bytes of the image
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 33-524 (bytes 16896-268800) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.23)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 528-538 (bytes 270336-275968) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.67)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 550-560 (bytes 281600-287232) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.65)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 572-582 (bytes 292864-298496) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.08)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 594-604 (bytes 304128-309760) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.15)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 616-626 (bytes 315392-321024) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.50)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 638-648 (bytes 326656-332288) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.67)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 660-670 (bytes 337920-343552) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.91)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 682-692 (bytes 349184-354816) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.77)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 704-714 (bytes 360448-366080) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.88)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 726-736 (bytes 371712-377344) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.11)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 748-758 (bytes 382976-388608) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.93)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 770-780 (bytes 394240-399872) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.68)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 792-802 (bytes 405504-411136) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.99)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 814-824 (bytes 416768-422400) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.54)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 836-846 (bytes 428032-433664) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.61)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 858-868 (bytes 439296-444928) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.66)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 880-890 (bytes 450560-456192) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.79)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 902-912 (bytes 461824-467456) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.81)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 924-934 (bytes 473088-478720) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.30)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 946-956 (bytes 484352-489984) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.55)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 968-978 (bytes 495616-501248) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.66)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 990-1000 (bytes 506880-512512) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.08)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 1012-1022 (bytes 518144-523776) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.70)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 1034-1044 (bytes 529408-535040) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.70)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 1056-1066 (bytes 540672-546304) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.21)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 1078-1088 (bytes 551936-557568) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.29)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 1122-1132 (bytes 574464-580096) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.99)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 1144-1154 (bytes 585728-591360) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.83)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 1210-1220 (bytes 619520-625152) outside the filesystem hold data, not code (opcode density 0.003, entropy 7.08)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 1232-1242 (bytes 630784-636416) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.73)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 1254-1264 (bytes 642048-647680) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.07)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 1298-1308 (bytes 664576-670208) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.00)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 1320-1330 (bytes 675840-681472) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.30)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 1342-1352 (bytes 687104-692736) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.10)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 1364-1374 (bytes 698368-704000) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.80)
- power drift (1989)(activision)(disk 1 of 2)/Power Drift (1989)(Activision)(Disk 1 of 2).adf: blocks 1386-1391 (bytes 709632-712704) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.22)
- power drift (1989)(activision)(disk 2 of 2)/Power Drift (1989)(Activision)(Disk 2 of 2).adf: blocks 11-328 (bytes 5632-168448) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.78)
- power drift (1989)(activision)(disk 2 of 2)/Power Drift (1989)(Activision)(Disk 2 of 2).adf: blocks 333-531 (bytes 170496-272384) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.30)
- power drift (1989)(activision)(disk 2 of 2)/Power Drift (1989)(Activision)(Disk 2 of 2).adf: blocks 533-561 (bytes 272896-287744) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.70)
- power drift (1989)(activision)(disk 2 of 2)/Power Drift (1989)(Activision)(Disk 2 of 2).adf: blocks 715-865 (bytes 366080-443392) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.14)
- power drift (1989)(activision)(disk 2 of 2)/Power Drift (1989)(Activision)(Disk 2 of 2).adf: blocks 880-1748 (bytes 450560-895488) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.81)
- 4 packed track region(s), 22528 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_dma_channels` (Power Drift (1989)(Activision)(Disk 1 of 2).adf boot-block load +an address to an address (4608 bytes)): Checks a global variable and, if negative, writes specific values to the disk length register, CIAB port B register, and multiple hardware registers including ADKCON, DMACON, and INTREQ. Kind: disk track sync check. Pass path: The routine sets up DMA for disk reading with specific ADKCON and DSKSYNC values, likely checking for a valid track sync pattern. If the check passes (i.e., the expected sync pattern is detected), the code continues execution normally; if n.
- `setup_disk_dma` (Power Drift (1989)(Activision)(Disk 1 of 2).adf boot-block load +an address to an address (4608 bytes)): Checks a global variable and, if negative, writes specific values to the disk length register, CIAB port B register, and DMA control register to enable channels. Kind: disk track check. Pass path: The routine sets up DMA for disk loading and checks if d0_0D68 >= 0. If so, it configures CIAB_PRB and DSKLEN registers to initiate a disk read operation, likely checking for a specific track or sector pattern on the disk to verify authenti.
- `setup_disk_transfer` (Power Drift (1989)(Activision)(Disk 1 of 2).adf boot-block load +an address to an address (4608 bytes)): Sets up disk transfer parameters by writing specific values to hardware registers, initializes a global variable, performs conditional operations based on interrupt flags, and enters an infinite loop. Kind: disk track sync check. Pass path: The routine sets up disk transfer parameters (ADKCON, DSKPTH, DSKSYNC) and waits for a specific condition on CIAA_PRA (bit 5, indicating drive status), then enters an infinite loop. A successful check would proceed past the initial setup an.
- `initialize_dma_transfer` (Power Drift (1989)(Activision)(Disk 1 of 2).adf boot-block load +an address to an address (4608 bytes)): Sets up DMA transfer parameters by writing specific values to hardware registers, initializes a global variable, and performs a loop of data processing operations involving bit manipulation and memory writes. Kind: disk track check. Pass path: the routine completes without triggering an error condition, and the data is loaded successfully into memory at an address.
- `toggle_ciab_bit` (Power Drift (1989)(Activision)(Disk 1 of 2).adf boot-block load +an address to an address (4608 bytes)): Calls a wait routine, toggles bit 7 of a global variable related to CIAB port B, and calls another routine. Kind: disk track check. Pass path: bra.b an address.
- `toggle_ciab_bits_1` (Power Drift (1989)(Activision)(Disk 1 of 2).adf boot-block load +an address to an address (4608 bytes)): Calls a wait routine, toggles bits 1 and 7 of a global variable related to CIAB port B, and calls another routine. Kind: disk track check. Pass path: bra.b an address.

## Functions


### Power Drift (1989)(Activision)(Disk 1 of 2).adf:bootblock

- `initialize_system`: Sets up the stack pointer, initializes memory areas with control words, copies data from one location to another, and configures various hardware registers including interrupt enable, display, and DMA settings.
- `initialize_stack_and_interrupts`: Sets up the stack pointer to a specific location, initializes a memory area with a control word, and then performs a loop that copies data from one location to another while decrementing a counter.

### logged disk load from Power Drift (1989)(Activision)(Disk 1 of 2).adf boot-block load

- `initialize_game_data`: Initializes global variables and DMA parameters, sets up hardware registers including interrupt and DMA control, and performs a series of memory operations to prepare game data structures before execution begins.
- `check_multiple_conditions`: Tests multiple global variables for non-zero values and branches if any are true, otherwise it loads zero into a register and returns, or loads -1 into the register and returns.
- `update_global_variable`: Stores a byte value from a register into a global variable, then calls another routine and branches based on the result of that call before returning.
- `update_dma_parameters`: Updates a table of values by adding each value to itself, copies data from one memory location to another using byte count, and returns after processing all bytes.
- `tiny_move_rte`: Small routine of 2 instructions that copies d0 to d0, then returns from the exception.
- `clear_interrupt_flags_2`: Clears bit 4 of a custom hardware register, tests bit 5 of the same register and branches if set, then tests bit 6 and branches if set, finally returns from the routine.
- `wait_for_interrupt_flag`: Writes a specific value to an interrupt request register, then polls an interrupt request register until a particular bit is set, clears the same interrupt request register, and returns.
- `clear_and_set_potgo`: Clears a global variable and sets a hardware register to a specific value.
- `read_keyboard_and_modify`: Reads from a hardware register, clears that register, writes a specific value to another hardware register, inverts the read value, stores it in a global variable, and modifies a memory location based on bits of the inverted value.
- `process_joystick_and_buttons`: Processes joystick data from multiple sources, combines various bit states to form a result, stores that result in a global variable, and sets or clears specific bits in other global variables based on input conditions.
- `initialize_graphics_registers`: Initializes several graphics hardware registers with specific values, clears memory locations, sets up color registers, configures display parameters, and enables DMA and interrupt channels.
- `toggle_copper_list_and_trap`: Triggers a trap instruction, modifies a global variable by toggling a specific bit, updates a hardware register with different values based on the bit state, and sets another global variable to a specific value.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (st.b) and then falls through into the code that follows, with no return of its own.
- `copy_memory_and_check_dma`: Checks a global variable for zero and branches if not, otherwise copies data from one memory area to another, saves a register value on the stack, reads a hardware register, checks a bit in that register, and sets up DMA control registers.
- `halt_cpu_execution`: Sets a global variable to one, tests that same variable for non-zero value and branches if true, clears a memory location, and sets a hardware register to a specific value before entering an infinite loop.
- `process_color_data`: Processes 16 pairs of bytes from a source pointer through multiplication, bit shifting, and masking operations, storing results in a local array before calling another routine.
- `halt_with_interrupts`: Traps to the operating system, sets the processor status register to enable interrupts, tests a global variable and branches if zero, then stops execution with a specific value and branches back to an earlier address.
- `handle_joystick_input`: Sets the processor status register to enable interrupts, clears a global variable, writes specific values to the interrupt request register, and returns with a reset vector.
- `initialize_hardware_timers`: Clears two global variables, sets multiple hardware registers to specific values, writes function addresses to interrupt vector table entries, configures custom interrupt enable register with a specific value, and returns.
- `check_global_flag`: Reads a byte from a memory location and branches if the value is non-zero, otherwise it returns immediately.
- `clear_interrupt_flags`: Checks specific bits in the CIAB interrupt control register and clears global variables when those bits are set, then sets a memory flag in the interrupt request register.
- `read_keyboard_and_modify_2`: Clears a hardware register, reads from another register, checks specific bits and branches if they're clear, calls another routine, then modifies global variables based on bit states before returning with an interrupt.
- `retry_on_error`: Repeatedly calls another routine until it returns a non-negative value, then jumps to a function pointer stored in a register.
- `copy_block_data`: Copies several longwords and bytes from a source pointer to global variables, including a calculated value derived from a byte in the source block.
- `verify_data_block_integrity`: Checks if a data pointer is null; if so, it initializes a global variable and calls helper routines.
- `process_dma_data`: Processes DMA data by copying structured information from a source table to a destination buffer, updating pointers and performing calculations based on table entries.
- `configure_dma_channels`: Checks a global variable and, if negative, writes specific values to the disk length register, CIAB port B register, and multiple hardware registers including ADKCON, DMACON, and INTREQ.
- `setup_disk_dma`: Checks a global variable and, if negative, writes specific values to the disk length register, CIAB port B register, and DMA control register to enable channels.
- `copy_dma_data`: Compares a word value with memory location 3420, branches if equal, saves registers to stack, calls another routine, restores registers from stack, and performs data copying operations in a loop.
- `setup_disk_transfer`: Sets up disk transfer parameters by writing specific values to hardware registers, initializes a global variable, performs conditional operations based on interrupt flags, and enters an infinite loop.
- `initialize_dma_transfer`: Sets up DMA transfer parameters by writing specific values to hardware registers, initializes a global variable, and performs a loop of data processing operations involving bit manipulation and memory writes.
- `process_cia_port_data`: Reads a word from memory location pointed by a1, checks if it's positive and performs bitwise operations on the CIAA_PRA register, then calls another routine based on conditions, and returns with different values depending on execution path.
- `check_joystick_fire_2`: Tests bit 4 of CIAA_PRA to determine if a joystick fire button is pressed, then calls two routines with different parameter sequences based on that test result.
- `check_joystick_fire`: Tests bit 2 of the CIAA_PRA register and branches to a specific address if the bit is clear, otherwise it returns.
- `toggle_ciab_bit`: Calls a wait routine, toggles bit 7 of a global variable related to CIAB port B, and calls another routine.
- `check_mouse_button`: Tests bit 2 of the CIAA_PRA register and branches to a specific address if the bit is clear, otherwise it returns.
- `toggle_ciab_bits_1`: Calls a wait routine, toggles bits 1 and 7 of a global variable related to CIAB port B, and calls another routine.
- `compare_and_store_byte`: Compares a byte value with memory location a fixed address, branches if equal, stores the byte to that memory location, then performs unconditional branches to specific addresses.
