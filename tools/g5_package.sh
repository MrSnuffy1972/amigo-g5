#!/bin/bash
# g5_package.sh - the G5 playable builds of one game (M9, 29 Oct 2026, docs/G5-PLAYABLE.md "The packaging contract").
# Usage: tools/g5_package.sh --pack PACK.bin --name NAME --id ID --out OUTDIR [--android] [--windows]
#          [--icon PNG] [--banner PNG] [--skip-symcheck] [--dry-run]
# Runs on Ryzen inside the amigo-tools image, offline, after `cargo fetch` in rust/amigo_g5_play. The game's code is
# compiled in from AMIGO_G5_GAME_RS (m68k2c.py --emit rust), as for amigo-g5-boot; without it the build boots to
# the first instruction and stops.
#   --windows: OUTDIR/ID.exe, x86_64-pc-windows-gnu, SDL2 built in, the pack built in (AMIGO_G5_PACK), no console
#              window; OUTDIR/unstripped/ID.exe keeps the symbols.
#   --android: OUTDIR/ID.apk, package org.amigo.g5.ID, arm64-v8a and armeabi-v7a (x86_64 too with
#              G5_ANDROID_X86_64=1), the Rust libmain.so beside the prebuilt libSDL2.so and libc++_shared.so, the pack
#              as the asset gamepack.bin, SDL's Java activity, phone and TV launcher entries, debug-signed;
#              OUTDIR/unstripped/ABI-libmain.so keeps the symbols.
# Every built binary goes through Ryzen's G5 symbol check (G5_SYMCHECK=path to g5_symcheck.sh, called with the
# unstripped binary); --skip-symcheck builds without it and says so. --dry-run checks the arguments and the pack,
# writes the APK's manifest and Java into OUTDIR/work and prints each build command instead of running it (the tests).
# The APK stages follow kit/amigo/android/build_apk.sh (Java, manifest, resources, signing) with only the native step
# replaced. They are copied rather than called because build_apk.sh needs C sources, and changing a kit/ file moves
# Ryzen's kit_rev, which rebuilds every G2 to G4 workspace (docs/G5-RYZEN-CONTRACT.md section 4).
# Exit: 0 built, 2 bad arguments or pack, 4 a symbol check failed or could not run, 1 a build step failed.
set -euo pipefail
PACK=""; NAME=""; ID=""; OUT=""; ANDROID=0; WINDOWS=0; ICON=""; BANNER=""; SKIPSYM=0; DRY=0
usage() { echo "usage: g5_package.sh --pack PACK.bin --name NAME --id ID --out OUTDIR [--android] [--windows] [--icon PNG] [--banner PNG] [--skip-symcheck] [--dry-run]" >&2; exit 2; }
while [ $# -gt 0 ]; do
  case "$1" in --pack|--name|--id|--out|--icon|--banner) [ $# -ge 2 ] || usage;; esac
  case "$1" in
  --pack) PACK=${2:-}; shift 2;; --name) NAME=${2:-}; shift 2;; --id) ID=${2:-}; shift 2;; --out) OUT=${2:-}; shift 2;;
  --icon) ICON=${2:-}; shift 2;; --banner) BANNER=${2:-}; shift 2;;
  --android) ANDROID=1; shift;; --windows) WINDOWS=1; shift;; --skip-symcheck) SKIPSYM=1; shift;; --dry-run) DRY=1; shift;;
  *) echo "g5_package.sh: unknown argument $1" >&2; usage;; esac
