#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <string.h>

int main() {
	int forked = fork();
	
	if (forked < 0) {
		//An error occurred
		return 1;
	} else if (forked == 0) {
		//We're in the child process
		//char* command[3];
		//command[0] = "/bin/ls";
		//command[1] = "-l";
		//command[2] = NULL;
		
		//execv(command[0], command);
		//execl("/bin/ls", "-l", NULL);
		//execlp("ls", "-l", NULL);
		execle("ls", "-l", NULL);
	} else {
		//We're in the parent process
		//char* command[3];
		//command[0] = "/bin/ls";
		//command[1] = "-l";
		//command[2] = NULL;
		//execv(command[0], command);
	}
}
