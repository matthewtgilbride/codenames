#!/usr/bin/env node
import 'source-map-support/register';
import { App, Construct, Stack } from '@aws-cdk/core';
import { ClusterConstruct } from '../lib/ClusterConstruct';
import { RepositoryConstruct } from '../lib/RepositoryConstruct';
import { DevelopmentInstanceConstruct } from '../lib/DevelopmentInstanceConstruct';
import { InstanceConstruct } from '../lib/InstanceConstruct';

type DeployType = 'app-instance' | 'dev-instance' | 'cluster' | 'registry';

class CodenamesStack extends Stack {
  constructor(scope: Construct, id: string, deployType: DeployType) {
    super(scope, id, {
      env: {
        region: process.env.AWS_DEFAULT_REGION,
        account: process.env.AWS_ACCOUNT_NUMBER,
      },
      stackName: `Codenames-${deployType}`,
    });

    switch (deployType) {
      case 'cluster':
        new ClusterConstruct(this, `${id}-Cluster`);
        break;
      case 'registry':
        new RepositoryConstruct(this, `${id}-Repositories`);
        break;
      case 'dev-instance':
        if (!process.env.PUBLIC_IP) {
          throw new Error(
            `PUBLIC_IP environment variable must be provided to deploy dev EC2 instance`,
          );
        }
        new DevelopmentInstanceConstruct(this, `${id}-DevelopmentInstance`, {
          publicIp: process.env.PUBLIC_IP,
        });
        break;
      case 'app-instance':
        if (!process.env.PUBLIC_IP) {
          throw new Error(
            `PUBLIC_IP environment variable must be provided to deploy dev EC2 instance`,
          );
        }
        new InstanceConstruct(this, `${id}-AppInstance`, {
          publicIp: process.env.PUBLIC_IP,
        });
        break;
      default:
        throw new Error(
          `expected DEPLOY_TYPE of app-instance, dev-instance, registry, or cluster but got ${deployType}`,
        );
    }
  }
}

const app = new App();
new CodenamesStack(app, 'Codenames', process.env.DEPLOY_TYPE as DeployType);
