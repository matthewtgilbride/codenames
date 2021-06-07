import { Construct } from '@aws-cdk/core';
import {
  BlockDeviceVolume,
  Instance,
  InstanceClass,
  InstanceSize,
  InstanceType,
  MachineImage,
  Peer,
  Port,
  SecurityGroup,
  UserData,
  Vpc,
} from '@aws-cdk/aws-ec2';
import { readFileSync } from 'fs';
import * as path from 'path';
import { ManagedPolicy, Role, ServicePrincipal } from '@aws-cdk/aws-iam';

export class DevelopmentInstanceConstruct extends Construct {
  constructor(
    scope: Construct,
    id: string,
    { publicIp }: { publicIp: string },
  ) {
    super(scope, id);

    const vpc = Vpc.fromLookup(this, 'default-vpc', { isDefault: true });

    const securityGroup = new SecurityGroup(this, 'local-ssh-sg', {
      securityGroupName: `codenames local ssh security group`,
      vpc,
    });

    securityGroup.addIngressRule(Peer.ipv4(`${publicIp}/32`), Port.tcp(22));

    const userData = UserData.forLinux();
    const installScript = readFileSync(
      path.resolve(process.cwd(), 'dev_user_data.sh'),
      { encoding: 'utf-8' },
    );
    userData.addCommands(installScript);

    const machineImage = MachineImage.lookup({
      name: 'ubuntu/images/hvm-ssd/ubuntu-focal-20.04-amd64-server-20210223',
    });

    new Instance(this, 'codenames development instance', {
      vpc,
      userData,
      machineImage,
      instanceType: InstanceType.of(InstanceClass.T2, InstanceSize.SMALL),
      availabilityZone: 'us-east-1b',
      keyName: 'aws_ssh',
      securityGroup,
      role: new Role(this, 'ecr-role', {
        managedPolicies: [
          ManagedPolicy.fromAwsManagedPolicyName(
            'AmazonEC2ContainerRegistryFullAccess',
          ),
        ],
        assumedBy: new ServicePrincipal('ec2.amazonaws.com'),
      }),
      blockDevices: [
        {
          deviceName: '/dev/sda1',
          volume: BlockDeviceVolume.ebs(8),
        },
      ],
    });
  }
}
