#include <gtk/gtkimmodule.h>
#include <glib-2.0/gobject/gtypemodule.h>

void im_module_init(GTypeModule *module)
{
	system("echo init >> /tmp/log.txt");
	printf("init\n");
}

void im_module_exit(void)
{
	system("echo exit >> /tmp/log.txt");
	printf("exit\n");
}

void im_module_list(const GtkIMContextInfo ***contexts, guint *n_contexts)
{
	system("echo module create >> /tmp/log.txt");
}

void im_module_create(const gchar *context_id)
{
	system("echo module create >> /tmp/log.txt");
	printf("%s\n", context_id);
}
