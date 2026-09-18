// ==================================================================================
// ABSTRACT RESOLVER
// Copyright (c) 2026 Marcus Pratt / Fulminix Systems. All Rights Reserved.
// Licensed under the GNU Affero General Public License v3 (AGPLv3).
// For commercial inquiries, contact the author directly.
// ==================================================================================
// main.rs
pub mod lexer;
pub mod tracker;
pub mod healer;
pub mod format;

use healer::ActionType;
use std::io::{self, Write};
use std::fs;

fn main() {
    println!("==================================================");
    println!("  STRUCTURAL AST SELF-REPAIRING ENGINE - INTERACTIVE ");
    println!("==================================================");
    println!("Instructions: Type or drag-and-drop a file path (.rs, .v, .go, etc.)");
    println!("Type 'exit' or 'quit' to terminate the session safely.\n");

    loop {
        print!("[FILE WIZARD] >> ");
        // Force the prompt to flush to the screen immediately
        let _ = io::stdout().flush();

        let mut input_path = String::new();
        if io::stdin().read_line(&mut input_path).is_err() {
            println!("[ERROR]: Failed to read command-line input.");
            continue;
        }

        // Clean up trailing newline characters from input text
        let trimmed_path = input_path.trim().trim_matches('"').trim_matches('\'');

        if trimmed_path.eq_ignore_ascii_case("exit") || trimmed_path.eq_ignore_ascii_case("quit") {
            println!("[SYSTEM]: Terminating session. Good day, sir.");
            break;
        }

        if trimmed_path.is_empty() {
            continue;
        }

        // Read the target file contents out-of-band directly from the hard drive layout
        match fs::read_to_string(trimmed_path) {
            Ok(file_content) => {
                println!("\n==================================================");
                println!("DIAGNOSING CORE SYNTAX GEOMETRY: {}", trimmed_path);
                println!("==================================================");
                execute_pipeline(&file_content);
                println!("==================================================\n");
            }
            Err(e) => {
                println!("[FILE ERROR]: Unable to open path '{}' ({})\n", trimmed_path, e);
            }
        }
    }
}

/// Pipes raw source text sequentially through the spatial modules.
fn execute_pipeline(input_text: &str) {
    // Step 1: Scan text into unallocated spatial coordinate tokens
    let tokens = lexer::scan(input_text);

    // Step 2: Catalog block nesting heights, counts, and integrity flags
    let structural_map = tracker::analyze(&tokens);

    // Step 3: Run the geometric elimination engine to isolate the required fix
    let diagnostic = healer::find_fix(&structural_map, input_text);

    // Step 4: Audit structural padding directly against raw string layout (4 spaces per depth scale)
    let format_violations = format::audit_alignment(input_text, &tokens, &structural_map, 4);

    // Output clean diagnostic summaries directly to the screen
    println!("Total Valid Structural Blocks Verified : {}", diagnostic.total_valid_blocks);
    println!("Maximum Localized Nesting Depth Level  : {}", diagnostic.peak_nesting_depth);
    println!("Downstream Dislocated Cascades Caught  : {}", diagnostic.cascade_count);
    println!("Total Formatting Alignment Violations  : {}", format_violations.len());

    // Initialize our active working text string for physical mutation updates
    let mut working_text = String::from(input_text);

    // Step 5: Physically execute spacing padding updates based on structural audit constants
    for violation in format_violations.iter().rev() {
        let correct_padding = " ".repeat(violation.expected_spaces);
        working_text.replace_range(
            violation.coordinate..violation.coordinate + violation.observed_spaces,
            &correct_padding
        );
    }

    // Step 6: Physically execute missing delimiter injections or orphan purges
    match &diagnostic.instruction {
        Some(instruction) => {
            match &instruction.action {
                ActionType::InjectClose => {
                    println!("\n[DIAGNOSTIC STATUS]: DEFICIT FOUND");
                    println!(" -> Action Required : INJECT missing closing delimiter");
                    println!(" -> Required Symbol : '{}'", instruction.target_symbol);
                    println!(" -> Target Location : Absolute Byte Offset [{}]", instruction.coordinate);
                    
                    working_text.insert_str(instruction.coordinate, instruction.target_symbol);
                }
                ActionType::PurgeOrphan => {
                    println!("\n[DIAGNOSTIC STATUS]: STRUCTURAL WALL DETECTED");
                    println!(" -> Action Required : PURGE orphaned closing delimiter");
                    println!(" -> Target Location : Absolute Byte Offset [{}]", instruction.coordinate);
                    
                    working_text.replace_range(instruction.coordinate..instruction.coordinate + 1, "");
                }
            }
        }
        None => {
            println!("\n[DIAGNOSTIC STATUS]: STRUCTURE STABLE");
            println!(" -> Action Required : None. Tree is completely aligned and self-contained.");
        }
    } match &diagnostic.instruction {
        Some(ref instruction) => {
            println!("\n[FINAL REPAIR INSTRUCTION]:");
            println!(" -> Action Type : {:?}", instruction.action);
            println!(" -> Target Symbol : '{}'", instruction.target_symbol);
            println!(" -> Coordinate Offset : [{}]", instruction.coordinate);
        }
        None => {
            println!("\n[FINAL REPAIR INSTRUCTION]: No action required. Structure is stable.");
        }
    }

    // Output the final transformed text directly to verify the structural fix works
    println!("\n[FINAL REPAIRED OUTPUT]:");
    println!("--------------------------------------------------");
    
    // Process the lines to print structural diagnostic pointers natively beneath errors
    if let Some(ref instruction) = diagnostic.instruction {
        let mut byte_accumulator = 0;
        for line in working_text.lines() {
            println!("{}", line);
            let line_len_with_newline = line.len() + 1; // Approximate newline boundary
            
            // If the calculated modification point sits directly within this visual text line frame
            if instruction.coordinate >= byte_accumulator && instruction.coordinate <= byte_accumulator + line_len_with_newline {
                let local_offset = instruction.coordinate - byte_accumulator;
                let padding = " ".repeat(local_offset.saturating_sub(1));
                println!("{}^ [MODIFICATION INDEX]", padding);
            }
            byte_accumulator += line_len_with_newline;
        }
    } else {
        println!("{}", working_text);
    }
    println!("--------------------------------------------------");
}
