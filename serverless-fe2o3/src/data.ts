export const x86_64_linux = "x86_64-unknown-linux-gnu"
export const aarch64_linux = "aarch64-unknown-linux-gnu"

export type RustTargetTypes = "x86_64-unknown-linux-gnu"
  | "aarch64-unknown-linux-gnu"

export type ToolChain = "stable" | "beta" | "nightly"
 
export interface RustOptions {
  version: string
  target: RustTargetTypes
  toolchain: ToolChain
  pkg: string
  src_dir: string
}

export interface DockerOptions {
  build: boolean
  tag: string
  extras: string[]
}