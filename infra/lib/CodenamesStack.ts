import { Construct, Stack } from '@aws-cdk/core';
import { ClusterConstruct } from './ClusterConstruct';
import { RepositoryConstruct } from './RepositoryConstruct';

export class CodenamesStack extends Stack {
  constructor(scope: Construct, id: string) {
    super(scope, id, {
      env: {
        region: process.env.AWS_DEFAULT_REGION,
        account: process.env.AWS_ACCOUNT_NUMBER,
      },
      stackName: 'Codenames',
    });

    new RepositoryConstruct(this, `${id}-Repositories`);
    new ClusterConstruct(this, `${id}-Cluster`);
  }
}
