build: lib.c
	gcc `pkg-config --cflags --libs gtk+-3.0` lib.c -shared -g -o im-fake.so

# don't need for local testing
deploy:
	sudo cp im-fake.so /usr/lib/gtk-3.0/3.0.0/immodules/

