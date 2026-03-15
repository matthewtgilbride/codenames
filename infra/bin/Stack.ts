#!/usr/bin/env node
import 'source-map-support/register';
import { App, Stack } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { CloudfrontConstruct } from '../lib/CloudfrontConstruct';
import { DynamoConstruct } from '../lib/DynamoConstruct';
import { LambdaConstruct } from '../lib/LambdaConstruct';
import { StringParameter } from 'aws-cdk-lib/aws-ssm';

class CodenamesStack extends Stack {
  constructor(scope: Construct, id: string) {
    super(scope, id, {
      env: {
        region: process.env.AWS_DEFAULT_REGION,
        account: process.env.AWS_ACCOUNT_NUMBER,
      },
      stackName: 'Codenames',
    });

    // valueFromLookup resolves at synth-time (needed for HostedZone.fromLookup)
    const domainName = StringParameter.valueFromLookup(this, 'domainName');
    // valueForStringParameter resolves at deploy-time (avoids ARN validation issues)
    const certificateArn = StringParameter.valueForStringParameter(this, 'certificateArn');

    const { table } = new DynamoConstruct(this, `${id}-Dynamo`);

    const { functionUrl } = new LambdaConstruct(this, `${id}-Lambda`, {
      table,
      allowedOrigins: [
        `https://codenames.${domainName}`,
        'http://localhost:3000',
      ],
    });

    new CloudfrontConstruct(this, `${id}-Cloudfront`, {
      functionUrl,
      domainName,
      certificateArn,
    });
  }
}

const app = new App();
new CodenamesStack(app, 'Codenames');
