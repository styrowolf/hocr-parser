use crate::{element::{Element, ElementBorrowed}, hocr::{HOCRBorrowed, HOCR}};


pub struct ElementsIterator<'a> {
    elements: Vec<&'a Element>,
}

impl<'a> Iterator for ElementsIterator<'a> {
    type Item = &'a Element;

    fn next(&mut self) -> Option<Self::Item> {
        self.elements.pop()
    }
}

pub struct ElementsBorrowedIterator<'a> {
    elements: Vec<&'a ElementBorrowed<'a>>,
}

impl<'a> Iterator for ElementsBorrowedIterator<'a> {
    type Item = &'a ElementBorrowed<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.elements.pop()
    }
}

impl HOCR {
    /// Returns an iterator over all elements in the hOCR document.
    /// Note that the iterator returns elements in a breadth-first order.
    pub fn iter(&self) -> ElementsIterator {
        let mut elements: Vec<&Element> = self.elements.iter().collect();
        let mut index = 0;
        
        while let Some(element) = elements.get(index) {
            elements.extend(element.children.iter());
            index += 1;
        }
        
        ElementsIterator { elements }
    }
}

impl Element {
    /// Returns an iterator over all descendants of this hOCR element.
    /// Note that the iterator returns elements in a breadth-first order.
    pub fn descendants(&self) -> ElementsIterator {
        let mut elements: Vec<&Element> = self.children.iter().collect();
        let mut index = 0;
        
        while let Some(element) = elements.get(index) {
            elements.extend(element.children.iter());
            index += 1;
        }
        
        ElementsIterator { elements }
    }
}

impl<'a> HOCRBorrowed<'a> {
    /// Returns an iterator over all elements in the hOCR document.
    /// Note that the iterator returns elements in a breadth-first order.
    pub fn iter(&self) -> ElementsBorrowedIterator {
        let mut elements: Vec<&ElementBorrowed> = self.elements.iter().collect();
        let mut index = 0;
        
        while let Some(element) = elements.get(index) {
            elements.extend(element.children.iter());
            index += 1;
        }
        
        ElementsBorrowedIterator { elements }
    }
}

impl<'a> ElementBorrowed<'a> {
    /// Returns an iterator over all descendants of this hOCR element.
    /// Note that the iterator returns elements in a breadth-first order.
    pub fn descendants(&self) -> ElementsBorrowedIterator {
        let mut elements: Vec<&ElementBorrowed> = self.children.iter().collect();
        let mut index = 0;
        
        while let Some(element) = elements.get(index) {
            elements.extend(element.children.iter());
            index += 1;
        }
        
        ElementsBorrowedIterator { elements }
    }
}