#!/usr/bin/env node
import 'source-map-support/register';
import { App } from '@aws-cdk/core';
import { CodenamesStack } from '../lib/CodenamesStack';

const app = new App();
new CodenamesStack(app, 'Codenames');
