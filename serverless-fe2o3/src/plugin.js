//import { dirname } from "path"
const path = require("path")
const { spawnSync } = require("child_process")
const os = require("os")

const x86_64_linux = "x86_64-unknown-linux-gnu"
const aarch64_linux = "aarch64-unknown-linux-gnu"

// Defaults
const defaultRustcVersion = "1.57.0"
const defaultRustToolchain = "stable"
const defaultBuildContainerName = "fe2o3"
const defaultZip = "lambda.zip"

class RustPlugin {
  constructor(serverless, options) {
    this.serverless = serverless
    this.service = serverless.service
    this.opts = options
    this.runtime = this.service.provider.runtime
    this.containerName = "fe2o3"

    // Get values from custom.rust
    let { rust: rustOpts } = this.service.custom ?? {}
    rustOpts = rustOpts || {}
    this.serverless.cli.log(`rustOpts = ${JSON.stringify(rustOpts, null, 2)}`)

    this.options = {
      target: rustOpts.target || x86_64_linux,
      version: rustOpts.version || defaultRustcVersion,
      toolchain: rustOpts.toolchain || defaultRustToolchain,
      src_dir: rustOpts.src_dir || process.cwd()
    }

    // Get values from custom.docker
    let docker = Object.assign({
      build: true, tag: defaultBuildContainerName, extras: []
    }, this.service.custom.docker || {})
    this.docker = docker

    this.log = this.serverless.cli.log

    // hooks to call into the plugin's functionality
    this.hooks = {
      "before:package:createDeploymentArtifacts": () => {
        this.log("In before:package:createDeploymentArtifacts")
        this.build()
      },
      "before:deploy:function:packageFunction": () => {
        this.log("In before:deploy:function:packageFunction")
        this.build()
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

  /**
   * Calls the builder-fe2o3 script
   * 
   * @param {string} pkg Name of the cargo package to build
   * @param {string} target The rustup target (eg x86_64-unknown-linux-gnu)
   * @param {string} toolchain The toolchain
   * @param {string} version rustc version
   * @param {string} zip name of the zip file to create
   * @returns 
   */
  buildScript = (pkg, target, toolchain, version, zip) => {
    if (!pkg) throw new Error("Must supply a package name")
    target = target || this.options.target
    toolchain = toolchain || this.options.toolchain
    version = version || this.options.version
    zip = zip || defaultZip
    const toolchainArg = toolchain == "stable" ? "" : `--toolchain ${toolchain}`
    return `builder-fe2o3.sh -p ${pkg} -t ${target} -v ${version} ${toolchainArg} -z ${zip}`
  }

  /**
   * Returns path to the Dockerfile
   * 
   * @param {string} dockerType "builder" | "tester"
   * @returns 
   */
  dockerFile = (dockerType) => {
    return `${path.dirname(__dirname)}/docker/${dockerType}/Dockerfile`
  }

  /**
   * Stops and removes a container
   * 
   * @param {string} name 
   */
  removeContainer = (name) => {
    if (!name) {
      name = this.containerName
    }
    this.run("docker", ["stop", name])
    this.run("docker", ["rm", name])
  }

  /**
   * Builds the rust code through docker
   * 
   * @param {*} pkg 
   * @param {*} target 
   * @param {*} toolchain 
   * @param {*} version 
   * @param {*} zip 
   */
  buildLambda = (pkg, target, toolchain, version, zip) => {
    version = version || this.options.version
    target = target || this.options.target
    toolchain = toolchain || this.options.toolchain
    zip = zip || defaultZip
    const { src_dir } = this.options
    const { extras, tag, build, context } = this.docker

    if (build) {
      // determine the build args for the Dockerfile
      const rustBuildArg = `--build-arg RUST_TARGET=${target}`
      const extraBuildArgs = extras.length != 0 ? `--build-arg EXTRAS="${extras.join(" ")}"` : ""
      const buildArgs = `${rustBuildArg} ${extraBuildArgs}`

      const dockerPath = this.dockerFile("builder")
      const dockerBuildCmd = `docker build -f ${dockerPath} ${buildArgs} -t ${tag} ${context || src_dir}`
      const [ cmd, ...args ] = dockerBuildCmd.split(" ")
      this.log(`Executing: ${cmd} ${args.filter(s => s != "")}`)
      const exitVal = this.run(cmd, args.filter(s => s != ""))
    } else {
      // TODO: Make sure we have a build image. and if not pull it down
    }

    // Need to remove container before running
    this.removeContainer()

    // mount the rust source code into the container's /code
    const volume = `-v ${src_dir}:/code`
    // get the user and group so we build as it instead of container root
    const { uid, gid } = os.userInfo()
    // build up the actual docker command
    const scriptArg = this.buildScript(pkg, target, toolchain, version, zip)
    const dockerRunCmd = `docker run --name fe2o3 --user ${uid}:${gid} ${volume} ${tag} ${scriptArg}`
    // run the dockerRunCmd
    const [ cmd, ...args ] = dockerRunCmd.split(' ')
    this.log(`Running ${cmd} ${args}`)
    // invoke the child processs
    const runExitVal = this.run(
      cmd, 
      args.filter(s => s != ""),
      Object.assign({ 
        RUST_BUILD_VERSION: version,
        RUST_TARGET: target
      }, process.env)
    )
    this.log(`exitval = ${runExitVal}`)
  }

  getFunctions = () => {
    return this.opts.functions ? this.opts.functions : this.service.getAllFunctions()
  }

  build = () => {
    if (this.service.provider.name != "aws") {
      this.log(`provider ${this.service.provider} is not supported`)
      return
    }

    const usedDefaultZips = []
    this.getFunctions().forEach(fnName => {
      const fn = this.service.getFunction(fnName)
      const runtime = fn.tags?.runtime
      if (runtime != "rust") {
        this.log(`Skipping non-rust function ${fnName}`)
        return
      }
      const pkg = fn.handler
      if (!pkg) {
        throw new Error("Must supply handler name")
      } else {
        this.log(`Building pkg ${pkg}`)
      }
      // If we have more than one function, and we have more than one defaultZip, it's an error
      //this.log(JSON.stringify(this.service.package, null, 2))
      const zip = fn.package?.artifact || this.service.package?.artifact || defaultZip
      if (zip == defaultZip) usedDefaultZips.push(fnName)
      if (usedDefaultZips.length > 1) throw new Error(`${zip} already used for ${fnName}`)

      // get tags to see if we build with a different toolchain
      const { toolchain, target, version } = fn.tags

      // build with the docker image
      // TODO: If docker.build is true, we should name each image and see if we can reuse based on target, 
      // toolchain and version
      this.buildLambda(pkg, target, toolchain, version, zip)

    })
  }
}

module.exports = RustPlugin