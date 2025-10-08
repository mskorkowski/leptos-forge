//! Handles custom Markdown elements for `leptos-forge`
//! 

#[cfg(test)]
mod tests;

/// Function used to match the tokens in the Markdown parser
/// 
/// Matching always starts at the beginning of the string
pub type Matcher = for<'source> fn(&'source str)->Option<MarkdownToken<'source>>;

/// Enum holding various matched results for Markdown   
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkdownToken<'source> {
    /// Token to be emitted if the 
    /// `<Story of="subpath" />` 
    Story{
        /// Subpath of the story
        /// 
        /// If we can't find subpath to the story then this will be `None`
        story: Option<&'source str>,
        /// Length of matching text from the original Markdown source
        len: usize,
        /// Wherever the control panel should be shown
        controls: bool,
    },
    /// Chunk of the Markdown
    Markdown{
        /// Raw markdown text
        text: &'source str,
        /// Length of matching text from the original Markdown source
        len: usize,
    },
    /// Markdown header
    Header {
        /// nesting level
        level: usize,
        /// Title
        text: &'source str,
        /// Length of matching text from the original Markdown source
        len: usize,
    }
}

impl<'source> MarkdownToken<'source> {
    /// Returns the length of token
    /// 
    /// - **Length of token** - is the length of matched string from the source
    pub fn len(&self) -> usize {
        use MarkdownToken::*;
        match self {
            Story{ len, ..} | Markdown { len, .. } | Header{ len, ..} => *len
        }
    }
}

/// Maximum length of code fence size we are willing to support
const MAX_TAG_SEQUENCE_SIZE_CODE_FENCE: usize = 1024;
/// Maximum length of header level we are going to detect
/// 
/// Markdown supports up to level 6 so 8 is good enough to make sure we can 
/// ignore things being almost a header, but not
const MAX_TAG_SEQUENCE_SIZE_HEADER: usize = 8;

/// Custom syntax parser for Markdown
/// 
/// # Currently it does
/// 
/// - Detect headers so we can generate section toc (currently only `#` one)
/// - Detect custom nodes like `<Canvas>` so we can render the stories
/// - Whatever we don't render directly it will be returned as a big chunk
///   of Markdown so it can be handled by the external markdown parser directly
/// 
/// # Limitations
/// 
/// We ignore all headers which are not part of the main level. So headers
/// embedded in lists, headers in citations, ...
/// 
/// We don't support the `===` and `---` headers.
/// 
/// ## Limits on code block fences
/// 
/// Code block fences detection caps out at the [MAX_TAG_SEQUENCE_SIZE_CODE_FENCE]
/// number of characters. If your code fence is above this size
/// 
/// 1. You are trying to break the parsing
/// 2. Whatever splitting we do for the rest of the Markdown document is undefined.
///    1. Output can be total garbage - because we will match wrong code guard
///    2. Output can be almost correct - just a bit of code fence at the start and end
///       of the code block
///    3. Anything in between - part of the page goes bonkers
/// 
/// ## Limitations of the `<Story />` tag parsing
/// 
/// > [!WARNING]
/// >
/// > Examples of Markdown code below are broken. They work due to bugs/limitations 
/// > of the software today. Fixing the code so they will no longer work is planned
/// > in the near future.
/// >
/// > We don't consider removing these bugs to be a breaking changes.
/// 
/// Due to the way the `<Story />` tag parsing is implemented there are some
/// limitations/bugs
/// 
/// ### Parsing `of` attribute
/// 
/// If you write your story like this 
/// 
/// ```markdown
/// <Story whatever="some value of="subpath/to/the/story" />
/// ```
/// 
/// Then we will find the `of` attribute with `subpath/to/the/story` being path
/// to the story.
/// 
/// ### Parsing boolean attributes
/// 
/// We currently support boolean attributes only in form of the attribute name
/// 
/// So only valid syntax for such attributes, for example `controls` looks like
/// 
/// ```markdown
/// <Story of="subpath/to/story" controls />
/// ```
/// 
/// Second issue comes from the way we look up for the boolean attributes. If you
/// write a story tag
/// 
/// ```markdown
/// <Story of="mechanism which controls the world" />
/// ```
/// 
/// It will also enable controls for your story.
pub struct MarkdownParser{
    /// List of matchers used to parse the markdown code
    matchers: Vec<Matcher>,
}

