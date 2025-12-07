use std::env;
use std::fs;
use std::path::Path;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin new_day -- <day_number>");
        eprintln!("Example: cargo run --bin new_day -- 3");
        std::process::exit(1);
    }
    
    let day_number = &args[1];
    
    // Validate day number
    if let Err(_) = day_number.parse::<u32>() {
        eprintln!("Error: Day number must be a valid integer");
        std::process::exit(1);
    }
    
    let template_dir = Path::new("src/days/day_template");
    let new_day_dir = Path::new("src/days").join(format!("day{}", day_number));
    
    if new_day_dir.exists() {
        eprintln!("Error: Day {} already exists!", day_number);
        std::process::exit(1);
    }
    
    if !template_dir.exists() {
        eprintln!("Error: Template directory not found at {:?}", template_dir);
        std::process::exit(1);
    }
    
    // Copy template directory
    println!("Creating day{}...", day_number);
    copy_dir_recursive(&template_dir, &new_day_dir).expect("Failed to copy template");
    
    // Update main.rs to use correct paths
    let main_rs_path = new_day_dir.join("main.rs");
    let main_content = fs::read_to_string(&main_rs_path).expect("Failed to read main.rs");
    let updated_content = main_content.replace("day_template", &format!("day{}", day_number));
    fs::write(&main_rs_path, updated_content).expect("Failed to update main.rs");
    
    // Update mod.rs to include the new day
    let mod_rs_path = Path::new("src/days/mod.rs");
    let mut mod_content = fs::read_to_string(&mod_rs_path).expect("Failed to read mod.rs");
    let new_mod_line = format!("pub mod day{};\n", day_number);
    
    if !mod_content.contains(&new_mod_line.trim()) {
        mod_content.push_str(&new_mod_line);
        fs::write(&mod_rs_path, mod_content).expect("Failed to update mod.rs");
    }
    
    println!("✓ Created day{} successfully!", day_number);
    println!("  - Source: src/days/day{}/main.rs", day_number);
    println!("  - Input: src/days/day{}/input_files/input.txt", day_number);
    println!("  - Test input: src/days/day{}/input_files/test_input.txt", day_number);
    println!("  - Part 1 description: src/days/day{}/part_1.md", day_number);
    println!("  - Part 2 description: src/days/day{}/part_2.md", day_number);
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}
