build: lib.c
	gcc `pkg-config --cflags --libs gtk+-3.0` lib.c -shared -o im-fake.so

deploy:
	sudo cp im-fake.so /usr/lib/gtk-3.0/3.0.0/immodules/

