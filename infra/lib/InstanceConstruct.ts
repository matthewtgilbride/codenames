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

export class InstanceConstruct extends Construct {
  constructor(
    scope: Construct,
    id: string,
    { publicIp }: { publicIp: string },
  ) {
    super(scope, id);

    const vpc = Vpc.fromLookup(this, 'default-vpc', { isDefault: true });

    const securityGroup = new SecurityGroup(this, 'local-ssh-sg', {
      securityGroupName: `codenames security group`,
      vpc,
    });

    securityGroup.addIngressRule(Peer.ipv4(`${publicIp}/32`), Port.tcp(22));
    securityGroup.addIngressRule(Peer.anyIpv4(), Port.tcp(3000));
    securityGroup.addIngressRule(Peer.anyIpv4(), Port.tcp(8080));

    const userData = UserData.forLinux();
    const installScript = readFileSync(
      path.resolve(process.cwd(), 'app_user_data.sh'),
      { encoding: 'utf-8' },
    );
    userData.addCommands(installScript);

    const machineImage = MachineImage.lookup({
      name: 'ubuntu/images/hvm-ssd/ubuntu-focal-20.04-amd64-server-20210223',
    });

    new Instance(this, 'codenames instance', {
      vpc,
      userData,
      machineImage,
      instanceType: InstanceType.of(InstanceClass.T2, InstanceSize.MICRO),
      availabilityZone: 'us-east-1b',
      keyName: 'aws_ssh',
      securityGroup,
      blockDevices: [
        {
          deviceName: '/dev/sda1',
          volume: BlockDeviceVolume.ebs(8),
        },
      ],
    });
  }
}
