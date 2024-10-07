#include <stdio.h>
#include <dlfcn.h>
#include <stdlib.h>

typedef const char* lib_name();

void c_fn() {
  printf("Now we are in a c function.\n");
}

int main(int argc, char** argv) {
  void* dl = dlopen("librust_cdylib.dylib", RTLD_NOW|RTLD_GLOBAL);
  if(dl == NULL) dl = dlopen("librust_cdylib.so", RTLD_NOW|RTLD_GLOBAL);

  if(dl == NULL){
    fprintf(stderr, "dlopen failed. %s\n", dlerror());
    return -1;
  }

  lib_name* get_name = (lib_name*)dlsym(dl, "lib_name");
  if(get_name == NULL) {
    fprintf(stderr, "lib_name not found. %s\n", dlerror());
    return -1;
  }

  printf("name: %s\n", get_name());

  
  return 0;
}
