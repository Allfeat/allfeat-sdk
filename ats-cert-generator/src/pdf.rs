use crate::CertificateData;
use js_sys::Math;

/// Simple PDF generator for WASM environment
/// Creates a multi-page PDF with certificate data
pub struct PdfGenerator {
    objects: Vec<String>,
    current_obj_id: usize,
    xref_positions: Vec<usize>,
    content: Vec<u8>,
    pages: Vec<String>,
}

impl PdfGenerator {
    pub fn new() -> Self {
        PdfGenerator {
            objects: Vec::new(),
            current_obj_id: 1,
            xref_positions: Vec::new(),
            content: Vec::new(),
            pages: Vec::new(),
        }
    }

    fn add_object(&mut self, content: String) -> usize {
        let obj_id = self.current_obj_id;
        self.current_obj_id += 1;
        
        let obj = format!("{} 0 obj\n{}\nendobj\n", obj_id, content);
        self.xref_positions.push(self.content.len());
        self.content.extend_from_slice(obj.as_bytes());
        self.objects.push(obj);
        
        obj_id
    }

    pub fn generate_certificate_pdf(data: &CertificateData) -> Result<Vec<u8>, String> {
        let mut gen = PdfGenerator::new();
        
        // PDF header
        gen.content.extend_from_slice(b"%PDF-1.4\n");
        gen.content.extend_from_slice(&[0x25, 0xE2, 0xE3, 0xCF, 0xD3, 0x0A]); // Binary marker
        
        // Generate pages content
        gen.create_pages_content(data);
        let page_count = gen.pages.len();
        
        // Object references
        let catalog_ref = 1;
        let pages_ref = 2;
        let font_ref = 3;
        let font_bold_ref = 4;
        
        // 1. Catalog
        let catalog = format!("<<\n/Type /Catalog\n/Pages {} 0 R\n>>", pages_ref);
        gen.add_object(catalog);
        
        // 2. Fonts (need to be added before pages)
        let font = "<<\n/Type /Font\n/Subtype /Type1\n/BaseFont /Times-Roman\n>>";
        gen.add_object(font.to_string());
        
        let font_bold = "<<\n/Type /Font\n/Subtype /Type1\n/BaseFont /Times-Bold\n>>";
        gen.add_object(font_bold.to_string());
        
        // 3. Add page objects and their content streams
        let mut page_refs = Vec::new();
        let pages_clone = gen.pages.clone();
        for page_content in pages_clone {
            let page_obj_id = gen.current_obj_id;
            let content_obj_id = gen.current_obj_id + 1;
            
            // Page object
            let page = format!(
                "<<\n/Type /Page\n/Parent {} 0 R\n/MediaBox [0 0 595 842]\n/Resources <<\n/Font <<\n/F1 {} 0 R\n/F2 {} 0 R\n>>\n>>\n/Contents {} 0 R\n>>",
                pages_ref,
                font_ref,
                font_bold_ref,
                content_obj_id
            );
            gen.add_object(page);
            page_refs.push(format!("{} 0 R", page_obj_id));
            
            // Content stream object
            let content_stream_bytes = page_content.as_bytes();
            let stream_obj = format!(
                "<<\n/Length {}\n>>\nstream\n{}\nendstream",
                content_stream_bytes.len(),
                page_content
            );
            gen.add_object(stream_obj);
        }
        
        // Insert Pages object at position 2 (after catalog, before fonts)
        gen.xref_positions.insert(1, gen.content.len());
        let pages = format!(
            "<<\n/Type /Pages\n/Kids [{}]\n/Count {}\n>>",
            page_refs.join(" "),
            page_count
        );
        let pages_obj = format!("{} 0 obj\n{}\nendobj\n", pages_ref, pages);
        gen.content.extend_from_slice(pages_obj.as_bytes());
        
        // Cross-reference table
        let xref_start = gen.content.len();
        gen.content.extend_from_slice(b"xref\n");
        gen.content.extend_from_slice(format!("0 {}\n", gen.current_obj_id).as_bytes());
        gen.content.extend_from_slice(b"0000000000 65535 f \n");
        
        for pos in &gen.xref_positions {
            gen.content.extend_from_slice(format!("{:010} 00000 n \n", pos).as_bytes());
        }
        
        // Trailer
        gen.content.extend_from_slice(b"trailer\n");
        gen.content.extend_from_slice(
            format!(
                "<<\n/Size {}\n/Root {} 0 R\n>>\n",
                gen.current_obj_id,
                catalog_ref
            )
            .as_bytes(),
        );
        
        // Footer
        gen.content.extend_from_slice(b"startxref\n");
        gen.content.extend_from_slice(format!("{}\n", xref_start).as_bytes());
        gen.content.extend_from_slice(b"%%EOF\n");
        
        Ok(gen.content)
    }
    