impl MarkdownParser {
    /// Create new instance of the parser
    pub fn new() -> Self
    {
        Self{ 
            matchers: vec![
                | source | { header(source, false) },
                story,
                sink
            ]
        }
    }

    /// Parses the markdown and returns the list of tokens for further handling
    pub fn parse<'source>(&self, source: &'source str) -> Vec<MarkdownToken<'source>> {
        let mut result = Vec::new();
        let mut rest = source;

        // Initial parse of the header.
        if let Some(header) = header(rest, true) {
            result.push(header);
            rest = &rest[header.len()..];
        }

        while !rest.is_empty() {
            for matcher in &self.matchers {
                if let Some(token) = matcher(rest) {
                    rest = &rest[token.len() ..];
                    result.push(token);
                    break;
                }
            }
        }

        result
    }
}

/// Definition of the code block guard or header
/// 
/// For the `~~~` the gourd will be
/// 
/// ```rust,ignore
/// # // doctest can't link against private 
/// # leptos_forge::views::section::markdown::TagSequence;
/// 
/// TagSequence {
///   len: 3,
///   character: '~',
/// }
/// ```
struct TagSequence<'source> {
    /// length of code block guard
    len: usize,
    /// code guard sequence
    guard: &'source str,
}

impl<'source> TagSequence<'source> {
    /// Reads the codeblock guard at the beginning of the `source`
    /// 
    /// This is internal function for the [MarkdownParser] logic.
    ///
    /// # Safety
    ///  
    /// `source` must be at the start at the new line before the potential code 
    /// block or header, otherwise the result is undefined
    #[allow(unsafe_code)]
    unsafe fn parse(source: &'_ str, max: usize, skip: usize) -> TagSequence<'_> {
        let mut len = 1;
        let chars = source.chars();
        let mut chars = chars.skip(skip);

        let guard = chars.next().unwrap(); // we are at the start of the tag guarantee of
                                                 // new line and another character which might
                                                 // be repeated

        for char in chars {
            if char != guard {
                break;
            }
            len += 1;

            if len >= max {
                break;
            }
        }

        TagSequence{
            len, 
            guard: &source[..len],
        }
    }
}

/// Function returns a chunk of markdown until it finds the header or a custom
/// tag
/// 
/// This method always returns at least 1 character. Being here means we have 
/// some Markdown code which we need to match. The return type of `Option` is just
/// to match the `MarkdownParser::matchers` spec.
fn sink(source: &'_ str) -> Option<MarkdownToken<'_>> {
    let mut current = 1; // we know that we don't match at idx for
                                // any matcher in the parser so at least
                                // 1 character needs to go down the sink
    let length = source.len();
    
    while current < length {
        if source.is_char_boundary(current) {
            let slice = &source[current..];

            if !slice.starts_with("\n") &&
               !slice.starts_with("<Story ") && 
               !slice.starts_with("<Story\n") { // we are only interested in cases of
                                                // 
                                                // - headers - and that matches at the
                                                //   start of the line, by Markdown
                                                // - custom tags which we require to start
                                                //   with `<`
                                                //
                                                // So any character which doesn't match
                                                // these conditions can't be a start of
                                                // the interesting sequence

                current += 1;
                continue;
            }

            if slice.starts_with("\n```") || // we need to handle the code blocks gracefully
               slice.starts_with("\n~~~") {  // otherwise we won't be able to write Markdown examples

                // safety: This call is safe because we've just checked that slice
                // starts with code block guard sequence in if statement
                #[allow(unsafe_code)]
                let open_guard = unsafe {
                    TagSequence::parse(slice, MAX_TAG_SEQUENCE_SIZE_CODE_FENCE, 1) // skip new line character
                };

                // we create a slice after the code guard so we don't find opening
                // code guard which we already matched
                let markdown_slice = &slice[open_guard.len + 1..]; // +1 for the starting new line character

                if let Some(end) =  markdown_slice.find(open_guard.guard){
                    // We found end of the code block
                    current += 2*(open_guard.len) + 1 + // +1 for the starting new line character
                               end;
                    
                    // all of what we could do with this slice
                    continue; 
                }
                else {
                    // Unclosed code block guard 
                    return Some(MarkdownToken::Markdown{ 
                        text: source,
                        len: source.len()
                    });
                }

            }

            if slice.starts_with("\n#") || // we must preserve the new-line because the
                                           // markdown required for header sto start at column 0 of the
                                           // block
               slice.starts_with("<Story ") || 
               slice.starts_with("<Story\n") { // this is a header
                // We are not in a code block, so that's it folks
                return Some(MarkdownToken::Markdown{ 
                    text: &source[..current],
                    len: current,
                });
            }

        }

        current+=1;

    }

    // we've reached the end of the source without finding next candidate for
    // custom tag handling
    Some(MarkdownToken::Markdown{ 
        text: source,
        len: source.len(),
    })
}

/// Matches markdown `#` header
/// 
/// # Arguments
/// 
/// - source - The markdown document
/// - initial - We expect this to be set to true 
/// 
/// # Parsing details
/// 
/// Markdown requires that `#` header is at the col 0 of the nesting level
/// Because we don't parse the whole syntax, just a selected parts, we don't
/// know "nesting level" of the text.
/// 
/// We detect that a block of text is a header if it starts with a `#` after
/// the new line character (only main body headers).
/// 
/// So the following example will be detected
/// 
/// ```markdown
/// 
/// ### Header
/// 
/// ```
/// 
/// and this will be not
/// 
/// ```markdown
/// 
/// > ### Header
/// 
/// ```
/// 
/// Since we are detecting a sequence `\n#` that leads to a problem,
/// a header starting a Markdown document from the first character. For this 
/// we special case if we should detect a header without the `\n` character before
/// `#` using the `initial` argument
fn header(source: &'_ str, initial: bool) -> Option<MarkdownToken<'_>> {
    if source.starts_with("\n#") ||
       initial && source.starts_with("#") {

        let new_line_len = if source.starts_with("\n#") { 1 }
            else { 0 };

        // safety: We've just checked if we have a two char sequence 
        // `new_line` + `#` which can repeat
        #[allow(unsafe_code)]
        let header_guard = unsafe {
            TagSequence::parse(source, MAX_TAG_SEQUENCE_SIZE_HEADER, new_line_len)
        };

        let header_value = &source[header_guard.len + new_line_len ..]; // +1 for the starting new line character

        if header_value.starts_with(" ") {
            // there is a space after the last hash, we can go the next new line
            // and be happy
            //
            // we must be careful to not count the `\n` as part of the header, since
            // it will break the parsing of the next tokens otherwise
            if let Some(new_line) = header_value.find("\n") {
                return Some(MarkdownToken::Header{
                    level: header_guard.len,
                    text: &header_value[..new_line],
                    len: header_guard.len + new_line_len + new_line
                });
            }
            else {
                // No new line after space, we are at the end of the document and
                // last thing is this header
                return Some(MarkdownToken::Header{
                    level: header_guard.len,
                    text: header_value,
                    len: header_guard.len + new_line_len + header_value.len()
                });
            }
        }

        if header_value.starts_with("\n") {
            // Not really well defined case in the Markdown standard. Many parsers
            // considers this to be empty header, so we are going to follow
            //
            // we must be careful to not count the `\n` as part of the header, since
            // it will break the parsing of the next tokens otherwise
            return Some(MarkdownToken::Header{ 
                level: header_guard.len, 
                text: "",
                len: header_guard.len + new_line_len
            });
        }
    }

    None
}


/// Of attribute which was found
#[derive(Debug, PartialEq, Eq)]
struct OfAttribute<'source> {
    /// Subpath to which the `of` attribute points at
    subpath: &'source str,
}

/// Methods checks if at source[index] is the start of `of` attribute
fn parse_of_attribute(source: &'_ str) -> Option<OfAttribute<'_>> {
    if source[..2].to_ascii_lowercase().starts_with("of") {
        let mut rest = source[2..].chars().enumerate();

        let mut current = rest.next();

        while let Some((_, char)) = current &&
                char.is_whitespace()
        {
            current = rest.next();
        }

        if let Some((_, eq)) = current && 
           eq != '=' {
            return None;
        }

        current = rest.next();

        while let Some((_, char)) = current &&
                char.is_whitespace()
        {
            current = rest.next();
        }

        if let Some((idx, quote)) = current &&
            (quote == '\'' || quote == '"') {

            let value = &source[idx+2+1..]; // + 2 cause rest starts after "of" 
                                                  // + 1 cause we skip the opening quotation mark
            
            if let Some(close) = value.find(quote) {
                let subpath = &value[..close];

                return Some(OfAttribute{
                    subpath
                })
            }
        }



        return None;
    }
    

    None
}

/// Searches for the boolean attribute in the `<Story />` tag
/// 
/// # Arguments
/// 
/// - **attribute** - the name of the attribute
/// - **source** - content of the tag
/// 
/// # Limitations
/// 
/// Currently we are just searching for the `attribute` inside of the content
/// of the `<Story />`. Assuming we are searching for `controls` attribute, if 
/// you write your story tag like this
/// 
/// ```markdown
/// 
/// <Story of="my controls are/awesome" />
/// 
/// ```
/// 
/// You will also enable the controls panel
fn parse_bool_attribute(attribute: &'_ str, source: &'_ str) -> bool {
    let mut source = source;
    while let Some(idx) = source.find(attribute) {

        let start: usize = if idx == 0 {
            // match is at the beginning of the source, so we don't have to check 
            // the character before the attribute string
            0
        }
        else {
            idx - 1
        };

        let end = if idx + attribute.len() == source.len() { //we are at the last word in source
            source.len()
        }
        else {
            idx + attribute.len() + 1
        };

        let attribute_name = &source[start..end];
        let mut attribute_chars = attribute_name.chars();
        
        if let Some(first_char) = attribute_chars.next() &&
            (idx == 0 || first_char.is_whitespace()) {

            // -1 for the first character which we already verified is a whitespace
            // -1 for the last character which we still need to check is a whitespace
            let mut attribute_chars = attribute_chars.skip(attribute_name.len() -1 -1); 

            let last_char = attribute_chars.next();

            if let Some(last_char) = last_char &&
               last_char.is_whitespace() {
                
                return true;
            }
            else if end == source.len() {
                return true;
            }
        }

        source = &source[idx + attribute.len() ..];

        while source.starts_with(attribute) {
            // removes the regex case consecutive spellings of the attribute name
            // `attribute(attribute)+`
            source = &source[attribute.len() ..]; 
        }

    }

    false
}

/// Finds an `of` attribute in the source
fn find_of_attribute(source: &'_ str) -> Option<&'_ str> {

    if let Some(attribute) = parse_of_attribute(source) {
        return Some(attribute.subpath);
    }
    
    let mut rest = source;
    while !rest.is_empty() {
        let of_pos = rest.find("of");
        if let Some(idx) = of_pos {
            if idx == 0 {
                rest = &rest[1..]; // if starts at 0 and attribute is is None that
                                   // means it's not an of attribute value
                continue;
            }
            else {
                // unwrap is safe, we are checking character before idx. Since
                // rest[idx] exists, then rest[idx - 1] also since idx > 0;
                let check = &rest[idx -1 .. idx].chars().next().unwrap();
                if !check.is_whitespace() {
                    rest = &rest[1..];
                    continue;
                }
                else {
                    // we found starting `of`` (maybe)
                    //
                    // I hope nobody will be smart enough to write an attribute
                    // like `ble=" of="my/story"`
                    // https://github.com/mskorkowski/leptos-forge/issues/62
                    let of = &rest[idx..];
                    if let Some(of) = parse_of_attribute(of) {
                        return Some(of.subpath);
                    }
                }
            }
        }
        else {
            return None
        }
    }

    None
}

/// Reads the custom `<Story />` tag
fn story<'source>(source: &'source str) -> Option<MarkdownToken<'source>> {
    if source.starts_with("<Story") {
        if let Some(end) = source.find("/>") {
            let tag = &source["<Story".len() .. end];
            let story = find_of_attribute(tag);
            let controls = parse_bool_attribute("controls", tag);

            return Some(MarkdownToken::Story{ 
                story, 
                len: end+2,
                controls
            });
        }
        else {
            return None; // This is not a `<Story />` tag since it's unclosed
                         // and tags must be closed
        }
    }

    None
}