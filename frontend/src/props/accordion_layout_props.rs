use yew::{Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct AccordionLayoutProps {
    pub large: bool,
    pub bordered: bool,
    pub accordion_item_list: Vec<AccordionItemProps>
}

#[derive(Properties, PartialEq, Clone)]
pub struct AccordionItemProps {
    pub title: String,
    pub content: String,
}

impl AccordionLayoutProps{
    pub fn new(
        large: bool,
        bordered: bool,
        accordion_item_list: Vec<AccordionItemProps>
    ) -> Self{
        Self { large, bordered, accordion_item_list }
    }
}

impl AccordionItemProps{
    pub fn new(
        title: String,
        content: String,
    ) -> Self{
        Self{
            title,content
        }
    }
}