import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import path = require('path');
import { cdkRootDir } from './env';
import * as apigateway from 'aws-cdk-lib/aws-apigateway';
// import * as sqs from 'aws-cdk-lib/aws-sqs';

function getLambdaZip(binName: string): string {
  return path.join(cdkRootDir(), '..', 'archery-tools-lambdas', 'target', 'lambda', binName, 'bootstrap.zip');
}

export class ArcheryToolsCdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    // The code that defines your stack goes here


    // Test S3 bucket, just for demonstration purpose
    new cdk.aws_s3.Bucket(this, 'ArcheryToolsBucket', {
      versioned: true,
      removalPolicy: cdk.RemovalPolicy.DESTROY,
      autoDeleteObjects: true
    });

    // The Scoresheet RESTful resource
    const scoreSheetFunction = new lambda.Function(this, 'ScoreSheetFunction', {
      architecture: lambda.Architecture.ARM_64,
      runtime: lambda.Runtime.PROVIDED_AL2023,
      handler: 'bootstrap',
      code: lambda.Code.fromAsset(getLambdaZip('scoresheet')),
      tracing: lambda.Tracing.ACTIVE
    });

    // API Gateway
    new apigateway.LambdaRestApi(this, 'ScoreSheet', {
      handler: scoreSheetFunction,
      integrationOptions: {
        proxy: true,
        allowTestInvoke: true
      }
    });
  }
}