done
[ -n "$PACK" ] && [ -n "$NAME" ] && [ -n "$ID" ] && [ -n "$OUT" ] || usage
[ $ANDROID = 1 ] || [ $WINDOWS = 1 ] || { echo "g5_package.sh: give --android, --windows or both" >&2; exit 2; }
# the id is part of a Java package name and of file names: lower-case letters, digits and _, not starting with a digit
[[ "$ID" =~ ^[a-z][a-z0-9_]*$ ]] || { echo "g5_package.sh: --id must match [a-z][a-z0-9_]* (got '$ID')" >&2; exit 2; }
HERE=$(cd "$(dirname "$0")/.." && pwd)
PACK=$(readlink -f "$PACK")
python3 "$HERE/tools/g5_pack.py" list "$PACK" > /dev/null || { echo "g5_package.sh: $PACK is not a good game pack" >&2; exit 2; }
for f in "$ICON" "$BANNER"; do [ -z "$f" ] || [ -f "$f" ] || { echo "g5_package.sh: no file $f" >&2; exit 2; }; done
mkdir -p "$OUT/unstripped"; OUT=$(readlink -f "$OUT")
CRATE=$HERE/rust/amigo_g5_play
PKG="org.amigo.g5.$ID"
# the pre-boot's build line (docs/G5-PLAYABLE.md "Added by M8")
export AMIGO_G5_REV=${AMIGO_G5_REV:-$(git -C "$HERE" rev-parse --short HEAD 2>/dev/null || echo unknown)}
export AMIGO_G5_BUILD_DATE=${AMIGO_G5_BUILD_DATE:-$(date -u +%Y-%m-%d)}
# Ryzen measured these for cmake 4 and gcc 15 building SDL 2.26 (docs/G5-PLAYABLE.md); harmless on older tools
export CMAKE_POLICY_VERSION_MINIMUM=3.5 CFLAGS=${CFLAGS:--std=gnu17}
[ -n "${AMIGO_G5_GAME_RS:-}" ] || echo "g5_package.sh: WARNING AMIGO_G5_GAME_RS is not set: the build has no game code" >&2
run() { if [ $DRY = 1 ]; then echo "+ $*"; else "$@"; fi; }
symcheck() {  # symcheck BINARY
  if [ $SKIPSYM = 1 ]; then echo "SYMCHECK SKIPPED $1"; return; fi
  if [ $DRY = 1 ]; then echo "+ \$G5_SYMCHECK $1"; return; fi
  [ -n "${G5_SYMCHECK:-}" ] && [ -x "$G5_SYMCHECK" ] || { echo "g5_package.sh: set G5_SYMCHECK to g5_symcheck.sh (or --skip-symcheck)" >&2; exit 4; }
  "$G5_SYMCHECK" "$1" || { echo "g5_package.sh: SYMCHECK FAIL $1" >&2; exit 4; }
  echo "SYMCHECK PASS $1"
}

if [ $WINDOWS = 1 ]; then
  T=x86_64-pc-windows-gnu
  # with the pack built in, build.rs sets g5_embedded, which makes the exe a GUI program (main.rs)
  ( cd "$CRATE" && run env AMIGO_G5_PACK="$PACK" cargo build --release --offline --locked --target $T --bin amigo-g5-play )
  EXE=$CRATE/target/$T/release/amigo-g5-play.exe
  run cp "$EXE" "$OUT/unstripped/$ID.exe"
  symcheck "$OUT/unstripped/$ID.exe"
  run x86_64-w64-mingw32-strip -o "$OUT/$ID.exe" "$OUT/unstripped/$ID.exe"
  echo "EXE $OUT/$ID.exe"
fi

if [ $ANDROID = 1 ]; then
  SDK=${ANDROID_HOME:-/opt/android}; BT=$SDK/build-tools/37.0.0; JAR=$SDK/platforms/android-34/android.jar
  NDK=${ANDROID_NDK_HOME:-$SDK/ndk/26.3.11579264}; TC=$NDK/toolchains/llvm/prebuilt/linux-x86_64
  W=$OUT/work; rm -rf "$W"; mkdir -p "$W/java/org/amigo" "$W/classes" "$W/res/mipmap" "$W/res/drawable" "$W/apk" "$W/assets"
  ABIS="arm64-v8a armeabi-v7a"; [ "${G5_ANDROID_X86_64:-0}" = 1 ] && ABIS="$ABIS x86_64"
  # 1. native code: libmain.so from Rust (amigo_g5_android, SDL2 not built in) against the prebuilt libSDL2.so
  for a in $ABIS; do
    case $a in
      arm64-v8a)   T=aarch64-linux-android;   CC=aarch64-linux-android21-clang;   LIB=aarch64-linux-android;;
      armeabi-v7a) T=armv7-linux-androideabi; CC=armv7a-linux-androideabi21-clang; LIB=arm-linux-androideabi;;
      x86_64)      T=x86_64-linux-android;    CC=x86_64-linux-android21-clang;    LIB=x86_64-linux-android;;
    esac
    TU=$(echo "$T" | tr 'a-z-' 'A-Z_')
    # no AMIGO_G5_PACK: the APK carries the pack as an asset, so it is not in the library twice
    ( cd "$CRATE" && run env -u AMIGO_G5_PACK "CARGO_TARGET_${TU}_LINKER=$TC/bin/$CC" \
        "CARGO_TARGET_${TU}_RUSTFLAGS=-L native=/opt/sdl2-android/$a" \
        cargo build --release --offline --locked --target "$T" -p amigo_g5_android )
    mkdir -p "$W/apk/lib/$a"
    run cp "$CRATE/target/$T/release/libmain.so" "$OUT/unstripped/$a-libmain.so"
    symcheck "$OUT/unstripped/$a-libmain.so"
    run "$TC/bin/llvm-strip" -o "$W/apk/lib/$a/libmain.so" "$OUT/unstripped/$a-libmain.so"
    run cp "/opt/sdl2-android/$a/libSDL2.so" "$TC/sysroot/usr/lib/$LIB/libc++_shared.so" "$W/apk/lib/$a/"
  done
  run cp "$PACK" "$W/assets/gamepack.bin"
  # 2. Java: SDL's activity plus a one-line subclass (as build_apk.sh)
  run cp -r /opt/SDL2/android-project/app/src/main/java/org "$W/java/"
  cat > "$W/java/org/amigo/GameActivity.java" <<JAVA
