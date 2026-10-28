#!/bin/bash
# build_apk.sh - build a self-contained Android APK from SDL2 C sources, offline (Ami-Go, 13 Oct 2026).
# Usage: bash /w/kit/amigo/android/build_apk.sh --name "Game Name" --id p00002 --out /w/out/game.apk \
#          [--cflags "-O2 -I/w/src"] [--icon /w/out/icon.png] [--banner /w/out/banner.png] SRC.c [SRC.c ...]
# The game's C must use SDL2 and define main() (SDL renames it SDL_main). Data files the game reads at run time
# go in /w/out/android-assets/ (packed as APK assets; open them with SDL_RWFromFile("name", "rb")).
# Output: arm64-v8a + armeabi-v7a (+ x86_64) libmain.so with SDL2, a launcher entry for phones AND the
# Android TV home screen (LEANBACK_LAUNCHER + banner), game-controller support declared, debug-signed.
set -euo pipefail
NAME="Ami-Go game"; ID="game"; OUT=/w/out/game.apk; CFLAGS="-O2"; ICON=""; BANNER=""; SRCS=()
while [ $# -gt 0 ]; do case "$1" in
  --name) NAME=$2; shift 2;; --id) ID=$2; shift 2;; --out) OUT=$2; shift 2;; --cflags) CFLAGS=$2; shift 2;;
  --icon) ICON=$2; shift 2;; --banner) BANNER=$2; shift 2;; *) SRCS+=("$(readlink -f "$1")"); shift;; esac; done
