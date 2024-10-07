all: *.c
	cc -o a.out $^
	cd rust-cdylib && cargo b
	ls -1 rust-cdylib/target/debug/librust_cdylib.* | grep --regexp='\.dylib$$' --regexp='\.so$$' | xargs -J src cp src .

clean:
	rm a.out
	rm *.dylib
	cd rust-cdylib && cargo clean

