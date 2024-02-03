#include <stdio.h>
#include <unistd.h>

int main() {
	int x = 100;

	int forked = fork();

	if (forked < 0) {
		//An error occurred
		return 1;
	} else if (forked == 0) {
		//We're in the child process
		x = 5;
		printf("Value of x in child: %d\n", x);
		return 0;
	} else {
		//We're in the parent process
		x = 6;
		printf("Value of x in parent: %d\n", x);
	}
}
