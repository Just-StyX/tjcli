# tjcli 🚀

A lightning-fast command-line interface tool written in Rust to instantly scaffold a modern, fully configured **Spring Boot 4.1.1 + Thymeleaf + Tailwind CSS v4 + Vite** application. 

It handles all the configuration boilerplate—including setting up Java 25 properties, linking Vite's hot-reload server directly to your Thymeleaf templates, integrating Tailwind v4 dynamically, setting up a standard `.gitignore`, and creating a production-ready GitHub Actions CI/CD workflow.

---

## ✨ Features

- **Java 25 Ready:** Configures your `build.gradle.kts` toolchain properties out of the box.
- **Tailwind CSS v4 & Vite:** Completely skips legacy `tailwind.config.js` setups, opting for Tailwind v4's high-performance native Vite bundling (`@tailwindcss/vite`).
- **Instant Browser Live-Reload:** Bundles `vite-plugin-live-reload` so edits to your Thymeleaf `.html` pages dynamically refresh your browser window instantly.
- **Unified Build Pipeline:** Automatically links your Gradle resource building phases to execution tasks via `afterEvaluate` loops to run production frontend minification scripts gracefully.
- **Automated Bootstrapping:** Runs `npm install` automatically right after file generation.
- **Automated CI/CD:** Generates a pre-configured GitHub Actions workflow file (`ci.yml`) to compile your application into a deployable `.jar` package automatically.

---

## 🛠️ Requirements

Before running the generated project, ensure you have the following installed on your machine:
- [Rust](https://rust-lang.org) (to compile/run `tjcli`)
- [Java 25](https://adoptium.net)
- [Node.js (v20 or higher)](https://nodejs.org)

---

## 🚀 How to Use

### 1. Build or Install the CLI Tool Locally
Clone your repository and build the production binary:
```bash
cargo build --release
```
The optimized executable will be generated at `./target/release/tjcli`.

### 2. Scaffold a Project
Run the tool and specify your project options via flags:

```bash
./target/release/tjcli \
  --artifact-id "thymeleafDemo" \
  --group-id "jsl.group" \
  --java-version "25" \
  --node-version "20.11.0" \
  --app-name "thymeleaf-demo-app"
```

### Available Configuration Flags
All options fall back to sensible defaults if omitted:

| Flag | Long Flag | Default Value | Description |
| :--- | :--- | :--- | :--- |
| `-a` | `--artifact-id` | `thymeleafDemo` | The name of the project folder & final target `.jar` output |
| `-g` | `--group-id` | `jsl.group` | Your Java package group architecture structure |
| `-j` | `--java-version` | `25` | Target Java language SDK toolchain specification version |
| `-o` | `--node-version` | `26.1.0` | Node runtime version passed to Gradle's Node runner plugin |
| `-n` | `--app-name` | `thymeleaf-demo-app` | Internal `spring.application.name` property value in `application.yml` |
| `-p` | `--port` | `8080` | Local port binding definition assigned inside your application.yml file |

---

## 💻 Local Development Workflow

Once your project is scaffolded, navigate into the directory and launch the split-terminal local stack:

```bash
cd thymeleafDemo
```

### Step 1: Start the Frontend Watcher (Terminal 1)
This boots up the Vite HMR asset compiler at `http://localhost:5173` and tracks live saves inside your layout documents:
```bash
npm run dev
```

### Step 2: Boot Up the Spring Application (Terminal 2)
Run the backend web container. Because `spring.thymeleaf.cache` is disabled during development mode, it will seamlessly reference the live CSS modules served from Vite:
```bash
./gradlew bootRun
```
Open your browser to **`http://localhost:8080`** and start modifying files inside `src/main/resources/templates/index.html`!

---

## 📦 Production Bundling & Deployment

When you are ready to package your application for deployment, run standard Gradle packaging:
```bash
./gradlew build
```
**What happens behind the scenes:** Gradle triggers the node plugin execution cycle, builds the compiled frontend distribution styles, injects minified static assets directly into Spring's `/static/css/tailwind.css` structure, and outputs an executable self-contained package file to `build/libs/thymeleafDemo.jar`.

---

## 📄 Generated Project Structure

```text
my-app/
├── .github/workflows/ci.yml <-- Automated multi-step Java 25 & Node pipeline
├── build.gradle.kts         <-- Native lazy task linkages via afterEvaluate
├── settings.gradle.kts
├── package.json             <-- Vite + Tailwind CSS v4 compiler configurations
├── vite.config.js           <-- Handles HTML live-reloads and CSS asset targeting
├── .gitignore               <-- Clears build/, node_modules/, and IDE caches
└── src/
    └── main/
        ├── java/jsl/group/thymeleafdemo/
        │   ├── ThymeleafDemoApplication.java  <-- Main entry class
        │   └── HomeController.java            <-- Simple root route web controller
        └── resources/
            ├── application.yml                <-- App metadata & disabled hot caches
            ├── static/css/input.css           <-- Tailwind v4 standard entrypoint
            └── templates/index.html           <-- Dual dev-to-prod layout file
```

---

## 🛡️ License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
