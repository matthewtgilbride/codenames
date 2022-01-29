import { Construct } from 'constructs';
import { AttributeType, Table } from 'aws-cdk-lib/aws-dynamodb';

export class DynamoConstruct extends Construct {
  constructor(scope: Construct, id: string) {
    super(scope, id);

    new Table(scope, `${id}-table`, {
      tableName: 'codenames',
      partitionKey: { name: 'key', type: AttributeType.STRING },
      timeToLiveAttribute: 'ttl',
    });
  }
}
