import { Construct } from '@aws-cdk/core';
import { ApplicationLoadBalancedEc2Service } from '@aws-cdk/aws-ecs-patterns';
import {
  Cluster,
  ContainerDependencyCondition,
  ContainerImage,
  Ec2TaskDefinition,
  LogDriver,
  NetworkMode,
} from '@aws-cdk/aws-ecs';
import { Repository } from '@aws-cdk/aws-ecr';
import {
  InstanceClass,
  InstanceSize,
  InstanceType,
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

    // Create an ECS cluster
    const cluster = new Cluster(this, 'Cluster', {
      vpc,
      capacity: {
        instanceType: InstanceType.of(InstanceClass.T3A, InstanceSize.NANO),
        keyName: 'aws_ssh',
      },
    });

    const serviceTask = new Ec2TaskDefinition(this, 'service-task', {
      networkMode: NetworkMode.AWS_VPC,
    });

    const serviceContainer = serviceTask.addContainer('service', {
      image: ContainerImage.fromEcrRepository(
        Repository.fromRepositoryName(this, 'api-service', 'codenames_service'),
      ),
      memoryReservationMiB: 64,
      environment: {
        PORT: '80',
        REDIS_HOST: 'localhost',
        ALLOWED_ORIGINS: `https://${appDnsRecord}`,
      },
      logging: LogDriver.awsLogs({ streamPrefix: 'codenames-service' }),
    });
    serviceContainer.addPortMappings({ containerPort: 80 });

    const redisContainer = serviceTask.addContainer('redis', {
      image: ContainerImage.fromRegistry('redis:6.0.9'),
      memoryReservationMiB: 64,
      healthCheck: { command: ['CMD-SHELL', 'redis-cli PING || exit 1'] },
      logging: LogDriver.awsLogs({ streamPrefix: 'codenames-redis' }),
    });
    redisContainer.addPortMappings({ containerPort: 6369 });

    serviceContainer.addContainerDependencies({
      container: redisContainer,
      condition: ContainerDependencyCondition.HEALTHY,
    });

    new ApplicationLoadBalancedEc2Service(this, 'service', {
      cluster,
      certificate,
      domainZone: hostedZone,
      domainName: serviceDnsRecord,
      redirectHTTP: true,
      taskDefinition: serviceTask,
    });

    const appService = new ApplicationLoadBalancedEc2Service(this, 'app', {
      cluster,
      certificate,
      domainZone: hostedZone,
      domainName: appDnsRecord,
      redirectHTTP: true,
      memoryReservationMiB: 128,
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
