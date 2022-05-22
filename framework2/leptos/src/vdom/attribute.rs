use crate::{remove_attribute, set_attribute, set_property, IntoProperty, PropertyValue};

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    Attribute(String, Option<String>),
    Class(String, bool),
    Property(String, PropertyValue),
}

impl Attribute {
    pub(crate) fn attr_id(&self) -> (std::mem::Discriminant<Self>, &str) {
        (std::mem::discriminant(self), self.as_name())
    }

    pub fn as_name(&self) -> &str {
        match &self {
            Attribute::Attribute(name, _) => name,
            Attribute::Class(name, _) => name,
            Attribute::Property(name, _) => &name,
        }
    }

    pub fn set(&self, el: &web_sys::Element) {
        match &self {
            Attribute::Attribute(attr_name, value) => match value {
                Some(value) => set_attribute(el, attr_name, value),
                None => remove_attribute(el, attr_name),
            },
            Attribute::Class(class_name, on) => {
                let class_list = el.class_list();
                if *on {
                    class_list.add_1(&class_name);
                } else {
                    class_list.remove_1(&class_name);
                }
            }
            Attribute::Property(prop_name, value) => {
                set_property(el, prop_name, &value.to_property())
            }
        }
    }

    pub fn remove(&self, el: &web_sys::Element) {
        match &self {
            Attribute::Attribute(name, _) => remove_attribute(el, name),
            Attribute::Class(name, _) => {
                el.class_list().remove_1(&name);
            }
            Attribute::Property(name, _) => set_property(el, name, &None),
        }
    }
}

// Attribute generics

pub fn attribute(name: impl std::fmt::Display, value: impl ToAttribute) -> Attribute {
    value.to_attribute(name.to_string())
}

pub trait ToAttribute {
    fn to_attribute(&self, name: String) -> Attribute;
}

auto trait NotBool {}
impl !NotBool for bool {}
auto trait NotOption {}
impl<T> !NotOption for Option<T> {}

impl ToAttribute for bool {
    fn to_attribute(&self, name: String) -> Attribute {
        if *self {
            Attribute::Attribute(name.clone(), Some(name))
        } else {
            Attribute::Attribute(name, None)
        }
    }
}

impl<T> ToAttribute for Option<T>
where
    T: std::fmt::Display,
{
    fn to_attribute(&self, name: String) -> Attribute {
        Attribute::Attribute(name, self.as_ref().map(|value| value.to_string()))
    }
}

impl<T> ToAttribute for T
where
    T: std::fmt::Display + NotBool + NotOption,
{
    fn to_attribute(&self, name: String) -> Attribute {
        Attribute::Attribute(name, Some(self.to_string()))
    }
}
