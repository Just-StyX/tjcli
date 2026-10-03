# tjcli 🚀

A lightning-fast command-line interface tool written in Rust to instantly scaffold a modern, fully configured **Spring Boot 4.1.1 + Thymeleaf + Tailwind CSS v4 + Vite + TypeScript** application. 

It handles all the configuration boilerplate—including setting up Java 25 properties, linking Vite's hot-reload server directly to your Thymeleaf templates, integrating Tailwind v4 dynamically, setting up a standard `.gitignore`, creating a production-ready GitHub Actions CI/CD workflow, configuring Cloud Native Buildpacks for containerization, whitelisting asset delivery paths via a built-in Spring Security layout, and mapping multi-container production topologies using Docker Compose with an integrated PostgreSQL backend database.

---

## ✨ Features

- **Java 25 Ready:** Configures your `build.gradle.kts` toolchain properties out of the box.
- **Tailwind CSS v4 & Vite:** Completely skips legacy `tailwind.config.js` setups, opting for Tailwind v4's high-performance native Vite bundling (`@tailwindcss/vite`).
- **TypeScript First Asset Delivery:** Configures your client-side compilation layers with a native `tsconfig.json` template setup right from the directory root folder.
- **Instant Browser Live-Reload:** Bundles `vite-plugin-live-reload` so edits to your Thymeleaf `.html` pages dynamically refresh your browser window instantly.
- **Unified Build Pipeline:** Automatically links your Gradle resource building phases to explicit Node tasks using strict casting configurations, running production frontend minification scripts gracefully.
- **Automated Bootstrapping:** Runs `npm install` automatically right after file generation using an optimized `--no-audit` pipeline cache to bypass dev-dependency false-positives instantly.
- **🔐 Built-in Security Architecture:** Provisions an out-of-the-box Spring Security filtering web-configuration layer. It locks down sensitive domain paths while ensuring public static delivery routes (`/dist/**`, `/css/**`) remain completely readable by your template loaders.
- **🛡️ Unattended Code Quality Guards:** Instruments a JavaScript-driven modern flat ESLint module configuration (`eslint.config.js`) explicitly wired to track and audit TypeScript syntax before software compilation cycles execute.
- **🐘 Automated Multi-Container Topologies:** Generates a production-ready `compose.yaml` file mapping out an isolated backend PostgreSQL database instance bound alongside your Spring service with automated container healthcheck loops.
- **Automated CI/CD:** Generates a pre-configured GitHub Actions workflow file (`ci.yml`) that triggers strict linting validation checks, executes unit test scopes, and compiles your application automatically.
- **🐳 Built-in OCI Image Support:** Native hooks into Spring Boot's Cloud Native Buildpack system (`./gradlew bootBuildImage`) with customizable names and image registries via CLI flags.

---

## 📐 System Architecture & Workflow

The generated application balances a dual-engine architecture designed for instant frontend feedback during local coding and optimized asset bundling during compilation phases:

```text
  [ Local Coding Mode ]
  ┌──────────────────┐
  │  Vite Dev Server │ ───( Serves Live CSS/TS Modules )───┐
  │  (Port 5173 HMR) │                                     │
  └──────────────────┘                                     ▼
  ┌──────────────────┐                           ┌──────────────────┐
  │   Thymeleaf HTML │ ◄──( Live Reload Trigger )───── │  Web Browser     │
  │   Layout Template│                           │  http://localhost│
  └──────────────────┘                           └──────────────────┘
  ┌──────────────────┐                                     ▲
  │ Spring Boot Dev  │ ───( Emits Securely Parsed HTML )───┘
  │ (Port 8080 Engine│     [Intercepts Custom <vite:vite> tags via ViteDialect]
  └──────────────────┘
           │
           ▼ (Auto-connects via Localhost Overrides)
  ┌──────────────────┐
  │ PostgreSQL DB Container
  │ (Port 5432 Data) │
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
  │ Asset to /static/dist│     │ Container / App .jar │
  └──────────────────────┘     └──────────────────────┘
```

## 🛠️ Requirements

