import { Construct, Duration } from '@aws-cdk/core';
import {
  AllowedMethods,
  CacheHeaderBehavior,
  CachePolicy,
  Distribution,
  OriginProtocolPolicy,
  ViewerProtocolPolicy,
} from '@aws-cdk/aws-cloudfront';
import { HttpOrigin } from '@aws-cdk/aws-cloudfront-origins';
import { StringParameter } from '@aws-cdk/aws-ssm';
import { Certificate } from '@aws-cdk/aws-certificatemanager';
import {
  HostedZone,
  RecordSet,
  RecordTarget,
  RecordType,
} from '@aws-cdk/aws-route53';
import { constants } from 'http2';

export class CloudfrontConstruct extends Construct {
  constructor(scope: Construct, id: string, instanceDnsName: string) {
    super(scope, id);

    const domainName = StringParameter.valueFromLookup(this, 'domainName');
    const zone = HostedZone.fromLookup(this, `${id}-HostedZone`, {
      domainName,
    });
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

    const appDist = new Distribution(this, 'AppDist', {
      certificate,
      defaultBehavior: {
        origin: new HttpOrigin(instanceDnsName, {
          protocolPolicy: OriginProtocolPolicy.HTTP_ONLY,
          httpPort: 3000,
        }),
        viewerProtocolPolicy: ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
      },
      domainNames: [appDnsRecord],
    });

    const serviceDist = new Distribution(this, 'ServiceDist', {
      certificate,
      defaultBehavior: {
        origin: new HttpOrigin(instanceDnsName, {
          protocolPolicy: OriginProtocolPolicy.HTTP_ONLY,
          httpPort: 8080,
        }),
        viewerProtocolPolicy: ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
        allowedMethods: AllowedMethods.ALLOW_ALL,
        cachePolicy: new CachePolicy(this, 'Codenames-Api-Cache-Policy', {
          headerBehavior: CacheHeaderBehavior.allowList(
            'Origin',
            'Access-Control-Request-Method',
            'Access-Control-Request-Headers',
            'Access-Control-Allow-Origin',
            'Access-Control-Allow-Method',
          ),
        }),
      },
      domainNames: [serviceDnsRecord],
    });

    new RecordSet(this, 'app-rs', {
      zone,
      recordName: appDnsRecord,
      recordType: RecordType.CNAME,
      // todo: make longer once stable
      ttl: Duration.seconds(60),
      target: RecordTarget.fromValues(appDist.distributionDomainName),
    });

    new RecordSet(this, 'service-rs', {
      zone,
      recordName: serviceDnsRecord,
      recordType: RecordType.CNAME,
      // todo: make longer once stable
      ttl: Duration.seconds(60),
      target: RecordTarget.fromValues(serviceDist.distributionDomainName),
    });
  }
}
