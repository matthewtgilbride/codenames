import { Duration, Fn, RemovalPolicy } from 'aws-cdk-lib';
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
  OriginRequestHeaderBehavior,
  OriginRequestPolicy,
  OriginRequestQueryStringBehavior,
  ViewerProtocolPolicy,
} from 'aws-cdk-lib/aws-cloudfront';
import { HttpOrigin } from 'aws-cdk-lib/aws-cloudfront-origins';
import { Construct } from 'constructs';
import { BlockPublicAccess, Bucket } from 'aws-cdk-lib/aws-s3';
import { FunctionUrl } from 'aws-cdk-lib/aws-lambda';

export interface CloudfrontConstructProps {
  functionUrl: FunctionUrl;
  domainName: string;
  certificateArn: string;
}

export class CloudfrontConstruct extends Construct {
  constructor(scope: Construct, id: string, props: CloudfrontConstructProps) {
    super(scope, id);

    const { functionUrl, domainName, certificateArn } = props;
    const zone = HostedZone.fromLookup(this, `${id}-HostedZone`, {
      domainName,
    });

    const certificate = Certificate.fromCertificateArn(
      this,
      'cert',
      certificateArn,
    );

    const appDnsRecord = `codenames.${domainName}`;
    const serviceDnsRecord = `codenamesapi.${domainName}`;

    // Content bucket
    // Skip bucketName during initial synth pass when SSM returns dummy values
    const isDummyValue = domainName.includes('dummy-value');
    const siteBucket = new Bucket(this, `${id}-AppBucket`, {
      ...(!isDummyValue && { bucketName: appDnsRecord }),
      websiteIndexDocument: 'index.html',
      websiteErrorDocument: 'index.html',
      publicReadAccess: true,
      blockPublicAccess: BlockPublicAccess.BLOCK_ACLS,
      removalPolicy: RemovalPolicy.DESTROY,
    });

    const appDist = new Distribution(this, 'AppDist', {
      certificate,
      defaultBehavior: {
        origin: new HttpOrigin(siteBucket.bucketWebsiteDomainName, {
          protocolPolicy: OriginProtocolPolicy.HTTP_ONLY,
        }),
        viewerProtocolPolicy: ViewerProtocolPolicy.REDIRECT_TO_HTTPS,
      },
      domainNames: [appDnsRecord],
    });

    // Extract hostname from Lambda Function URL token: "https://abc123.lambda-url.us-east-1.on.aws/"
    const lambdaOriginDomain = Fn.select(2, Fn.split('/', functionUrl.url));

    const serviceDist = new Distribution(this, 'ServiceDist', {
      certificate,
      defaultBehavior: {
        origin: new HttpOrigin(lambdaOriginDomain, {
          protocolPolicy: OriginProtocolPolicy.HTTPS_ONLY,
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
        originRequestPolicy: new OriginRequestPolicy(this, 'Codenames-Api-Origin-Request-Policy', {
          headerBehavior: OriginRequestHeaderBehavior.allowList(
            'Origin',
            'Access-Control-Request-Method',
            'Access-Control-Request-Headers',
          ),
          queryStringBehavior: OriginRequestQueryStringBehavior.all(),
        }),
      },
      domainNames: [serviceDnsRecord],
    });

    new RecordSet(this, 'app-rs', {
      zone,
      recordName: appDnsRecord,
      recordType: RecordType.CNAME,
      ttl: Duration.seconds(60),
      target: RecordTarget.fromValues(appDist.distributionDomainName),
    });

    new RecordSet(this, 'service-rs', {
      zone,
      recordName: serviceDnsRecord,
      recordType: RecordType.CNAME,
      ttl: Duration.seconds(60),
      target: RecordTarget.fromValues(serviceDist.distributionDomainName),
    });
  }
}
