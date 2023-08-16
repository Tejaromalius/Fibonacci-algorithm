# Fibonacci algorithm

## Overview

This repository includes algorithms in *C*, *Java*, and *Python* that take a different approach to the Fibonacci algorithm which is named **worm moves**.

## Explanation

In each file, there exists 3 main variables: **index**, **current_val**, **previous_val**.

When user enters the Fibonacci index, the algorithm iterates from 0 to entered value. In each run, current_val is increased by the previous_val while the previous_val takes the previous current_val’s value. 

```jsx
Set:
	current_val = 1
	previous_val = 0

Loop #1:
		current_val = 1 -> current_val + previous_val = 1 + 0 
		previous_val = 1 -> current_val - previous_val = 1 - 0

Loop #2:
		current_val = 2 = current_val + previous_val = 1 + 1
		previous_val = 1 = current_val - previous_val = 2 - 1

Loop #3:
		current_val = 3 = current_val + previous_val = 2 + 1
		previous_val = 2 = current_val - previous_val = 3 - 1

Loop #4:
		current_val = 5 = current_val + previous_val = 3 + 2
		previous_val = 3 = current_val - previous_val = 5 - 3

Loop #5:
		current_val = 8 = current_val + previous_val = 5 + 3
		previous_val = 5 = current_val - previous_val = 8 - 3

Loop #6:
		current_val = 13 = current_val + previous_val = 8 + 5
		previous_val = 8 = current_val - previous_val = 13 - 5
```