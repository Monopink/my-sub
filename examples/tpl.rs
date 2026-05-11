use minijinja::{context, Environment};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Example that demonstrates rendering a template for Clash target
fn render_singbox_template() -> Result<String, Box<dyn std::error::Error>> {
    // Setup template environment
    let mut env = Environment::new();

    // Load the all_base.tpl template
    let template_path = Path::new("base/base/all_base.tpl");
    let template_content = fs::read_to_string(template_path)?;

    // Add the template to the environment
    env.add_template("all_base.tpl", &template_content)?;

    // Setup template context data
    let mut node_list = HashMap::new();
    node_list.insert(
        "proxies".to_string(),
        vec![
            HashMap::from([
                ("name".to_string(), "Node1".to_string()),
                ("type".to_string(), "ss".to_string()),
                ("server".to_string(), "server1.example.com".to_string()),
                ("port".to_string(), "8388".to_string()),
            ]),
            HashMap::from([
                ("name".to_string(), "Node2".to_string()),
                ("type".to_string(), "vmess".to_string()),
                ("server".to_string(), "server2.example.com".to_string()),
                ("port".to_string(), "443".to_string()),
            ]),
        ],
    );

    // Set up global variables
    let mut global_vars = HashMap::new();
    global_vars.insert("clash_proxies_style".to_string(), "flow".to_string());
    global_vars.insert("udp".to_string(), "true".to_string());

    // Set up request parameters
    let mut request_params = HashMap::new();
    request_params.insert("target".to_string(), "singbox".to_string());
    request_params.insert("url".to_string(), "https://example.com/sub".to_string());

    // Set up local variables
    let mut local_vars = HashMap::new();
    local_vars.insert("clash_use_new_field_name".to_string(), "true".to_string());

    // Render the template
    let template = env.get_template("all_base.tpl")?;
    let result = template.render(context! {
        global => global_vars,
        request => request_params,
        local => local_vars,
        node_list => node_list,
    })?;

    Ok(result)
}

/// Example that demonstrates rendering a template for Surge target
fn render_surge_template() -> Result<String, Box<dyn std::error::Error>> {
    // Setup template environment
    let mut env = Environment::new();

    // Load the all_base.tpl template
    let template_path = Path::new("base/base/all_base.tpl");
    let template_content = fs::read_to_string(template_path)?;

    // Add the template to the environment
    env.add_template("all_base.tpl", &template_content)?;

    // Setup template context data for a different target
    let mut node_list = HashMap::new();
    node_list.insert(
        "proxies".to_string(),
        vec![HashMap::from([
            ("name".to_string(), "Node1".to_string()),
            ("type".to_string(), "ss".to_string()),
            ("server".to_string(), "server1.example.com".to_string()),
            ("port".to_string(), "8388".to_string()),
        ])],
    );

    // Set up request parameters with a different target
    let mut request_params = HashMap::new();
    request_params.insert("target".to_string(), "surge".to_string());
    request_params.insert("url".to_string(), "https://example.com/sub".to_string());

    // Render the template
    let template = env.get_template("all_base.tpl")?;
    let result = template.render(context! {
        request => request_params,
        node_list => node_list,
    })?;

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Template Rendering Examples ===\n");

    // Example 1: Render template for Singbox
    match render_singbox_template() {
        Ok(result) => {
            println!("Example 1: Rendered template for Singbox target:");
            println!("{}\n", result);
        }
        Err(e) => println!("Error rendering Singbox template: {}", e),
    }

    // Example 2: Render template for Surge
    match render_surge_template() {
        Ok(result) => {
            println!("Example 2: Rendered template for Surge target:");
            println!("{}\n", result);
        }
        Err(e) => println!("Error rendering Surge template: {}", e),
    }

    Ok(())
}
