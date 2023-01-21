import {App, Stack, StackProps} from 'aws-cdk-lib'
import { Architecture, Code, Function, Runtime, Tracing } from 'aws-cdk-lib/aws-lambda';
import { Construct } from 'constructs';

export class AwsCostNotifierStack extends Stack {
    constructor(scope: Construct, id: string, props?: StackProps) {
        super(scope, id, props)

        new Function(this, 'aws-cost-notifier-lambda-function', {
            runtime: Runtime.PROVIDED_AL2,
            architecture: Architecture.ARM_64,
            handler: 'aws-cost-notifier',
            code: Code.fromAsset(`${__dirname}/../target/lambda/aws-cost-notifier`),
            tracing: Tracing.ACTIVE,
        })
    }
}

const app = new App();

new AwsCostNotifierStack(app, 'aws-cost-notifier-stack');

app.synth();