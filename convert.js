const fs = require('fs');

const content = fs.readFileSync('src/main.rs', 'utf8');
const marker = '__START_DATA__';
const dataPart = content.slice(content.indexOf(marker) + marker.length)
const joined = dataPart.split('\n').map(line => line.trim()).join('');
const bytes = Buffer.from(joined, 'utf8');
const byteArray = Array.from(bytes).join(',');

const output = `pub const BYTES: &[u8] = &[${byteArray}];`;
fs.writeFileSync('src/bytes.rs', output);
