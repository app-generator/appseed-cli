import chalk from 'chalk';
import execa from 'execa';
import fs from 'fs';
import Listr from 'listr';
import path from 'path';


async function debug(string) {
    console.log(chalk.grey(`   ${string}`));
    return;
}

async function showHelpAfterFinishing(options) {
    // get current os
    const os = require('os');
    const currentOS = os.platform();

    // get if its flask or django
    const isFlask = options.template.includes('flask');
    const isDjango = options.template.includes('django');
    // if windows
    if (options.docker) {
        console.log(`

${chalk.green.bold(' Next Steps: ')}

    ${chalk.yellow.bold('cd')} ${chalk.white.italic(options.folderName)}
    ${chalk.yellow.bold('docker-compose up')}${chalk.blue.italic(' --build')}

    And open your browser at: ${chalk.blue.italic('http://localhost:5085')}
        `);
    }
    else {
        if (currentOS === 'win32') {
            if (isFlask) {
        console.log(`

${chalk.green.bold(' Next Steps: ')}

    # Install dependencies
    ${chalk.yellow.bold('cd')} ${chalk.white.italic(options.folderName)}
    ${chalk.yellow.bold('virtualenv env')}
    ${chalk.yellow.bold('.\\env\\Scripts\\activate')}
    ${chalk.yellow.bold('pip install -r requirements.txt')}

    # CMD
    ${chalk.blue.bold('set')} FLASK_APP=app.py
    ${chalk.blue.bold('set')} FLASK_ENV=development

    # Powershell
    ${chalk.blue.bold('$env:FLASK_APP')} = ".\\run.py"
    ${chalk.blue.bold('$env:FLASK_ENV')} = "development"

    # Run the app
    ${chalk.yellow.bold('flask run')}
    // or with https
    ${chalk.yellow.bold('flask run --cert=adhoc')}

    And open your browser at: ${chalk.blue.italic('http://localhost:5000')}


        `);
        }
        else if (isDjango) {
            console.log(`

${chalk.green.bold(' Next Steps: ')}

    # Install dependencies
    ${chalk.yellow.bold('cd')} ${chalk.white.italic(options.folderName)}
    ${chalk.yellow.bold('virtualenv env')}
    ${chalk.yellow.bold('.\\env\\Scripts\\activate')}
    ${chalk.yellow.bold('pip install -r requirements.txt')}

    # Run the app
    ${chalk.yellow.bold('python manage.py runserver')}

    And open your browser at: ${chalk.blue.italic('http://localhost:8000')}

    `);
        }
        if (currentOS === 'linux' || currentOS === 'darwin' || currentOS === 'freebsd') {
    console.log(`

${chalk.green.bold(' Next Steps: ')}

    # Install dependencies
    ${chalk.yellow.bold('cd')} ${chalk.white.italic(options.folderName)}
    ${chalk.blue.bold('source')} env/bin/activate
    ${chalk.yellow.bold('pip install -r requirements.txt')}

    # Set environment variables
    ${chalk.blue.bold('export')} FLASK_APP=run.py
    ${chalk.blue.bold('export')} FLASK_ENV=development

    # Run the app
    ${chalk.yellow.bold('flask run')}
    // or with https
    ${chalk.yellow.bold('flask run --cert=adhoc')}

    And open your browser at: ${chalk.blue.italic('http://localhost:5000')}

        `);
        }
    }
}
async function downloadTemplate(options) {
    const template = `https://github.com/app-generator/${options.template}.git`;
    debug(`Starting the download process`);
    if (fs.existsSync(options.folderName)) {
        return Promise.reject(new Error('Target directory already exists'));
    }
    const result = await execa('git', ['clone', template, options.folderName], {
        cwd: process.cwd(),
    });
    if (result.failed) {
        return Promise.reject(new Error('Failed to download template'));
    }
    return;
}


export async function createProject(options) {
    const tasks = new Listr(
        [
            {
                title: ' Downloading template files...',
                task: () => downloadTemplate(options),
            },
        ],
        {
            exitOnError: false,
        }
    );
    await tasks.run();
    console.log('%s Project ready!', chalk.green.bold('DONE'));
    await showHelpAfterFinishing(options);
    return true;
}
