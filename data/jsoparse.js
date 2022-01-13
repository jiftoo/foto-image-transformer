const {readFileSync, accessSync, constants} = require("fs");

const path = process.argv[2];

if (!path) {
	console.log("");
} else {
	try {
		accessSync(path, constants.R_OK);
		const jsObjectAsTest = readFileSync(path).toString();
		console.log(JSON.stringify(eval(jsObjectAsTest)));
	} catch (error) {
		console.log("");
	}
}
