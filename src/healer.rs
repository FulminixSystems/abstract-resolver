// ==================================================================================
// ABSTRACT RESOLVER
// Copyright (c) 2026 Marcus Pratt / Fulminix Systems. All Rights Reserved.
// Licensed under the GNU Affero General Public License v3 (AGPLv3).
// For commercial inquiries, contact the author directly.
// ==================================================================================
// healer.rs
use crate::tracker::{BlockStatus, StructuralBlock};

/// The explicit type of repair execution required to reconcile the syntax tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    InjectClose,   // Missing delimiter: drop a closing marker to complete a leaking circuit.
    PurgeOrphan,   // Extra delimiter: remove an orphaned closure that forms a structural wall.
}

/// A concrete, actionable structural patch directive containing exact character locations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealingInstruction {
    pub action: ActionType,
    pub coordinate: usize,   // The absolute byte offset in the file text
    pub target_symbol: &'static str, // The specific closing brace symbol determined by context
}

/// The final structural snapshot containing precise geometric diagnostic facts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealingDiagnostic {
    pub total_valid_blocks: usize,
    pub peak_nesting_depth: usize,
    pub cascade_count: usize,
    pub instruction: Option<HealingInstruction>,
}

/// Ingests tracked layout states and performs a single geometric boundary audit to output a definitive fix.
pub fn find_fix(blocks: &[StructuralBlock], input: &str) -> HealingDiagnostic {
    let mut total_valid_blocks = 0;
    let mut peak_nesting_depth = 0;
    let mut cascade_count = 0;
    
    let mut leaking_block: Option<&StructuralBlock> = None;
    let mut orphaned_block: Option<&StructuralBlock> = None;

    // Analyze block metrics and isolate target structural faults
    for block in blocks {
        match block.status {
            BlockStatus::Complete => {
                total_valid_blocks += 1;
                if block.max_nesting_depth > peak_nesting_depth {
                    peak_nesting_depth = block.max_nesting_depth;
                }
            }
            BlockStatus::Dislocated => {
                cascade_count += 1;
                if block.max_nesting_depth > peak_nesting_depth {
                    peak_nesting_depth = block.max_nesting_depth;
                }
            }
            BlockStatus::Leaking => {
                // Track the leak context; prioritize the inner-most leaking block if nested
                leaking_block = Some(block);
            }
            BlockStatus::Orphaned => {
                // Track the very first structural wall that caused the downstream dislocation
                if orphaned_block.is_none() {
                    orphaned_block = Some(block);
                }
            }
        }
    }

    let instruction = if let Some(orphan) = orphaned_block {
        // SCENARIO B: Purge the structural wall to resolve cascading dislocated blocks down the page
        Some(HealingInstruction {
            action: ActionType::PurgeOrphan,
            coordinate: orphan.span.start,
            target_symbol: " ", // Empty space character implies a clean extraction zone
        })
    } else if let Some(leak) = leaking_block {
        // SCENARIO A: Isolate the unclosed gap by evaluating inner perfect block boundaries
        let mut target_index = leak.span.end;

        // Loop through complete blocks to find any valid children resting right at the leak edge
        let mut max_covered_index = leak.span.start;
        for child in blocks.iter().filter(|b| b.status == BlockStatus::Complete) {
            if child.span.start >= leak.span.start && child.span.end <= leak.span.end {
                if child.span.end > max_covered_index {
                    max_covered_index = child.span.end;
                }
            }
        }

        // If perfect child anchors exist, the missing delimiter sits immediately behind the final stable element
        if max_covered_index > leak.span.start && max_covered_index < leak.span.end {
            target_index = max_covered_index;
        }

        // Evaluate context string to inject the correct inverse structural brace archetype
        // Evaluate context string to inject the correct inverse structural boundary archetype
        let slice = &input[leak.span.start..leak.span.end];
        let symbol = if slice.contains('{') {
            "}"
        } else if slice.contains('[') {
            "]"
        } else if slice.contains('(') {
            ")"
        } else if slice.contains("begin") {
            "end"
        } else if slice.contains("module") {
            "endmodule"
        } else if slice.contains("fork") {
            "join"
        } else if slice.contains("`ifdef") || slice.contains("`ifndef") {
            "`endif"
        } else {
            "}" // Safeguard fallback structural delimiter
        };

        Some(HealingInstruction {
            action: ActionType::InjectClose,
            coordinate: target_index,
            target_symbol: symbol,
        })
    } else {
        None
    };

    HealingDiagnostic {
        total_valid_blocks,
        peak_nesting_depth,
        cascade_count,
        instruction,
    }
}
