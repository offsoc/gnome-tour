use crate::utils::i18n_f;
use gettextrs::gettext;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use super::gsd::touchpad_is_present;
use super::pages::{ImagePageWidget, WelcomePageWidget};
use super::paginator::PaginatorWidget;
use crate::config::{APP_ID, PROFILE};

pub struct Window {
    pub widget: libhandy::ApplicationWindow,
    pub paginator: RefCell<Rc<PaginatorWidget>>,
}

impl Window {
    pub fn new(app: &gtk::Application) -> Self {
        let widget = libhandy::ApplicationWindow::new();
        widget.set_application(Some(app));

        let paginator = RefCell::new(PaginatorWidget::new());

        let mut window_widget = Window { widget, paginator };

        window_widget.init();
        window_widget
    }

    pub fn start_tour(&self) {
        self.paginator.borrow_mut().set_page(1);
    }

    pub fn reset_tour(&self) {
        self.paginator.borrow_mut().set_page(0);
    }

    fn init(&mut self) {
        let has_touchpad = touchpad_is_present();
        self.widget.set_default_size(960, 720);
        self.widget.set_icon_name(Some(APP_ID));

        // Devel Profile
        if PROFILE == "Devel" {
            self.widget.get_style_context().add_class("devel");
        }
        if has_touchpad {
            self.widget.get_style_context().add_class("touchpad");
        } else {
            self.widget.get_style_context().add_class("no-touchpad");
        }
        self.paginator
            .borrow_mut()
            .add_page(WelcomePageWidget::new().widget.upcast::<gtk::Widget>());
        self.paginator.borrow_mut().add_page(
            ImagePageWidget::new(
                "/org/gnome/Tour/overview.svg",
                gettext("Get an Overview"),
                gettext("Press Activities to see open windows and apps."),
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        self.paginator.borrow_mut().add_page(
            ImagePageWidget::new(
                "/org/gnome/Tour/makeyourown.svg",
                gettext("Make Apps Your Own"),
                gettext("Arrange your apps so they make sense for you."),
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        self.paginator.borrow_mut().add_page(
            ImagePageWidget::new(
                "/org/gnome/Tour/workspaces.svg",
                gettext("Keep on Top with Workspaces"),
                gettext("Easily organise windows with the new workspaces view."),
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        self.paginator.borrow_mut().add_page(
            ImagePageWidget::new(
                "/org/gnome/Tour/blank.svg",
                if has_touchpad {
                    gettext("Up/Down for the Overview")
                } else {
                    gettext("Logo+Alt+Up/Down for the Overview")
                },
                if has_touchpad {
                    gettext("On a touchpad, use three-finger vertical swipes. Try it!")
                } else {
                    gettext("Write a better string here")
                },
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        self.paginator.borrow_mut().add_page(
            ImagePageWidget::new(
                "/org/gnome/Tour/blank.svg",
                if has_touchpad {
                    gettext("Left/Right for Workspaces")
                } else {
                    gettext("Logo+Alt+Left/Right for Workspaces")
                },
                if has_touchpad {
                    gettext("On a touchpad, use three-finger horizontal swipes. Try it!")
                } else {
                    gettext("Hit some keys. Try it!")
                },
            )
            .widget
            .upcast::<gtk::Widget>(),
        );

        let name = glib::get_os_info("NAME").unwrap_or_else(|| "GNOME".into());
        let last_page = ImagePageWidget::new(
            "/org/gnome/Tour/ready-to-go.svg",
            // Translators: The following string is formated as "We hope that you enjoy GNOME"
            i18n_f("That's it! We hope that you enjoy {}.", &[&name]),
            gettext("To get more advice and tips, see the Help app."),
        );
        last_page.widget.get_style_context().add_class("last-page");
        self.paginator
            .borrow_mut()
            .add_page(last_page.widget.upcast::<gtk::Widget>());

        self.widget.add(&self.paginator.borrow().widget);
    }
}
