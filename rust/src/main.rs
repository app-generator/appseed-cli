use std::env;
use colored::*;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};


fn main() {
    let args: Vec<String> = env::args().collect();
    
    // options list:
    // if --docker/ -d, use_docker = true, else, false
    // --template/ -t: template_name: string
    // --folder-name/ -n: folder_name: string
    
    let mut use_docker = false;
    let mut template_name = "default".to_string();
    let mut folder_name = "default".to_string();
    let templates = vec!["flask-datta-able", "django-datta-able", "django-volt-dashboard", "flask-volt-dashboard"];

    let theme = ColorfulTheme::default();

    if args.contains(&"--docker".to_string()) || args.contains(&"-d".to_string()) {
        use_docker = true;
    }
    if args.contains(&"--template".to_string()) || args.contains(&"-t".to_string()) {
        let index = args.iter().position(|x| x == "--template" || x == "-t").unwrap();
        template_name = args[index + 1].to_string();
    }
    if args.contains(&"--folder-name".to_string()) || args.contains(&"-n".to_string()) {
        let index = args.iter().position(|x| x == "--folder-name" || x == "-n").unwrap();
        folder_name = args[index + 1].to_string();
    }
    // Check for the help flag
    if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
        print_help();
        std::process::exit(0);
    }
    // check if template name is valid or is default
    if template_name == "default" || !templates.contains(&template_name.as_str()) {
        let selection = Select::with_theme(&theme)
            .with_prompt(" Select a template")
            .items(&templates)
            .default(0)
            .interact()
            .unwrap();
        template_name = templates[selection].to_string();
    }
    if folder_name == "default" {
        folder_name = Input::with_theme(&theme)
            .with_prompt(" Enter a folder name for your new project")
            .interact_text()
            .unwrap();
    }


    println!("Use Docker: {}", use_docker.to_string().green());
    println!("Template Name: {}", template_name.as_str().green());
    println!("Folder Name: {}", folder_name.as_str().green());



        // Continue with the rest of the program...
}

fn print_help() {
    println!("Usage: {} [OPTIONS]", "appseed-cli".green());
    println!("");
    println!("Options:");
    println!("    --template,    -t     {} The template to use from app-generator", "[Required]".red());
    println!("    --folder-name, -n     {} The name of the folder to create", "[Required]".red());
    println!("    --docker,      -d     {} Docker support at the end of the installation", "[Optional]".yellow());
    println!("    --list,        -l     {} Print the list of available templates", "[Optional]".yellow());
    println!("    --help,        -h     Print this help message");
}
