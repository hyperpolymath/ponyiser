// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//
// ponyiser library API.
//
// Provides programmatic access to the ponyiser analysis and code generation
// engine. Use this crate as a library to embed Pony capability analysis
// into your own tooling.

pub mod abi;
pub mod codegen;
pub mod manifest;

pub use abi::{
    check_subtype, is_subtype, validate_sendability, Actor, Behaviour, BehaviourParam,
    CapabilityViolation, Field, RefCapability, SubtypingResult,
};
pub use codegen::capability::{analyse, infer_capability, AnalysisResult};
pub use manifest::{load_manifest, parse_manifest, validate, Manifest};

/// Convenience: load, validate, and generate all artifacts.
///
/// Reads a ponyiser.toml manifest, validates it, runs capability analysis,
/// and generates .pony source files in the output directory.
pub fn generate(manifest_path: &str, output_dir: &str) -> anyhow::Result<()> {
    let m = load_manifest(manifest_path)?;
    validate(&m)?;
    codegen::generate_all(&m, output_dir)?;
    Ok(())
}
