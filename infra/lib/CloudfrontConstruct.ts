import { Construct } from '@aws-cdk/core';
import { Distribution, OriginProtocolPolicy } from '@aws-cdk/aws-cloudfront';
import { HttpOrigin } from '@aws-cdk/aws-cloudfront-origins';
import { StringParameter } from '@aws-cdk/aws-ssm';
import { Certificate } from '@aws-cdk/aws-certificatemanager';

export class CloudfrontConstruct extends Construct {
  constructor(scope: Construct, id: string) {
    super(scope, id);

    const domainName = StringParameter.valueFromLookup(this, 'domainName');
    const certificateArn = StringParameter.valueFromLookup(
      this,
      'certificateArn',
    );

    const certificate = Certificate.fromCertificateArn(
      this,
      'cert',
      certificateArn,
    );

    const appDnsRecord = `codenames.${domainName}`;
    const serviceDnsRecord = `codenamesapi.${domainName}`;

    new Distribution(this, 'AppDist', {
      certificate,
      defaultBehavior: {
        origin: new HttpOrigin('TODO: EC2 DNS', {
          protocolPolicy: OriginProtocolPolicy.HTTP_ONLY,
          httpPort: 3000,
        }),
      },
      domainNames: [appDnsRecord],
    });

    new Distribution(this, 'ServiceDist', {
      certificate,
      defaultBehavior: {
        origin: new HttpOrigin('TODO: EC2 DNS', {
          protocolPolicy: OriginProtocolPolicy.HTTP_ONLY,
          httpPort: 8080,
        }),
      },
      domainNames: [serviceDnsRecord],
    });
  }
}
