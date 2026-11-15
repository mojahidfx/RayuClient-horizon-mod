# RayuClient roadmap

## Milestone 0 — Repository and compatibility

- [x] Create the GitHub project.
- [x] Record the Starling native-mod ABI limitation.
- [ ] Confirm exact Starling 0.8.0 build metadata.
- [ ] Resolve the matching Horizon and horizon-mixins revisions.

## Milestone 1 — Native hook proof of concept

- [ ] Create a source bundle from the official Horizon native-mod template.
- [ ] Verify the name-tag hook against the pinned source.
- [ ] Preserve vanilla name-tag behavior.
- [ ] Build and run the source-level checks.
- [ ] Test on a supported ARM64 Starling build.

## Milestone 2 — RayuClient foundation

- [ ] Add a small configurable client identity feature.
- [ ] Add an in-game settings/configuration approach supported by Horizon hooks.
- [ ] Keep features modular and independently testable.

## Device support note

The currently reported `armeabi-v7a` setup is blocked by the official Starling native-mod build service's ARM64-only output. Revisit this milestone if Starling adds 32-bit native builds or if the target APK changes to ARM64.
