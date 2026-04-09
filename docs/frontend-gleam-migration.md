# Frontend Gleam Migration Plan

## Goals

- Replace the current `TypeScript` DOM-manipulation frontend with `Gleam + Lustre`.
- Eliminate mutable global UI state from `src/main.ts`.
- Keep the Tauri boundary narrow and explicit through JS FFI wrappers.
- Minimize npm dependencies to `vite`, `vite-plugin-gleam`, `@tauri-apps/api`, and `@tauri-apps/cli`.

## Current State

The current frontend mixes three concerns in one place:

- application state
- DOM construction and mutation
- async Tauri command orchestration

The main global states are:

- selected plan id
- loaded plan config
- current year/month
- pending skip flags
- modal interaction callbacks

These are currently spread across `src/main.ts`, `src/ui/config.ts`, `src/ui/calendar.ts`, and `src/ui/modal.ts`.

## Target Architecture

### Core Model

The Lustre root `Model` will own all application state:

- boot status
- selected view
- selected plan id
- plan list
- current config
- current calendar month
- derived monthly shift result
- pending skip map
- modal state
- in-flight request markers
- latest error state

### Message Design

State changes will flow only through `Msg` constructors, grouped by feature:

- app boot and initial loading
- plan selection and creation
- config CRUD actions
- calendar month navigation
- calendar generation and reset
- modal open/close/submit
- async success/failure callbacks

### Effects

All Tauri commands will be triggered from `update` via Lustre effects.

### FFI Boundary

`ffi.js` will be the only JS file that knows about `@tauri-apps/api/core`.

Rules:

- Gleam trusts the runtime shape of Tauri responses.
- No runtime decoders are added in JS.
- JS only adapts argument names to Tauri `invoke`.
- Gleam uses `@external` declarations with concrete result types.

## File Layout

Planned frontend layout:

- `gleam.toml`
- `main.js`
- `ffi.js`
- `src/shift_manager.gleam`
- `src/shift_manager/app.gleam`
- `src/shift_manager/model.gleam`
- `src/shift_manager/api.gleam`
- `src/shift_manager/view/*.gleam`
- `src/shift_manager/types.gleam`

The initial scaffold uses fewer files and will be split once the basic loop is stable.

## Migration Phases

### Phase 1: Toolchain

- add `gleam.toml`
- add `vite-plugin-gleam`
- switch Vite entrypoint to `main.js`
- keep existing `TypeScript` UI files in place temporarily

Exit condition:

- a minimal Lustre app mounts successfully

### Phase 2: App Shell

- implement root `Model`, `Msg`, `init`, `update`, `view`
- port plan list loading
- port plan selection
- port view switching
- port month navigation shell

Exit condition:

- user can open the app, load plans, select a plan, and switch tabs

### Phase 3: Config Domain

- port group list rendering
- port member CRUD
- port weekly rule CRUD
- port rule assignment CRUD
- port calendar initial delta editing
- port generic modal flow

Exit condition:

- config tab no longer depends on `TypeScript` DOM mutation modules

### Phase 4: Calendar Domain

- port monthly shift load
- port pending skip editing
- port generate flow
- port reset future shifts flow

Exit condition:

- calendar tab behavior matches the current app

### Phase 5: Cleanup

- remove `src/main.ts`
- remove `src/api.ts`
- remove `src/ui/*.ts`
- remove `src/types.ts`
- remove obsolete `tsconfig.json` if no longer needed
- verify no old frontend entrypoints remain

Exit condition:

- frontend build is purely `Gleam + Lustre`

## Risk Controls

- Do not switch the HTML entrypoint until the Lustre shell mounts.
- Keep Tauri command names unchanged.
- Migrate one feature area at a time.
- Keep CSS initially to reduce simultaneous risk.
- Use a single FFI module to prevent JS sprawl.

## First Implementation Slice

The first slice will do only this:

- add Gleam project files
- add Lustre bootstrap
- add minimal FFI wrappers for plan loading
- mount a shell view while keeping the old `TypeScript` code available until the shell is ready to replace it
