use std::error::Error;
use std::fs::{create_dir_all, write};
use std::path::Path;
use std::process::Command;

use clap::Parser;

pub type GenericError = Box<dyn Error + Send + Sync>;
pub type TclResult<T> = Result<T, GenericError>;

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
    pub artifact_id: String,

    #[arg(short = 'g', long = "group-id", default_value = "jsl.group")]
    pub group_id: String,

    #[arg(short = 'j', long = "java-version", default_value = "25")]
    pub java_version: String,

    #[arg(short = 'n', long = "app-name", default_value = "thymeleaf-demo-app")]
    pub app_name: String,

    #[arg(short = 'o', long = "node-version", default_value = "26.1.0")]
    pub node_version: String,

    #[arg(short = 'p', long = "port", default_value_t = 8080)]
    pub port: u16,

    #[arg(short = 'b', long = "build-image", default_value_t = true)]
    pub build_image: bool,

    #[arg(long = "image-name")]
    pub image_name: Option<String>,

    /// Specific container registry domain prefix or namespace (e.g. "ghcr.io/my-username" or "docker.io/library")
    #[arg(long = "registry")]
    pub registry: Option<String>,

    #[arg(short = 's', long = "with-security", default_value_t = true)]
    pub with_security: bool,
}

pub fn run(args: TclConfig) -> TclResult<()> {
    print_banner(&args.app_name);
    let root = Path::new(&args.artifact_id);
    println!(
        "🚀 Scaffolding Spring Boot app: '{}' inside folder '{}'...",
        args.app_name, args.artifact_id
    );
    
    // 1. Convert Group ID & Artifact ID package structure to safe paths
    let package_clean = args.group_id.replace('.', "/") + "/" + &args.artifact_id.to_lowercase();
    
    // Main Source Directories
    let java_package_path = root.join("src/main/java").join(&package_clean);
    let resources_css_path = root.join("src/main/resources/static/css");
    // Added explicit directory creation for frontend TypeScript/JavaScript code assets
    let resources_js_path = root.join("src/main/resources/static/js");
    let resources_templates_path = root.join("src/main/resources/templates");
    let github_workflow_path = root.join(".github/workflows");

    // Test Directories
    let test_package_path = root.join("src/test/java").join(&package_clean);
    let test_resources_path = root.join("src/test/resources");

    // Create all directory structures
    create_dir_all(&java_package_path)?;
    create_dir_all(&resources_css_path)?;
    create_dir_all(&resources_js_path)?; // Guaranteed presence for ESLint verification checks
    create_dir_all(&resources_templates_path)?;
    create_dir_all(&github_workflow_path)?;
    create_dir_all(&test_package_path)?;
    create_dir_all(&test_resources_path)?;

    // 2. Build Tools Configuration
    write(
        root.join("build.gradle.kts"),
        get_gradle_config(
            &args.group_id,
            &args.artifact_id,
            &args.java_version,
            &args.node_version,
            args.image_name.as_deref(),
            args.registry.as_deref(),
            args.with_security,
        ),
    )?;
    write(
        root.join("settings.gradle.kts"),
        format!("rootProject.name = \"{}\"\n", args.artifact_id),
    )?;
    write(root.join("package.json"), get_package_json())?;
    write(root.join("vite.config.js"), get_vite_config())?;
    write(root.join(".gitignore"), get_gitignore())?;
    write(root.join("eslint.config.js"), get_eslint_config())?;
    // Added local TypeScript orchestration mapping file
    write(root.join("tsconfig.json"), get_tsconfig_config())?;
    write(
        root.join("compose.yaml"), 
        get_docker_compose_config(&args.app_name, args.port)
    )?;


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
    write(
        java_package_path.join("ViteDialect.java"),
        get_vite_dialect_class(&args.group_id, &args.artifact_id),
    )?;
    // write(
    //     java_package_path.join("ViteClientTagProcessor.java"),
    //     get_vite_client_processor_class(&args.group_id, &args.artifact_id),
    // )?;
    write(
        java_package_path.join("ViteTagProcessor.java"),
        get_vite_tag_processor_class(&args.group_id, &args.artifact_id),
    )?;
    
    // Pass the safety flag straight down to isolate class dependencies
    write(
        java_package_path.join("ViteWebConfig.java"),
        get_vite_config_class(&args.group_id, &args.artifact_id, args.with_security),
    )?;

    if args.with_security {
        println!("🔒 Injecting default Spring Security Authentication Layer configurations...");
        write(
            java_package_path.join("SecurityConfig.java"),
            get_security_config_class(&args.group_id, &args.artifact_id),
        )?;
    }

    // 4. Spring Test Layer Generation 🧪
    write(
        test_package_path.join(format!(
            "{}ApplicationTests.java",
            uppercase_first(&args.artifact_id)
        )),
        get_application_test_class(&args.group_id, &args.artifact_id),
    )?;

    // 5. Stylesheets & UI Entry Point
    write(
        resources_css_path.join("input.css"),
        r#"@import "tailwindcss";
        @source "../../../../../templates/**/*.html";
        "#,
    )?;
    // Create a dummy TypeScript entry point to keep compiler tasks clean out of the box
    write(
        resources_js_path.join("main.ts"),
        "console.log('Vite TypeScript Context Active');\n",
    )?;
    write(
        resources_templates_path.join("index.html"),
        get_index_html(),
    )?;

    // 6. Native Github Actions Pipeline Generation
    write(
        github_workflow_path.join("ci.yml"),
        get_github_workflow(&args.java_version, &args.node_version),
    )?;

    // 7. Automatically trigger internal system dependencies bootstrap pipeline
    println!("📦 Triggering 'npm install' inside project environment...");
    let npm_status = Command::new("npm")
        .arg("install")
        .arg("--loglevel=error")
        .arg("--no-audit") 
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

    // 8. Conditional Spring Boot OCI Docker Image Generation
    if args.build_image {
        println!("🐳 Request detected to compile application into an OCI container...");

        if !root.join("gradlew").exists() {
            println!("⚙️ Local Gradle wrapper missing. Attempting to initialize with host 'gradle wrapper' setup...");
            let _ = Command::new("gradle")
                .arg("wrapper")
                .current_dir(root)
                .status();
        }

        let gradle_executable = if cfg!(target_os = "windows") { "gradlew.bat" } else { "./gradlew" };

        println!("🛠️  Executing Spring Boot Buildpack Image construction system (this may take a few minutes)...");
        let boot_build_status = Command::new(gradle_executable)
            .arg("bootBuildImage")
            .current_dir(root)
            .status();

        match boot_build_status {
            Ok(status) if status.success() => {
                println!("✅ OCI Container Image successfully provisioned inside Docker engine environment.")
            }
            _ => println!(
                "❌ Execution failure during 'bootBuildImage'. Verify that Docker daemon engine is actively running."
            ),
        }
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
    name: {app_name}
  
  # 1. Database Connection Engine Setup
  datasource:
    url: ${{SPRING_DATASOURCE_URL:jdbc:postgresql://localhost:5432/appdb}}
    username: ${{SPRING_DATASOURCE_USERNAME:devuser}}
    password: ${{SPRING_DATASOURCE_PASSWORD:devpassword}}
    driver-class-name: org.postgresql.Driver

  # 2. Hibernate / JPA Object Relational Mapping Setup
  jpa:
    database-platform: org.hibernate.dialect.PostgreSQLDialect
    hibernate:
      ddl-auto: update # Automatically safely creates tables based on Java Entity definitions
    properties:
      hibernate:
        format_sql: true
    show-sql: true

  # 3. Dynamic Templating Architecture Setup
  thymeleaf:
    cache: false
    
  # 4. Local Development Environment Utility Boosters
  devtools:
    restart:
      enabled: true
    livereload:
      enabled: true

server:
  port: {port}
"#,
        app_name = app_name,
        port = port
    )
}

fn get_application_test_class(group_id: &str, artifact_id: &str) -> String {
    let package_name = format!("{}.{}", group_id, artifact_id.to_lowercase());
    let test_class_name = format!("{}ApplicationTests", uppercase_first(artifact_id));
    
    format!(
        r#"package {};

import org.junit.jupiter.api.Test;
import org.springframework.boot.test.context.SpringBootTest;

@SpringBootTest
class {} {{

    @Test
    void contextLoads() {{
        // Basic sanity check to ensure Spring context loads properly
    }}

}}
"#,
        package_name, test_class_name
    )
}

fn get_vite_dialect_class(group_id: &str, artifact_id: &str) -> String {
    let package_name = format!("{}.{}", group_id, artifact_id.to_lowercase());
    format!(
        r#"package {};

import org.thymeleaf.dialect.AbstractProcessorDialect;
import org.thymeleaf.processor.IProcessor;
import java.util.HashSet;
import java.util.Set;

public class ViteDialect extends AbstractProcessorDialect {{

    public ViteDialect() {{
        super("Vite Dialect", "vite", 1000);
    }}

    @Override
    public Set<IProcessor> getProcessors(String dialectPrefix) {{
        Set<IProcessor> processors = new HashSet<>();
        processors.add(new ViteTagProcessor(dialectPrefix));
        return processors;
    }}
}}
"#,
        package_name
    )
}

fn _get_vite_client_processor_class(group_id: &str, artifact_id: &str) -> String {
    let package_name = format!("{}.{}", group_id, artifact_id.to_lowercase());
    format!(
        r#"package {};

import org.thymeleaf.context.ITemplateContext;
import org.thymeleaf.model.IModel;
import org.thymeleaf.model.IModelFactory;
import org.thymeleaf.processor.element.AbstractElementModelProcessor;
import org.thymeleaf.processor.element.IElementModelStructureHandler;
import org.thymeleaf.templatemode.TemplateMode;

public class ViteClientTagProcessor extends AbstractElementModelProcessor {{

    public ViteClientTagProcessor(String dialectPrefix) {{
        super(TemplateMode.HTML, dialectPrefix, "client", true, null, false, 1000);
    }}

    @Override
    protected void doProcess(ITemplateContext context, IModel model, IElementModelStructureHandler structureHandler) {{
        IModelFactory modelFactory = context.getModelFactory();
        IModel lines = modelFactory.createModel();

        boolean isDev = context.getVariable("activeProfile") != null && context.getVariable("activeProfile").equals("dev");

        if (isDev) {{
            lines.add(modelFactory.createOpenElementTag("script", "type", "module"));
            lines.add(modelFactory.createOpenElementTag("script", "src", "http://localhost:5173/@vite/client"));
            lines.add(modelFactory.createCloseElementTag("script"));
            
            // ✅ Safely inserts model structures using standard, stable structural body mutators
            model.reset();
            model.addModel(lines);
        }} else {{
            // Clears development tags completely out of production code
            model.reset();
        }}
    }}
}}
"#,
        package_name
    )
}

fn get_vite_tag_processor_class(group_id: &str, artifact_id: &str) -> String {
    let package_name = format!("{}.{}", group_id, artifact_id.to_lowercase());
    format!(
        r#"package {package_name};

import org.thymeleaf.context.ITemplateContext;
import org.thymeleaf.engine.AttributeName;
import org.thymeleaf.model.IModel;
import org.thymeleaf.model.IModelFactory;
import org.thymeleaf.model.IProcessableElementTag;
import org.thymeleaf.processor.element.AbstractAttributeTagProcessor;
import org.thymeleaf.processor.element.IElementTagStructureHandler;
import org.thymeleaf.templatemode.TemplateMode;

public class ViteTagProcessor extends AbstractAttributeTagProcessor {{

    public ViteTagProcessor(String dialectPrefix) {{
        // Set the final argument to 'false' so it processes the whole element rather than just its children
        super(TemplateMode.HTML, dialectPrefix, null, false, "inject", true, 1000, false);
    }}

    @Override
    protected void doProcess(ITemplateContext context, IProcessableElementTag tag, 
                             AttributeName attributeName, String attributeValue, 
                             IElementTagStructureHandler structureHandler) {{
        
        IModelFactory modelFactory = context.getModelFactory();
        IModel generatedHtml = modelFactory.createModel();

        boolean isDev = context.getVariable("activeProfile") != null && context.getVariable("activeProfile").equals("dev");

        if (isDev) {{
            // 1. Inject standalone live Vite HMR websocket client script tag
            generatedHtml.add(modelFactory.createOpenElementTag("script", "type", "module"));
            generatedHtml.add(modelFactory.createOpenElementTag("script", "src", "http://localhost:5173/@vite/client"));
            generatedHtml.add(modelFactory.createCloseElementTag("script"));

            // 2. Inject standalone development style entry point
            generatedHtml.add(modelFactory.createOpenElementTag("link", "rel", "stylesheet"));
            generatedHtml.add(modelFactory.createOpenElementTag("link", "href", "http://localhost:5173/src/main/resources/static" + attributeValue.replace("css/tailwind.css", "css/input.css")));
        }} else {{
            // 3. Fall back cleanly to production minified bundled styles inside static/dist
            generatedHtml.add(modelFactory.createOpenElementTag("link", "rel", "stylesheet"));
            generatedHtml.add(modelFactory.createOpenElementTag("link", "href", "/dist/css/tailwind.css"));
        }}

        // ✅ Atomic Fix: Replaces the entire host script tag placeholder with our pristine generated model array
        structureHandler.replaceWith(generatedHtml, false);
    }}
}}
"#,
        package_name = package_name
    )
}


fn get_vite_config_class(group_id: &str, artifact_id: &str, with_security: bool) -> String {
    let package_name = format!("{}.{}", group_id, artifact_id.to_lowercase());
    
    // Conditionally write out the ControllerAdvice block based on security configurations
    let advice_block = if with_security {
        r#"    @org.springframework.web.bind.annotation.ControllerAdvice
    public static class GlobalAttributesAdvice {
        private final org.springframework.core.env.Environment environment;

        public GlobalAttributesAdvice(org.springframework.core.env.Environment environment) {
            this.environment = environment;
        }

        @org.springframework.web.bind.annotation.ModelAttribute("activeProfile")
        public String activeProfile() {
            return java.util.Arrays.stream(environment.getActiveProfiles())
                    .filter(profile -> "dev".equals(profile))
                    .findFirst()
                    .orElse("prod");
        }
    }"#
    } else {
        r#"    @org.springframework.web.bind.annotation.ControllerAdvice
    public static class GlobalAttributesAdvice {
        @org.springframework.web.bind.annotation.ModelAttribute("activeProfile")
        public String activeProfile() {
            return "prod"; // Bypasses hot checks gracefully if security layer is deactivated
        }
    }"#
    };

    format!(
        r#"package {};

import org.springframework.context.annotation.Configuration;

@Configuration
public class ViteWebConfig {{
    // Removed ViteDialect bean registration to let native Thymeleaf expressions handle asset routing
{}
}}
"#,
        package_name,
        advice_block
    )
}


fn get_tsconfig_config() -> &'static str {
    r#"{
  "compilerOptions": {
    "target": "ES2022",
    "module": "NodeNext",
    "moduleResolution": "NodeNext",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": ["src/main/resources/static/js/**/*.ts", "eslint.config.js", "vite.config.js"]
}
"#
}

fn get_security_config_class(group_id: &str, artifact_id: &str) -> String {
    let package_name = format!("{}.{}", group_id, artifact_id.to_lowercase());
    format!(
        r#"package {};

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.core.userdetails.User;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.provisioning.InMemoryUserDetailsManager;
import org.springframework.security.web.SecurityFilterChain;

@Configuration
@EnableWebSecurity
public class SecurityConfig {{

    @Bean
    public SecurityFilterChain securityFilterChain(HttpSecurity http) throws Exception {{
        http
            .authorizeHttpRequests(authorize -> authorize
                // Crucial: Whitelist static assets and compiled Vite bundles
                .requestMatchers("/css/**", "/js/**", "/dist/**", "/favicon.ico").permitAll()
                .anyRequest().authenticated()
            )
            .formLogin(form -> form
                .permitAll()
            )
            .logout(logout -> logout
                .permitAll()
            );

        return http.build();
    }}

    @Bean
    public UserDetailsService userDetailsService() {{
        UserDetails user = User.withDefaultPasswordEncoder()
            .username("admin")
            .password("password")
            .roles("USER")
            .build();
        return new InMemoryUserDetailsManager(user);
    }}
}}
"#,
        package_name
    )
}

fn get_gradle_config(
    group_id: &str,
    artifact_id: &str,
    java_version: &str,
    node_version: &str,
    custom_image_name: Option<&str>,
    registry: Option<&str>,
    with_security: bool,
) -> String {
    let computed_image_name = match (registry, custom_image_name) {
        (Some(reg), Some(name)) => format!("{}/{}", reg.trim_end_matches('/'), name),
        (Some(reg), None) => format!("{}/{}", reg.trim_end_matches('/'), artifact_id.to_lowercase()),
        (None, Some(name)) => name.to_string(),
        (None, None) => format!("docker.io/library/{}:latest", artifact_id.to_lowercase()),
    };

    let should_publish = registry.is_some();
    
    let security_dependency = if with_security {
        "    implementation(\"org.springframework.boot:spring-boot-starter-security\")"
    } else {
        ""
    };

    format!(
            r#"plugins {{
        java
        id("org.springframework.boot") version "4.1.1"
        id("io.spring.dependency-management") version "1.1.7"
        id("com.github.node-gradle.node") version "7.0.2"
    }}
    
    group = "{group_id}"
    version = "0.0.1-SNAPSHOT"
    
    java {{
        toolchain {{
            languageVersion.set(JavaLanguageVersion.of({java_version}))
        }}
    }}
    
    repositories {{
        mavenCentral()
    }}
    
    dependencies {{
        implementation("org.springframework.boot:spring-boot-starter-thymeleaf")
        implementation("org.springframework.boot:spring-boot-starter-web")
    {security_dependency}
        developmentOnly("org.springframework.boot:spring-boot-devtools")
        implementation("org.springframework.boot:spring-boot-starter-data-jpa")
        runtimeOnly("org.postgresql:postgresql")
        testImplementation("org.springframework.boot:spring-boot-starter-test")
    }}
    
    node {{
        version.set("{node_version}")
        download.set(true)
    }}
    
    tasks.named<org.springframework.boot.gradle.tasks.bundling.BootJar>("bootJar") {{
        archiveFileName.set("{artifact_id}.jar")
    }}
    
    tasks.named<org.springframework.boot.gradle.tasks.bundling.BootBuildImage>("bootBuildImage") {{
        imageName.set("{computed_image_name}")
        publish.set({should_publish})
        docker {{
            publishRegistry {{
                username.set(System.getenv("REGISTRY_USERNAME") ?: "")
                password.set(System.getenv("REGISTRY_PASSWORD") ?: "")
            }}
        }}
    }}
    
    tasks.named<com.github.gradle.node.npm.task.NpmTask>("npm_run_build") {{
        dependsOn(tasks.named("npmInstall"))
        inputs.dir("src/main/resources/static/css")
        inputs.file("vite.config.js")
        inputs.file("package.json")
        outputs.dir("src/main/resources/static/dist")
    }}
    
    tasks.named<ProcessResources>("processResources") {{
        dependsOn(tasks.named("npm_run_build"))
    }}
    "#,
            group_id = group_id,
            java_version = java_version,
            node_version = node_version,
            artifact_id = artifact_id,
            computed_image_name = computed_image_name,
            should_publish = should_publish,
            security_dependency = security_dependency
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

import org.springframework.security.core.Authentication;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;

@Controller
public class HomeController {{

    @GetMapping("/")
    public String index(Model model, Authentication authentication) {{
        if (authentication != null && authentication.isAuthenticated()) {{
            model.addAttribute("username", authentication.getName());
            model.addAttribute("roles", authentication.getAuthorities().toString());
        }} else {{
            model.addAttribute("username", "Anonymous");
            model.addAttribute("roles", "None");
        }}
        return "index";
    }}
}}
"#,
        group_id,
        artifact_id.to_lowercase()
    )
}

