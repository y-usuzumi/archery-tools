import path = require('path');

export function cdkRootDir(): string {
    return path.join(__dirname, '..');
}