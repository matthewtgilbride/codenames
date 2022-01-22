import { Duration } from 'aws-cdk-lib';
import { StringParameter } from 'aws-cdk-lib/aws-ssm';
import {
  HostedZone,
  RecordSet,
  RecordTarget,
  RecordType,
} from 'aws-cdk-lib/aws-route53';
import { Certificate } from 'aws-cdk-lib/aws-certificatemanager';
import {
  AllowedMethods,
  CacheHeaderBehavior,
  CachePolicy,
  Distribution,
  OriginProtocolPolicy,
  OriginRequestPolicy,
  ViewerProtocolPolicy,
} from 'aws-cdk-lib/aws-cloudfront';
import { HttpOrigin } from 'aws-cdk-lib/aws-cloudfront-origins';
import { Construct } from 'constructs';

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
        cachePolicy: CachePolicy.CACHING_DISABLED,
        originRequestPolicy: OriginRequestPolicy.ALL_VIEWER,
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
          ),
          minTtl: Duration.seconds(0),
          defaultTtl: Duration.seconds(0),
        }),
        originRequestPolicy: OriginRequestPolicy.ALL_VIEWER,
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
