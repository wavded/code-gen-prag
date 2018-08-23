var fs = require('fs')
var contents = fs.readFileSync('product.txt').toString()

var code = contents.split('\n').map(line => {
  var [command, ...data] = line.split(/\s+/)

  switch (command) {
  case '#':
    return `// ${line.substr(2)}`
  case 'M':
    return `class ${data[0]} {\n  constructor() {`
  case 'F':
    return `    this.${data[0]} // ${data[1]}`
  case 'E':
    return `  }\n}`
  }
})

console.log(code.join('\n'))
