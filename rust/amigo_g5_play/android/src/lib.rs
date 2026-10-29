//! libmain.so for the Android APK (M9, 29 Oct 2026, docs/G5-PLAYABLE.md): SDL's Java activity loads libSDL2.so and
//! this library, then calls `SDL_main` on its own thread. The game pack is the APK asset `gamepack.bin`, read through
//! SDL_RWops by `amigo_g5_play::app::main`, the same start as the desktop binary's.

mod ffi {
    use std::os::raw::{c_char, c_int};

    /// The entry SDL's activity calls (SDL_android.c, `nativeRunMain`). Its arguments are only the library's name,
    /// so they are not read: no pointer is touched and nothing here is unsafe. A panic is caught here, because
    /// unwinding into SDL's C would abort the process without the message.
    #[no_mangle]
    pub extern "C" fn SDL_main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
        std::panic::catch_unwind(|| {
            let v = vec!["amigo-g5-play".to_string()];
            amigo_g5_play::app::main(&v, None, &mut amigo_g5_play::app::game_step)
        })
        .unwrap_or(1)
    }
}
