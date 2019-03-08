#import "dbg.dll"
void init();
void get_msg(uchar &p_s[], unsigned long n);
#import

#property service

void OnStart()
{
    uchar s[1024];

    init();

    for (;;) {
        get_msg(s, 1024);
        Print(CharArrayToString(s));
    }
}
