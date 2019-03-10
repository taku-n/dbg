#import "dbg.dll"
void init();
void get_msg(uchar &p_s[], unsigned long n);
#import

#property service

#define SIZE 256

void OnStart()
{
    uchar s[SIZE];

    init();

    for (;;) {
        get_msg(s, SIZE);
        Print(CharArrayToString(s));
    }
}
