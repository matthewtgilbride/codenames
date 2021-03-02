import { Construct } from '@aws-cdk/core';
import { ApplicationLoadBalancedEc2Service } from '@aws-cdk/aws-ecs-patterns';
import { Cluster, ContainerImage, LogDriver } from '@aws-cdk/aws-ecs';
import { CfnCacheCluster } from '@aws-cdk/aws-elasticache';
import { Repository } from '@aws-cdk/aws-ecr';
import {
  InstanceClass,
  InstanceSize,
  InstanceType,
  Peer,
  Port,
  SecurityGroup,
  Vpc,
} from '@aws-cdk/aws-ec2';
import { StringParameter } from '@aws-cdk/aws-ssm';
import { HostedZone } from '@aws-cdk/aws-route53';
import { Certificate } from '@aws-cdk/aws-certificatemanager';

export class ClusterConstruct extends Construct {
  constructor(scope: Construct, id: string) {
    super(scope, id);

    const domainName = StringParameter.valueFromLookup(this, 'domainName');
    const certificateArn = StringParameter.valueFromLookup(
      this,
      'certificateArn',
    );

    const vpc = Vpc.fromLookup(this, 'default-vpc', { isDefault: true });

    const hostedZone = HostedZone.fromLookup(this, 'hz', { domainName });

    const certificate = Certificate.fromCertificateArn(
      this,
      'cert',
      certificateArn,
    );

    const appDnsRecord = `codenames.${domainName}`;
    const serviceDnsRecord = `codenamesapi.${domainName}`;

    const redisSg = new SecurityGroup(this, 'redis-sg', {
      securityGroupName: `codenames_redis`,
      vpc,
    });
    redisSg.addIngressRule(Peer.ipv4('0.0.0.0/0'), Port.tcp(6379));

    const redis = new CfnCacheCluster(this, 'redis', {
      cacheNodeType: 'cache.t3.micro',
      engine: 'redis',
      numCacheNodes: 1,
      vpcSecurityGroupIds: [redisSg.securityGroupId],
    });

    // Create an ECS cluster
    const cluster = new Cluster(this, 'Cluster', {
      vpc,
      capacity: {
        instanceType: InstanceType.of(InstanceClass.T3A, InstanceSize.SMALL),
        keyName: 'aws_ssh',
      },
    });

    new ApplicationLoadBalancedEc2Service(this, 'service', {
      cluster,
      certificate,
      domainZone: hostedZone,
      domainName: serviceDnsRecord,
      redirectHTTP: true,
      memoryReservationMiB: 256,
      taskImageOptions: {
        image: ContainerImage.fromEcrRepository(
          Repository.fromRepositoryName(
            this,
            'api-service',
            'codenames_service',
          ),
        ),
        environment: {
          PORT: '80',
          REDIS_HOST: redis.attrRedisEndpointAddress,
          ALLOWED_ORIGINS: `https://${appDnsRecord}`,
        },
      },
    });

    const appService = new ApplicationLoadBalancedEc2Service(this, 'app', {
      cluster,
      certificate,
      domainZone: hostedZone,
      domainName: appDnsRecord,
      redirectHTTP: true,
      memoryReservationMiB: 256,
      taskImageOptions: {
        image: ContainerImage.fromEcrRepository(
          Repository.fromRepositoryName(this, 'app-service', 'codenames_app'),
        ),
        environment: {
          API_URL: `https://${serviceDnsRecord}`,
          PORT: '80',
        },
      },
    });

    appService.targetGroup.configureHealthCheck({
      path: '/api/health',
    });
  }
}
