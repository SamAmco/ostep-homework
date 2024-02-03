#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <string.h>
#include <sys/wait.h>

int main() {
	int forked = fork();
	
	if (forked < 0) {
		//An error occurred
		return 1;
	} else if (forked == 0) {
		//We're in the child process
		printf("Hello\n");
	} else {
		//We're in the parent process
		wait(NULL);
		printf("Goodbye\n");
	}
}
