#!/bin/bash
# native_package.sh - the Android APK (and desktop programs) of one native game (native stage 1, 31 Oct 2026,
# docs/G5-NATIVE.md "Added by native stage 1"). Runs on Ryzen inside the amigo-tools image, offline, after the game's
# asset pack has been built (outside the repo) and packed with napack.
#
#   tools/native_package.sh --game g296 --pack PACK.nap --name NAME --out OUTDIR [--android] [--linux] [--windows]
#                           [--package ANDROID.PACKAGE] [--dry-run]
#
#   --android: OUTDIR/<game>-native.apk: arm64-v8a and armeabi-v7a, the Rust libmain.so (rust/games/<game>/android)
#              beside the prebuilt libSDL2.so and libc++_shared.so, the pack as the asset gamepack.bin, SDL's Java
#              activity, phone and TV launcher entries, debug-signed. Package org.amigo.g5.<id>.nativegame unless
#              --package is given (a different package from the translated G5 build, so both can be installed).
#   --linux / --windows: OUTDIR/<game>-native (x86_64 Linux) / <game>-native.exe, SDL2 and the pack built in.
# The APK stages follow tools/g5_package.sh (Java, manifest, resources, signing); only the native library differs.
# Exit: 0 built, 2 bad arguments, 1 a build step failed.
set -euo pipefail
GAME=""; PACK=""; NAME=""; OUT=""; ANDROID=0; LINUX=0; WINDOWS=0; PKG=""; DRY=0
usage() { echo "usage: native_package.sh --game ID --pack PACK.nap --name NAME --out OUTDIR [--android] [--linux] [--windows] [--package P] [--dry-run]" >&2; exit 2; }
while [ $# -gt 0 ]; do
  case "$1" in --game|--pack|--name|--out|--package) [ $# -ge 2 ] || usage;; esac
  case "$1" in
  --game) GAME=$2; shift 2;; --pack) PACK=$2; shift 2;; --name) NAME=$2; shift 2;; --out) OUT=$2; shift 2;;
  --package) PKG=$2; shift 2;; --android) ANDROID=1; shift;; --linux) LINUX=1; shift;; --windows) WINDOWS=1; shift;;
  --dry-run) DRY=1; shift;; *) echo "native_package.sh: unknown argument $1" >&2; usage;; esac
done
[ -n "$GAME" ] && [ -n "$PACK" ] && [ -n "$NAME" ] && [ -n "$OUT" ] || usage
[[ "$GAME" =~ ^g[0-9]+$ ]] || { echo "native_package.sh: --game is the crate name under rust/games (g296)" >&2; exit 2; }
HERE=$(cd "$(dirname "$0")/.." && pwd)
CRATE=$HERE/rust/games/$GAME
[ -d "$CRATE/android" ] || { echo "native_package.sh: no $CRATE/android" >&2; exit 2; }
PACK=$(readlink -f "$PACK"); [ -f "$PACK" ] || { echo "native_package.sh: no pack $PACK" >&2; exit 2; }
head -c 4 "$PACK" | grep -q NAP1 || { echo "native_package.sh: $PACK is not a binary asset pack (napack pack DIR OUT)" >&2; exit 2; }
mkdir -p "$OUT/unstripped"; OUT=$(readlink -f "$OUT")
ID=${GAME#g}; ID=$(printf 'g%05d' "$ID")
PKG=${PKG:-org.amigo.g5.$ID.nativegame}
export NATIVE_REV=${NATIVE_REV:-$(git -C "$HERE" rev-parse --short HEAD 2>/dev/null || echo unknown)}
export NATIVE_BUILD_DATE=${NATIVE_BUILD_DATE:-$(date -u +%Y-%m-%d)}
export CMAKE_POLICY_VERSION_MINIMUM=3.5 CFLAGS=${CFLAGS:--std=gnu17}
WS=$HERE/rust/native_engine
run() { if [ $DRY = 1 ]; then echo "+ $*"; else "$@"; fi; }

if [ $LINUX = 1 ]; then
  ( cd "$WS" && run env NATIVE_PACK="$PACK" cargo build --release --offline --locked -p "$GAME" --bin "$GAME" )
  run cp "$WS/target/release/$GAME" "$OUT/unstripped/$GAME-native"
  run strip -o "$OUT/$GAME-native" "$OUT/unstripped/$GAME-native"
  echo "LINUX $OUT/$GAME-native"
fi
if [ $WINDOWS = 1 ]; then
  T=x86_64-pc-windows-gnu
  ( cd "$WS" && run env NATIVE_PACK="$PACK" cargo build --release --offline --locked --target $T -p "$GAME" --bin "$GAME" )
  run cp "$WS/target/$T/release/$GAME.exe" "$OUT/unstripped/$GAME-native.exe"
  run x86_64-w64-mingw32-strip -o "$OUT/$GAME-native.exe" "$OUT/unstripped/$GAME-native.exe"
  echo "EXE $OUT/$GAME-native.exe"
fi
if [ $ANDROID = 1 ]; then
  SDK=${ANDROID_HOME:-/opt/android}; BT=$SDK/build-tools/37.0.0; JAR=$SDK/platforms/android-34/android.jar
  NDK=${ANDROID_NDK_HOME:-$SDK/ndk/26.3.11579264}; TC=$NDK/toolchains/llvm/prebuilt/linux-x86_64
  # the work directory of an earlier run is moved aside, not deleted (Karl's rule: nothing is deleted)
  W=$OUT/work; if [ -e "$W" ]; then mkdir -p "$OUT/_to_delete"; mv "$W" "$OUT/_to_delete/work-$(date +%Y%m%d-%H%M%S)"; fi; mkdir -p "$W/java/org/amigo" "$W/classes" "$W/res/mipmap" "$W/res/drawable" "$W/apk" "$W/assets"
  for a in arm64-v8a armeabi-v7a; do
    case $a in
      arm64-v8a)   T=aarch64-linux-android;   CC=aarch64-linux-android21-clang;   LIB=aarch64-linux-android;;
      armeabi-v7a) T=armv7-linux-androideabi; CC=armv7a-linux-androideabi21-clang; LIB=arm-linux-androideabi;;
    esac
    TU=$(echo "$T" | tr 'a-z-' 'A-Z_')
    ( cd "$WS" && run env -u NATIVE_PACK "CARGO_TARGET_${TU}_LINKER=$TC/bin/$CC" \
        "CARGO_TARGET_${TU}_RUSTFLAGS=-L native=/opt/sdl2-android/$a" \
        cargo build --release --offline --locked --target "$T" -p "${GAME}_android" )
    mkdir -p "$W/apk/lib/$a"
    run cp "$WS/target/$T/release/libmain.so" "$OUT/unstripped/$a-libmain.so"
    run "$TC/bin/llvm-strip" -o "$W/apk/lib/$a/libmain.so" "$OUT/unstripped/$a-libmain.so"
    run cp "/opt/sdl2-android/$a/libSDL2.so" "$TC/sysroot/usr/lib/$LIB/libc++_shared.so" "$W/apk/lib/$a/"
  done
  run cp "$PACK" "$W/assets/gamepack.bin"
  run cp -r /opt/SDL2/android-project/app/src/main/java/org "$W/java/"
  cat > "$W/java/org/amigo/GameActivity.java" <<JAVA
