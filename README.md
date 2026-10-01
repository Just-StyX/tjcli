# tjcli 🚀

A lightning-fast command-line interface tool written in Rust to instantly scaffold a modern, fully configured **Spring Boot 4.1.1 + Thymeleaf + Tailwind CSS v4 + Vite** application. 

It handles all the configuration boilerplate—including setting up Java 25 properties, linking Vite's hot-reload server directly to your Thymeleaf templates, integrating Tailwind v4 dynamically, setting up a standard `.gitignore`, creating a production-ready GitHub Actions CI/CD workflow, and configuring Cloud Native Buildpacks for containerization.

---

## ✨ Features

- **Java 25 Ready:** Configures your `build.gradle.kts` toolchain properties out of the box.
- **Tailwind CSS v4 & Vite:** Completely skips legacy `tailwind.config.js` setups, opting for Tailwind v4's high-performance native Vite bundling (`@tailwindcss/vite`).
- **Instant Browser Live-Reload:** Bundles `vite-plugin-live-reload` so edits to your Thymeleaf `.html` pages dynamically refresh your browser window instantly.
- **Unified Build Pipeline:** Automatically links your Gradle resource building phases to execution tasks to run production frontend minification scripts gracefully.
- **Automated Bootstrapping:** Runs `npm install` automatically right after file generation.
- **Automated CI/CD:** Generates a pre-configured GitHub Actions workflow file (`ci.yml`) to compile your application into a deployable `.jar` package automatically.
- **🐳 Built-in OCI Image Support:** Native hooks into Spring Boot's Cloud Native Buildpack system (`./gradlew bootBuildImage`) with customizable names and image registries via CLI flags.

---

## 📐 System Architecture & Workflow

The generated application balances a dual-engine architecture designed for instant frontend feedback during local coding and optimized asset bundling during compilation phases:

```text
  [ Local Coding Mode ]
  ┌──────────────────┐
  │  Vite Dev Server │ ───( Serves Live CSS Modules )───┐
  │  (Port 5173 HMR) │                                  │
  └──────────────────┘                                  ▼
  ┌──────────────────┐                        ┌──────────────────┐
  │   Thymeleaf HTML │ ◄──( Live Reload Trigger )─── │  Web Browser     │
  │   Layout Template│                        │  http://localhost│
  └──────────────────┘                        └──────────────────┘
  ┌──────────────────┐                                  ▲
  │ Spring Boot Dev  │ ───( Emits HTML Content )────────┘
  │ (Port 8080 Engine│
  └──────────────────┘

  [ Production Compilation Pipeline ]
  ┌──────────────┐     ┌─────────────────────┐     ┌──────────────────────┐
  │ ./gradlew    │ ──► │ Gradle Node Plugin  │ ──► │ Vite Production Build│
  │ build/Image  │     │ Environment Setup   │     │ (Tailwind CSS v4)    │
  └──────────────┘     └─────────────────────┘     └──────────────────────┘
                                                              │
   ┌──────────────────────────────────────────────────────────┘
   ▼
  ┌──────────────────────┐     ┌──────────────────────┐
  │ Injects Compiled     │ ──► │ Spring Boot OCI      │
  │ Asset to /static/css │     │ Container / App .jar │
  └──────────────────────┘     └──────────────────────┘
```

## 🛠️ Requirements

