import { Construct } from 'constructs';
import { Repository } from 'aws-cdk-lib/aws-ecr';
import { RemovalPolicy } from 'aws-cdk-lib';

export class RepositoryConstruct extends Construct {
  constructor(scope: Construct, id: string) {
    super(scope, id);

    new Repository(this, `${id}-serviceRepo`, {
      removalPolicy: RemovalPolicy.DESTROY,
      repositoryName: 'codenames_service',
      imageScanOnPush: true,
      lifecycleRules: [{ maxImageCount: 3 }],
    });

    new Repository(this, `${id}-appRepo`, {
      removalPolicy: RemovalPolicy.DESTROY,
      repositoryName: 'codenames_app',
      imageScanOnPush: true,
      lifecycleRules: [{ maxImageCount: 3 }],
    });
  }
}
