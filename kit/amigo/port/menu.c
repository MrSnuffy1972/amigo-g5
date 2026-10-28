/* menu.c: the launcher (pre-start menu), the in-game options menu, the on-screen Amiga keyboard and controller mapping for
 * every Ami-Go port (SDL2; Windows and Android). Included by main.c in SDL builds only.
 *
 * Karl, 14 Oct 2026: "Each game should have a Pre-Start menu where you can set options such as screen resolution
 * and the CRT look option ... setting the controller, with a default option ... a pop up screen keyboard 'hot key'
 * on the controller as well, set to a default that is not used by anything that uses an Xbox controller."
 * Karl, 14 Oct: the pre-start menu shows at every start for 30 s (any input stops the countdown), with a picture of
 * the controller and what each button does.
 * Karl, 17 Oct (Cowork rewrite): a launcher screen that looks like a current console or PC game's - clean type, smooth animated
 * transitions, a subtle animated background, focus highlights, controller hints with button glyphs, the game title prominent -
 * drawn at the display's native resolution (never below 1080 lines: smaller displays get a 1080-line picture scaled down) and
 * usable with a controller, keyboard, mouse and touch. A pop-up Amiga keyboard for every game (all Amiga keys, drawn by us after
 * scaling, never into the game's picture); keys on it go into the same kbd_push() queue the real keyboard uses. Screenshot and
 * video: see reccap.c.
 * Karl, 17 Oct evening: the controller scheme from the Copilot design he approved (see "controllers -> Amiga" below: Back opens
 * the keyboard, Start is the menu, LT the F-key layer, the right stick and LB/RB the mouse, Start + LB/RB capture), a keyboard
 * with GAME / ABC / NUMPAD pages, and scaling modes and filters for the picture (presentation only).
 *
 * Settings are kept per game in the user's profile (SDL_GetPrefPath: %APPDATA%\AmiGo\<game>\settings.ini on Windows, the
 * app's private storage on Android). Drawing: amigo_ui.h (SDL_RenderGeometry plus an embedded Inter font via stb_truetype).
 */
#include <math.h>
#include <time.h>
#include "amigo_ui.h"

/* ------------------------------------------------------------------ Amiga keys and actions */
typedef struct { const char *name; int code; } akey_t;
enum { ACT_NONE = 0, ACT_FIRE, ACT_UP, ACT_DOWN, ACT_LEFT, ACT_RIGHT, ACT_FIRE2, ACT_SHOT, ACT_REC,
       ACT_MLB, ACT_MRB, ACT_FKEYS, ACT_PRECISE, ACT_KEYBOARD, ACT_MENU, ACT_KEY0 };     /* saved by name, so the numbers may move */
static const akey_t akeys[] = {     /* Amiga raw key codes */
    {"Space", 0x40}, {"Return", 0x44}, {"Esc", 0x45}, {"Backspace", 0x41}, {"Tab", 0x42}, {"Del", 0x46}, {"Help", 0x5F},
    {"F1", 0x50}, {"F2", 0x51}, {"F3", 0x52}, {"F4", 0x53}, {"F5", 0x54}, {"F6", 0x55}, {"F7", 0x56}, {"F8", 0x57},
    {"F9", 0x58}, {"F10", 0x59}, {"P", 0x19}, {"Y", 0x15}, {"N", 0x36}, {"1", 0x01}, {"2", 0x02}, {"3", 0x03},
    {"4", 0x04}, {"5", 0x05}, {"L Shift", 0x60}, {"Ctrl", 0x63}, {"L Alt", 0x64}, {"L Amiga", 0x66},
    {"Cursor Up", 0x4C}, {"Cursor Down", 0x4D}, {"Cursor Left", 0x4F}, {"Cursor Right", 0x4E},
};
#define NAKEYS ((int)(sizeof akeys / sizeof akeys[0]))
#define NACTS (ACT_KEY0 + NAKEYS)
static const char *act_name(int a) {
    static const char *b[] = {"(nothing)", "Fire", "Joystick up", "Joystick down", "Joystick left", "Joystick right", "Fire 2 (second button)",
                              "Screenshot", "Record video", "Left mouse button", "Right mouse button", "Function keys (hold)",
                              "Precise mouse (hold)", "Amiga keyboard", "Menu"};
    return a < 0 || a >= NACTS ? b[0] : a < ACT_KEY0 ? b[a] : akeys[a - ACT_KEY0].name;
}

/* controller inputs that can be mapped */
enum { PB_A, PB_B, PB_X, PB_Y, PB_LB, PB_RB, PB_LT, PB_RT, PB_BACK, PB_START, NPB };
static const char *pb_name[NPB] = {"A", "B", "X", "Y", "LB", "RB", "LT", "RT", "Back", "Start"};
static const int pb_glyph[NPB] = {GL_A, GL_B, GL_X, GL_Y, GL_LB, GL_RB, GL_LT, GL_RT, GL_BACK, GL_START};
static int act_of_key(const char *n) { for (int i = 0; i < NAKEYS; i++) if (!strcmp(akeys[i].name, n)) return ACT_KEY0 + i; return 0; }

/* ------------------------------------------------------------------ settings */
typedef struct {
    int fullscreen, scale, aspect43, crt, refresh50, show_menu;
    int paula, volume, stereo;
    int map[NPB], deadzone, osk_combo, menu_combo;
    int game_opt[8];
    int osk_hold, rec_mode, replay_len, cap_indicator, cfgver;     /* added 17 Oct 2026 (cfgver 2) */
    int rstick, osk_chord, scale_mode, filter;                     /* added 17 Oct 2026 evening (cfgver 3): Karl's controller scheme, scaling */
} settings_t;
static settings_t S_;
static const char *crt_names[] = {"Off", "Scanlines", "Scanlines + mask"};
static const char *paula_names[] = {"Off (raw)", "Amiga 500", "Amiga 1200"};
enum { OSK_L3R3, OSK_BACKY, OSK_LBRBY, OSK_LBRB, OSK_NCOMBO };   /* osk_combo: kept in the file for older builds, no longer used */
static const char *osk_hold_names[] = {"30 ms", "100 ms", "250 ms", "500 ms"};
static const char *rstick_names[] = {"Mouse", "Cursor keys"};
static const char *scale_names[] = {"Automatic", "Aspect fit", "Integer fit", "Fill (crop)", "Stretch"};
static const char *filter_names[] = {"Automatic", "Nearest", "Linear", "Sharp"};
static const int osk_hold_ms[] = {30, 100, 250, 500};
static const char *rec_mode_names[] = {"Start / stop recording", "Save the last few seconds"};
static const char *replay_names[] = {"10 seconds", "20 seconds", "30 seconds", "60 seconds"};
static const int replay_secs[] = {10, 20, 30, 60};

static void settings_default(settings_t *s) {
    memset(s, 0, sizeof *s);
#ifdef __ANDROID__
    s->fullscreen = 1;
#endif
    s->scale = 3; s->aspect43 = 1; s->crt = 0; s->refresh50 = 1; s->show_menu = 1;
    s->paula = 1; s->volume = 80; s->stereo = 70;      /* 100 = the Amiga's hard left/right split */
    /* Karl 17 Oct (the scheme from the Copilot design he approved): A Fire 1, B Fire 2, X Space, Y Return, LB / RB the mouse
     * buttons, LT the F-key layer, RT precise mouse, Back the pop-up keyboard, Start the menu (capture moved to Start + LB / RB) */
    s->map[PB_A] = ACT_FIRE; s->map[PB_B] = ACT_FIRE2; s->map[PB_X] = act_of_key("Space"); s->map[PB_Y] = act_of_key("Return");
    s->map[PB_LB] = ACT_MLB; s->map[PB_RB] = ACT_MRB; s->map[PB_LT] = ACT_FKEYS; s->map[PB_RT] = ACT_PRECISE;
    s->map[PB_BACK] = ACT_KEYBOARD; s->map[PB_START] = ACT_MENU; s->deadzone = 25;
    s->osk_combo = OSK_LBRB; s->osk_hold = 0; s->osk_chord = 0;  /* LB + RB held for the keyboard: optional, off (they are the mouse buttons) */
    s->rec_mode = 0; s->replay_len = 2; s->cap_indicator = 1;
    s->rstick = 0; s->scale_mode = 0; s->filter = 0; s->cfgver = 3;
}
static char *settings_path(void) {
    static char p[1024];
    char *b = SDL_GetPrefPath("AmiGo", game_title);
    snprintf(p, sizeof p, "%ssettings.ini", b ? b : "./");
    if (b) SDL_free(b);
    return p;
}
#define SETTINGS_FIELDS(X) X(fullscreen) X(scale) X(aspect43) X(crt) X(refresh50) X(show_menu) X(paula) X(volume) \
    X(stereo) X(deadzone) X(osk_combo) X(menu_combo) X(osk_hold) X(rec_mode) X(replay_len) X(cap_indicator) X(cfgver) \
    X(rstick) X(osk_chord) X(scale_mode) X(filter)
static void settings_save(const settings_t *s) {
    FILE *f = fopen(settings_path(), "w");
    if (!f) return;
#define W_(k) fprintf(f, #k "=%d\n", s->k);
    SETTINGS_FIELDS(W_)
    for (int i = 0; i < NPB; i++) fprintf(f, "map_%s=%s\n", pb_name[i], act_name(s->map[i]));
    for (int i = 0; i < 8; i++) fprintf(f, "game_opt%d=%d\n", i, s->game_opt[i]);
    fclose(f);
}
static void settings_load(settings_t *s) {
    settings_default(s);
    FILE *f = fopen(settings_path(), "r");
    if (!f) return;
    s->cfgver = 1;                      /* a file without cfgver was written before 17 Oct */
    char l[256], k[64], v[128];
    while (fgets(l, sizeof l, f)) {
        if (sscanf(l, "%63[^=]=%127[^\n]", k, v) != 2) continue;
#define R_(n) if (!strcmp(k, #n)) s->n = atoi(v);
        SETTINGS_FIELDS(R_)
        for (int i = 0; i < NPB; i++) { char kk[32]; snprintf(kk, sizeof kk, "map_%s", pb_name[i]);
            if (!strcmp(k, kk)) { s->map[i] = 0; for (int a = 0; a < NACTS; a++) if (!strcmp(act_name(a), v)) s->map[i] = a; } }
        for (int i = 0; i < 8; i++) { char kk[16]; snprintf(kk, sizeof kk, "game_opt%d", i); if (!strcmp(k, kk)) s->game_opt[i] = atoi(v); }
    }
    fclose(f);
    if (s->cfgver < 3) {   /* every menu exit saved the old defaults, so Karl's new controller scheme comes in once (17 Oct) */
        settings_t d; settings_default(&d);
        memcpy(s->map, d.map, sizeof s->map);
        s->osk_chord = 0; s->osk_hold = 0; s->rstick = 0; s->scale_mode = 0; s->filter = 0;
        if (s->cfgver < 2) { s->rec_mode = 0; s->replay_len = 2; s->cap_indicator = 1; }
        s->cfgver = 3;
    }
    if (s->scale < 1 || s->scale > 8) s->scale = 3;
    if (s->osk_combo < 0 || s->osk_combo >= OSK_NCOMBO) s->osk_combo = OSK_LBRB;
    if (s->osk_hold < 0 || s->osk_hold > 3) s->osk_hold = 0;
    if (s->rstick < 0 || s->rstick > 1) s->rstick = 0;
    if (s->osk_chord < 0 || s->osk_chord > 1) s->osk_chord = 0;
    if (s->scale_mode < 0 || s->scale_mode > 4) s->scale_mode = 0;
    if (s->filter < 0 || s->filter > 3) s->filter = 0;
    if (s->rec_mode < 0 || s->rec_mode > 1) s->rec_mode = 0;
    if (s->replay_len < 0 || s->replay_len > 3) s->replay_len = 2;
}

/* Game-specific switches (trainer, infinite lives...) come from the game's hooks_c, if it defines these. */
__attribute__((weak)) int amigo_game_option_count(void) { return 0; }
__attribute__((weak)) const char *amigo_game_option_name(int i) { (void)i; return ""; }
__attribute__((weak)) void amigo_game_option_set(int i, int on) { (void)i; (void)on; }

/* capture (reccap.c, included after this file) and the keyboard (below) */
static void reccap_shot(void);
static void reccap_button(void);
static const char *reccap_dir(void);
static void reccap_draw(int ow, int oh, float dt);
static int reccap_wants_draw(void);
static int osk_open;
static void osk_toggle(void);

/* ------------------------------------------------------------------ menu items, grouped in sections */
typedef struct { const char *label; int kind; int *val; int lo, hi; const char **names; int id; int sec; } item_t;
enum { K_HEAD, K_CHOICE, K_ACTION, K_MAP, K_GAMEOPT, K_INFO };
enum { ID_START = 1, ID_DEFAULTS, ID_QUIT, ID_PCT = 5 };
enum { SEC_PLAY, SEC_DISPLAY, SEC_SOUND, SEC_PAD, SEC_KEYS, SEC_CAPTURE, SEC_GAME, SEC_SYSTEM, NSEC };
static const char *sec_name[NSEC] = {"Play", "Display", "Sound", "Controller", "Keyboard", "Capture", "Game", "System"};
static const char *sec_sub[NSEC] = {"", "Screen mode, picture shape and the CRT look", "Amiga sound filters, volume and stereo width",
    "What each button does (the D-pad and left stick are the joystick)", "The pop-up Amiga keyboard",
    "Screenshots and video", "Switches for this game", "Settings and quitting"};
static item_t items[72]; static int nitems;
static void add(int sec, const char *l, int kind, int *v, int lo, int hi, const char **names, int id) {
    if (nitems < 72) items[nitems++] = (item_t){l, kind, v, lo, hi, names, id, sec};
}
static void build_items(settings_t *s, int pause) {
    static const char *onoff[] = {"Off", "On"}, *fsn[] = {"Window", "Full screen"};
    static const char *asp[] = {"Square pixels", "4:3 (as on a TV)"};
    static char scales[9][16]; static const char *scn[9];
    for (int i = 0; i < 9; i++) { snprintf(scales[i], 16, "%dx", i); scn[i] = scales[i]; }
    static char pct[21][8]; static const char *pcn[21];
    for (int i = 0; i <= 20; i++) { snprintf(pct[i], 8, "%d%%", i * 5); pcn[i] = pct[i]; }
    nitems = 0;
    add(SEC_PLAY, pause ? "Resume game" : "Start game", K_ACTION, 0, 0, 0, 0, ID_START);
    add(SEC_DISPLAY, "Mode", K_CHOICE, &s->fullscreen, 0, 1, fsn, 0);
    add(SEC_DISPLAY, "Window size", K_CHOICE, &s->scale, 1, 8, scn, 0);
    add(SEC_DISPLAY, "Picture shape", K_CHOICE, &s->aspect43, 0, 1, asp, 0);
    add(SEC_DISPLAY, "Scaling", K_CHOICE, &s->scale_mode, 0, 4, scale_names, 0);
    add(SEC_DISPLAY, "Filter", K_CHOICE, &s->filter, 0, 3, filter_names, 0);
    add(SEC_DISPLAY, "Automatic: whole-number scaling when x2 or more fits well, otherwise aspect fit, sharp", K_INFO, 0, 0, 0, 0, 0);
    add(SEC_DISPLAY, "CRT look", K_CHOICE, &s->crt, 0, 2, crt_names, 0);
    add(SEC_DISPLAY, "Use a 50 Hz display mode if the screen has one", K_CHOICE, &s->refresh50, 0, 1, onoff, 0);
    add(SEC_SOUND, "Amiga sound filters", K_CHOICE, &s->paula, 0, 2, paula_names, 0);
    add(SEC_SOUND, "Volume", K_CHOICE, &s->volume, 0, 20, pcn, ID_PCT);
    add(SEC_SOUND, "Stereo width", K_CHOICE, &s->stereo, 0, 20, pcn, ID_PCT);
    for (int i = 0; i < NPB; i++) add(SEC_PAD, pb_name[i], K_MAP, &s->map[i], 0, NACTS - 1, 0, i);
    add(SEC_PAD, "Right stick (click it to switch)", K_CHOICE, &s->rstick, 0, 1, rstick_names, 0);
    add(SEC_PAD, "D-pad and left stick: joystick (port 2). A second controller is joystick port 1", K_INFO, 0, 0, 0, 0, 0);
    add(SEC_PAD, "Hold LT: D-pad F1-F4, A B X Y F5-F8, LB RB F9 F10, Start Esc, Back Help", K_INFO, 0, 0, 0, 0, 0);
    add(SEC_PAD, "Start + LB screenshot, + RB video, + RT picture, + X help", K_INFO, 0, 0, 0, 0, 0);
    add(SEC_KEYS, "Back (View) opens and closes it", K_INFO, 0, 0, 0, 0, 0);
    add(SEC_KEYS, "Also open it by holding LB + RB", K_CHOICE, &s->osk_chord, 0, 1, onoff, 0);
    add(SEC_KEYS, "Hold LB + RB for", K_CHOICE, &s->osk_hold, 0, 3, osk_hold_names, 0);
    add(SEC_KEYS, "Touch or mouse: tap the keyboard button in the corner", K_INFO, 0, 0, 0, 0, 0);
    add(SEC_KEYS, "On it: D-pad move, A press, B close, X lock a modifier, Y next page", K_INFO, 0, 0, 0, 0, 0);
    add(SEC_KEYS, "Modifiers: tap once for the next key, twice to lock, again to let go", K_INFO, 0, 0, 0, 0, 0);
    add(SEC_CAPTURE, "Video button", K_CHOICE, &s->rec_mode, 0, 1, rec_mode_names, 0);
    add(SEC_CAPTURE, "Instant replay length", K_CHOICE, &s->replay_len, 0, 3, replay_names, 0);
    add(SEC_CAPTURE, "Recording indicator", K_CHOICE, &s->cap_indicator, 0, 1, onoff, 0);
    add(SEC_CAPTURE, "Controller: Start + LB screenshot, Start + RB video", K_INFO, 0, 0, 0, 0, 0);
    add(SEC_CAPTURE, "Keyboard: F12 screenshot, Shift+F12 video", K_INFO, 0, 0, 0, 0, 0);
    add(SEC_CAPTURE, "Saved to", K_INFO, 0, 0, 0, 0, 1);
    int ng = amigo_game_option_count();
    for (int i = 0; i < ng && i < 8; i++) add(SEC_GAME, amigo_game_option_name(i), K_GAMEOPT, &s->game_opt[i], 0, 1, onoff, 0);
    add(SEC_SYSTEM, "Reset all settings to defaults", K_ACTION, 0, 0, 0, 0, ID_DEFAULTS);
    add(SEC_SYSTEM, pause ? "Quit the game" : "Quit", K_ACTION, 0, 0, 0, 0, ID_QUIT);
}
/* items with a step of 5% store 0..100 but show 0..20 */
static int istep(const item_t *it) { return it->kind == K_CHOICE && it->id == ID_PCT ? 5 : 1; }
static const char *ival(const item_t *it) {
    if (it->kind == K_MAP) return act_name(*it->val);
    if (it->kind == K_CHOICE || it->kind == K_GAMEOPT) { int v = *it->val / istep(it); if (v < it->lo) v = it->lo; if (v > it->hi) v = it->hi; return it->names[v]; }
    if (it->kind == K_INFO && it->id == 1) return reccap_dir();
    return "";
}
static int sec_has(int sec) { for (int i = 0; i < nitems; i++) if (items[i].sec == sec) return 1; return 0; }
static int selectable(const item_t *it) { return it->kind != K_HEAD && it->kind != K_INFO; }

