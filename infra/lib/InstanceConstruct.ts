import { readFileSync } from 'fs';
import * as path from 'path';
import { Construct } from 'constructs';
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
} from 'aws-cdk-lib/aws-ec2';
import { ManagedPolicy, Role, ServicePrincipal } from 'aws-cdk-lib/aws-iam';

export class InstanceConstruct extends Construct {
  public instanceDnsName: string;

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
    securityGroup.addIngressRule(Peer.anyIpv4(), Port.tcp(8080));

    const userData = UserData.forLinux();
    const installScript = readFileSync(
      path.resolve(process.cwd(), 'app_user_data.sh'),
      { encoding: 'utf-8' },
    );
    userData.addCommands(installScript);

    const machineImage = MachineImage.genericLinux({
      // ubuntu 20.04 x86
      'us-east-1': 'ami-0c4f7023847b90238',
    });

    const instance = new Instance(this, 'codenames instance', {
      vpc,
      userData,
      machineImage,
      instanceType: InstanceType.of(InstanceClass.T3A, InstanceSize.NANO),
      availabilityZone: 'us-east-1b',
      keyName: 'aws_ssh',
      securityGroup,
      blockDevices: [
        {
          deviceName: '/dev/sda1',
          volume: BlockDeviceVolume.ebs(8),
        },
      ],
      role: new Role(this, 'ecr-role', {
        managedPolicies: [
          ManagedPolicy.fromAwsManagedPolicyName('AmazonDynamoDBFullAccess'),
        ],
        assumedBy: new ServicePrincipal('ec2.amazonaws.com'),
      }),
    });

    this.instanceDnsName = instance.instancePublicDnsName;
  }
}