Before running the generated project, ensure you have the following installed on your machine:
- [Rust](https://rust-lang.org) (to compile/run `tjcli`)
- [Java 25](https://adoptium.net)
- [Node.js (v20 or higher)](https://nodejs.org)
- [Docker Engine / Docker Desktop](https://docker.com) (required for database container management and OCI container image creation)

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
  --with-security true \
  --port 8080 \
  --build-image false
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
| `-s` | `--with-security` | `true` | Injects a preconfigured, whitelist-optimized Spring Security filtering layer |
| `-b` | `--build-image` | `true` | Automatically invoke `./gradlew bootBuildImage` right after scaffolding |
| | `--image-name` | *None* | Overrides the default image name (falls back to lowercased `artifact-id`) |
| | `--registry` | *None* | Target container registry namespace (e.g. `ghcr.io/username` or `docker.io/library`) |
## 💻 Local Development Workflow

Once your project is scaffolded, navigate into the directory and spin up your stack components.

### Step 1: Initialize Supporting Infrastructure (Terminal 1)
Boot up your decoupled PostgreSQL storage volume stack container in detached background execution mode:
```bash
docker compose up postgres-db -d
```

### Step 2: Start the Frontend Watcher (Terminal 2)
This boots up the Vite HMR asset compiler at `http://localhost:5173` and tracks live saves inside your client-side assets:
```bash
npm run dev
```

### Step 3: Boot Up the Spring Application (Terminal 3)
Run the backend web container. To map into development mode asset endpoints and activate template live-reloading, explicitly pass the matching runtime profile argument flag:
```bash
./gradlew bootRun --args='--spring.profiles.active=dev'
```
Open your browser to **`http://localhost:8080`**. You will be greeted by the secure Spring Login layout screen. Enter the default scaffolding credentials:
- **Username:** `admin`
- **Password:** `password`

Once logged in, the custom Java `ViteDialect` engine will intercept the custom `<vite:client>` and `<vite:vite>` layout blocks to bridge your live TypeScript compilation modules cleanly. Your session identity tokens are passed down securely from the controller directly into your Thymeleaf view templates.

---

## 📦 Production Bundling & Containerization

### Standard JAR Build
When you are ready to package your application for production deployment, run standard Gradle packaging:
```bash
./gradlew build
```
**What happens behind the scenes:** Gradle casts tasks into the explicit `com.github.node-gradle.node` runtime graph. This invokes `npmInstall` safely, triggers your ESLint verification checks, processes the minified Vite compilation, bundles output styles to `src/main/resources/static/dist`, and outputs a self-contained package archive file to `build/libs/thymeleafDemo.jar`.

### Full Ecosystem Containerization Orchestration
To simulate your full cloud architecture locally, compile your application container via Cloud Native Buildpacks first:
```bash
./gradlew bootBuildImage
```
Once completed, launch your full production layout topology. Your application container will automatically wait for the PostgreSQL container to complete its health check loop before launching, overriding the internal `application.yml` profile database configurations cleanly via runtime docker variables:
```bash
docker compose up
```

### 🚀 Automated GitHub Actions CI/CD Containerization Pipeline

The generated workflow (`.github/workflows/ci.yml`) is completely automated. 

On `pull_request` interactions, it initializes a clean runner environment, boots up Node and Java toolchains, installs packages, runs an **unattended ESLint code linting structural validation check** against your code assets, and completes a full backend compilation sanity run via `./gradlew test`.

When code merges securely into your `main` branch, it executes your validation checks, flags down the container registry credentials, bypasses duplicate compilation tasks using the optimized `-x test` execution flag, and builds and publishes your image onto your remote environment registry infrastructure instantly.

#### Required Repository Secrets Setup
To let the pipeline authenticate with your registry successfully, navigate to **Settings > Secrets and variables > Actions** in your GitHub repository interface and create the following repository secrets:

1. `REGISTRY_URL`: The domain of your target server host registry platform (e.g., `ghcr.io` for GitHub Packages, or `index.docker.io/v1/` for DockerHub).
2. `REGISTRY_USERNAME`: Your profile username handle or administrative deployment identification token key.
3. `REGISTRY_PASSWORD`: Your secret personal access token (PAT) or secure private API key secret payload.

---

## 🔐 Advanced: Private Registry Authentication

If you configure your application to automatically publish your container image onto private networks or providers (such as GitHub Packages Container Registry `ghcr.io` or Amazon ECR), your generated `build.gradle.kts` task configuration block is already pre-configured to dynamically map authentication tokens from your environment:

```kotlin
tasks.named<org.springframework.boot.gradle.tasks.bundling.BootBuildImage>("bootBuildImage") {
    imageName.set("ghcr.io/my-username/custom-app:latest")
    publish.set(true) // Auto-push to target repository after compilation finishes
    docker {
        publishRegistry {
            username.set(System.getenv("REGISTRY_USERNAME") ?: "")
            password.set(System.getenv("REGISTRY_PASSWORD") ?: "")
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

## 🏃 Running the Full Multi-Container Stack Locally

Because your scaffolding engine packages a full application topology alongside an isolated database, you do not need to run manual `docker run` loops. You can spin up the entire production-ready environment—with your Spring Boot app server safely waiting for your PostgreSQL storage layers to complete health checks—using a single command:

```bash
docker compose up --build
```

### Manual Individual Container Verification (Fallback)
If you want to spin up your compiled Spring application independently inside your local Docker engine and bypass the Compose routing layout, map your configured application port and supply your database credentials as inline runtime overrides:

```bash
# General Syntax: docker run -p <HostPort>:<ContainerPort> -e <EnvOverrides> <ImageName>
docker run -p 8080:8080 \
  -e SPRING_DATASOURCE_URL=jdbc:postgresql://host.docker.internal:5432/appdb \
  -e SPRING_DATASOURCE_USERNAME=devuser \
  -e SPRING_DATASOURCE_PASSWORD=devpassword \
  docker.io/library/thymeleafdemo:latest
```

---

## 📄 Generated Project Structure

Your CLI tool creates a beautifully clean, unified workspace structure that groups your Java backend package layers and modern TypeScript frontend compilers into a single, cohesive repository footprint:

```text
my-app/
├── .github/workflows/ci.yml   <-- Automated Linting, Java 25 & Node pipeline
├── build.gradle.kts           <-- Chained Node-Gradle task graph configuration
├── settings.gradle.kts
├── compose.yaml               <-- Production multi-container PostgreSQL architecture
├── package.json               <-- Vite + Tailwind CSS v4 + Overrides security maps
├── vite.config.js             <-- Handles HTML live-reloads and CORS port origins
├── eslint.config.js           <-- Modern Flat JS config auditing TypeScript rules
├── tsconfig.json              <-- TypeScript target compilation options file
├── .gitignore                 <-- Clears target/, dist/, .vite/, and node_modules/
└── src/
    ├── main/
    │   ├── java/jsl/group/thymeleafdemo/
    │   │   ├── ThymeleafDemoApplication.java <-- Spring standard bootstrapper
    │   │   ├── HomeController.java           <-- Injects Auth profile context tokens
    │   │   ├── ViteWebConfig.java            <-- Context profile interceptor beans
    │   │   ├── ViteDialect.java              <-- Custom Thymeleaf tag namespace definition
    │   │   ├── ViteClientTagProcessor.java   <-- Processes development <vite:client> loops
    │   │   ├── ViteTagProcessor.java         <-- Processes production <vite:vite> links
    │   │   └── SecurityConfig.java           <-- Whitelists static asset delivery paths
    │   └── resources/
    │       ├── application.yml               <-- Database parameters & profile fallbacks
    │       ├── static/
    │       │   ├── css/input.css             <-- Tailwind v4 standard entrypoint
    │       │   └── js/main.ts                <-- TypeScript core client entry file
    │       └── templates/index.html          <-- Secured template dashboard layout
    └── test/
        └── java/jsl/group/thymeleafdemo/
            └── ThymeleafDemoApplicationTests.java <-- Automated application health checker
```

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