fn get_github_workflow(java_version: &str, node_version: &str) -> String {
    let major_node = node_version.split('.').next().unwrap_or("26");
    format!(
        r#"name: Java CI/CD DevOps Pipeline

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

jobs:
  build-and-test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4

    - name: Set up JDK {java_version}
      uses: actions/setup-java@v4
      with:
        java-version: '{java_version}'
        distribution: 'temurin'
        cache: 'gradle'

    - name: Set up Node.js {major_node}
      uses: actions/setup-node@v4
      with:
        node-version: '{major_node}'
        cache: 'npm'

    - name: Install Project Dependencies
      run: npm install

    - name: Execute Unattended Code Linting Validation
      # ✅ Updated: Triggers your unified TypeScript rules via package.json script
      run: npm run lint

    - name: Grant execute permission for gradlew
      run: chmod +x gradlew

    - name: Execute Project Test Suite
      run: ./gradlew test

    - name: Set up Docker Buildx
      if: github.ref == 'refs/heads/main' && github.event_name == 'push'
      uses: docker/setup-buildx-action@v3

    - name: Authenticate Container Registry
      if: github.ref == 'refs/heads/main' && github.event_name == 'push'
      uses: docker/login-action@v3
      with:
        registry: ${{ secrets.REGISTRY_URL || 'docker.io' }}
        username: ${{ secrets.REGISTRY_USERNAME }}
        password:  ${{ secrets.REGISTRY_PASSWORD }}

    - name: Publish Production OCI Buildpack Image
      if: github.ref == 'refs/heads/main' && github.event_name == 'push'
      env:
        REGISTRY_USERNAME: ${{ secrets.REGISTRY_USERNAME }}
        REGISTRY_PASSWORD: ${{ secrets.REGISTRY_PASSWORD }}
      run: ./gradlew bootBuildImage -x test
"#,
        java_version = java_version,
        major_node = major_node
    )
}

