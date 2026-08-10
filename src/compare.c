#include <stddef.h>
#include <stdio.h>

int main(int argc, char** argv) {
    printf("Hello world in C!\n");
    if(argc>1) {
        printf("You can even print your argv:\n");
        for(size_t i=1;i<argc;++i) {
            printf(" * %s\n",argv[i]);
        }
        printf(":D\n");
    }
    return 0;
}
