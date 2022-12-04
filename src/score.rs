#[cfg(test)]
mod tests {

    #[test]
    fn f() {
        let mut document = svg::Document::new();

        svg::save("image.svg", &document).unwrap();
    }
}