[ ${#SRCS[@]} -gt 0 ] || { echo "no C sources given"; exit 2; }
PKG="org.amigo.$(echo "$ID" | tr -c 'a-zA-Z0-9\n' '_' | tr 'A-Z' 'a-z')"
SDK=/opt/android; BT=$SDK/build-tools/37.0.0; JAR=$SDK/platforms/android-34/android.jar
B=$(mktemp -d /w/out/apkbuild_XXXX); mkdir -p "$B/jni/src" "$B/java" "$B/classes" "$B/res/mipmap" "$B/res/drawable" "$B/apk"
ABIS="arm64-v8a armeabi-v7a x86_64"
# 1. native code: game sources against the prebuilt SDL2
cat > "$B/jni/Android.mk" <<MK
LOCAL_PATH := \$(call my-dir)
include \$(CLEAR_VARS)
LOCAL_MODULE := SDL2
LOCAL_SRC_FILES := /opt/sdl2-android/\$(TARGET_ARCH_ABI)/libSDL2.so
LOCAL_EXPORT_C_INCLUDES := /opt/SDL2/include
include \$(PREBUILT_SHARED_LIBRARY)
include \$(CLEAR_VARS)
LOCAL_MODULE := main
LOCAL_SRC_FILES := ${SRCS[*]}
LOCAL_CFLAGS := $CFLAGS
LOCAL_SHARED_LIBRARIES := SDL2
LOCAL_LDLIBS := -lGLESv1_CM -lGLESv2 -lOpenSLES -llog -landroid -lm
include \$(BUILD_SHARED_LIBRARY)
MK
printf 'APP_ABI := %s\nAPP_PLATFORM := android-21\nAPP_STL := c++_shared\n' "$ABIS" > "$B/jni/Application.mk"
( cd "$B" && ndk-build -j8 NDK_PROJECT_PATH=. APP_BUILD_SCRIPT=jni/Android.mk NDK_APPLICATION_MK=jni/Application.mk > ndk.log 2>&1 ) \
  || { echo "ndk-build failed:"; grep -E "error|Error" "$B/ndk.log" | head -20; exit 1; }
for a in $ABIS; do mkdir -p "$B/apk/lib/$a"; cp "$B/libs/$a/"*.so "$B/apk/lib/$a/"; done
# the libs in the APK are stripped: keep the unstripped libmain.so of each ABI so the G5 symbol check (symcheck.sh) has symbols to read (15 Oct)
mkdir -p /w/out/apk-unstripped; for a in $ABIS; do [ -f "$B/obj/local/$a/libmain.so" ] && cp "$B/obj/local/$a/libmain.so" "/w/out/apk-unstripped/$a-libmain.so"; done
# 2. Java: SDL's activity plus a one-line subclass
cp -r /opt/SDL2/android-project/app/src/main/java/org "$B/java/"
mkdir -p "$B/java/org/amigo"
cat > "$B/java/org/amigo/GameActivity.java" <<JAVA
package org.amigo;
public class GameActivity extends org.libsdl.app.SDLActivity {
    @Override protected String[] getLibraries() { return new String[] { "c++_shared", "SDL2", "main" }; }
}
JAVA
javac -nowarn --release 11 -classpath "$JAR" -d "$B/classes" $(find "$B/java" -name '*.java') 2> "$B/javac.log" \
  || { echo "javac failed:"; head -20 "$B/javac.log"; exit 1; }
"$BT/d8" --min-api 21 --lib "$JAR" --output "$B/apk" $(find "$B/classes" -name '*.class') || { echo "d8 failed"; exit 1; }
# 3. resources: icon and TV banner (from the given images, or plain generated ones)
if [ -n "$ICON" ]; then convert "$ICON" -resize 192x192^ -gravity center -extent 192x192 "$B/res/mipmap/ic_launcher.png"
# no fonts in the tools image (measured 13 Oct: convert -annotate fails), so the default art is shapes only:
# the Ami-Go rainbow bars on Workbench blue. Pass --icon/--banner (e.g. the reference title frame) for real art.
else convert -size 192x192 xc:'#0055aa' -fill '#e8352f' -draw 'rectangle 36,60 156,72' -fill '#ff8a1f' -draw 'rectangle 36,78 156,90' \
       -fill '#ffd31a' -draw 'rectangle 36,96 156,108' -fill '#3fbf4a' -draw 'rectangle 36,114 156,126' -fill '#2f7fe0' -draw 'rectangle 36,132 156,144' \
       "$B/res/mipmap/ic_launcher.png"; fi
if [ -n "$BANNER" ]; then convert "$BANNER" -resize 320x180^ -gravity center -extent 320x180 "$B/res/drawable/banner.png"
else convert -size 320x180 xc:'#05070d' -fill '#e8352f' -draw 'rectangle 0,150 53,180' -fill '#ff8a1f' -draw 'rectangle 53,150 106,180' \
       -fill '#ffd31a' -draw 'rectangle 106,150 160,180' -fill '#3fbf4a' -draw 'rectangle 160,150 213,180' -fill '#2f7fe0' -draw 'rectangle 213,150 266,180' \
       -fill '#8a4fd0' -draw 'rectangle 266,150 320,180' -fill '#0055aa' -draw 'rectangle 20,20 300,130' "$B/res/drawable/banner.png"; fi
LABEL=$(printf '%s' "$NAME" | sed 's/&/\&amp;/g; s/</\&lt;/g; s/"/\&quot;/g')
cat > "$B/AndroidManifest.xml" <<XML
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
"$BT/aapt2" compile --dir "$B/res" -o "$B/res.zip" > /dev/null
ASSETS=(); [ -d /w/out/android-assets ] && ASSETS=(-A /w/out/android-assets)
"$BT/aapt2" link -o "$B/base.apk" -I "$JAR" --manifest "$B/AndroidManifest.xml" --min-sdk-version 21 --target-sdk-version 34 \
  "${ASSETS[@]}" "$B/res.zip" || { echo "aapt2 link failed"; exit 1; }
( cd "$B/apk" && zip -qr "$B/base.apk" classes.dex lib )
"$BT/zipalign" -f -p 4 "$B/base.apk" "$B/aligned.apk"
mkdir -p "$(dirname "$OUT")"
"$BT/apksigner" sign --ks /opt/android/debug.keystore --ks-pass pass:android --key-pass pass:android --out "$OUT" "$B/aligned.apk"
rm -rf "$B"
echo "APK OK $OUT ($(stat -c %s "$OUT") bytes, package $PKG)"
