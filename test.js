let fs = require('fs');
let action_validator = require('./pkg');

let src = fs.readFileSync('test-workflow.yml', 'utf8');

console.log(JSON.stringify(action_validator.validateWorkflow(src, true), null, 2));