/* ------------------------------------------------------------------ applying settings */
static SDL_Texture *crt_tex;
static void apply_settings(SDL_Window *win, SDL_Renderer *ren, settings_t *s, int *aspect43) {
    *aspect43 = s->aspect43;
    SDL_RenderSetLogicalSize(ren, 0, 0);   /* the game is drawn in screen pixels by fe_present (scaling and filters, below) */
    paula_set_model(s->paula);
    if (s->fullscreen) {
        int done = 0;
        if (s->refresh50) {   /* exclusive full screen at 50 Hz, desktop resolution, if the display offers it */
            int di = SDL_GetWindowDisplayIndex(win); SDL_DisplayMode dm, cur;
            SDL_GetDesktopDisplayMode(di, &cur);
            for (int i = 0; i < SDL_GetNumDisplayModes(di); i++)
                if (SDL_GetDisplayMode(di, i, &dm) == 0 && dm.refresh_rate == 50 && dm.w == cur.w && dm.h == cur.h) {
                    SDL_SetWindowDisplayMode(win, &dm); SDL_SetWindowFullscreen(win, SDL_WINDOW_FULLSCREEN); done = 1; break; }
        }
        if (!done) SDL_SetWindowFullscreen(win, SDL_WINDOW_FULLSCREEN_DESKTOP);
    } else {
        SDL_SetWindowFullscreen(win, 0);
        /* never larger than the desktop: drop the scale until the window fits the display's usable area */
        int sc = s->scale, ww = s->aspect43 ? SCR_W * 16 / 15 : SCR_W; SDL_Rect ub;
        if (SDL_GetDisplayUsableBounds(SDL_GetWindowDisplayIndex(win), &ub) == 0)
            while (sc > 1 && (ww * sc > ub.w || SCR_H * sc > ub.h)) sc--;
        SDL_SetWindowSize(win, ww * sc, SCR_H * sc);
        SDL_SetWindowPosition(win, SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED);
    }
    /* the CRT look is made by fe_present at the size the picture is prescaled to (one dark line per Amiga line) */
    for (int i = 0; i < amigo_game_option_count() && i < 8; i++) amigo_game_option_set(i, s->game_opt[i]);
}

/* ------------------------------------------------------------------ the launcher: layout and state */
static int pad_btn(SDL_GameController *p, int b) { return p && SDL_GameControllerGetButton(p, b); }
static int ui_input;                    /* which hints to show: 0 controller, 1 keyboard, 2 touch or mouse */
static void ui_set_padstyle(SDL_GameController *p) {
    ui_padstyle = 0;
    if (p) { SDL_GameControllerType t = SDL_GameControllerGetType(p);
        if (t == SDL_CONTROLLER_TYPE_PS3 || t == SDL_CONTROLLER_TYPE_PS4 || t == SDL_CONTROLLER_TYPE_PS5) ui_padstyle = 1; }
}
/* logical canvas: the renderer's output size, but never under 1080 lines (smaller outputs get a 1080-line canvas scaled down
 * by SDL); the 1920x1080 design area is scaled by u and centred, so ultrawide and 4K displays keep the proportions */
static struct { float LW, LH, u, x0, y0; int ow, oh; } ML;
static void menu_layout(SDL_Window *win, SDL_Renderer *ren) {
    int ow = 0, oh = 0; SDL_GetRendererOutputSize(ren, &ow, &oh);
    if (ow < 16 || oh < 16) { ow = 1920; oh = 1080; }
    SDL_DisplayMode dm, cm; int di = SDL_GetWindowDisplayIndex(win), dw = 0, dh = 0, cw = 0, ch = 0;
    if (SDL_GetDesktopDisplayMode(di, &dm) == 0) { dw = dm.w; dh = dm.h; }
    if (SDL_GetCurrentDisplayMode(di, &cm) == 0) { cw = cm.w; ch = cm.h; }
    ML.LH = oh < 1080 ? 1080.0f : (float)oh;
    ML.LW = ML.LH * (float)ow / (float)oh;
    ML.u = fminf(ML.LH / 1080.0f, ML.LW / 1920.0f);
    ML.x0 = (ML.LW - 1920.0f * ML.u) / 2; ML.y0 = (ML.LH - 1080.0f * ML.u) / 2;
    ML.ow = ow; ML.oh = oh;
    SDL_RenderSetLogicalSize(ren, (int)(ML.LW + 0.5f), (int)(ML.LH + 0.5f));
    ui_px = ML.LH / (float)oh;
    fprintf(stderr, "menu: canvas %.0fx%.0f (scale %.2f) on output %dx%d; desktop mode %dx%d, current mode %dx%d\n",
            ML.LW, ML.LH, ML.u, ow, oh, dw, dh, cw, ch);
}
#define MX(v) (ML.x0 + (v) * ML.u)
#define MY(v) (ML.y0 + (v) * ML.u)
#define MS(v) ((v) * ML.u)
#define UU(v) ((v) * ui_u)      /* the current drawing scale (menu or overlay) */
static int m_nav, m_col, m_row[NSEC], m_shown = -1, m_prev = -1;
static float m_tt = 1, m_hl_nav = -1, m_hl_row = -1, m_scroll[NSEC], m_scroll_to[NSEC];
static SDL_FRect m_hit_nav[NSEC], m_hit_row[72], m_hit_l[72], m_hit_r[72]; static int m_hit_n, m_hit_item[72], m_vis[NSEC], m_nvis;
static char m_toast[96]; static float m_toast_t;
static float m_ease(float t) { if (t <= 0) return 0; if (t >= 1) return 1; t = 1 - t; return 1 - t * t * t; }
static int m_in(const SDL_FRect *r, float x, float y) { return x >= r->x && x < r->x + r->w && y >= r->y && y < r->y + r->h; }
static int m_sec_items(int sec, int *out) { int n = 0; for (int i = 0; i < nitems; i++) if (items[i].sec == sec) out[n++] = i; return n; }
static int m_first_sel(int sec) { int ix[72], n = m_sec_items(sec, ix); for (int k = 0; k < n; k++) if (selectable(&items[ix[k]])) return k; return 0; }

/* ------------------------------------------------------------------ the launcher: drawing */
static void m_background(float t) {
    float W = ML.LW, H = ML.LH, u = ML.u;
    ui_quad4(0, 0, W, H, 0xFF0B1022u, 0xFF0E1631u, 0xFF080A17u, 0xFF120A24u);
    ui_soft(W * (0.20f + 0.06f * sinf(t * 0.11f)), H * (0.26f + 0.07f * cosf(t * 0.09f)), 780 * u, 0x582F6BFFu, 1);
    ui_soft(W * (0.84f + 0.05f * cosf(t * 0.07f)), H * (0.66f + 0.06f * sinf(t * 0.08f)), 880 * u, 0x40A23CFFu, 1);
    ui_soft(W * (0.52f + 0.12f * sinf(t * 0.05f)), H * 1.04f, 700 * u, 0x2E18D6C0u, 1);
    /* a slow retro horizon grid (a nod to the Amiga demo scene), faint enough to stay behind the text */
    float hy = H * 0.80f, vx = W / 2;
    for (int i = 0; i < 14; i++) { float p = fmodf(i / 14.0f + t * 0.035f, 1.0f), y = hy + (H - hy) * p * p; ui_line(0, y, W, y, 1.4f * ui_px, ui_a(0xFF6FA8FFu, 0.04f + 0.12f * p)); }
    for (int i = -14; i <= 14; i++) ui_line(vx + i * W / 70.0f, hy, vx + i * W / 9.0f, H, 1.2f * ui_px, ui_a(0xFF6FA8FFu, 0.07f));
    ui_quad4(0, hy - 2, W, (H - hy) * 0.45f, 0xFF0A0D1Du, 0xFF0A0D1Du, 0x000A0D1Du, 0x000A0D1Du);
    ui_quad4(0, 0, W, H * 0.22f, 0x70000000u, 0x70000000u, 0, 0);
    ui_quad4(0, H * 0.84f, W, H * 0.16f, 0, 0, 0x80000000u, 0x80000000u);
}
static void m_icon(int sec, float cx, float cy, float s, uint32_t c) {
    switch (sec) {
    case SEC_PLAY: ui_tri3(cx - s * .35f, cy - s * .45f, cx + s * .5f, cy, cx - s * .35f, cy + s * .45f, c); break;
    case SEC_DISPLAY: ui_rrect_line(cx - s * .55f, cy - s * .4f, s * 1.1f, s * .7f, s * .1f, s * .12f, c); ui_rrect(cx - s * .25f, cy + s * .38f, s * .5f, s * .1f, s * .05f, c, c); break;
    case SEC_SOUND: ui_rrect(cx - s * .5f, cy - s * .18f, s * .3f, s * .36f, s * .05f, c, c); ui_tri3(cx - s * .25f, cy - s * .18f, cx + s * .1f, cy - s * .48f, cx + s * .1f, cy + s * .48f, c);
                    ui_tri3(cx - s * .25f, cy - s * .18f, cx + s * .1f, cy + s * .48f, cx - s * .25f, cy + s * .18f, c);
                    ui_line(cx + s * .28f, cy - s * .2f, cx + s * .28f, cy + s * .2f, s * .1f, c); ui_line(cx + s * .46f, cy - s * .34f, cx + s * .46f, cy + s * .34f, s * .1f, c); break;
    case SEC_PAD: ui_rrect_line(cx - s * .6f, cy - s * .32f, s * 1.2f, s * .64f, s * .32f, s * .12f, c); ui_circle(cx + s * .28f, cy, s * .09f, c); ui_rrect(cx - s * .38f, cy - s * .04f, s * .26f, s * .08f, s * .03f, c, c); break;
    case SEC_KEYS: for (int r = 0; r < 2; r++) for (int k = 0; k < 4; k++) ui_rrect(cx - s * .56f + k * s * .29f, cy - s * .3f + r * s * .28f, s * .22f, s * .2f, s * .04f, c, c);
                   ui_rrect(cx - s * .4f, cy + s * .26f, s * .8f, s * .14f, s * .04f, c, c); break;
    case SEC_CAPTURE: ui_ring_line(cx, cy, s * .45f, s * .1f, c); ui_circle(cx, cy, s * .22f, 0xFFFF5A5Au); break;
    case SEC_GAME: ui_tri3(cx, cy - s * .5f, cx + s * .45f, cy, cx, cy + s * .5f, c); ui_tri3(cx, cy - s * .5f, cx, cy + s * .5f, cx - s * .45f, cy, c); break;
    default: ui_ring_line(cx, cy, s * .4f, s * .12f, c); for (int i = 0; i < 8; i++) { float a = i * UI_PI / 4; ui_line(cx + cosf(a) * s * .4f, cy + sinf(a) * s * .4f, cx + cosf(a) * s * .56f, cy + sinf(a) * s * .56f, s * .14f, c); } break;
    }
}
/* the shortcut strip under the controller picture: "Start + LB Screenshot" and so on, centred on cx; returns the width */
static float m_shortcuts(float cx, float yc, float h) {
    static const struct { int g1, g2; const char *t; } S[] = {
        {GL_START, GL_LB, "Screenshot"}, {GL_START, GL_RB, "Video"}, {GL_START, GL_RT, "Picture"}, {GL_START, GL_X, "Help"}};
    float tot = 0;
    for (int pass = 0; pass < 2; pass++) {
        float x = cx - tot / 2, w = 0;
        ui_measure = !pass;
        for (int i = 0; i < 4; i++) {
            if (i) w += UU(40);
            w += ui_glyph(S[i].g1, x + w, yc, h);
            float pw = ui_text_w(F_SMALL, "+", 1, 0); if (pass) ui_textc(F_SMALL, x + w + UU(6), yc, "+", 0xFF9FB0CCu, 1, 0, 0); w += pw + UU(12);
            w += ui_glyph(S[i].g2, x + w, yc, h) + UU(10);
            w += pass ? ui_textc(F_SMALL, x + w, yc, S[i].t, 0xFFDCE3F0u, 1, 0, 0) : ui_text_w(F_SMALL, S[i].t, 1, 0);
        }
        tot = w;
    }
    ui_measure = 0;
    return tot;
}
static void m_hint_g(float *xr, float yc, int g1, int g2, const char *label) {
    float h = UU(34), lw = ui_text_w(F_SMALL, label, 1, 0);
    ui_textc(F_SMALL, *xr, yc, label, 0xFFDCE3F0u, 1, 2, 0);
    float x = *xr - lw - UU(12);
    ui_measure = 1; float w2 = g2 >= 0 ? ui_glyph(g2, 0, 0, h) : 0, w1 = ui_glyph(g1, 0, 0, h); ui_measure = 0;
    if (g2 >= 0) { x -= w2; ui_glyph(g2, x, yc, h); x -= UU(6); }
    x -= w1; ui_glyph(g1, x, yc, h);
    *xr = x - UU(40);
}
static void m_hint_k(float *xr, float yc, const char *k1, const char *k2, const char *label) {
    float h = UU(34), lw = ui_text_w(F_SMALL, label, 1, 0);
    ui_textc(F_SMALL, *xr, yc, label, 0xFFDCE3F0u, 1, 2, 0);
    float x = *xr - lw - UU(12);
    ui_measure = 1; float w2 = k2 ? ui_keycap(k2, 0, 0, h) : 0, w1 = ui_keycap(k1, 0, 0, h); ui_measure = 0;
    if (k2) { x -= w2; ui_keycap(k2, x, yc, h); x -= UU(6); }
    x -= w1; ui_keycap(k1, x, yc, h);
    *xr = x - UU(40);
}
/* the Play card: a controller with a callout for every button (Karl 14 Oct: "a picture of an Xbox style controller and
 * arrows to each mapping"), and the hotkeys underneath */