fn get_docker_compose_config(app_name: &str, port: u16) -> String {
    format!(
        r#"services:
  # 1. Backing Relational Database Service Engine
  postgres-db:
    image: postgres:17-alpine
    container_name: {app_name}-database
    environment:
      POSTGRES_USER: devuser
      POSTGRES_PASSWORD: devpassword
      POSTGRES_DB: appdb
    ports:
      - "5432:5432"
    volumes:
      - pgdata:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U devuser -d appdb"]
      interval: 5s
      timeout: 5s
      retries: 5

  # 2. Main Spring Boot Application Server Container Layer
  app-server:
    image: docker.io/library/{app_name}:latest
    container_name: {app_name}-server
    build:
      context: .
    environment:
      SPRING_PROFILES_ACTIVE: prod
      SERVER_PORT: "{port}"
      SPRING_DATASOURCE_URL: jdbc:postgresql://postgres-db:5432/appdb
      SPRING_DATASOURCE_USERNAME: devuser
      SPRING_DATASOURCE_PASSWORD: devpassword
    ports:
      - "{port}:{port}"
    depends_on:
      postgres-db:
        condition: service_healthy

volumes:
  pgdata:
    name: {app_name}-postgres-volume
"#,
        app_name = app_name.to_lowercase(),
        port = port
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
  server: {
    // Allows Spring Boot on port 8080 to fetch assets from Vite securely
    cors: true,
    strictPort: true,
    origin: 'http://localhost:5173'
  },
  build: {
    // Outputs production assets into a sub-folder to keep static directory clean
    outDir: './src/main/resources/static/dist',
    emptyOutDir: true,
    manifest: true, // Optional: useful if your backend reads Vite manifests
    rollupOptions: {
      input: './src/main/resources/static/css/input.css',
      output: {
        // Keeps files cleanly structured inside the distribution folder
        entryFileNames: 'js/[name].[hash].js',
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
    "build": "vite build",
    "lint": "eslint src/main/resources/static/js/*"
  },
  "devDependencies": {
    "@tailwindcss/vite": "^4.0.0",
    "@types/eslint__js": "^8.42.3",
    "eslint": "^9.17.0",
    "tailwindcss": "^4.0.0",
    "typescript": "^5.7.0",
    "typescript-eslint": "^8.18.0",
    "vite": "^6.0.0",
    "vite-plugin-live-reload": "^3.0.2"
  }
}"#
}

fn get_index_html() -> &'static str {
    r#"<!DOCTYPE html>
<html xmlns:th="http://thymeleaf.org" lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Spring Boot + Tailwind v4</title>
    
    <!-- 1. LOCAL DEVELOPMENT PROFILE ASSET MAPS -->
    <th:block th:if="${activeProfile == 'dev'}">
        <script type="module" src="http://localhost:5173/@vite/client"></script>
        <link rel="stylesheet" href="http://localhost:5173/src/main/resources/static/css/input.css">
    </th:block>

    <!-- 2. PRODUCTION ARCHITECTURE STATIC PACKAGES -->
    <th:block th:unless="${activeProfile == 'dev'}">
        <link rel="stylesheet" th:href="@{/dist/css/tailwind.css}">
    </th:block>
</head>
<body class="bg-zinc-950 text-zinc-50 flex items-center justify-center min-h-screen">
    <div class="p-8 bg-zinc-900 border border-zinc-800 rounded-3xl text-center shadow-2xl max-w-md w-full space-y-6">
        <div>
            <h1 class="text-4xl font-extrabold text-emerald-400">Spring Boot Secured</h1>
            <p class="mt-2 text-zinc-400">Thymeleaf + Vite + Tailwind v4 is live.</p>
        </div>

        <div class="p-4 bg-zinc-950 border border-zinc-800 rounded-2xl text-left space-y-2">
            <div class="flex justify-between items-center">
                <span class="text-sm font-medium text-zinc-500">Current User:</span>
                <span class="text-sm font-bold text-teal-400" th:text="${username}">Admin</span>
            </div>
            <div class="flex justify-between items-center">
                <span class="text-sm font-medium text-zinc-500">Assigned Roles:</span>
                <span class="text-xs font-mono bg-zinc-800 px-2 py-0.5 rounded text-zinc-300" th:text="${roles}">[ROLE_USER]</span>
            </div>
        </div>

        <form th:action="@{/logout}" method="post" class="w-full">
            <button type="submit" class="w-full bg-zinc-800 hover:bg-zinc-700 active:bg-zinc-900 text-zinc-200 text-sm font-semibold py-2.5 px-4 rounded-xl transition duration-200 cursor-pointer">
                Log Out Secure Session
            </button>
        </form>
    </div>
</body>
</html>
"#
}

fn get_eslint_config() -> &'static str {
    r#"import js from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";

export default tseslint.config(
    js.configs.recommended,
    ...tseslint.configs.recommended,
    {
        // 1. Tell ESLint to check both standard JavaScript and TypeScript files
        files: ["**/*.{js,ts}"],
        languageOptions: {
            ecmaVersion: "latest",
            sourceType: "module",
            globals: {
                ...globals.browser,
                ...globals.node
            }
        },
        rules: {
            "no-unused-vars": "warn",
            "no-console": "off",
            // You can add TypeScript-specific rule modifications here
            "@typescript-eslint/no-explicit-any": "warn"
        }
    }
);
"#
}

