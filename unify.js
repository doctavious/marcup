var unified = require('unified')
var markdown = require('remark-parse')

var tree = unified().use(markdown).parse('# Hello world!')

console.log(tree)