static void m_play_card(const settings_t *s, float px, float py, float pw, float ph) {
    float cx = px + pw / 2, cy = py + MS(318);
    ui_text(F_H2, px + MS(48), py + MS(34), "Controls", 0xFFFFFFFFu, 1, 0, 0);
    ui_text(F_SMALL, px + MS(48), py + MS(78), "Change any of these under Controller", 0xFF9FB0CCu, 1, 0, 0);
    const uint32_t b1 = 0xFF2E3954u, b2 = 0xFF1C2337u, dk = 0xFF121828u, kc = 0xFF3A4660u;
    ui_rrect(cx - MS(205), cy - MS(160), MS(92), MS(56), MS(20), 0xFF46526Cu, 0xFF2A3348u);
    ui_rrect(cx + MS(113), cy - MS(160), MS(92), MS(56), MS(20), 0xFF46526Cu, 0xFF2A3348u);
    ui_rrect(cx - MS(228), cy - MS(122), MS(146), MS(32), MS(16), 0xFF56637Fu, 0xFF38425Au);
    ui_rrect(cx + MS(82), cy - MS(122), MS(146), MS(32), MS(16), 0xFF56637Fu, 0xFF38425Au);
    ui_circle(cx - MS(152), cy + MS(64), MS(92), b2); ui_circle(cx + MS(152), cy + MS(64), MS(92), b2);
    ui_rrect(cx - MS(238), cy - MS(104), MS(476), MS(196), MS(98), b1, b2);
    ui_rrect_line(cx - MS(238), cy - MS(104), MS(476), MS(196), MS(98), ui_px * 1.5f, 0x22FFFFFFu);
    ui_circle(cx - MS(128), cy - MS(28), MS(40), dk); ui_circle(cx - MS(128), cy - MS(28), MS(28), kc);
    ui_circle(cx + MS(68), cy + MS(48), MS(40), dk); ui_circle(cx + MS(68), cy + MS(48), MS(28), kc);
    ui_rrect(cx - MS(78), cy + MS(20), MS(20), MS(56), MS(5), dk, dk); ui_rrect(cx - MS(96), cy + MS(38), MS(56), MS(20), MS(5), dk, dk);
    ui_glyph(GL_Y, cx + MS(113), cy - MS(60), MS(30)); ui_glyph(GL_X, cx + MS(81), cy - MS(28), MS(30));
    ui_glyph(GL_B, cx + MS(145), cy - MS(28), MS(30)); ui_glyph(GL_A, cx + MS(113), cy + MS(4), MS(30));
    ui_rrect(cx - MS(56), cy - MS(59), MS(28), MS(14), MS(7), kc, kc); ui_rrect(cx + MS(28), cy - MS(59), MS(28), MS(14), MS(7), kc, kc);
    ui_circle(cx, cy - MS(72), MS(15), 0xFF3F4B66u);
    struct { int g; float ax, ay, ly; const char *t; } L[5] = {
        {GL_LT, -160, -150, -186, 0}, {GL_LB, -175, -110, -132, 0}, {GL_BACK, -42, -52, -78, 0},
        {GL_LS, -128, -28, -24, "Joystick"}, {GL_DPAD, -68, 48, 40, "Joystick"}};
    struct { int g; float ax, ay, ly; const char *t; } R[8] = {
        {GL_RT, 160, -150, -196, 0}, {GL_RB, 175, -110, -150, 0}, {GL_START, 42, -52, -104, 0}, {GL_Y, 128, -60, -58, 0},
        {GL_X, 96, -28, -12, 0}, {GL_B, 160, -28, 34, 0}, {GL_A, 128, 4, 80, 0}, {GL_RS, 68, 48, 126, s->rstick ? "Cursor keys" : "Mouse"}};
    const int pbof[] = {PB_A, PB_B, PB_X, PB_Y, PB_LB, PB_RB, PB_LT, PB_RT, PB_BACK, PB_START};
    float gh = MS(32), maxw = MS(250);
    for (int side = 0; side < 2; side++) {
        int n = side ? 8 : 5;
        for (int i = 0; i < n; i++) {
            int g = side ? R[i].g : L[i].g; float ax = cx + MS(side ? R[i].ax : L[i].ax), ay = cy + MS(side ? R[i].ay : L[i].ay), ly = cy + MS(side ? R[i].ly : L[i].ly);
            const char *t = side ? R[i].t : L[i].t;
            if (!t) { for (int b = 0; b < NPB; b++) if (pb_glyph[pbof[b]] == g) t = act_name(s->map[pbof[b]]); }
            float sc = ui_fit(F_SMALL, t, maxw - gh - MS(12), 0), tw = ui_text_w(F_SMALL, t, sc, 0);
            ui_measure = 1; float gw = ui_glyph(g, 0, 0, gh); ui_measure = 0;
            float x0, x1;
            if (!side) { x1 = cx - MS(292); x0 = x1 - gw - MS(12) - tw; ui_glyph(g, x0, ly, gh); ui_textc(F_SMALL, x0 + gw + MS(12), ly, t, 0xFFE6ECF7u, sc, 0, 0);
                         ui_line(x1 + MS(12), ly, ax, ay, MS(2), 0x70AFC6FFu); }
            else { x0 = cx + MS(292); ui_glyph(g, x0, ly, gh); ui_textc(F_SMALL, x0 + gw + MS(12), ly, t, 0xFFE6ECF7u, sc, 0, 0);
                   ui_line(x0 - MS(12), ly, ax, ay, MS(2), 0x70AFC6FFu); (void)x1; }
            ui_circle(ax, ay, MS(4.5f), 0xFF9CC2FFu);
        }
    }
    /* hotkeys */
    float yc = py + ph - MS(62), h = MS(34);
    ui_rrect(px + MS(32), yc - MS(34), pw - MS(64), MS(68), MS(20), 0x14FFFFFFu, 0x0CFFFFFFu);
    m_shortcuts(cx, yc, h);
}
/* a settings section: title, then rows (label left, value right with arrows) */
static void m_section(int sec, int focused_sec, float px, float py, float pw, float ph, float dt) {
    ui_text(F_H1, px + MS(48), py + MS(30), sec_name[sec], 0xFFFFFFFFu, 1, 0, 0);
    ui_text(F_SMALL, px + MS(50), py + MS(92), sec_sub[sec], 0xFF9FB0CCu, 1, 0, 0);
    int ix[72], n = m_sec_items(sec, ix);
    float ry0 = py + MS(146), rh = MS(74), area = ph - MS(146) - MS(22);
    float maxs = n * rh - area; if (maxs < 0) maxs = 0;
    if (m_scroll_to[sec] < 0) m_scroll_to[sec] = 0; if (m_scroll_to[sec] > maxs) m_scroll_to[sec] = maxs;
    m_scroll[sec] += (m_scroll_to[sec] - m_scroll[sec]) * (1 - expf(-dt * 16));
    SDL_Rect clip = {(int)px, (int)(ry0 - MS(4)), (int)pw, (int)(area + MS(8))};
    ui_clip(&clip);
    int mine = sec == focused_sec;
    if (mine) {
        float ty = ry0 + m_row[sec] * rh - m_scroll[sec];
        if (m_hl_row < 0) m_hl_row = ty;
        m_hl_row += (ty - m_hl_row) * (1 - expf(-dt * 22));
        if (m_col == 1) {
            ui_soft(px + pw / 2, m_hl_row + rh / 2, MS(420), 0x162F6BFFu, 1);
            ui_rrect(px + MS(22), m_hl_row + MS(4), pw - MS(44), rh - MS(8), MS(16), 0x3A3B74FFu, 0x305A4BFFu);
            ui_rrect_line(px + MS(22), m_hl_row + MS(4), pw - MS(44), rh - MS(8), MS(16), ui_px * 1.5f, 0x667FA6FFu);
            ui_rrect(px + MS(22), m_hl_row + MS(16), MS(5), rh - MS(32), MS(2.5f), 0xFF6FA8FFu, 0xFF8A6BFFu);
        }
        m_hit_n = 0;
    }
    for (int k = 0; k < n; k++) {
        item_t *it = &items[ix[k]]; float y = ry0 + k * rh - m_scroll[sec], yc = y + rh / 2;
        if (y + rh < ry0 - MS(4) || y > ry0 + area + MS(4)) continue;
        int foc = mine && m_col == 1 && k == m_row[sec];
        uint32_t lc = foc ? 0xFFFFFFFFu : 0xFFD5DDEBu;
        float lx = px + MS(56), vr = px + pw - MS(64);
        if (it->kind == K_INFO) {
            ui_textc(F_SMALL, lx, yc, it->label, 0xFF8E9DB8u, 1, 0, 0);
            const char *v = ival(it);
            if (*v) { float lw = ui_text_w(F_SMALL, it->label, 1, 0) + MS(24); ui_textc(F_SMALL, vr + MS(30), yc, v, 0xFFB8C6DDu, ui_fit(F_SMALL, v, pw - MS(110) - lw, 0), 2, 0); }
            continue;
        }
        if (it->kind == K_MAP) { float gw = ui_glyph(pb_glyph[it->id], lx, yc, MS(36)); ui_textc(F_BODY, lx + gw + MS(16), yc, it->label, lc, 1, 0, 0); }
        else if (it->kind == K_ACTION) {
            uint32_t c = it->id == ID_QUIT ? 0xFFFF8A80u : it->id == ID_DEFAULTS ? 0xFFFFCB7Au : lc;
            ui_textc(F_BODY, lx, yc, it->label, c, ui_fit(F_BODY, it->label, pw - MS(200), 0), 0, 0);
        } else ui_textc(F_BODY, lx, yc, it->label, lc, ui_fit(F_BODY, it->label, pw * 0.52f, 0), 0, 0);
        const char *v = ival(it);
        if (mine && m_hit_n < 72) { m_hit_item[m_hit_n] = k; m_hit_row[m_hit_n] = (SDL_FRect){px + MS(22), y, pw - MS(44), rh};
                                   m_hit_l[m_hit_n] = (SDL_FRect){0, 0, 0, 0}; m_hit_r[m_hit_n] = (SDL_FRect){0, 0, 0, 0}; }
        if (it->kind == K_ACTION) {
            if (foc && ui_input == 0) ui_glyph(GL_A, vr - MS(34), yc, MS(34));
            else ui_textc(F_H2, vr, yc, "\xE2\x80\xBA", foc ? 0xFFFFFFFFu : 0xFF6C7A94u, 1, 2, 0);
        } else if (*v) {
            float sc = ui_fit(F_BODY, v, pw * 0.36f, 0), vw = ui_text_w(F_BODY, v, sc, 0);
            uint32_t vc = foc ? 0xFFFFFFFFu : 0xFFAFC4E8u, ac = foc ? 0xFFFFFFFFu : 0x556C7A94u;
            if (foc) ui_rrect(vr - vw - MS(66), yc - MS(22), vw + MS(100), MS(44), MS(22), 0x334D8DFFu, 0x334D8DFFu);
            ui_textc(F_BODY, vr - MS(4), yc, v, vc, sc, 2, 0);
            ui_textc(F_H2, vr - vw - MS(40), yc, "\xE2\x80\xB9", ac, 1, 1, 0);
            ui_textc(F_H2, vr + MS(20), yc, "\xE2\x80\xBA", ac, 1, 1, 0);
            if (mine && m_hit_n < 72) { m_hit_l[m_hit_n] = (SDL_FRect){vr - vw - MS(70), y, MS(60), rh}; m_hit_r[m_hit_n] = (SDL_FRect){vr, y, MS(60), rh}; }
        }
        if (mine && m_hit_n < 72) m_hit_n++;
    }
    ui_clip(NULL);
    if (maxs > 0) {   /* scroll position */
        float bh = area * area / (n * rh), by = ry0 + (area - bh) * (m_scroll[sec] / maxs);
        ui_rrect(px + pw - MS(16), by, MS(5), bh, MS(2.5f), 0x40FFFFFFu, 0x40FFFFFFu);
    }
}

/* ------------------------------------------------------------------ the launcher: the loop */
static item_t *m_item_at(int sec, int k) { int ix[72], n = m_sec_items(sec, ix); return k >= 0 && k < n ? &items[ix[k]] : NULL; }
static void m_change(item_t *it, int d) {
    if (!it || !(it->kind == K_CHOICE || it->kind == K_GAMEOPT || it->kind == K_MAP)) return;
    int st = istep(it), v = *it->val / st + d;
    if (v < it->lo) v = it->hi; if (v > it->hi) v = it->lo;
    *it->val = v * st;
}
/* returns 0 = start or resume, 1 = quit */
static int menu_run(SDL_Window *win, SDL_Renderer *ren, settings_t *s, int pause, SDL_GameController **pad, int *aspect43) {
    if (adev_global) SDL_PauseAudioDevice(adev_global, 1);
#ifndef __ANDROID__
    if (!pause && !s->fullscreen) {   /* a PC window: give the launcher most of the desktop (the game's own size comes back after) */
        SDL_Rect ub;
        if (SDL_GetDisplayUsableBounds(SDL_GetWindowDisplayIndex(win), &ub) == 0) {
            int w = ub.w * 85 / 100, h = w * 9 / 16;
            if (h > ub.h * 85 / 100) { h = ub.h * 85 / 100; w = h * 16 / 9; }
            SDL_SetWindowSize(win, w, h); SDL_SetWindowPosition(win, SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED);
        }
    }
#endif
    if (s->fullscreen && !(SDL_GetWindowFlags(win) & SDL_WINDOW_FULLSCREEN)) {   /* the launcher fills the screen too */
        SDL_SetWindowFullscreen(win, SDL_WINDOW_FULLSCREEN_DESKTOP); SDL_PumpEvents();
    }
    menu_layout(win, ren);
    ui_init(ren, ML.u);
    ui_set_padstyle(*pad);
#ifdef __ANDROID__
    ui_input = *pad ? 0 : 2;
#else
    ui_input = *pad ? 0 : 1;
#endif
    build_items(s, pause);
    m_nvis = 0;
    for (int sc = 0; sc < NSEC; sc++) if (sc == SEC_PLAY || sec_has(sc)) m_vis[m_nvis++] = sc;
    if (!pause || m_nav >= m_nvis) { m_nav = 0; m_col = 0; }
    m_shown = m_vis[m_nav]; m_prev = -1; m_tt = 1; m_hl_nav = -1; m_hl_row = -1; m_hit_n = 0;
    Uint32 t_open = SDL_GetTicks(), last = t_open;
    int touched = pause, result = -1;
    const int AUTO_S = 30;
    int pdir = 0; Uint32 prep = 0;
    int mdown = 0, mdrag = 0; float mdx = 0, mdy = 0, clx = 0, cly = 0;
    while (result < 0) {
        Uint32 now = SDL_GetTicks(); float dt = (now - last) / 1000.0f; if (dt > 0.1f) dt = 0.1f; last = now;
        float t = (now - t_open) / 1000.0f;
        int mv = 0, chg = 0, ok = 0, back = 0, secd = 0, start = 0, clicked = 0, any = 0, padev = 0;
        SDL_Event e;
        while (SDL_PollEvent(&e)) {
            switch (e.type) {
            case SDL_QUIT: result = 1; break;
            case SDL_WINDOWEVENT:
                if (e.window.event == SDL_WINDOWEVENT_SIZE_CHANGED) { menu_layout(win, ren); ui_init(ren, ML.u); m_hl_nav = m_hl_row = -1; }
                break;
            case SDL_CONTROLLERDEVICEADDED:
                if (!*pad) { *pad = SDL_GameControllerOpen(e.cdevice.which); ui_set_padstyle(*pad); if (*pad) ui_input = 0; }
                break;
            case SDL_CONTROLLERDEVICEREMOVED:
                if (*pad && SDL_GameControllerFromInstanceID(e.cdevice.which) == *pad) {
                    SDL_GameControllerClose(*pad); *pad = NULL;
                    for (int i = 0; i < SDL_NumJoysticks() && !*pad; i++) if (SDL_IsGameController(i)) *pad = SDL_GameControllerOpen(i);
                    ui_set_padstyle(*pad);
                }
                break;
            case SDL_KEYDOWN:
                fprintf(stderr, "menu: key down sym %d scancode %d\n", (int)e.key.keysym.sym, (int)e.key.keysym.scancode);   /* logcat (tag amigo) */
                ui_input = 1; any = 1;
                switch (e.key.keysym.sym) {
                case SDLK_UP: mv = -1; break; case SDLK_DOWN: mv = 1; break;
                case SDLK_LEFT: chg = -1; break; case SDLK_RIGHT: chg = 1; break;
                case SDLK_RETURN: case SDLK_KP_ENTER: case SDLK_SPACE: case SDLK_SELECT: ok = 1; break;
                case SDLK_ESCAPE: case SDLK_BACKSPACE: case SDLK_AC_BACK: case SDLK_F9: back = 1; break;
                case SDLK_TAB: secd = (e.key.keysym.mod & KMOD_SHIFT) ? -1 : 1; break;
                case SDLK_PAGEUP: secd = -1; break; case SDLK_PAGEDOWN: secd = 1; break;
                default: break;
                }
                break;
            case SDL_CONTROLLERBUTTONDOWN:
                fprintf(stderr, "menu: pad button %d\n", (int)e.cbutton.button);
                ui_input = 0; any = 1; padev = 1;
                switch (e.cbutton.button) {
                case SDL_CONTROLLER_BUTTON_DPAD_UP: mv = -1; pdir = 1; prep = now + 380; break;
                case SDL_CONTROLLER_BUTTON_DPAD_DOWN: mv = 1; pdir = 2; prep = now + 380; break;
                case SDL_CONTROLLER_BUTTON_DPAD_LEFT: chg = -1; pdir = 3; prep = now + 380; break;
                case SDL_CONTROLLER_BUTTON_DPAD_RIGHT: chg = 1; pdir = 4; prep = now + 380; break;
                case SDL_CONTROLLER_BUTTON_A: ok = 1; break;
                case SDL_CONTROLLER_BUTTON_B: back = 1; break;
                case SDL_CONTROLLER_BUTTON_LEFTSHOULDER: secd = -1; break;
                case SDL_CONTROLLER_BUTTON_RIGHTSHOULDER: secd = 1; break;
                case SDL_CONTROLLER_BUTTON_START: start = 1; break;
                default: break;
                }
                break;
            case SDL_MOUSEMOTION:
                ui_input = 2;
                if (mdown) {
                    mdx += e.motion.xrel; mdy += e.motion.yrel;
                    if (fabsf(mdx) > MS(16) || fabsf(mdy) > MS(16)) mdrag = 1;
                    if (mdrag && m_shown != SEC_PLAY) { m_scroll_to[m_shown] -= e.motion.yrel; m_scroll[m_shown] = m_scroll_to[m_shown]; }
                } else if (abs(e.motion.xrel) + abs(e.motion.yrel) > 2) {
                    any = 1;
                    for (int h = 0; h < m_hit_n; h++) if (m_in(&m_hit_row[h], (float)e.motion.x, (float)e.motion.y)) {
                        item_t *it = m_item_at(m_vis[m_nav], m_hit_item[h]);
                        if (it && selectable(it)) { m_col = 1; m_row[m_vis[m_nav]] = m_hit_item[h]; }
                    }
                }
                break;
            case SDL_MOUSEBUTTONDOWN:
                if (e.button.button == SDL_BUTTON_LEFT) { mdown = 1; mdrag = 0; mdx = mdy = 0; }
                ui_input = 2; any = 1;
                break;
            case SDL_MOUSEBUTTONUP:
                if (e.button.button == SDL_BUTTON_LEFT && mdown) { mdown = 0; if (!mdrag) { clicked = 1; clx = (float)e.button.x; cly = (float)e.button.y; } }
                break;
            case SDL_MOUSEWHEEL:
                if (m_shown != SEC_PLAY) m_scroll_to[m_shown] -= e.wheel.y * MS(74);
                ui_input = 2; any = 1;
                break;
            default: break;
            }
        }
        if (result >= 0) break;
        if (clicked) {
            int done = 0;
            for (int i = 0; i < m_nvis && !done; i++) if (m_in(&m_hit_nav[i], clx, cly)) {
                done = 1;
                if (m_vis[i] == SEC_PLAY) start = 1; else { m_nav = i; m_col = 0; }
            }
            for (int h = 0; h < m_hit_n && !done; h++) if (m_in(&m_hit_row[h], clx, cly)) {
                done = 1;
                item_t *it = m_item_at(m_vis[m_nav], m_hit_item[h]);
                if (it && selectable(it)) {
                    m_col = 1; m_row[m_vis[m_nav]] = m_hit_item[h];
                    if (m_in(&m_hit_l[h], clx, cly)) chg = -1; else if (m_in(&m_hit_r[h], clx, cly)) chg = 1; else ok = 1;
                }
            }
        }
        if (*pad) {   /* D-pad and left stick, with key repeat */
            int ay = SDL_GameControllerGetAxis(*pad, SDL_CONTROLLER_AXIS_LEFTY), ax = SDL_GameControllerGetAxis(*pad, SDL_CONTROLLER_AXIS_LEFTX);
            int d = (pad_btn(*pad, SDL_CONTROLLER_BUTTON_DPAD_UP) || ay < -20000) ? 1 : (pad_btn(*pad, SDL_CONTROLLER_BUTTON_DPAD_DOWN) || ay > 20000) ? 2 :
                    (pad_btn(*pad, SDL_CONTROLLER_BUTTON_DPAD_LEFT) || ax < -20000) ? 3 : (pad_btn(*pad, SDL_CONTROLLER_BUTTON_DPAD_RIGHT) || ax > 20000) ? 4 : 0;
            int fire = 0;
            if (d != pdir) { fire = d && !padev; pdir = d; prep = now + 380; }
            else if (d && now >= prep) { fire = 1; prep = now + 90; }
            if (fire) { ui_input = 0; any = 1; if (d == 1) mv = -1; else if (d == 2) mv = 1; else if (d == 3) chg = -1; else chg = 1; }
        }
        if (any || mv || chg || ok || back || secd || start || clicked) touched = 1;
        int left = touched ? -1 : AUTO_S - (int)((now - t_open) / 1000);
        if (!touched && left <= 0) result = 0;
        if (start) result = 0;
        int sec = m_vis[m_nav], scrollvis = 0;
        if (secd) {
            m_nav = (m_nav + secd + m_nvis) % m_nvis; sec = m_vis[m_nav];
            if (m_col == 1) { if (sec == SEC_PLAY) m_col = 0; else { m_row[sec] = m_first_sel(sec); scrollvis = 1; } }
        } else if (m_col == 0) {
            if (mv) { m_nav += mv; if (m_nav < 0) m_nav = 0; if (m_nav >= m_nvis) m_nav = m_nvis - 1; }
            sec = m_vis[m_nav];
            if (ok && sec == SEC_PLAY) result = 0;
            else if ((ok || chg > 0) && sec != SEC_PLAY) {
                m_col = 1; item_t *it = m_item_at(sec, m_row[sec]);
                if (!it || !selectable(it)) m_row[sec] = m_first_sel(sec);
                scrollvis = 1;
            }
            if (back) { if (pause) result = 0; else m_nav = 0; }
        } else {
            int ix[72], n = m_sec_items(sec, ix), k = m_row[sec];
            if (k < 0 || k >= n || !selectable(&items[ix[k]])) k = m_first_sel(sec);
            if (mv) { int j = k; do { j += mv; } while (j >= 0 && j < n && !selectable(&items[ix[j]])); if (j >= 0 && j < n) k = j; scrollvis = 1; }
            m_row[sec] = k;
            item_t *it = n ? &items[ix[k]] : NULL;
            if (it && chg) {
                if (it->kind == K_CHOICE || it->kind == K_GAMEOPT || it->kind == K_MAP) m_change(it, chg);
                else if (chg < 0) m_col = 0;
            }
            if (it && ok) {
                if (it->kind == K_ACTION) {
                    if (it->id == ID_START) result = 0;
                    else if (it->id == ID_QUIT) result = 1;
                    else if (it->id == ID_DEFAULTS) { settings_default(s); snprintf(m_toast, sizeof m_toast, "All settings are back to their defaults"); m_toast_t = 2.5f; }
                } else m_change(it, 1);
            }
            if (back) m_col = 0;
        }
        if (scrollvis && m_col == 1) {
            float rh = MS(74), area = MS(650 - 146 - 22), y = m_row[sec] * rh;
            if (y < m_scroll_to[sec]) m_scroll_to[sec] = y;
            if (y + rh > m_scroll_to[sec] + area) m_scroll_to[sec] = y + rh - area;
        }
        if (m_vis[m_nav] != m_shown) { m_prev = m_shown; m_shown = m_vis[m_nav]; m_tt = 0; m_hl_row = -1; }
        if (result >= 0) break;

        /* ---- draw */
        SDL_SetRenderDrawColor(ren, 0, 0, 0, 255); SDL_RenderClear(ren);
        ui_u = ML.u; ui_alpha = 1; ui_ox = ui_oy = 0;
        m_background(t);
        float intro = pause ? 1 : m_ease(t / 0.5f), sl = (1 - intro) * MS(28);
        ui_alpha = intro;
        ui_text(F_SMALL, MX(98), MY(64), pause ? "AMI-GO  \xC2\xB7  GAME PAUSED" : "AMI-GO  \xC2\xB7  NATIVE AMIGA PORT", 0xFF8FB8FFu, 1, 0, 3.0f);
        ui_text(F_TITLE, MX(92), MY(90) + sl, game_title, 0xFFFFFFFFu, ui_fit(F_TITLE, game_title, MS(1180), -1), 0, -1.0f);
        ui_quad4(MX(98), MY(214) + sl, MS(150), MS(6), 0xFF4D8DFFu, 0xFFA05CFFu, 0xFFA05CFFu, 0xFF4D8DFFu);
        {   time_t tt = time(NULL); struct tm *lt = localtime(&tt); char clk[16] = "";
            if (lt) strftime(clk, sizeof clk, "%H:%M", lt);
            ui_text(F_H2, MX(1824), MY(58), clk, 0xFFE8EEF8u, 1, 2, 0);
            const char *pn = *pad ? SDL_GameControllerName(*pad) : NULL; char st[128];
            snprintf(st, sizeof st, "%s", pn ? pn : "No controller - keyboard, mouse and touch work");
            float ssc = ui_fit(F_SMALL, st, MS(560), 0), sw = ui_text_w(F_SMALL, st, ssc, 0);
            ui_textc(F_SMALL, MX(1824), MY(122), st, 0xFF9FB0CCu, ssc, 2, 0);
            ui_circle(MX(1824) - sw - MS(16), MY(122), MS(6), pn ? 0xFF54D17Au : 0xFF7A8499u);
        }
        /* navigation column */
        float nx = MX(96), nw = MS(470), ny0 = MY(292), nh = MS(72), ng = MS(12);
        float ty = ny0 + m_nav * (nh + ng);
        if (m_hl_nav < 0) m_hl_nav = ty;
        m_hl_nav += (ty - m_hl_nav) * (1 - expf(-dt * 20));
        ui_ox = -(1 - intro) * MS(40);
        if (m_col == 0) {
            float pulse = 0.5f + 0.5f * sinf(t * 3.0f);
            ui_soft(nx + nw * 0.5f, m_hl_nav + nh / 2, MS(340), ui_a(0x503B74FFu, 0.65f + 0.35f * pulse), 1);
            ui_rrect(nx, m_hl_nav, nw, nh, MS(18), 0xFF3F7DFFu, 0xFF6A4BFFu);
            ui_rrect_line(nx, m_hl_nav, nw, nh, MS(18), ui_px * 1.5f, 0x55FFFFFFu);
        } else {
            ui_rrect(nx, m_hl_nav, nw, nh, MS(18), 0x26FFFFFFu, 0x18FFFFFFu);
            ui_rrect_line(nx, m_hl_nav, nw, nh, MS(18), ui_px * 1.5f, 0x557FA6FFu);
        }
        for (int i = 0; i < m_nvis; i++) {
            int sc = m_vis[i], foc = i == m_nav; float y = ny0 + i * (nh + ng), yc = y + nh / 2;
            uint32_t c = foc ? 0xFFFFFFFFu : 0xFFC3CEE2u;
            m_hit_nav[i] = (SDL_FRect){nx, y, nw, nh};
            m_icon(sc, nx + MS(42), yc, MS(30), c);
            ui_textc(F_H2, nx + MS(84), yc, sc == SEC_PLAY ? (pause ? "Resume" : "Play") : sec_name[sc], c, 1, 0, 0);
            if (sc == SEC_PLAY && left > 0) {
                char b[16]; snprintf(b, sizeof b, "%d s", left);
                ui_textc(F_SMALL, nx + nw - MS(26), yc, b, 0xE6FFFFFFu, 1, 2, 0);
                float frac = (now - t_open) / 1000.0f / AUTO_S; if (frac > 1) frac = 1;
                ui_rrect(nx + MS(22), y + nh - MS(12), nw - MS(44), MS(4), MS(2), 0x30FFFFFFu, 0x30FFFFFFu);
                ui_rrect(nx + MS(22), y + nh - MS(12), (nw - MS(44)) * (1 - frac), MS(4), MS(2), 0xE6FFFFFFu, 0xE6FFFFFFu);
            } else if (sc != SEC_PLAY) ui_textc(F_H2, nx + nw - MS(26), yc, "\xE2\x80\xBA", foc ? 0xFFFFFFFFu : 0x806C7A94u, 1, 2, 0);
        }
        /* the panel, with a slide and fade between sections */
        float px = MX(620), py = MY(292), pw = MS(1204), ph = MS(650), base = (1 - intro) * MS(40);
        ui_ox = base;
        ui_rrect(px, py, pw, ph, MS(28), 0x1CFFFFFFu, 0x0EFFFFFFu);
        ui_rrect_line(px, py, pw, ph, MS(28), ui_px * 1.5f, 0x26FFFFFFu);
        m_tt += dt / 0.24f; if (m_tt > 1) m_tt = 1;
        float e2 = m_ease(m_tt); int fsec = m_vis[m_nav];
        for (int pass = 0; pass < 2; pass++) {
            int sc = pass ? m_shown : m_prev;
            if (sc < 0 || (!pass && m_tt >= 1)) continue;
            ui_alpha = intro * (pass ? e2 : 1 - e2); ui_ox = base + (pass ? MS(50) * (1 - e2) : -MS(50) * e2);
            if (sc == SEC_PLAY) m_play_card(s, px, py, pw, ph); else m_section(sc, fsec, px, py, pw, ph, dt);
        }
        if (m_shown == SEC_PLAY) m_hit_n = 0;
        /* hints */
        ui_alpha = intro; ui_ox = 0;
        float yc = MY(1018), xr = MX(1824);
        if (m_toast_t > 0) { m_toast_t -= dt; ui_alpha = intro * (m_toast_t < 0.5f ? m_toast_t / 0.5f : 1); ui_textc(F_SMALL, MX(98), yc, m_toast, 0xFF8CE0A8u, 1, 0, 0); ui_alpha = intro; }
        else if (left > 0) { char b[96]; snprintf(b, sizeof b, "Starting in %d s - press any button to stay here", left); ui_textc(F_SMALL, MX(98), yc, b, 0xFFFFD27Au, 1, 0, 0); }
        if (ui_input == 0) {
            if (m_col == 0) { m_hint_g(&xr, yc, GL_START, -1, pause ? "Resume" : "Play"); m_hint_g(&xr, yc, GL_LB, GL_RB, "Section");
                              if (pause) m_hint_g(&xr, yc, GL_B, -1, "Resume"); m_hint_g(&xr, yc, GL_A, -1, "Select"); }
            else { m_hint_g(&xr, yc, GL_LB, GL_RB, "Section"); m_hint_g(&xr, yc, GL_B, -1, "Back"); m_hint_g(&xr, yc, GL_DPAD_LR, -1, "Change"); m_hint_g(&xr, yc, GL_A, -1, "Select"); }
        } else if (ui_input == 1) {
            m_hint_k(&xr, yc, "Tab", 0, "Section"); m_hint_k(&xr, yc, "Esc", 0, (m_col == 0 && pause) ? "Resume" : "Back");
            m_hint_k(&xr, yc, "\xE2\x86\x90", "\xE2\x86\x92", "Change"); m_hint_k(&xr, yc, "Enter", 0, "Select");
        } else ui_textc(F_SMALL, xr, yc, "Tap or click a setting to change it", 0xFFDCE3F0u, 1, 2, 0);
        ui_flush(); ui_alpha = 1; ui_ox = 0;
        SDL_RenderPresent(ren);
        Uint32 el = SDL_GetTicks() - now;
        if (el < 15) SDL_Delay(15 - el);
    }
    ui_alpha = 1; ui_ox = ui_oy = 0;
    settings_save(s);
    apply_settings(win, ren, s, aspect43);
    if (adev_global) SDL_PauseAudioDevice(adev_global, 0);
    fprintf(stderr, "menu: result %d touched %d\n", result, touched);
    return result;
}

