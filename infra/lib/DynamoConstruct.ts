import { Construct } from 'constructs';
import { ITable, Table } from 'aws-cdk-lib/aws-dynamodb';

export class DynamoConstruct extends Construct {
  public readonly table: ITable;

  constructor(scope: Construct, id: string) {
    super(scope, id);

    // Import the existing table created by the previous stack
    this.table = Table.fromTableName(this, `${id}-table`, 'codenames');
  }
}
