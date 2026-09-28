use std::error::Error;
use std::fs::{create_dir_all, write};
use std::path::Path;
use std::process::Command;

use clap::Parser;

type GenericError = Box<dyn Error + Send + Sync>;
type TclResult<T> = Result<T, GenericError>;

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    name = "tjcli",
    about,
    help_template = "{name} {version}\n{author}\n{about}\n\n{usage-heading} {usage}\n\n{all-args}"
)]
pub struct TclConfig {
    #[arg(short = 'a', long = "artifact-id", default_value = "thymeleafDemo")]
    artifact_id: String,

    #[arg(short = 'g', long = "group-id", default_value = "jsl.group")]
    group_id: String,

    #[arg(short = 'j', long = "java-version", default_value = "25")]
    java_version: String,

    #[arg(short = 'n', long = "app-name", default_value = "thymeleaf-demo-app")]
    app_name: String,

    #[arg(short = 'o', long = "node-version", default_value = "26.1.0")]
    node_version: String,

    #[arg(short = 'p', long = "port", default_value_t = 8080)]
    port: u16,
}

pub fn run(args: TclConfig) -> TclResult<()> {
    let root = Path::new(&args.artifact_id);
    println!(
        "🚀 Scaffolding Spring Boot app: '{}' inside folder '{}'...",
        args.app_name, args.artifact_id
    );
    // 1. Convert Group ID & Artifact ID package structure to safe path paths (e.g. jsl/group/thymeleafdemo)
    let package_clean = args.group_id.replace('.', "/") + "/" + &args.artifact_id.to_lowercase();
    let java_package_path = root.join("src/main/java").join(&package_clean);
    let resources_css_path = root.join("src/main/resources/static/css");
    let resources_templates_path = root.join("src/main/resources/templates");
    let github_workflow_path = root.join(".github/workflows");

    create_dir_all(&java_package_path)?;
    create_dir_all(&resources_css_path)?;
    create_dir_all(&resources_templates_path)?;
    create_dir_all(&github_workflow_path)?;

    // 2. Build Tools Configuration
    write(
        root.join("build.gradle.kts"),
        get_gradle_config(
            &args.group_id,
            &args.artifact_id,
            &args.java_version,
            &args.node_version,
        ),
    )?;
    write(
        root.join("settings.gradle.kts"),
        format!("rootProject.name = \"{}\"\n", args.artifact_id),
    )?;
    write(root.join("package.json"), get_package_json())?;
    write(root.join("vite.config.js"), get_vite_config())?;
    write(root.join(".gitignore"), get_gitignore())?;

    // 3. Spring Code Layer Generation
    write(
        root.join("src/main/resources/application.yml"),
        get_application_yml(&args.app_name, args.port),
    )?;
    write(
        java_package_path.join(format!(
            "{}Application.java",
            uppercase_first(&args.artifact_id)
        )),
        get_application_class(&args.group_id, &args.artifact_id),
    )?;
    write(
        java_package_path.join("HomeController.java"),
        get_home_controller(&args.group_id, &args.artifact_id),
    )?;

    // 4. Stylesheets & UI Entry Point
    write(
        resources_css_path.join("input.css"),
        "@import \"tailwindcss\";\n",
    )?;
    write(
        resources_templates_path.join("index.html"),
        get_index_html(),
    )?;

    // 5. Native Github Actions Pipeline Generation
    write(
        github_workflow_path.join("ci.yml"),
        get_github_workflow(&args.java_version, &args.node_version),
    )?;

    // 6. Automatically trigger internal system dependencies bootstrap pipeline via command engine
    println!("📦 Triggering 'npm install' inside project environment...");
    let npm_status = Command::new("npm")
        .arg("install")
        .current_dir(root)
        .status();

    match npm_status {
        Ok(status) if status.success() => {
            println!("✅ Node package architectures built successfully.")
        }
        _ => println!(
            "⚠️ Failed to run 'npm install' automatically. Make sure Node.js is installed locally and run it manually."
        ),
    }

    println!("\n✨ App Generation Process Completed Successfully!");
    println!(
        "👉 Run: 'cd {}' then initiate hot reloading via 'npm run dev' and './gradlew bootRun'",
        args.artifact_id
    );

    Ok(())
}

fn uppercase_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn get_application_yml(app_name: &str, port: u16) -> String {
    format!(
        r#"spring:
  application:
    name: {}
  thymeleaf:
    cache: false
  devtools:
    restart:
      enabled: true
    livereload:
      enabled: true
server:
  port: {}
"#,
        app_name, port
    )
}

fn get_gradle_config(
    group_id: &str,
    artifact_id: &str,
    java_version: &str,
    node_version: &str,
) -> String {
    format!(
        r#"plugins {{
    java
    id("org.springframework.boot") version "4.1.1"
    id("io.spring.dependency-management") version "1.1.7"
    id("com.github.node-gradle.node") version "7.0.2"
}}

