// All Gherkin step definitions are consolidated in ask_steps.rs.
// Cucumber v0.23 registers steps globally; splitting step definitions
// across modules causes duplicate registration panics.
//
// This module exists only to declare the mod in the step hierarchy
// for future splitting if cucumber adds per-module scoping.
