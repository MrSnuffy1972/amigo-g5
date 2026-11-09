# Game 259 (DGeneration v1.3 0677): code recovery summary

## Functions (part 2 of 2)

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)
(continued)

- `undecodable_bytes_29`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_90`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_30`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_91`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_92`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_93`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_94`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_95`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_96`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_97`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_98`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_99`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_31`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_32`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_100`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_101`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_102`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_33`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_103`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_104`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_105`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_106`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_107`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_108`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_109`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_routine_then_jump`: Calls another routine four times in succession, then jumps to a different routine and returns.
- `jump_vector_entry_110`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `single_call_then_jump`: Calls another routine once, then jumps to a different routine and returns.
- `jump_vector_entry_111`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_112`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_113`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_114`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_115`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_116`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `return_then_jump`: Returns immediately to the caller, then jumps to a different routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_d0_byte_to_global`: Stores the byte in d0 into a global variable at offset 0 of the structure pointed to by the table at offset 1724, then restores a0 and returns.
- `clear_audio_and_disable_dma`: Clears a byte at offset 1 of the structure and zeroes four audio volume registers, then sets bit 15 of the DMA control register to disable DMA channels before returning.
- `load_byte_from_offset_one`: Loads a byte from offset 1 of the structure into d0, clears the upper bits of d0 with a moveq, then restores a0 and returns the value.
- `init_audio_from_table`: Copies four bytes from a table of decoded values into fields of a structure in a1, advancing both pointers by four until the index reaches 16.
- `process_audio_stream`: Clears the audio volume and channel registers in the custom chip, decrements a timer in a global structure, and if it reaches zero resets the audio state and returns.
- `decode_table_stream`: Reads a table index from a structure, then walks a table of bytes that contains a 253 delimiter, copying decoded values into several fields of the structure and branching on bit 6 of each byte, looping until the structure field is exhausted.

### amiga.eff (raw code file, load address unknown)

- `initialize_and_clear_registers`: Initializes a structure by writing all-ones to the words at offsets 0 and 26 and clearing the words at offsets 16 and 12.

### amiga.mus (raw code file, load address unknown)

- `init_globals_and_clear_audio`: Clears four global variables to zero, sets a byte at offset 26 of the structure in a0 to 255, sets bit 1 of a status byte, and returns.
- `decode_table_stream_2`: Reads a 16-bit table index from offset 124 of the structure in a1, then walks a byte table starting at offset 253, copying decoded values into fields at offsets 30, 18, 0, 26, 39 and 43 of the structure in a0 and branching on bit 6 of each byte, looping until the structure field is exhausted.
