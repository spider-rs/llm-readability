/// Get the content with proper encoding. Pass in a proper encoding label like SHIFT_JIS.
pub fn get_html_encoded(html: &Option<bytes::Bytes>, label: &str) -> String {
    use encoding_rs::CoderResult;

    match html.as_ref() {
        Some(html) => match encoding_rs::Encoding::for_label(label.as_bytes()) {
            Some(enc) => {
                let process = |buffer: &mut str| {
                    let mut bytes_in_buffer: usize = 0usize;
                    let mut output = String::new();
                    let mut decoder = enc.new_decoder();
                    let mut total_read_from_current_input = 0usize;

                    loop {
                        let (result, read, written, _had_errors) = decoder.decode_to_str(
                            &html[total_read_from_current_input..],
                            &mut buffer[bytes_in_buffer..],
                            false,
                        );
                        total_read_from_current_input += read;
                        bytes_in_buffer += written;
                        match result {
                            CoderResult::InputEmpty => {
                                break;
                            }
                            CoderResult::OutputFull => {
                                output.push_str(&buffer[..bytes_in_buffer]);
                                bytes_in_buffer = 0usize;
                                continue;
                            }
                        }
                    }

                    loop {
                        let (result, _, written, _had_errors) =
                            decoder.decode_to_str(b"", &mut buffer[bytes_in_buffer..], true);
                        bytes_in_buffer += written;
                        output.push_str(&buffer[..bytes_in_buffer]);
                        bytes_in_buffer = 0usize;
                        match result {
                            CoderResult::InputEmpty => {
                                break;
                            }
                            CoderResult::OutputFull => {
                                continue;
                            }
                        }
                    }

                    output
                };

                match html.len() {
                    15001..=usize::MAX => {
                        let mut buffer_bytes = [0u8; 2048];
                        process(std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap_or_default())
                    }
                    1000..=15000 => {
                        let mut buffer_bytes = [0u8; 1024];
                        process(std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap_or_default())
                    }
                    _ => {
                        let mut buffer_bytes = [0u8; 512];
                        process(std::str::from_utf8_mut(&mut buffer_bytes[..]).unwrap_or_default())
                    }
                }
                .into()
            }
            _ => Default::default(),
        },
        _ => Default::default(),
    }
}
