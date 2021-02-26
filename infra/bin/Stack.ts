#!/usr/bin/env node
import 'source-map-support/register';
import { App, Construct, Stack } from '@aws-cdk/core';
import { ClusterConstruct } from '../lib/ClusterConstruct';
import { RepositoryConstruct } from '../lib/RepositoryConstruct';

type DeployType = 'cluster' | 'registry';

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
      default:
        throw new Error(
          `expected DEPLOY_TYPE of repository or cluster but got ${deployType}`,
        );
    }
  }
}

const app = new App();
new CodenamesStack(app, 'Codenames', process.env.DEPLOY_TYPE as DeployType);
