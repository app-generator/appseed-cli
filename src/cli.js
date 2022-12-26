import arg from 'arg';
import inquirer from 'inquirer';
import { createProject } from './main';
import execa from 'execa';
function parseArgumentsIntoOptions(rawArgs) {
    try {
        const args = arg(
        {
            '-t': String,
            '-d': Boolean,
            '-n': String,
            '--template': '-t',
            '--docker': '-d',
            '--folder-name': '-n',
        },
        {
            argv: rawArgs.slice(2),
        },
        );
        return {
            useDocker: args['-d'] || false, // default to false
            folderName: args['-n'] || false, // default folder name
            template: args['-t'] || false, // default to undefined
        };
    } catch (err) {
        printHelp();
        process.exit(1);
    }
}

async function promptForMissingOptions(options) {
    const defaultTemplate = 'argon-rust';
    const questions = [];
    const templates = ['flask-datta-able', 'django-datta-able', 'django-volt-dashboard', 'flask-volt-dashboard'];
    //if the template provided is not in the list of templates, then we prompt the user to choose one
    if (options.template != false){
        if (!templates.includes(options.template)) {
            console.log('Template not found, if you are not sure, remove the -t flag');
            //exit
            process.exit(0);
        }
    }
    if (options.useDocker) {
        questions.push({
            type: 'confirm',
            name: 'useDocker',
            message: 'Confirm that you want to use Docker?',
            default: options.useDocker,
        });
    }

    if (!options.folderName) {
        // Ask for folder name
        questions.push({
            type: 'input',
            name: 'folderName',
            message: 'Enter a folder name for your project:',
            default: 'my-project',
        });
    }
    if (!options.template) {
        // Ask for template
        questions.push({
            type: 'list',
            name: 'template',
            message: 'Which project template would you like to use?',
            choices: templates,
            default: defaultTemplate,
        });
    }
    const answers = await inquirer.prompt(questions);
    return{
        ...options,
        folderName: options.folderName || answers.folderName,
        template: options.template || answers.template,
    };
}
function printHelp() {
    console.log('APPSEED CLI - Made with ❤️ by Saponciou');
    console.log('Usage: appseed-cli [options]');
    console.log('Options:');
    console.log('  -t, --template <template>  Template to use for the project');
    console.log('  -d, --docker               Use Docker for the project');
    console.log('  -n, --folder-name <name>   Name of the folder to create');
    console.log('  -h, --help                 Display this help message');
}

export async function cli(args) {
    // check if there is help flag
    let help = args.indexOf('--help') >= 0 || args.indexOf('-h') >= 0;
    if (help) {
        printHelp();
        return;
    }
    // check for git installations by running git --version
    let gitInstalled = true;
    const result = await execa('git', ['--version']);
    if (result.failed) {
        gitInstalled = false;
    }
    if (!gitInstalled) {
        console.log('Git is not installed, please install it and try again');
        // ask the user to install git
        const questions = [];
        questions.push({
            type: 'confirm',
            name: 'installGit',
            message: 'Do you want to install git?',
            default: true,
        });
        const answers = await inquirer.prompt(questions);
        if (answers.installGit) {
            // install git
            console.log('Installing git... (Must run appseed-cli again as root)');
            const newResult = await execa('sudo', ['apt', 'install', 'git']);
            if (newResult.failed) {
                console.log('Failed to install git');
                process.exit(1);
            }
            console.log('Git installed successfully');
        }

    }
    let options = parseArgumentsIntoOptions(args);
    options = await promptForMissingOptions(options);


    //console.log(options);
    await createProject(options);
}

