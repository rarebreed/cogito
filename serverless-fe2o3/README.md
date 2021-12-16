# serverless-fe2o3

A serverless framework plugin for rust

## Prerequisites

fe2o3 is currently geared towards building in a docker based environment and thus requires either docker engine for
linux, or docker desktop for macos and windows.  It also assumes that your rust project is using the aws-rust-runtime

## What does it do?

serverless-fe2o3 (henceforth fe2o3) is a plugin for the serverless framework that allows you to have a cargo based
rust project, and build it in the same execution environment as the AWS provided.al2 environment.  

The plugin adds some new options for ther serverless.yml file so that the plugin knows what it needs to do.  The plugin
will build your binary and bundle it for you by hooking in before either the `sls package` or `sls deploy` commands. It
rebames your executable to `bootstrap` and zips it up to lambda.zip (by default) as required by the AWS custom runtime.

Optionally, you can provide your own custom Dockerfile and build context environment if you wish to create your own
rust builder.

## serverless yaml file

Some new variables to the serverless.yml file are now recognized

```yaml
plugins:
  - serverless-fe2o3

custom:
  # Contains "global" rust config values
  rust:
    target: x86_64-unknown-linux-gnu   # Optional, defaults to x86_64-unknown-linux-gnu
    toolchain: stable                  # Optional, defaults to stable     
    version: 1.57.0                    # Required if not set under functions
    src_dir: /path/to/project          # Optional, defaults to directory npx sls command is run
  docker:
    build: true                        # Optional. build the docker image defaults to false
    context: /path/to/build/context    # The build context path.
    tag: ""                            # Optional  docker tag.  defaults to fe2o3
    extras: []                         # optional. other packages to install
    
functions:
  fn-name:
    handler: package-name               # Required the cargo package
    tags:
      runtime: rust                     # required
      version: 1.55.0                   # Optional: rustc version to use
      target: x86_64-unknown-linux-gnu  # Optional: target triple
      toolchain: beta                   # optional:
      src_dir: /path/to/project         # required if different from rust.src_dir
  fn2:
    handler: other-package
```

The `custom.rust` map sets a serverless-wide configuration for various rust related information, such as what rustc
version to use, the target (rustc target triple) or optional toolchain (eg, beta or nightly).  These values will be
overridden in each `function.name.tags` map if those are provided.  This allows you to have different functions that
compile against different toolchains.

The only required field is the `function.name.handler` which should match the cargo package name.  If you have a cargo
workspace project, it knows which member to build and bundle.  If it's a regular cargo project, it should just match
the package name.  In aws provider.al2 runtime with the aws-lambda-runtime project, the handler name typically
doesn't matter.  We just use it here 

The `custom.docker` options set whether you wish to build your rust binary and zip bundle using your own Dockerfile
and build context.

## Why not serverless-rust?

I owe the serverless-rust plugin for helping me understand how to build a serverless franework plugin, but I noticed
that it was not using the official AWS aws-lambda-provded image as the base image to build from.  It was also still 
trying to compile against the musl target instead of the gnu target.  The default image it uses for building was last 
updated in early 2021, and it uses rustc version 1.45 which is far too old for some things.

However, serverless-rust does provide for building locally, which is something this plugin does not yet do.