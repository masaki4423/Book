#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

static void do_count(const char *path);
static void die(const char *s);

int main(int argc, char *argv[])
{
  int i;
  if (argc < 2) {
    fprintf(stderr, "%s: file name not given\n", argv[0]);
    exit(1);
  }
  for (i = 1; i < argc; i++) {
    do_count(argv[i]);
  }
  exit(0);
}

#define BUFFER_SIZE 2048

static void do_count(const char *path)
{
  int fd;
  unsigned char buf[BUFFER_SIZE];
  int n;
  int index;
  unsigned int cnt = 0;

  fd = open(path, O_RDONLY);
  if (fd < 0) die(path);
  for (;;) {
    n = read(fd, buf, sizeof buf);
    if (n < 0) die(path);
    if (n == 0) break;
    for(index = 0; buf[index] != '\0'; index++){
      if (buf[index] == '\n'){
	cnt++;
      }
    }
    printf("%d\n", cnt);
  }
  if (close(fd) < 0) die(path);
}

static void die(const char *s)
{
  perror(s);
  exit(1);
}
