#!/usr/bin/env python3
"""g5check.py - G5 'Android' check (13 Oct 2026). Static checks of /w/out/game.apk with the SDK's own tools.

Passes when: the signature verifies (apksigner); the manifest has a LAUNCHER and a LEANBACK_LAUNCHER entry and a
TV banner (aapt2 dump badging); libmain.so and libSDL2.so are present for arm64-v8a and armeabi-v7a; libmain.so
needs only Android system libraries plus the bundled SDL2/libc++ (readelf); it was built from the same
sources as the Windows build (a game symbol, SDL_main, is exported).
NOT checked yet: running it. Ryzen has no /dev/kvm (no emulator acceleration), so a run test needs either KVM
enabled or a real device; the output says so plainly rather than pretending.
"""
import os, re, subprocess, sys, zipfile
BT = "/opt/android/build-tools/37.0.0"
APK = sys.argv[1] if len(sys.argv) > 1 else "out/game.apk"
ALLOWED = {"libSDL2.so", "libc++_shared.so", "liblog.so", "libandroid.so", "libGLESv1_CM.so", "libGLESv2.so",
           "libGLESv3.so", "libEGL.so", "libOpenSLES.so", "libaaudio.so", "libc.so", "libm.so", "libdl.so", "libz.so"}
errs = []
if not os.path.exists(APK):
    print("G5 FAIL: %s not built" % APK); sys.exit(1)
r = subprocess.run([BT + "/apksigner", "verify", APK], capture_output=True, text=True)
if r.returncode != 0:
    errs.append("signature does not verify: " + (r.stderr or r.stdout)[-200:])
b = subprocess.run([BT + "/aapt2", "dump", "badging", APK], capture_output=True, text=True).stdout
if "launchable-activity" not in b: errs.append("no LAUNCHER activity")
if "leanback-launchable-activity" not in b: errs.append("no LEANBACK_LAUNCHER activity (not on the Android TV home screen)")
if "banner=" not in b and "application-banner" not in b:
    xml = subprocess.run([BT + "/aapt2", "dump", "xmltree", APK, "--file", "AndroidManifest.xml"], capture_output=True, text=True).stdout
    if "banner" not in xml: errs.append("no TV banner")
names = zipfile.ZipFile(APK).namelist()
for abi in ("arm64-v8a", "armeabi-v7a"):
    for lib in ("libmain.so", "libSDL2.so"):
        if "lib/%s/%s" % (abi, lib) not in names:
            errs.append("missing lib/%s/%s" % (abi, lib))
if "lib/arm64-v8a/libmain.so" in names:
    tmp = "/tmp/g5_libmain.so"
    open(tmp, "wb").write(zipfile.ZipFile(APK).read("lib/arm64-v8a/libmain.so"))
    d = subprocess.run(["readelf", "-dW", tmp], capture_output=True, text=True).stdout
    need = set(re.findall(r"Shared library: \[([^\]]+)\]", d))
    bad = sorted(need - ALLOWED)
    if bad: errs.append("libmain.so needs libraries not on Android or not bundled: " + ", ".join(bad))
    syms = subprocess.run(["readelf", "-sW", "--dyn-syms", tmp], capture_output=True, text=True).stdout
    if "SDL_main" not in syms: errs.append("libmain.so exports no SDL_main (the game's main)")
    size = len(zipfile.ZipFile(APK).read("lib/arm64-v8a/libmain.so"))
    if size < 20000: errs.append("libmain.so is only %d bytes: not a whole game" % size)
pkg = re.search(r"package: name='([^']+)'", b)
if errs:
    print("G5 FAIL: " + "; ".join(errs)); sys.exit(1)
print("G5 OK (static): %s, signed, phone and TV launcher entries, arm64-v8a + armeabi-v7a, %d bytes. Not run on a "
      "device or emulator (no KVM on Ryzen)." % (pkg.group(1) if pkg else "?", os.path.getsize(APK)))