group = "{}"
version = "0.0.1-SNAPSHOT"

java {{
    toolchain {{
        languageVersion.set(JavaLanguageVersion.of({}))
    }}
}}

repositories {{
    mavenCentral()
}}

dependencies {{
    implementation("org.springframework.boot:spring-boot-starter-thymeleaf")
    implementation("org.springframework.boot:spring-boot-starter-web")
    developmentOnly("org.springframework.boot:spring-boot-devtools")
    testImplementation("org.springframework.boot:spring-boot-starter-test")
}}

node {{
    version.set("{}")
    download.set(true)
}}

tasks.named<org.springframework.boot.gradle.tasks.bundling.BootJar>("bootJar") {{
    archiveFileName.set("{}.jar")
}}

tasks.named("processResources") {{
    dependsOn(tasks.matching {{ it.name == "npm_run_build" }})
}}
"#,
        group_id, java_version, node_version, artifact_id
    )
}

fn get_application_class(group_id: &str, artifact_id: &str) -> String {
    let package_name = format!("{}.{}", group_id, artifact_id.to_lowercase());
    let class_name = format!("{}Application", uppercase_first(artifact_id));
    format!(
        r#"package {};

import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class {} {{
    public static void main(String[] args) {{
        SpringApplication.run({}.class, args);
    }}
}}
"#,
        package_name, class_name, class_name
    )
}

fn get_home_controller(group_id: &str, artifact_id: &str) -> String {
    format!(
        r#"package {}.{};

import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;

@Controller
public class HomeController {{
    @GetMapping("/")
    public String index() {{
        return "index";
    }}
}}
"#,
        group_id,
        artifact_id.to_lowercase()
    )
}

fn get_github_workflow(java_version: &str, node_version: &str) -> String {
    // Extracts major node version from string payload (e.g. 20.11.0 -> 20)
    let major_node = node_version.split('.').next().unwrap_or("26");
    format!(
        r#"name: Java CI with Gradle and Vite

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4

    - name: Set up JDK {}
      uses: actions/setup-java@v4
      with:
        java-version: '{}'
        distribution: 'temurin'
        cache: 'gradle'

    - name: Set up Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '{}'
        cache: 'npm'

    - name: Grant execute permission for gradlew
      run: chmod +x gradlew

    - name: Execute Full Production Bundle (Build Frontend & Compiles Jar)
      run: ./gradlew build

    - name: Upload Artifact
      uses: actions/upload-artifact@v4
      with:
        name: spring-boot-jar
        path: build/libs/*.jar
"#,
        java_version, java_version, major_node
    )
}

fn get_vite_config() -> &'static str {
    r#"import { defineConfig } from 'vite';
import tailwindcss from '@tailwindcss/vite';
import liveReload from 'vite-plugin-live-reload';

export default defineConfig({
  plugins: [
    tailwindcss(),
    liveReload('./src/main/resources/templates/**/*.html')
  ],
  build: {
    outDir: './src/main/resources/static',
    emptyOutDir: false,
    rollupOptions: {
      input: './src/main/resources/static/css/input.css',
      output: {
        assetFileNames: 'css/tailwind.[ext]'
      }
    }
  }
});
"#
}

fn get_package_json() -> &'static str {
    r#"{
  "name": "thymeleaf-demo-frontend",
  "private": true,
  "version": "1.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build"
  },
  "devDependencies": {
    "@tailwindcss/vite": "^4.0.0",
    "tailwindcss": "^4.0.0",
    "vite": "^5.1.0",
    "vite-plugin-live-reload": "^3.0.1"
  }
}"#
}

fn get_index_html() -> &'static str {
    r#"<!DOCTYPE html>
<html xmlns:th="http://www.thymeleaf.org" xmlns="https://www.w3.org/1999/xhtml">
<head>
    <meta charset="UTF-8">
    <title>Spring Boot + Tailwind v4</title>
    <link rel="stylesheet" href="http://localhost:5173/src/main/resources/static/css/input.css">
    <link rel="stylesheet" th:href="@{/css/tailwind.css}">
</head>
<body class="bg-zinc-950 text-zinc-50 flex items-center justify-center min-h-screen">
    <div class="p-8 bg-zinc-900 border border-zinc-800 rounded-3xl text-center shadow-2xl">
        <h1 class="text-4xl font-extrabold text-emerald-400">Tailwind v4 Is Live!</h1>
        <p class="mt-4 text-zinc-400">Dynamic variables parsed from Rust completed this scaffold setup successfully.</p>
    </div>
</body>
</html>
"#
}

fn get_gitignore() -> &'static str {
    r#"# Node/Frontend Dependencies
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*
.pnpm-debug.log*

# Build and Distribution Directories
build/
bin/
out/
target/
.gradle/
.vite/

# IDE and Code Editors Config Files
.idea/
*.iml
*.ipr
*.iws
.vscode/
.settings/
.project
.classpath

# Operating System Overheads
.DS_Store
Thumbs.db
"#
}
