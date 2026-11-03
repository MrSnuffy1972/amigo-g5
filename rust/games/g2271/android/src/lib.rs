//! libmain.so of game 2271's APK: SDL's Java activity loads libSDL2.so and this library, then calls `SDL_main` on its
//! own thread. The asset pack is the APK asset `gamepack.bin`, read through SDL by the shared start.

mod ffi {
    use std::os::raw::{c_char, c_int};

    /// The entry SDL's activity calls. Its arguments are empty for normal play; started with the `amigo_verify`
    /// intent extra they are the headless verification arguments (see `platform::sdl::c_args`). A panic is caught
    /// here, because unwinding into SDL's C would abort the process without the message.
    #[no_mangle]
    pub extern "C" fn SDL_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
        // SAFETY: SDL passes argc NUL-terminated strings ("app_process", then the activity's arguments).
        let args = unsafe {
            g2271::native_engine::platform::sdl::c_args(argc, argv as *const *const c_char)
        };
        std::panic::catch_unwind(|| {
            let mut v = vec!["g2271".to_string()];
            v.extend(args);
            g2271::native_engine::platform::sdl::app_main(&v, None, &g2271::spec())
        })
        .unwrap_or(1)
    }
}
