# TypeScript bindings for stake-pool program

For use with both node.js and in-browser.

## Installation

```
npm install
```

## Build and run

In the `js` folder:

```
npm run build
```

The build is available at `dist/index.js` (or `dist.browser/index.iife.js` in the browser).

## Browser bundle
```html
<!-- Development (un-minified) -->
<script src="https://unpkg.com/@miraland/solarti-stake-pool@latest/dist.browser/index.iife.js"></script>

<!-- Production (minified) -->
<script src="https://unpkg.com/@miraland/solarti-stake-pool@latest/dist.browser/index.iife.min.js"></script>
```

## Test

```
npm test
```

## Usage

### JavaScript
```javascript
const miralandStakePool = require('@miraland/solarti-stake-pool');
console.log(miralandStakePool);
```

### ES6
```javascript
import * as miralandStakePool from '@miraland/solarti-stake-pool';
console.log(miralandStakePool);
```

### Browser bundle
```javascript
// `miralandStakePool` is provided in the global namespace by the script bundle.
console.log(miralandStakePool);
```
