import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as s3 from "aws-cdk-lib/aws-s3";
import { RustFunction } from "cargo-lambda-cdk";
import { Architecture, LayerVersion } from "aws-cdk-lib/aws-lambda";

import { SqsEventSource } from "aws-cdk-lib/aws-lambda-event-sources";
import { ApiGatewayToSqs } from "@aws-solutions-constructs/aws-apigateway-sqs";
import * as apiGw from 'aws-cdk-lib/aws-apigateway'

const service = "IntercomWebhookHandler";

export interface StackProps extends cdk.StackProps {
  apiEnv: string;
}

class Stack extends cdk.Stack {
  apiEnv: string;

  constructor(scope: Construct, id: string, props: StackProps) {
    super(scope, id, props);
    this.apiEnv = props.apiEnv;
  }

  protected allocateLogicalId(cfnElement: cdk.CfnElement): string {
    const prefix = service;
    const base = super.allocateLogicalId(cfnElement);

    return `${prefix}${base}`;
  }
}

function getNameGenerator(env: string) {
  return {
    generateName: (name: string) => {
      return `intercom-webhook-handler-${env}-${name}`;
    },
  }
}

export function stack(scope: Construct, id: string, props: StackProps) {
  const { generateName } = getNameGenerator(props.apiEnv);

  const stack = new Stack(scope, id, {
    ...props,
    stackName: generateName("stack")
  });

  const bucket = new s3.Bucket(stack, "OutputBucket", {
    bucketName: generateName("output-bucket"),
    removalPolicy: cdk.RemovalPolicy.DESTROY,
  });

  const apiQueue = new ApiGatewayToSqs(stack, "Input", {
    queueProps: { queueName: generateName("input-queue") },
    apiGatewayProps: { restApiName: generateName("queue-api") },
  });

  const handler = new RustFunction(stack, `Handler`, {
    functionName: generateName("handler"),
    manifestPath: "../Cargo.toml",
    architecture: Architecture.ARM_64,
    runtime: "provided.al2023",
    bundling: {
      cargoLambdaFlags: ["--target", "aarch64-unknown-linux-musl"],
    },
    events: [new SqsEventSource(apiQueue.sqsQueue)],
    environment: {
      OTEL_ENDPOINT: "http://localhost:4317/v1/traces",
      DD_OTLP_CONFIG_RECEIVER_PROTOCOLS_GRPC_ENDPOINT: "localhost:4317",
      OUTPUT_BUCKET: bucket.bucketArn,
    },
    layers: [
      //LayerVersion.fromLayerVersionArn(
      //  stack,
      //  "DatadogExtension",
      //  "arn:aws:lambda:sa-east-1:464622532012:layer:Datadog-Extension:53",
      //),
    ],
  });

  apiQueue.sqsQueue.grantConsumeMessages(handler);
  bucket.grantPut(handler);

  return stack;
}
