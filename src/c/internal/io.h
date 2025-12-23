#ifndef IO_H
#define IO_H

#define NO_KEY 0
#define TEXT_FORMAT 0x0f00

void print(const char *msg);
void print_str(int row, int col, int format, const char* msg);
void print_char(int row, int col, int format, const char c);

#endif