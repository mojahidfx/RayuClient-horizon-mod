# RayuClient

A custom client/mod project for Starling Android, built around Horizon's native mod system.

> **Current target:** Starling Android 0.8.0  
> **Requested device ABI:** `armeabi-v7a` (32-bit ARM)  
> **Status:** project initialized; native-mod compatibility is not yet available for the requested ABI.

## Important compatibility note

Starling's current native-mod documentation says its native-mod build service produces **ARM64 libraries only**. Other APK architectures do not offer native-mod compilation or prebuilt downloads. That means a native `.hnmod` cannot currently be compiled and installed through the official Starling flow on a 32-bit `armeabi-v7a` installation.

We will not label an untested archive as a working mod. The first step is to establish the exact Starling 0.8.0 / Horizon source revision and verify the native hook API against that revision.

## Planned first feature

A small name-tag experiment:

- Preserve the vanilla name-tag text.
- Add a `★ ` prefix only when the hook receives an explicit text value.
- Leave vanilla-generated text untouched when the hook receives `None`.
- Keep the experiment isolated so it can be removed without affecting other client behavior.

## Project status

See [docs/COMPATIBILITY.md](docs/COMPATIBILITY.md) and [ROADMAP.md](ROADMAP.md).

## Upstream references

- [Starling](https://github.com/starling-client/starling)
- [Starling native mods](https://github.com/starling-client/starling/blob/starling/docs/starling-native-mods.md)
- [Horizon native mods](https://github.com/starling-client/starling/blob/starling/docs/native-mods.md)
- [Horizon native mod template](https://github.com/horizon-rust/horizon-native-mod-template)

## Safety

Native mods are native code and run with the game's permissions. Only build and install source you have reviewed and trust.