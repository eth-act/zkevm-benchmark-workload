use std::{fs, path::Path};
use witness_generator::{generate_stateless_witness, BlocksAndWitnesses};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting witness generation...");
    
    // Generate witnesses using the existing function
    let witnesses = generate_stateless_witness::generate();
    
    println!("Generated {} witness collections", witnesses.len());
    
    // Create the output directory
    let workspace_root = Path::new(env!("CARGO_WORKSPACE_DIR"));
    let output_dir = workspace_root.join("zkevm-fixtures-with-witnesses");
    
    // Remove existing directory if it exists and create fresh
    if output_dir.exists() {
        println!("Removing existing witness directory...");
        fs::remove_dir_all(&output_dir)?;
    }
    
    fs::create_dir_all(&output_dir)?;
    println!("Created output directory: {}", output_dir.display());
    
    // Save each witness collection to a separate file
    for witness in &witnesses {
        let filename = format!("{}.json", sanitize_filename(&witness.name));
        let file_path = output_dir.join(filename);
        
        println!("Saving witness: {} -> {}", witness.name, file_path.display());
        
        let json_content = serde_json::to_string_pretty(witness)?;
        fs::write(&file_path, json_content)?;
    }
    
    // Create an index file that lists all available witnesses
    let index_path = output_dir.join("index.json");
    let witness_names: Vec<&str> = witnesses.iter().map(|w| w.name.as_str()).collect();
    let index_content = serde_json::to_string_pretty(&witness_names)?;
    fs::write(&index_path, index_content)?;
    
    println!("Created index file with {} witnesses", witness_names.len());
    println!("Witness generation completed successfully!");
    println!("Witnesses saved to: {}", output_dir.display());
    
    Ok(())
}

/// Sanitize a filename by replacing invalid characters with underscores
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '[' | ']' => '_',
            c => c,
        })
        .collect()
}
