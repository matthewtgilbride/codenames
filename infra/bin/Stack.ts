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

    const domainName = StringParameter.valueFromLookup(this, 'domainName');

    const { table } = new DynamoConstruct(this, `${id}-Dynamo`);

    const { functionUrl } = new LambdaConstruct(this, `${id}-Lambda`, {
      table,
      allowedOrigins: [
        `https://codenames.${domainName}`,
        'http://localhost:3000',
      ],
    });

    new CloudfrontConstruct(this, `${id}-Cloudfront`, functionUrl);
  }
}

const app = new App();
new CodenamesStack(app, 'Codenames');
