a.out: *.c
	cc -o $@ $^
	cd rust-cdylib && cargo b
	ls -1 rust-cdylib/target/debug/librust_cdylib.* | grep --regexp='\.dylib$$' --regexp='\.so$$' | xargs -J src cp src .

clean:
	ls -1 | grep -e '\.out$$' -e '\.dylib$$' -e '\.so$$' | xargs -J tar rm tar
	cd rust-cdylib && cargo clean

