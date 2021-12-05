import Serverless from "serverless"
import { RustOptions, x86_64_linux, aarch64_linux, DockerOptions } from "./data"
import { dirname } from "path"
import { spawn } from "child_process"

const run = (cmd: string, args: string[]) => {
  const command = spawn(cmd, args);

  command.stdout.on('data', (data) => {
    console.log(`stdout: ${data}`);
  });

  command.on('close', (code) => {
    console.log(`child process close all stdio with code ${code}`);
  });

  command.on('exit', (code) => {
    console.log(`child process exited with code ${code}`);
  });
}

class RustPlugin {
  serverless: Serverless
  options: RustOptions
  docker: DockerOptions

  constructor(serverless: Serverless, options: any) {
    this.serverless = serverless

    const { rustOpts } = this.serverless.service.custom.rust ?? {}
    let { target, version, toolchain, pkg, src_dir } = rustOpts
    if (!pkg) throw new Error("Must supply package")

    this.options = {
      target: target || x86_64_linux,
      version: version || "1.57.0",
      toolchain: toolchain || "stable",
      src_dir: src_dir || process.cwd(),
      pkg
    }

    let docker = this.serverless.service.custom.docker ?? {
      build: true, tag: pkg, extras: []
    }
    this.docker = docker

  }

  cargo = (): string => {
    const { pkg, target } = this.options
    return `cargo build --release --package ${pkg} --target ${target}`
  }

  dockerFile = (dockerType: "builder" | "tester") => {
    return `${dirname(__dirname)}/${dockerType}/Dockerfile`
  }

  // Executes the docker run command
  buildLambda = () => {
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
    }
    const dockerRunCmd = `docker run -it --name fe2o3 -v ${volume} ${tag} ${this.cargo()}`
    // run the dockerRunCmd
  }
}