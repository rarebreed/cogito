import Serverless from "serverless"


class RustPlugin {
  serverless: Serverless

  constructor(serverless: Serverless, options: any) {
    this.serverless = serverless
  }

  // Executes the docker run command
  buildLambda = () => {

  }
}