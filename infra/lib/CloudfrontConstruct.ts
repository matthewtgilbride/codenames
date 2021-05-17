import { Construct } from '@aws-cdk/core';
import { Distribution, OriginProtocolPolicy } from '@aws-cdk/aws-cloudfront';
import { HttpOrigin } from '@aws-cdk/aws-cloudfront-origins';

export class CloudfrontConstruct extends Construct {
  constructor(scope: Construct, id: string, instanceDnsName: string) {
    super(scope, id);

    new Distribution(this, 'AppDist', {
      defaultBehavior: {
        origin: new HttpOrigin(instanceDnsName, {
          protocolPolicy: OriginProtocolPolicy.HTTP_ONLY,
          httpPort: 3000,
        }),
      },
    });

    new Distribution(this, 'ServiceDist', {
      defaultBehavior: {
        origin: new HttpOrigin(instanceDnsName, {
          protocolPolicy: OriginProtocolPolicy.HTTP_ONLY,
          httpPort: 8080,
        }),
      },
    });
  }
}
