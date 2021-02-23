import { Construct } from '@aws-cdk/core';
import { ApplicationLoadBalancedEc2Service } from '@aws-cdk/aws-ecs-patterns';
import { Cluster, ContainerImage } from '@aws-cdk/aws-ecs';
import { CfnCacheCluster } from '@aws-cdk/aws-elasticache';
import { Repository } from '@aws-cdk/aws-ecr';
import {
  InstanceClass,
  InstanceSize,
  InstanceType,
  Vpc,
} from '@aws-cdk/aws-ec2';

export class ClusterConstruct extends Construct {
  constructor(scope: Construct, id: string) {
    super(scope, id);

    const redis = new CfnCacheCluster(this, 'redis', {
      cacheNodeType: 'cache.t3.micro',
      engine: 'redis',
      numCacheNodes: 1,
    });

    const vpc = Vpc.fromLookup(this, 'default-vpc', { isDefault: true });

    // Create an ECS cluster
    const cluster = new Cluster(this, 'Cluster', {
      vpc,
      capacity: {
        instanceType: InstanceType.of(InstanceClass.T4G, InstanceSize.MICRO),
      },
    });

    new ApplicationLoadBalancedEc2Service(this, 'service', {
      cluster,
      memoryLimitMiB: 1024,
      taskImageOptions: {
        image: ContainerImage.fromEcrRepository(
          Repository.fromRepositoryName(
            this,
            'api-service',
            'codenames_service',
          ),
        ),
        environment: {
          REDIS_HOST: redis.attrConfigurationEndpointAddress,
          ALLOWED_ORIGINS: 'codenames.mattgilbride.com',
        },
      },
    });

    new ApplicationLoadBalancedEc2Service(this, 'app', {
      cluster,
      memoryLimitMiB: 1024,
      taskImageOptions: {
        image: ContainerImage.fromEcrRepository(
          Repository.fromRepositoryName(this, 'app-service', 'codenames_app'),
        ),
        environment: {
          API_URL: 'https://service.codenames.mattgilbride.com',
        },
      },
    });
  }
}
