//! libmain.so of game 296's APK: SDL's Java activity loads libSDL2.so and this library, then calls `SDL_main` on its
//! own thread. The asset pack is the APK asset `gamepack.bin`, read through SDL by the shared start.

mod ffi {
    use std::os::raw::{c_char, c_int};

    /// The entry SDL's activity calls. Its arguments are only the library's name, so they are not read: no pointer is
    /// touched and nothing here is unsafe. A panic is caught here, because unwinding into SDL's C would abort the
    /// process without the message.
    #[no_mangle]
    pub extern "C" fn SDL_main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
        std::panic::catch_unwind(|| {
            let v = vec!["g296".to_string()];
            g296::native_engine::platform::sdl::app_main(&v, None, &g296::spec())
        })
        .unwrap_or(1)
    }
}