Before running the generated project, ensure you have the following installed on your machine:
- [Rust](https://rust-lang.org) (to compile/run `tjcli`)
- [Java 25](https://adoptium.net)
- [Node.js (v20 or higher)](https://nodejs.org)
- [Docker Engine](https://docker.com) (only required if building OCI container images)

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
  --node-version "26.1.0" \
  --app-name "thymeleaf-demo-app" \
  --build-image \
  --registry "ghcr.io/my-username" \
  --image-name "custom-app"
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
| `-b` | `--build-image` | `false` | Automatically invoke `./gradlew bootBuildImage` right after scaffolding |
| | `--image-name` | *None* | Overrides the default image name (falls back to lowercased `artifact-id`) |
| | `--registry` | *None* | Target container registry namespace (e.g. `ghcr.io/username` or `docker.io/library`) |

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

## 📦 Production Bundling & Containerization

### Standard JAR Build
When you are ready to package your application for production deployment, run standard Gradle packaging:
```bash
./gradlew build
```
**What happens behind the scenes:** Gradle triggers the node plugin execution cycle, builds the compiled frontend distribution styles, injects minified static assets directly into Spring's `/static/css/tailwind.css` structure, and outputs an executable self-contained package file to `build/libs/thymeleafDemo.jar`.

### OCI Docker Container Compilation
If you didn't pass the `-b` flag during the scaffolding step, you can build a light, secure production Docker image at any time by executing:
```bash
./gradlew bootBuildImage
```
This requires a running local Docker daemon. The image will be compiled via **Cloud Native Buildpacks**, incorporating your custom image name and registry properties baked directly into your generated `build.gradle.kts`.

### 🚀 Automated GitHub Actions CI/CD Containerization Pipeline

The generated workflow (`.github/workflows/ci.yml`) is completely automated. On `pull_request` interactions, it verifies code compiling sanity. When code merges securely into your `main` branch, it automatically initializes build environments, hooks into Docker engines, and invokes Spring Boot's Buildpack compiler to publish directly to your cloud registry.

#### Required Repository Secrets Setup
To let the pipeline authenticate with your registry successfully, navigate to **Settings > Secrets and variables > Actions** in your GitHub repository interface and create the following repository secrets:

1. `REGISTRY_URL`: The domain of your target server host registry platform (e.g., `ghcr.io` for GitHub Packages, or `index.docker.io/v1/` for DockerHub).
2. `REGISTRY_USERNAME`: Your profile username handle or administrative deployment identification token key.
3. `REGISTRY_PASSWORD`: Your secret personal access token (PAT) or secure private API key secret payload.

---

## 🔐 Advanced: Private Registry Authentication

If you configure your application to automatically publish your container image onto private networks or providers (such as GitHub Packages Container Registry `ghcr.io` or Amazon ECR), update your generated `build.gradle.kts` task configuration block to dynamically map authentication tokens from your environment:

```kotlin
tasks.named<org.springframework.boot.gradle.tasks.bundling.BootBuildImage>("bootBuildImage") {
    imageName.set("ghcr.io/my-username/custom-app:latest")
    publish.set(true) // Auto-push to target repository after compilation finishes
    docker {
        publishRegistry {
            username.set(System.getenv("REGISTRY_USERNAME"))
            password.set(System.getenv("REGISTRY_PASSWORD"))
        }
    }
}
```

Expose the environment flags in your console terminal session prior to execution:
```bash
export REGISTRY_USERNAME="your-user-or-token-id"
export REGISTRY_PASSWORD="your-secret-access-token"
./gradlew bootBuildImage
```

---

## 🏃 Running the Compiled OCI Container Locally

Once Cloud Native Buildpacks finishes compilation, verification of your fully containerized production bundle can be initiated instantly inside your local Docker runtime environment.

Expose the target port (mapping host machine port `8080` straight to internal container server specifications) and run your image:

```bash
# General Syntax: docker run -p <HostPort>:<ContainerPort> <RegistryPath>/<ImageName>
docker run -p 8080:8080 ghcr.io/my-username/custom-app:latest
```

If you launched container setup without modifying the defaults (`--registry` or `--image-name`), run:
```bash
docker run -p 8080:8080 docker.io/library/thymeleafdemo:latest
```

---

## 📄 Generated Project Structure

```text
my-app/
├── .github/workflows/ci.yml <-- Automated multi-step Java 25 & Node pipeline
├── build.gradle.kts         <-- Handles plugin tasks and bootBuildImage customization
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
