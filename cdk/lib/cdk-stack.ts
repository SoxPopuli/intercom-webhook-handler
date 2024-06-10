import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as s3 from 'aws-cdk-lib/aws-s3';
import { RustFunction } from "cargo-lambda-cdk";
import { Architecture, LayerVersion } from "aws-cdk-lib/aws-lambda";

import { SqsEventSource } from "aws-cdk-lib/aws-lambda-event-sources";
import { ApiGatewayToSqs } from '@aws-solutions-constructs/aws-apigateway-sqs';

export function stack(scope: Construct, id: string, props: cdk.StackProps) {
  const stack = new cdk.Stack(scope, id, props);

  const bucket = new s3.Bucket(stack, "OutputBucket", {
    bucketName: `intercom-webhook-handler-${id.toLowerCase()}-output-bucket`,
    removalPolicy: cdk.RemovalPolicy.DESTROY,
  })

  const apiQueue = new ApiGatewayToSqs(stack, "InputQueue", { });

  const handler = new RustFunction(stack, `Handler`, {
    manifestPath: "../Cargo.toml",
    architecture: Architecture.ARM_64,
    bundling: {
      cargoLambdaFlags: ["--target", "aarch64-unknown-linux-musl"],
    },
    events: [
      new SqsEventSource(apiQueue.sqsQueue),
    ],
    environment: {
      OTEL_ENDPOINT: "http://localhost:4317/v1/traces",
      DD_OTLP_CONFIG_RECEIVER_PROTOCOLS_GRPC_ENDPOINT: "localhost:4317",
      OUTPUT_BUCKET: bucket.bucketArn,
    },
    layers: [
      LayerVersion.fromLayerVersionArn(
        stack,
        "DatadogExtension",
        "arn:aws:lambda:sa-east-1:464622532012:layer:Datadog-Extension:53",
      ),
    ],
  });

  apiQueue.sqsQueue.grantConsumeMessages(handler);
  bucket.grantPut(handler);

  return stack;
}
