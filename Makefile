binFile = target/debug/ft_ls

run:
	cargo build
	sudo ./$(binFile) $(ARGS)

clean:
	cargo clean