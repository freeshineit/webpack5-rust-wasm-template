/* eslint-disable @typescript-eslint/no-var-requires */
const rm = require('rimraf');
const path = require('path');

rm.sync(path.resolve('./docs'));
rm.sync(path.resolve('./dist'));
