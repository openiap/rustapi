const { execFileSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const dir = path.join(__dirname, 'lib');
if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
}
try {
    execFileSync('openiap-bootstrap', ['--dir', dir], { stdio: 'inherit' });
} catch (e) {
    console.warn('openiap-bootstrap failed', e);
}
