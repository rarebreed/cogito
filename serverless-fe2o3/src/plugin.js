//import { dirname } from "path"
const { dirname } = require("path")
const { spawn } = require("child_process")

const x86_64_linux = "x86_64-unknown-linux-gnu"
const aarch64_linux = "aarch64-unknown-linux-gnu"

class RustPlugin {
  // serverless: Serverless
  // options: RustOptions
  // docker: DockerOptions

  constructor(serverless, options) {
    this.serverless = serverless

    const { rust: rustOpts } = this.serverless.service.custom ?? {}
    this.serverless.cli.log(`rustOpts = ${JSON.stringify(rustOpts, null, 2)}`)
    let { pkg } = rustOpts
    if (!pkg) throw new Error("Must supply package")

    this.options = {
      target: rustOpts.target || x86_64_linux,
      version: rustOpts.version || "1.57.0",
      toolchain: rustOpts.toolchain || "stable",
      src_dir: rustOpts.src_dir || process.cwd(),
      pkg
    }

    let docker = this.serverless.service.custom.docker ?? {
      build: true, tag: pkg, extras: []
    }
    this.docker = docker

    this.hooks = {
    
    }

  }

  /**
   * Simple function that runs a command asynchronously
   * 
   * @param string: cmd 
   * @param string[]: args 
   * @returns Promise<number>
   */
  run = async (cmd, args) => {
    const command = spawn(cmd, args);

    command.stdout.on('data', (data) => {
      this.serverless.cli.log(`stdout: ${data}`);
    });

    return new Promise((
      resolve, // (code: number) => void,
      reject   // (err: string) => void
    ) => {
      command.on("exit", resolve)
    })
  }

  cargo = () => {
    const { pkg, target } = this.options
    return `cargo build --release --package ${pkg} --target ${target}`
  }

  // dockerType: "builder" | "tester"
  dockerFile = (dockerType) => {
    return `${dirname(__dirname)}/${dockerType}/Dockerfile`
  }

  // Executes the docker rcommands to possibly build the image, and run it
  buildLambda = async () => {
    const { src_dir, version } = this.options
    const volume = `-v ${src_dir}:/code`
    const rustBuildArg = `--build-arg RUST_TARGET=${version}`
    const { extras, tag, build } = this.docker
    const extraBuildArgs = extras.length != 0 ? `--build-arg EXTRAS="${extras.join(" ")}"` : ""
    const buildArgs = `${rustBuildArg} ${extraBuildArgs}`

    const dockerPath = this.dockerFile("builder")
    if (build) {
      const dockerBuildCmd = `docker build -f ${dockerPath} ${buildArgs} -t ${tag} ${src_dir}`
      // Run the dockerBuildCmd
      const [ cmd, ...args ] = dockerBuildCmd.split(" ")
      const exitVal = await this.run(cmd, args)
    } else {
      // Make sure we have a build image. and if not pull it down
    }

    const dockerRunCmd = `docker run -it --name fe2o3 -v ${volume} ${tag} ${this.cargo()}`
    // run the dockerRunCmd
    const [ cmd, ...args ] = dockerRunCmd.split(' ')
    const runExitVal = await this.run(cmd, args)
  }
}

module.exports = RustPlugin