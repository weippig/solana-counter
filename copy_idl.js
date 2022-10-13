// copyIdl.js
const fs = require('fs');
const idl = require('./target/idl/counter.json');

fs.writeFileSync('./app/src/idl.json', JSON.stringify(idl))