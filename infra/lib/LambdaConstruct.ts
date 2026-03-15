import { Duration } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import {
  Function,
  Runtime,
  Code,
  Architecture,
  FunctionUrl,
  FunctionUrlAuthType,
  HttpMethod,
} from 'aws-cdk-lib/aws-lambda';
import { ITable } from 'aws-cdk-lib/aws-dynamodb';
import * as path from 'path';

export interface LambdaConstructProps {
  table: ITable;
  allowedOrigins: string[];
}

export class LambdaConstruct extends Construct {
  public readonly functionUrl: FunctionUrl;

  constructor(scope: Construct, id: string, props: LambdaConstructProps) {
    super(scope, id);

    const fn = new Function(this, `${id}-Function`, {
      functionName: 'codenames-api',
      runtime: Runtime.PROVIDED_AL2023,
      handler: 'bootstrap',
      architecture: Architecture.X86_64,
      code: Code.fromAsset(
        path.join(__dirname, '../../service/target/lambda/bootstrap'),
      ),
      memorySize: 256,
      timeout: Duration.seconds(10),
      environment: {
        RUST_LOG: 'info',
      },
    });

    props.table.grantReadWriteData(fn);

    this.functionUrl = fn.addFunctionUrl({
      authType: FunctionUrlAuthType.NONE,
      cors: {
        allowedOrigins: props.allowedOrigins,
        allowedMethods: [
          HttpMethod.GET,
          HttpMethod.POST,
          HttpMethod.PUT,
          HttpMethod.DELETE,
        ],
        allowedHeaders: ['*'],
      },
    });
  }
}
