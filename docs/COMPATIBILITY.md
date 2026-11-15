# Compatibility

## Requested setup

| Item | Value |
|---|---|
| Client | Starling Android 0.8.0 |
| APK ABI reported by user | `armeabi-v7a` |
| Native mod format | `.hnmod` |
| First experiment | Name-tag text hook |

## Verified limitation

The current official Starling documentation states:

- Native mods (`.hnmod`) are built by Starling's native build service.
- The service currently produces ARM64 libraries.
- Other APK architectures do not offer native-mod compilation or prebuilt downloads.

Therefore, an `armeabi-v7a` APK cannot use the current official native-mod build path. This is a platform/build-service limitation, not a problem that can be fixed by changing the mod's Rust target alone.

## What must be checked before producing a package

1. Confirm the exact APK version and ABI from the installed Starling build.
2. Identify the Horizon commit bundled with Starling 0.8.0.
3. Pin the matching `horizon-mixins` revision.
4. Verify the `native_render_name_tag` hook signature and its declared hook API level at that exact Horizon revision.
5. Build and test on a supported ARM64 target before calling the package functional.

## Name-tag hook behavior

The documented hook is:

`native_render_name_tag(entity_id, kind, vanilla_visible, visible, text)`

The hook can override visibility and provide literal replacement text. The experiment must not replace vanilla text when `text` is `None`; that case is left unchanged so the game can produce its normal name.

## Status

No installable `.hnmod` is included yet. A source package will only be added after the version pin, hook declaration, and build are verified.
