// ==================================================================================
// ABSTRACT RESOLVER
// Copyright (c) 2026 Mark Pratt / Fulminix Systems. All Rights Reserved.
// Licensed under the GNU Affero General Public License v3 (AGPLv3).
// For commercial inquiries, contact the author directly.
// ==================================================================================
// format.rs
use crate::lexer::Token;
use crate::tracker::StructuralBlock;

/// Represents a distinct formatting alignment anomaly found on the spatial map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormattingViolation {
    pub coordinate: usize,       // The absolute byte offset of the misaligned line
    pub expected_spaces: usize,  // What the mathematical nesting depth demanded
    pub observed_spaces: usize,  // What was physically counted in the file string
}

/// Audits structural blocks directly against the raw text layout and token streams.
/// Anchored strictly to your exact Token definitions and layout parameter logic.
pub fn audit_alignment(
    input: &str,
    tokens: &[Token],
    blocks: &[StructuralBlock],
    spaces_per_depth: usize,
) -> Vec<FormattingViolation> {
    let mut violations = Vec::new();
    let bytes = input.as_bytes();

    // Verify tokens are present to ensure coordinate synchronization is valid
    if tokens.is_empty() {
        return violations;
    }

    for block in blocks {
        let start_pos = block.span.start;
        
        // Scan backward from the start of the block to calculate leading spaces on this line
        let mut current_pos = start_pos;
        let mut observed_spaces = 0;
        let mut is_start_of_line = false;

        while current_pos > 0 {
            current_pos -= 1;
            match bytes[current_pos] {
                b' ' => {
                    observed_spaces += 1;
                }
                b'\n' | b'\r' => {
                    is_start_of_line = true;
                    break;
                }
                _ => {
                    // Encountered non-whitespace text before a newline; this block is inline, skip it
                    break;
                }
            }
        }

        // If the block sits natively at the start of a new line, audit its padding
        if is_start_of_line || current_pos == 0 {
            // Adjust calculation base: complete outer blocks sit at base level (0 spaces)
            let base_depth = block.max_nesting_depth.saturating_sub(1);
            let expected_spaces = base_depth * spaces_per_depth;

            if observed_spaces != expected_spaces {
                violations.push(FormattingViolation {
                    coordinate: start_pos - observed_spaces,
                    expected_spaces,
                    observed_spaces,
                });
            }
        }
    }

    violations
}
