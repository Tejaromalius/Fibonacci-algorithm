import 'dart:io';

void runAlgo() {
  stdout.write("Please enter Fibonacci index: ");
  int ix = int.parse(stdin.readLineSync()!);

  int currentVal = 1, previousVal = 0;

  for (int counter = 0; counter < ix; counter++) {
    currentVal += previousVal;
    if (counter != 0) {
      previousVal = currentVal - previousVal;
    }
    stdout.write("-> $currentVal ");
  }
  print("");
}

void main() {
  runAlgo();
}