/* ------------------------------------------------------------------ guest input: who holds each Amiga key
 * Karl, 17 Oct (controller scheme from the Copilot design he approved): every Amiga key, button and direction is tracked per
 * source, so letting go of one of two sources that hold the same Amiga input does not let it go; a key goes down when its first
 * source presses it and up when its last source lets go, and stays down for at least 2 guest frames so a quick tap is never
 * lost. Only raw Amiga key down/up events reach the game (kbd_push, the queue the real keyboard uses). */
enum { SRC_KBD = 0, SRC_OSK = 1, SRC_IME = 2, SRC_PAD0 = 8 };
#define SRC_PAD(n, i) (SRC_PAD0 + (n) * 24 + (i))                 /* pad n (0, 1), input i (0-23) */
#define SRC_PADMASK(n) ((((uint64_t)1 << 24) - 1) << (SRC_PAD0 + (n) * 24))
static uint64_t gk_mask[128]; static uint32_t gk_at[128], gk_frame; static uint8_t gk_pend[128];
static void gk_key(int code, int down, int src) {
    if (code < 0 || code > 127 || src < 0 || src > 63) return;
    uint64_t b = (uint64_t)1 << src, old = gk_mask[code];
    if (down) {
        if (old & b) return;
        gk_mask[code] = old | b;
        if (!old) { if (gk_pend[code]) gk_pend[code] = 0; else { kbd_push((uint8_t)code, 1); gk_at[code] = gk_frame; } }
    } else {
        if (!(old & b)) return;
        gk_mask[code] = old & ~b;
        if (!gk_mask[code]) { if (gk_frame - gk_at[code] >= 2) kbd_push((uint8_t)code, 0); else gk_pend[code] = 1; }
    }
}
static void gk_release_mask(uint64_t m) {
    for (int c = 0; c < 128; c++) if (gk_mask[c] & m) for (int s = 0; s < 64; s++) if (gk_mask[c] & m & ((uint64_t)1 << s)) gk_key(c, 0, s);
}
static void gk_tick(void) {   /* once per guest frame */
    gk_frame++;
    for (int c = 0; c < 128; c++) if (gk_pend[c] && gk_frame - gk_at[c] >= 2) { gk_pend[c] = 0; kbd_push((uint8_t)c, 0); }
}

/* ------------------------------------------------------------------ the Amiga mouse port (port 1)
 * The runtime reads JOY0DAT from mouse_joy0dat, the left button (CIA-A PRA bit 6) from mouse_lmb and the right button (POTGOR
 * bit 10) from amigo_potgor_clear. Weak here, so a runtime without them still links (the mouse then does nothing there).
 * Only changes are written, so the twin play harness, which drives the same pins, is never overwritten while we are idle. */
__attribute__((weak)) uint16_t mouse_joy0dat;
__attribute__((weak)) int mouse_lmb;
__attribute__((weak)) uint16_t amigo_potgor_clear;
static int fe_mq_x, fe_mq_y;                                 /* fractions of a mouse count, 1/256 */
static int fe_last_lmb = -1, fe_last_rmb = -1, fe_last_p1joy = -1;
static int fe_pcm_l, fe_pcm_r, fe_tap_l;                     /* PC mouse buttons, a touch tap (frames left) */
static int fe_fdiv256(int v) { return v >= 0 ? v / 256 : -((-v + 255) / 256); }
static void fe_mouse_q8(int dxq, int dyq) {                 /* move the Amiga mouse by dx/256, dy/256 counts */
    fe_mq_x += dxq; fe_mq_y += dyq;
    int cx = fe_fdiv256(fe_mq_x), cy = fe_fdiv256(fe_mq_y);
    fe_mq_x -= cx * 256; fe_mq_y -= cy * 256;
    if (cx || cy) { uint16_t v = mouse_joy0dat; mouse_joy0dat = (uint16_t)(((((v >> 8) + cy) & 0xff) << 8) | (((v & 0xff) + cx) & 0xff)); }
}
static uint16_t fe_joyenc(int u, int d, int l, int r) { return (uint16_t)((r << 1) | (l << 9) | ((r ^ d) & 1) | (((l ^ u) & 1) << 8)); }
static void fe_port1_out(int lmb, int rmb) {
    if (lmb != fe_last_lmb) { mouse_lmb = lmb; fe_last_lmb = lmb; }
    if (rmb != fe_last_rmb) { if (rmb) amigo_potgor_clear |= 0x0400; else amigo_potgor_clear &= (uint16_t)~0x0400; fe_last_rmb = rmb; }
}
static unsigned fe_isqrt(unsigned long long v) { unsigned long long r = 0, b = 1ull << 62; while (b > v) b >>= 2;
    while (b) { if (v >= r + b) { v -= r + b; r = (r >> 1) + b; } else r >>= 1; b >>= 2; } return (unsigned)r; }

/* ------------------------------------------------------------------ toasts, the input help and the F-key layer picture */
static char fe_toast_s[128]; static Uint32 fe_toast_until;
static void fe_toast(const char *s) { snprintf(fe_toast_s, sizeof fe_toast_s, "%s", s); fe_toast_until = SDL_GetTicks() + 2200; fprintf(stderr, "fe: %s\n", s); }
static int fe_help_on, fe_layer_shown;
static Uint32 fe_help_until;

/* ------------------------------------------------------------------ presentation: scaling and filters
 * Presentation only: the game's 352 x 272 frame (framebuf) is never changed; the capture code reads it as it is. The viewport
 * is worked out in integer and rational arithmetic from the drawable size in pixels, every frame, so a resize or a rotation
 * takes effect at once. Picture shape: 4:3 makes a lores pixel 16:15 wide (a PAL TV), otherwise square. */
