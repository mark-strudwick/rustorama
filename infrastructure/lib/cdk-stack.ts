import * as path from 'path';
import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { RustFunction } from 'cargo-lambda-cdk';
import { AttributeType, Table } from 'aws-cdk-lib/aws-dynamodb';
import {HttpApi, HttpMethod} from '@aws-cdk/aws-apigatewayv2-alpha';
import {HttpLambdaIntegration} from '@aws-cdk/aws-apigatewayv2-integrations-alpha';

export class CdkStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const ddb = new Table(this, 'pokemon', {
      tableName: 'pokemon',
      partitionKey: {
        name: 'pk',
        type: AttributeType.STRING,
      },
      removalPolicy: cdk.RemovalPolicy.DESTROY, // NOT for production
    });

    const func = new RustFunction(this, 'rust-function', {
      manifestPath: path.join(__dirname, '..', '..'),
      functionName: 'rust-function',
      binaryName: 'rustorama'
    });
    ddb.grantReadWriteData(func);

    const apigw = new HttpApi(this, 'apigw', {
      apiName: 'serverless-rust',
    });

    apigw.addRoutes({
      path: '/{proxy+}',
      methods: [HttpMethod.ANY],
      integration: new HttpLambdaIntegration(
        'function-integration',
        func
      ),
    });

    new cdk.CfnOutput(this, 'function name', {
      value: func.functionName,
    });
    new cdk.CfnOutput(this, 'api endpoint', {
      value: apigw.apiEndpoint,
    });
  }
}