fn get_gitignore() -> &'static str {
    r#"# 1. Node / Frontend Toolchain Cache Repositories
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*
.pnpm-debug.log*
.vite/

# 2. Java / Spring / Gradle Build and Distribution Artifacts
build/
bin/
out/
target/
.gradle/

# 🛡️ CRITICAL DEVOPS SECURITY HOOKS: Ignore dynamically compiled asset bundles
src/main/resources/static/dist/

# 🐘 Local Docker Multi-Container Volume Storage Mappings
.pgdata/
pgdata/

# 3. IDE and Integrated Code Editors Workspace Configs
.idea/
*.iml
*.ipr
*.iws
.vscode/
.settings/
.project
.classpath

# 4. Operating System Background Cache Overheads
.DS_Store
Thumbs.db
"#
}

pub fn print_banner(app_name: &str) {
    let cyan = "\x1b[36m";
    let green = "\x1b[32m";
    let yellow = "\x1b[33m";
    let reset = "\x1b[0m";
    let bold = "\x1b[1m";

    println!(
        r#"{cyan}{bold}
  _____    _  ____   _       ___ 

 |_   _|  | |/ ___| | |     |_ _|
   | | _  | | |     | |      | | 
   | || |_| | |___  | |___   | | 
   |_| \___/ \____| |_____| |___|
                                 {reset}"#,
        cyan = cyan,
        bold = bold,
        reset = reset
    );
    println!(
        "{green}{bold}  ✨ Thymeleaf + Java Spring + Tailwind v4 + Vite Scaffold {reset}",
        green = green,
        bold = bold,
        reset = reset
    );
    println!(
        "  {yellow}===================================================={reset}",
        yellow = yellow,
        reset = reset
    );
    println!("  🚀 Preparing deployment matrix for target: {bold}'{}'{reset}\n", app_name, bold = bold, reset = reset);
}