static struct { int dx, dy, dw, dh, sx, sy, sw, sh, mode, filter, ok; } fe_vp;
static SDL_Texture *fe_big; static int fe_big_p;
static int fe_crt_p, fe_crt_kind;
static void fe_viewport(int ow, int oh, int mode, int a43) {
    long long num = (long long)SCR_W * (a43 ? 16 : 15), den = 15, H = SCR_H;     /* picture width in display units = num / den */
    int sx = 0, sy = 0, sw = SCR_W, sh = SCR_H; long long dw, dh;
    long long afw, afh;                                                           /* aspect fit */
    if ((long long)ow * H * den >= (long long)oh * num) { afh = oh; afw = (long long)oh * num / (H * den); }
    else { afw = ow; afh = (long long)ow * H * den / num; }
    long long k = (long long)ow * den / num; if ((long long)oh / H < k) k = oh / H;
    if (mode == 0) mode = (k >= 2 && k * H * 100 >= 85 * afh) ? 2 : 1;            /* Karl 17 Oct: integer when x2 or more fits comfortably */
    if (mode == 2 && k < 1) mode = 1;
    switch (mode) {
    case 2: dw = k * num / den; dh = k * H; break;
    case 3:
        dw = ow; dh = oh;
        if ((long long)ow * H * den >= (long long)oh * num) { sh = (int)((long long)oh * num / ((long long)ow * den)); if (sh < 1) sh = 1; if (sh > SCR_H) sh = SCR_H; sy = (SCR_H - sh) / 2; }
        else { sw = (int)((long long)ow * H * SCR_W * den / ((long long)oh * num)); if (sw < 1) sw = 1; if (sw > SCR_W) sw = SCR_W; sx = (SCR_W - sw) / 2; }
        break;
    case 4: dw = ow; dh = oh; break;
    default: mode = 1; dw = afw; dh = afh; break;
    }
    fe_vp.dw = (int)dw; fe_vp.dh = (int)dh; fe_vp.dx = (int)((ow - dw) / 2); fe_vp.dy = (int)((oh - dh) / 2);
    fe_vp.sx = sx; fe_vp.sy = sy; fe_vp.sw = sw; fe_vp.sh = sh; fe_vp.mode = mode; fe_vp.ok = dw > 0 && dh > 0;
}
static void fe_crt_make(SDL_Renderer *ren, int p, int kind) {   /* CRT look in the prescaled picture's pixels: one dark line per Amiga line */
    if (crt_tex) { SDL_DestroyTexture(crt_tex); crt_tex = NULL; }
    fe_crt_p = p; fe_crt_kind = kind;
    if (!kind || p < 2) return;
    int W = SCR_W * p, Hh = SCR_H * p;
    uint32_t *ov = malloc((size_t)W * Hh * 4); if (!ov) return;
    for (int y = 0; y < Hh; y++) for (int x = 0; x < W; x++) {
        uint32_t a = (y % p == p - 1) ? (kind == 1 ? 110u : 150u) : 0u, c = 0;
        if (kind == 2 && !a) { static const uint32_t m[3] = {0x0000FFFFu, 0x00FF00FFu, 0x00FFFF00u}; a = 40u; c = m[x % 3]; }
        ov[(size_t)y * W + x] = (a << 24) | (c & 0x00FFFFFFu);
    }
    crt_tex = SDL_CreateTexture(ren, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STATIC, W, Hh);
    if (crt_tex) { SDL_UpdateTexture(crt_tex, NULL, ov, W * 4); SDL_SetTextureBlendMode(crt_tex, SDL_BLENDMODE_BLEND); }
    free(ov);
}
static void fe_present(SDL_Renderer *ren, SDL_Texture *tex, SDL_Texture *big) {
    (void)big;
    int lw = 0, lh = 0; SDL_RenderGetLogicalSize(ren, &lw, &lh);
    if (lw || lh) SDL_RenderSetLogicalSize(ren, 0, 0);
    int ow = 0, oh = 0; SDL_GetRendererOutputSize(ren, &ow, &oh);
    SDL_SetRenderDrawColor(ren, 0, 0, 0, 255); SDL_RenderClear(ren);
    if (ow < 1 || oh < 1) return;
    fe_viewport(ow, oh, S_.scale_mode, S_.aspect43);
    int f = S_.filter; if (f == 0) f = fe_vp.mode == 2 ? 1 : 3;                   /* automatic: integer -> nearest, otherwise sharp */
    fe_vp.filter = f;
    SDL_Rect src = {fe_vp.sx, fe_vp.sy, fe_vp.sw, fe_vp.sh}, dst = {fe_vp.dx, fe_vp.dy, fe_vp.dw, fe_vp.dh};
    if (f != 3 && !S_.crt) {
        SDL_SetTextureScaleMode(tex, f == 1 ? SDL_ScaleModeNearest : SDL_ScaleModeLinear);
        SDL_RenderCopy(ren, tex, &src, &dst);
        return;
    }
    /* sharp: an integer prescale with nearest-neighbour into a target texture, then linear the rest of the way */
    int p = fe_vp.dh / (fe_vp.sh > 0 ? fe_vp.sh : 1); if (p < 1) p = 1; if (p > 8) p = 8;
    if (S_.crt && p < 2) p = 2;
    SDL_RendererInfo ri; if (SDL_GetRendererInfo(ren, &ri) == 0 && ri.max_texture_width > 0) while (p > 1 && (SCR_W * p > ri.max_texture_width || SCR_H * p > ri.max_texture_height)) p--;
    if (!fe_big || fe_big_p != p) {
        if (fe_big) SDL_DestroyTexture(fe_big);
        fe_big = SDL_CreateTexture(ren, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_TARGET, SCR_W * p, SCR_H * p); fe_big_p = p;
    }
    if (S_.crt != fe_crt_kind || (S_.crt && fe_crt_p != p)) fe_crt_make(ren, p, S_.crt);
    if (!fe_big) { SDL_SetTextureScaleMode(tex, SDL_ScaleModeLinear); SDL_RenderCopy(ren, tex, &src, &dst); return; }
    SDL_SetTextureScaleMode(tex, SDL_ScaleModeNearest);
    SDL_SetRenderTarget(ren, fe_big); SDL_RenderCopy(ren, tex, NULL, NULL);
    if (S_.crt && crt_tex) SDL_RenderCopy(ren, crt_tex, NULL, NULL);
    SDL_SetRenderTarget(ren, NULL);
    SDL_SetTextureScaleMode(fe_big, f == 1 ? SDL_ScaleModeNearest : SDL_ScaleModeLinear);
    SDL_Rect s2 = {src.x * p, src.y * p, src.w * p, src.h * p};
    SDL_RenderCopy(ren, fe_big, &s2, &dst);
}
/* screen pixel -> is it on the game's picture (letterbox bars are not) */
static int fe_in_picture(float x, float y) { return fe_vp.ok && x >= fe_vp.dx && x < fe_vp.dx + fe_vp.dw && y >= fe_vp.dy && y < fe_vp.dy + fe_vp.dh; }
/* a move of (dx, dy) screen pixels -> Amiga mouse counts (lores pixels), through the inverse of the picture's scale */
static void fe_mouse_px(float dx, float dy) {
    if (!fe_vp.ok || fe_vp.dw < 1 || fe_vp.dh < 1) return;
    fe_mouse_q8((int)lroundf(dx * 256.0f * fe_vp.sw / fe_vp.dw), (int)lroundf(dy * 256.0f * fe_vp.sh / fe_vp.dh));
}
static const int fe_presets[][2] = {{0, 0}, {2, 1}, {1, 3}, {1, 2}, {1, 1}, {3, 3}, {4, 2}};   /* Start + RT: scaling, filter */
static void fe_scale_cycle(void) {
    int n = (int)(sizeof fe_presets / sizeof fe_presets[0]), i = 0;
    for (int k = 0; k < n; k++) if (fe_presets[k][0] == S_.scale_mode && fe_presets[k][1] == S_.filter) i = k;
    i = (i + 1) % n; S_.scale_mode = fe_presets[i][0]; S_.filter = fe_presets[i][1];
    char b[96]; snprintf(b, sizeof b, "Picture: %s, %s", scale_names[S_.scale_mode], filter_names[S_.filter]); fe_toast(b);
    settings_save(&S_);
}

/* ------------------------------------------------------------------ the on-screen Amiga keyboard
 * Karl, 17 Oct: three pages, GAME (first: the keys games ask for - F1-F10, Esc, 0-9, Space, Return, Backspace, Tab, Help, Del,
 * the cursor keys, Ctrl, Shift, Alt, both Amiga keys, Y and N), ABC (the whole Amiga 500 main block) and NUMPAD (the keypad's
 * own raw codes, not the number row's). F1-F10 are on every page. Raw Amiga key down/up only; a tapped key stays down at least 2
 * guest frames; F-keys, Esc and Help never repeat (a tap, however long it is held); modifiers: one tap = one-shot (let go after
 * the next key), a quick double tap = locked, a tap when locked = let go. Controller: D-pad moves, A presses, B closes, X makes
 * the focused modifier sticky, Y changes page. Positions are in key units. */
typedef struct { const char *cap; unsigned char code; float x, y, w, h; unsigned char kind, norep; } okey_t;
enum { OK_KEY, OK_MOD, OK_CAPS, OK_TYPE, OK_MORE, OK_CLOSE };
#define FX(i) (1.2f + (i) * 1.3f)
#define OK_FROW {"Esc", 0x45, 0, 0, 1, 1, 0, 1}, {"F1", 0x50, FX(0), 0, 1.3f, 1, 0, 1}, {"F2", 0x51, FX(1), 0, 1.3f, 1, 0, 1}, \
    {"F3", 0x52, FX(2), 0, 1.3f, 1, 0, 1}, {"F4", 0x53, FX(3), 0, 1.3f, 1, 0, 1}, {"F5", 0x54, FX(4), 0, 1.3f, 1, 0, 1}, \
    {"F6", 0x55, FX(5), 0, 1.3f, 1, 0, 1}, {"F7", 0x56, FX(6), 0, 1.3f, 1, 0, 1}, {"F8", 0x57, FX(7), 0, 1.3f, 1, 0, 1}, \
    {"F9", 0x58, FX(8), 0, 1.3f, 1, 0, 1}, {"F10", 0x59, FX(9), 0, 1.3f, 1, 0, 1}, \
    {"Help", 0x5F, 14.4f, 0, 1.5f, 1, 0, 1}, {"Del", 0x46, 15.9f, 0, 1.5f, 1, 0, 0}
#define AR_U "\xE2\x86\x91"
#define AR_D "\xE2\x86\x93"
#define AR_L "\xE2\x86\x90"
#define AR_R "\xE2\x86\x92"
static const okey_t okp_game[] = {
    OK_FROW,
    {"1", 0x01, 0, 1, 1, 1, 0, 0}, {"2", 0x02, 1, 1, 1, 1, 0, 0}, {"3", 0x03, 2, 1, 1, 1, 0, 0}, {"4", 0x04, 3, 1, 1, 1, 0, 0}, {"5", 0x05, 4, 1, 1, 1, 0, 0},
    {"6", 0x06, 5, 1, 1, 1, 0, 0}, {"7", 0x07, 6, 1, 1, 1, 0, 0}, {"8", 0x08, 7, 1, 1, 1, 0, 0}, {"9", 0x09, 8, 1, 1, 1, 0, 0}, {"0", 0x0A, 9, 1, 1, 1, 0, 0},
    {"Backspace", 0x41, 10, 1, 2.4f, 1, 0, 0}, {AR_U, 0x4C, 15.4f, 1, 1, 1, 0, 0},
    {"Tab", 0x42, 0, 2, 1.5f, 1, 0, 0}, {"Ctrl", 0x63, 1.5f, 2, 1.5f, 1, OK_MOD, 1}, {"Shift", 0x60, 3, 2, 2, 1, OK_MOD, 1}, {"Alt", 0x64, 5, 2, 1.5f, 1, OK_MOD, 1},
    {"Y", 0x15, 6.5f, 2, 1, 1, 0, 0}, {"N", 0x36, 7.5f, 2, 1, 1, 0, 0}, {"Return", 0x44, 8.5f, 2, 3.9f, 1, 0, 0},
    {AR_L, 0x4F, 14.4f, 2, 1, 1, 0, 0}, {AR_D, 0x4D, 15.4f, 2, 1, 1, 0, 0}, {AR_R, 0x4E, 16.4f, 2, 1, 1, 0, 0},
    {"Amiga", 0x66, 0, 3, 1.5f, 1, OK_MOD, 1}, {"Space", 0x40, 1.5f, 3, 7, 1, 0, 0}, {"Amiga", 0x67, 8.5f, 3, 1.5f, 1, OK_MOD, 1},
    {"More", 0, 10, 3, 2.4f, 1, OK_MORE, 1}, {"Close", 0, 14.4f, 3, 3, 1, OK_CLOSE, 1},
};
static const okey_t okp_abc[] = {
    OK_FROW,
    {"`", 0x00, 0, 1, 1, 1, 0, 0}, {"1", 0x01, 1, 1, 1, 1, 0, 0}, {"2", 0x02, 2, 1, 1, 1, 0, 0}, {"3", 0x03, 3, 1, 1, 1, 0, 0}, {"4", 0x04, 4, 1, 1, 1, 0, 0},
    {"5", 0x05, 5, 1, 1, 1, 0, 0}, {"6", 0x06, 6, 1, 1, 1, 0, 0}, {"7", 0x07, 7, 1, 1, 1, 0, 0}, {"8", 0x08, 8, 1, 1, 1, 0, 0}, {"9", 0x09, 9, 1, 1, 1, 0, 0},
    {"0", 0x0A, 10, 1, 1, 1, 0, 0}, {"-", 0x0B, 11, 1, 1, 1, 0, 0}, {"=", 0x0C, 12, 1, 1, 1, 0, 0}, {"\\", 0x0D, 13, 1, 1, 1, 0, 0}, {"Bksp", 0x41, 14, 1, 1.5f, 1, 0, 0},
    {"Type", 0, 16.0f, 1, 1.4f, 1, OK_TYPE, 1},
    {"Tab", 0x42, 0, 2, 1.5f, 1, 0, 0}, {"Q", 0x10, 1.5f, 2, 1, 1, 0, 0}, {"W", 0x11, 2.5f, 2, 1, 1, 0, 0}, {"E", 0x12, 3.5f, 2, 1, 1, 0, 0}, {"R", 0x13, 4.5f, 2, 1, 1, 0, 0},
    {"T", 0x14, 5.5f, 2, 1, 1, 0, 0}, {"Y", 0x15, 6.5f, 2, 1, 1, 0, 0}, {"U", 0x16, 7.5f, 2, 1, 1, 0, 0}, {"I", 0x17, 8.5f, 2, 1, 1, 0, 0}, {"O", 0x18, 9.5f, 2, 1, 1, 0, 0},
    {"P", 0x19, 10.5f, 2, 1, 1, 0, 0}, {"[", 0x1A, 11.5f, 2, 1, 1, 0, 0}, {"]", 0x1B, 12.5f, 2, 1, 1, 0, 0}, {"Return", 0x44, 13.5f, 2, 2, 2, 0, 0},
    {"More", 0, 16.0f, 2, 1.4f, 1, OK_MORE, 1},
    {"Ctrl", 0x63, 0, 3, 1.25f, 1, OK_MOD, 1}, {"Caps", 0x62, 1.25f, 3, 1.25f, 1, OK_CAPS, 1}, {"A", 0x20, 2.5f, 3, 1, 1, 0, 0}, {"S", 0x21, 3.5f, 3, 1, 1, 0, 0},
    {"D", 0x22, 4.5f, 3, 1, 1, 0, 0}, {"F", 0x23, 5.5f, 3, 1, 1, 0, 0}, {"G", 0x24, 6.5f, 3, 1, 1, 0, 0}, {"H", 0x25, 7.5f, 3, 1, 1, 0, 0}, {"J", 0x26, 8.5f, 3, 1, 1, 0, 0},
    {"K", 0x27, 9.5f, 3, 1, 1, 0, 0}, {"L", 0x28, 10.5f, 3, 1, 1, 0, 0}, {";", 0x29, 11.5f, 3, 1, 1, 0, 0}, {"'", 0x2A, 12.5f, 3, 1, 1, 0, 0},
    {"Close", 0, 16.0f, 3, 1.4f, 1, OK_CLOSE, 1},
    {"Shift", 0x60, 0, 4, 2.25f, 1, OK_MOD, 1}, {"Z", 0x31, 2.25f, 4, 1, 1, 0, 0}, {"X", 0x32, 3.25f, 4, 1, 1, 0, 0}, {"C", 0x33, 4.25f, 4, 1, 1, 0, 0},
    {"V", 0x34, 5.25f, 4, 1, 1, 0, 0}, {"B", 0x35, 6.25f, 4, 1, 1, 0, 0}, {"N", 0x36, 7.25f, 4, 1, 1, 0, 0}, {"M", 0x37, 8.25f, 4, 1, 1, 0, 0},
    {",", 0x38, 9.25f, 4, 1, 1, 0, 0}, {".", 0x39, 10.25f, 4, 1, 1, 0, 0}, {"/", 0x3A, 11.25f, 4, 1, 1, 0, 0}, {"Shift", 0x61, 12.25f, 4, 2.75f, 1, OK_MOD, 1},
    {AR_U, 0x4C, 15.4f, 4, 1, 1, 0, 0},
    {"Alt", 0x64, 0, 5, 1.5f, 1, OK_MOD, 1}, {"Amiga", 0x66, 1.5f, 5, 1.5f, 1, OK_MOD, 1}, {"Space", 0x40, 3, 5, 9, 1, 0, 0},
    {"Amiga", 0x67, 12, 5, 1.5f, 1, OK_MOD, 1}, {"Alt", 0x65, 13.5f, 5, 1.5f, 1, OK_MOD, 1},
    {AR_L, 0x4F, 15.4f - 1, 5, 1, 1, 0, 0}, {AR_D, 0x4D, 15.4f, 5, 1, 1, 0, 0}, {AR_R, 0x4E, 16.4f, 5, 1, 1, 0, 0},
};
#define KX(c) (6.0f + (c) * 1.1f)
static const okey_t okp_num[] = {
    OK_FROW,
    {"More", 0, 0, 1, 3, 1, OK_MORE, 1}, {"Close", 0, 0, 2, 3, 1, OK_CLOSE, 1},
    {"(", 0x5A, KX(0), 1, 1.1f, 1, 0, 0}, {")", 0x5B, KX(1), 1, 1.1f, 1, 0, 0}, {"/", 0x5C, KX(2), 1, 1.1f, 1, 0, 0}, {"*", 0x5D, KX(3), 1, 1.1f, 1, 0, 0},
    {"7", 0x3D, KX(0), 2, 1.1f, 1, 0, 0}, {"8", 0x3E, KX(1), 2, 1.1f, 1, 0, 0}, {"9", 0x3F, KX(2), 2, 1.1f, 1, 0, 0}, {"-", 0x4A, KX(3), 2, 1.1f, 1, 0, 0},
    {"4", 0x2D, KX(0), 3, 1.1f, 1, 0, 0}, {"5", 0x2E, KX(1), 3, 1.1f, 1, 0, 0}, {"6", 0x2F, KX(2), 3, 1.1f, 1, 0, 0}, {"+", 0x5E, KX(3), 3, 1.1f, 1, 0, 0},
    {"1", 0x1D, KX(0), 4, 1.1f, 1, 0, 0}, {"2", 0x1E, KX(1), 4, 1.1f, 1, 0, 0}, {"3", 0x1F, KX(2), 4, 1.1f, 1, 0, 0}, {"Enter", 0x43, KX(3), 4, 1.1f, 2, 0, 0},
    {"0", 0x0F, KX(0), 5, 2.2f, 1, 0, 0}, {".", 0x3C, KX(2), 5, 1.1f, 1, 0, 0},
    {"Return", 0x44, 14.4f, 1, 3, 1, 0, 0}, {"Space", 0x40, 14.4f, 2, 3, 1, 0, 0},
    {AR_U, 0x4C, 15.4f, 4, 1, 1, 0, 0}, {AR_L, 0x4F, 14.4f, 5, 1, 1, 0, 0}, {AR_D, 0x4D, 15.4f, 5, 1, 1, 0, 0}, {AR_R, 0x4E, 16.4f, 5, 1, 1, 0, 0},
};
enum { OSK_PGAME, OSK_PABC, OSK_PNUM, OSK_NPAGE };
static const struct { const char *name; const okey_t *k; int n; float w, h; } osk_pages[OSK_NPAGE] = {
    {"GAME", okp_game, (int)(sizeof okp_game / sizeof okp_game[0]), 17.4f, 4},
    {"ABC", okp_abc, (int)(sizeof okp_abc / sizeof okp_abc[0]), 17.4f, 6},
    {"NUMPAD", okp_num, (int)(sizeof okp_num / sizeof okp_num[0]), 17.4f, 6},
};
#define OSK_MAXK 96
static int osk_page;
#define OKEYS (osk_pages[osk_page].k)
#define NOKEYS (osk_pages[osk_page].n)
static float osk_anim; static int osk_focus = 1, osk_padkey = -1, osk_pressed[OSK_MAXK], osk_rdir; static Uint32 osk_rnext;
static int osk_mod[8]; static Uint32 osk_mod_t[8];                /* 0x60-0x67: 0 off, 1 one-shot, 2 locked; Caps uses 1 = on */
static uint32_t osk_oneshot_at;                                    /* guest frame to let one-shot modifiers go (0 = none) */
static float osk_x0, osk_y0, osk_unit; static SDL_FRect osk_close_r, osk_fab_r, osk_tab_r[OSK_NPAGE];
static SDL_Renderer *osk_ren; static SDL_GameController *osk_pad;
static Uint32 ui_pointer_t; static int ui_ime;
static int osk_slot_key[11] = {-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1};

