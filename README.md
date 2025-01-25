# USFM-Merge

HTTP APIs to resolve git merge conflicts in USFM files via considering the parse structure. 

Built in rust.

Uses [mergiraf](https://codeberg.org/mergiraf/mergiraf) and [tree-sitter-usfm3](https://crates.io/crates/tree-sitter-usfm3).

## API Usage

## API Usage Guide

The `/resolve` API endpoint processes three input values (`base`, `left`, and `right`) and returns a conflict resolved result.

#### Endpoint
`POST /resolve`

#### Request
The request should include a JSON payload with the following structure:

```json
{
  "base": "string",  // Primary USFM input
  "left": "string",  // Variant or earlier version of USFM
  "right": "string"  // Target or revised version of USFM
}
```

#### Response

On success, the API responds with:
```json
{
  "result": "string"  // Processed result of the inputs
}
```
If the conflict resolution failed, you would see regions of conflict marked in the output string.

When API call fails, you may receive an error message or status code. 

### Example Usage

Here’s how you can send a request using JavaScript's fetch API:
```javascript
const response = await fetch('/resolve', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ base: value1, left: value2, right: value3 })
});

if (response.ok) {
    const data = await response.json();
    console.log('Result:', data.result);
} else {
    console.error('Error processing request');
}
```

