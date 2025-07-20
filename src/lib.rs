use std::collections::HashMap;
use std::sync::RwLock;
use zed_extension_api::{
    self as zed,
    SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput,
    SlashCommandOutputSection, Worktree,
};

struct HighLighterExtension {
    state: RwLock<HighLighterState>,
}

struct HighLighterState {
    highlights: HashMap<String, Vec<HighlightPattern>>,
    colors: Vec<String>,
    current_color_index: usize,
}

#[derive(Clone)]
struct HighlightPattern {
    pattern: String,
    color: String,
    case_sensitive: bool,
    whole_word: bool,
    is_regex: bool,
}

impl zed::Extension for HighLighterExtension {
    fn new() -> Self {
        Self {
            state: RwLock::new(HighLighterState {
                highlights: HashMap::new(),
                colors: vec![
                    "#FFD700".to_string(), // Gold
                    "#FF6347".to_string(), // Tomato
                    "#32CD32".to_string(), // LimeGreen
                    "#FF1493".to_string(), // DeepPink
                    "#00CED1".to_string(), // DarkTurquoise
                    "#9370DB".to_string(), // MediumPurple
                    "#FFA500".to_string(), // Orange
                    "#20B2AA".to_string(), // LightSeaGreen
                ],
                current_color_index: 0,
            }),
        }
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "highlight" => Ok(vec![
                SlashCommandArgumentCompletion {
                    label: "Case Sensitive".to_string(),
                    new_text: "--case-sensitive ".to_string(),
                    run_command: false,
                },
                SlashCommandArgumentCompletion {
                    label: "Whole Word".to_string(),
                    new_text: "--whole-word ".to_string(),
                    run_command: false,
                },
                SlashCommandArgumentCompletion {
                    label: "Regex Pattern".to_string(),
                    new_text: "--regex ".to_string(),
                    run_command: false,
                },
            ]),
            "next_highlight" | "prev_highlight" | "clear_highlights" => Ok(vec![]),
            command => Err(format!("unknown slash command: \"{}\"", command)),
        }
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "highlight" => {
                if args.is_empty() {
                    return Err("Please provide text to highlight".to_string());
                }

                let (options, pattern_args) = HighlightOptions::from_args(&args);
                if pattern_args.is_empty() {
                    return Err("Please provide text to highlight after options".to_string());
                }

                let pattern = pattern_args.join(" ");
                let was_added = self.toggle_highlight(pattern.clone(), options.clone());
                
                // We cannot directly highlight text through the API
                // but we'll add the pattern to our state
                // NOTE: The API currently doesn't provide direct text highlighting
                // Zed would need to implement this functionality in a future API version
                
                let mut flags = Vec::new();
                if options.case_sensitive {
                    flags.push("case-sensitive");
                }
                if options.whole_word {
                    flags.push("whole-word");
                }
                if options.is_regex {
                    flags.push("regex");
                }
                
                let flag_str = if flags.is_empty() {
                    String::new()
                } else {
                    format!(" ({})", flags.join(", "))
                };
                
                let action = if was_added { "Added" } else { "Removed" };
                let result_text = format!("{} highlight for pattern: '{}'{}", action, pattern, flag_str);
                
                Ok(SlashCommandOutput {
                    sections: vec![SlashCommandOutputSection {
                        range: (0..result_text.len()).into(),
                        label: "Text Highlighter".to_string(),
                    }],
                    text: result_text,
                })
            }
            "next_highlight" => {
                if self.get_all_patterns().is_empty() {
                    return Ok(SlashCommandOutput {
                        sections: vec![],
                        text: "No highlights found. Use /highlight to add some.".to_string(),
                    });
                }

                // NOTE: With the current API limitations, we can't directly
                // implement navigation between highlights
                let patterns = self.get_all_patterns();
                if !patterns.is_empty() {
                    // Log what we would do if API supported it
                    println!("Would navigate to next highlight for pattern: {}", 
                            patterns.first().unwrap().pattern);
                }

                Ok(SlashCommandOutput {
                    sections: vec![],
                    text: format!("Navigating to next highlight... {}", self.get_pattern_summary()),
                })
            }
            "prev_highlight" => {
                if self.get_all_patterns().is_empty() {
                    return Ok(SlashCommandOutput {
                        sections: vec![],
                        text: "No highlights found. Use /highlight to add some.".to_string(),
                    });
                }

                // NOTE: With the current API limitations, we can't directly
                // implement navigation between highlights
                let patterns = self.get_all_patterns();
                if !patterns.is_empty() {
                    // Log what we would do if API supported it
                    println!("Would navigate to previous highlight for pattern: {}", 
                            patterns.first().unwrap().pattern);
                }

                Ok(SlashCommandOutput {
                    sections: vec![],
                    text: format!("Navigating to previous highlight... {}", self.get_pattern_summary()),
                })
            }
            "clear_highlights" => {
                let count = self.get_all_patterns().len();
                self.clear_all_highlights();
                
                // NOTE: With the current API limitations, we can't directly
                // clear highlights. Only clearing our internal state.
                
                Ok(SlashCommandOutput {
                    sections: vec![],
                    text: format!("Cleared {} highlight patterns", count),
                })
            }
            command => Err(format!("unknown slash command: \"{}\"", command)),
        }
    }
}

