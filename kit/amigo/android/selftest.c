/* selftest.c - smallest SDL2 program, used to prove the Android build chain end to end (Ami-Go, 13 Oct 2026). */
#include "SDL.h"
int main(int argc, char *argv[]) {
    (void)argc; (void)argv;
    if (SDL_Init(SDL_INIT_VIDEO | SDL_INIT_GAMECONTROLLER) != 0) return 1;
    SDL_Window *w = SDL_CreateWindow("Ami-Go self-test", 0, 0, 640, 512, SDL_WINDOW_FULLSCREEN_DESKTOP);
    SDL_Renderer *r = SDL_CreateRenderer(w, -1, 0);
    for (int running = 1; running;) {
        SDL_Event e;
        while (SDL_PollEvent(&e)) if (e.type == SDL_QUIT || (e.type == SDL_KEYDOWN && e.key.keysym.sym == SDLK_AC_BACK)) running = 0;
        SDL_SetRenderDrawColor(r, 0x00, 0x55, 0xaa, 255); SDL_RenderClear(r);
        SDL_Rect bar = {0, 0, 640, 40}; SDL_SetRenderDrawColor(r, 0xff, 0x88, 0x00, 255); SDL_RenderFillRect(r, &bar);
        SDL_RenderPresent(r); SDL_Delay(16);
    }
    SDL_Quit(); return 0;
}
