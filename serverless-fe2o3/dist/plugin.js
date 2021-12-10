"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const data_1 = require("./data");
const path_1 = require("path");
const child_process_1 = require("child_process");
class RustPlugin {
    constructor(serverless, options) {
        /**
         * Simple function that runs a command asynchronously
         *
         * @param cmd
         * @param args
         * @returns
         */
        this.run = async (cmd, args) => {
            const command = (0, child_process_1.spawn)(cmd, args);
            command.stdout.on('data', (data) => {
                this.serverless.cli.log(`stdout: ${data}`);
            });
            return new Promise((resolve, reject) => {
                command.on("exit", resolve);
            });
        };
        this.cargo = () => {
            const { pkg, target } = this.options;
            return `cargo build --release --package ${pkg} --target ${target}`;
        };
        this.dockerFile = (dockerType) => {
            return `${(0, path_1.dirname)(__dirname)}/${dockerType}/Dockerfile`;
        };
        // Executes the docker rcommands to possibly build the image, and run it
        this.buildLambda = async () => {
            const { src_dir, version } = this.options;
            const volume = `-v ${src_dir}:/code`;
            const rustBuildArg = `--build-arg RUST_TARGET=${version}`;
            const { extras, tag, build } = this.docker;
            const extraBuildArgs = extras.length != 0 ? `--build-arg EXTRAS="${extras.join(" ")}"` : "";
            const buildArgs = `${rustBuildArg} ${extraBuildArgs}`;
            const dockerPath = this.dockerFile("builder");
            if (build) {
                const dockerBuildCmd = `docker build -f ${dockerPath} ${buildArgs} -t ${tag} ${src_dir}`;
                // Run the dockerBuildCmd
                const [cmd, ...args] = dockerBuildCmd.split(" ");
                const exitVal = await this.run(cmd, args);
            }
            else {
                // Make sure we have a build image. and if not pull it down
            }
            const dockerRunCmd = `docker run -it --name fe2o3 -v ${volume} ${tag} ${this.cargo()}`;
            // run the dockerRunCmd
            const [cmd, ...args] = dockerRunCmd.split(' ');
            const runExitVal = await this.run(cmd, args);
        };
        this.serverless = serverless;
        const { rustOpts } = this.serverless.service.custom.rust ?? {};
        let { target, version, toolchain, pkg, src_dir } = rustOpts;
        if (!pkg)
            throw new Error("Must supply package");
        this.options = {
            target: target || data_1.x86_64_linux,
            version: version || "1.57.0",
            toolchain: toolchain || "stable",
            src_dir: src_dir || process.cwd(),
            pkg
        };
        let docker = this.serverless.service.custom.docker ?? {
            build: true, tag: pkg, extras: []
        };
        this.docker = docker;
    }
}
