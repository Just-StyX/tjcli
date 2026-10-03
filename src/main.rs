use clap::Parser;
use dialoguer::{Confirm, Input,};
use std::error::Error;

// Import your existing configurations from your lib module
use tjcli::{run, TclConfig,};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // 1. Sniff the host terminal arguments array list
    let args: Vec<String> = std::env::args().collect();

    let final_config = if args.len() == 1 {
        // 2. Zero flags detected -> Initiate Interactive Setup Mode 🛠️
        println!("\x1b[36m\x1b[1m⚙️  Entering Interactive Project Scaffolder Mode...\x1b[0m\n");

        let artifact_id: String = Input::new()
            .with_prompt("📁 Enter your Project Folder/Artifact ID")
            .with_initial_text("thymeleafDemo")
            .interact_text()?;

        let group_id: String = Input::new()
            .with_prompt("📦 Enter your Java Package Group ID")
            .with_initial_text("jsl.group")
            .interact_text()?;

        let app_name: String = Input::new()
            .with_prompt("🏷️  Enter your Application Name")
            .with_initial_text("thymeleaf-demo-app")
            .interact_text()?;

        let java_version: String = Input::new()
            .with_prompt("☕ Enter Target Java Version")
            .with_initial_text("25")
            .interact_text()?;

        let node_version: String = Input::new()
            .with_prompt("🟢 Enter Target Node.js Version")
            .with_initial_text("26.1.0")
            .interact_text()?;

        let port: u16 = Input::new()
            .with_prompt("🔌 Enter Server Port Local Binding")
            .with_initial_text("8080")
            .interact_text()?;

        let with_security = Confirm::new()
            .with_prompt("🔒 Would you like to include Spring Security Whitelisting?")
            .default(true)
            .interact()?;

        let build_image = Confirm::new()
            .with_prompt("🐳 Compile into an OCI Docker Container Image automatically?")
            .default(false)
            .interact()?;

        // Construct the exact TclConfig parsing wrapper structure programmatically
        TclConfig {
            artifact_id,
            group_id,
            java_version,
            app_name,
            node_version,
            port,
            with_security,
            build_image,
            image_name: None,
            registry: None,
        }
    } else {
        // 3. Flags present -> Safe parsing via native Clap specifications
        TclConfig::parse()
    };

    // 4. Pass the validated execution matrix into your core application function
    run(final_config)?;
    Ok(())
}
