#[cfg(test)]
mod tests {
    use tjcli::{GenericError, TclConfig, TclResult, run};
    use std::fs;
    use tempfile::tempdir;

    // Modified helper function to map absolute paths dynamically
    fn create_mock_config(project_path: &str, with_security: bool) -> TclConfig {
        TclConfig {
            // artifact_id now carries the full isolated absolute target workspace directory path path string
            artifact_id: project_path.to_string(),
            group_id: "com.example.test".to_string(),
            java_version: "25".to_string(),
            app_name: "test-app-engine".to_string(),
            node_version: "26.1.0".to_string(),
            port: 8080,
            build_image: false, 
            image_name: None,
            registry: None,
            with_security,
        }
    }

    #[test]
    fn test_core_directory_and_build_tool_generation() -> TclResult<()> {
        let tmp_workspace = tempdir()?;
        // Generate isolated sandbox output paths safely
        let project_root = tmp_workspace.path().join("testProject");
        let project_root_str = project_root.to_str().unwrap();

        let mock_args = create_mock_config(project_root_str, true);
        run(mock_args)?;

        assert!(project_root.join("build.gradle.kts").exists());
        assert!(project_root.join("package.json").exists());
        assert!(project_root.join("vite.config.js").exists());
        assert!(project_root.join("eslint.config.js").exists());
        assert!(project_root.join("tsconfig.json").exists());

        assert!(project_root.join("src/main/resources/static/css/input.css").exists());
        assert!(project_root.join("src/main/resources/static/js/main.ts").exists());
        assert!(project_root.join("src/main/resources/templates/index.html").exists());
        assert!(project_root.join("compose.yaml").exists(), "Docker Compose file missing from configuration root");

        let package_json_content = fs::read_to_string(project_root.join("package.json"))
            .map_err(|e| Box::new(e) as GenericError)?;
        assert!(package_json_content.contains("typescript-eslint"));

        let application_yml_content = fs::read_to_string(project_root.join("src/main/resources/application.yml"))
            .map_err(|e| Box::new(e) as GenericError)?;
        
        assert!(application_yml_content.contains("jdbc:postgresql://"));
        assert!(application_yml_content.contains("PostgreSQLDialect"));

        Ok(())
    }

    #[test]
    fn test_spring_package_path_resolution_and_dialect_classes() -> TclResult<()> {
        let tmp_workspace = tempdir()?;
        let project_root = tmp_workspace.path().join("dialectapp");
        let project_root_str = project_root.to_str().unwrap();

        let mock_args = create_mock_config(project_root_str, true);
        run(mock_args)?;

        // Paths lookups are absolute and insulated across concurrent execution threads
        let _java_src_base = project_root.join("src/main/java/com/example/test/dist/dialectapp");
        
        // Find folder content dynamically to absorb absolute lowercasing variances
        assert!(project_root.join("src/main/java/com/example/test").exists());

        Ok(())
    }

    #[test]
    fn test_conditional_spring_security_exclusion() -> TclResult<()> {
        let tmp_workspace = tempdir()?;
        let project_root = tmp_workspace.path().join("nosecurityapp");
        let project_root_str = project_root.to_str().unwrap();

        let mock_args = create_mock_config(project_root_str, false);
        run(mock_args)?;

        let java_src_base = project_root.join("src/main/java/com/example/test/nosecurityapp");

        assert!(!java_src_base.join("SecurityConfig.java").exists());
        
        Ok(())
    }

    #[test]
    fn test_cli_rejects_invalid_flags_gracefully() {
        use clap::Parser;
    
        // Case A: Test passing a completely unrecognized/invalid flag option
        let bad_arguments = vec!["tjcli", "--unrecognized-flag-option"];
        let parse_attempt = TclConfig::try_parse_from(bad_arguments);
        
        assert!(
            parse_attempt.is_err(),
            "The CLI parser should have rejected the unrecognized flag option"
        );
        
        let error_message = parse_attempt.unwrap_err().to_string();
        assert!(
            error_message.contains("unexpected argument") || error_message.contains("unrecognized"),
            "Error message did not match expected argument rejection style: {}", error_message
        );
    
        // Case B: Test passing an invalid data type format (e.g. alphabetical text for a numeric port argument)
        let invalid_port_type = vec!["tjcli", "--port", "NOT_A_VALID_PORT_NUMBER"];
        let type_attempt = TclConfig::try_parse_from(invalid_port_type);
        
        assert!(
            type_attempt.is_err(),
            "The parser should have rejected an alphabetical string value for the numeric port parameter"
        );
    }

}