package org.amigo;
public class GameActivity extends org.libsdl.app.SDLActivity {
    @Override protected String[] getLibraries() { return new String[] { "c++_shared", "SDL2", "main" }; }
}
JAVA
  if [ $DRY = 1 ]; then echo "+ javac ... d8 ..."
  else
    javac -nowarn --release 11 -classpath "$JAR" -d "$W/classes" $(find "$W/java" -name '*.java') 2> "$W/javac.log" \
      || { echo "javac failed:"; head -20 "$W/javac.log"; exit 1; }
    "$BT/d8" --min-api 21 --lib "$JAR" --output "$W/apk" $(find "$W/classes" -name '*.class') || { echo "d8 failed"; exit 1; }
  fi
  # the icon and TV banner: plain shapes on Workbench blue with a neon "N" bar, so the native build is told apart
  run convert -size 192x192 xc:'#0055aa' -fill '#00e5ff' -draw 'rectangle 36,84 156,96' -fill '#ff2bd6' -draw 'rectangle 36,108 156,120' "$W/res/mipmap/ic_launcher.png"
  run convert -size 320x180 xc:'#05070d' -fill '#0055aa' -draw 'rectangle 20,20 300,130' -fill '#ff2bd6' -draw 'rectangle 0,150 320,180' "$W/res/drawable/banner.png"
  LABEL=$(printf '%s' "$NAME" | sed 's/&/\&amp;/g; s/</\&lt;/g; s/"/\&quot;/g')
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
  run "$BT/aapt2" compile --dir "$W/res" -o "$W/res.zip"
  run "$BT/aapt2" link -o "$W/base.apk" -I "$JAR" --manifest "$W/AndroidManifest.xml" --min-sdk-version 21 \
    --target-sdk-version 34 -A "$W/assets" "$W/res.zip"
  if [ $DRY = 1 ]; then echo "+ zip classes.dex lib"; else ( cd "$W/apk" && zip -qr "$W/base.apk" classes.dex lib ); fi
  run "$BT/zipalign" -f -p 4 "$W/base.apk" "$W/aligned.apk"
  # the debug keystore of the image, as tools/g5_package.sh signs
  run "$BT/apksigner" sign --ks "$SDK/debug.keystore" --ks-pass pass:android --key-pass pass:android \
    --out "$OUT/$GAME-native.apk" "$W/aligned.apk"
  echo "APK $OUT/$GAME-native.apk (package $PKG)"
fi
