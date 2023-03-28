#include <gtk/gtkimmodule.h>
#include <glib-2.0/gobject/gtypemodule.h>

void im_module_init(GTypeModule *module)
{
	printf("init\n");
}

void im_module_exit(void)
{
	printf("exit\n");
}

void im_module_list(const GtkIMContextInfo ***contexts, guint *n_contexts)
{
}

void im_module_create(const gchar *context_id)
{
	printf("%s\n", context_id);
}
