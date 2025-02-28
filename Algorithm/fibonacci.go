package main

import "fmt"

func runAlgo() {
	var ix int
	fmt.Print("Please enter Fibonacci index: ")
	fmt.Scan(&ix)

	currentVal, previousVal := 1, 0

	for counter := 0; counter < ix; counter++ {
		currentVal += previousVal
		if counter != 0 {
			previousVal = currentVal - previousVal
		}
		fmt.Printf("-> %d ", currentVal)
	}
	fmt.Println()
}

func main() {
	runAlgo()
}

