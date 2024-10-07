c_lib.so: *.c
	cc -o $@ -fPIC -dynamiclib -Wl,-undefined,dynamic_lookup $^
	cp $@ ../

clean: c_lib.so
	rm $^
