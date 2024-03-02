#[derive(Default)]
pub struct XmlWriter {
    buf: Vec<u8>,
    stack: Vec<String>,
    attrs: bool,
}

impl XmlWriter {
    pub fn declare(&mut self) {
        let decl = b"<?xml version=\"1.0\"?>";
        self.buf.extend_from_slice(decl);
    }

    pub fn attr(&mut self, name: &str, value: &str) {
        self.buf.push(b' ');
        self.buf.extend_from_slice(name.as_bytes());
        self.buf.push(b'=');
        self.buf.push(b'"');
        self.buf.extend_from_slice(value.as_bytes());
        self.buf.push(b'"');
        self.attrs = true;
    }

    pub fn open(&mut self, tag: &str) {
        if self.buf.last() != Some(&b'>') {
            self.buf.push(b'>');
        }

        self.indent();

        self.buf.push(b'<');
        self.buf.extend_from_slice(tag.as_bytes());
        self.stack.push(tag.to_string());
    }

    pub fn close(&mut self) {
        if self.stack.is_empty() {
            return;
        }

        let tag = self.stack.pop().unwrap();
        let needle = format!("<{}", tag);

        if self.buf.ends_with(needle.as_bytes()) || self.attrs {
            self.buf.push(b'/');
            self.buf.push(b'>');
            return;
        }

        if self.buf.last() == Some(&b'>') {
            self.indent();
        }

        self.buf.push(b'<');
        self.buf.push(b'/');
        self.buf.extend_from_slice(tag.as_bytes());
        self.buf.push(b'>');
    }

    pub fn text(&mut self, text: &str) {
        if self.buf.last() != Some(&b'>') {
            self.buf.push(b'>');
        }

        self.buf.extend_from_slice(text.as_bytes());
        self.attrs = false;
    }

    pub fn write_comment(&mut self, text: &str) {
        self.indent();

        self.buf.extend_from_slice(b"<!--");
        self.buf.extend_from_slice(text.as_bytes());
        self.buf.extend_from_slice(b"-->");
    }

    pub fn end(&mut self) -> String {
        while !self.stack.is_empty() {
            self.close();
        }

        String::from_utf8(self.buf.clone()).unwrap_or_default()
    }

    #[inline]
    fn indent(&mut self) {
        self.buf.push(b'\n');

        for _ in 0..self.stack.len() * 4 {
            self.buf.push(b' ');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_writer() {
        let mut w = XmlWriter::default();

        w.declare();
        w.open("compendium");
        w.attr("xmlns:exsl", "http://exslt.org/common");
        w.attr("version", "5");
        w.attr("auto_indent", "NO");
        w.open("item");
        w.open("name");
        w.text("Copper (c)");
        w.close();
        w.open("text");
        w.close();
        w.write_comment("Cash money");
        w.open("type");
        w.text("$");

        let xml = w.end();
        let expected = r#"<?xml version="1.0"?>
<compendium xmlns:exsl="http://exslt.org/common" version="5" auto_indent="NO">
    <item>
        <name>Copper (c)</name>
        <text/>
        <!--Cash money-->
        <type>$</type>
    </item>
</compendium>"#;

        assert_eq!(xml, expected);
    }
}
