/*
Simple Perceptron in C
A perceptron is a basic artificial neuron that takes multiple inputs, applies weights, sums them, and passes the result through an activation function (usually a step function).

Simple Implementation

This minimal perceptron uses a fixed learning rate and binary step activation.

How It Works
Takes two inputs and two weights.
Computes weighted sum + bias.
Passes result through step function (1 if positive, 0 otherwise).

*/

#include <stdio.h>

// Activation function: Step function
int step_function(float x) {
	return (x >= 0) ? 1 : 0;
}

// Perceptron function
int perceptron(float inputs[], float weights[], float bias, int num_inputs) {
	float sum = bias;
	for (int i = 0; i < num_inputs; i++) {
		sum += inputs[i] * weights[i];
	}
	return step_function(sum);
}

int main() {
	float inputs[] = {0, 1};   // Example input
	float weights[] = {0.5, -0.5}; // Example weights
	float bias = 0.1;
	
	int output = perceptron(inputs, weights, bias, 2);
	printf("Perceptron Output: %d\n", output);
	
	return 0;
}
