#include <stdio.h>
#include <unistd.h>
#include <sys/ioctl.h>

unsigned short terminal_width(void) {
    struct winsize w;

    ioctl(STDOUT_FILENO, TIOCGWINSZ, &w);

    return w.ws_col;
}