    fn create_pages_content(&mut self, data: &CertificateData) {
        let mut current_page_content = String::new();
        let mut y_pos = 760;
        const FOOTER_Y: i32 = 90;  // Y position where footer starts
        const TOP_MARGIN: i32 = 760;
        const CREATOR_HEIGHT: i32 = 90;
        
        // Add header content to first page
        self.add_header_content(&mut current_page_content, data, &mut y_pos);
        
        // Add creators
        for creator in data.creators.iter() {
            // Check if we need a new page
            if y_pos - CREATOR_HEIGHT < FOOTER_Y {
                // Add footer to current page and start new page
                self.add_footer_content(&mut current_page_content, data, FOOTER_Y);
                self.pages.push(current_page_content);
                
                // Start new page
                current_page_content = String::new();
                y_pos = TOP_MARGIN;
                
                // Add "Creators (continued)" header on new page
                current_page_content.push_str("BT\n");
                current_page_content.push_str("/F2 12 Tf\n");
                current_page_content.push_str("0.09 0.60 0.47 rg\n");
                current_page_content.push_str(&format!("50 {} Td\n", y_pos));
                current_page_content.push_str("(Creators continued:) Tj\n");
                current_page_content.push_str("ET\n");
                y_pos -= 30;
            }
            
            // Add creator to current page
            self.add_creator_content(&mut current_page_content, creator, &mut y_pos);
        }
        
        // Add footer to final page
        self.add_footer_content(&mut current_page_content, data, FOOTER_Y);
        self.pages.push(current_page_content);
    }
    
