#include <genesis.h>

int main()
{
  VDP_drawText("Hello World!", 10, 13);
  while (1)
  {
    SYS_doVBlankProcess();
  }
  return (0);
}