import chalk from 'chalk';
import execa from 'execa';
import fs from 'fs';
import Listr from 'listr';
import path from 'path';


async function debug(string){
    console.log(chalk.grey(`   ${string}`));
    return;
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
    console.log('%s Project ready!', chalk.green.bold('  ✔  DONE'));
    return true;
}