static void osk_layout(int ow, int oh, float *unit, float *pad, float *head, float *pw, float *ph, float *px, float *py);
static void osk_mods_release(int all) {   /* one-shot modifiers (or all of them) let go */
    for (int b = 0; b < 8; b++) {
        if (b == 2) { if (all && osk_mod[2]) { osk_mod[2] = 0; gk_key(0x62, 0, SRC_OSK); } continue; }   /* Caps Lock */
        if (osk_mod[b] == 1 || (all && osk_mod[b])) { osk_mod[b] = 0; gk_key(0x60 + b, 0, SRC_OSK); }
    }
}
static void osk_type(void) {   /* the platform's own keyboard, for typing names and text */
    ui_ime = 1;
    SDL_StartTextInput();
#ifdef _WIN32
    if (SDL_OpenURL("C:\\Program Files\\Common Files\\microsoft shared\\ink\\TabTip.exe") != 0) fprintf(stderr, "osk: touch keyboard: %s\n", SDL_GetError());
#endif
}
static void osk_set_page(int p) {
    for (int i = 0; i < NOKEYS && i < OSK_MAXK; i++) if (osk_pressed[i]) { osk_pressed[i] = 0; gk_key(OKEYS[i].code, 0, SRC_OSK); }
    osk_padkey = -1; for (int s = 0; s < 11; s++) osk_slot_key[s] = -1;
    osk_page = (p % OSK_NPAGE + OSK_NPAGE) % OSK_NPAGE; osk_focus = 1;          /* F1 */
    fprintf(stderr, "osk: page %s\n", osk_pages[osk_page].name);
}
static void osk_mod_tap(int code, int lock_only) {
    int b = code - 0x60; Uint32 now = SDL_GetTicks();
    if (b < 0 || b > 7) return;
    if (lock_only) {   /* controller X: sticky (locked), or let go when it already is */
        if (osk_mod[b] == 2) { osk_mod[b] = 0; gk_key(code, 0, SRC_OSK); }
        else { if (!osk_mod[b]) gk_key(code, 1, SRC_OSK); osk_mod[b] = 2; }
        return;
    }
    if (osk_mod[b] == 0) { osk_mod[b] = 1; osk_mod_t[b] = now; gk_key(code, 1, SRC_OSK); }
    else if (osk_mod[b] == 1 && now - osk_mod_t[b] < 500) osk_mod[b] = 2;          /* a double tap locks it */
    else { osk_mod[b] = 0; gk_key(code, 0, SRC_OSK); }
}
static void osk_toggle(void);
static void osk_down(int i) {
    if (i < 0 || i >= NOKEYS) return;
    const okey_t *k = &OKEYS[i];
    switch (k->kind) {
    case OK_TYPE: osk_type(); return;
    case OK_MORE: osk_set_page(osk_page + 1); return;
    case OK_CLOSE: osk_toggle(); return;
    case OK_MOD: osk_mod_tap(k->code, 0); return;
    case OK_CAPS: if (osk_mod[2]) { osk_mod[2] = 0; gk_key(0x62, 0, SRC_OSK); } else { osk_mod[2] = 1; gk_key(0x62, 1, SRC_OSK); } return;
    default: break;
    }
    if (k->norep) {   /* F-keys, Esc, Help: a tap (down now, up after the 2-frame minimum), however long it is held */
        gk_key(k->code, 1, SRC_OSK); gk_key(k->code, 0, SRC_OSK);
        osk_oneshot_at = gk_frame + 3; return;
    }
    if (!osk_pressed[i]) gk_key(k->code, 1, SRC_OSK);
    osk_pressed[i]++;
}
static void osk_up(int i) {
    if (i < 0 || i >= NOKEYS || OKEYS[i].kind != OK_KEY || OKEYS[i].norep || osk_pressed[i] <= 0) return;
    if (--osk_pressed[i] == 0) { gk_key(OKEYS[i].code, 0, SRC_OSK); osk_oneshot_at = gk_frame + 3; }
}
static void osk_toggle(void) {
    osk_open = !osk_open;
    fprintf(stderr, "osk: %s\n", osk_open ? "open" : "closed");
    if (!osk_open) {
        for (int i = 0; i < OSK_MAXK; i++) osk_pressed[i] = 0;
        gk_release_mask((uint64_t)1 << SRC_OSK);
        for (int b = 0; b < 8; b++) osk_mod[b] = 0;
        osk_padkey = -1; osk_oneshot_at = 0;
        for (int s = 0; s < 11; s++) osk_slot_key[s] = -1;
        if (ui_ime) { SDL_StopTextInput(); ui_ime = 0; }
    }
}
static void osk_nav(int dx, int dy) {
    if (osk_focus < 0 || osk_focus >= NOKEYS) osk_focus = 0;
    const okey_t *f = &OKEYS[osk_focus]; float cx = f->x + f->w / 2, cy = f->y + f->h / 2, bs = 1e9f; int best = -1;
    for (int pass = 0; pass < 2 && best < 0; pass++)
        for (int j = 0; j < NOKEYS; j++) {
            if (j == osk_focus) continue;
            const okey_t *k = &OKEYS[j]; float jx = k->x + k->w / 2, jy = k->y + k->h / 2;
            float sx = cx < k->x ? k->x - cx : cx > k->x + k->w ? cx - k->x - k->w : 0, sy = cy < k->y ? k->y - cy : cy > k->y + k->h ? cy - k->y - k->h : 0, sc;
            if (pass == 0) {
                if (dx) { if ((jx - cx) * dx <= 0.05f) continue; sc = fabsf(jx - cx) + 4 * sy; }
                else { if ((jy - cy) * dy <= 0.05f) continue; sc = fabsf(jy - cy) + 3 * sx; }
            } else {   /* nothing that way: wrap round to the far side of the same row or column */
                if (dx) { if (sy > 0) continue; sc = dx > 0 ? jx : -jx; }
                else { if (sx > 0) continue; sc = dy > 0 ? jy : -jy; }
            }
            if (sc < bs) { bs = sc; best = j; }
        }
    if (best >= 0) osk_focus = best;
}
static int osk_hit(float x, float y) {
    if (osk_unit <= 0) return -1;
    float kx = (x - osk_x0) / osk_unit, ky = (y - osk_y0) / osk_unit;
    for (int i = 0; i < NOKEYS; i++) if (kx >= OKEYS[i].x && kx < OKEYS[i].x + OKEYS[i].w && ky >= OKEYS[i].y && ky < OKEYS[i].y + OKEYS[i].h) return i;
    return -1;
}
static void osk_fab_rect(int ow, int oh) { float u = fminf(oh / 1080.0f, ow / 1920.0f), s = 92 * u; osk_fab_r = (SDL_FRect){ow - s - 36 * u, oh - s - 36 * u, s, s}; }
/* a touch or click on the keyboard at x,y (screen pixels); phase 1 down, 0 up; slot 0-9 fingers, 10 the mouse */
static void osk_pointer(int phase, int slot, float x, float y) {
    if (slot < 0 || slot > 10) return;
    if (!phase) { if (osk_slot_key[slot] >= 0) { osk_up(osk_slot_key[slot]); osk_slot_key[slot] = -1; } return; }
    if (osk_ren) { int ow, oh; float a, b, c, d, f, g, h; SDL_GetRendererOutputSize(osk_ren, &ow, &oh); osk_layout(ow, oh, &a, &b, &c, &d, &f, &g, &h); }
    if (m_in(&osk_close_r, x, y)) { osk_toggle(); return; }
    for (int p = 0; p < OSK_NPAGE; p++) if (m_in(&osk_tab_r[p], x, y)) { if (p != osk_page) osk_set_page(p); return; }
    int k = osk_hit(x, y);
    if (k >= 0) { osk_focus = k; osk_down(k); if (osk_open && OKEYS[k].kind == OK_KEY && !OKEYS[k].norep) osk_slot_key[slot] = k; }
}
static void osk_text(const char *s) {   /* text from the system keyboard -> Amiga key presses (US layout), raw down/up pairs */
    static const char plain[] = "`1234567890-=\\qwertyuiop[]asdfghjkl;'zxcvbnm,./", shifted[] = "~!@#$%^&*()_+|QWERTYUIOP{}ASDFGHJKL:\"ZXCVBNM<>?";
    static const unsigned char code[] = {0x00, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0x0A, 0x0B, 0x0C, 0x0D,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B,
        0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A,
        0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A};
    for (; *s; s++) {
        int c = (unsigned char)*s, k = -1, sh = 0;
        if (c == ' ') k = 0x40; else if (c == '\n' || c == '\r') k = 0x44;
        else { const char *p = strchr(plain, c); if (p && c) k = code[p - plain]; else { p = strchr(shifted, c); if (p && c) { k = code[p - shifted]; sh = 1; } } }
        if (k < 0) continue;
        if (sh) kbd_push(0x60, 1);
        kbd_push((uint8_t)k, 1); kbd_push((uint8_t)k, 0);
        if (sh) kbd_push(0x60, 0);
    }
}
/* the game's mouse from a PC mouse or touch: relative moves through the inverse of the picture's scale; a press only counts on
 * the picture (touches in the letterbox bars are ignored); a touch drags the pointer and a quick tap is a left click */
