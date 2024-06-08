import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as sqs from "aws-cdk-lib/aws-sqs";
import * as api from "aws-cdk-lib/aws-apigatewayv2";
import * as integrations from "aws-cdk-lib/aws-apigatewayv2-integrations";
import { RustFunction } from "cargo-lambda-cdk";
import { Architecture, LayerVersion } from "aws-cdk-lib/aws-lambda";

import * as dotenv from "dotenv";

dotenv.config({ path: "../.env" });

export function tryLoadEnvVars(names: string[]): { [key: string]: string } {
  const data = names
    .map((name) => [name, process.env[name]])
    .filter((x) => x[1] !== undefined);

  return Object.fromEntries(data);
}

export function stack(scope: Construct, id: string, props: cdk.StackProps) {
  const stack = new cdk.Stack(scope, id, props);

  const queue = new sqs.Queue(stack, "MessageQueue");

  const handler = new RustFunction(stack, `Handler`, {
    manifestPath: "../Cargo.toml",
    architecture: Architecture.ARM_64,
    bundling: {
      cargoLambdaFlags: ["--target", "aarch64-unknown-linux-musl"],
    },
    environment: {
      OTEL_ENDPOINT: "http://localhost:4317/v1/traces",
      DD_OTLP_CONFIG_RECEIVER_PROTOCOLS_GRPC_ENDPOINT: "localhost:4317",
      MESSAGE_QUEUE: queue.queueArn,
    },
    layers: [
      LayerVersion.fromLayerVersionArn(
        stack,
        "DatadogExtension",
        "arn:aws:lambda:sa-east-1:464622532012:layer:Datadog-Extension:53",
      ),
    ],
  });

  queue.grantConsumeMessages(handler);

  new api.HttpApi(stack, "Api", {
    defaultIntegration: new integrations.HttpLambdaIntegration(
      "ApiIntegration",
      handler,
    ),
  });

  return stack;
}
