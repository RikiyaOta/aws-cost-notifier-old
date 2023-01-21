import {App, Stack, StackProps} from 'aws-cdk-lib'
import { Architecture, Code, Function, Runtime, Tracing } from 'aws-cdk-lib/aws-lambda';
import { Effect, PolicyStatement } from 'aws-cdk-lib/aws-iam';
import { Construct } from 'constructs';

export class AwsCostNotifierStack extends Stack {
    constructor(scope: Construct, id: string, props?: StackProps) {
        super(scope, id, props)

        const lambdaFunction = new Function(this, 'aws-cost-notifier-lambda-function', {
            runtime: Runtime.PROVIDED_AL2,
            architecture: Architecture.ARM_64,
            functionName: 'aws-cost-notifier',
            description: 'AWS 料金の通知を行う。(RikiyaOta 実装)',
            handler: 'aws-cost-notifier',
            code: Code.fromAsset(`${__dirname}/../target/lambda/aws-cost-notifier`),
            tracing: Tracing.ACTIVE,
        })

        lambdaFunction.addToRolePolicy(new PolicyStatement({
            effect: Effect.ALLOW,
            actions: ['ce:GetCostAndUsage'],
            resources: ['*']
        }))
    }
}

const app = new App();

new AwsCostNotifierStack(app, 'aws-cost-notifier-stack');

app.synth();