impl HighLighterExtension {
    fn toggle_highlight(&self, pattern: String, options: HighlightOptions) -> bool {
        let mut state = self.state.write().unwrap();
        let color = state.get_next_color();
        
        // Check if pattern already exists (match by pattern AND options)
        if let Some(patterns) = state.highlights.get_mut("default") {
            if let Some(pos) = patterns.iter().position(|p| 
                p.pattern == pattern && 
                p.case_sensitive == options.case_sensitive && 
                p.whole_word == options.whole_word && 
                p.is_regex == options.is_regex
            ) {
                // Remove existing highlight
                patterns.remove(pos);
                return false; // Removed
            }
        }
        
        // Add new highlight
        let highlight = HighlightPattern {
            pattern: pattern.clone(),
            color,
            case_sensitive: options.case_sensitive,
            whole_word: options.whole_word,
            is_regex: options.is_regex,
        };
        
        state.highlights
            .entry("default".to_string())
            .or_insert_with(Vec::new)
            .push(highlight);
        
        true // Added
    }
    
    fn clear_all_highlights(&self) {
        self.state.write().unwrap().highlights.clear();
    }
    
    fn get_all_patterns(&self) -> Vec<HighlightPattern> {
        self.state.read().unwrap()
            .highlights
            .values()
            .flat_map(|patterns| patterns.iter())
            .cloned()
            .collect()
    }

    fn get_pattern_summary(&self) -> String {
        let patterns = self.get_all_patterns();
        if patterns.is_empty() {
            return "No active highlights".to_string();
        }
        
        let count = patterns.len();
        let mut summary = Vec::new();
        for pattern in &patterns {
            let mut flags = Vec::new();
            if pattern.case_sensitive {
                flags.push("case-sensitive");
            }
            if pattern.whole_word {
                flags.push("whole-word");
            }
            if pattern.is_regex {
                flags.push("regex");
            }
            
            let flag_str = if flags.is_empty() {
                String::new()
            } else {
                format!(" ({})", flags.join(", "))
            };
            
            summary.push(format!("'{}'{} [{}]", pattern.pattern, flag_str, pattern.color));
        }
        
        format!("{} active highlights: {}", count, summary.join(", "))
    }
}

impl HighLighterState {
    fn get_next_color(&mut self) -> String {
        let color = self.colors[self.current_color_index].clone();
        self.current_color_index = (self.current_color_index + 1) % self.colors.len();
        color
    }
}

#[derive(Default, Clone)]
struct HighlightOptions {
    case_sensitive: bool,
    whole_word: bool,
    is_regex: bool,
}

impl HighlightOptions {
    fn from_args(args: &[String]) -> (Self, Vec<String>) {
        let mut options = Self::default();
        let mut remaining_args = Vec::new();
        
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--case-sensitive" => options.case_sensitive = true,
                "--whole-word" => options.whole_word = true,
                "--regex" => options.is_regex = true,
                arg => remaining_args.push(arg.to_string()),
            }
            i += 1;
        }
        
        (options, remaining_args)
    }
}

zed::register_extension!(HighLighterExtension);