#include <stdio.h>

void main(void){
  short sum;

  __asm{
        mov ax, 0
        mov bx, 1
  LOOP1: add ax, bx
        inc bx
        cmp bx, 11
        jne LOOP1
        mov sum, ax
  }
  printf("sum=%d\n", sum);
}
