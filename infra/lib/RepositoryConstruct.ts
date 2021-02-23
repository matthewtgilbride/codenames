import { Construct } from '@aws-cdk/core';
import { Repository } from '@aws-cdk/aws-ecr';

export class RepositoryConstruct extends Construct {
  constructor(scope: Construct, id: string) {
    super(scope, id);

    new Repository(this, `${id}-serviceRepo`, {
      repositoryName: 'codenames_service',
      imageScanOnPush: true,
      lifecycleRules: [{ maxImageCount: 3 }],
    });

    new Repository(this, `${id}-appRepo`, {
      repositoryName: 'codenames_app',
      imageScanOnPush: true,
      lifecycleRules: [{ maxImageCount: 3 }],
    });
  }
}
