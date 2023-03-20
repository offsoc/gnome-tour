use gtk::{glib, prelude::*, subclass::prelude::*};

mod imp {
    use std::cell::RefCell;

    use glib::once_cell::sync::OnceCell;

    use super::*;

    #[derive(Debug, Default, glib::Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::ImagePageWidget)]
    #[template(resource = "/org/gnome/Tour/ui/image-page.ui")]
    pub struct ImagePageWidget {
        #[property(get, set= Self::set_resource_uri, construct_only)]
        pub(super) resource_uri: OnceCell<String>,
        #[property(get, set, construct_only)]
        pub(super) head: OnceCell<String>,
        #[property(get, set, construct)]
        pub(super) body: RefCell<Option<String>>,
        #[template_child]
        pub(super) picture: TemplateChild<gtk::Picture>,
        #[template_child]
        pub(super) container: TemplateChild<gtk::Box>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ImagePageWidget {
        const NAME: &'static str = "ImagePageWidget";
        type ParentType = gtk::Widget;
        type Type = super::ImagePageWidget;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.set_layout_manager_type::<adw::ClampLayout>();
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ImagePageWidget {
        fn dispose(&self) {
            self.container.unparent();
        }

        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }
    }
    impl WidgetImpl for ImagePageWidget {}

    impl ImagePageWidget {
        fn set_resource_uri(&self, resource_uri: &str) {
            self.resource_uri.set(resource_uri.to_owned()).unwrap();
            self.picture.set_resource(Some(resource_uri));
        }
    }
}

glib::wrapper! {
    pub struct ImagePageWidget(ObjectSubclass<imp::ImagePageWidget>)
        @extends gtk::Widget;
}