static int fe_touch_on; static Uint32 fe_touch_t; static float fe_touch_x, fe_touch_y, fe_touch_moved;
static void fe_game_pointer(const SDL_Event *e, float x, float y, float sx) {
    int touch = (e->type == SDL_MOUSEMOTION ? e->motion.which : e->button.which) == SDL_TOUCH_MOUSEID;
    if (e->type == SDL_MOUSEMOTION) {
        if (touch) { if (fe_touch_on) { fe_mouse_px(x - fe_touch_x, y - fe_touch_y); fe_touch_moved += fabsf(x - fe_touch_x) + fabsf(y - fe_touch_y); fe_touch_x = x; fe_touch_y = y; } }
        else fe_mouse_px(e->motion.xrel * sx, e->motion.yrel * sx);
        return;
    }
    int dn = e->type == SDL_MOUSEBUTTONDOWN;
    if (touch) {
        if (dn) { if (fe_in_picture(x, y)) { fe_touch_on = 1; fe_touch_t = SDL_GetTicks(); fe_touch_x = x; fe_touch_y = y; fe_touch_moved = 0; } }
        else if (fe_touch_on) { fe_touch_on = 0; if (SDL_GetTicks() - fe_touch_t < 300 && fe_touch_moved < 24 * sx + 12) fe_tap_l = 3; }
        return;
    }
    if (dn && !fe_in_picture(x, y)) return;
    if (e->button.button == SDL_BUTTON_LEFT) fe_pcm_l = dn; else if (e->button.button == SDL_BUTTON_RIGHT) fe_pcm_r = dn;
}
/* the keyboard takes the controller while it is open, and touch, the mouse and IME text always; returns 1 when it used the event */
static int osk_event(const SDL_Event *e) {
    Uint32 now = SDL_GetTicks();
    switch (e->type) {
    case SDL_CONTROLLERBUTTONDOWN: case SDL_CONTROLLERBUTTONUP: {
        if (!osk_open) return 0;
        int dn = e->type == SDL_CONTROLLERBUTTONDOWN;
        ui_input = 0;
        switch (e->cbutton.button) {
        case SDL_CONTROLLER_BUTTON_DPAD_UP: if (dn) { osk_nav(0, -1); osk_rdir = 1; osk_rnext = now + 380; } break;
        case SDL_CONTROLLER_BUTTON_DPAD_DOWN: if (dn) { osk_nav(0, 1); osk_rdir = 2; osk_rnext = now + 380; } break;
        case SDL_CONTROLLER_BUTTON_DPAD_LEFT: if (dn) { osk_nav(-1, 0); osk_rdir = 3; osk_rnext = now + 380; } break;
        case SDL_CONTROLLER_BUTTON_DPAD_RIGHT: if (dn) { osk_nav(1, 0); osk_rdir = 4; osk_rnext = now + 380; } break;
        case SDL_CONTROLLER_BUTTON_A:
            if (dn) { if (osk_padkey < 0) { int f = osk_focus; osk_down(f); if (osk_open && f < NOKEYS && OKEYS[f].kind == OK_KEY && !OKEYS[f].norep) osk_padkey = f; } }
            else if (osk_padkey >= 0) { osk_up(osk_padkey); osk_padkey = -1; }
            break;
        case SDL_CONTROLLER_BUTTON_B: case SDL_CONTROLLER_BUTTON_BACK: if (dn) osk_toggle(); break;
        case SDL_CONTROLLER_BUTTON_X: if (dn && osk_focus < NOKEYS && OKEYS[osk_focus].kind == OK_MOD) osk_mod_tap(OKEYS[osk_focus].code, 1); break;
        case SDL_CONTROLLER_BUTTON_Y: if (dn) osk_set_page(osk_page + 1); break;
        case SDL_CONTROLLER_BUTTON_LEFTSHOULDER: if (dn) osk_set_page(osk_page - 1); break;
        case SDL_CONTROLLER_BUTTON_RIGHTSHOULDER: if (dn) osk_set_page(osk_page + 1); break;
        default: break;
        }
        return 1; }
    case SDL_FINGERDOWN: case SDL_FINGERUP: case SDL_FINGERMOTION:
        /* touches are taken from the mouse events SDL makes from them (SDL_TOUCH_MOUSEID), in window pixels */
        ui_pointer_t = now; ui_input = 2;
        return 1;
    case SDL_MOUSEMOTION: case SDL_MOUSEBUTTONDOWN: case SDL_MOUSEBUTTONUP: {
        ui_pointer_t = now; ui_input = 2;
        if (!osk_ren) return 1;
        int ww, wh, ow, oh, wx = 0, wy = 0;
        SDL_Window *w = SDL_RenderGetWindow(osk_ren); SDL_GetWindowSize(w, &ww, &wh); SDL_GetRendererOutputSize(osk_ren, &ow, &oh);
        if (ww < 1 || wh < 1) return 1;
        float ex = e->type == SDL_MOUSEMOTION ? (float)e->motion.x : (float)e->button.x, ey = e->type == SDL_MOUSEMOTION ? (float)e->motion.y : (float)e->button.y;
        SDL_RenderLogicalToWindow(osk_ren, ex, ey, &wx, &wy);   /* logical units (if any) -> window */
        float sx = (float)ow / ww, x = (float)wx * sx, y = (float)wy * oh / wh;
        if (e->type != SDL_MOUSEMOTION && e->button.button == SDL_BUTTON_LEFT && (osk_open || e->type == SDL_MOUSEBUTTONDOWN))
            fprintf(stderr, "osk: %s %s at %.0f,%.0f\n", e->button.which == SDL_TOUCH_MOUSEID ? "touch" : "click", e->type == SDL_MOUSEBUTTONDOWN ? "down" : "up", x, y);
        if (osk_open) {
            if (e->type != SDL_MOUSEMOTION && e->button.button == SDL_BUTTON_LEFT) osk_pointer(e->type == SDL_MOUSEBUTTONDOWN, 10, x, y);
            return 1;
        }
        if (e->type == SDL_MOUSEBUTTONDOWN && e->button.button == SDL_BUTTON_LEFT && ui_pointer_t) {
            osk_fab_rect(ow, oh);
            if (m_in(&osk_fab_r, x, y)) { osk_toggle(); return 1; }
        }
        fe_game_pointer(e, x, y, sx);
        return 1; }
    case SDL_TEXTINPUT:
        if (ui_ime) { osk_text(e->text.text); return 1; }
        return 0;
    default: return 0;
    }
}
/* once per guest frame: the 2-frame key minimum, one-shot modifiers, held-direction repeat on the keyboard, touch taps */
static void osk_frame(void) {
    gk_tick();
    if (fe_tap_l > 0) fe_tap_l--;
    if (osk_oneshot_at && gk_frame >= osk_oneshot_at) { osk_oneshot_at = 0; osk_mods_release(0); }
    if (!osk_open || !osk_pad) return;
    Uint32 now = SDL_GetTicks();
    int ay = SDL_GameControllerGetAxis(osk_pad, SDL_CONTROLLER_AXIS_LEFTY), ax = SDL_GameControllerGetAxis(osk_pad, SDL_CONTROLLER_AXIS_LEFTX);
    int d = (pad_btn(osk_pad, SDL_CONTROLLER_BUTTON_DPAD_UP) || ay < -20000) ? 1 : (pad_btn(osk_pad, SDL_CONTROLLER_BUTTON_DPAD_DOWN) || ay > 20000) ? 2 :
            (pad_btn(osk_pad, SDL_CONTROLLER_BUTTON_DPAD_LEFT) || ax < -20000) ? 3 : (pad_btn(osk_pad, SDL_CONTROLLER_BUTTON_DPAD_RIGHT) || ax > 20000) ? 4 : 0;
    int fire = 0;
    if (d != osk_rdir) { fire = d != 0; osk_rdir = d; osk_rnext = now + 380; }
    else if (d && now >= osk_rnext) { fire = 1; osk_rnext = now + 90; }
    if (fire) { if (d == 1) osk_nav(0, -1); else if (d == 2) osk_nav(0, 1); else if (d == 3) osk_nav(-1, 0); else osk_nav(1, 0); }
}
/* where the keyboard sits on a screen ow x oh when fully open (also used for hit testing, before it has been drawn) */
static void osk_layout(int ow, int oh, float *unit, float *pad, float *head, float *pw, float *ph, float *px, float *py) {
    float W = osk_pages[osk_page].w, H = osk_pages[osk_page].h;
    *unit = fminf(ow * 0.94f / W, oh * 0.52f / (H + 1.3f)); *pad = *unit * 0.35f; *head = *unit * 1.0f;
    *pw = W * *unit + *pad * 2; *ph = H * *unit + *head + *pad;
    *px = (ow - *pw) / 2; *py = oh - *ph - *unit * 0.25f;
    float cr = *head * 0.3f, hy = *py + *head * 0.5f;
    osk_close_r = (SDL_FRect){*px + *pw - *pad - cr * 2.6f, hy - cr * 1.3f, cr * 2.6f, cr * 2.6f};
    float tx = *px + *pad + *unit * 4.6f, th = *head * 0.62f;
    for (int p = 0; p < OSK_NPAGE; p++) { float tw = *unit * (p == 2 ? 2.3f : 1.7f); osk_tab_r[p] = (SDL_FRect){tx, hy - th / 2, tw, th}; tx += tw + *unit * 0.2f; }
    osk_x0 = *px + *pad; osk_y0 = *py + *head; osk_unit = *unit;
}
static void osk_draw(int ow, int oh) {
    float unit, pad, head, pw, ph, px, py, e = m_ease(osk_anim);
    osk_layout(ow, oh, &unit, &pad, &head, &pw, &ph, &px, &py);
    float slide = (1 - e) * (ph + unit);
    py += slide; for (int p = 0; p < OSK_NPAGE; p++) osk_tab_r[p].y += slide;
    ui_alpha = e;
    ui_rrect(px, py, pw, ph, unit * 0.35f, 0xEB151B2Du, 0xF20B0F1Cu);
    ui_rrect_line(px, py, pw, ph, unit * 0.35f, 1.5f, 0x38FFFFFFu);
    float hy = py + head * 0.5f;
    ui_textc(F_H2, px + pad, hy, "Amiga keyboard", 0xFFFFFFFFu, ui_fit(F_H2, "Amiga keyboard", unit * 4.3f, 0), 0, 0);
    for (int p = 0; p < OSK_NPAGE; p++) {   /* page tabs */
        SDL_FRect r = osk_tab_r[p]; int on = p == osk_page;
        ui_rrect(r.x, r.y, r.w, r.h, r.h / 2, on ? 0xFF4A86FFu : 0x26FFFFFFu, on ? 0xFF6A4BFFu : 0x18FFFFFFu);
        ui_textc(F_SMALL, r.x + r.w / 2, r.y + r.h / 2, osk_pages[p].name, on ? 0xFFFFFFFFu : 0xFFB8C6DDu, ui_fit(F_SMALL, osk_pages[p].name, r.w * 0.86f, 1.0f), 1, 1.0f);
    }
    float cr = head * 0.3f;
    ui_circle(px + pw - pad - cr * 1.3f, hy, cr, 0x30FFFFFFu);
    ui_line(px + pw - pad - cr * 1.75f, hy - cr * .45f, px + pw - pad - cr * .85f, hy + cr * .45f, 2.5f, 0xFFFFFFFFu);
    ui_line(px + pw - pad - cr * 1.75f, hy + cr * .45f, px + pw - pad - cr * .85f, hy - cr * .45f, 2.5f, 0xFFFFFFFFu);
    if (ui_input == 0) {
        float xr = px + pw - pad - cr * 3.4f;
        m_hint_g(&xr, hy, GL_Y, -1, "Page"); m_hint_g(&xr, hy, GL_X, -1, "Lock"); m_hint_g(&xr, hy, GL_B, -1, "Close"); m_hint_g(&xr, hy, GL_A, -1, "Press");
    }
    float x0 = px + pad, y0 = py + head, g = unit * 0.05f;
    for (int i = 0; i < NOKEYS; i++) {
        const okey_t *k = &OKEYS[i];
        float x = x0 + k->x * unit + g, y = y0 + k->y * unit + g, w = k->w * unit - 2 * g, h = k->h * unit - 2 * g;
        int foc = i == osk_focus && (ui_input == 0 || ui_input == 1), held = (i < OSK_MAXK && osk_pressed[i] > 0) || i == osk_padkey;
        int ms = (k->kind == OK_MOD || k->kind == OK_CAPS) ? osk_mod[(k->code - 0x60) & 7] : 0;
        if (k->kind == OK_KEY && k->norep && gk_mask[k->code & 127] & ((uint64_t)1 << SRC_OSK)) held = 1;
        uint32_t t1 = 0xFF3C465Fu, t2 = 0xFF2A3247u, tc = 0xFFF0F4FAu;
        if (k->y == 0 || k->kind != OK_KEY) { t1 = 0xFF323B52u; t2 = 0xFF232A3Cu; tc = 0xFFC9D4E8u; }
        if (k->kind == OK_MORE || k->kind == OK_CLOSE || k->kind == OK_TYPE) { t1 = 0xFF2B4A7Au; t2 = 0xFF22385Cu; tc = 0xFFDCE8FFu; }
        if (held) { t1 = 0xFF79A6FFu; t2 = 0xFF5478E8u; tc = 0xFFFFFFFFu; }
        else if (foc) { ui_soft(x + w / 2, y + h / 2, unit * 1.3f, 0x603B74FFu, 1); t1 = 0xFF4A86FFu; t2 = 0xFF6A4BFFu; tc = 0xFFFFFFFFu; }
        ui_rrect(x, y, w, h, unit * 0.14f, t1, t2);
        ui_rrect_line(x, y, w, h, unit * 0.14f, foc ? 2.0f : 1.0f, foc ? 0xCCFFFFFFu : 0x22FFFFFFu);
        if (ms) {   /* one-shot: a green outline; locked: filled dot and a thicker outline */
            ui_rrect_line(x, y, w, h, unit * 0.14f, ms == 2 ? 3.5f : 2.0f, 0xFF7FE0A6u);
            ui_circle(x + w - unit * 0.16f, y + unit * 0.16f, unit * (ms == 2 ? 0.08f : 0.05f), 0xFF7FE0A6u);
        }
        int one = k->cap[0] && (!k->cap[1] || (unsigned char)k->cap[0] >= 0xC0);
        int f = one ? F_BODY : F_SMALL; float sc = ui_fit(f, k->cap, w - unit * 0.18f, 0);
        ui_textc(f, x + w / 2, y + h * 0.5f, k->cap, tc, sc, 1, 0);
    }
    ui_alpha = 1;
}
/* the F-key layer while LT is held: a small translucent picture laid out like the buttons themselves */
static void fe_layer_draw(int ow, int oh) {
    float u = fminf(oh / 1080.0f, ow / 1920.0f), W = 940 * u, H = 300 * u, x = (ow - W) / 2, y = oh - H - 40 * u, gh = 30 * u;
    ui_rrect(x, y, W, H, 26 * u, 0xC0151B2Du, 0xC80B0F1Cu);
    ui_rrect_line(x, y, W, H, 26 * u, 1.5f, 0x38FFFFFFu);
    ui_textc(F_SMALL, x + W / 2, y + 26 * u, "Function keys (LT held)", 0xFF9FB0CCu, 1, 1, 1.0f);
    struct { float dx, dy; const char *t; } dp[4] = {{0, -62, "F1"}, {62, 0, "F2"}, {0, 62, "F3"}, {-62, 0, "F4"}};
    float cx = x + 150 * u, cy = y + 170 * u;
    ui_rrect(cx - 20 * u, cy - 80 * u, 40 * u, 160 * u, 8 * u, 0x40FFFFFFu, 0x30FFFFFFu); ui_rrect(cx - 80 * u, cy - 20 * u, 160 * u, 40 * u, 8 * u, 0x40FFFFFFu, 0x30FFFFFFu);
    for (int i = 0; i < 4; i++) { float kx = cx + dp[i].dx * u, ky = cy + dp[i].dy * u; ui_rrect(kx - 28 * u, ky - 20 * u, 56 * u, 40 * u, 10 * u, 0xFF3F7DFFu, 0xFF6A4BFFu); ui_textc(F_SMALL, kx, ky, dp[i].t, 0xFFFFFFFFu, 1, 1, 0); }
    struct { int g; float dx, dy; const char *t; } fb[4] = {{GL_Y, 0, -62, "F8"}, {GL_B, 62, 0, "F6"}, {GL_A, 0, 62, "F5"}, {GL_X, -62, 0, "F7"}};
    cx = x + W - 150 * u;
    for (int i = 0; i < 4; i++) {
        float kx = cx + fb[i].dx * u, ky = cy + fb[i].dy * u;
        ui_rrect(kx - 34 * u, ky - 22 * u, 68 * u, 44 * u, 22 * u, 0x30FFFFFFu, 0x20FFFFFFu);
        ui_measure = 1; float gw = ui_glyph(fb[i].g, 0, 0, gh * 0.8f); ui_measure = 0;
        ui_glyph(fb[i].g, kx - 30 * u, ky, gh * 0.8f); ui_textc(F_SMALL, kx - 30 * u + gw + 4 * u, ky, fb[i].t, 0xFFFFFFFFu, 1, 0, 0);
    }
    float mx = x + W / 2, my = y + 100 * u, xr;
    ui_glyph(GL_LB, mx - 170 * u, my - 30 * u, gh); ui_textc(F_SMALL, mx - 110 * u, my - 30 * u, "F9", 0xFFFFFFFFu, 1, 0, 0);
    ui_glyph(GL_RB, mx + 40 * u, my - 30 * u, gh); ui_textc(F_SMALL, mx + 100 * u, my - 30 * u, "F10", 0xFFFFFFFFu, 1, 0, 0);
    ui_glyph(GL_BACK, mx - 170 * u, my + 60 * u, gh); ui_textc(F_SMALL, mx - 110 * u, my + 60 * u, "Help", 0xFFFFFFFFu, 1, 0, 0);
    ui_glyph(GL_START, mx + 40 * u, my + 60 * u, gh); ui_textc(F_SMALL, mx + 100 * u, my + 60 * u, "Esc", 0xFFFFFFFFu, 1, 0, 0);
    (void)xr;
}
/* the input help (Start + X) */
static void fe_help_draw(int ow, int oh) {
    float u = fminf(oh / 1080.0f, ow / 1920.0f), W = 1180 * u, H = 640 * u, x = (ow - W) / 2, y = (oh - H) / 2, gh = 34 * u;
    ui_rrect(x, y, W, H, 28 * u, 0xEE151B2Du, 0xF20B0F1Cu);
    ui_rrect_line(x, y, W, H, 28 * u, 1.5f, 0x38FFFFFFu);
    ui_text(F_H2, x + 44 * u, y + 30 * u, "Controller", 0xFFFFFFFFu, 1, 0, 0);
    ui_textc(F_SMALL, x + W - 44 * u, y + 52 * u, "Start + X closes this", 0xFF9FB0CCu, 1, 2, 0);
    char la[4][64]; snprintf(la[0], 64, "%s", act_name(S_.map[PB_A])); snprintf(la[1], 64, "%s", act_name(S_.map[PB_B]));
    snprintf(la[2], 64, "%s", act_name(S_.map[PB_X])); snprintf(la[3], 64, "%s", act_name(S_.map[PB_Y]));
    struct { int g1, g2; const char *t; } L[] = {
        {GL_DPAD, GL_LS, "Joystick (port 2)"}, {GL_A, -1, la[0]}, {GL_B, -1, la[1]}, {GL_X, -1, la[2]}, {GL_Y, -1, la[3]},
        {GL_RS, -1, S_.rstick ? "Cursor keys (click: mouse)" : "Mouse (click: cursor keys)"}, {GL_LB, GL_RB, "Left and right mouse buttons"},
    }, R[] = {
        {GL_LT, -1, "Hold: F1-F10 (D-pad F1-F4, A B X Y F5-F8, LB RB F9 F10)"}, {GL_RT, -1, "Hold: precise mouse"}, {GL_BACK, -1, "Amiga keyboard"},
        {GL_START, -1, "Menu (tap)"}, {GL_START, GL_LB, "Screenshot"}, {GL_START, GL_RB, "Record video"}, {GL_START, GL_RT, "Picture scaling"},
    };
    for (int col = 0; col < 2; col++) for (int i = 0; i < 7; i++) {
        int g1 = col ? R[i].g1 : L[i].g1, g2 = col ? R[i].g2 : L[i].g2; const char *t = col ? R[i].t : L[i].t;
        float lx = x + 44 * u + col * W / 2, yc = y + 128 * u + i * 70 * u, w = ui_glyph(g1, lx, yc, gh);
        if (g2 >= 0) { ui_textc(F_SMALL, lx + w + 8 * u, yc, "+", 0xFF9FB0CCu, 1, 0, 0); w += 26 * u; w += ui_glyph(g2, lx + w, yc, gh); }
        ui_textc(F_SMALL, lx + w + 16 * u, yc, t, 0xFFE6ECF7u, ui_fit(F_SMALL, t, W / 2 - w - 80 * u, 0), 0, 0);
    }
}
static void ui_overlay(SDL_Renderer *ren) {   /* drawn after the game picture is scaled, in screen pixels, just before present */
    static Uint32 last;
    Uint32 now = SDL_GetTicks(); float dt = last ? (now - last) / 1000.0f : 0; if (dt > 0.1f) dt = 0.1f; last = now;
    osk_ren = ren;
    osk_anim += (osk_open ? 1.0f : -1.0f) * dt / 0.18f; if (osk_anim < 0) osk_anim = 0; if (osk_anim > 1) osk_anim = 1;
    if (osk_open && osk_anim <= 0) osk_anim = 0.01f;
    int fab = !osk_open && ui_pointer_t && now - ui_pointer_t < 4000;
    if (fe_help_on && now > fe_help_until) fe_help_on = 0;
    int toast = fe_toast_until && now < fe_toast_until;
    if (osk_anim <= 0 && !fab && !reccap_wants_draw() && !fe_layer_shown && !fe_help_on && !toast) return;
    int ow, oh, lw, lh; SDL_GetRendererOutputSize(ren, &ow, &oh); SDL_RenderGetLogicalSize(ren, &lw, &lh);
    float u = fminf(oh / 1080.0f, ow / 1920.0f), su = ui_u, spx = ui_px;
    if (!ui_atlas) ui_init(ren, u);
    if (lw || lh) SDL_RenderSetLogicalSize(ren, 0, 0);
    ui_u = u; ui_px = 1; ui_alpha = 1; ui_ox = ui_oy = 0;
    if (osk_anim > 0) osk_draw(ow, oh);
    if (fe_layer_shown && !osk_open) fe_layer_draw(ow, oh);
    if (fe_help_on) fe_help_draw(ow, oh);
    if (fab) {
        osk_fab_rect(ow, oh);
        float a = now - ui_pointer_t > 3000 ? (4000 - (now - ui_pointer_t)) / 1000.0f : 1, r = osk_fab_r.w / 2, cx = osk_fab_r.x + r, cy = osk_fab_r.y + r;
        ui_alpha = a;
        ui_circle(cx, cy, r, 0xD01B2236u); ui_ring_line(cx, cy, r, 2.0f, 0x66FFFFFFu);
        m_icon(SEC_KEYS, cx, cy, r * 0.95f, 0xFFFFFFFFu);
        ui_alpha = 1;
    }
    if (toast) {
        float a = fe_toast_until - now < 400 ? (fe_toast_until - now) / 400.0f : 1, tw = ui_text_w(F_SMALL, fe_toast_s, 1, 0) + 56 * u;
        ui_alpha = a;
        ui_rrect((ow - tw) / 2, 34 * u, tw, 56 * u, 28 * u, 0xE01B2236u, 0xE0121828u);
        ui_textc(F_SMALL, ow / 2.0f, 62 * u, fe_toast_s, 0xFFFFFFFFu, 1, 1, 0);
        ui_alpha = 1;
    }
    reccap_draw(ow, oh, dt);
    ui_flush();
    ui_u = su; ui_px = spx; ui_alpha = 1;
    if (lw || lh) SDL_RenderSetLogicalSize(ren, lw, lh);
}

/* ------------------------------------------------------------------ controllers -> Amiga
 * Karl's scheme (17 Oct, from the Copilot design he approved): D-pad and left stick = joystick port 2 (stick: on at 0.50, off at
 * 0.38; opposite directions together cancel both); A Fire 1, B Fire 2, X Space, Y Return; right stick = the mouse (dead zone
 * 0.15, an integer acceleration curve, the fractions kept from one guest tick to the next), right-stick click switches it to the
 * cursor keys and back; LB/RB = left/right mouse buttons; RT held = precise mouse; Back = the pop-up keyboard; Start = the menu
 * (never an Amiga key). LT held = the F-key layer: D-pad F1-F4, A B X Y F5-F8, LB RB F9 F10, Start Esc, Back Help (down on press,
 * up on release; LT itself sends nothing). Start + LB screenshot, + RB video, + X input help, + RT picture scaling, + Y / + A
 * quick save / load (not in this build yet). Each input keeps what it was bound to when pressed, so changing layer while a button
 * is held never leaves a key down. Controller 2 = joystick port 1. A controller that goes away lets go of everything it held. */
enum { PI_DU = NPB, PI_DD, PI_DL, PI_DR, PI_RU, PI_RD, PI_RL, PI_RR, PI_RCLICK, PI_N };
#define BOUND_KEY 0x1000
#define BOUND_CHORD 0x2000
typedef struct {
    SDL_GameController *c; SDL_JoystickID id; int fresh, weak;
    int was[PI_N], bound[PI_N], ls[4], rs[4], trig[2];
    int menu_chord, chord_lbrb, chord_fired, sh_wait_t[2], sh_tap[2], sh_chorded[2]; Uint32 chord_t;
} fpad_t;
static fpad_t fp[2];
static int fe_skip_edges;
static int fe_hyst(int v, int was) { return was ? v >= 12452 : v >= 16384; }          /* 0.38 and 0.50 of full deflection */
/* a TV remote that Android reports as a game controller (the Vontar shows "MIC Device Consumer Control", "ir_keypad" and
 * "ir_keypad1", measured 17 Oct): by name, or fewer than 4 axes (a real pad has two sticks). It may be controller 1 while there
 * is no real controller, never controller 2 (that would take the mouse port away) */
