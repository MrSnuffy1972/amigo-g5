# Game 2656 (alien strike (1987)(diamond games)): code recovery summary

## Functions (part 2 of 2)

### Alien Strike (1987)(Diamond Games).adf:puppychow
(continued)

- `frag_jmp_258`: Short code fragment that jumps to another point in the code.
- `frag_clr_tst_bne_move_jmp`: Short code fragment that clears a fixed memory address (2 times in a row), then tests a fixed memory address, then branches if the last result was not zero, then copies 1 to a fixed memory address (2 times in a row), then jumps to another point in the code.
- `frag_move_cmpi_bne_clr_jmp_3`: Short code fragment that copies a fixed memory address to d5, then compares d5 with 2, then branches if the last result was not zero, then clears a fixed memory address (2 times in a row), then jumps to another point in the code.
- `initialize_object_2_2`: Initializes object properties by setting specific bytes in a structure, performing calculations involving offsets and registers, and calling.
- `frag_clr_move_cmpi_bne_bra`: Short code fragment that clears a fixed memory address, then copies a fixed memory address to d5, then compares d5 with 1, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_jmp_259`: Short code fragment that jumps to another point in the code.
- `set_jet_parameters`: Initializes jet properties by calculating and storing velocity, missile capacity, and hit points based on skill level and configuration values.
- `frag_clr_cmpi_bls_jmp_13`: Short code fragment that clears a fixed memory address, then compares a fixed memory address with 3500, then branches if the comparison was lower or same (unsigned), then jumps to another point in the code.
- `frag_cmpi_bcc_addq_bra_2`: Short code fragment that compares a fixed memory address with 3500, then branches if the carry flag is clear, then adds 1 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bsr_addq_cmp_bcc_bra`: Short code fragment that copies 4 to the stack, then copies 225 to the stack, then copies 1 to the stack, then calls another routine, then adds 6 to a7, then copies a fixed memory address to d0, then compares d0 with the top of the stack, then branches if the carry flag is clear, then adds 1 to a fixed memory address, then jumps to another point in the code.
- `frag_addq_clr_cmpi_bls_jmp`: Short code fragment that adds 2 to a7, then clears a fixed memory address, then compares a fixed memory address with 4, then branches if the comparison was lower or same (unsigned), then jumps to another point in the code.
- `frag_clr_move_mulu_cmp_bls_jmp`: Short code fragment that clears a fixed memory address, then copies a fixed memory address to d5, then multiplies d5 by 4000 (unsigned), then copies d5 to the stack, then copies a fixed memory address to d0, then compares d0 with the top of the stack, then branches if the comparison was lower or same (unsigned), then jumps to another point in the code.
- `frag_move_cmp_bcc_addq_bra_3`: Short code fragment that copies a fixed memory address to d0, then compares d0 with the top of the stack, then branches if the carry flag is clear, then adds 1 to a fixed memory address, then jumps to another point in the code.
- `frag_addq_cmpi_bcc_bra`: Short code fragment that adds 2 to a7, then compares a fixed memory address with 4, then branches if the carry flag is clear, then adds 1 to a fixed memory address, then jumps to another point in the code.
- `frag_clr_cmpi_bls_jmp_14`: Short code fragment that clears a fixed memory address, then compares a fixed memory address with 1, then branches if the comparison was lower or same (unsigned), then jumps to another point in the code.
- `update_sequence_counters`: Updates sequence-related counters and flags based on table lookups and conditional operations, then increments a global counter and loops back to process the next iteration.
- `initialize_sequence`: Sets up sequence-related variables including counters and flags, then performs a series of operations involving table lookups and conditional jumps based on counter values.
- `frag_jmp_260`: Short code fragment that jumps to another point in the code.
- `update_sequence_counter`: Reads a word value from a global variable, multiplies it by 100, uses the result as an index to access a table, passes the table entry address and a zero word to another routine, then clears the global variable and compares its value to 79.
- `update_battle_sequence`: Updates battle sequence counters and performs numeric calculations based on values from global variables, then checks if a counter exceeds 500 and branches accordingly.
- `frag_cmpi_bcc_addq_bra_3`: Short code fragment that compares a fixed memory address with 500, then branches if the carry flag is clear, then adds 1 to a fixed memory address, then jumps to another point in the code.
- `frag_cmpi_bcc_addq_jmp_7`: Short code fragment that compares a fixed memory address with 79, then branches if the carry flag is clear, then adds 1 to a fixed memory address, then jumps to another point in the code.
- `initialize_object_properties`: Initializes object properties by setting specific word values in a data structure, clears a counter field, and performs conditional updates based on comparison results before jumping to another routine.
- `frag_move_mulu_lea_clr_cmpi_bcc_addq_br_2`: Short code fragment that copies a fixed memory address to d5, then multiplies d5 by 108 (unsigned), then loads the address of a fixed memory address into a4, then clears an indexed entry from a4, then compares a fixed memory address with 2, then branches if the carry flag is clear, then adds 1 to a fixed memory address, then jumps to another point in the code.
- `frag_bsr_move_cmp_ble_jmp`: Short code fragment that calls another routine, then copies a fixed memory address to d5, then compares d5 with a fixed memory address, then branches if the comparison was less or equal, then copies 125 to a fixed memory address, then jumps to another point in the code.
- `frag_move_cmp_ble_jmp`: Short code fragment that copies a fixed memory address to d5, then compares d5 with a fixed memory address, then branches if the comparison was less or equal, then copies 25 to a fixed memory address, then jumps to another point in the code.
- `frag_move_clr_cmpi_bls_jmp_2`: Short code fragment that copies 100 to a fixed memory address, then clears a fixed memory address, then compares a fixed memory address with 29, then branches if the comparison was lower or same (unsigned), then jumps to another point in the code.
- `update_counter_and_execute`: Updates a counter in a global variable by incrementing it twice, then executes a function with specific parameters including a word value and a pointer to a table.
- `frag_cmpi_bcc_addq_bra_4`: Short code fragment that compares a fixed memory address with 1000, then branches if the carry flag is clear, then adds 1 to a fixed memory address, then jumps to another point in the code.
- `frag_cmpi_bcc_addq_jmp_8`: Short code fragment that compares a fixed memory address with 29, then branches if the carry flag is clear, then adds 1 to a fixed memory address, then jumps to another point in the code.
- `initialize_sequence_and_compare`: Initializes sequence-related variables and compares values from memory locations, setting specific fields in a structure based on conditions.
- `update_game_state_6`: Updates a global simulation counter and compares it with a threshold value, branching based on the comparison result.
- `frag_jmp_261`: Short code fragment that jumps to another point in the code.
- `update_simulation_state`: Updates a global simulation counter and checks if it exceeds 32000, branching to a different routine if so.
- `frag_cmpi_bcc_addq_bra_5`: Short code fragment that compares a fixed memory address with 32000, then branches if the carry flag is clear, then adds 1 to a fixed memory address, then jumps to another point in the code.
- `frag_pea_jsr_addq_jmp`: Short code fragment that pushes the address of a fixed memory address onto the stack, then calls another routine, then adds 4 to a7, then jumps to another point in the code.
- `frag_move_cmp_ble_bra`: Short code fragment that copies a fixed memory address to d5, then compares d5 with a fixed memory address, then branches if the comparison was less or equal, then jumps to another point in the code.
- `frag_jmp_262`: Short code fragment that jumps to another point in the code.
- `update_game_state_5`: Updates a global counter and checks if it exceeds 32000, branching to another routine if so.
- `frag_cmpi_bcc_addq_bra_6`: Short code fragment that compares a fixed memory address with 32000, then branches if the carry flag is clear, then adds 1 to a fixed memory address, then jumps to another point in the code.
- `frag_pea_jsr_addq_jmp_2`: Short code fragment that pushes the address of a fixed memory address onto the stack, then calls another routine, then adds 4 to a7, then jumps to another point in the code.
- `calculate_score`: Calculates a score value by multiplying a word from a structure offset by 100, then loads the address of another structure into a4 and pushes parameters onto the stack before calling a library function.
- `frag_clr_cmpi_bls_jmp_15`: Short code fragment that clears a fixed memory address, then compares a fixed memory address with 30, then branches if the comparison was lower or same (unsigned), then jumps to another point in the code.
- `frag_cmpi_bcc_addq_bra_7`: Short code fragment that compares a fixed memory address with 30, then branches if the carry flag is clear, then adds 1 to a fixed memory address, then jumps to another point in the code.
- `frag_cmpi_bcc_addq_bra_8`: Short code fragment that compares a fixed memory address with 10000, then branches if the carry flag is clear, then adds 1 to a fixed memory address, then jumps to another point in the code.
- `frag_move_mulu_lea_pea_clr_jsr_addq_jmp`: Short code fragment that copies a fixed memory address to d5, then multiplies d5 by 100 (unsigned), then loads the address of a fixed memory address into a4, then pushes the address of an indexed entry from a4 onto the stack, then clears the stack, then calls another routine, then adds 6 to a7, then jumps to another point in the code.
- `frag_jsr_bsr_unlk_rts`: Short code fragment that calls another routine (5 times in a row), then removes the stack frame held in a6, then returns.

### Alien Strike (1987)(Diamond Games).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
