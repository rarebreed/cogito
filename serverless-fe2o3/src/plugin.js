//import { dirname } from "path"
const path = require("path")
const { spawnSync } = require("child_process")
const { copyFileSync, dirname, mkdtempSync } = require("fs")
const os = require("os")

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

    let docker = Object.assign({
      build: true, tag: pkg, extras: []
    }, this.serverless.service.custom.docker || {})
    this.docker = docker
    this.log = this.serverless.cli.log

    // hooks to call into the plugin's functionality
    this.hooks = {
      "before:package:createDeploymentArtifacts": () => {
        this.log("In before:package:createDeploymentArtifacts")
        this.buildLambda()
      },
      "before:deploy:function:packageFunction": () => {
        this.log("In before:deploy:function:packageFunction")
      },
      "before:offline:start": () => {
        this.log("in before:offline:start event")
        //this.buildLambda()
      },
      "before:offline:start:init": () => {
        this.log("in before:offline:start:init")
        //this.buildLambda()
      },
    };
  }

  /**
   * Simple function that runs a command asynchronously
   * 
   * @param string: cmd 
   * @param string[]: args 
   * @returns Promise<number>
   */
  run = (cmd, args, env) => {
    const command = spawnSync(cmd, args, {
      stdio: ["ignore", process.stdout, process.stderr],
      env: env || process.env
    });

    return command.status
  }

  cargo = () => {
    const { pkg } = this.options
    return `builder-fe2o3.sh ${pkg}`
  }

  // dockerType: "builder" | "tester"
  dockerFile = (dockerType) => {
    return `${path.dirname(__dirname)}/docker/${dockerType}/Dockerfile`
  }

  copyBuilder = () => {
    const parent = dirname(__dirname)
    const source = `${parent}/docker/builder/builder.sh`
    const tmp = mkdtempSync(path.join(os.tmpdir(), "builder-")) 
    copyFileSync(source, `${tmp}/builder.sh`)
    return tmp
  }

  // Executes the docker rcommands to possibly build the image, and run it
  buildLambda = () => {
    const { src_dir, version, target } = this.options
    // mount the rust source code into the container's /code
    const volume = `-v ${src_dir}:/code`
    const rustBuildArg = `--build-arg RUST_TARGET=${target}`
    const { extras, tag, build } = this.docker
    const extraBuildArgs = extras.length != 0 ? `--build-arg EXTRAS="${extras.join(" ")}"` : ""
    const buildArgs = `${rustBuildArg} ${extraBuildArgs}`

    const dockerPath = this.dockerFile("builder")
    if (build) {
      const dockerBuildCmd = `docker build -f ${dockerPath} ${buildArgs} -t ${tag} ${src_dir}`
      // Run the dockerBuildCmd
      const [ cmd, ...args ] = dockerBuildCmd.split(" ")
      this.log(`Executing: ${cmd} ${args.filter(s => s != "")}`)
      const exitVal = this.run(cmd, args.filter(s => s != ""))
    } else {
      // TODO: Make sure we have a build image. and if not pull it down
    }

    const dockerRunCmd = `docker run -it --name fe2o3 ${volume} ${tag} ${this.cargo()}`
    this.log(`Running ${dockerRunCmd}`)
    // run the dockerRunCmd
    const [ cmd, ...args ] = dockerRunCmd.split(' ')
    const env = Object.assign({ 
      RUST_BUILD_VERSION: version,
      RUST_TARGET: target
    }, process.env)
    const runExitVal = this.run(cmd, args, env)
  }
}

module.exports = RustPlugin