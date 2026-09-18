use std::ops::Range;

/// Minimal variants tracking only structural anchors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Open,       // Represents structures starting like '{', '[', or '('
    Close,      // Represents structures ending like '}', ']', or ')'
    Identifier, // Represents continuous alphanumeric text blocks/names to count as units
    Terminal,   // Represents semicolons or explicit statement-end markers
}

/// Pairs a TokenType with an exact, non-allocating absolute byte span in the file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Range<usize>,
}

/// Scans raw character bytes linearly to emit flat structural locations.
/// Runs in O(N) time with zero allocations on the source text.
pub fn scan(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let bytes = input.as_bytes();
    let mut pos = 0;

    while pos < bytes.len() {
        // 1. Verilog Module Scope Boundaries
        if bytes[pos..].starts_with(b"endmodule") {
            tokens.push(Token {
                token_type: TokenType::Close,
                span: pos..pos + 9,
            });
            pos += 9;
            continue;
        }
        if bytes[pos..].starts_with(b"module") {
            tokens.push(Token {
                token_type: TokenType::Open,
                span: pos..pos + 6,
            });
            pos += 6;
            continue;
        }

        // 2. Verilog Procedural Block Boundaries
        if bytes[pos..].starts_with(b"begin") {
            tokens.push(Token {
                token_type: TokenType::Open,
                span: pos..pos + 5,
            });
            pos += 5;
            continue;
        }
        if bytes[pos..].starts_with(b"end") {
            tokens.push(Token {
                token_type: TokenType::Close,
                span: pos..pos + 3,
            });
            pos += 3;
            continue;
        }

        // 3. Verilog Parallel Thread Boundaries
        if bytes[pos..].starts_with(b"fork") {
            tokens.push(Token {
                token_type: TokenType::Open,
                span: pos..pos + 4,
            });
            pos += 4;
            continue;
        }
        if bytes[pos..].starts_with(b"join") {
            tokens.push(Token {
                token_type: TokenType::Close,
                span: pos..pos + 4,
            });
            pos += 4;
            continue;
        }
        // 4. Verilog Preprocessor Directives
if bytes[pos..].starts_with(b"`endif") {
    tokens.push(Token {
        token_type: TokenType::Close,
        span: pos..pos + 6,
    });
    pos += 6;
    continue;
}
if bytes[pos..].starts_with(b"`ifdef") {
    tokens.push(Token {
        token_type: TokenType::Open,
        span: pos..pos + 6,
    });
    pos += 6;
    continue;
}
if bytes[pos..].starts_with(b"`ifndef") {
    tokens.push(Token {
        token_type: TokenType::Open,
        span: pos..pos + 7,
    });
    pos += 7;
    continue;
}
// 5. Skip Single-Line Comments Immediately
if bytes[pos..].starts_with(b"//") {
    pos += 2;
    while pos < bytes.len() && bytes[pos] != b'\n' {
        pos += 1;
    }
    continue;
}

// 6. Skip Multi-Line Block Comments Immediately
if bytes[pos..].starts_with(b"/*") {
    pos += 2;
    while pos < bytes.len() && !bytes[pos..].starts_with(b"*/") {
        pos += 1;
    }
    if pos < bytes.len() {
        pos += 2; // Advance past the closing '*/'
    }
    continue;
}
// 7. Skip Script-Style Single-Line Comments (# for Python/Shell/TOML)
if bytes[pos] == b'#' {
    pos += 1;
    while pos < bytes.len() && bytes[pos] != b'\n' {
        pos += 1;
    }
    continue;
}
// 8. Skip VHDL/SQL Style Single-Line Comments (-- for VHDL/SQL)
if bytes[pos..].starts_with(b"--") {
    pos += 2;
    while pos < bytes.len() && bytes[pos] != b'\n' {
        pos += 1;
    }
    continue;
}

        match bytes[pos] {
            // Skip layout padding entirely
            b' ' | b'\t' | b'\n' | b'\r' => {
                pos += 1;
            }
            // Structural Open Anchors
            b'{' | b'[' | b'(' => {
                tokens.push(Token {
                    token_type: TokenType::Open,
                    span: pos..pos + 1,
                });
                pos += 1;
            }
            // Structural Close Anchors
            b'}' | b']' | b')' => {
                tokens.push(Token {
                    token_type: TokenType::Close,
                    span: pos..pos + 1,
                });
                pos += 1;
            }
            // Statement Terminals
            b';' => {
                tokens.push(Token {
                    token_type: TokenType::Terminal,
                    span: pos..pos + 1,
                });
                pos += 1;
            }
            // Aggregate contiguous characters into a single spatial block boundary
            _ => {
                let start = pos;
                while pos < bytes.len()
                    && !matches!(
                        bytes[pos],
                        b' ' | b'\t' | b'\n' | b'\r' | b'{' | b'[' | b'(' | b'}' | b']' | b')' | b';'
                    )
                {
                    pos += 1;
                }
                tokens.push(Token {
                    token_type: TokenType::Identifier,
                    span: start..pos,
                });
            }
        }
    }

    tokens
}