static int fe_pad_weak(const char *nm, SDL_GameController *c) {
    if (nm && (strstr(nm, "Consumer Control") || strstr(nm, "emote") || strstr(nm, "keypad") || strstr(nm, "ir_"))) return 1;
    SDL_Joystick *j = c ? SDL_GameControllerGetJoystick(c) : NULL;
    return j && SDL_JoystickNumAxes(j) < 4;
}
static SDL_JoystickID fe_ignored[16]; static int fe_nignored;
static void fe_pad_drop(int n) {
    gk_release_mask(SRC_PADMASK(n));
    if (fp[n].c) SDL_GameControllerClose(fp[n].c);
    memset(&fp[n], 0, sizeof fp[n]);
    fe_nignored = 0;
}
static void fe_pads_sync(void) {
    for (int n = 0; n < 2; n++) if (fp[n].c && !SDL_GameControllerGetAttached(fp[n].c)) { fe_pad_drop(n); fe_toast(n ? "Controller 2 disconnected" : "Controller disconnected"); }
    if (!fp[0].c && fp[1].c) { gk_release_mask(SRC_PADMASK(1)); SDL_GameController *c = fp[1].c; SDL_JoystickID id = fp[1].id; memset(&fp[1], 0, sizeof fp[1]);
                               memset(&fp[0], 0, sizeof fp[0]); fp[0].c = c; fp[0].id = id; fp[0].fresh = 1; }
    for (int j = 0, nj = SDL_NumJoysticks(); j < nj; j++) {
        if (!SDL_IsGameController(j)) continue;
        SDL_JoystickID id = SDL_JoystickGetDeviceInstanceID(j);
        if ((fp[0].c && fp[0].id == id) || (fp[1].c && fp[1].id == id)) continue;
        int skip = 0; for (int k = 0; k < fe_nignored; k++) if (fe_ignored[k] == id) skip = 1;
        if (skip) continue;
        SDL_GameController *c = SDL_GameControllerOpen(j); if (!c) continue;
        int weak = fe_pad_weak(SDL_GameControllerName(c), c), n;
        if (!fp[0].c) n = 0;
        else if (fp[0].weak && !weak) { fe_pad_drop(0); n = 0; }      /* a real controller replaces a TV remote */
        else if (!weak && !fp[1].c) n = 1;
        else {                                                         /* a second remote, or a third controller: not used */
            SDL_GameControllerClose(c);
            if (fe_nignored < 16) fe_ignored[fe_nignored++] = id;    /* not opened again every frame (the list clears when a controller goes) */
            continue;
        }
        memset(&fp[n], 0, sizeof fp[n]); fp[n].c = c; fp[n].id = id; fp[n].fresh = 1; fp[n].weak = weak;
        fprintf(stderr, "fe: controller %d: %s%s\n", n + 1, SDL_GameControllerName(c) ? SDL_GameControllerName(c) : "?", weak ? " (remote)" : "");
        if (n == 1) fe_toast("Controller 2 is joystick port 1");
    }
}
static int fe_layer_key(int i) {
    switch (i) {
    case PB_A: return 0x54; case PB_B: return 0x55; case PB_X: return 0x56; case PB_Y: return 0x57;
    case PB_LB: return 0x58; case PB_RB: return 0x59; case PB_START: return 0x45; case PB_BACK: return 0x5F;
    case PI_DU: return 0x50; case PI_DR: return 0x51; case PI_DD: return 0x52; case PI_DL: return 0x53;
    default: return -1;
    }
}
static int fe_chord(int i) {   /* Start + another button */
    switch (i) {
    case PB_LB: reccap_shot(); break;
    case PB_RB: reccap_button(); break;
    case PB_X: fe_help_on = !fe_help_on; fe_help_until = SDL_GetTicks() + 20000; break;
    case PB_RT: fe_scale_cycle(); break;
    case PB_Y: fe_toast("Quick save is not in this build yet"); break;
    case PB_A: fe_toast("Quick load is not in this build yet"); break;
    default: break;
    }
    return BOUND_CHORD;
}
/* one controller, once per guest frame; out = up, down, left, right, fire, fire 2 of its joystick port */
static int fe_pad_frame(int n, int out[6], int *mlb, int *mrb, int p1joy) {
    fpad_t *P = &fp[n]; SDL_GameController *c = P->c; settings_t *s = &S_;
    static const int sb[NPB] = {SDL_CONTROLLER_BUTTON_A, SDL_CONTROLLER_BUTTON_B, SDL_CONTROLLER_BUTTON_X, SDL_CONTROLLER_BUTTON_Y,
        SDL_CONTROLLER_BUTTON_LEFTSHOULDER, SDL_CONTROLLER_BUTTON_RIGHTSHOULDER, -1, -2, SDL_CONTROLLER_BUTTON_BACK, SDL_CONTROLLER_BUTTON_START};
    int cur[PI_N], ret = 0, prec = 0; Uint32 now = SDL_GetTicks();
    memset(cur, 0, sizeof cur);
    for (int b = 0; b < NPB; b++) {
        if (sb[b] >= 0) cur[b] = pad_btn(c, sb[b]);
        else { int k = sb[b] == -1 ? 0 : 1; P->trig[k] = fe_hyst(SDL_GameControllerGetAxis(c, k ? SDL_CONTROLLER_AXIS_TRIGGERRIGHT : SDL_CONTROLLER_AXIS_TRIGGERLEFT), P->trig[k]); cur[b] = P->trig[k]; }
    }
    cur[PI_DU] = pad_btn(c, SDL_CONTROLLER_BUTTON_DPAD_UP); cur[PI_DD] = pad_btn(c, SDL_CONTROLLER_BUTTON_DPAD_DOWN);
    cur[PI_DL] = pad_btn(c, SDL_CONTROLLER_BUTTON_DPAD_LEFT); cur[PI_DR] = pad_btn(c, SDL_CONTROLLER_BUTTON_DPAD_RIGHT);
    int lx = SDL_GameControllerGetAxis(c, SDL_CONTROLLER_AXIS_LEFTX), ly = SDL_GameControllerGetAxis(c, SDL_CONTROLLER_AXIS_LEFTY);
    P->ls[0] = fe_hyst(-ly, P->ls[0]); P->ls[1] = fe_hyst(ly, P->ls[1]); P->ls[2] = fe_hyst(-lx, P->ls[2]); P->ls[3] = fe_hyst(lx, P->ls[3]);
    int rx = SDL_GameControllerGetAxis(c, SDL_CONTROLLER_AXIS_RIGHTX), ry = SDL_GameControllerGetAxis(c, SDL_CONTROLLER_AXIS_RIGHTY);
    if (n == 0 && s->rstick == 1) {
        P->rs[0] = fe_hyst(-ry, P->rs[0]); P->rs[1] = fe_hyst(ry, P->rs[1]); P->rs[2] = fe_hyst(-rx, P->rs[2]); P->rs[3] = fe_hyst(rx, P->rs[3]);
        for (int k = 0; k < 4; k++) cur[PI_RU + k] = P->rs[k];
    } else memset(P->rs, 0, sizeof P->rs);
    cur[PI_RCLICK] = pad_btn(c, SDL_CONTROLLER_BUTTON_RIGHTSTICK);
    /* the optional LB + RB keyboard hotkey (off by default: LB and RB are the mouse buttons): with it on, a shoulder waits 60 ms
     * to see whether the other joins; a quicker tap still reaches the game as a short press */
    if (s->osk_chord) {
        int lb = cur[PB_LB], rb = cur[PB_RB];
        if (lb && rb) { if (!P->chord_lbrb) { P->chord_lbrb = 1; P->chord_t = now; P->chord_fired = 0; P->sh_chorded[0] = P->sh_chorded[1] = 1; }
                        if (!P->chord_fired && now - P->chord_t >= (Uint32)osk_hold_ms[s->osk_hold & 3]) { P->chord_fired = 1; ret |= 2; } }
        else if (!lb && !rb) P->chord_lbrb = 0;
        for (int k = 0; k < 2; k++) {
            int b = k ? PB_RB : PB_LB, raw = cur[b];
            if (P->chord_lbrb) cur[b] = 0;
            else if (raw) { if (!P->sh_wait_t[k]) P->sh_wait_t[k] = (int)(now | 1); if (!P->was[b] && now - (Uint32)P->sh_wait_t[k] < 60) cur[b] = 0; }
            else { if (P->sh_wait_t[k] && !P->was[b] && !P->sh_chorded[k]) P->sh_tap[k] = 3; P->sh_wait_t[k] = 0; P->sh_chorded[k] = 0; }
            if (P->sh_tap[k] > 0) { cur[b] = 1; P->sh_tap[k]--; }
        }
    }
    int nopress = P->fresh || fe_skip_edges; P->fresh = 0;
    int layer = 0, menu_i = -1;
    for (int i = 0; i < PI_N; i++) if (P->was[i] && cur[i]) { if (P->bound[i] == ACT_FKEYS) layer = 1; if (P->bound[i] == ACT_MENU) menu_i = i; }
    for (int i = 0; i < NPB; i++) if (cur[i] && !P->was[i] && !nopress && s->map[i] == ACT_FKEYS) layer = 1;   /* LT and a button in the same frame */
    for (int i = 0; i < PI_N; i++) {
        int src = SRC_PAD(n, i);
        if (i == PI_RCLICK) {
            if (cur[i] && !P->was[i] && !nopress && n == 0) {
                s->rstick = !s->rstick; settings_save(s);
                fe_toast(s->rstick ? "Right stick: cursor keys" : "Right stick: mouse");
            }
            continue;
        }
        if (cur[i] && !P->was[i]) {
            int bnd = 0, act = i < NPB ? s->map[i] : 0, lk = fe_layer_key(i);
            if (nopress) bnd = 0;
            else if (i < NPB && act == ACT_FKEYS) bnd = ACT_FKEYS;
            else if (layer && lk >= 0) bnd = BOUND_KEY | lk;
            else if (menu_i >= 0 && i < NPB && act != ACT_MENU) { bnd = fe_chord(i); P->menu_chord = 1; }
            else if (i < NPB) bnd = act;
            else if (i >= PI_DU && i <= PI_DR) bnd = ACT_UP + (i - PI_DU);
            else if (i >= PI_RU && i <= PI_RR) { static const int ck[4] = {0x4C, 0x4D, 0x4F, 0x4E}; bnd = BOUND_KEY | ck[i - PI_RU]; }
            if (bnd & BOUND_KEY) gk_key(bnd & 0xff, 1, src);
            else if (bnd >= ACT_KEY0 && bnd < NACTS) gk_key(akeys[bnd - ACT_KEY0].code, 1, src);
            else if (bnd == ACT_SHOT) reccap_shot();
            else if (bnd == ACT_REC) reccap_button();
            else if (bnd == ACT_KEYBOARD) ret |= 2;
            else if (bnd == ACT_MENU) P->menu_chord = 0;
            P->bound[i] = bnd;
        } else if (!cur[i] && P->was[i]) {
            int bnd = P->bound[i];
            if (bnd & BOUND_KEY) gk_key(bnd & 0xff, 0, src);
            else if (bnd >= ACT_KEY0 && bnd < NACTS) gk_key(akeys[bnd - ACT_KEY0].code, 0, src);
            else if (bnd == ACT_MENU && !P->menu_chord) ret |= 1;
            P->bound[i] = 0;
        }
        if (cur[i]) {
            int b = P->bound[i];
            if (b == ACT_FIRE) out[4] = 1; else if (b == ACT_FIRE2) out[5] = 1;
            else if (b >= ACT_UP && b <= ACT_RIGHT) out[b - ACT_UP] = 1;
            else if (b == ACT_MLB) *mlb = 1; else if (b == ACT_MRB) *mrb = 1;
            else if (b == ACT_PRECISE) prec = 1;
            else if (b == ACT_FKEYS) fe_layer_shown = 1;
        }
        P->was[i] = cur[i];
    }
    for (int k = 0; k < 4; k++) if (P->ls[k]) out[k] = 1;
    if (n == 0 && s->rstick == 0 && !p1joy) {   /* the right stick moves the mouse */
        unsigned long long m2 = (unsigned long long)((long long)rx * rx + (long long)ry * ry); int mag = (int)fe_isqrt(m2);
        const int DZ = 4915;                                                           /* 0.15 */
        if (mag > DZ) {
            int t = (int)((long long)(mag - DZ) * 1024 / (32767 - DZ)); if (t > 1024) t = 1024;
            int sp = (t * 128 >> 10) + (int)(((long long)t * t * 1536) >> 20);         /* 1/256 counts per guest tick: up to 7 counts */
            if (prec) sp /= 4;
            fe_mouse_q8((int)((long long)sp * rx / mag), (int)((long long)sp * ry / mag));
        }
    }
    return ret;
}
/* once per guest frame, before it runs: returns 1 = open the options menu, 2 = toggle the on-screen keyboard */
/* test aid (Cowork 17 Oct): show the F-key layer or the input help for the first 20 s of play without a controller, to check
 * their drawing on a device with no pad: AMIGO_FE_DEMO=layer|help (Windows, Linux), or a file fe_demo.txt holding "layer" or
 * "help" in the app's external files folder (Android). Nothing happens without them. */
static Uint32 fe_demo_layer_until;
static void fe_demo_check(void) {
    char b[64] = "";
    const char *e = getenv("AMIGO_FE_DEMO"); if (e) snprintf(b, sizeof b, "%s", e);
#ifdef __ANDROID__
    const char *ext = SDL_AndroidGetExternalStoragePath();
    if (ext && !b[0]) { char p[512]; snprintf(p, sizeof p, "%s/fe_demo.txt", ext); FILE *f = fopen(p, "r"); if (f) { if (!fgets(b, sizeof b, f)) b[0] = 0; fclose(f); } }
#endif
    Uint32 now = SDL_GetTicks();
    if (strstr(b, "layer")) { fe_demo_layer_until = now + 20000; fprintf(stderr, "fe: demo: F-key layer for 20 s\n"); }
    if (strstr(b, "help")) { fe_help_on = 1; fe_help_until = now + 20000; fprintf(stderr, "fe: demo: input help for 20 s\n"); }
}
static int pad_apply(SDL_GameController *p_main, const settings_t *s_, int kb_joy[5]) {
    (void)p_main; (void)s_;
    static int demo_done; if (!demo_done) { demo_done = 1; fe_demo_check(); }
    fe_pads_sync();
    osk_pad = fp[0].c;
    int ret = 0, j2[6] = {0}, j1[6] = {0}, mlb = 0, mrb = 0, p1joy = fp[1].c != NULL;
    fe_layer_shown = SDL_GetTicks() < fe_demo_layer_until;
    {   /* logcat / stderr: the mouse port as the game sees it, at most twice a second when it changes */
        static Uint32 t; static uint16_t lj; static int ll, lr;
        int rmb = (amigo_potgor_clear & 0x0400) != 0;
        if ((mouse_joy0dat != lj || mouse_lmb != ll || rmb != lr) && SDL_GetTicks() - t > 500) {
            t = SDL_GetTicks(); lj = mouse_joy0dat; ll = mouse_lmb; lr = rmb;
            fprintf(stderr, "fe: port 1 JOY0DAT %04X (x %d y %d) left %d right %d%s\n", lj, lj & 0xff, lj >> 8, ll, lr, p1joy ? " (joystick)" : "");
        }
    }
    if (osk_open) {   /* the keyboard has the controllers: nothing they held reaches the game */
        for (int n = 0; n < 2; n++) if (fp[n].c) { gk_release_mask(SRC_PADMASK(n)); memset(fp[n].bound, 0, sizeof fp[n].bound); fp[n].fresh = 1; }
    } else for (int n = 0; n < 2; n++) if (fp[n].c) ret |= fe_pad_frame(n, n == 0 ? j2 : j1, &mlb, &mrb, p1joy);
    fe_skip_edges = 0;
    int u = j2[0] || kb_joy[0], d = j2[1] || kb_joy[1], l = j2[2] || kb_joy[2], r = j2[3] || kb_joy[3];
    if (u && d) u = d = 0; if (l && r) l = r = 0;                                   /* opposite directions cancel */
    joy_up = u; joy_down = d; joy_left = l; joy_right = r; joy_fire = j2[4] || kb_joy[4]; joy_fire2 = j2[5];
    if (p1joy) {
        int u1 = j1[0], d1 = j1[1], l1 = j1[2], r1 = j1[3];
        if (u1 && d1) u1 = d1 = 0; if (l1 && r1) l1 = r1 = 0;
        int v = fe_joyenc(u1, d1, l1, r1);
        if (v != fe_last_p1joy) { mouse_joy0dat = (uint16_t)v; fe_last_p1joy = v; }
        fe_port1_out(j1[4], j1[5]);
    } else {
        if (fe_last_p1joy >= 0) { mouse_joy0dat = 0; fe_last_p1joy = -1; }
        fe_port1_out(mlb || fe_pcm_l || fe_tap_l > 0, mrb || fe_pcm_r);
    }
    if (ret & 1) { fe_skip_edges = 1; return 1; }
    return ret & 2 ? 2 : 0;
}

/* ------------------------------------------------------------------ PC keyboard -> Amiga raw keys */
static int sdl_to_amiga(SDL_Scancode sc) {
    if (sc >= SDL_SCANCODE_A && sc <= SDL_SCANCODE_Z) {
        static const unsigned char az[26] = {0x20,0x35,0x33,0x22,0x12,0x23,0x24,0x25,0x17,0x26,0x27,0x28,0x37,0x36,0x18,0x19,0x10,0x13,0x21,0x14,0x16,0x34,0x11,0x32,0x15,0x31};
        return az[sc - SDL_SCANCODE_A]; }
    if (sc >= SDL_SCANCODE_1 && sc <= SDL_SCANCODE_0) return 0x01 + (sc - SDL_SCANCODE_1);
    if (sc >= SDL_SCANCODE_F1 && sc <= SDL_SCANCODE_F8) return 0x50 + (sc - SDL_SCANCODE_F1);   /* F9 = options menu, F10-F12 = display, capture */
    switch (sc) {
    case SDL_SCANCODE_SPACE: return 0x40; case SDL_SCANCODE_RETURN: return 0x44; case SDL_SCANCODE_ESCAPE: return 0x45;
    case SDL_SCANCODE_BACKSPACE: return 0x41; case SDL_SCANCODE_TAB: return 0x42; case SDL_SCANCODE_DELETE: return 0x46;
    case SDL_SCANCODE_MINUS: return 0x0B; case SDL_SCANCODE_EQUALS: return 0x0C; case SDL_SCANCODE_BACKSLASH: return 0x0D;
    case SDL_SCANCODE_LEFTBRACKET: return 0x1A; case SDL_SCANCODE_RIGHTBRACKET: return 0x1B; case SDL_SCANCODE_SEMICOLON: return 0x29;
    case SDL_SCANCODE_APOSTROPHE: return 0x2A; case SDL_SCANCODE_COMMA: return 0x38; case SDL_SCANCODE_PERIOD: return 0x39;
    case SDL_SCANCODE_SLASH: return 0x3A; case SDL_SCANCODE_GRAVE: return 0x00; case SDL_SCANCODE_LSHIFT: return 0x60;
    case SDL_SCANCODE_RSHIFT: return 0x61; case SDL_SCANCODE_CAPSLOCK: return 0x62; case SDL_SCANCODE_LALT: return 0x64;
    case SDL_SCANCODE_RALT: return 0x65; case SDL_SCANCODE_LGUI: return 0x66; case SDL_SCANCODE_RGUI: return 0x67;
    case SDL_SCANCODE_HOME: return 0x5F;   /* Help */
    case SDL_SCANCODE_KP_0: return 0x0F; case SDL_SCANCODE_KP_1: return 0x1D; case SDL_SCANCODE_KP_2: return 0x1E;
    case SDL_SCANCODE_KP_3: return 0x1F; case SDL_SCANCODE_KP_4: return 0x2D; case SDL_SCANCODE_KP_5: return 0x2E;
    case SDL_SCANCODE_KP_6: return 0x2F; case SDL_SCANCODE_KP_7: return 0x3D; case SDL_SCANCODE_KP_8: return 0x3E;
    case SDL_SCANCODE_KP_9: return 0x3F; case SDL_SCANCODE_KP_ENTER: return 0x43;
    case SDL_SCANCODE_KP_PERIOD: return 0x3C; case SDL_SCANCODE_KP_MINUS: return 0x4A; case SDL_SCANCODE_KP_PLUS: return 0x5E;
    case SDL_SCANCODE_KP_MULTIPLY: return 0x5D; case SDL_SCANCODE_KP_DIVIDE: return 0x5C;
    default: return -1;
    }
}
