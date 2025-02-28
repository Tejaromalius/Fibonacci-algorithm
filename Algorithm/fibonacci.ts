function runAlgo(): void {
    const readline = require('readline-sync');
    const ix: number = parseInt(readline.question("Please enter Fibonacci index: "));

    let currentVal: number = 1, previousVal: number = 0;

    for (let counter = 0; counter < ix; counter++) {
        currentVal += previousVal;
        if (counter !== 0) {
            previousVal = currentVal - previousVal;
        }
        process.stdout.write(`-> ${currentVal} `);
    }
    console.log();
}

runAlgo();

