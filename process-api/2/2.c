#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <string.h>

int main() {
	char* filename = "file.txt";
	int file = open(filename, O_CREAT|O_WRONLY|O_TRUNC, S_IWUSR|S_IRUSR);

	if (file == -1) {
		fprintf(stderr, "Error opening %s", filename);
	}

	int forked = fork();
	
	if (forked < 0) {
		//An error occurred
		close(file);
		return 1;
	} else if (forked == 0) {
		//We're in the child process
		char* text = "I am the child process\n";
		for (int i = 0; i < 100; i++) {
			write(file, text, strlen(text));
		}
		close(file);
	} else {
		//We're in the parent process
		char* text = "I am the parent process\n";
		for (int i = 0; i < 10; i++) {
			write(file, text, strlen(text));
		}
		close(file);
	}
}