package org.amigo;
public class GameActivity extends org.libsdl.app.SDLActivity {
    @Override protected String[] getLibraries() { return new String[] { "c++_shared", "SDL2", "main" }; }
}
JAVA
  if [ $DRY = 1 ]; then echo "+ javac ... d8 ... (the Java under $W/java)"
  else
    javac -nowarn --release 11 -classpath "$JAR" -d "$W/classes" $(find "$W/java" -name '*.java') 2> "$W/javac.log" \
      || { echo "javac failed:"; head -20 "$W/javac.log"; exit 1; }
    "$BT/d8" --min-api 21 --lib "$JAR" --output "$W/apk" $(find "$W/classes" -name '*.class') || { echo "d8 failed"; exit 1; }
  fi
  # 3. resources: icon and TV banner, from the given images or plain shapes on Workbench blue (as build_apk.sh)
  if [ -n "$ICON" ]; then run convert "$ICON" -resize 192x192^ -gravity center -extent 192x192 "$W/res/mipmap/ic_launcher.png"
  else run convert -size 192x192 xc:'#0055aa' -fill '#00e5ff' -draw 'rectangle 36,84 156,96' -fill '#ff8800' -draw 'rectangle 36,108 156,120' \
         "$W/res/mipmap/ic_launcher.png"; fi
  if [ -n "$BANNER" ]; then run convert "$BANNER" -resize 320x180^ -gravity center -extent 320x180 "$W/res/drawable/banner.png"
  else run convert -size 320x180 xc:'#05070d' -fill '#0055aa' -draw 'rectangle 20,20 300,130' -fill '#00e5ff' -draw 'rectangle 0,150 160,180' \
         -fill '#ff2bd6' -draw 'rectangle 160,150 320,180' "$W/res/drawable/banner.png"; fi
  LABEL=$(printf '%s' "$NAME" | sed 's/&/\&amp;/g; s/</\&lt;/g; s/"/\&quot;/g')
  # landscape, touchscreen not required, gamepad declared, phone and TV launchers (docs/G5-PLAYABLE.md)
  cat > "$W/AndroidManifest.xml" <<XML
<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android" package="$PKG" android:versionCode="1" android:versionName="1.0">
  <uses-feature android:glEsVersion="0x00020000" />
  <uses-feature android:name="android.hardware.touchscreen" android:required="false" />
  <uses-feature android:name="android.hardware.gamepad" android:required="false" />
  <uses-feature android:name="android.software.leanback" android:required="false" />
  <application android:label="$LABEL" android:icon="@mipmap/ic_launcher" android:banner="@drawable/banner"
      android:allowBackup="true" android:hardwareAccelerated="true" android:isGame="true" android:extractNativeLibs="true">
    <activity android:name="org.amigo.GameActivity" android:exported="true" android:screenOrientation="sensorLandscape"
        android:configChanges="layoutDirection|locale|orientation|uiMode|screenLayout|screenSize|smallestScreenSize|keyboard|keyboardHidden|navigation"
        android:launchMode="singleInstance" android:preferMinimalPostProcessing="true">
      <intent-filter><action android:name="android.intent.action.MAIN" /><category android:name="android.intent.category.LAUNCHER" /></intent-filter>
      <intent-filter><action android:name="android.intent.action.MAIN" /><category android:name="android.intent.category.LEANBACK_LAUNCHER" /></intent-filter>
    </activity>
  </application>
</manifest>
XML
  # 4. link, align and sign (as build_apk.sh)
  run "$BT/aapt2" compile --dir "$W/res" -o "$W/res.zip"
  run "$BT/aapt2" link -o "$W/base.apk" -I "$JAR" --manifest "$W/AndroidManifest.xml" --min-sdk-version 21 \
    --target-sdk-version 34 -A "$W/assets" "$W/res.zip"
  if [ $DRY = 1 ]; then echo "+ zip classes.dex lib into $W/base.apk"; else ( cd "$W/apk" && zip -qr "$W/base.apk" classes.dex lib ); fi
  run "$BT/zipalign" -f -p 4 "$W/base.apk" "$W/aligned.apk"
  run "$BT/apksigner" sign --ks "$SDK/debug.keystore" --ks-pass pass:android --key-pass pass:android \
    --out "$OUT/$ID.apk" "$W/aligned.apk"
  [ $DRY = 1 ] || rm -rf "$W"
  echo "APK $OUT/$ID.apk (package $PKG, ABIs $ABIS)"
  # kit/amigo/g6check.py (only system libraries plus SDL2 and libc++, SDL_main exported) is Ryzen's: run it there
fi
