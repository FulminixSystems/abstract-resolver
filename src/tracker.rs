// ==================================================================================
// ABSTRACT RESOLVER
// Copyright (c) 2026 Marcus Pratt / Fulminix Systems. All Rights Reserved.
// Licensed under the GNU Affero General Public License v3 (AGPLv3).
// For commercial inquiries, contact the author directly.
// ==================================================================================
// tracker.rs
use crate::lexer::{Token, TokenType};
use std::ops::Range;

/// Binary and structural states flagging the integrity of a tracking block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockStatus {
    Complete,   // Both open and close delimiters match perfectly.
    Leaking,    // Open delimiter is present, but never received a close before the scope boundary.
    Orphaned,   // A close delimiter appeared with no matching open on the active stack.
    Dislocated, // Internally balanced block pushed out of position by a upstream premature closure.
}

/// A verified or unaligned structural block container mapped directly by physical spans.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralBlock {
    pub span: Range<usize>,       // The absolute physical text boundaries in bytes
    pub token_index: usize,       // Added to store the starting token index order
    pub current_depth: usize,     // Added to store the exact depth level where this block opened
    pub children_count: usize,    // Total number of immediate child structures contained within
    pub max_nesting_depth: usize, // The deepest nested layer reached within this specific scope
    pub status: BlockStatus,      // The structural integrity categorization
}

/// Internal scratchpad state to monitor unclosed structures on the active stack.
struct OpenFrame {
    token_index: usize,
    start_byte: usize,
    current_depth: usize,
    child_count: usize,
    max_inner_depth: usize,
}

/// Analyzes a flat stream of tokens to build a comprehensive map of structural boundaries.
/// Runs in a single linear pass without halting on syntax blowouts.
pub fn analyze(tokens: &[Token]) -> Vec<StructuralBlock> {
    let mut blocks = Vec::new();
    let mut open_stack: Vec<OpenFrame> = Vec::new();
    let mut current_depth = 0;
    let mut is_dislocated_state = false;

    for (index, token) in tokens.iter().enumerate() {
        match token.token_type {
            TokenType::Open => {
                current_depth += 1;
                
                // Track if this entry is entering an already unaligned or broken global container
                if open_stack.is_empty() && is_dislocated_state {
                    // Left tracking boundary is set to true outer limits
                }

                open_stack.push(OpenFrame {
                    token_index: index,
                    start_byte: token.span.start,
                    current_depth,
                    child_count: 0,
                    max_inner_depth: current_depth,
                });
            }
            TokenType::Close => {
                if let Some(frame) = open_stack.pop() {
                    let end_byte = token.span.end;
                    let max_depth_reached = std::cmp::max(frame.max_inner_depth, current_depth);
                    
                    let status = if is_dislocated_state && open_stack.is_empty() {
                        BlockStatus::Dislocated
                    } else {
                        BlockStatus::Complete
                    };

                    let block = StructuralBlock {
                        span: frame.start_byte..end_byte,
                        token_index: frame.token_index,
                        current_depth: frame.current_depth,
                        children_count: frame.child_count,
                        max_nesting_depth: max_depth_reached,
                        status,
                    };

                    // If a parent structure exists on the stack, register this as its child
                    if let Some(parent_frame) = open_stack.last_mut() {
                        parent_frame.child_count += 1;
                        parent_frame.max_inner_depth = std::cmp::max(parent_frame.max_inner_depth, max_depth_reached);
                    }

                    blocks.push(block);
                    current_depth -= 1;
                } else {
                    // The stack is empty: an unanchored extra closing delimiter has hit the pipeline
                    is_dislocated_state = true;
                    blocks.push(StructuralBlock {
                        span: token.span.clone(),
                        token_index: index,
                        current_depth: 0,
                        children_count: 0,
                        max_nesting_depth: current_depth,
                        status: BlockStatus::Orphaned,
                    });
                }
            }
            _ => {
                // For general structural identifiers, check if we need to adjust child context
                if token.token_type == crate::lexer::TokenType::Terminal {
                    if let Some(frame) = open_stack.last_mut() {
                        frame.child_count += 1; // Terminals map inside an open structure but don't adjust block heights
                    }
                }
            }
        }
    }

    // Flush any leftover unclosed elements remaining on the tracking stack at EOF
    while let Some(frame) = open_stack.pop() {
        let end_byte = if let Some(last_token) = tokens.last() {
            last_token.span.end
        } else {
            frame.start_byte
        };

        blocks.push(StructuralBlock {
            span: frame.start_byte..end_byte,
            token_index: frame.token_index,
            current_depth: frame.current_depth,
            children_count: frame.child_count,
            max_nesting_depth: frame.max_inner_depth,
            status: BlockStatus::Leaking,
        });
    }

    blocks
}
