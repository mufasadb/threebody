# Three Body Problem

A real-time gravitational N-body simulation (starting with 3 bodies) built in Rust using the nannou creative coding framework.

## Goal

A visually compelling simulation of chaotic orbital mechanics rendered natively at speed — inspired by the Liu Cixin novel. Bodies exert gravity on each other, producing the unpredictable, sensitive-to-initial-conditions behaviour that makes the three-body problem famously unsolvable analytically.

## Stack

- **Rust** (stable) — chosen for performance; physics runs fast enough to simulate hours per frame
- **nannou 0.19** — creative coding framework for the windowed display
- **cargo** — build and test

## Current state

`Vec3` struct is complete — f64 components, Add/Sub/scalar Mul/magnitude/normalise, 9 tests passing.

**Next:** `Body` struct (mass, radius, position, velocity) and gravitational force calculation between pairs.

## Key decisions

- `f64` throughout — physics precision matters at these scales
- 3D vectors, rendered in 2D by projecting onto the XY plane
- Softening term on gravity to prevent force singularities at close approach
- Leapfrog integration preferred over Euler — conserves energy over long runs

## Collaboration

Beachy is working through this step by step. Discuss before implementing — don't run ahead.
