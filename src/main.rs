use std::env;
use colored::*;
use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use std::{process::Stdio, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // options list:
    // if --docker/ -d, use_docker = true, else, false
    // --template/ -t: template_name: string
    // --folder-name/ -n: folder_name: string
    
    let mut use_docker = false;
    let mut template_name = "default".to_string();
    let mut folder_name = "default".to_string();
    let templates = vec!["flask-datta-able", "django-datta-able", "django-volt-dashboard", "flask-volt-dashboard", "flask-adminlte", "django-soft-ui-dashboard"];

    let theme = ColorfulTheme::default();
    
    // if args length is 1, then no args were passed so help message is displayed
    if args.len() == 1 {
        print_help();
        std::process::exit(0);
    }
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
    if args.contains(&String::from("--list")) || args.contains(&String::from("-l")) {
        println!("{}", "Available templates:".yellow());
        let mut flask_templates = vec![];
        let mut django_templates = vec![];
        let mut other_templates = vec![];
        for template in templates {
            if template.contains("flask"){
                flask_templates.push(template);
            }
            else if template.contains("django"){
                django_templates.push(template);
            }
            else{
                other_templates.push(template);
            }
        }
        println!("  {}", "Flask".blue());
        for template in flask_templates {
            println!("    {} {}","->".italic().blue(), template.green());
        }
        println!("  {}", "Django".blue());
        for template in django_templates {
            println!("    {} {}","->".italic().blue(), template.green());
        }
        if other_templates.len() > 0 {
            println!("  {}", "Other".blue());
            for template in other_templates {
                println!("    {} {}","->".italic().blue(), template.green());
            }
        }
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
    
    download_template(template_name, folder_name, use_docker);
}

fn print_help() {
    println!("Usage: {} [OPTIONS]", "appseed-cli".green());
    println!("");
    println!("Options:");
    println!("    --template,    -t     {} The template to use from app-generator", "[Required]".yellow());
    println!("    --folder-name, -n     {} The name of the folder to create", "[Required]".yellow());
    println!("    --docker,      -d     {} Docker support at the end of the installation", "[Optional]".yellow());
    println!("    --list,        -l     {} Print the list of available templates", "[Optional]".yellow());
    println!("    --help,        -h     Print this help message");
}

fn download_template(template_name: String, folder_name: String, use_docker: bool) {
    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(" Do you want to continue?")
        .interact()
        .unwrap();

    let template = format!("https://github.com/app-generator/{}.git", template_name);
    let folderr = format!("{}", folder_name);    
    if confirm {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            // For more spinners check out the cli-spinners project:
            // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
            .tick_strings(&[
                "⠋",
                "⠙",
                "⠹",
                "⠸",
                "⠼",
                "⠴",
                "⠦",
                "⠧",
                "⠇",
                "⠏",
                "✔",
            ]),

        );
        pb.set_message(" Downloading...");
        // spawn a new process to run the git clone command
        let output = std::process::Command::new("git")
            .arg("clone")
            .arg(template)
            .arg(folder_name)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap();
        if output.success() {
            pb.finish_with_message(format!(" {}", "Done!".bold()));
            help_after_finish(use_docker, template_name, folderr);
        } else {
            pb.finish_with_message("Failed!");
        }
    } else {
        println!("Aborting...");
        std::process::exit(0);
    }
}


fn help_after_finish(use_docker: bool, template: String, folder_name: String) {
    let os = env::consts::OS;

    let is_flask = template.contains("flask");
    let is_django = template.contains("django");
    let template_url = format!("https://github.com/app-generator/{}", template);
    println!(" {}", "Next steps:".bold().green());
    println!("");
    if use_docker {
        println!("    {} {}", "cd".bold().yellow(), folder_name.italic());
        println!("    {} {}", "docker-compose up".bold().yellow(), "--build".italic().blue());
        println!("");
        println!("    {} {}", "And open your browser at:".italic().blue(), "http://localhost:5000".bold());
    }
    else{
        if os.contains("windows"){
            println!("    # Install dependencies");
            println!("    {} {}", "cd".bold().yellow(), folder_name.italic());
            println!("    {}", "python -m venv venv".bold().yellow());
            println!("    {}", "venv\\Scripts\\activate".bold().yellow());
            println!("    {}", "pip install -r requirements.txt".bold().yellow());
            if is_flask {
                println!("");
                println!("    # CMD");
                println!("    {} FLASK_APP=run.py", "set".bold().blue());
                println!("    {} FLASK_ENV=development", "set".bold().blue());
                println!("");
                println!("    # Powershell");
                println!("    {} = \".\\run.py\"", "$env:FLASK_APP".bold().blue());
                println!("    {} = \"development\"", "$env:FLASK_ENV".bold().blue());
                println!("");
                println!("    # Run");
                println!("    {}", "flask run".bold().yellow());
                println!("    {} {}", "And open your browser at:".italic().blue(), "http://localhost:5000");
            }
            if is_django {
                println!("");
                println!("    # Setup the Database");
                println!("    {}", "python manage.py makemigrations".bold().yellow());
                println!("    {}", "python manage.py migrate".bold().yellow());
                if template.contains("soft-ui"){
                    println!("    {} # Optional", "python manage.py generate-api".bold().yellow());
                }
                println!("");
                println!("    # Run");
                println!("    {}", "python manage.py runserver".bold().yellow());
                println!("    {} {}", "And open your browser at:".italic().blue(), "http://localhost:8000");
            }
        }
        else{
            println!("    # Install dependencies");
            println!("    {} {}", "cd".bold().yellow(), folder_name.italic());
            println!("    {}", "python3 -m venv venv".bold().yellow());
            println!("    {}", "source venv/bin/activate".bold().yellow());
            println!("    {}", "pip install -r requirements.txt".bold().yellow());
            if is_flask {
                println!("");
                println!("    # Set variables");
                println!("    {} FLASK_APP=run.py", "export".bold().blue());
                println!("    {} FLASK_ENV=development", "export".bold().blue());
                println!("");
                println!("    # Run");
                println!("    {}", "flask run".bold().yellow());
                println!("    {} {}", "And open your browser at:".italic().blue(), "http://localhost:5000")                
            }
            if is_django {
                println!("    # Setup the Database");
                println!("    {}", "python manage.py makemigrations".bold().yellow());
                println!("    {}", "python manage.py migrate".bold().yellow());
                if template.contains("soft-ui"){
                    println!("    {} # Optional", "python manage.py generate-api".bold().yellow());
                }
                println!("");
                println!("    # Run");
                println!("    {}", "python manage.py runserver".bold().yellow());
                println!("    {} {}", "And open your browser at:".italic().blue(), "http://localhost:8000");
            }
        }
    }
    println!("");
    println!("    {} {}", "For more information:".italic().blue(), template_url);
}