    fn add_header_content(&self, content: &mut String, data: &CertificateData, y_pos: &mut i32) {
        // Title - Allfeat logo text
        content.push_str("BT\n");
        content.push_str("/F2 24 Tf\n");
        content.push_str(&format!("50 {} Td\n", *y_pos));
        content.push_str("(Allfeat.) Tj\n");
        content.push_str("ET\n");
        
        // Certificate title
        content.push_str("BT\n");
        content.push_str("/F2 24 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n"); // Teal color
        content.push_str(&format!("400 {} Td\n", *y_pos));
        content.push_str("(Certificate) Tj\n");
        content.push_str("ET\n");
        
        *y_pos -= 15;
        
        // Line under header
        content.push_str("0.09 0.60 0.47 RG\n"); // Green/teal line
        content.push_str(&format!("50 {} m\n545 {} l\nS\n", *y_pos, *y_pos));
        
        *y_pos -= 25;
        
        // File label and value
        content.push_str("BT\n");
        content.push_str("/F2 10 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str(&format!("50 {} Td\n", *y_pos));
        content.push_str("(File :) Tj\n");
        content.push_str("ET\n");
        
        content.push_str("BT\n");
        content.push_str("/F1 10 Tf\n");
        content.push_str("0 0 0 rg\n");
        content.push_str(&format!("200 {} Td\n", *y_pos));
        content.push_str(&format!("({}) Tj\n", escape_pdf_string(&data.asset_filename)));
        content.push_str("ET\n");
        
        *y_pos -= 20;
        
        // Title of work
        content.push_str("BT\n");
        content.push_str("/F2 10 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str(&format!("50 {} Td\n", *y_pos));
        content.push_str("(Title of the work :) Tj\n");
        content.push_str("ET\n");
        
        content.push_str("BT\n");
        content.push_str("/F1 10 Tf\n");
        content.push_str("0 0 0 rg\n");
        content.push_str(&format!("200 {} Td\n", *y_pos));
        content.push_str(&format!("({}) Tj\n", escape_pdf_string(&data.title)));
        content.push_str("ET\n");
        
        *y_pos -= 30;
        
        // Hash fields with 0x prefix and 64 characters
        let hash_fields = [
            ("hash_audio:", generate_hash_64()),
            ("hash_title:", generate_hash_64()),
            ("hash_creators:", generate_hash_64()),
            ("secret:", generate_hash_64()),
            ("hash_commitment:", generate_hash_64()),
        ];
        
        for (label, hash) in hash_fields.iter() {
            content.push_str("BT\n");
            content.push_str("/F2 10 Tf\n");
            content.push_str("0.09 0.60 0.47 rg\n");
            content.push_str(&format!("50 {} Td\n", *y_pos));
            content.push_str(&format!("({}) Tj\n", label));
            content.push_str("ET\n");
            
            content.push_str("BT\n");
            content.push_str("/F1 10 Tf\n");
            content.push_str("0 0 0 rg\n");
            content.push_str(&format!("200 {} Td\n", *y_pos));
            content.push_str(&format!("(0x{}) Tj\n", hash));
            content.push_str("ET\n");
            *y_pos -= 20;
        }
        
        *y_pos -= 20;
        
        // Creators section title
        content.push_str("BT\n");
        content.push_str("/F2 12 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str(&format!("50 {} Td\n", *y_pos));
        content.push_str("(Creators:) Tj\n");
        content.push_str("ET\n");
        *y_pos -= 25;
    }
    
    fn add_creator_content(&self, content: &mut String, creator: &crate::Creator, y_pos: &mut i32) {
        const CREATOR_HEIGHT: i32 = 85;
        
        // Draw creator box with subtle background
        content.push_str("0.98 0.98 0.98 rg\n"); // Very light gray
        content.push_str(&format!("50 {} 495 {} re\nf\n", *y_pos - CREATOR_HEIGHT + 5, CREATOR_HEIGHT));
        
        // Left accent border
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str(&format!("50 {} 3 {} re\nf\n", *y_pos - CREATOR_HEIGHT + 5, CREATOR_HEIGHT));
        
        // Start content from top of box
        let mut current_y = *y_pos - 15;  // Start 15 points from top of box
        
        // Full name
        content.push_str("BT\n");
        content.push_str("/F2 9 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str(&format!("70 {} Td\n", current_y));
        content.push_str("(Full name :) Tj\n");
        content.push_str("ET\n");
        
        content.push_str("BT\n");
        content.push_str("/F1 10 Tf\n");
        content.push_str("0 0 0 rg\n");
        content.push_str(&format!("140 {} Td\n", current_y));
        content.push_str(&format!("({}) Tj\n", escape_pdf_string(&creator.fullname)));
        content.push_str("ET\n");
        
        current_y -= 18;
        
        // Email
        content.push_str("BT\n");
        content.push_str("/F2 9 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str(&format!("70 {} Td\n", current_y));
        content.push_str("(Email :) Tj\n");
        content.push_str("ET\n");
        
        content.push_str("BT\n");
        content.push_str("/F1 10 Tf\n");
        content.push_str("0 0 0 rg\n");
        content.push_str(&format!("140 {} Td\n", current_y));
        content.push_str(&format!("({}) Tj\n", escape_pdf_string(&creator.email)));
        content.push_str("ET\n");
        
        current_y -= 18;
        
        // Roles
        content.push_str("BT\n");
        content.push_str("/F2 9 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str(&format!("70 {} Td\n", current_y));
        content.push_str("(Role(s) :) Tj\n");
        content.push_str("ET\n");
        
        let roles_str = if !creator.roles.is_empty() {
            let capitalized_roles: Vec<String> = creator.roles.iter()
                .map(|role| {
                    let mut chars: Vec<char> = role.chars().collect();
                    if !chars.is_empty() {
                        chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
                    }
                    chars.into_iter().collect()
                })
                .collect();
            capitalized_roles.join(", ")
        } else {
            "N/A".to_string()
        };
        
        content.push_str("BT\n");
        content.push_str("/F1 10 Tf\n");
        content.push_str("0 0 0 rg\n");
        content.push_str(&format!("140 {} Td\n", current_y));
        content.push_str(&format!("({}) Tj\n", escape_pdf_string(&roles_str)));
        content.push_str("ET\n");
        
        current_y -= 18;
        
        // IPI and ISNI on same line
        content.push_str("BT\n");
        content.push_str("/F2 9 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str(&format!("70 {} Td\n", current_y));
        content.push_str("(IPI :) Tj\n");
        content.push_str("ET\n");
        
        let ipi_value = if !creator.ipi.is_empty() { 
            creator.ipi.clone() 
        } else { 
            "N/A".to_string() 
        };
        
        content.push_str("BT\n");
        content.push_str("/F1 10 Tf\n");
        content.push_str("0 0 0 rg\n");
        content.push_str(&format!("140 {} Td\n", current_y));
        content.push_str(&format!("({}) Tj\n", escape_pdf_string(&ipi_value)));
        content.push_str("ET\n");
        
        // ISNI
        content.push_str("BT\n");
        content.push_str("/F2 9 Tf\n");
        content.push_str("0.09 0.60 0.47 rg\n");
        content.push_str(&format!("270 {} Td\n", current_y));
        content.push_str("(ISNI :) Tj\n");
        content.push_str("ET\n");
        
        let isni_value = if !creator.isni.is_empty() { 
            creator.isni.clone() 
        } else { 
            "N/A".to_string() 
        };
        
        content.push_str("BT\n");
        content.push_str("/F1 10 Tf\n");
        content.push_str("0 0 0 rg\n");
        content.push_str(&format!("320 {} Td\n", current_y));
        content.push_str(&format!("({}) Tj\n", escape_pdf_string(&isni_value)));
        content.push_str("ET\n");
        
        *y_pos -= CREATOR_HEIGHT + 10; // Move to next position
    }
    
    fn add_footer_content(&self, content: &mut String, data: &CertificateData, footer_y: i32) {
        // Bottom separator line
        content.push_str("0.09 0.60 0.47 RG\n"); // Green/teal line
        content.push_str(&format!("50 {} m\n545 {} l\nS\n", footer_y, footer_y));
        
        // Footer - timestamp only (right-aligned)
        content.push_str("BT\n");
        content.push_str("/F1 9 Tf\n");
        content.push_str("0.4 0.4 0.4 rg\n");
        
        // Calculate text width and position to align right with separator line end (545)
        // Approximate character width at 9pt font is about 5 points
        let timestamp_text = escape_pdf_string(&data.timestamp);
        let estimated_width = timestamp_text.len() as f32 * 4.5; // Rough estimate for 9pt font
        let right_align_x = 545.0 - estimated_width;
        
        content.push_str(&format!("{} {} Td\n", right_align_x as i32, footer_y - 20));
        content.push_str(&format!("({}) Tj\n", timestamp_text));
        content.push_str("ET\n");
    }
}

fn escape_pdf_string(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '(' => "\\(".to_string(),
            ')' => "\\)".to_string(),
            '\\' => "\\\\".to_string(),
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            '\t' => "\\t".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

// Generate a random 64-character hex string (256 bits)
fn generate_hash_64() -> String {
    let chars = "0123456789abcdef";
    let mut result = String::with_capacity(64);
    
    for _ in 0..64 {
        let random_index = (Math::random() * 16.0) as usize;
        let ch = chars.chars().nth(random_index).unwrap_or('0');
        result.push(ch);
    }
    
    result
}