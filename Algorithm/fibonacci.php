<?php

function runAlgo() {
    $ix = (int)readline("Please enter Fibonacci index: ");
    
    $currentVal = 1;
    $previousVal = 0;

    for ($counter = 0; $counter < $ix; $counter++) {
        $currentVal += $previousVal;
        if ($counter != 0) {
            $previousVal = $currentVal - $previousVal;
        }
        echo "-> $currentVal ";
    }
    echo PHP_EOL;
}

runAlgo();
?>

